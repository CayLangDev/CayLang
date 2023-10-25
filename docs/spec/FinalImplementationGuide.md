# Final Implementation Guide
A guide to writing cay scripts compatible with the final submitted Cay implementation.

# Fold Operation

Describes manipulating a filesystem dataset

```
fold "<target>": <prototype> {
    <label> { name as <destructure_name> } => {
        <label> { name as <destructure_name> } => {
            ...
        }
    }
}
```

The dataset's path is defined by target.
Each layer of the dataset expected to exist is written out in the fold statement, and has a corresponding label and destructured_name.
The label functions essentially as a comment, but the destructured_name has semantic signifance for writing out the template that determines were leaf nodes (files) are relocated to.

```
fold "<target>": <prototype> {
    <label> { name as <destructure_name> } => {
        ...
        <label> { name as <destructure_name> } => {
            <label> { name as <destructure_name> } => {
                    <label> { name as <destructure_name> } => `<template>`
            }
        }
    }
}
```

This last part of the fold statement is its relocation element.

Ideally it would be possible to write multiple relocation elements, specifying different prototypes, so that each leaf node in the dataset will be matched to one of the prototypes, and would then be relocated based on their template.

However specifying prototypes in the fold statement outside of the very top level is currently non-functional, so any relocation element will match any file in the dataset, and therefore only one is applicable.

## Templates

A particular file being relocated based on a template will have each of their ancestors names accessed based on the occurence of destructure_names between brace pairs.

for a tree containing only the file `T/A/B/F`.

```
fold "T": <prototype> {
    A { name as a } => {
        B { name as b } => {
            F { name as <destructure_name> } => `{a}_{b}_{f}`
        }
    }
}
```

Remake the path underneath the root `T`, it will contain exactly one component (as the template contains no slashes), the files oldest non-root ancestor component will be inserted at `{a}`, second oldest at `{b}` and own component part will be inserted at `{f}` to generate its new path.

# Fold Validation

Although we cannot identify prototypes in the layers of a fold statement, we must identify a tree prototype at its top level, which will be used to validate the dataset matches that prototype.

# Prototypes

The language has several built in prototypes,

* Built in prototypes
    * Directory: Node prototype matching any directory
    * File: Node prototype matching any file
        * Directory<r"regex"> and File<r"regex"> matches nodes of the corresponding type with names that match the given regex.
    * Star<k>: Tree prototype matching k-1 layers of directories and 1 layer of files.

We can now immediately rewrite our old fold code to work using `Star<k>`
```
fold "T": Star<3> {
    A { name as a } => {
        B { name as b } => {
            F { name as <destructure_name> } => `{a}_{b}_{f}`
        }
    }
}
```
Successfully runs!

# User Defined Prototypes

We can also define our own tree prototypes, which are composed of node prototypes for their layers.

```
TreeDirectorySet SmallTreeDir {
    Names: r"T",
    Structure: {
        layers: {
            A: Directory,
            B: Directory
        },
        edges: {
            F: File
        }
    }
}
```

Defines the identifer `SmallTreeDir` as a new tree prototype, which we can see here is equivalent to `Star<3>`.

We can define our own node prototypes that only match files or directories with names matching particular regex.

```
DirectorySet LayerA {
    Names: r"A"
}

DirectorySet LayerB {
    Names: r"B"
}

FileSet EdgeF {
    Names: r"F"
}

TreeDirectorySet SmallTreeDir {
    Names: r"T",
    Structure: {
        layers: {
            A: LayerA,
            B: LayerB
        },
        edges: {
            F: EdgeF
        }
    }
}
```

This tree prototype now matches exactly our example structure and nothing else!

But as mentioned before this functionality can be replicated more concisely using our built in parameterised node prototypes.

```
TreeDirectorySet SmallTreeDir {
    Names: r"T",
    Structure: {
        layers: {
            A: Directory<r"A">,
            B: Directory<r"B">
        },
        edges: {
            F: Directory<r"F">
        }
    }
}
```

# Full Application

We can now apply a fold to our test dataset with absolute confidence of its structure.

```
TreeDirectorySet SmallTreeDir {
    Names: r"T",
    Structure: {
        layers: {
            A: Directory<r"A">,
            B: Directory<r"B">
        },
        edges: {
            F: Directory<r"F">
        }
    }
}

fold "T": SmallTreeDir {
    A { name as a } => {
        B { name as b } => {
            F { name as <destructure_name> } => `{a}_{b}_{f}`
        }
    }
}
```
We can add flexibility to our structure as required by altering our regex, and change it fundamentally by adding and removing layers.

Since our nesting in fold has no semantic meaning besides sequence we can actually replace it using the `;` operator.

```
TreeDirectorySet SmallTreeDir {
    Names: r"T",
    Structure: {
        layers: {
            A: Directory<r"A">,
            B: Directory<r"B">
        },
        edges: {
            F: Directory<r"F">
        }
    }
}

fold "T": SmallTreeDir {
    A { name as a };
    B { name as b };
    F { name as <destructure_name> } => `{a}_{b}_{f}`
}
```
