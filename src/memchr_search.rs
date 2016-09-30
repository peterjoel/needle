use memchr::memchr;
use super::SearchIn;

pub struct MemchrSearch <'a> {
    needle: &'a [u8]
}


impl <'a> MemchrSearch <'a> {
    pub fn new(needle: &'a [u8]) -> MemchrSearch {
        MemchrSearch { 
            needle: needle
        }
    }
}


impl <'a> SearchIn<'a, [u8]> for MemchrSearch<'a> {
    type Iter = MemchrSearchIter<'a>;

    fn find_in(&'a self, haystack: &'a [u8]) -> MemchrSearchIter<'a> {
        MemchrSearchIter {
            searcher: &self,
            haystack: haystack,
            position: 0,
            overlapping_matches: false,
        }
    }

    fn find_overlapping_in(&'a self, haystack: &'a [u8]) -> MemchrSearchIter<'a> {
        MemchrSearchIter {
            searcher: &self,
            haystack: &haystack,
            position: 0,
            overlapping_matches: true
        }
    }
}


pub struct MemchrSearchIter <'a> {
    searcher: &'a MemchrSearch<'a>,
    haystack: &'a [u8],
    position: usize,
    overlapping_matches: bool,
}


fn find_from_position<'a>(&MemchrSearch { needle }: &'a MemchrSearch, haystack: &'a [u8], mut position: usize) -> Option<usize> {
    while position <= haystack.len() - needle.len() {
        match memchr(needle[0], &haystack[position ..]) {
            None => return None,
            Some(needle_pos) => {
                let needle_pos = needle_pos + position;
                if (1 .. needle.len()).all(|needle_index| {
                    needle[needle_index] == haystack[needle_index + needle_pos]
                }) {
                    return Some(needle_pos);
                } else {
                    position = needle_pos + 1;
                }
            }
        }
    }
    None
}


impl <'a> Iterator for MemchrSearchIter<'a> {
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
    use super::super::{SearchIn};

    #[test]
    pub fn test_simple() {
        let needle = MemchrSearch::new(b"ghi");
        let haystack = b"abc def ghi jkl";
        assert_eq!(Some(8), needle.find_first_in(haystack));
    }


    #[test]
    pub fn test_bad_char() {
        let haystack = b"acacacababadabacacad";
        assert_eq!(Some(12), MemchrSearch::new(b"abacac").find_first_in(haystack));
    }
}