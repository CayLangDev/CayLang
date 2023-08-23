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
        layers {
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

```
match SmallNumDir f"root" |>  // f"dataset" is the path to our dataset folder
fold Num {
    r"1"/file => `1_{file}`,
    r"2"/file => `2_{file}`,
    r"3"/file => `3_{file}`,
}
```

Now we've flattened our dataset
```
                                                                dataset
       ┌──────────────┬──────────────┬──────────────┬──────────────┼──────────────┬──────────────┬──────────────┬──────────────┐
 1_file_a.txt   1_file_b.txt   1_file_c.txt   2_file_a.txt   2_file_b.txt   2_file_c.txt   3_file_a.txt   3_file_b.txt   3_file_c.txt
```

Alternatively we could write
```
match SmallNumDir f"root" |>  // f"dataset" is the path to our dataset folder
fold Num {
    num/file => `{num}_{file}`
}
```
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
                                                                                            │
                                                                                           000
                       ┌─────────────────────────────────────────┬──────────────────────────┴────────┬───────────────────────────────────────────────────────────┐
                      000                                       001                                 002                                                         003
                       │                             ┌───────────┴───────────┐                       │                 ┌───────────────────────────────────┬─────┴───────────────────────┬─────────────────┐
                      000                           000                     001                     000               000                                 001                           002               003
     ┌───────────┬─────┴─────┬───────────┐           │           ┌───────────┼───────────┐           │           ┌─────┴─────┐           ┌───────────┬─────┴─────┬───────────┐           │           ┌─────┴─────┐
 26e9c.txt   d9993.txt   d36c1.txt   96f84.txt   bd4a0.txt   e0bbf.txt   6fb3b.txt   82b3d.txt   09f32.txt   64dfa.txt   2965a.txt   0c8b4.txt   7a1c1.txt   f1cdc.txt   7a42f.txt   036d0.txt   decc9.txt   a414a.tx
 ```


Flattened
```
                                                                                                                                                                                                                                                                                                                                                              root
                   ┌──────────────────────────────────────┬──────────────────────────────────────┬──────────────────────────────────────┬──────────────────────────────────────┬──────────────────────────────────────┬──────────────────────────────────────┬──────────────────────────────────────┬──────────────────────────────────────┬───────────────────┴──────────────────┬──────────────────────────────────────┬──────────────────────────────────────┬──────────────────────────────────────┬──────────────────────────────────────┬──────────────────────────────────────┬──────────────────────────────────────┬──────────────────────────────────────┬──────────────────────────────────────┐
 root_000_000_000_e4ea9.txt_e4ea9.txt   root_000_000_000_01ec4.txt_01ec4.txt   root_000_000_000_95695.txt_95695.txt   root_000_000_000_2e502.txt_2e502.txt   root_000_001_000_1542d.txt_1542d.txt   root_000_001_001_563f1.txt_563f1.txt   root_000_001_001_e66aa.txt_e66aa.txt   root_000_001_001_330ac.txt_330ac.txt   root_000_002_000_66847.txt_66847.txt   root_000_003_000_0bae3.txt_0bae3.txt   root_000_003_000_4e98a.txt_4e98a.txt   root_000_003_001_3d304.txt_3d304.txt   root_000_003_001_028db.txt_028db.txt   root_000_003_001_b62b9.txt_b62b9.txt   root_000_003_001_662ef.txt_662ef.txt   root_000_003_002_49e19.txt_49e19.txt   root_000_003_003_5b9b3.txt_5b9b3.txt   root_000_003_003_2dbef.txt_2dbef.txt
 ```
