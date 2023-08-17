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


        
        
```
DirectorySet Years {
  Names: [0-9]{4}  
}

DirectorySet Subjects {
  Names: {
    "Math",
    "Science",
    "Geography",
    "Arts",
    "IT"
  }
}

DirectorySet Categories {
  Names: [A-D]
}

FileSet StudentGrades {
  names: [A-Za-z]_[A-Za-z]
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

def layered l -> reduce (rev l) (rev contains)

constructor LabelledTree this {layers edge} ->
    take a labelled set as layers and edge
    calls function layered on values of layers set
    has last layer of layers set contain all of the edges set
    
    
DirectorySet Dataset {
  Names: "Dataset",
  Structure:
    LabelledTree {
      layers: {
        years: Years,
        subjects: Subjects,
        categories: Categories,
      },
      edge: {
        description: TextFile,
        grades: File
      }
    },
  Tags: { 
  // Tags are attributes assigned to particular directories based on their name and other tags
      years: {
        version: ifelse asint years > 2019 2.0 1.0, // each year directory will have a distinct version tag
        new: version == 2 // and a corresponding version tag
      }
  }
  Specialisations: {
 // specialisations are functions that have their corresponding directory set applied, and replace them
 // we can access tags in specialisations, but only tags of directory sets higher in the tree
      years: atleast 5,
      subjects: exactly ifelse years.new 5 3, // the amount of subject folders in a year folder is dependent on its value
      categories: exactly 4,
      grades: specialise ifelse years.new JSONFile CSVFile // the file type of the grades files is dependent on the value of its years ancestor
  }
}


LabelledDirectorySet Dataset {
  Names: "Dataset",
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
        grades: File
      }
  },
  Tags: { 
  // Tags are attributes assigned to particular directories based on their name and other tags
      years: {
        version: ifelse asint years > 2019 2.0 1.0, // each year directory will have a distinct version tag
        new: version == 2 // and a corresponding version tag
      }
  }
  Specialisations: {
 // specialisations are functions that have their corresponding directory set applied, and replace them
 // we can access tags in specialisations, but only tags of directory sets higher in the tree
      years: atleast 5,
      subjects: exactly ifelse years.new 5 3, // the amount of subject folders in a year folder is dependent on its value
      categories: exactly 4,
      grades: specialise ifelse years.new JSONFile CSVFile // the file type of the grades files is dependent on the value of its years ancestor
  }
}
```




