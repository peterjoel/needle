
#![feature(test)]
extern crate test;

#[cfg(test)]
mod benchmarks;

pub mod boyer_moore;
mod horspool;
mod skip_search;

pub use boyer_moore::BoyerMoore;
pub use horspool::Horspool;
