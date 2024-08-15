#[derive(Debug, Default)]
pub struct BitSet {
    inner: Vec<bool>,
}

impl BitSet {
    pub fn new() -> BitSet {
        BitSet::default()
    }

    pub fn put(&mut self, state: bool) {
        self.inner.push(state)
    }

    pub fn set(&mut self, idx: usize, state: bool) {
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
