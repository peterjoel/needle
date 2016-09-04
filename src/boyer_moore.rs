use std::collections::BTreeMap;
use super::traits::Search;

pub struct BoyerMoore <'a> {
    needle: &'a [u8],
    bad_chars: BTreeMap<u8, usize>,
}

impl <'a> BoyerMoore <'a> {

    pub fn new(needle: &'a [u8]) -> BoyerMoore {
        BoyerMoore { 
            needle: needle,
            bad_chars: build_bad_chars_map(&needle),
        }
    }
}


fn build_bad_chars_map(needle: &[u8]) -> BTreeMap<u8, usize> {
    let mut map = BTreeMap::new();
    let mut n = needle.len() - 1;
    while n != 0 {
        n -= 1;
        let c = needle[n];
        if !map.contains_key(&c) {
            map.insert(c, needle.len() - n - 1);
        }
    }
    map
}

impl <'a> Search<'a> for BoyerMoore <'a> {

    fn first_index_of(&self, haystack: &'a [u8]) -> Option<usize> {
        let mut i = self.needle.len() - 1;
        'outer: while i < haystack.len() {
            let mut j = self.needle.len();
            let mut k = i + 1;
            while j != 0 {
                j -= 1;
                k -= 1;
                if self.needle[j] != haystack[k] {
                    i += *self.bad_chars.get(&haystack[k]).unwrap_or(&self.needle.len());
                    continue 'outer;
                }
            }
            return Some(i - self.needle.len() + 1);
        }
        None
    }
}



#[cfg(test)]
pub mod test {

    use super::BoyerMoore;
    use traits::Search;

    #[test]
    pub fn test_simple() {
        let needle = BoyerMoore::new(&"ghi".as_bytes());
        let haystack = "abc def ghi jkl".as_bytes();
        assert_eq!(Some(8), needle.first_index_of(&haystack));
    }


    #[test]
    pub fn test_bad_char() {
        let needle = BoyerMoore::new(&"abacac".as_bytes());
        let haystack = "acacacababadabacacad".as_bytes();
        assert_eq!(Some(12), needle.first_index_of(&haystack));
    }
}