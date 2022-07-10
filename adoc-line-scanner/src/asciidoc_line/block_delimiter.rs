pub struct BlockDelimiter<'a> {
    pub delimiter: &'a str,
    pub level: usize,
}

impl<'a> TryFrom<&'a str> for BlockDelimiter<'a> {
    type Error = ();

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        todo!()
    }
}