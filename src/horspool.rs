use skip_search::*;

pub struct Horspool <'a> {
    needle: &'a [u8],
    bad_chars: [usize; 256],
}


impl <'a> Horspool <'a> {
    pub fn new(needle: &'a [u8]) -> Horspool {
        Horspool { 
            needle: needle,
            bad_chars: build_bad_chars_table(&needle),
        }
    }

    pub fn first_index<'b>(&'b self, haystack: &'b [u8]) -> Option<usize> {
        self.find_in(&haystack).next()
    }


    /// Returns an iterator that will produce the indices of the needle in the haystack.
    /// This iterator will not find overlapping matches; the first character of a match 
    /// will start after the last character of the previous match.
    ///
    /// # Example
    /// ```
    /// use needle::Horspool;
    /// let needle = Horspool::new(b"aaba");
    /// let haystack = b"aabaabaabaabaaba";
    /// assert_eq!(vec![0,6,12], needle.find_in(haystack).collect::<Vec<usize>>());
    /// ```
    pub fn find_in<'b>(&'b self, haystack: &'b [u8]) -> HorspoolIter {
        HorspoolIter {
            searcher: &self,
            haystack: &haystack,
            position: 0,
            overlapping_matches: false,
        }
    }

    /// Returns an iterator that will produce the indices of the needle in the haystack.
    /// This iterator will find overlapping matches; the first character of a match is 
    /// allowed to be matched from within the previous match.
    ///
    /// # Example
    /// ```
    /// use needle::Horspool;
    /// let needle = Horspool::new(b"aaba");
    /// let haystack = b"aabaabaabaabaaba";
    /// assert_eq!(vec![0,3,6,9,12], needle.find_overlapping_in(haystack).collect::<Vec<usize>>());
    /// ```
    pub fn find_overlapping_in<'b>(&'b self, haystack: &'b [u8]) -> HorspoolIter {
        HorspoolIter {
            searcher: &self,
            haystack: &haystack,
            position: 0,
            overlapping_matches: true
        }
    }
}

impl <'a> SkipSearch for &'a Horspool <'a> {
    #[inline]
    fn skip_offset(&self, bad_char: u8, _: usize) -> usize {
        self.bad_chars[bad_char as usize]
    }

    #[inline]
    fn len(&self) -> usize {
        self.needle.len()
    }

    #[inline]
    fn char_at(&self, index: usize) -> u8 {
        self.needle[index]
    }
}

pub struct HorspoolIter <'a> {
    searcher: &'a Horspool<'a>,
    haystack: &'a [u8],
    position: usize,
    overlapping_matches: bool,
}


impl <'a> Iterator for HorspoolIter<'a> {
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
