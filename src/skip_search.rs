//! The skip-search logic for Boyer-Moore algorithm
pub trait SkipSearch<T> {
    /// Given `bad_char`, a character from haystack that didn't match with the character in the needle at 
    /// `needle_position`, calculate how many characters can be skipped 
    fn skip_offset(&self, bad_char: T, needle_position: usize) -> usize;

    /// The number of characters in the needle
    fn len(&self) -> usize;

    /// Retrieve a character from the index within needle
    fn char_at(&self, index: usize) -> T;
}

/// Find needle in haystack, starting at position within haystack
pub fn find_from_position<'a, T, H: ?Sized, S>(searcher: &'a S, haystack: &'a H, mut position: usize) -> Option<usize>
    where T: PartialEq + Into<usize> + Copy,
          H: Searchable<Item = T>,
          S: SkipSearch<T>
{
    let max_position = haystack.num_items() - searcher.len(); 
    while position <= max_position {
        let mut needle_position = searcher.len() - 1;
        while haystack.item_at(position + needle_position) == searcher.char_at(needle_position) {
            if needle_position == 0 {
                return Some(position);
            } else {
                needle_position -= 1;
            }
        }
        let bad_char = haystack.item_at(position + searcher.len() - 1);
        position += searcher.skip_offset(bad_char, needle_position);
    }
    None
}

// Bad characters table is used for when the last (rightmost) character of the needle doesn't match. The table
// gives the number of elements to skip, to find a character that does match.
pub fn build_bad_chars_table<T, H: ?Sized>(needle: &H) -> [usize; 256]
    where T: Into<usize> + Copy,
          H: Searchable<Item = T>
{
    let mut table = [needle.num_items(); 256];
    let num_items = needle.num_items();
    for i in 0 .. num_items - 1 {
        let c: usize = needle.item_at(i).into();
        table[c] = num_items - i - 1;
    }
    table
}

// Produces a table, whose indices are indices of needle, and whose entries are the size of 
// the largest suffix of needle that matches the substring ending at that index
pub fn get_suffix_table<T, H: ?Sized>(needle: &H) -> Vec<usize>
    where T: PartialEq,
          H: Searchable<Item = T>
{
    // The algorthm builds the table in steps as follows:
    // a b c b a b c a b a b | suffix (length)
    // --------------------- | ------
    // 0 1 0 1 0 1 0 0 1 0 1 |       b (1)
    // 0 2 0 1 0 2 0 0 2 0 2 |     a b (2)
    // 0 2 0 1 0 3 0 0 2 0 3 |   b a b (3)
    // 0 2 0 1 0 3 0 0 2 0 4 | a b a b (4)
    // etc..
    let len = needle.num_items();
    let mut suffixes = vec![0; len];
    for suffix_len in 1 .. len {
        let mut found_suffix = false;
        for i in (0 .. len - suffix_len).rev() {
            // either 0 or a previous match for a 1-smaller suffix
            if suffixes[i + suffix_len - 1] == suffix_len - 1 && needle.item_at(i) == needle.item_at(len - suffix_len) {
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
pub fn build_good_suffixes_table<T, H: ?Sized>(needle: &H) -> Vec<usize> 
    where T: PartialEq,
          H: Searchable<Item = T>
{
    let suffixes = get_suffix_table(needle);
    let len = needle.num_items();
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


// pub trait SearchIterable<T> {
//     type SearchIter: Iterator<Item = T>;
//     default type SearchOverlappingIter = SearchIter; 

//     fn find_in<'b>(&'b self, haystack: &'b [T]) -> SearchIter;
//     fn find_overlapping_in<'b>(&'b self, haystack: &'b [T]) -> SearchOverlappingIter;
// }

pub trait Searchable {
    type Item;

    fn num_items(&self) -> usize;
    fn item_at(&self, index: usize) -> Self::Item;
}

impl <T: Copy> Searchable for [T] {
    type Item = T;

    fn num_items(&self) -> usize {
        self.len()
    }

    fn item_at(&self, index: usize) -> Self::Item {
        self[index]
    }   
}

impl <'a, T: Copy> Searchable for &'a [T] {
    type Item = T;

    fn num_items(&self) -> usize {
        self.len()
    }

    fn item_at(&self, index: usize) -> Self::Item {
        self[index]
    }   
}

impl Searchable for String {
    type Item = char;

    fn num_items(&self) -> usize {
        self.len()
    }

    fn item_at(&self, index: usize) -> Self::Item {
        // XXX Um.. this can't be fast...
        self.chars().nth(index).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_good_suffix_table2() {
        let needle = "GCAGAGAG".as_bytes();
        let table = build_good_suffixes_table(needle);
        assert_eq!(vec![7,7,7,2,7,4,7,1], table);
    }


    #[test]
    pub fn test_suffix_table() {
        let needle = "abcbabcabab".as_bytes();
        let table = get_suffix_table(needle);
        assert_eq!(vec![0,2,0,1,0,3,0,0,2,0,0], table);
    }


    #[test]
    pub fn test_good_suffix_table() {
        let needle = "abcbabcabab".as_bytes();
        let table = build_good_suffixes_table(needle);
        assert_eq!(vec![10,10,10,10,10,10,10,5,2,7,1], table);
    }
}
