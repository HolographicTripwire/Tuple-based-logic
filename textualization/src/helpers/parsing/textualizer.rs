use either::Either;

use crate::{helpers::lexing::Lexer, structures::{TblToken, TblLexer}, Detextualize, Textualizer, Textualize};

use super::{ExprPattern, ExprPatternComponent, ExprPatternToken};

pub struct ExprPatternTextualizer {
    lexer: Box<TblLexer>
}
impl ExprPatternTextualizer {
    pub fn new(lexer: Box<TblLexer>) -> Self {
        Self { lexer }
    }
}

const VAR_INDIC_TOKEN: TblToken = TblToken::Pattern(ExprPatternToken::VariableIndicator);
const VAR_ENUM_TOKEN: TblToken = TblToken::Pattern(ExprPatternToken::VariableIndicator);

impl Textualizer<ExprPattern> for ExprPatternTextualizer {}
impl Textualize<ExprPattern> for ExprPatternTextualizer {
    fn textualize(&self, pattern: &ExprPattern) -> Result<String,()> {
        let var_indic_token = self.lexer.string_from_token(&VAR_INDIC_TOKEN);
        let var_enum_token = self.lexer.string_from_token(&VAR_ENUM_TOKEN);
        let mut string = "".to_string();
        for component in &pattern.components { match component {
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
impl Detextualize<ExprPattern> for ExprPatternTextualizer {
    fn detextualize(&self, string: &String) -> Result<ExprPattern,()> {
        let token_sequence = self.lexer.detextualize(&string)?;
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
            }}, Either::Left(TblToken::Pattern(token)) => match token {
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
