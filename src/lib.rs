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
            #[inline]
            fn isqrt(self) -> Self {
                (self as f64).sqrt() as Self

                // an order of magnitude worse
                //let mut x = self;
                //let mut next_x = self / 2 + self % 2;
                //
                //while next_x < x {
                //    x = next_x;
                //    next_x = (x + self / x)/2;
                //}
                //x
                //
            }

            #[inline]
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

            #[inline]
            fn is_square(self) -> bool {
                self.sqrt().is_some()
            }
        }
    };
}

implement_int_sqrt!(u8);
implement_int_sqrt!(u16);
implement_int_sqrt!(u32);
implement_int_sqrt!(u64);
implement_int_sqrt!(usize);

// FIXME: pull into separate tests
#[test]
fn squares() {
    for i in 0u8..16 {
        let ii = i*i;
        assert!((ii).is_square());
        assert!(ii.isqrt() == i);
        assert!(ii.sqrt().unwrap() == ii.isqrt());

        let i = i as u16;
        let ii = ii as u16;
        assert!((ii).is_square());
        assert!(ii.isqrt() == i);
        assert!(ii.sqrt().unwrap() == ii.isqrt());

        let i = i as u32;
        let ii = ii as u32;
        assert!((ii).is_square());
        assert!(ii.isqrt() == i);
        assert!(ii.sqrt().unwrap() == ii.isqrt());

        let i = i as u64;
        let ii = ii as u64;
        assert!((ii).is_square());
        assert!(ii.isqrt() == i);
        assert!(ii.sqrt().unwrap() == ii.isqrt());

        let i = i as usize;
        let ii = ii as usize;
        assert!((ii).is_square());
        assert!(ii.isqrt() == i);
        assert!(ii.sqrt().unwrap() == ii.isqrt());
    }
}

// FIXME: pull into separate tests
#[test]
fn nonsquares_plus_one() {
    for i in 1u8..16 {
        let ii = i*i + 1;
        assert!(!(ii).is_square());
        assert!(ii.isqrt() == i);
        assert!(ii.sqrt().is_none());

        let i = i as u16;
        let ii = ii as u16;
        assert!(!(ii).is_square());
        assert!(ii.isqrt() == i);
        assert!(ii.sqrt().is_none());

        let i = i as u32;
        let ii = ii as u32;
        assert!(!(ii).is_square());
        assert!(ii.isqrt() == i);
        assert!(ii.sqrt().is_none());

        let i = i as u64;
        let ii = ii as u64;
        assert!(!(ii).is_square());
        assert!(ii.isqrt() == i);
        assert!(ii.sqrt().is_none());

        let i = i as usize;
        let ii = ii as usize;
        assert!(!(ii).is_square());
        assert!(ii.isqrt() == i);
        assert!(ii.sqrt().is_none());
    }
}

// FIXME: pull into separate tests
#[test]
fn nonsquares_minus_one() {
    for i in 2u8..16 {
        let ii = i*i - 1;
        assert!(!(ii).is_square());
        assert!(ii.isqrt() == i - 1);
        assert!(ii.sqrt().is_none());

        let i = i as u16;
        let ii = ii as u16;
        assert!(!(ii).is_square());
        assert!(ii.isqrt() == i - 1);
        assert!(ii.sqrt().is_none());

        let i = i as u32;
        let ii = ii as u32;
        assert!(!(ii).is_square());
        assert!(ii.isqrt() == i - 1);
        assert!(ii.sqrt().is_none());

        let i = i as u64;
        let ii = ii as u64;
        assert!(!(ii).is_square());
        assert!(ii.isqrt() == i - 1);
        assert!(ii.sqrt().is_none());

        let i = i as usize;
        let ii = ii as usize;
        assert!(!(ii).is_square());
        assert!(ii.isqrt() == i - 1);
        assert!(ii.sqrt().is_none());
    }
}

#[test]
fn overflow() {
    (!0u8).isqrt(); // must not panic
}

#[test]
fn zero() {
    0u8.isqrt();
}
