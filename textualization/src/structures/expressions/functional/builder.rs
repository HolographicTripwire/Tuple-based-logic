use crate::structures::expressions::{patterns::special_case::ExprPatternPair, raw::RawExpressionControls, SpecialCases};

use super::special_case::*;

pub struct SpecialCasesBuilder {
    controls: RawExpressionControls,
    atomic_symbols: Vec<ExprPatternPair>,
    functions: Vec<ExprPatternPair>
}
impl SpecialCasesBuilder {
    pub fn new(controls: RawExpressionControls) -> Self { Self { controls, atomic_symbols: Vec::new(), functions: Vec::new() } }
    pub fn build<'a>(&self) -> SpecialCases<'a> {
        self.atomic_symbols.iter().chain(self.functions.iter())
            .map(|pair| -> Box<dyn SpecialCase<'a>> { Box::new(pair.clone()) }).collect()
    }

    pub fn add_atomic_symbol(&mut self, atom_id: usize, symbol: String) {
        self.atomic_symbols.push(symbol_atom(atom_id, &symbol, &self.controls));
    }

    pub fn add_atomic_prefix_function(&mut self, atom_id: usize, nonfix_symbol: String, prefix_symbol: String) {
        self.atomic_symbols.push(symbol_atom(atom_id, &nonfix_symbol, &self.controls));
        self.atomic_symbols.push(prefix_function(&nonfix_symbol, &prefix_symbol, &self.controls))
    }

    pub fn add_atomic_infix_function(&mut self, atom_id: usize, nonfix_symbol: String, infix_symbol: String) {
        self.atomic_symbols.push(symbol_atom(atom_id, &nonfix_symbol, &self.controls));
        self.atomic_symbols.push(infix_function(&nonfix_symbol, &infix_symbol, &self.controls))
    }

    pub fn add_atomic_postfix_function(&mut self, atom_id: usize, nonfix_symbol: String, postfix_symbol: String) {
        self.atomic_symbols.push(symbol_atom(atom_id, &nonfix_symbol, &self.controls));
        self.atomic_symbols.push(postfix_function(&nonfix_symbol, &postfix_symbol, &self.controls))
    }
}
