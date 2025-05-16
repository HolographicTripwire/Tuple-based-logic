use shared::{atom::BuiltInAtom, proposition::Proposition, term::Term};

use crate::VerificationError;

/// Verify that the assumptions and the conclusion form a valid instance of atomicity assertion ("Atomic(Verbatim(a))" for any atom a)
pub fn verify_atomicity_assertion(assumptions: &Vec<Proposition>, conclusion: &Proposition) -> Result<(),VerificationError> {
    todo!();
}
