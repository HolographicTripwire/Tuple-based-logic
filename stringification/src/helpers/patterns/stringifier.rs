use crate::{helpers::controls::{Controls, StringifierControl, StringifierControls}, Destringify, Stringifier, Stringify};

use super::ExprPattern;

pub struct ExprPatternStringifier {
    controls: Box<StringifierControls>
}
impl ExprPatternStringifier {
    pub fn new(controls: Box<StringifierControls>) -> Self {
        Self { controls }
    }
}

impl Stringifier<ExprPattern> for ExprPatternStringifier {}
impl Stringify<ExprPattern> for ExprPatternStringifier {
    fn stringify(&self, object: &ExprPattern) -> Result<String,()> {
        todo!()
    }
}
impl Destringify<ExprPattern> for ExprPatternStringifier {
    fn destringify(&self, string: &String) -> Result<ExprPattern,()> {
        // Get control strings
        let escape_string = self.controls.string_from_control(&StringifierControl::Escape);
        let pattern_controls = &self.controls;

        todo!()
    }
}
