#[derive(Debug, Clone)]
pub struct FpVec<T> {
    pub inner: Vec<T>,
}

impl<T> FpVec<T> {
    pub fn new() -> Self {
        Self { inner: vec![] }
    }

    pub fn from_vec(inner: Vec<T>) -> Self {
        Self { inner }
    }

    pub fn push(self, item: T) -> Self {
        let mut inner = self.inner;
        inner.push(item);
        Self { inner }
    }

    pub fn pop(self) -> (Self, Option<T>) {
        let mut inner = self.inner;
        let item = inner.pop();
        (Self { inner }, item)
    }

    pub fn extend(self, other: FpVec<T>) -> Self {
        let mut inner = self.inner;
        inner.extend(other.inner);
        Self { inner }
    }
}
