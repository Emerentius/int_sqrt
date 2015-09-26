# int_sqrt
Compute integer squareroot `isqrt` and regular squareroot `sqrt` for unsigned integers, currently only primitives `u8..u64, usized`.  
`n.isqrt()` returns the biggest integer `<= sqrt(n)`.  
`n.sqrt()` returns `Some(sqrt(n))` if n is square and `None` otherwise.
