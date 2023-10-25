# User Story 1: Simple Dataset Sanity Check

A user is able to write a simple cay script that describes the structure of a simple dataset they’re working with/have created. They can then run that cay file with `cay` throughout development as a sanity check, ensuring it hasn’t been changed from its expected structure.

Suppose for example a developer is working on a Rust project.

# User Story 2: Simple Dataset Shuffle

A user is able to write a cay script that can permute the order of layers in a file system. Running the cay file with ‘cay’ will change the relation of the file layers in the file system. A user may wish to use this to shuffle the student grades layer relation from the structure year/student-id/course-grade.txt to student-id/year/course-grade.txt, which would be difficult without such a tool.

Suppose the user's tree looks like the following

```
{{user}}
```

Then they should be able to write cay code to shuffle the first to layers.

# User Story 3: Simple Dataset Flatten

A user is able to write a cay script that can flatten directories in the file system. Running the cay file with ‘cay’ will merge various directory and file paths to easily simplify the structure. A user may wish to use this to flatten a student grades dataset from the structure year/student-id/course-grade.txt to year_student-id_course-grade.txt.

Suppose the user's tree looks like the following


```
{{user}}
```


Then they should be able to write cay code to flatten the layers.

# User Story 4: Full Featured Dataset Manipulation

A user is able to write a cay script that can shuffle and flatten various layers of a file system structure arbitrarily. Running the file with ‘cay’ will apply these changes, which would be otherwise very time consuming and complex. A user may wish to restructure a grades dataset from year/student-id/course-grade.txt to student-id_year_course-grade.txt.

# User Story 5: Fearless Dataset Manipulation

A user is able to write a cay script with a confidently correct prototype description to apply an arbitrary manipulation on a dataset. Running the file with ‘cay’ will apply the changes in exactly the way the user expects as they have ensured that the dataset matches their prototype.

# User Story 6: Complex Dataset Manipulation

Take for example the structure used by the librispeech dataset. A root Librispeech folder, with a layer of subset folders, with a layer of reader folders, with numbered flac files and a transcription of the reading at the edges.

Example generated below.
```
{{librispeech}}
```

We set up a simple tree directory set for matching our structure.
```
TreeDirectorySet LSDataSet {
    Names: f"Librispeech",
    Structure: {
        layers: {
            Subset: Dir,
            Reader: Dir<r"RD\d+">,
            Chapter: Dir<r"CH\d+">,
        }
        edges: {
            Audio: File<r"*.flac">,
            Transcript: File<r"*.trans.txt">
        }
    }
    Specialisations: {
        Subset: atleast 1,
        Reader: atleast 1,
        Chapter: atleast 1,
        Audio: atleast 1,
        Transcript: exactly 1
    }
}
```
Fairly compact description, we're not verifying the names of the audio files or transcript, or breaking our subdirectory and file prototypes into seperate DirectorySets to extract metadata.

Good enough for a programmer to hack on, not ideal for a distributor.

Lets say we want to leave the structure as is but concatenate all the flac files.
```
fold "~": LSDataSet {
  Subset: Dir {name as subset} => {
    Reader: ReaderDir {name as reader} => {
      Chapter: ChapterDir {name as chapter} => {
        Audio: File<r"*.flac"> { .. } => cat_all_flac => `{root}/{subset}/{reader}/{chapter}/{reader}-{chapter}.flac`
        Transcript: File<r"*.trans.txt"> {name as transcipt} => `{root}/{subset}/{reader}/{chapter}/{transcipt}`
      }
    }
  }
}
```

* Note: our `Audio: File<r"*.flac"> { .. }` matches all the audio files without destructuring their name, `cat_all_flac` then combines them into one audio file in order.
  * In this way we reduce over the matches, moving them to a single destination, instead of applying over them, moving them each to their own destination.
  * This form of computation is currently WIP, and may not be typed as above or follow the same semantics in the final prototype. The other examples could leave the audio files unconcatenated instead.

Now our tree is as follows
```
{{librispeech_folded_r_c}}
```

Now suppose we want to change our structure so that the subset layer is followed by a reader layer, which has the chapter layer folded into it.

```
fold "~": LSDataSet {
  Subset: Dir {name as subset} => {
    Reader: ReaderDir {name as reader} => {
      Chapter: ChapterDir {name as chapter} => {
        Audio: File<r"*.flac"> { .. } => cat_all => `{root}/{subset}/{reader}/{reader}-{chapter}.flac`
        Transcript: File<r"*.trans.txt"> {name as transcipt} => `{root}/{subset}/{reader}/{transcipt}`
      }
    }
  }
}
```

Now our structure is as follows.

```
{{librispeech_partflattened}}
```

Now suppose instead we want to change the structure so that the subset layer is followed by a chapter layer which is followed by a reader layer; each chapter folder contains a folder for each reader who has read it rather than vice versa.

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
{{librispeech_folded_c_r}}
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
{{librispeech_flattened}}
```

# User Story 7: Complex Dataset Understanding Validation

Now suppose Librispeech has noticed the success of our first user with CayLang, and would like to add it to their project so other users can manipulate the dataset as easily.

The distributor can also strengthen the conditions on the dataset prototype, allowing them to validate the format is strictly maintained after new additions, and provide more metadata to the users, to help improve the flexibility of their dataset.

```
DirectorySet ReaderDir {
    Names: r"RD\d+",
    Tags: {
      id: asint name[2:] // store integer id as a tag
    }
}

DirectorySet ChapterDir {
    Names: r"CH\d+"
    Tags: {
      id: asint name[2:] // store integer id as a tag
    }
}

FileSet RecordingFile {
  names: r"RD\d+-CH\d+-\d+.flac"
  Tags: {
    parts: split name.stem "-",
    reader_id: asint parts[0][2:],
    chapter_id: asint parts[1][2:],
    num: asint parts[2]
  }
}

FileSet TranscriptFile {
  names: r"RD\d+-CH\d+.trans.txt"
  Tags: {
    parts: split name.stem "-",
    reader_id: asint parts[0][2:],
    chapter_id: asint parts[1][2:]
  }
}

```
Here each of our Directory and FileSets uses regex to ensure the names are in the right format, and functions to extract ids from their names as integers, and stores them as tags.

We then add stronger constraints to our structure using these tags.
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
            Audio: RecordingFile,
            Transcript: TranscriptFile
        }
    },
    Specialisations: {
        Subset: atleast 1,
        Reader: atleast 1,
        Chapter: atleast 1,
        Audio: has (range 0 ((max ..num) + 1) 1 == ..num) atleast 1 has (..reader_id == Reader.id && ..chapter_id == Chapter.id),
        Transcript: exactly 1 has (..reader_id == Reader.id && ..chapter_id == Chapter.id)
    }
}
```

here we use the `has` function, to restrict what our Audio and Transcript edges can match based on the tag values of their potential matches.

`has` takes two arguments, a condition and a prototype, the condition may use the notation `..<tag>` to refer conditioning on a tag of the passed prototype; the returned prototype matches only files which match the original prototype and then also meet first condition. i.e. `Transcript: has ..chapter_id == 0` matches only transcripts with chapter id `0`.

Here we ensure our edge files all have ids matching their parents in the tree `has ..reader_id == Reader.id has ..chapter_id == Chapter.id`.

We then use the `exactly` and `atleast` functions, which transform the prototype from a single node (file or dir) prototype to a group prototype. `atleast` takes an integer argument `k` and a singular prototype argument and returns a group prototype that must match atleast `k` of the original prototype to be "matched".

Since the prototype is now a group we can specialise it across the values of the entire group as accessing the tags of the original group will access a list of the tag values.

As such we calculate the integer sequence from the 0 to the maximum `num` tag value in the group (increasing by 1) using `range`, we then check if this sequence is equal to the list of tag values of the group.
* Note since files are matched alphanumerically, and num is the only part of these files name that is distinct, we can assume our group is matched in order sorted for num.
  * Side Note: This means we can only specialise grouped children of a node, not by layer

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
            Audio: RecordingFile,
            Transcript: TranscriptFile
        }
    },
    Specialisations: {
        Subset: atleast 1,
        Reader: atleast 1,
        Chapter: atleast 1,
        Audio: has (range 0 ((max ..num) + 1) 1 == ..num) atleast 1 has (..reader_id == Reader.id && ..chapter_id == Chapter.id),
        Transcript: exactly 1 has (..reader_id == Reader.id && ..chapter_id == Chapter.id)
    },
    LayerSpecialisations: {
        Reader: has (range min ..id ((max ..id) + 1) == ..id),
        Chapter: has (range min ..id ((max ..id) + 1) == ..id)
    }
}
```

Specialisations in LayerSpecialisation apply across every matching of the labelled prototype, not just those beneath a specific node.
These Layer Specialisations in particular check that for each id-ed layer there are no ids in the range presented by the smallest ids and largest ids that are missing from the dataset.
That there are no "gaps".

We could define helpers to make our last two normal specialisations and our layer specialisations neater.
```
def gapless(seq) -> range min seq ((max seq) + 1) == seq
def complete(seq) -> range 0 ((max seq) + 1) == seq
```
Function definition sequence WIP.
