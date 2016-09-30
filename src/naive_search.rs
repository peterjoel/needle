use super::SearchIn;

pub struct NaiveSearch <'a> {
    needle: &'a [u8]
}


impl <'a> NaiveSearch <'a> {
    pub fn new(needle: &'a [u8]) -> NaiveSearch {
        NaiveSearch { 
            needle: needle
        }
    }
}


impl <'a> SearchIn<'a, [u8]> for NaiveSearch<'a> {
    type Iter = NaiveSearchIter<'a>;

    fn find_in(&'a self, haystack: &'a [u8]) -> NaiveSearchIter<'a> {
        NaiveSearchIter {
            searcher: &self,
            haystack: haystack,
            position: 0,
            overlapping_matches: false,
        }
    }

    fn find_overlapping_in(&'a self, haystack: &'a [u8]) -> NaiveSearchIter<'a> {
        NaiveSearchIter {
            searcher: &self,
            haystack: &haystack,
            position: 0,
            overlapping_matches: true
        }
    }
}


pub struct NaiveSearchIter <'a> {
    searcher: &'a NaiveSearch<'a>,
    haystack: &'a [u8],
    position: usize,
    overlapping_matches: bool,
}



fn find_from_position<'a>(&NaiveSearch { needle }: &'a NaiveSearch, haystack: &'a [u8], position: usize) -> Option<usize> {
    (position .. haystack.len() - needle.len() + 1)
        .find( |needle_pos| {
            (0 .. needle.len()).all(|needle_index| {
                needle[needle_index] == haystack[needle_index + needle_pos]
            })
        })
}


impl <'a> Iterator for NaiveSearchIter<'a> {
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
        let needle = NaiveSearch::new(b"ghi");
        let haystack = b"abc def ghi jkl";
        assert_eq!(Some(8), needle.find_first_in(haystack));
    }


    #[test]
    pub fn test_bad_char() {
        let haystack = b"acacacababadabacacad";
        assert_eq!(Some(12), NaiveSearch::new(b"abacac").find_first_in(haystack));
    }
}