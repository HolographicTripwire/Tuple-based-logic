use crate::{helpers::{controls::StringifierControls, patterns::{ExprPattern, ExprPatternStringifier}}, Destringify, Stringifier, Stringify};

use super::SpecialCase;

pub struct PatternStringifier2(ExprPattern,ExprPattern);

impl PatternStringifier2 {
    pub fn from_strings(pre: &str, post: &str, controls: Box<StringifierControls>) -> Result<Self,()> {
        let pattern_stringifier = ExprPatternStringifier::new(controls);
        Ok(Self(
            pattern_stringifier.destringify(&pre.to_string())?,
            pattern_stringifier.destringify(&post.to_string())?
        ))
    }
}

impl Stringifier<SpecialCase> for PatternStringifier2 {}
impl Stringify<SpecialCase>  for PatternStringifier2{
    fn stringify(&self, object: &SpecialCase) -> Result<String,()> {
        let string = object.vecified_whole.clone();
        let Ok(replacements) = self.0.match_string(string) else { return Err(()) };
        let pattern = self.1.replace_variables(replacements)?;
        pattern.try_into()
    }
}
impl Destringify<SpecialCase> for PatternStringifier2 {
    fn destringify(&self, string: &String) -> Result<SpecialCase,()> {
        let Ok(replacements) = self.1.match_string(string.clone()) else { return Err(()) };
        let pattern = self.0.replace_variables(replacements)?;
        Ok(SpecialCase {
            expr_components: Vec::new(),
            string_components: Vec::new(),
            vecified_whole: pattern.try_into()?,
        })
    }
}
