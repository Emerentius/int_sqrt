use std::cmp::min;

pub trait IntSqrt
    where Self: Sized
{
    fn isqrt(self) -> Self;
    fn sqrt(self) -> Option<Self>;
    fn is_square(self) -> bool;
}

macro_rules! implement_int_sqrt {
    ( $T:ty ) => {
        impl IntSqrt for $T {
            // newton's method
            fn isqrt(self) -> Self {
                let mut last_x = 1;
                let mut x = (last_x+self/last_x)/2;
                let mut next_x = (x+self/x)/2;

                while x != next_x && last_x != next_x {
                    last_x = x;
                    x = next_x;
                    next_x = (x+self/x)/2
                }
                min(x, next_x)
            }

            fn sqrt(self) -> Option<Self> {
                match self % 16 {
                    0 | 1 | 4 | 9 => {
                        let root = self.isqrt();
                        match root*root == self {
                            true => Some(root),
                            false => None,
                        }
                    },
                    _ => None,
                }
            }

            fn is_square(self) -> bool {
                match self % 16 {
                    0 | 1 | 4 | 9 => {
                        let root = self.isqrt();
                        root*root == self
                    },
                    _ => false,
                }
            }
        }
    };
}

implement_int_sqrt!(u8);
implement_int_sqrt!(u16);
implement_int_sqrt!(u32);
implement_int_sqrt!(u64);
implement_int_sqrt!(usize);

#[test]
fn trait_isqrt_correctness() {
    for n in 1u32..1_000 {
        let sq = n*n;
        let next_sq = (n+1)*(n+1);
        for m in sq..next_sq {
            assert!( m.isqrt() == n);
        }
    }
}

#[test]
fn trait_sqrt_correctness() {
    for n in 1u32..1_000 {
        let sq = n*n;
        let next_sq = (n+1)*(n+1);

        assert!( sq.sqrt().is_some() );

        for m in sq+1..next_sq {
            assert!( m.sqrt().is_none() );
        }
    }
}

#[test]
fn squareness_check() {
    let squares: Vec<u32> = (1..1000).map(|n| n*n).collect();
    for &n_sq in &squares {
        assert!(n_sq.is_square());
    }
    for n_non_sq in (1..1_000_000).filter(|n| squares.binary_search(&n).is_err()) {
        assert!( !n_non_sq.is_square() );
    }
}
