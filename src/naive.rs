

pub struct NaiveSearch <'a> {
    needle: &'a [u8]
}

impl <'a> NaiveSearch <'a> {

    pub fn new(needle: &'a [u8]) -> NaiveSearch {
        NaiveSearch { needle: needle }
    }

    fn find_from_position(&self, haystack: &'a [u8], position: usize) -> Option<usize> {
        'outer: for x in position .. (haystack.len() - self.needle.len()) {
            for y in 0 .. self.needle.len() {
                if haystack[x + y] != self.needle[y] {
                    continue 'outer;
                }
            }
            return Some(x);
        }
        None
    }

    pub fn first_index<'b>(&'b self, haystack: &'b [u8]) -> Option<usize> {
        self.find_in(&haystack).next()
    }

    pub fn find_in<'b>(&'b self, haystack: &'b [u8]) -> NaiveSearchIter {
        NaiveSearchIter {
            searcher: &self,
            haystack: &haystack,
            position: 0,
        }
    }
}

pub struct NaiveSearchIter<'a> {
    searcher: &'a NaiveSearch<'a>,
    haystack: &'a [u8],
    position: usize,
}


impl <'a> Iterator for NaiveSearchIter<'a> {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        self.searcher
            .find_from_position(&self.haystack, self.position)
            .and_then(|position| {
                self.position = position + 1;
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