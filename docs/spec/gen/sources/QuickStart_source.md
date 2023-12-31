# Example 1

## Matching
```
{{ex1}}
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
{{ex2}}
```
...
Right now our tree directory set doesn't specify how many SmallNumDir's are in the Nums layer, and how many Files are in the Files layes.

```
{{ex3}}
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
fold "~": NumDataSet {
  Root {name as root, ..} => {
    SmallNumDir {name as nums, ..} => {
      File {name as files, ..} => `{root}/{nums}_{files}`
    }
  }
  _ => .
}
```
`Operation syntax is WIP, see LanguageSpec.md and Operations.md`

Now we've flattened our dataset
```
{{ex4}}
```

Alternatively we could write
```
fold "~": NumDataSet {
  Root { name as root, .. } => {
    Num { name as num, .. } => {
      File { name as files, .. }
        | matches num r"1" => `{root}/{num}_{files} // r"1" is regex matching only 1
        | matches num r"2" => `{root}/2_{files}
        | matches num r"." => `{root}/{num}_{files}
    }
  }
}
```
`Operation syntax is WIP, see LanguageSpec.md and Operations.md`

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
{{ex5.1}}
```


Flattened
```
{{ex5.2}}
```
