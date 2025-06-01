use either::Either;

use crate::{helpers::controls::Controls, structures::{TblStringifierControl, TblStringifierControls}, Destringify, Stringifier, Stringify};

use super::{ExprPattern, ExprPatternComponent, ExprPatternControl};

pub struct ExprPatternStringifier {
    controls: Box<TblStringifierControls>
}
impl ExprPatternStringifier {
    pub fn new(controls: Box<TblStringifierControls>) -> Self {
        Self { controls }
    }
}

const VAR_INDIC_CONTROL: TblStringifierControl = TblStringifierControl::Pattern(ExprPatternControl::VariableIndicator);
const VAR_ENUM_CONTROL: TblStringifierControl = TblStringifierControl::Pattern(ExprPatternControl::VariableIndicator);

impl Stringifier<ExprPattern> for ExprPatternStringifier {}
impl Stringify<ExprPattern> for ExprPatternStringifier {
    fn stringify(&self, pattern: &ExprPattern) -> Result<String,()> {
        let var_indic_control = self.controls.string_from_control(&VAR_INDIC_CONTROL);
        let var_enum_control = self.controls.string_from_control(&VAR_ENUM_CONTROL);
        let mut string = "".to_string();
        for component in &pattern.components { match component {
            ExprPatternComponent::Constant(constant) => { string = string + 
                constant
            }, ExprPatternComponent::Variable(var) => { string = string +
                var_indic_control + var
            }, ExprPatternComponent::Variables((var1, var2), sep) => { string = string + 
                var_indic_control + var1 + 
                var_enum_control + sep + var_enum_control +
                var_indic_control + var2
            },
        }}
        Ok(string)
    }
}
enum VarDeclarationStage { Begin, FirstIndic, FirstVar, FirstEnum, Sep, SecondEnum, SecondIndic }
impl Destringify<ExprPattern> for ExprPatternStringifier {
    fn destringify(&self, string: &String) -> Result<ExprPattern,()> {
        let control_sequence = self.controls.destringify(&string)?;
        let mut components = Vec::new();
        let mut var_declaration_stage = VarDeclarationStage::Begin;
        for control_or_string in control_sequence.0 { match control_or_string {
            Either::Right(string) => { match var_declaration_stage {
                VarDeclarationStage::Begin => components.push(ExprPatternComponent::Constant(string)),
                VarDeclarationStage::FirstIndic => todo!(),
                VarDeclarationStage::FirstVar => todo!(),
                VarDeclarationStage::FirstEnum => todo!(),
                VarDeclarationStage::Sep => todo!(),
                VarDeclarationStage::SecondEnum => todo!(),
                VarDeclarationStage::SecondIndic => todo!(),
            }}, Either::Left(TblStringifierControl::Pattern(control)) => match control {
                ExprPatternControl::VariableIndicator => var_declaration_stage = match var_declaration_stage {
                    VarDeclarationStage::Begin | VarDeclarationStage::FirstVar => VarDeclarationStage::FirstIndic,
                    VarDeclarationStage::SecondEnum => VarDeclarationStage::SecondIndic,
                    _ => return Err(())
                }, ExprPatternControl::VariableEnumerator => var_declaration_stage = match var_declaration_stage {
                    VarDeclarationStage::FirstVar => VarDeclarationStage::FirstEnum,
                    VarDeclarationStage::FirstEnum | VarDeclarationStage::Sep => VarDeclarationStage::SecondEnum,
                    _ => return Err(())
                },
            }, Either::Left(control) => {
                let s = self.controls.string_from_control(&control);
                components.push(ExprPatternComponent::Constant(s.clone()))
            }
        }}
        Ok(ExprPattern::new(components, self.controls.clone()))
    }
}
