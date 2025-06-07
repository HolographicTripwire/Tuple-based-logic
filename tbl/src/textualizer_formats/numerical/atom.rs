use std::sync::LazyLock;

use tbl_textualization::structures::atom::AtomTextualizer;

pub static ATOM_TEXTUALIZER: LazyLock<AtomTextualizer> = LazyLock::new(|| -> AtomTextualizer { 
    AtomTextualizer::from_strings(vec![
        // No built in atoms interpretations; they will all be treated as numbers
    ])
});
