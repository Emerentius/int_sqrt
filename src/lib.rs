use std::cmp::min;

pub trait IntSqrt
    where Self: Sized
{
    fn isqrt(self) -> Self;
    fn sqrt(self) -> Option<Self>;
}

macro_rules! implement_int_sqrt {
    ( $T:ty ) => {
        impl IntSqrt for $T {
            fn isqrt(self) -> Self {
                let mut last_x = 1;
                let mut x = (last_x+self/last_x)/2;
                let mut next_x = (x+self/x)/2;

                while last_x != next_x && x != next_x {
                    last_x = x;
                    x = next_x;
                    next_x = (x+self/x)/2
                }
                min(x, next_x)
            }

            fn sqrt(self) -> Option<Self> {
                let root = self.isqrt();
                match root*root == self {
                    true => Some(root),
                    false => None,
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
