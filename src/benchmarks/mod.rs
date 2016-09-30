use test::Bencher;
use super::BoyerMoore;
use super::Horspool;
use super::naive_search::NaiveSearch;
use super::memchr_search::MemchrSearch;
use super::memchr_skip_back::MemchrSkipBack;
use super::boyer_moore_memchr::BoyerMooreMemchr;
use super::horspool_memchr::HorspoolMemchr;
use super::SearchIn;
mod pi_digits;
use self::pi_digits::pi_100k_digits;

macro_rules! make_bench (
    ($name:ident, $searcher:expr, $test:expr) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            let haystack = pi_100k_digits().as_bytes();
            let needle = $searcher($test.needle);
            b.iter(|| {
                assert_eq!($test.expected, needle.find_first_in(&haystack))
            });
        }
    }
);


make_bench!(find_pi_100k_digits_non_num_needle_BoyerMoore,        BoyerMoore::new,       non_num());
make_bench!(find_pi_100k_digits_non_num_needle_BoyerMoore_memchr, BoyerMooreMemchr::new, non_num());
make_bench!(find_pi_100k_digits_non_num_needle_Horspool_memchr,   HorspoolMemchr::new,   non_num());
make_bench!(find_pi_100k_digits_non_num_needle_Horspool,          Horspool::new,         non_num());
make_bench!(find_pi_100k_digits_non_num_needle_MemchrSearch,      MemchrSearch::new,     non_num());
make_bench!(find_pi_100k_digits_non_num_needle_NaiveSearch,       NaiveSearch::new,      non_num());

make_bench!(find_pi_100k_digits_non_num_10_needle_BoyerMoore,        BoyerMoore::new,       non_num_10());
make_bench!(find_pi_100k_digits_non_num_10_needle_BoyerMoore_memchr, BoyerMooreMemchr::new, non_num_10());
make_bench!(find_pi_100k_digits_non_num_10_needle_Horspool_memchr,   HorspoolMemchr::new,   non_num_10());
make_bench!(find_pi_100k_digits_non_num_10_needle_Horspool,          Horspool::new,         non_num_10());
make_bench!(find_pi_100k_digits_non_num_10_needle_MemchrSearch,      MemchrSearch::new,     non_num_10());
make_bench!(find_pi_100k_digits_non_num_10_needle_NaiveSearch,       NaiveSearch::new,      non_num_10());

make_bench!(find_pi_100k_digits_non_num_100_needle_BoyerMoore,        BoyerMoore::new,       non_num_100());
make_bench!(find_pi_100k_digits_non_num_100_needle_BoyerMoore_memchr, BoyerMooreMemchr::new, non_num_100());
make_bench!(find_pi_100k_digits_non_num_100_needle_Horspool_memchr,   HorspoolMemchr::new,   non_num_100());
make_bench!(find_pi_100k_digits_non_num_100_needle_Horspool,          Horspool::new,         non_num_100());
make_bench!(find_pi_100k_digits_non_num_100_needle_MemchrSearch,      MemchrSearch::new,     non_num_100());
make_bench!(find_pi_100k_digits_non_num_100_needle_NaiveSearch,       NaiveSearch::new,      non_num_100());

make_bench!(find_pi_100k_digits_BoyerMoore,          BoyerMoore::new,       subsequence());
make_bench!(find_pi_100k_digits_BoyerMoore_memchr,   BoyerMooreMemchr::new, subsequence());
make_bench!(find_pi_100k_digits_Horspool_memchr,     HorspoolMemchr::new,   subsequence());
make_bench!(find_pi_100k_digits_Horspool,            Horspool::new,         subsequence());
make_bench!(find_pi_100k_digits_MemchrSearch,        MemchrSearch::new,     subsequence());
make_bench!(find_pi_100k_digits_NaiveSearch,         NaiveSearch::new,      subsequence());

make_bench!(find_pi_100k_digits_short_needle_BoyerMoore,        BoyerMoore::new,       subsequence_short());
make_bench!(find_pi_100k_digits_short_needle_BoyerMoore_memchr, BoyerMooreMemchr::new, subsequence_short());
make_bench!(find_pi_100k_digits_short_needle_Horspool_memchr,   HorspoolMemchr::new,   subsequence_short());
make_bench!(find_pi_100k_digits_short_needle_Horspool,          Horspool::new,         subsequence_short());
make_bench!(find_pi_100k_digits_short_needle_MemchrSearch,      MemchrSearch::new,     subsequence_short());
make_bench!(find_pi_100k_digits_short_needle_NaiveSearch,       NaiveSearch::new,      subsequence_short());


struct Case <'a> {
    needle: &'a [u8],
    expected: Option<usize>,
}
impl <'a> Case <'a> {
    fn new(needle: &'a [u8], expected: Option<usize> ) -> Case<'a> {
        Case { needle: needle, expected: expected }
    }
}

fn non_num<'a>() -> Case<'a> { Case::new(b"X", None) }
fn non_num_10<'a>() -> Case<'a> { Case::new(b"XXXXXXXXXX", None) }
fn non_num_100<'a>() -> Case<'a> { Case::new(b"XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX", None) }

fn subsequence<'a>() -> Case<'a> { Case::new(b"9857501636341131462753049901913564682380432997069577015078933772865803571279091376742080565549362541", Some(99_901)) }
fn subsequence_short<'a>() -> Case<'a> { Case::new(b"985750", Some(99_901)) }
