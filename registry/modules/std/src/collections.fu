use crate::security::verify_current_context;
/// Fusion replacement for `Vec`.
#[derive(Clone, Debug, Default)]
pub struct FVec<T> {
    inner: FVec<T>,
}
impl<T> FVec<T> {
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }
    pub fn push(&mut self, value: T) {
        if !verify_current_context() {
            panic!("Write denied: FVec");
        }
        self.inner.push(value);
    }
    pub fn get(&self, index: FSize) -> Option<&T> {
        if !verify_current_context() {
            return None;
        }
        self.inner.get(index)
    }
    pub fn len(&self) -> FSize {
        if !verify_current_context() {
            panic!("Read denied: FVec");
        }
        self.inner.len()
    }
    pub fn is_empty(&self) -> FBool {
        if !verify_current_context() {
            panic!("Read denied: FVec");
        }
        self.inner.is_empty()
    }
    pub fn iter(&self) -> FusionIter<'_, T> {
        if !verify_current_context() {
            panic!("Access denied: FVec iter");
        }
        FusionIter {
            inner: self.inner.iter(),
        }
    }
}
pub struct FusionIter<'a, T> {
    inner: std::slice::Iter<'a, T>,
}
impl<'a, T> Iterator for FusionIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if !verify_current_context() {
            return None;
        }
        self.inner.next()
    }
}
#[macro_export]
macro_rules! fvec {
    ($($x:expr),* $(,)?) => {
        { let mut temp = $crate::collections::FVec::new(); $(temp.push($x);)* temp }
    };
}
