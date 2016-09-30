use memchr::memchr;
use std::cmp::max;
use skip_search::*;
use super::SearchIn;

pub struct BoyerMooreMemchr <'a> {
    needle: &'a [u8],
    bad_chars: [usize; 256],
    good_suffixes: Vec<usize>
}

impl <'a> BoyerMooreMemchr <'a> {
    pub fn new(needle: &'a [u8]) -> BoyerMooreMemchr {
        BoyerMooreMemchr { 
            needle: needle,
            bad_chars: build_bad_chars_table(&needle),
            good_suffixes: build_good_suffixes_table(&needle)
        }
    }
}


impl <'a> SearchIn<'a, [u8]> for BoyerMooreMemchr<'a> {
    type Iter = BoyerMooreMemchrIter<'a>;

    fn find_in(&'a self, haystack: &'a [u8]) -> BoyerMooreMemchrIter<'a> {
        BoyerMooreMemchrIter {
            searcher: &self,
            haystack: haystack,
            position: 0,
            overlapping_matches: false,
        }
    }

    fn find_overlapping_in(&'a self, haystack: &'a [u8]) -> BoyerMooreMemchrIter<'a> {
        BoyerMooreMemchrIter {
            searcher: &self,
            haystack: &haystack,
            position: 0,
            overlapping_matches: true
        }
    }
}


impl <'a> SkipSearch<u8> for &'a BoyerMooreMemchr <'a> {
    #[inline]
    default fn skip_offset(&self, bad_char: u8, needle_position: usize, haystack: &[u8], haystack_position: usize) -> usize {
        let skip = max(self.bad_chars[bad_char as usize], self.good_suffixes[needle_position]);
        if skip < self.needle.len() {
            skip
        } else {
            let last_char = self.needle[self.needle.len() - 1];
            let search_position = haystack_position + 2 * self.needle.len() - 1;
            memchr(last_char, &haystack[search_position .. ]).map(|x| x + 1).unwrap_or(haystack.len())
        }
    }

    #[inline]
    fn len(&self) -> usize {
        self.needle.len()
    }

    #[inline]
    fn char_at(&self, index: usize) -> u8 {
        self.needle[index]
    }
}



pub struct BoyerMooreMemchrIter <'a> {
    searcher: &'a BoyerMooreMemchr<'a>,
    haystack: &'a [u8],
    position: usize,
    overlapping_matches: bool,
}

impl <'a> Iterator for BoyerMooreMemchrIter<'a> {
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
        let needle = BoyerMooreMemchr::new(b"ghi");
        let haystack = b"abc def ghi jkl";
        assert_eq!(Some(8), needle.find_first_in(haystack));
    }


    #[test]
    pub fn test_bad_char() {
        let haystack = b"acacacababadabacacad";
        assert_eq!(Some(12), BoyerMooreMemchr::new(b"abacac").find_first_in(haystack));
    }


    #[test]
    pub fn test_bad_char2() {
        let needle = BoyerMooreMemchr::new(b"abacab");
        let haystack = b"acacacababadabacabad";
        assert_eq!(Some(12), needle.find_first_in(haystack));
    }

    #[test]
    pub fn test_search_twice() {
        let needle = BoyerMooreMemchr::new(b"xyz");
        let haystack = b"01xyzxyz901xyz56xyz";
        assert_eq!(Some(2), needle.find_first_in(haystack));
        assert_eq!(Some(2), needle.find_first_in(haystack));
    }


    #[test]
    pub fn test_iter() {
        let needle = BoyerMooreMemchr::new(b"xyz");
        let haystack = b"01xyzxyz890xyz45xyz";
        assert_eq!(vec![2,5,11,16], needle.find_in(haystack).collect::<Vec<usize>>());
    }


    #[test]
    pub fn test_overlapping() {
        let needle = BoyerMooreMemchr::new(b"aaba");
        let haystack = b"aabaabaabaabaaba";
        assert_eq!(vec![0,3,6,9,12], needle.find_overlapping_in(haystack).collect::<Vec<usize>>());
    }

    #[test]
    pub fn test_non_overlapping() {
        let needle = BoyerMooreMemchr::new(b"aaba");
        let haystack = b"aabaabaabaabaaba";
        assert_eq!(vec![0,6,12], needle.find_in(haystack).collect::<Vec<usize>>());
    }

    #[test]
    pub fn test_occurs_in() {
        let needle = BoyerMooreMemchr::new(b"abc");
        let haystack = b"xxxxxxabcxxxxabc";
        assert_eq!(true, needle.occurs_in(haystack));
    }


    #[test]
    pub fn test_not_occurs_in() {
        let needle = BoyerMooreMemchr::new(b"abc");
        let haystack = b"xxxxxxabacxxxxaba";
        assert_eq!(false, needle.occurs_in(haystack));
    }


    #[test]
    pub fn test_count() {
        let needle = BoyerMooreMemchr::new(b"sea");
        let haystack = b"She sells sea shells on the sea shore.";
        assert_eq!(2, needle.count_in(haystack));
    }
}
