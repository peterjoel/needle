//! Implementation of the Boyer-Moore-Horspool search algorithm, which is essentially
//! the Boyer-Moore algorithm but without the good suffix rule. In most common cases, 
//! it is faster than Boyer-Moore, but there are some pathological cases, involving 
//! densely repeating patterns, where the suffix rule actually offers an improvement.
//!
//! # Examples
//!
//! ```
//! use needle::{Horspool, SearchIn};
//! let needle = Horspool::new(b"example");
//! let haystack = b"This is an example of searching for a word";
//! assert_eq!(Some(11), needle.find_in(haystack).next());
//! ```
use skip_search::*;
use super::SearchIn;

pub struct Horspool <'a, T:'a> {
    needle: &'a [T],
    bad_chars: [usize; 256],
}


impl <'a, T> Horspool <'a, T>
    where T: Copy + PartialEq + Into<usize>
{
    /// Construct a new Horspool search object, and pre-compute the skip tables.
    /// If you intend to search for the same needle in multiple haystacks, it is more
    /// efficient to create just one instance and then re-use it."
    pub fn new(needle: &'a [T]) -> Horspool<T> {
        Horspool { 
            needle: needle,
            bad_chars: build_bad_chars_table(&needle),
        }
    }
}


impl <'a, T> SearchIn<'a, [T]> for Horspool<'a, T>
    where T: Copy + PartialEq + Into<usize>
{
    type Iter = HorspoolIter<'a, T>;

    /// Returns an iterator that will produce the indices of the needle in the haystack.
    /// This iterator will not find overlapping matches; the first character of a match 
    /// will start after the last character of the previous match.
    ///
    /// # Example
    /// ```
    /// use needle::{Horspool, SearchIn};
    /// let needle = Horspool::new(b"aaba");
    /// let haystack = b"aabaabaabaabaaba";
    /// assert_eq!(vec![0,6,12], needle.find_in(haystack).collect::<Vec<usize>>());
    /// ```
    fn find_in(&'a self, haystack: &'a [T]) -> HorspoolIter<'a, T> {
        HorspoolIter {
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
    /// use needle::{Horspool, SearchIn};
    /// let needle = Horspool::new(b"aaba");
    /// let haystack = b"aabaabaabaabaaba";
    /// assert_eq!(vec![0,3,6,9,12], needle.find_overlapping_in(haystack).collect::<Vec<usize>>());
    /// ```
    fn find_overlapping_in(&'a self, haystack: &'a [T]) -> HorspoolIter<'a, T> {
        HorspoolIter {
            searcher: &self,
            haystack: &haystack,
            position: 0,
            overlapping_matches: true
        }
    }
}


impl <'a, T> SkipSearch<T> for &'a Horspool <'a, T>
    where T: Copy + Into<usize>
{
    #[inline]
    fn skip_offset(&self, bad_char: T, _: usize, _haystack: &[T], _haystack_position: usize) -> usize {
        self.bad_chars[bad_char.into()]
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

pub struct HorspoolIter <'a, T:'a> {
    searcher: &'a Horspool<'a, T>,
    haystack: &'a [T],
    position: usize,
    overlapping_matches: bool,
}


impl <'a, T> Iterator for HorspoolIter<'a, T> 
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
