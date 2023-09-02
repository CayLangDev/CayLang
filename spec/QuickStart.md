# Example 1

## Matching
```
  dataset 
 ┌───┼───┐
 1   2   3 
```
Matching using DirectorySet and Tree Directory Set

```
DirectorySet SmallNumDir {
  Names: r"[123]",
  Tags: {
    num: asint name
  }
}
```
...

```
TreeDirectorySet NumDataSet {
    Names: f"dataset",
    Structure: {
        layers {
            Nums: SmallNumDir,
            Files: File
        }
    }
}
```

```
                                                       dataset 
                   ┌──────────────────────────────────────┼──────────────────────────────────────┐
                   1                                      2                                      3 
      ┌────────────┼────────────┐            ┌────────────┼────────────┐            ┌────────────┼────────────┐
 file_a.txt   file_b.txt   file_c.txt   file_a.txt   file_b.txt   file_c.txt   file_a.txt   file_b.txt   file_c.txt 
```
...
Right now our tree directory set doesn't specify how many SmallNumDir's are in the Nums layer, and how many Files are in the Files layes.

```
                                                                 dataset 
             ┌─────────────────────────┬────────────────────────────┴──┬────────────────────────────────┬──────────────────┐
             1                         1                               2                                3                  3 
      ┌──────┴─────┐            ┌──────┴─────┐            ┌────────────┼────────────┐            ┌──────┴─────┐            │
 file_a.txt   file_b.txt   file_a.txt   file_b.txt   file_a.txt   file_b.txt   file_c.txt   file_a.txt   file_b.txt   file_a.txt 
```
So the above is a valid match.
While this flexibility is useful, we'd like to be able to control it so we can be as specific (or partially specific) as we like.

```
TreeDirectorySet NumDataSet {
    Names: f"dataset",
    Structure: {
        layers: {
            Nums: SmallNumDir,
            Files: File
        }
    }
    Specialisations: {
        Nums: exactly 3,
        Files: exactly 3
    }
}
```
Now we'll only match a dataset with the symmetric and full structure first shown!

## Operation

Suppose we have our dataset located as a subfolder of our home directory ("~"), such that it's path is `~/dataset`.
Then we can flatten our dataset as follows.

```
from f"~" fold NumDataSet {
  root => {
    nums => {
      files => `{root}/{nums}_{files}`
    }
  }
}
```
`Operation syntax is WIP, see Operations.md`

Now we've flattened our dataset
```
                                                                dataset 
       ┌──────────────┬──────────────┬──────────────┬──────────────┼──────────────┬──────────────┬──────────────┬──────────────┐
 1_file_a.txt   1_file_b.txt   1_file_c.txt   2_file_a.txt   2_file_b.txt   2_file_c.txt   3_file_a.txt   3_file_b.txt   3_file_c.txt 
```

Alternatively we could write
```
from f"~" fold NumDataSet {
  root => {
    r"1" num => { // r"1" is regex matching only 1, num is an identifier capturing the match
      files => `{root}/{num}_{files}
    },
    r"2" => { // if we don't care about the match's value we can omit the identifier
      files => `{root}/2_{files}
    },
    r"." num => { // but sometimes we need it
      files => `{root}/{num}_{files}
    }
  }
}
```
`Operation syntax is WIP, see Operations.md`
The former is more useful for edge cases, especially when there are different types of files.


# Example 2
Matching with more complex semantics
```
DirectorySet Years {
  Names: r"[0-9]{4}",
  Tags: {
    year: asint name,
    version: ifelse year > 2019 2.0 1.0,
    new: version > 1.0
  }
}

DirectorySet Subjects {
  Names: r"Math|Science|Geography|Arts|IT"
}

DirectorySet Categories {
  Names: r"[A-D]"
}

FileSet StudentGrades {
  Names: r"[A-Za-z]_[A-Za-z]"
}

TreeDirectorySet Dataset {
  Names: r"Dataset",
  Structure: {
      // layers are nested beneath each other sequentially
      layers: {
        years: Years,
        subjects: Subjects,
        categories: Categories,
      },
      // edges are all nested beneath the last layer
      edge: {
        description: TextFile,
        grades: StudentGrades
      }
  },
  // Specialisations are functions that have their corresponding directory set applied, and replace them
  // we can access tags in specialisations, but only tags of directory sets higher in the tree
  Specialisations: {
      years: atleast 5,
      subjects: exactly ifelse years.new 5 3, // the amount of subject folders in a year folder is dependent on its value
      categories: exactly 4,
      grades: specialise ifelse years.new JSONFile CSVFile // the file type of the grades files is dependent on the value of its years ancestor
  }
}
```

# Example 3
tbd make the trees smaller
Unstructured matching
```
                                                               root 
                          ┌─────────────────────────────────────┴┬───────────┬───────────┬───────────┐
                         000                                    001         002         003         004 
           ┌──────────────┴──────────────┐                       │           │           │           │
          000                           001                  6ca9a.txt   833ba.txt   7699f.txt   19981.txt 
     ┌─────┴─────┐           ┌───────────┼───────────┐
    000         001         002         003         004 
     │           │           │           │           │
 0f5b8.txt   84e37.txt   edb7b.txt   97c21.txt   d64e8.txt 
```


Flattened
```
                                                                                                                                                                root 
                   ┌──────────────────────────────────────┬──────────────────────────────────────┬──────────────────────────────────────┬────────────────────────┴─────────────┬──────────────────────────────────┬──────────────────────────────┬──────────────────────────────┬──────────────────────────────┐
 root_000_000_000_53b11.txt_53b11.txt   root_000_000_001_6a913.txt_6a913.txt   root_000_001_002_8a79b.txt_8a79b.txt   root_000_001_003_732e7.txt_732e7.txt   root_000_001_004_3a7d6.txt_3a7d6.txt   root_001_f0834.txt_f0834.txt   root_002_cc946.txt_cc946.txt   root_003_08e28.txt_08e28.txt   root_004_0a495.txt_0a495.txt 
```
