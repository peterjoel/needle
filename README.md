# needle
Search algorithms written in Rust

Boyer-Moore and BM-Horspool are supported, and can be used to search in arrays of any `Copy` type, with a few restrictions.

When you only need to search in bytes, without special consideration for unicode characters, this 
implementation is often faster than the Rust standard library's `&str::find()`.

# Examples

The interfaces for BoyerMoore and Horspool are essentially the same. This example uses Boyer-Moore to find all instances of
"Peter Piper" in the text.

```Rust
use needle::BoyerMoore;
let haystack = b"Peter Piper picked a peck of pickled peppers.\
                 A peck of pickled peppers Peter Piper picked.\
                 If Peter Piper picked a peck of pickled peppers,\
                 Where's the peck of pickled peppers Peter Piper picked?";
let needle = BoyerMoore::new(&b"Peter Piper"[..]);
for i in needle.find_in(haystack) {
    println!("Found Peter Piper at index {}.", i);
}
```

In general, the fastest searches are over bytes. But you can search other alphabets if it's convenient. For example:

```Rust
use needle::Horspool;

#[derive(Copy, Clone, Debug, PartialEq)]
enum Nucleotide {
    A, T, C, G
}

// An Into<usize> impl is required for the search alphabet
impl Into<usize> for Nucleotide {
    #[inline]
    fn into(self) -> usize { self as usize }
}

// Convenience to create an RNA chain from a string representation
fn from_str(other: &[u8]) -> Vec<Nucleotide> {
    other.into_iter().map( |&c| {
        match c.into() {
            b'A' => Nucleotide::A,
            b'T' => Nucleotide::T,
            b'C' => Nucleotide::C,
            b'G' => Nucleotide::G,
            _ => panic!("Unknown nucleotide {:?}", &c),
        }
    }).collect()
}

fn main() {
    let haystack = from_str(b"ACCTGATCGGGTGGTACACGATAATATCGTGGCATGCACTTGCTGATCGCTTAGACTGCAAAATCGTAGCCAGTAGGT");
    let haystack = haystack.as_slice();
    let subsequence = &[Nucleotide::C, Nucleotide::G, Nucleotide::C, Nucleotide::T][..];
    let needle = Horspool::new(subsequence);
    assert!(needle.find_first_in(haystack).is_some());
}
```