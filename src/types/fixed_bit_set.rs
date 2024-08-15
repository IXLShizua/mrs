#[derive(Debug)]
pub struct FixedBitSet<const N: usize> {
    inner: [bool; N],
}

impl<const N: usize> FixedBitSet<N> {
    pub const fn new() -> FixedBitSet<N> {
        FixedBitSet { inner: [false; N] }
    }

    pub fn set(&mut self, idx: usize, state: bool) {
        assert!(idx < self.inner.len());

        self.inner[idx] = state;
    }

    pub fn get(&mut self, idx: usize) -> Option<bool> {
        self.inner.get(idx).map(ToOwned::to_owned)
    }

    pub fn first(&self) -> Option<bool> {
        self.inner.first().map(ToOwned::to_owned)
    }

    pub fn last(&self) -> Option<bool> {
        self.inner.last().map(ToOwned::to_owned)
    }
}
