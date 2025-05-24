use std::{sync::LazyLock};

use tbl_structures::{atoms::AtomId, propositions::Term};
use tbl_stringification::{terms::{NoSpecialCasesStringifier, TermStringifier}, Destringify, Stringifier, Stringify};

use super::{VecStringifier};

struct NumAtomStringifier();

impl Stringifier<AtomId> for NumAtomStringifier {}
impl Stringify<AtomId> for NumAtomStringifier {
    fn stringify(&self, e: &AtomId) -> Result<String,()> {
        Ok(e.0.0.to_string())
    }
}
impl Destringify<AtomId> for NumAtomStringifier {
    fn destringify(&self, s: &String) -> Result<AtomId,()> {
        let Ok(u) = s.parse::<usize>() else { return Err(()) };
        let Ok(atom) = AtomId::try_from(u) else { return Err(()) };
        Ok(atom)
    }
}

pub static TERM_STRINGIFIER: LazyLock<Box<dyn Stringifier<Term>>> = 
    LazyLock::new(|| -> Box<dyn Stringifier<Term>> { 
        Box::new(TermStringifier::new(
            NumAtomStringifier(),
            VecStringifier(),
            NoSpecialCasesStringifier()
        ))
    });

#[cfg(test)]
mod tests {
    use tbl_structures::propositions::{Term};

    use super::*;

    #[test]
    fn test_textualize_atom() {
        let term = Term::from(AtomId::try_from(0).unwrap());
        assert_eq!(TERM_STRINGIFIER.stringify(&term),Ok("0".to_string()));
    }

    #[test]
    fn test_textualize_unary_tuple() {
        let atom_0 = Term::from(AtomId::try_from(0).unwrap());
        let term = Term::from(vec![atom_0]);
        assert_eq!(TERM_STRINGIFIER.stringify(&term),Ok("(0)".to_string()));
    }

    #[test]
    fn test_textualize_binary_tuple() {
        let atom_0 = Term::from(AtomId::try_from(0).unwrap());
        let atom_1 = Term::from(AtomId::try_from(1).unwrap());
        let term = Term::from(vec![atom_0,atom_1]);
        assert_eq!(TERM_STRINGIFIER.stringify(&term),Ok("(0, 1)".to_string()));
    }

    #[test]
    fn test_textualize_nested_tuple() {
        let atom_0 = Term::from(AtomId::try_from(0).unwrap());
        let atom_1 = Term::from(AtomId::try_from(1).unwrap());
        let term = Term::from(vec![Term::from(vec![atom_0]),atom_1]);
        assert_eq!(TERM_STRINGIFIER.stringify(&term),Ok("((0), 1)".to_string()));
    }
}
