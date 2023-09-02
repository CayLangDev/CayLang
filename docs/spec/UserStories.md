Take for example the structure used by the librispeech dataset. A root Librispeech folder, with a layer of subset folders, with a layer of reader folders, with numbered flac files and a transcription of the reading at the edges.

Example generated below.
```
                                       ┌ RD129-CH275-0.flac 
                                       │
                                       ├ RD129-CH275-1.flac 
                                       │
                               ┌ CH275 ┼ RD129-CH275-2.flac 
                               │       │
                               │       ├ RD129-CH275-3.flac 
                               │       │
                               │       └ RD129-CH275.trans.txt 
                       ┌ RD129 ┤
                       │       │       ┌ RD129-CH276-0.flac 
                       │       │       │
                       │       │       ├ RD129-CH276-1.flac 
                       │       │       │
                       │       └ CH276 ┼ RD129-CH276-2.flac 
                       │               │
                       │               ├ RD129-CH276-3.flac 
                       │               │
                       │               └ RD129-CH276.trans.txt 
                       │
                       │               ┌ RD130-CH276-0.flac 
                       │               │
                       │               ├ RD130-CH276-1.flac 
                       │               │
- Librispeech ─ subset ┤       ┌ CH276 ┼ RD130-CH276-2.flac 
                       │       │       │
                       │       │       ├ RD130-CH276-3.flac 
                       │       │       │
                       │       │       └ RD130-CH276.trans.txt 
                       ├ RD130 ┤
                       │       │       ┌ RD130-CH277-0.flac 
                       │       │       │
                       │       │       ├ RD130-CH277-1.flac 
                       │       │       │
                       │       └ CH277 ┼ RD130-CH277-2.flac 
                       │               │
                       │               ├ RD130-CH277-3.flac 
                       │               │
                       │               └ RD130-CH277.trans.txt 
                       │
                       │               ┌ RD131-CH275-0.flac 
                       │               │
                       │               ├ RD131-CH275-1.flac 
                       │               │
                       └ RD131 ─ CH275 ┼ RD131-CH275-2.flac 
                                       │
                                       ├ RD131-CH275-3.flac 
                                       │
                                       └ RD131-CH275.trans.txt 
```

We set up simple directory sets for matching our different directory types
```
DirectorySet ReaderDir {
    Names: r"RD\d{4,}"
}

DirectorySet ChapterDir {
    Names: r"CH\d{4,}"
}
```


```
TreeDirectorySet LSDataSet {
    Names: f"Librispeech",
    Structure: {
        layers: {
            Subset: Dir,
            Reader: ReaderDir,
            Chapter: ChapterDir
        }
        edges: {
            Audio: File<r"*.flac">,
            Transcript: File<r"*.trans.txt">
        }
    }
    Specialisations: {
        Reader: atleast 1
        Chapter: atleast 1
        Audio: atleast 1
        Transcript: exactly 1
    }
}
```
Fairly compact description, we're not verifying the names of the audio files or transcript.

Good enough for a programmer to hack on, not ideal for a distributor.

Lets say we want to leave the structure as is but concatenate all the flac files.
```
fold "~": LSDataSet {
  Subset: Dir {name as subset} => {
    Reader: ReaderDir {name as reader} => {
      Chapter: ChapterDir {name as chapter} => {
        Audio: File<r"*.flac"> { .. } => cat_all => `{root}/{subset}/{reader}/{chapter}/{reader}-{chapter}.flac`
        Transcript: File<r"*.trans.txt"> {name as transcipt} => `{root}/{subset}/{reader}/{chapter}/{transcipt}`
      }
    }
  }
}
```

Now our tree is as follows
```
                       ┌ RD129-CH275.flac 
               ┌ CH275 ┤
               │       └ RD129-CH275.trans.txt 
       ┌ RD129 ┤
       │       │       ┌ RD129-CH276.flac 
       │       └ CH276 ┤
       │               └ RD129-CH276.trans.txt 
       │
       │               ┌ RD130-CH276.flac 
- root ┤       ┌ CH276 ┤
       │       │       └ RD130-CH276.trans.txt 
       ├ RD130 ┤
       │       │       ┌ RD130-CH277.flac 
       │       └ CH277 ┤
       │               └ RD130-CH277.trans.txt 
       │
       │               ┌ RD131-CH275.flac 
       └ RD131 ─ CH275 ┤
                       └ RD131-CH275.trans.txt 
```

Now suppose we want to change the structure so that the subset layer is followed by a chapter layer which is followed by a reader layer; each chapter folder contains a folder for each reader who has read it rather than vice versa.

```
fold "~": LSDataSet {
  Subset: Dir {name as subset} => {
    Reader: ReaderDir {name as reader} => {
      Chapter: ChapterDir {name as chapter} => {
        Audio: File<r"*.flac"> { .. } => cat_all => `{root}/{subset}/{chapter}/{reader}/{reader}-{chapter}.flac`
        Transcript: File<r"*.trans.txt"> {name as transcipt} => `{root}/{subset}/{chapter}/{reader}/{transcipt}`
      }
    }
  }
}
```
Note now we write ``{root}/{subset}/{chapter}/{reader}/...`` not `{root}/{subset}/{reader}/{chapter}/...`.
The fold operation rebuilds our tree from the root, allowing this change in structure.

Now our tree is as follows
```
                       ┌ RD129-CH275.flac 
               ┌ RD129 ┤
               │       └ RD129-CH275.trans.txt 
       ┌ CH275 ┤
       │       │       ┌ RD131-CH275.flac 
       │       └ RD131 ┤
       │               └ RD131-CH275.trans.txt 
       │
       │               ┌ RD129-CH276.flac 
- root ┤       ┌ RD129 ┤
       │       │       └ RD129-CH276.trans.txt 
       ├ CH276 ┤
       │       │       ┌ RD130-CH276.flac 
       │       └ RD130 ┤
       │               └ RD130-CH276.trans.txt 
       │
       │               ┌ RD130-CH277.flac 
       └ CH277 ─ RD130 ┤
                       └ RD130-CH277.trans.txt 
```

Finally suppose we want to fully flatten our dataset and concatenate all audio files from the same reading.
```
fold "~": LSDataSet {
  Subset: Dir {name as subset} => {
    Reader: ReaderDir {name as reader} => {
      Chapter: ChapterDir {name as chapter} => {
        Audio: File<r"*.flac"> { .. } => cat_all => `{root}/{reader}-{chapter}.flac`
        Transcript: File<r"*.trans.txt"> {name as transcipt} => `{root}/{transcipt}`
      }
    }
  }
}
```

Now our tree is fully flattened.
```
              ┌ RD129-CH275.flac 
              │
              ├ RD129-CH275.trans.txt 
              │
              ├ RD129-CH276.flac 
              │
              ├ RD129-CH276.trans.txt 
              │
              ├ RD130-CH276.flac 
- Librispeech ┤
              ├ RD130-CH276.trans.txt 
              │
              ├ RD130-CH277.flac 
              │
              ├ RD130-CH277.trans.txt 
              │
              ├ RD131-CH275.flac 
              │
              └ RD131-CH275.trans.txt 
```
