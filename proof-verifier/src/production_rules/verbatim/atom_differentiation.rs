use shared::{atom::BuiltInAtom, proposition::Proposition, term::Term};

use crate::VerificationError;

/// Verify that the assumptions and the conclusion form a valid instance of atomicity assertion ("(Verbatim(a) != Verbatim(b))" for any two different atoms a and b)
pub fn verify_atom_differentiation(assumptions: &Vec<Proposition>, conclusion: &Proposition) -> Result<(),VerificationError> {
    todo!();
}
