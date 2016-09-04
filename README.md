# needle
Search algorithms written in Rust

Currently, only Boyer-Moore is supported, it works only with `u8` arrays and is not particularly optimised. Having 
said that, when you only need to search in bytes, without special consideration for unicode characters, this 
implementation is often faster than the Rust standard library's `&str::find()`.

# Example Usage
```Rust
use needle::BoyerMoore;
use needle::Search;
let needle = BoyerMoore::new("example".as_bytes());
let haystack = "This is an example of searching for a word".as_bytes();
assert_eq!(Some(11), needle.first_index(&haystack));
```
