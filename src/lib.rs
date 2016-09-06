
#![feature(test)]
extern crate test;

pub mod boyer_moore;
pub mod naive;
mod benchmarks;

pub use boyer_moore::BoyerMoore;
pub use boyer_moore::Horspool;
