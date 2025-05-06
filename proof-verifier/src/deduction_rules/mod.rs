mod conjunction_introduction;
mod implication_elimination;
mod universal_instantiation;
mod tuple_appendation;

pub use conjunction_introduction::verify_conjunction_introduction;
pub use implication_elimination::verify_implication_elimination;
pub use universal_instantiation::verify_universal_instantiation;
pub use tuple_appendation::verify_tuple_appendation;
