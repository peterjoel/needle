# needle
Search algorithms written in Rust

Boyer-Moore and BM-Horspool are supported, and can be used to search in arrays of any `Copy` type, with a few restrictions.

When you only need to search in bytes, without special consideration for unicode characters, this 
implementation is often faster than the Rust standard library's `&str::find()`.

# Example Usage
```Rust
use needle::BoyerMoore;
let haystack = b"Peter Piper picked a peck of pickled peppers.\
                 A peck of pickled peppers Peter Piper picked.\
                 If Peter Piper picked a peck of pickled peppers,\
                 Where's the peck of pickled peppers Peter Piper picked?";
let needle = BoyerMoore::new(b"Peter Piper");
for i in needle.find_in(haystack) {
    println!("Found Peter Piper at index {}.", i);
}
```
