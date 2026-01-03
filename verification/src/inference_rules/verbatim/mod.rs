mod atomicity_assertion;
mod atom_differentiation;
mod tuple_appendation;

pub use atomicity_assertion::*;
pub use atom_differentiation::*;
pub use tuple_appendation::*;

use itertools::Either;

use crate::{assertions::*};
use tbl_structures::{atoms::BuiltInAtom, expressions::Expression, path_composites::ExpressionInInference};

