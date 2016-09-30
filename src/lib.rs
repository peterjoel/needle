#![feature(specialization)]
#![feature(test)]
extern crate test;
extern crate memchr;

#[macro_use]
extern crate log;

#[cfg(test)]
mod benchmarks;

mod boyer_moore_memchr;
mod horspool_memchr;
mod boyer_moore;
mod skip_search;
mod horspool;
// internal for benchmarks
mod naive_search;
mod memchr_search;

pub use boyer_moore::BoyerMoore;
pub use horspool::Horspool;

pub trait SearchIn<'a, H: ?Sized> {
    type Iter: Iterator<Item = usize>;
    fn find_in(&'a self, haystack: &'a H) -> Self::Iter;
    fn find_overlapping_in(&'a self, haystack: &'a H) -> Self::Iter;

    /// Finds the first occurence of the search term in haystack and returns the index if it is found.
    fn find_first_in(&'a self, haystack: &'a H) -> Option<usize> {
        self.find_in(&haystack).next()
    }
}

pub trait CountIn<'a, H: ?Sized> {
    fn count_in(&'a self, haystack: &'a H) -> usize;
    fn occurs_in(&'a self, haystack: &'a H) -> bool {
        self.count_in(&haystack) > 0
    }
}

impl <'a, H: ?Sized, S> CountIn<'a, H> for S
    where S: SearchIn<'a, H>
{
    fn count_in(&'a self, haystack: &'a H) -> usize {
        self.find_in(&haystack).count()
    }

    fn occurs_in(&'a self, haystack: &'a H) -> bool {
        self.find_first_in(&haystack).is_some()
    }
}