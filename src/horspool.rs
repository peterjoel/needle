//! Implementation of the Boyer-Moore-Hospool search algorithm
//!
//! # Examples
//!
//! ```
//! use needle::BoyerMoore;
//! let needle = BoyerMoore::new(b"example");
//! let haystack = b"This is an example of searching for a word";
//! assert_eq!(Some(11), needle.find_in(haystack).next());
//! ```
use skip_search::*;

pub struct Horspool <'a, T:'a> {
    needle: &'a [T],
    bad_chars: [usize; 256],
}


impl <'a, T> Horspool <'a, T>
    where T: Copy + PartialEq + Into<usize>
{
    pub fn new(needle: &'a [T]) -> Horspool<T> {
        Horspool { 
            needle: needle,
            bad_chars: build_bad_chars_table(&needle),
        }
    }

    pub fn first_index<'b>(&'b self, haystack: &'b [T]) -> Option<usize> {
        self.find_in(&haystack).next()
    }


    /// Returns an iterator that will produce the indices of the needle in the haystack.
    /// This iterator will not find overlapping matches; the first character of a match 
    /// will start after the last character of the previous match.
    ///
    /// # Example
    /// ```
    /// use needle::Horspool;
    /// let needle = Horspool::new(b"aaba");
    /// let haystack = b"aabaabaabaabaaba";
    /// assert_eq!(vec![0,6,12], needle.find_in(haystack).collect::<Vec<usize>>());
    /// ```
    pub fn find_in<'b>(&'b self, haystack: &'b [T]) -> HorspoolIter<T> {
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
    /// use needle::Horspool;
    /// let needle = Horspool::new(b"aaba");
    /// let haystack = b"aabaabaabaabaaba";
    /// assert_eq!(vec![0,3,6,9,12], needle.find_overlapping_in(haystack).collect::<Vec<usize>>());
    /// ```
    pub fn find_overlapping_in<'b>(&'b self, haystack: &'b [T]) -> HorspoolIter<T> {
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
    fn skip_offset(&self, bad_char: T, _: usize) -> usize {
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
