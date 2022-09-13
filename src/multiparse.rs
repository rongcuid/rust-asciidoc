pub trait MultiParse {
    /// Parse for multiple possibilities
    fn multiparse(s: &str) -> VecSet<Self>
    where
        Self: Sized;
}

#[derive(Debug)]
pub struct VecSet<T>(Vec<T>);

impl<T> VecSet<T> {
    pub fn new(v: Vec<T>) -> Self {
        Self(v)
    }
}

/// Two vectors each contain all of other's elements
impl<T: PartialEq> PartialEq for VecSet<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0.iter().all(|item| other.0.contains(item))
            && other.0.iter().all(|item| self.0.contains(item))
    }
}
