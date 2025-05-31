use crate::{helpers::patterns::{ExprPattern, ExprPatternComponent, ExprPatternStringifier}, Destringify, Stringifier, Stringify};

use super::SpecialCase;

pub struct PatternStringifier2(ExprPattern,ExprPattern);

impl PatternStringifier2 {
    fn from_strings(pre: &str, post: &str, pattern_stringifier: ExprPatternStringifier) -> Result<Self,()> {
        Ok(Self(
            pattern_stringifier.destringify(&pre.to_string())?,
            pattern_stringifier.destringify(&post.to_string())?
        ))
    }
}

impl Stringifier<SpecialCase> for PatternStringifier2 {}
impl Stringify<SpecialCase>  for PatternStringifier2{
    fn stringify(&self, object: &SpecialCase) -> Result<String,()> {
        let strings = object.1;
        let Ok(replacements) = self.0.match_string(strings) else { return Err(()) };
        self.1.make_replacements(replacements)
    }
}
impl Destringify<SpecialCase> for PatternStringifier2 {
    fn destringify(&self, string: &String) -> Result<SpecialCase,()> {
        let strings = object.1;
    }
}
