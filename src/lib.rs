
#![feature(test)]
extern crate test;

#[cfg(test)]
mod benchmarks;

pub mod boyer_moore;
mod skip_search;
mod horspool;

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
