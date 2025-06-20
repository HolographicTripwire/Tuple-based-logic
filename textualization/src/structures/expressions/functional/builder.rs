use crate::structures::expressions::{patterns::special_case::ExprPatternPair, raw::RawExpressionStyle, SpecialCase, SpecialCases};

use super::special_case::*;

pub struct SpecialCasesBuilder {
    style: RawExpressionStyle,
    atomic_symbols: Vec<ExprPatternPair>,
    functions: Vec<ExprPatternPair>
}
impl SpecialCasesBuilder {
    pub fn new(style: RawExpressionStyle) -> Self { Self { style, atomic_symbols: Vec::new(), functions: Vec::new() } }
    pub fn build<'a>(&self) -> SpecialCases<'a> {
        self.atomic_symbols.iter().chain(self.functions.iter())
            .map(|pair| -> Box<dyn SpecialCase<'a>> { Box::new(pair.clone()) }).collect()
    }

    pub fn add_atomic_symbol(mut self, atom_id: usize, symbol: String) -> Self {
        self.atomic_symbols.push(symbol_atom(atom_id, &symbol, &self.style));
        self
    }

    pub fn add_variadic_atomic_prefix_function(mut self, atom_id: usize, nonfix_symbol: &str, prefix_symbol: &str) -> Self {
        self.atomic_symbols.push(symbol_atom(atom_id, nonfix_symbol, &self.style));
        self.atomic_symbols.push(variadic_prefix_function(nonfix_symbol, prefix_symbol, &self.style));
        self
    }
    pub fn add_atomic_prefix_function<I: IntoIterator<Item = usize>>(mut self, atom_id: usize, arities: I, nonfix_symbol: &str, prefix_symbol: &str) -> Self {
        self.atomic_symbols.push(symbol_atom(atom_id, nonfix_symbol, &self.style));
        for arity in arities.into_iter() {
            self.atomic_symbols.push(prefix_function(nonfix_symbol, arity, prefix_symbol, &self.style));
        } self
    }

    pub fn add_variadic_atomic_infix_function(mut self, atom_id: usize, nonfix_symbol: &str, infix_symbol: &str) -> Self {
        self.atomic_symbols.push(symbol_atom(atom_id, nonfix_symbol, &self.style));
        self.atomic_symbols.push(variadic_infix_function(nonfix_symbol, &infix_symbol, &self.style));
        self
    }
    pub fn add_atomic_infix_function<I: IntoIterator<Item = usize>>(mut self, atom_id: usize, arities: I, nonfix_symbol: &str, infix_symbol: &str) -> Self {
        self.atomic_symbols.push(symbol_atom(atom_id, nonfix_symbol, &self.style));
        for arity in arities.into_iter() {
            self.atomic_symbols.push(infix_function(nonfix_symbol, arity, &infix_symbol, &self.style));
        } self
    }

    pub fn add_variadic_atomic_postfix_function(mut self, atom_id: usize, nonfix_symbol: &str, postfix_symbol: &str) -> Self {
        self.atomic_symbols.push(symbol_atom(atom_id, nonfix_symbol, &self.style));
        self.atomic_symbols.push(variadic_postfix_function(nonfix_symbol, postfix_symbol, &self.style));
        self
    }
    pub fn add_atomic_postfix_function<I: IntoIterator<Item = usize>>(mut self, atom_id: usize, arities: I, nonfix_symbol: &str, postfix_symbol: &str) -> Self {
        self.atomic_symbols.push(symbol_atom(atom_id, nonfix_symbol, &self.style));
        for arity in arities.into_iter() {
            self.atomic_symbols.push(postfix_function(nonfix_symbol, arity, postfix_symbol, &self.style));
        } self
    }

    pub fn add_variadic_atomic_outfix_function(mut self, atom_id: usize, nonfix_symbol: &str, left_symbol: &str, right_symbol: &str) -> Self {
        self.atomic_symbols.push(symbol_atom(atom_id, nonfix_symbol, &self.style));
        self.atomic_symbols.push(variadic_outfix_function(nonfix_symbol, left_symbol, right_symbol, &self.style));
        self
    }
    pub fn add_atomic_outfix_function<I: IntoIterator<Item = usize>>(mut self, atom_id: usize, arities: I, nonfix_symbol: &str, left_symbol: &str, right_symbol: &str) -> Self {
        self.atomic_symbols.push(symbol_atom(atom_id, nonfix_symbol, &self.style));
        for arity in arities.into_iter() {
            self.atomic_symbols.push(outfix_n_function(nonfix_symbol, arity, left_symbol, right_symbol, &self.style));
        } self
    }
    
    pub fn add_variadic_atomic_allfix_function(mut self, atom_id: usize, nonfix_symbol: &str, left_symbol: &str, infix_symbol: &str, right_symbol: &str) -> Self {
        self.atomic_symbols.push(symbol_atom(atom_id, nonfix_symbol, &self.style));
        self.atomic_symbols.push(variadic_allfix_function(nonfix_symbol, left_symbol, infix_symbol, right_symbol, &self.style));
        self
    }
    pub fn add_atomic_allfix_function<I: IntoIterator<Item = usize>>(mut self, atom_id: usize, arities: I, nonfix_symbol: &str, left_symbol: &str, infix_symbol: &str, right_symbol: &str) -> Self {
        self.atomic_symbols.push(symbol_atom(atom_id, nonfix_symbol, &self.style));
        for arity in arities.into_iter() {
            self.atomic_symbols.push(allfix_function(nonfix_symbol, arity, left_symbol, infix_symbol, right_symbol, &self.style));
        } self
    }
}
