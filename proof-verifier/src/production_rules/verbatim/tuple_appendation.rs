use shared::{atom::BuiltInAtom, proposition::Proposition, term::Term};

use crate::VerificationError;

/// Verify that the assumptions and the conclusion form a valid instance of atomicity assertion ("Append(Verbatim((v1,v2,v3,...,vn)),Verbatim(vm)) = Verbatim((v1,v2,v3,...,vn,vm))" for any (v1,v2,v3,...,vn) and vm)
pub fn verify_tuple_appendation(assumptions: &Vec<Proposition>, conclusion: &Proposition) -> Result<(),VerificationError> {
    todo!()
}
