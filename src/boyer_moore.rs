use super::traits::Search;

pub struct BoyerMoore <'a> {
    needle: &'a [u8],
    bad_chars: [usize; 256],
}

impl <'a> BoyerMoore <'a> {

    pub fn new(needle: &'a [u8]) -> BoyerMoore {
        BoyerMoore { 
            needle: needle,
            bad_chars: build_bad_chars_map(&needle),
        }
    }

    #[inline]
    fn get_bad_char_offset(&self, bad_char: u8) -> usize {
        self.bad_chars[bad_char as usize]
    }
}


fn build_bad_chars_map(needle: &[u8]) -> [usize; 256] {
    let mut map = [needle.len(); 256];
    for i in 0 .. needle.len() - 1 {
        let c = needle[i] as usize;
        map[c] = needle.len() - i - 1;
    }
    map
}

impl <'a> Search<'a> for BoyerMoore <'a> {

    fn first_index_of(&self, haystack: &'a [u8]) -> Option<usize> {
        let mut position = 0;
        let max_position = haystack.len() - self.needle.len(); 
        while position <= max_position {
            let mut needle_position = self.needle.len() - 1;
            while haystack[position + needle_position] == self.needle[needle_position] {
                if needle_position == 0 {
                    return Some(position);
                } else {
                    needle_position -= 1;
                }
            }
            let bad_char = haystack[position + self.needle.len() - 1];
            position += self.get_bad_char_offset(bad_char);
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

    #[test]
    pub fn test_bad_char2() {
        let needle = BoyerMoore::new(&"abacab".as_bytes());
        let haystack = "acacacababadabacabad".as_bytes();
        assert_eq!(Some(12), needle.first_index_of(&haystack));
    }
}
