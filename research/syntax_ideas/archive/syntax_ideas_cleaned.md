# Syntax for folding/reducing operations over a directory.

```
fold dir/ {
    .*grades_2018.csv => grades/2018.csv
    .*grades_2019.csv => grades/2019.csv
}
```
```
fold dir/ {
    .*grades_2018.csv => head 5 => grades/2018.csv
}
```
So all files that match the pattern go to the file on the right, if multiple files go to the same outfile, they get appended to/merged with each other. Then from there can do our einops-like functions on the file data themselves. So the expressions on the right of the => could be mutation functions on the files.

# Syntax for describing directory structure for applying operations over.

```
Directory StudentGrades {
    Math: {
        2018: csv
        2019: csv
    }
    Science: {
        2020: json
    }
}
```
Manual description of the complete structure - Jay

```
DirectorySet Years: [0-9]{4}
DirectorySet Grades: {
  "Math",
  "Science",
  "Geography"
}

# ops: apply_some, contains_some

Directory Dataset {
  apply_some Grades (contains_some Years) 
}
```
Manual description of atoms of the structure, conciser description of actual structure with operations - William
Omits type information for brevity, could be included with an map data structure and appropriate operation. Depending on how structured the type of each leaf directory is could still be more concise (or atleast extendable) with operations.

```
DirectorySet Years: [0-9]{4}: value(int)
DirectorySet Grades: {
  "Math",
  "Science",
  "Geography"
}

func versionType(year: int) -> filetype {
    return json if year > 2019 else csv
}

# ops: reduce_some, contains_some
# func typedDirectorySet(DirectorySet: dir, func: typingFunc)

Directory Dataset {
  reduce_some Grades ((contains_some (typedDirectorySet Years versionType)) ) 
}
```
An elaboration on the prior syntax, with typed directory prototypes.
This is assuming that simple logic covers typing, in simpler cases the function could be much simpler (i.e. just return json), and maybe lifted off with syntax sugar. More complex logic could utilise a map.
Problem with this typing is typedDirectorySet only takes the directory being typed, more complex logic would likely want to have its ancestors in context. Possibly writing the tree untyped, then operating over it with specialised typing operations would be better for more complex cases.


```
DirectorySet Years: [0-9]{4}
DirectorySet Grades: {
  "Math",
  "Science",
  "Geography"
}

func versionType(year: Directory) -> filetype {
    return json if int(year) > 2019 else csv
}

# ops: reduce_some, contains_some
# func typedDirectorySet(DirectorySet: dir, func: typingFunc)

Directory Dataset {
  reduce_some Grades ((contains_some (typedDirectorySet Years versionType)) ) 
}
```

```
DirectorySet Subjects {
  Names: {
    "Math",
    "Science",
    "Geography"
  }
}

DirectorySet Years {
  Names: [0-9]{4}  
}

DirectorySet Categories {
  Names: [A-D]
}

NameSet Student {
  [A-Za-z]_[A-Za-z]
}

FileSet Grade {
  Names: Student
}

# ops: some, reduce, contains
def contains x y -> "x contains y"
def some x -> "DirectorySet matching 1 or more of DirectorySet x"
def exactly i x -> "DirectorySet matching i  of DirectorySet x"
def rev f -> "Function f with arguments reversed"
def rev l -> "List l with elements reversed"
def apply x y -> "apply y to each element of x"
def reduce x y -> "reduce y over x"
// def someXhassomeY x y -> some apply x (rev contains) some y
def layered l -> reduce (rev l) (rev contains)
def roll (a, b) -> a, b # i.e. (1, 2, roll((3,4))) -> (1, 2, 3, 4)
def split a f -> (a, f(a))

DirectorySet Dataset {
  Names: "Dataset"
  Contains: layered (grades, years)
}

DirectorySet Dataset {
  Names: "Dataset2"
  Contains: layered (some Subjects, exactly 5 Years, some Categories)
}

DirectorySet Dataset {
  Names: "Dataset3"
  Contains: {
    layered {
      some Subjects,
      map (exactly 5 Years) \x roll split x \year {
        year < 2019 => {
          JSONFiles
        },
        year >= 2019 => {
          CSVFiles
        }
      }
    }
  }
}

DirectorySet Dataset {
  Names: "Dataset3"
  Contains: {
    layered {
      some Subjects,
      exactly 5 Years,
      split prior 0 \year {
        year < 2019 => {
          JSONFiles
        },
        year >= 2019 => {
          CSVFiles
        }
      }
    }
  }
}



DirectorySet Dataset {
  Names: "Dataset4"
  Contains: {
    layered {
      some Subjects,
      some Years,
      split prior 0 \year {
        year < 2019 => {
          exactly 3 Categories,
          CSVFiles
        },
        year >= 2019 => {
          exactly 5 Categories,
          JSONFiles
        }
      }
    }
  }
}

def layered l -> reduce (rev l) (rev contains)

constructor LabelledTree this {layers edge} ->
    take a labelled set as layers and edge
    calls function layered on values of layers set
    has last layer of layers set contain all of the edges set
    

DirectorySet Dataset {
  Names: "Dataset",
  Contains:
    LabelledTree {
      layers: {
        subjects: some Subjects,
        years: some Years,
        categories: some Categories,
      },
      edge: {
        description: JSONFile,
        grades: CSVFile
      }
    }
}

DirectorySet Dataset {
  Names: "Dataset",
  Structure:
    LabelledTree {
      layers: {
        subjects: Subjects,
        years: Years,
        categories: Categories,
      },
      edge: {
        description: TextFile,
        grades: File
      }
    }
  Tags: {
      years: {
        version: ifelse asint years > 2019 2.0 1.0,
        new: version == 2 
      }
  }
  Specialisations: {
      subjects: some,
      years: exactly ifelse years.new 5 3,
      categories: exactly 4,
      grades: specialise ifelse years.new JSONFile CSVFile
  }
}


DirectorySet Dataset {
  Names: "Dataset3"
  Contains: {
    layered {
      some Subjects, 
      carry split (some Years) \year {
        year < 2019 => {
          5
        },
        year >= 2019 => {
          2
        }
      },
      rev exactly Categories 
    }
  }
}

DirectorySet Dataset {
  Names: "Dataset3"
  Contains: {
    layered {
      some Subjects, 
      carry split (some Years) \year {
        year < 2019 => {
          OLD
        },
        year >= 2019 => {
          NEW
        }
      },
      some Categories,
      match {
        OLD => JSONFile
      }
    }
  }
}

```


```
def NumericDirectorySet n f -> TaggedDirectorySet {Names = n, value = f}

subjects <- DirectorySet {"Math", "Science", "Geography", "Art"}
years <- NumericDirectorySet {[0-9]{4}, int}
categories <- DirectorySet {[A-D]}
student <- NameSet {[A-Za-z]_[A-Za-z]}

dataset = Directory {"Dataset"}
dataset <| layered (some Subjects, exactly 5 Years, some Categories)
```


```
def NumericDirectorySet n f -> TaggedDirectorySet {Names = n, value = f}

DirectorySet Subjects {
  Arts {
      "Art",
      "History",
      "Geography"
  },
  STEM {
      "Science",
      "Math"
  }
}

DirectorySet Years {
  [0-9]{4}
}

DirectorySet Categories {
  DirectorySet {[A-D]}
}


dataset = Directory {"Dataset"}
dataset <| layered { some Subjects,exactly 5 Years, some Categories }

dataset { 
  some Subjects {
    match Subjects.tag {
      Arts => layered {exactly 5 Years, some Categories}
      STEM => layered {exactly 3 Years, some Categories}
    }
  }
}
```
