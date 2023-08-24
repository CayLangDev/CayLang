Basic Types
* Sets
    * `{a, b, ...}`
* Labelled Sets
    * `{a:..., b:..., ...}`
    * set of pairs of identifiers and objects
* Functions
    * `function_name arg1 arg2 .... argn`
    * left associative
* Constructs
    * special baselang functions that operate at compiletime
    * i.e.
        * DirectorySet is a construct that takes an identifier and a labelled set as arguments

Base Functions
```
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
def layered l -> reduce (rev l) (rev contains)
def ifelse cond x y -> "if cond is true x is returned else y"
```
        
Directory description
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
