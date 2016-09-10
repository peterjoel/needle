
#![feature(test)]
extern crate test;

pub mod boyer_moore;
mod benchmarks;
mod skip_search;
mod horspool;

pub use boyer_moore::BoyerMoore;
pub use horspool::Horspool;
