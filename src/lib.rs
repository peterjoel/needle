
#![feature(test)]
extern crate test;

pub mod boyer_moore;
pub mod naive;
mod benchmarks;

pub use boyer_moore::BoyerMoore;

pub trait Search<'a>{
    fn first_index(&self, haystack: &'a [u8]) -> Option<usize>;
}
