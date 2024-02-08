
impl<const N: usize> From<super::MinusOne<N>> for usize {
    fn from(_: super::MinusOne<N>) -> Self {
        N - 1
    }
}
