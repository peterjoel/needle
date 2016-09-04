# needle
Search algorithms written in Rust

Currently, Boyer-Moore is supported, though only working with `u8` arrays and not particularly optimised.

# Example Usage
```Rust
use needle::BoyerMoore;
use needle::Search;
let needle = BoyerMoore::new("example".as_bytes());
let haystack = "This is an example of searching for a word".as_bytes();
assert_eq!(Some(11), needle.first_index(&haystack));
```
