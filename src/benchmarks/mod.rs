#![allow(non_snake_case)]
use test::Bencher;
use super::BoyerMoore;
use super::Horspool;
use super::naive_search::NaiveSearch;
use super::memchr_search::MemchrSearch;
use super::boyer_moore_memchr::BoyerMooreMemchr;
use super::horspool_memchr::HorspoolMemchr;
use super::{SearchIn, CountIn};
mod pi_digits;
mod alice;
use self::pi_digits::pi_100k_digits;
use self::alice::alice_text;

macro_rules! bench_find_first (
    ($name:ident, $searcher:expr, $test:expr) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            let haystack = $test.haystack;
            let needle = $searcher($test.needle);
            b.iter(|| {
                assert_eq!($test.expected, needle.find_first_in(&haystack))
            });
        }
    }
);

macro_rules! bench_count (
    ($name:ident, $searcher:expr, $test:expr) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            let haystack = $test.haystack;
            let needle = $searcher($test.needle);
            b.iter(|| {
                assert_eq!($test.expected.unwrap(), needle.count_in(&haystack))
            });
        }
    }
);

bench_find_first!(find_pi_100k_digits_non_num_needle_BoyerMoore,        BoyerMoore::new,       non_num());
bench_find_first!(find_pi_100k_digits_non_num_needle_BoyerMoore_memchr, BoyerMooreMemchr::new, non_num());
bench_find_first!(find_pi_100k_digits_non_num_needle_Horspool_memchr,   HorspoolMemchr::new,   non_num());
bench_find_first!(find_pi_100k_digits_non_num_needle_Horspool,          Horspool::new,         non_num());
bench_find_first!(find_pi_100k_digits_non_num_needle_MemchrSearch,      MemchrSearch::new,     non_num());
bench_find_first!(find_pi_100k_digits_non_num_needle_NaiveSearch,       NaiveSearch::new,      non_num());

bench_find_first!(find_pi_100k_digits_non_num_10_needle_BoyerMoore,        BoyerMoore::new,       non_num_10());
bench_find_first!(find_pi_100k_digits_non_num_10_needle_BoyerMoore_memchr, BoyerMooreMemchr::new, non_num_10());
bench_find_first!(find_pi_100k_digits_non_num_10_needle_Horspool_memchr,   HorspoolMemchr::new,   non_num_10());
bench_find_first!(find_pi_100k_digits_non_num_10_needle_Horspool,          Horspool::new,         non_num_10());
bench_find_first!(find_pi_100k_digits_non_num_10_needle_MemchrSearch,      MemchrSearch::new,     non_num_10());
bench_find_first!(find_pi_100k_digits_non_num_10_needle_NaiveSearch,       NaiveSearch::new,      non_num_10());

bench_find_first!(find_pi_100k_digits_non_num_100_needle_BoyerMoore,        BoyerMoore::new,       non_num_100());
bench_find_first!(find_pi_100k_digits_non_num_100_needle_BoyerMoore_memchr, BoyerMooreMemchr::new, non_num_100());
bench_find_first!(find_pi_100k_digits_non_num_100_needle_Horspool_memchr,   HorspoolMemchr::new,   non_num_100());
bench_find_first!(find_pi_100k_digits_non_num_100_needle_Horspool,          Horspool::new,         non_num_100());
bench_find_first!(find_pi_100k_digits_non_num_100_needle_MemchrSearch,      MemchrSearch::new,     non_num_100());
bench_find_first!(find_pi_100k_digits_non_num_100_needle_NaiveSearch,       NaiveSearch::new,      non_num_100());

bench_find_first!(find_pi_100k_digits_BoyerMoore,          BoyerMoore::new,       subsequence());
bench_find_first!(find_pi_100k_digits_BoyerMoore_memchr,   BoyerMooreMemchr::new, subsequence());
bench_find_first!(find_pi_100k_digits_Horspool_memchr,     HorspoolMemchr::new,   subsequence());
bench_find_first!(find_pi_100k_digits_Horspool,            Horspool::new,         subsequence());
bench_find_first!(find_pi_100k_digits_MemchrSearch,        MemchrSearch::new,     subsequence());
bench_find_first!(find_pi_100k_digits_NaiveSearch,         NaiveSearch::new,      subsequence());

bench_find_first!(find_pi_100k_digits_short_needle_BoyerMoore,        BoyerMoore::new,       subsequence_short());
bench_find_first!(find_pi_100k_digits_short_needle_BoyerMoore_memchr, BoyerMooreMemchr::new, subsequence_short());
bench_find_first!(find_pi_100k_digits_short_needle_Horspool_memchr,   HorspoolMemchr::new,   subsequence_short());
bench_find_first!(find_pi_100k_digits_short_needle_Horspool,          Horspool::new,         subsequence_short());
bench_find_first!(find_pi_100k_digits_short_needle_MemchrSearch,      MemchrSearch::new,     subsequence_short());
bench_find_first!(find_pi_100k_digits_short_needle_NaiveSearch,       NaiveSearch::new,      subsequence_short());

bench_count!(count_alice_box_BoyerMoore,        BoyerMoore::new,       count_alice_box());
bench_count!(count_alice_box_BoyerMoore_memchr, BoyerMooreMemchr::new, count_alice_box());
bench_count!(count_alice_box_Horspool_memchr,   HorspoolMemchr::new,   count_alice_box());
bench_count!(count_alice_box_Horspool,          Horspool::new,         count_alice_box());
bench_count!(count_alice_box_MemchrSearch,      MemchrSearch::new,     count_alice_box());
bench_count!(count_alice_box_NaiveSearch,       NaiveSearch::new,      count_alice_box());

bench_count!(count_alice_rabbit_BoyerMoore,        BoyerMoore::new,       count_alice_rabbit());
bench_count!(count_alice_rabbit_BoyerMoore_memchr, BoyerMooreMemchr::new, count_alice_rabbit());
bench_count!(count_alice_rabbit_Horspool_memchr,   HorspoolMemchr::new,   count_alice_rabbit());
bench_count!(count_alice_rabbit_Horspool,          Horspool::new,         count_alice_rabbit());
bench_count!(count_alice_rabbit_MemchrSearch,      MemchrSearch::new,     count_alice_rabbit());
bench_count!(count_alice_rabbit_NaiveSearch,       NaiveSearch::new,      count_alice_rabbit());

bench_count!(count_alice_thoughtfully_BoyerMoore,        BoyerMoore::new,       count_alice_thoughtfully());
bench_count!(count_alice_thoughtfully_BoyerMoore_memchr, BoyerMooreMemchr::new, count_alice_thoughtfully());
bench_count!(count_alice_thoughtfully_Horspool_memchr,   HorspoolMemchr::new,   count_alice_thoughtfully());
bench_count!(count_alice_thoughtfully_Horspool,          Horspool::new,         count_alice_thoughtfully());
bench_count!(count_alice_thoughtfully_MemchrSearch,      MemchrSearch::new,     count_alice_thoughtfully());
bench_count!(count_alice_thoughtfully_NaiveSearch,       NaiveSearch::new,      count_alice_thoughtfully());

struct Case <'a> {
    needle: &'a [u8],
    haystack: &'a [u8],
    expected: Option<usize>,
}
impl <'a> Case <'a> {
    fn new(needle: &'a [u8], haystack: &'a [u8], expected: Option<usize> ) -> Case<'a> {
        Case { needle: needle, haystack: haystack, expected: expected }
    }
}

fn non_num<'a>() -> Case<'a> { Case::new(b"X", pi_100k(), None) }
fn non_num_10<'a>() -> Case<'a> { Case::new(b"XXXXXXXXXX", pi_100k(), None) }
fn non_num_100<'a>() -> Case<'a> { Case::new(b"XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX", pi_100k(), None) }

fn subsequence<'a>() -> Case<'a> { Case::new(b"9857501636341131462753049901913564682380432997069577015078933772865803571279091376742080565549362541", pi_100k(), Some(99_901)) }
fn subsequence_short<'a>() -> Case<'a> { Case::new(b"985750", pi_100k(), Some(99_901)) }

fn count_alice_box<'a>() -> Case<'a> { Case::new(b"box", alice(), Some(11)) }
fn count_alice_rabbit<'a>() -> Case<'a> { Case::new(b"Rabbit", alice(), Some(45)) }
fn count_alice_thoughtfully<'a>() -> Case<'a> { Case::new(b"thoughtfully", alice(), Some(4)) }

fn pi_100k<'a>() -> &'a [u8] { pi_100k_digits().as_bytes() }
fn alice<'a>() -> &'a [u8] { alice_text().as_bytes() }