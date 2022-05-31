#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Content {
    Compound,
    Simple,
    Verbatim,
    Table,
    Raw,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Context {
    Admonition,
    Audio,
    CalloutList,
    DescriptionList,
    Example,
    FloatingTitle,
    Image,
    ListItem,
    Listing,
    Literal,
    OrderedList,
    Open,
    PageBreak,
    Paragraph,
    Passthrough,
    Quote,
    Section,
    Sidebar,
    Table,
    TableCell,
    ThematicBreak,
    UnorderedList,
    Verse,
}
