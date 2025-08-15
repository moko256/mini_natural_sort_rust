## mini_natural_sort

Simple no-std natural (human-readable) sort crate.

### Examples

```rust
use mini_natural_sort::{compare_str_natural, NaturalSortKey};

// Sorting list by function.
let mut filenames = ["b001.txt", "b1.txt", "b000002.txt", "A.txt", "a.txt", "c.txt", "c_copy (1).txt"];

filenames.sort_by(compare_str_natural);

let sorted = ["a.txt", "b1.txt", "b001.txt", "b000002.txt", "c.txt", "c_copy (1).txt", "A.txt"];
assert_eq!(filenames, sorted);

// Comparing two strings by struct.
assert!(NaturalSortKey::from("0000001") < NaturalSortKey::from("2"));
```

### Install

```toml
[dependencies]
mini_natural_sort = { git = "https://github.com/moko256/mini_natural_sort_rust.git", tag = "v2.0.1" }
```

### License

SPDX-License-Identifier: MIT