## mini_natural_sort

Simple no-std natural (human-readable) sort crate.

### Examples

```rust
use mini_natural_sort::NaturalSortKey;

let mut filenames = [ "b001.txt", "b1.txt", "b000002.txt", "A.txt", "a.txt", "c.txt", "c_copy (1).txt" ];
filenames.sort_by_key(|v| NaturalSortKey::from_str(v));
```

### Install

```toml
[dependencies]
mini_natural_sort = { git = "https://github.com/moko256/mini_natural_sort_rust.git", tag = "v1.0.0" }
```

### License

SPDX-License-Identifier: MIT