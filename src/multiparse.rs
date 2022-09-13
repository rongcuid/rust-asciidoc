pub trait MultiParse {
    /// Parse for multiple possibilities
    fn multiparse(s: &str) -> Vec<Self> where Self: Sized;
}