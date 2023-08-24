I'm thinking of some syntax ideas for folding/reducing over a directory, we could do some pattern matching over files, something like
```
fold dir/ {
    .*grades_2018.csv => grades/2018.csv
    .*grades_2019.csv => grades/2019.csv
}
```
So all files that match the pattern go to the file on the right, if multiple files go to the same outfile, they get appended to/merged with each other. Then from there can do our einops-like functions on the file data themselves. So the expressions on the right of the => could be mutation functions on the files. Something like

```
fold dir/ {
    .*grades_2018.csv => head 5 => grades/2018.csv
}
```

So every expression could start with a `<file> =>` and end with a `=> <file>`, to indicate reading in and out.
Using regex could give a lot of flexibility
- Jay Taylor

This looks good for folding across directories, would be happy to combine this with syntax for interacting with and carrying on directory structure as metadata as a base prototype.
and yeah definitely think regex is the way to go for matching.
Also I've just had the thought that XQuery and XPath might provide some useful implementation, as they're DSLs interacting with tree data structures (XML).
- William Saffery

If we wanted could even do like a typescript interface thing for describing directories, something like
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
So there can even be compile-time checks of your operations
- Jay Taylor

This looks good, but I'd like operations to reduce redundancy in these descriptions
```
DirectorySet Years: [0-9]{4}
DirectorySet Grades: {
  "Math",
  "Science",
  "Geography"
}

# ops: reduce_some, contains_some

Directory Dataset {
  reduce_some Grades (contains_some Years) 
}
```
- William Saffery


Mm yeah it wouldn't be good to describe each file manually, maybe define some pattern that each file has to match?
- Jay Taylor

So here Dataset is a fuzzy directory structure prototype that would provide an interface for a folder set of some grades folders each containing some years folders. Well, I've used pattern matching for years, the grades being manual is kind of impossible to avoid. Loading from a name file makes sense though.
- William Saffery

 
