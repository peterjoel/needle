use test::Bencher;
use super::BoyerMoore;
use super::Horspool;
use super::SearchIn;
mod pi_digits;
use self::pi_digits::pi_100k_digits;

#[bench]
fn find_pi_100k_digits_boyer_moore(b: &mut Bencher) {
    let haystack = pi_100k_digits().as_bytes();
    b.iter(|| {
        let needle = BoyerMoore::new(subsequence().as_bytes());
        assert_eq!(Some(99_901), needle.find_first_in(&haystack))
    });
}

#[bench]
fn find_pi_100k_digits_boyer_moore_with_precompute(b: &mut Bencher) {
    let haystack = pi_100k_digits().as_bytes();;
    let needle = BoyerMoore::new(subsequence().as_bytes());
    b.iter(|| {
        assert_eq!(Some(99_901), needle.find_first_in(&haystack))
    });
}

#[bench]
fn find_pi_100k_digits_horspool(b: &mut Bencher) {
    let haystack = pi_100k_digits().as_bytes();
    b.iter(|| {
        let needle = Horspool::new(subsequence().as_bytes());
        assert_eq!(Some(99_901), needle.find_first_in(&haystack))
    });
}

#[bench]
fn find_pi_100k_digits_horspool_with_precompute(b: &mut Bencher) {
    let haystack = pi_100k_digits().as_bytes();
    let needle = Horspool::new(subsequence().as_bytes());
    b.iter(|| {
        assert_eq!(Some(99_901), needle.find_first_in(&haystack))
    });
}

#[bench]
fn find_pi_100k_digits_std_str(b: &mut Bencher) {
    // n.b. This isn't a fair comparison because find() has to consider multi-byte characters
    let haystack = pi_100k_digits();
    b.iter(|| {
        let index = haystack.find(subsequence());
        assert_eq!(Some(99_901), index)
    });
}

#[cfg(test)]
fn subsequence<'a>() -> &'a str { "9857501636341131462753049901913564682380432997069577015078933772865803571279091376742080565549362541" } 

