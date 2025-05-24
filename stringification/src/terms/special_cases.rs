use tbl_structures::propositions::Term;

use crate::{Destringify, Stringifier, Stringify};

pub struct SpecialCase(pub Vec<Term>,pub Vec<String>);

/// A rule textualizer that always returns Err(())
pub struct NoSpecialCasesStringifier();

impl Stringifier<SpecialCase> for NoSpecialCasesStringifier {}
impl Stringify<SpecialCase> for NoSpecialCasesStringifier {
    fn stringify(&self, _: &SpecialCase) -> Result<String,()> { Err(()) }
}
impl Destringify<SpecialCase> for NoSpecialCasesStringifier {
    fn destringify(&self, _: &String) -> Result<SpecialCase,()> { Err(()) }
}
