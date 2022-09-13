use super::{tag::Tag, BlockAttrMap, BlockMacroTag};

pub enum BlockEvent {
    Start(Tag),
    End(Tag),
    /// https://docs.asciidoctor.org/asciidoc/latest/attributes/document-attributes/
    DocAttribute {
        name: String,
        values: Vec<String>,
    },
    Sidebar,
    BlockAttributes(BlockAttrMap), // TODO
    Text(String, SubstitutionRules),
    ImageBlockMacro {
        target: String,
        title: Option<String>,
        width: Option<u64>,
        height: Option<u64>,
        attrs: BlockAttrMap,
    },
    // AudioBlockMacro {
    //     target: String,
    //     title: Option<String>,
    //     start: Option<f64>,
    //     end: Option<f64>,
    //     opt_autoplay: bool,
    //     opt_loop: bool,
    //     opt_controls: bool,
    // },
    // VideoBlockMacro {
    //     target: String,
    //     title: Option<String>,
    //     poster: Option<String>,
    //     width: Option<u64>,
    // }
    /// https://docs.asciidoctor.org/asciidoc/latest/blocks/breaks/
    ThematicBreak,
    /// https://docs.asciidoctor.org/asciidoc/latest/blocks/breaks/
    PageBreak,
}

/// https://docs.asciidoctor.org/asciidoc/latest/subs/
pub struct SubstitutionRules {
    special_characters: bool,
    quotes: bool,
    attributes: bool,
    replacements: bool,
    macros: bool,
    post_replacements: bool,
}