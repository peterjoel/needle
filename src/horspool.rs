//! Implementation of the Boyer-Moore-Horspool search algorithm, which is essentially
//! the Boyer-Moore algorithm but without the good suffix rule. In most common cases, 
//! it is faster than Boyer-Moore, but there are some pathological cases, involving 
//! densely repeating patterns, where the suffix rule actually offers an improvement.
//!
//! # Examples
//!
//! ```
//! use needle::Horspool;
//! let needle = Horspool::new(&b"example"[..]);
//! let haystack = b"This is an example of searching for a word";
//! assert_eq!(Some(11), needle.find_in(haystack).next());
//! ```
use skip_search::{build_bad_chars_table, find_from_position, Searchable, SkipSearch};

pub struct Horspool <'a, H:'a + ?Sized> {
    needle: &'a H,
    bad_chars: [usize; 256],
}


impl <'a, H: ?Sized, T> Horspool <'a, H>
    where T: 'a + Copy + PartialEq + Into<usize>,
          H: 'a + Searchable<Item = T>
{
    pub fn new(needle: &'a H) -> Horspool<H> {
        Horspool { 
            needle: *&needle,
            bad_chars: build_bad_chars_table(*&needle),
        }
    }

    /// Finds the first occurence of the search term in haystack and returns the index if it is found.
    pub fn find_first_in<'b>(&'b self, haystack: &'b H) -> Option<usize> {
        self.find_in(haystack).next()
    }

    /// Returns an iterator that will produce the indices of the needle in the haystack.
    /// This iterator will not find overlapping matches; the first character of a match 
    /// will start after the last character of the previous match.
    ///
    /// # Example
    /// ```
    /// use needle::Horspool;
    /// let needle = Horspool::new(&b"aaba"[..]);
    /// let haystack = b"aabaabaabaabaaba";
    /// assert_eq!(vec![0,6,12], needle.find_in(haystack).collect::<Vec<usize>>());
    /// ```
    pub fn find_in<'b>(&'b self, haystack: &'b H) -> HorspoolIter<T, H> {
        HorspoolIter {
            searcher: &self,
            haystack: &haystack,
            position: 0,
            overlapping_matches: false,
        }
    }

    /// Returns an iterator that will produce the indices of the needle in the haystack.
    /// This iterator will find overlapping matches; the first character of a match is 
    /// allowed to be matched from within the previous match.
    ///
    /// # Example
    /// ```
    /// use needle::Horspool;
    /// let needle = Horspool::new(&b"aaba"[..]);
    /// let haystack = b"aabaabaabaabaaba";
    /// assert_eq!(vec![0,3,6,9,12], needle.find_overlapping_in(haystack).collect::<Vec<usize>>());
    /// ```
    pub fn find_overlapping_in<'b>(&'b self, haystack: &'b H) -> HorspoolIter<T, H> {
        HorspoolIter {
            searcher: &self,
            haystack: &haystack,
            position: 0,
            overlapping_matches: true
        }
    }
}

impl <'a, H: ?Sized, T> SkipSearch<T> for Horspool <'a, H>
    where H: Searchable<Item = T>,
          T: Copy + Into<usize>
{
    #[inline]
    fn skip_offset(&self, bad_char: T, _: usize) -> usize {
        self.bad_chars[bad_char.into()]
    }

    #[inline]
    fn len(&self) -> usize {
        self.needle.num_items()
    }

    #[inline]
    fn char_at(&self, index: usize) -> T {
        self.needle.item_at(index)
    }
}

pub struct HorspoolIter <'a, T, H: ?Sized>
    where T: 'a,
          H: 'a + Searchable<Item = T>
{
    searcher: &'a Horspool<'a, H>,
    haystack: &'a H,
    position: usize,
    overlapping_matches: bool,
}


impl <'a, T, H: ?Sized> Iterator for HorspoolIter<'a, T, H>
    where T: Copy + PartialEq + Into<usize>,
          H: Searchable<Item = T>
{
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        find_from_position(self.searcher, self.haystack, self.position)
            .and_then(|position| {
                if self.overlapping_matches {
                    self.position = position + 1;
                } else {
                    self.position = position + self.searcher.needle.num_items();
                }
                Some(position)
            })
    }
}



#[cfg(test)]
mod test {
    use super::*;
    use std::str;
    use test::Bencher;

    #[derive(Copy, Clone, Debug, PartialEq)]
    enum Nucleotide {
        A, T, C, G
    }

    impl Into<usize> for Nucleotide {
        #[inline]
        fn into(self) -> usize { self as usize }
    }
    
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

    #[test]
    pub fn test_rna() {
        let haystack = from_str(b"ACCTGATCGGGTGGTACACGATAATATCGTGGCATGCACTTGCTGATCGCTTAGACTGCAAAATCGTAGCCAGTAGGT");
        let green_eyes = from_str(b"GCA");
        let needle = Horspool::new(green_eyes.as_slice());
        assert_eq!(vec![31, 35, 57], needle.find_in(&haystack.as_slice()).collect::<Vec<usize>>());

    }

    #[bench]
    pub fn bench_rna_enum(b: &mut Bencher) {
        let haystack_src = from_str(b"ACCTGATCGGGTGGTACACGATAATATCGTGGCATGCACTTGCTGATCGCTTAGACTGCAAAATCGTAGCCAGTAGGT");
        let haystack = haystack_src.as_slice();
        let green_eyes = from_str(b"GCA");
        let needle = Horspool::new(green_eyes.as_slice());
        b.iter(|| {
            assert_eq!(31, needle.find_first_in(haystack).unwrap())
        });
    }
    
    #[bench]
    pub fn bench_rna_bytes(b: &mut Bencher) {
        let haystack = b"ACCTGATCGGGTGGTACACGATAATATCGTGGCATGCACTTGCTGATCGCTTAGACTGCAAAATCGTAGCCAGTAGGT";
        let green_eyes = b"GCA";
        let needle = Horspool::new(&green_eyes[..]);
        b.iter(|| {
            assert_eq!(31, needle.find_first_in(haystack).unwrap())
        });
    }

}