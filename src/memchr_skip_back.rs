use skip_search::*;
use super::SearchIn;
use memchr::memrchr;
use memchr::memchr;

pub struct MemchrSkipBack <'a> {
    needle: &'a [u8],
    bad_chars: [usize; 256],
}


impl <'a> MemchrSkipBack <'a> {
    pub fn new(needle: &'a [u8]) -> MemchrSkipBack {
        MemchrSkipBack { 
            needle: needle,
            bad_chars: build_bad_chars_table(&needle),
        }
    }
}


impl <'a> SearchIn<'a, [u8]> for MemchrSkipBack<'a> {
    type Iter = MemchrSkipBackIter<'a>;
    fn find_in(&'a self, haystack: &'a [u8]) -> MemchrSkipBackIter<'a> {
        MemchrSkipBackIter {
            searcher: &self,
            haystack: haystack,
            position: 0,
            overlapping_matches: false,
        }
    }
    fn find_overlapping_in(&'a self, haystack: &'a [u8]) -> MemchrSkipBackIter<'a> {
        MemchrSkipBackIter {
            searcher: &self,
            haystack: &haystack,
            position: 0,
            overlapping_matches: true
        }
    }
}


impl <'a> SkipSearch<u8> for &'a MemchrSkipBack <'a> {
    #[inline]
    fn skip_offset(&self, bad_char: u8, _needle_pos: usize, haystack: &[u8], haystack_position: usize) -> usize {
        // let skip = self.bad_chars[bad_char as usize];
        // if skip == self.needle.len() {
            let last_needle_char = self.needle[self.needle.len() - 1];
            let start_pos = haystack_position + self.needle.len();// + skip;
            memchr(last_needle_char, &haystack[start_pos .. ]).unwrap_or(haystack.len())
        // } else {
        //     skip
        // }
    }

    // #[inline]
    // fn skip_offset(&self, bad_char: u8, needle_pos: usize, haystack: &[u8], haystack_position: usize) -> usize {
    //     let skip = self.bad_chars[bad_char as usize];
    //     if skip == self.needle.len() {
    //         let last_needle_char = self.needle[self.needle.len() - 1];
    //         memchr(last_needle_char, &haystack[haystack_position + skip ..]).unwrap_or(haystack.len())
    //     } else {
    //         skip
    //     }
    // }

    #[inline]
    fn len(&self) -> usize {
        self.needle.len()
    }

    #[inline]
    fn char_at(&self, index: usize) -> u8 {
        self.needle[index]
    }
}

pub struct MemchrSkipBackIter <'a> {
    searcher: &'a MemchrSkipBack<'a>,
    haystack: &'a [u8],
    position: usize,
    overlapping_matches: bool,
}


impl <'a> Iterator for MemchrSkipBackIter<'a> {
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
