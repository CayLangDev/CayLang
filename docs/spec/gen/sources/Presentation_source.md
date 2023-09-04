
The structure used by the openspeech* dataset is a root OpenSpeech folder, followed by a layer of subset folders (we'll include only one), followed by a layer of reader folders, with a single flac file and a text file containing a transcription of the reading at the edges.

*A fictional dataset based very closely on LibriSpeech, a target dataset of our client for this project.

Example generated below.
```
{{openspeech}}
```

We set up a simple tree directory set for matching our structure.
```
TreeDirectorySet OSDataSet {
    Names: r"OpenSpeech",
    Structure: {
        layers: {
            Subset: Dir,
            Reader: Dir,
            Chapter: Dir,
        }
        edges: {
            Audio: FlacFile,
            Transcript: TextFile
        }
    },
    Specialisations: {
        Subset: atleast 1,
        Reader: atleast 1,
        Chapter: atleast 1,
        Audio: exactly 1,
        Transcript: exactly 1
    }
}
```
Fairly compact description, we're not verifying the names of the audio files or transcript, or breaking our subdirectory and file prototypes into seperate DirectorySets to extract metadata.

Good enough for a programmer to hack on, not ideal for a distributor.

Now suppose we want to change our structure so that the subset layer is followed by a reader layer, which has the chapter layer folded into it.
```
fold "~": OSDataSet {
  Subset: Dir {name as subset} => {
    Reader: Dir {name as reader} => {
      Chapter: Dir {name as chapter} => {
        Audio: FlacFile { name as audio } => `{root}/{subset}/{reader}/{audio}`
        Transcript: TextFile {name as transcipt} => `{root}/{subset}/{reader}/{transcipt}`
      }
    }
  }
}
```

Now our tree is as follows

```
{{openspeech_partflattened}}
```

Now suppose instead we want to change the structure so that the subset layer is followed by a chapter layer which is followed by a reader layer; each chapter folder contains a folder for each reader who has read it rather than vice versa.

```
fold "~": OSDataSet {
  Subset: Dir {name as subset} => {
    Reader: Dir {name as reader} => {
      Chapter: Dir {name as chapter} => {
        Audio: FlacFile { name as audio } => `{root}/{subset}/{chapter}/{reader}/{audio}`
        Transcript: TextFile {name as transcipt} => `{root}/{subset}/{chapter}/{reader}/{transcipt}`
      }
    }
  }
}
```

Note now we write ``{root}/{subset}/{chapter}/{reader}/...`` not `{root}/{subset}/{reader}/{chapter}/...`.
The fold operation rebuilds our tree from the root, allowing this change in structure.

Now our tree is as follows
```
{{openspeech_folded_c_r}}
```


