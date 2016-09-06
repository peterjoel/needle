//! Naive search for comparison

use skip_search::*;

pub struct NaiveSearch <'a> {
    needle: &'a [u8]
}

impl <'a> NaiveSearch <'a> {

    pub fn new(needle: &'a [u8]) -> NaiveSearch {
        NaiveSearch { needle: needle }
    }

    pub fn first_index<'b>(&'b self, haystack: &'b [u8]) -> Option<usize> {
        self.find_in(&haystack).next()
    }

    pub fn find_in<'b>(&'b self, haystack: &'b [u8]) -> NaiveSearchIter {
        NaiveSearchIter {
            searcher: &self,
            haystack: &haystack,
            position: 0,
            overlapping_matches: false,
        }
    }
    
    pub fn find_overlapping_in<'b>(&'b self, haystack: &'b [u8]) -> NaiveSearchIter {
        NaiveSearchIter {
            searcher: &self,
            haystack: &haystack,
            position: 0,
            overlapping_matches: true,
        }
    }
}

impl <'a> SkipSearch for NaiveSearch<'a> {
    #[inline]
    fn skip_offset(&self, _: u8, _: usize) -> usize { 1 }

    #[inline]
    fn len(&self) -> usize {
        self.needle.len()
    }

    #[inline]
    fn char_at(&self, index: usize) -> u8 {
        self.needle[index]
    }
}

pub struct NaiveSearchIter<'a> {
    searcher: &'a NaiveSearch<'a>,
    haystack: &'a [u8],
    position: usize,
    overlapping_matches: bool,
}


impl <'a> Iterator for NaiveSearchIter<'a> {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        find_from_position(self.searcher, &self.haystack, self.position)
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

    use super::NaiveSearch;

    #[test]
    pub fn test_simple() {
        let needle = NaiveSearch::new(&"ghi".as_bytes());
        let haystack = "abc def ghi jkl".as_bytes();
        assert_eq!(Some(8), needle.first_index(&haystack));
    }
}