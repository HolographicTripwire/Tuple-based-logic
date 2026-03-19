pub mod expressions; // Propositions; which represent an expression with truth values
pub mod proof; // Proofs which show that some Propositions can be reached from others
pub mod path_composites; // Paths made of other, more granular paths

// /// A copy of the [Display](fmt::Display) trait which can be implemented on external types from within this crate
// pub trait DisplayExt {
//     fn display(&self) -> String;
// }
