use std::{sync::LazyLock};

use shared::atoms::AtomId;
use tuple_based_logic_textualizer::{terms::{NoRulesTextualizer, TermTextualizer}, Textualizer};

use super::{VecTextualizer};

struct NumAtomTextualizer();

impl Textualizer<AtomId> for NumAtomTextualizer {
    fn to_text(&self, e: &AtomId) -> Result<String,()> {
        Ok(e.0.0.to_string())
    }

    fn from_text(&self, s: &String) -> Result<AtomId,()> {
        todo!()
    }
}

pub static TERM_TEXTUALIZER: LazyLock<TermTextualizer> = LazyLock::new(|| -> TermTextualizer { TermTextualizer::new(
    Box::new(NumAtomTextualizer()),
    Box::new(VecTextualizer()),
    Box::new(NoRulesTextualizer())
)});

#[cfg(test)]
mod tests {
    use shared::propositions::{Term};

    use super::*;

    #[test]
    fn test_textualize_atom() {
        let term = Term::from(AtomId::try_from(0).unwrap());
        assert_eq!(TERM_TEXTUALIZER.to_text(&term),Ok("0".to_string()));
    }

    #[test]
    fn test_textualize_unary_tuple() {
        let atom_0 = Term::from(AtomId::try_from(0).unwrap());
        let term = Term::from(vec![atom_0]);
        assert_eq!(TERM_TEXTUALIZER.to_text(&term),Ok("(0)".to_string()));
    }

    #[test]
    fn test_textualize_binary_tuple() {
        let atom_0 = Term::from(AtomId::try_from(0).unwrap());
        let atom_1 = Term::from(AtomId::try_from(1).unwrap());
        let term = Term::from(vec![atom_0,atom_1]);
        assert_eq!(TERM_TEXTUALIZER.to_text(&term),Ok("(0, 1)".to_string()));
    }

    #[test]
    fn test_textualize_nested_tuple() {
        let atom_0 = Term::from(AtomId::try_from(0).unwrap());
        let atom_1 = Term::from(AtomId::try_from(1).unwrap());
        let term = Term::from(vec![Term::from(vec![atom_0]),atom_1]);
        assert_eq!(TERM_TEXTUALIZER.to_text(&term),Ok("((0), 1)".to_string()));
    }
}
