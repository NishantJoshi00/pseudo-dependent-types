//
// This type will take a number as an input, and when converted to a f64 that will be `number / N`
//
pub struct MinusOne<const N: usize>;

impl<const N: usize> MinusOne<N> {
    pub fn new() -> Self {
        MinusOne
    }
}

#[cfg(not(feature = "magic"))]
pub mod impls;
#[cfg(feature = "magic")]
pub mod magic;
