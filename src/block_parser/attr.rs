use std::collections::HashMap;

/// Block attributes, minus the style and style-specific elements
pub struct BlockAttrMap {
    positional: Vec<String>,
    named: HashMap<String, String>,
}