
#![feature(test)]
extern crate test;

#[cfg(test)]
mod benchmarks;

pub mod boyer_moore;
mod skip_search;
mod horspool;

pub use boyer_moore::BoyerMoore;
pub use horspool::Horspool;
