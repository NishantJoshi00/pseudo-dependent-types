impl<const N: usize> From<super::MinusOne<N>> for usize {
    fn from(_: super::MinusOne<N>) -> Self {
        _ = <super::MinusOne<N> as AssertGt0>::VALIDATE;

        N - 1
    }
}

trait AssertGt0 {
    const VALIDATE: ();
}

impl<const N: usize> AssertGt0 for super::MinusOne<N> {
    const VALIDATE: () = assert!(N > 0, "N must be greater than 0");
}
