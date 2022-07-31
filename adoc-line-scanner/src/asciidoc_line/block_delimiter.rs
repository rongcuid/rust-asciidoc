#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockDelimiter {
    pub delimiter: String,
    pub level: usize,
}

impl<'a> TryFrom<&'a str> for BlockDelimiter {
    type Error = ();

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        todo!()
    }
}