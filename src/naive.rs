
use super::traits::Search;

pub struct NaiveSearch <'a> {
    needle: &'a [u8]
}

impl <'a> NaiveSearch <'a> {

    pub fn new(needle: &'a [u8]) -> NaiveSearch {
        NaiveSearch { needle: needle }
    }
}

impl <'a> Search<'a> for NaiveSearch <'a> {

    fn first_index_of(&self, haystack: &'a [u8]) -> Option<usize> {
        'outer: for x in 0 .. (haystack.len() - self.needle.len()) {
            for y in 0 .. self.needle.len() {
                if haystack[x + y] != self.needle[y] {
                    continue 'outer;
                }
            }
            return Some(x);
        }
        None
    }
}



#[cfg(test)]
pub mod test {

    use super::NaiveSearch;
    use traits::Search;

    #[test]
    pub fn test_simple() {
        let needle = NaiveSearch::new(&"ghi".as_bytes());
        let haystack = "abc def ghi jkl".as_bytes();
        assert_eq!(Some(8), needle.first_index_of(&haystack));
    }
}