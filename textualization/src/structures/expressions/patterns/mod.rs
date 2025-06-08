use either::Either;

use crate::{helpers::lexing::Lexer, structures::expressions::patterns::expr_pattern::{ExprPattern, ExprPatternComponent, ExprPatternLexer, ExprPatternToken}, Destringify, Stringify, Stringifier};

pub mod expr_pattern;
pub mod variable_assignments;

#[derive(Default,Clone)]
pub struct ExprPatternParser {
    lexer: Box<ExprPatternLexer>
}
impl ExprPatternParser {
    pub fn new(lexer: Box<ExprPatternLexer>) -> Self {
        Self { lexer }
    }
}

const VAR_INDIC_TOKEN: ExprPatternToken = ExprPatternToken::VariableIndicator;
const VAR_ENUM_TOKEN: ExprPatternToken = ExprPatternToken::VariableEnumerator;

impl Stringifier<ExprPattern> for ExprPatternParser {}
impl Stringify<ExprPattern> for ExprPatternParser {
    fn stringify(&self, pattern: &ExprPattern) -> Result<String,()> {
        let var_indic_token = self.lexer.string_from_token(&VAR_INDIC_TOKEN);
        let var_enum_token = self.lexer.string_from_token(&VAR_ENUM_TOKEN);
        let mut string = "".to_string();
        for component in pattern.get_components() { match component {
            ExprPatternComponent::Constant(constant) => { string = string + 
                constant
            }, ExprPatternComponent::Variable(var) => { string = string +
                var_indic_token + var
            }, ExprPatternComponent::Variables((var1, var2), sep) => { string = string + 
                var_indic_token + var1 + 
                var_enum_token + sep + var_enum_token +
                var_indic_token + var2
            },
        }}
        Ok(string)
    }
}
enum VarDeclarationStage { Begin, FirstIndic, FirstVar, FirstEnum, Sep, SecondEnum, SecondIndic }
impl Destringify<ExprPattern> for ExprPatternParser {
    fn destringify(&self, string: &String) -> Result<ExprPattern,()> {
        let token_sequence = self.lexer.destringify(&string)?;
        let mut components = Vec::new();
        let mut var_declaration_stage = VarDeclarationStage::Begin;
        for token_or_string in token_sequence.0 { match token_or_string {
            Either::Right(string) => { match var_declaration_stage {
                VarDeclarationStage::Begin => components.push(ExprPatternComponent::Constant(string)),
                VarDeclarationStage::FirstIndic => todo!(),
                VarDeclarationStage::FirstVar => todo!(),
                VarDeclarationStage::FirstEnum => todo!(),
                VarDeclarationStage::Sep => todo!(),
                VarDeclarationStage::SecondEnum => todo!(),
                VarDeclarationStage::SecondIndic => todo!(),
            }}, Either::Left(token) => match token {
                ExprPatternToken::VariableIndicator => var_declaration_stage = match var_declaration_stage {
                    VarDeclarationStage::Begin | VarDeclarationStage::FirstVar => VarDeclarationStage::FirstIndic,
                    VarDeclarationStage::SecondEnum => VarDeclarationStage::SecondIndic,
                    _ => return Err(())
                }, ExprPatternToken::VariableEnumerator => var_declaration_stage = match var_declaration_stage {
                    VarDeclarationStage::FirstVar => VarDeclarationStage::FirstEnum,
                    VarDeclarationStage::FirstEnum | VarDeclarationStage::Sep => VarDeclarationStage::SecondEnum,
                    _ => return Err(())
                },
            }, Either::Left(token) => {
                let s = self.lexer.string_from_token(&token);
                components.push(ExprPatternComponent::Constant(s.clone()))
            }
        }}
        Ok(ExprPattern::new(components, self.lexer.clone()))
    }
}

#[cfg(test)]
mod tests {
    use std::sync::LazyLock;

    use super::*;

    const TEST_PARSER: LazyLock<Box<ExprPatternParser>> = LazyLock::new(|| -> Box<ExprPatternParser> 
        { Box::new(ExprPatternParser::default()) }
    );

    fn pre_stringify_test(string: &str, components: Vec<ExprPatternComponent>) -> (Result<String,()>,String) {
        let pattern = ExprPattern::new(components,TEST_PARSER.clone().lexer);
        (TEST_PARSER.stringify(&pattern), string.to_string())
    }

    fn pre_destringify_test(string: &str, components: Vec<ExprPatternComponent>) -> (Result<ExprPattern,()>,ExprPattern) {
        let pattern = ExprPattern::new(components,TEST_PARSER.clone().lexer);
        (TEST_PARSER.destringify(&string.to_string()),pattern)
    }
    
    #[test]
    fn test_parse_with_const() {
        let components = vec![ExprPatternComponent::new_const("AA")];
        let (result, check) = pre_stringify_test("AA", components);
        assert_eq!(result, Ok(check));
    }

    #[test]
    fn test_unparse_with_const() {
        let components = vec![ExprPatternComponent::new_const("AA")];
        let (result, check) = pre_destringify_test("AA", components);
        assert_eq!(result, Ok(check));
    }

    #[test]
    fn test_parse_with_var() {
        let components = vec![ExprPatternComponent::new_var("Potato ")];
        let (result, check) = pre_stringify_test("#Potato ", components);
        assert_eq!(result, Ok(check));
    }

    #[test]
    fn test_deparse_with_var() {
        let components = vec![ExprPatternComponent::new_var("Potato ")];
        let (result, check) = pre_destringify_test("#Potato ", components);
        assert_eq!(result, Ok(check));
    }

    #[test]
    fn test_parse_with_vars_no_joiner() {
        let components = vec![ExprPatternComponent::new_vars("A","","B")];
        let (result, check) = pre_stringify_test("#A....#B", components);
        assert_eq!(result, Ok(check));
    }

    #[test]
    fn test_deparse_with_vars_no_joiner() {
        let components = vec![ExprPatternComponent::new_vars("A","","B")];
        let (result, check) = pre_destringify_test("#A....#B", components);
        assert_eq!(result, Ok(check));
    }

    #[test]
    fn test_parse_with_vars_and_joiner() {
        let components = vec![ExprPatternComponent::new_vars("A"," & ","B")];
        let (result, check) = pre_stringify_test("#A.. & ..#B", components);
        assert_eq!(result, Ok(check));
    }

    #[test]
    fn test_deparse_with_vars_and_joiner() {
        let components = vec![ExprPatternComponent::new_vars("A"," & ","B")];
        let (result, check) = pre_destringify_test("#A.. & ..#B", components);
        assert_eq!(result, Ok(check));
    }

    #[test]
    fn test_parse_with_complex_string() {
        let components = vec![ExprPatternComponent::new_const("("), ExprPatternComponent::new_var("G"), ExprPatternComponent::new_const(",(f,"), ExprPatternComponent::new_vars("A"," & ","B"), ExprPatternComponent::new_const("))")];
        let (result, check) = pre_stringify_test("(#G,(f,#A.. & ..#B))", components);
        assert_eq!(result, Ok(check));
    }

    #[test]
    fn test_deparse_with_complex_string() {
        let components = vec![ExprPatternComponent::new_const("("), ExprPatternComponent::new_var("G"), ExprPatternComponent::new_const(",(f,"), ExprPatternComponent::new_vars("A"," & ","B"), ExprPatternComponent::new_const("))")];
        let (result, check) = pre_destringify_test("(#G,(f,#A.. & ..#B))", components);
        assert_eq!(result, Ok(check));
    }
}
