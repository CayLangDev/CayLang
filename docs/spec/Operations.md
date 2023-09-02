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
from f"~" fold NumDataSet {
  root => {
    nums => {
      files => `{root}/{nums}_{files}`
    }
  }
}
```

...

```
TreeDirectorySet NumCatDataSet {
    Names: f"dataset",
    Structure: {
        layers {
            Nums: SmallNumDir,
            Cat: Dir<r"[ABCD]">
            Files: File
        }
    }
}
```

```
from f"~" fold NumCatDataSet {
  root => {
    num => {
      r"[A-B]" cat => {
        file => `{root}/{cat}_{num}_{file}`
      },
      r"C" => {
        r"*.txt" file => `{root}/C_{num}_{file}`,
        r"*.json" file => `{root}/CC_{num}_{file}`,
      }
    }
  }
}
```


```
from f"~" fold NumCatDataSet {
  root => {
    r"1" => {
        cat => {
            file => `{root}/{cat}_{num}_{file}`
        }
    }
    r"2" => {
        cat => {
            file => `{root}/{cat}{cat}_{num}_{file}`
        }
    }
    r"3" => {
        r"[A-B]" cat => {
            file => `{root}/{cat}{cat}{cat}_{num}_{file}`
        },
        r"C" => {
            r"*.txt" file => `{root}/C_{num}_{file}`,
            r"*.json" file => `{root}/CCC_{num}_{file}`,
        }
    }
  }
}
```
