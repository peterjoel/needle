extern crate rand;

use ::SearchIn;
use ::boyer_moore::BoyerMoore;
use self::rand::Rng;
use self::rand::distributions::{IndependentSample, Range};
use self::rand::distributions::range::SampleRange;
use std::cmp;

#[test]
fn random_1_char_needle_4_alphabet_overlapping_bm() {
    // sample test that uses BoyerMoore search impl
    check_overlapping_search_indices(range(1, 2),
                                     range(0, 4),
                                     range(1, 1000),
                                     |n, h| {
        BoyerMoore::new(n)
            .find_overlapping_in(&h)
            .collect::<Vec<usize>>()
    })
}

#[test]
fn random_5_char_needle_4_alphabet_overlapping_bm() {
    check_overlapping_search_indices(range(5, 6),
                                     range(0, 4),
                                     range(1, 100),
                                     |n, h| {
        BoyerMoore::new(n)
            .find_overlapping_in(&h)
            .collect::<Vec<usize>>()
    })
}

#[test]
fn random_5_char_needle_10_alphabet_overlapping_bm() {
    check_overlapping_search_indices(range(5, 6),
                                     range(0, 4),
                                     range(1, 100),
                                     |n, h| {
        BoyerMoore::new(n)
            .find_overlapping_in(&h)
            .collect::<Vec<usize>>()
    })
}

#[test]
fn random_needle_4_alphabet_overlapping_bm() {
    check_overlapping_search_indices(range(1, 10),
                                     range(0, 4),
                                     range(1, 100),
                                     |n, h| {
        BoyerMoore::new(n)
            .find_overlapping_in(&h)
            .collect::<Vec<usize>>()
    })
}

fn check_overlapping_search_indices<F>(
    needle_len_range: Range<usize>,
    alphabet_range: Range<u8>,
    haystack_len_range: Range<usize>,
    run_candidate: F,
) where
    F: Fn(&[u8], &[u8]) -> Vec<usize>,
{
    let mut rng = rand::thread_rng();
    let mut haystack: Vec<u8> = Vec::new();
    let mut needle: Vec<u8> = Vec::new();

    for _ in 0..10_000 {
        haystack.clear();
        needle.clear();

        for _ in 0..needle_len_range.ind_sample(&mut rng) {
            needle.push(alphabet_range.ind_sample(&mut rng));
        }

        for _ in 0..haystack_len_range.ind_sample(&mut rng) {
            haystack.push(alphabet_range.ind_sample(&mut rng));
        }

        // compare the results against a naive (slow, but correct) search
        assert_eq!(
            overlapping_indices(&needle, &haystack),
            run_candidate(&needle, &haystack),
            "needle {:?} haystack {:?}", &needle, &haystack
        );
    }
}


fn overlapping_indices(needle: &[u8], haystack: &[u8]) -> Vec<usize> {
    haystack.windows(needle.len())
        .enumerate()
        .filter(|&(_, w)| w == needle)
        .map(|(i, _)| i)
        .collect()
}

fn range<T: SampleRange + cmp::PartialOrd>(lo: T, hi: T) -> Range<T> {
    Range::new(lo, hi)
}

#[test]
fn needle_genome() {
    let haystack = b"CGGACTCGACAGATGTGAAGAACGACAATGTGAAGACTCGACACGACAGAGTGAAGAGAAGAGGAAACATTGTAA";
    let needle = BoyerMoore::new(&b"GAAGA"[..]);

    assert_eq!(
        vec![16, 31, 52, 57],
        needle.find_overlapping_in(haystack).collect::<Vec<usize>>()
    );
}
