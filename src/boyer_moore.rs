//! Implementation of the Boyer-Moore search algorithm, currently confined to `u8` arrays for input.
//!
//! # Examples
//!
//! ```
//! use needle::BoyerMoore;
//! let needle = BoyerMoore::new(&b"example"[..]);
//! let haystack = b"This is an example of searching for a word";
//! assert_eq!(Some(11), needle.find_in(haystack).next());
//! ```
use std::cmp::max;
use skip_search::{build_bad_chars_table, build_good_suffixes_table, find_from_position, Searchable, SkipSearch};

pub struct BoyerMoore <'a, H:'a + ?Sized> {
    needle: &'a H,
    bad_chars: [usize; 256],
    good_suffixes: Vec<usize>,
}


impl <'a, H: ?Sized, T> BoyerMoore <'a, H>
    where T: 'a + Copy + PartialEq + Into<usize>,
          H: 'a + Searchable<Item = T>
{
    /// Construct a new Boyer-Moore search object, and pre-compute the skip tables.
    /// If you intend to search for the same needle in multiple haystacks, it is more
    /// efficient to create just one instance and the re-use it."]
    pub fn new(needle: &'a H) -> BoyerMoore<'a, H> {
        BoyerMoore { 
            needle: needle,
            bad_chars: build_bad_chars_table(needle),
            good_suffixes: build_good_suffixes_table(needle),
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
    /// use needle::BoyerMoore;
    /// let needle = BoyerMoore::new(&b"aaba"[..]);
    /// let haystack = b"aabaabaabaabaaba";
    /// assert_eq!(vec![0,6,12], needle.find_in(haystack).collect::<Vec<usize>>());
    /// ```
    pub fn find_in<'b>(&'b self, haystack: &'b H) -> BoyerMooreIter<T, H> {
        BoyerMooreIter {
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
    /// use needle::BoyerMoore;
    /// let needle = BoyerMoore::new(&b"aaba"[..]);
    /// let haystack = b"aabaabaabaabaaba";
    /// assert_eq!(vec![0,3,6,9,12], needle.find_overlapping_in(haystack).collect::<Vec<usize>>());
    /// ```
    pub fn find_overlapping_in<'b>(&'b self, haystack: &'b H) -> BoyerMooreIter<T, H> {
        BoyerMooreIter {
            searcher: &self,
            haystack: &haystack,
            position: 0,
            overlapping_matches: true
        }
    }
}

impl <'a, H: ?Sized, T> SkipSearch<T> for BoyerMoore <'a, H>
    where H: Searchable<Item = T>,
          T: Copy + Into<usize>
{
    #[inline]
    fn skip_offset(&self, bad_char: T, needle_position: usize) -> usize {
        max(self.bad_chars[bad_char.into()], self.good_suffixes[needle_position])
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

pub struct BoyerMooreIter <'a, T, H: ?Sized>
    where T: 'a,
          H: 'a + Searchable<Item = T>
{
    searcher: &'a BoyerMoore<'a, H>,
    haystack: &'a H,
    position: usize,
    overlapping_matches: bool,
}

impl <'a, T, H: ?Sized> Iterator for BoyerMooreIter<'a, T, H>
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
pub mod test {

    use super::*;

    #[test]
    pub fn test_simple() {
        let needle = b"ghi";
        let needle = BoyerMoore::new(&needle[..]);
        let haystack = b"abc def ghi jkl";
        assert_eq!(Some(8), needle.find_first_in(haystack));
    }


    #[test]
    pub fn test_bad_char() {
        let haystack = b"acacacababadabacacad";
        assert_eq!(Some(12), BoyerMoore::new(&b"abacac"[..]).find_first_in(haystack));
    }


    #[test]
    pub fn test_bad_char2() {
        let needle = b"abacab";
        let needle = BoyerMoore::new(&needle[..]);
        let haystack = b"acacacababadabacabad";
        assert_eq!(Some(12), needle.find_first_in(haystack));
    }

    #[test]
    pub fn test_search_twice() {
        let needle = b"xyz";
        let needle = BoyerMoore::new(&needle[..]);
        let haystack = b"01xyzxyz901xyz56xyz";
        assert_eq!(Some(2), needle.find_first_in(haystack));
        assert_eq!(Some(2), needle.find_first_in(haystack));
    }


    #[test]
    pub fn test_iter() {
        let needle = b"xyz";
        let needle = BoyerMoore::new(&needle[..]);
        let haystack = b"01xyzxyz890xyz45xyz";
        assert_eq!(vec![2,5,11,16], needle.find_in(haystack).collect::<Vec<usize>>());
    }


    #[test]
    pub fn test_overlapping() {
        let needle = b"aaba";
        let needle = BoyerMoore::new(&needle[..]);
        let haystack = b"aabaabaabaabaaba";
        assert_eq!(vec![0,3,6,9,12], needle.find_overlapping_in(haystack).collect::<Vec<usize>>());
    }

    #[test]
    pub fn test_non_overlapping() {
        let needle = b"aaba";
        let needle = BoyerMoore::new(&needle[..]);
        let haystack = b"aabaabaabaabaaba";
        assert_eq!(vec![0,6,12], needle.find_in(haystack).collect::<Vec<usize>>());
    }


}
