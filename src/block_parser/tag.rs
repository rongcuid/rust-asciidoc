pub enum Tag {
    /// https://docs.asciidoctor.org/asciidoc/latest/document/header/
    DocumentHeader,
    /// https://docs.asciidoctor.org/asciidoc/latest/sections/titles-and-levels/
    SectionTitle(SectionLevel),
    /// https://docs.asciidoctor.org/asciidoc/latest/blocks/paragraphs/
    Paragraph,
    /// https://docs.asciidoctor.org/asciidoc/latest/blocks/discrete-headings/
    DiscreteHeading(SectionLevel),
    /// https://docs.asciidoctor.org/asciidoc/latest/lists/unordered/
    UnorderedList {
        style: UnorderedListMarker,
    },
    /// https://docs.asciidoctor.org/asciidoc/latest/lists/ordered/
    OrderedList {
        start: i64,
        reversed: bool,
        style: OrderedListStyle,
    },
    /// https://docs.asciidoctor.org/asciidoc/latest/lists/checklist/
    CheckList,
    /// https://docs.asciidoctor.org/asciidoc/latest/lists/description/
    DescriptionList {
        style: DescriptionListStyle,
    },
    /// https://docs.asciidoctor.org/asciidoc/latest/blocks/admonitions/
    Admonition(AdmonitionLabel),
    /// https://docs.asciidoctor.org/asciidoc/latest/blocks/sidebars/
    Sidebar,
    /// https://docs.asciidoctor.org/asciidoc/latest/blocks/example-blocks/
    Example,
    /// https://docs.asciidoctor.org/asciidoc/latest/blocks/blockquotes/
    BlockQuote {
        attribution: Option<String>,
        title: Option<String>,
    },
    /// https://docs.asciidoctor.org/asciidoc/latest/blocks/verses/
    Verse {
        attribution: Option<String>,
        title: Option<String>,
    },
    /// https://docs.asciidoctor.org/asciidoc/latest/verbatim/source-blocks/
    Source {
        language: Option<String>,
    },
    /// https://docs.asciidoctor.org/asciidoc/latest/verbatim/listing-blocks/
    Listing,
    /// https://docs.asciidoctor.org/asciidoc/latest/verbatim/literal-blocks/
    Literal,
    /// https://docs.asciidoctor.org/asciidoc/latest/tables/build-a-basic-table/
    Table,
    TableRow,
    TableCell,
    /// https://docs.asciidoctor.org/asciidoc/latest/stem/
    Stem {
        latex: bool,
    },
    /// https://docs.asciidoctor.org/asciidoc/latest/blocks/open-blocks/
    Open,
    /// https://docs.asciidoctor.org/asciidoc/latest/blocks/collapsible/
    Collapsible {
        open: bool,
    },
}

pub enum SectionLevel {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
}

pub enum UnorderedListMarker {
    Square,
    Circle,
    Disc,
    NoBullet,
    Unstyled,
}

pub enum OrderedListStyle {
    Arabic,
    Decimal,
    LowerAlpha,
    UpperAlpha,
    LowerRoman,
    UpperRoman,
    LowerGreek,
    Custom(String),
}

pub enum DescriptionListStyle {
    Horizontal,
    QAndA,
    Marker {
        ordered: bool,
        stacked: bool,
        subject_stop: String,
    },
}

pub enum AdmonitionLabel {
    Note,
    Tip,
    Important,
    Caution,
    Warning,
}