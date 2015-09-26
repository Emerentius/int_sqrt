#![cfg(test)]
#![feature(test)]
extern crate test;
extern crate int_sqrt;
use int_sqrt::IntSqrt;

#[bench]
fn isqrt_bench(b: &mut test::Bencher) {
    b.iter(|| {
        let mut sum = 0;
        for n in 1u32..1_00 {
            let sq = n*n;
            let next_sq = (n+1)*(n+1);
            for m in sq..next_sq {
                if m.isqrt() == n {
                    sum += m;
                }
            }
        }
        test::black_box(sum);
    })
}

#[bench]
fn is_square_bench(b: &mut test::Bencher) {
    b.iter(|| {
        let mut sum = 0;
        for n in 1u32..1_00 {
            let sq = n*n;
            let next_sq = (n+1)*(n+1);
            for m in sq..next_sq {
                if m.is_square() {
                    sum += m;
                }
            }
        }
        test::black_box(sum);
    })
}

#[bench]
fn sqrt_bench(b: &mut test::Bencher) {
    b.iter(|| {
        let mut sum = 0;
        for n in 1u32..1_00 {
            let sq = n*n;
            let next_sq = (n+1)*(n+1);
            for m in sq..next_sq {
                if m.sqrt().is_some() {
                    sum += m;
                }
            }
        }
        test::black_box(sum);
    })
}
