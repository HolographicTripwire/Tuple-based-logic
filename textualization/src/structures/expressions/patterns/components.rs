use parsertools::Parser;

use crate::{helpers::{parsers::{controlled::{controlled_word_parser, ControlStrings}, string_parser}, styles::Style}, structures::expressions::patterns::{assignments::ExprPatternAssignment, parser::ExprPatternStyle}};

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ExprPatternComponent {
    Constant(String),
    Variable(String),
    Variables((String,String),String),
}
impl ExprPatternComponent {
    pub fn new_const(const_string: &str) -> Self { Self::Constant(const_string.to_string()) }
    pub fn new_var(var_name: &str) -> Self { Self::Variable(var_name.to_string()) }
    pub fn new_vars(from: &str, joiner: &str, to: &str) -> Self { Self::Variables((from.to_string(),to.to_string()),joiner.to_string()) }
    pub (super) fn assign(&self, assignment: &ExprPatternAssignment) -> ExprPatternComponent { match assignment {
            ExprPatternAssignment::Constant => self.clone(),
            ExprPatternAssignment::Variable(var, val) => {
                let ExprPatternComponent::Variable(self_var) = self else { return self.clone() };
                if var == self_var { ExprPatternComponent::Constant(val.clone()) } else { self.clone() }
            }, ExprPatternAssignment::Variables((var1, var2), vals) => {
                let ExprPatternComponent::Variables((self_var1,self_var2), sep) = self else { return self.clone() };
                if (var1 == self_var1) & (var2 == self_var2) { ExprPatternComponent::Constant(vals.join(sep)) } else { self.clone() }
            }
        }
    }
}
impl ExprPatternStyle {
    pub fn const_parser<'a>(&self, blacklist: ControlStrings) -> Parser<'a, char, ExprPatternComponent>
        { controlled_word_parser(blacklist).map(|s| ExprPatternComponent::Constant(s)) }
    pub fn var_parser<'a>(&self, blacklist: ControlStrings) -> Parser<'a, char, ExprPatternComponent> {
        let var_indic_parser = self.var_indic_parser();
        var_indic_parser.clone()
            .then(controlled_word_parser(blacklist))
            .then(var_indic_parser)
            .map(|((_,s),_)| ExprPatternComponent::Variable(s))
    }
    pub fn vars_parser<'a>(&self, blacklist: ControlStrings) -> Parser<'a, char, ExprPatternComponent> {
        let var_indic_parser = self.var_indic_parser();
        let var_enum_parser = self.var_enum_parser();
        let word_parser = controlled_word_parser(blacklist);
        var_indic_parser.clone()
            .then(word_parser.clone()).then(var_enum_parser.clone())
            .then(word_parser.clone()).then(var_enum_parser)
            .then(word_parser.clone()).then(var_indic_parser)
            .map(|((((((_,v1),_),sep),_),v2),_)| ExprPatternComponent::Variables((v1,v2),sep))
    }

    fn var_indic_parser<'a>(&self) -> Parser<'a,char,()> { string_parser(self.var_indic()).unwrap().map(|_| ()) }
    fn var_enum_parser<'a>(&self) -> Parser<'a,char,()> { string_parser(self.var_enum()).unwrap().map(|_| ()) }
}
impl Style<ExprPatternComponent> for ExprPatternStyle {
    type ParseParams = ControlStrings;

    fn stringify(&self, stylable: &ExprPatternComponent) -> String {
        match stylable {
            ExprPatternComponent::Constant(_) => todo!(),
            ExprPatternComponent::Variable(_) => todo!(),
            ExprPatternComponent::Variables(_, _) => todo!(),
        }
    }
    
    fn parser<'a>(&self, params: Self::ParseParams) -> Parser<'a,char,ExprPatternComponent> {
        self.const_parser(params.clone())
            .or(self.var_parser(params.clone()))
            .or(self.vars_parser(params))
    }
}

#[cfg(test)]
mod tests {

    use crate::{structures::expressions::patterns::components::{ExprPatternAssignment, ExprPatternComponent}};

    #[test]
    fn test_assign_with_const_and_const() {
        assert_eq!(
            ExprPatternComponent::new_const("fewhuow").assign(&ExprPatternAssignment::new_const()),
            ExprPatternComponent::new_const("fewhuow")
        )
    }

    #[test]
    fn test_assign_with_const_and_var() {
        assert_eq!(
            ExprPatternComponent::new_const("fewhuow").assign(&ExprPatternAssignment::new_var("fewhuow","dawuhofawohu")),
            ExprPatternComponent::new_const("fewhuow")
        )
    }

    #[test]
    fn test_assign_with_const_and_vars() {
        assert_eq!(
            ExprPatternComponent::new_const("fewhuow").assign(&ExprPatternAssignment::new_vars("fewhuow","fewhuow",vec!["odwfawaio", "groahba0"])),
            ExprPatternComponent::new_const("fewhuow")
        )
    }

    #[test]
    fn test_assign_with_var_and_const() {
        assert_eq!(
            ExprPatternComponent::new_var("grogr").assign(&ExprPatternAssignment::new_const()),
            ExprPatternComponent::new_var("grogr")
        )
    }

    #[test]
    fn test_assign_with_var_and_var_unaligned() {
        assert_eq!(
            ExprPatternComponent::new_const("grogr").assign(&ExprPatternAssignment::new_var("fewhuow","dawuhofawohu")),
            ExprPatternComponent::new_const("grogr")
        )
    }

    #[test]
    fn test_assign_with_var_and_var_aligned() {
        assert_eq!(
            ExprPatternComponent::new_var("grogr").assign(&ExprPatternAssignment::new_var("grogr","dawuhofawohu")),
            ExprPatternComponent::new_const("dawuhofawohu")
        )
    }

    #[test]
    fn test_assign_with_var_and_vars() {
        assert_eq!(
            ExprPatternComponent::new_var("grogr").assign(&ExprPatternAssignment::new_vars("grogr","grogr",vec!["odwfawaio", "groahba0"])),
            ExprPatternComponent::new_var("grogr")
        )
    }

    #[test]
    fn test_assign_with_vars_and_const() {
        assert_eq!(
            ExprPatternComponent::new_vars("kbnfeoji","bdijak","ijoaef").assign(&ExprPatternAssignment::new_const()),
            ExprPatternComponent::new_vars("kbnfeoji","bdijak","ijoaef")
        )
    }

    #[test]
    fn test_assign_with_vars_and_var() {
        assert_eq!(
            ExprPatternComponent::new_vars("kbnfeoji","bdijak","ijoaef").assign(&ExprPatternAssignment::new_var("fewhuow","dawuhofawohu")),
            ExprPatternComponent::new_vars("kbnfeoji","bdijak","ijoaef")
        )
    }

    #[test]
    fn test_assign_with_vars_and_vars_unaligned() {
        assert_eq!(
            ExprPatternComponent::new_vars("kbnfeoji","bdijak","ijoaef").assign(&ExprPatternAssignment::new_vars("fewhuow","fewhuow",vec!["odwfawaio", "groahba0"])),
            ExprPatternComponent::new_vars("kbnfeoji","bdijak","ijoaef")
        )
    }

    #[test]
    fn test_assign_with_vars_and_vars_aligned() {
        assert_eq!(
            ExprPatternComponent::new_vars("kbnfeoji","ijoaef","bdijak").assign(&ExprPatternAssignment::new_vars("kbnfeoji","bdijak",vec!["odwfawaio", "groahba0"])),
            ExprPatternComponent::new_const("odwfawaioijoaefgroahba0")
        )
    }
}
