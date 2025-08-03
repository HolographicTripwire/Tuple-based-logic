pub mod atoms; // The atoms which make up expressions
pub mod expressions; // Propositions; which represent an expression with truth values
pub mod inference; // Single steps of a larger proof
pub mod proof; // Proofs which show that some Propositions can be reached from others

/// A copy of the [Display](fmt::Display) trait which can be implemented on external types from within this crate
pub trait DisplayExt {
    fn display(&self) -> String;
}
