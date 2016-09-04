//! Implementation of the Boyer-Moore search algorithm, currently confined to `u8` arrays for input.
//!
//! # Examples
//!
//! ```
//! use needle::BoyerMoore;
//! use needle::Search;
//! let needle = BoyerMoore::new("example".as_bytes());
//! let haystack = "This is an example of searching for a word".as_bytes();
//! assert_eq!(Some(11), needle.first_index(&haystack));
//! ```
use super::Search;
use std::cmp::max;

pub struct BoyerMoore <'a> {
    needle: &'a [u8],
    bad_chars: [usize; 256],
    good_suffixes: Vec<usize>,
}

impl <'a> BoyerMoore <'a> {
    pub fn new(needle: &'a [u8]) -> BoyerMoore {
        BoyerMoore { 
            needle: needle,
            bad_chars: build_bad_chars_table(&needle),
            good_suffixes: build_good_suffixes_table(&needle),
        }
    }

    #[inline]
    fn get_bad_char_offset(&self, bad_char: u8) -> usize {
        self.bad_chars[bad_char as usize]
    }

    #[inline]
    fn get_good_suffix_offset(&self, needle_position: usize) -> usize {
        self.good_suffixes[needle_position]
    }
}

// Bad characters table is used for when the last (rightmost) character of the needle doesn't match. The table
// gives the number of elements to skip, to find a character that does match.
fn build_bad_chars_table(needle: &[u8]) -> [usize; 256] {
    let mut table = [needle.len(); 256];
    for i in 0 .. needle.len() - 1 {
        let c = needle[i] as usize;
        table[c] = needle.len() - i - 1;
    }
    table
}

// Produces a table, whose indices are indices of needle, and whose entries are the size of 
// the largest suffix of needle that matches the substring ending at that index
fn get_suffix_table(needle: &[u8]) -> Vec<usize> {
    // The algorthm builds the table in steps as follows:
    // a b c b a b c a b a b | suffix (length)
    // --------------------- | ------
    // 0 1 0 1 0 1 0 0 1 0 1 |       b (1)
    // 0 2 0 1 0 2 0 0 2 0 2 |     a b (2)
    // 0 2 0 1 0 3 0 0 2 0 3 |   b a b (3)
    // 0 2 0 1 0 3 0 0 2 0 4 | a b a b (4)
    // etc..
    let len = needle.len();
    let mut suffixes = vec![0; len];
    for suffix_len in 1 .. needle.len() {
        let mut found_suffix = false;
        for i in (0 .. len - suffix_len).rev() {
            // either 0 or a previous match for a 1-smaller suffix
            if suffixes[i + suffix_len - 1] == suffix_len - 1 && needle[i] == needle[len - suffix_len] {
                suffixes[i + suffix_len - 1] = suffix_len;
                found_suffix = true;
            }
        }
        if !found_suffix {
            break;
        }
    }
    suffixes
}

// When a suffix of the needle matches, but fails at the next character, this table gives the number of 
// elements to skip, to find another subsequence that matches the suffix but with a different preceding character.
fn build_good_suffixes_table(needle: &[u8]) -> Vec<usize> {
    let suffixes = get_suffix_table(&needle);
    let len = needle.len();
    let mut table = vec![len - 1; len];

    for (i, suffix_len) in suffixes.into_iter().enumerate() {
        let needle_index = len - suffix_len - 1;
        let skip = len - i - 1;
        if table[needle_index] > skip {
            table[needle_index] = skip;
        }
    }
    table[len - 1] = 1;
    table
}


impl <'a> Search<'a> for BoyerMoore <'a> {
    fn first_index(&self, haystack: &'a [u8]) -> Option<usize> {
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
            position += max(self.get_bad_char_offset(bad_char),
                            self.get_good_suffix_offset(needle_position));
        }
        None
    }
}



#[test]
pub fn test_good_suffix_table2() {
    let needle = "GCAGAGAG".as_bytes();
    let table = build_good_suffixes_table(&needle);
    assert_eq!(vec![7,7,7,2,7,4,7,1], table);
}


#[test]
pub fn test_suffix_table() {
    let needle = "abcbabcabab".as_bytes();
    let table = get_suffix_table(&needle);
    assert_eq!(vec![0,2,0,1,0,3,0,0,2,0,0], table);
}


#[test]
pub fn test_good_suffix_table() {
    let needle = "abcbabcabab".as_bytes();
    let table = build_good_suffixes_table(&needle);
    assert_eq!(vec![10,10,10,10,10,10,10,5,2,7,1], table);
}


#[cfg(test)]
pub mod test {

    use super::*;
    use super::super::Search;

    #[test]
    pub fn test_simple() {
        let needle = BoyerMoore::new(&"ghi".as_bytes());
        let haystack = "abc def ghi jkl".as_bytes();
        assert_eq!(Some(8), needle.first_index(&haystack));
    }


    #[test]
    pub fn test_bad_char() {
        let needle = BoyerMoore::new(&"abacac".as_bytes());
        let haystack = "acacacababadabacacad".as_bytes();
        assert_eq!(Some(12), needle.first_index(&haystack));
    }


    #[test]
    pub fn test_bad_char2() {
        let needle = BoyerMoore::new(&"abacab".as_bytes());
        let haystack = "acacacababadabacabad".as_bytes();
        assert_eq!(Some(12), needle.first_index(&haystack));
    }
}
