use enum_iterator::Sequence;
use vec::{VecControl, VecControls};

use crate::helpers::{lexing::{Token, Lexer}, patterns::{ExprPatternControl, ExprPatternControls}};

pub mod atom;
pub mod vec;
pub mod expressions;

#[derive(Sequence, Clone, Copy)]
pub enum TblStringifierControl { Vec(VecControl), Pattern (ExprPatternControl) }
impl Token for TblStringifierControl {}

#[derive(Clone)]
pub struct TblStringifierControls{
    escape_string: String,
    vec_controls: VecControls,
    pattern_controls: ExprPatternControls,
}
impl TblStringifierControls {
    pub fn new(escape_string: String, vec_controls: VecControls, pattern_controls: ExprPatternControls) -> Self
        { Self { escape_string, vec_controls, pattern_controls } }
}
impl Lexer<TblStringifierControl> for TblStringifierControls {
    fn string_from_control(&self, control: &TblStringifierControl) -> &String { match control {
        TblStringifierControl::Vec(vec_control) => self.vec_controls.string_from_control(vec_control),
        TblStringifierControl::Pattern(pattern_control) => self.pattern_controls.string_from_control(pattern_control),
    }}
    
    fn escape_string(&self) -> &String { &self.escape_string }
}
impl Default for TblStringifierControls {
    fn default() -> Self { Self {     
        escape_string: "\\".to_string(),
        vec_controls: VecControls::default(),
        pattern_controls: ExprPatternControls::default()
    }}
}
