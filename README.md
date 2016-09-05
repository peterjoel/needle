# needle
Search algorithms written in Rust

Currently, only Boyer-Moore is supported, it works only with `u8` arrays and is not particularly optimised. Having 
said that, when you only need to search in bytes, without special consideration for unicode characters, this 
implementation is often faster than the Rust standard library's `&str::find()`.

# Example Usage
```Rust
use needle::BoyerMoore;
let needle = BoyerMoore::new(b"Peter Piper");
let haystack = b"Peter Piper picked a peck of pickled peppers.\
                 A peck of pickled peppers Peter Piper picked.\
                 If Peter Piper picked a peck of pickled peppers,\
                 Where's the peck of pickled peppers Peter Piper picked?";
for i in needle.find_in(haystack) {
    println!("Found Peter Piper at index {}.", i);
}
```
