//! Implementation of the Boyer-Moore search algorithm, currently confined to `u8` arrays for input.
//!
//! # Examples
//!
//! ```
//! use needle::{BoyerMoore, SearchIn};
//! let needle = BoyerMoore::new(b"example");
//! let haystack = b"This is an example of searching for a word";
//! assert_eq!(Some(11), needle.find_in(haystack).next());
//! ```
use std::cmp::max;
use skip_search::*;
use super::SearchIn;

pub struct BoyerMoore <'a, T:'a> {
    needle: &'a [T],
    bad_chars: [usize; 256],
    good_suffixes: Vec<usize>
}

impl <'a, T> BoyerMoore <'a, T>
    where T: Copy + PartialEq + Into<usize>
{
    /// Construct a new Boyer-Moore search object, and pre-compute the skip tables.
    /// If you intend to search for the same needle in multiple haystacks, it is more
    /// efficient to create just one instance and then re-use it."
    pub fn new(needle: &'a [T]) -> BoyerMoore<T> {
        BoyerMoore { 
            needle: needle,
            bad_chars: build_bad_chars_table(&needle),
            good_suffixes: build_good_suffixes_table(&needle)
        }
    }
}


impl <'a, T> SearchIn<'a, [T]> for BoyerMoore<'a, T>
    where T: Copy + PartialEq + Into<usize>
{
    type Iter = BoyerMooreIter<'a, T>;

    /// Returns an iterator that will produce the indices of the needle in the haystack.
    /// This iterator will not find overlapping matches; the first character of a match 
    /// will start after the last character of the previous match.
    ///
    /// # Example
    /// ```
    /// use needle::{BoyerMoore, SearchIn};
    /// let needle = BoyerMoore::new(b"aaba");
    /// let haystack = b"aabaabaabaabaaba";
    /// assert_eq!(vec![0,6,12], needle.find_in(haystack).collect::<Vec<usize>>());
    /// ```
    fn find_in(&'a self, haystack: &'a [T]) -> BoyerMooreIter<'a, T> {
        BoyerMooreIter {
            searcher: &self,
            haystack: haystack,
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
    /// use needle::{BoyerMoore, SearchIn};
    /// let needle = BoyerMoore::new(b"aaba");
    /// let haystack = b"aabaabaabaabaaba";
    /// assert_eq!(vec![0,3,6,9,12], needle.find_overlapping_in(haystack).collect::<Vec<usize>>());
    /// ```
    fn find_overlapping_in(&'a self, haystack: &'a [T]) -> BoyerMooreIter<'a, T> {
        BoyerMooreIter {
            searcher: &self,
            haystack: &haystack,
            position: 0,
            overlapping_matches: true
        }
    }
}


impl <'a, T> SkipSearch<T> for &'a BoyerMoore <'a, T>
    where T: Copy + Into<usize>
{
    #[inline]
    fn skip_offset(&self, bad_char: T, needle_position: usize) -> usize {
        max(self.bad_chars[bad_char.into()], self.good_suffixes[needle_position])
    }

    #[inline]
    fn len(&self) -> usize {
        self.needle.len()
    }

    #[inline]
    fn char_at(&self, index: usize) -> T {
        self.needle[index]
    }
}

pub struct BoyerMooreIter <'a, T:'a> {
    searcher: &'a BoyerMoore<'a, T>,
    haystack: &'a [T],
    position: usize,
    overlapping_matches: bool,
}

impl <'a, T> Iterator for BoyerMooreIter<'a, T> 
    where T: Copy + PartialEq + Into<usize>
{
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        find_from_position(&self.searcher, &self.haystack, self.position)
            .and_then(|position| {
                if self.overlapping_matches {
                    self.position = position + 1;
                } else {
                    self.position = position + self.searcher.needle.len();
                }
                Some(position)
            })
    }
}


#[cfg(test)]
pub mod test {
    use super::*;
    use super::super::{SearchIn, CountIn};

    #[test]
    pub fn test_simple() {
        let needle = BoyerMoore::new(b"ghi");
        let haystack = b"abc def ghi jkl";
        assert_eq!(Some(8), needle.find_first_in(haystack));
    }


    #[test]
    pub fn test_bad_char() {
        let haystack = b"acacacababadabacacad";
        assert_eq!(Some(12), BoyerMoore::new(b"abacac").find_first_in(haystack));
    }


    #[test]
    pub fn test_bad_char2() {
        let needle = BoyerMoore::new(b"abacab");
        let haystack = b"acacacababadabacabad";
        assert_eq!(Some(12), needle.find_first_in(haystack));
    }

    #[test]
    pub fn test_search_twice() {
        let needle = BoyerMoore::new(b"xyz");
        let haystack = b"01xyzxyz901xyz56xyz";
        assert_eq!(Some(2), needle.find_first_in(haystack));
        assert_eq!(Some(2), needle.find_first_in(haystack));
    }


    #[test]
    pub fn test_iter() {
        let needle = BoyerMoore::new(b"xyz");
        let haystack = b"01xyzxyz890xyz45xyz";
        assert_eq!(vec![2,5,11,16], needle.find_in(haystack).collect::<Vec<usize>>());
    }


    #[test]
    pub fn test_overlapping() {
        let needle = BoyerMoore::new(b"aaba");
        let haystack = b"aabaabaabaabaaba";
        assert_eq!(vec![0,3,6,9,12], needle.find_overlapping_in(haystack).collect::<Vec<usize>>());
    }

    #[test]
    pub fn test_non_overlapping() {
        let needle = BoyerMoore::new(b"aaba");
        let haystack = b"aabaabaabaabaaba";
        assert_eq!(vec![0,6,12], needle.find_in(haystack).collect::<Vec<usize>>());
    }

    #[test]
    pub fn test_occurs_in() {
        let needle = BoyerMoore::new(b"abc");
        let haystack = b"xxxxxxabcxxxxabc";
        assert_eq!(true, needle.occurs_in(haystack));
    }


    #[test]
    pub fn test_not_occurs_in() {
        let needle = BoyerMoore::new(b"abc");
        let haystack = b"xxxxxxabacxxxxabc";
        assert_eq!(true, needle.occurs_in(haystack));
    }


    #[test]
    pub fn test_count() {
        let needle = BoyerMoore::new(b"sea");
        let haystack = b"She sells sea shells on the sea shore.";
        assert_eq!(2, needle.count_in(haystack));
    }
}
