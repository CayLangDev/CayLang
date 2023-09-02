# CayLang Spec 0.001

## Basic Data Types
In CayLang we have a small set of primitive datatypes from which the rest of the language is built.

### Atoms

* Regular Expressions
    * Written as `r"<regex here>"`
* Integer
    * Written as `1`
* Float
    * Written as `1.0`
* Prototypes
    * Abstract objects that match a file or directory following certain conditions.

### Collections

* Sets
    * Sets are unordered containers of objects
    * Instantiated as `{a, b, ...}`
* Labelled Sets
    * A set of pairs of identifiers and objects
    * Instantiated as `{a:..., b:..., ...}`
None of the collections in CayLang can be interacted with by the user right now.
Instead they are instantiated and passed to functions, to declare computation.

### Computation

#### Functions
Functions are called in haskell style, `function_name arg1 arg2 .... argn`.
Functions are left associative, so if `func_a` and `func_b` both accept two arguments.

`func_a 1 func_b 1 2` is equivalent to `(func_a 1 (func_b 1 2))` or `f_a(1, f_b(1,2))` in C style syntax.

#### Constructs
Constructs are special base language functions that operate at compiletime, they may take identifiers as arguments.

i.e. DirectorySet is a construct that takes an identifier and a labelled set as arguments
```
DirectorySet SmallNumDir {
  Names: r"[123]",
  Tags: {
    num: asint name
  }
}
```
Here the DirectorySet construct will define a prototype named SmallNumDir that matches any directory with the name "1", "2", or "3"; with a tag `num`, a value associated.

## Control Flow
### Fold
Folds are a powerful control flow mechanism in CayLang. It is the primary mechanism for operating over directories and files, whereby one tree structure is transformed into another. Folds pattern match on the structure of the input tree (similar to Rust's `match` expressions) and apply functions to files and directories, with Haskell-like guard conditions on attributes. A fold has the following form:

> FoldExpression := `fold` Path `:` DestructuredType `{` \
>&nbsp;&nbsp; (Clause)<sup>+</sup>\
> `}`
>
> DestructuredType := \
>&nbsp;&nbsp; Identifier ( `{` (Identifier (`as` Identifier)<sup>?</sup>`,` )<sup>*</sup> `..`<sup>?</sup> `}`)<sup>?</sup>
>
> Clause := \
>&nbsp;&nbsp; DestructuredType `=>` `{` (Clause)<sup>+</sup> `}` \
>&nbsp;&nbsp; DestructuredType (Guard)<sup>+</sup> \
>&nbsp;&nbsp; DestructuredType (`=>` Function)<sup>\*</sup> `=>` Path
>
> Guard := \
>&nbsp;&nbsp; `|` Function (`&&` Function) <sup>\*</sup> (`=>` Function)<sup>\*</sup> `=>` Path \
>&nbsp;&nbsp; `|` Function (`&&` Function)<sup>*</sup> `=>` `{` (Clause)<sup>+</sup> `}`
>
> Path := `"`String`"`
>

The following example demonstrates a simple fold:

```rust
fold "dir": MyDir { .. } {
    File { .. } => "out.txt"
    _ => .
}
```

In this example, we have a custom directory structure `MyDir`, and we're sending every file to a file `out.txt`. The first clause is a file read, and you can think of the first action being a shell `cat`, and the final action being a `>` write. In the case of multiple files going to the same outfile, they are appended to each other. Folds should be exhaustive matches, and the final clause is a special case which simply doesn't change anything else.

```rust
fold "dir": MyDir {
    SubDirOne => {
        SubDirTwo => {
            SubDirThree => {
                CsvFile { name, .. }
                    | lengthGreaterThan10 name => head 5 => reverse => "~/flattened/{name}"
            }
        }
    }
    _ => .
}
```

This example demonstrates a more complex nested fold. In this case, we're folding over a directory `dir/` with type `MyDir`, and going to the specific subdirectory matched by the type `SubDirThree`, which is parented by a `SubDirTwo`, which is parented by a `SubDirOne`. We then match on all CSV files and extract the name of each. If a CSV file has a name longer than 10 characters, we take the first 5 lines, reverse them, and write them to a file in the `~/flattened/` directory with the same name as the CSV file.

## Built Ins

### Description Constructs

#### DirectorySet
DirectorySet defines a prototype that matches, and may associate metadata with, a directory based on its name.
```
DirectorySet <directoryset_name> {
   Names: r"<regex describing possible names>",
   Tags: {
        <tag_label>: <tab_operation>,
        ...
   }
}
```
The Names field defines a regular expression that matches the names of a directory that may be considered a valid member of this DirectorySet.
The Tags field defines metadata that is computed for each particular instance of the directory set based on the actual name of the matched directory.
Each tag has a label, which allows it's value to be accessed, and an expression that calculates its value.
Expressions may pass the labels of other tags to functions, which will perform calculations using their values.
When a directory is matched it is given that before any other tags are calculated the directory will be given a `name` tag corresponding to its actual name.

##### Example
```
DirectorySet SmallNumDir {
  Names: r"[123]",
  Tags: {
    num: asint name
  }
}
```
Here we define a directory set SmallNumDir that matches any directory with the name "1", "2", or "3" and associates a tag `num`, equal to the directory's name as an integer with each matched directory.

#### TreeDirectorySet
TreeDirectorySet defines a prototype that matches, and may associate metadata with, a directory based on the structure of its file tree.
```
TreeDirectorySet <treedirectoryset_name> {
    Names: r"<regex describing possible names>",
    Structure: {
      // layers are nested beneath each other sequentially
      layers: {
        <layer_1_label>: <layer_1_prototype>,
        <layer_2_label>: <layer_2_prototype>,
        ...
      },
      // edges are all nested beneath the last layer
      edge: {
        <edge_node_1_label>: <edge_node_1_prototype>,
        <edge_node_2_label>: <edge_node_2_prototype>,
      }
  },
  // Specialisations are labelled functions that have the corresponding (same label) prototype applied to them,
  // they return a new prototype which replaces their input
  // we can access tags in specialisations, but only tags of directory sets higher in the tree
  Specialisations: {
      <specialisation_label>: <specialisation_function>,
      <specialisation_label>: <specialisation_function>,
      ...
  }
}
```
The Names field defines a regular expression that matches the names of a directory that may be considered a valid member of this TreeDirectorySet.
The Structure field wraps the layers and edge labelled sets, which describes the structure of a matching file tree.
* Layers, consists of labels and depth bounded protypes
    * Depth-bounded, a layer's prototype can't match a ...


### Operation Constructs


### Built In Functions
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
def either a b -> "takes prototypes a and b, returns a prototype matching either"
def star -> "returns a prototype matching any file or directory"
```
