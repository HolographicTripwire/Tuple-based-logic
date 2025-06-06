pub mod special_cases;

pub use special_cases::{SpecialCase};

use tbl_structures::{atoms::AtomId, propositions::Expression};

use crate::{Destringify, Stringifier, Stringify};

trait ExpressionStringifyInterface<A: Stringify<AtomId>, V: Stringify<Vec<String>>, S: Stringify<SpecialCase>> {
    // Simple getters
    fn atoms(&self) -> &A;
    fn vecs(&self) -> &V;
    fn special_cases(&self) -> &S;
    /// This function will be used by [ExpressionStringifier] and [ExpressionStringify] as a proxy for [Stringify::stringify]. 
    fn to_text(&self, expr: &Expression) -> Result<String,()> {
        match expr {
            Expression::Atomic(atom_id) => self.atoms().stringify(&atom_id),
            Expression::Tuple(expr_components) => {
                // Convert each expression in the tuple to a string
                let string_components: Vec<String> = expr_components
                    .iter()
                    .map(|expr| -> Result<String,()> { self.to_text(expr) })
                    .collect::<Result<Vec<String>,()>>()?;
                let vecified_whole = self.vecs().stringify(&string_components)?;
                // Pair expressions with strings
                let special_case = SpecialCase { 
                    expr_components: expr_components.clone(),
                    string_components, 
                    vecified_whole: vecified_whole.clone()
                };
                
                // If there are any optional rules, apply them
                if let Ok(string) = self.special_cases().stringify(&special_case) { Ok(string) }
                // Otherwise just treat this vec as we would any other vec
                else { Ok(vecified_whole) }
            },
        }
    }
}
trait ExpressionDestringifyInterface<A: Destringify<AtomId>, V: Destringify<Vec<String>>, S: Destringify<SpecialCase>> {
    fn atoms(&self) -> &A;
    fn vecs(&self) -> &V;
    fn special_cases(&self) -> &S;
    /// This function will be used by [ExpressionStringifier] and [ExpressionDestringify] as a proxy for [Destringify::destringify]. 
    fn from_text(&self, string: &String) -> Result<Expression,()> {
        // Remove whitespace on either side of the string
        let trimmed = string.trim().to_string();
        
        // Try to interpret the provided string with each of our inner textualizers
        let atom_result = self.atoms().destringify(&trimmed);
        let tuple_result = self.vecs().destringify(&trimmed);
        let optional_rules_result = self.special_cases().destringify(&trimmed);
        // Calculate the number of valid interpretations we found
        let ok_results = (atom_result.is_ok() as u8) + (tuple_result.is_ok() as u8) + (optional_rules_result.is_ok() as u8);
        
        // Throw an error if this string has multiple valid interpretations
        if ok_results > 1 { Err(()) }
        // If there is only a single valid interpretation, use that one
        else if let Ok(atom) = atom_result { Ok(Expression::Atomic(atom)) }
        else if let Ok(strings) = tuple_result {
            let exprs: Result<Vec<Expression>,()> = strings.iter()
                .map(|s| -> Result<Expression,()> { self.from_text(s) })
                .collect();
            Ok(Expression::Tuple(exprs?)) 
        } else if let Ok(special_case) = optional_rules_result {
            let exprs: Result<Vec<Expression>,()> = special_case.string_components.iter()
                .map(|string| -> Result<Expression,()> { self.from_text(string) })
                .collect();
            Ok(Expression::Tuple(exprs?))
        }
        // Throw an error if this string has no interpretations
        else { Err(()) }
    }
}

/// A struct that can both Stringify and Destringify expressions
pub struct ExpressionStringifier<A: Stringifier<AtomId>, V: Stringifier<Vec<String>>, S: Stringifier<SpecialCase>> {
    atoms: A,
    vecs: V,
    special_cases: S,
}
/// A struct that can stringify, but not destringify, expressions
pub struct ExpressionStringify<A: Stringify<AtomId>, V: Stringify<Vec<String>>, S: Stringify<SpecialCase>> {
    atoms: A,
    vecs: V,
    special_cases: S,
}
/// A struct that can destringify, but not stringify, expressions
pub struct ExpressionDestringify<A: Destringify<AtomId>, V: Destringify<Vec<String>>, S: Destringify<SpecialCase>> {
    atoms: A,
    vecs: V,
    special_cases: S,
}

// Implement new for Expression stringification structs
impl <A: Stringifier<AtomId>, V: Stringifier<Vec<String>>, S: Stringifier<SpecialCase>> ExpressionStringifier<A,V,S> {
    pub fn new(atoms: A, vecs: V, special_cases: S) -> Self { Self {atoms, vecs, special_cases} }
}
impl <A: Stringify<AtomId>, V: Stringify<Vec<String>>, S: Stringify<SpecialCase>> ExpressionStringify<A,V,S> {
    pub fn new(atoms: A, vecs: V, special_cases: S) -> Self { Self {atoms, vecs, special_cases} }
}
impl <A: Destringify<AtomId>, V: Destringify<Vec<String>>, S: Destringify<SpecialCase>> ExpressionDestringify<A,V,S> {
    pub fn new(atoms: A, vecs: V, special_cases: S) -> Self { Self {atoms, vecs, special_cases} }
}

// Implement the expression stringifier interface for Expression stringification structs
impl <A: Stringifier<AtomId>, V: Stringifier<Vec<String>>, S: Stringifier<SpecialCase>> ExpressionStringifyInterface<A,V,S> for ExpressionStringifier<A,V,S> {
    fn atoms(&self) -> &A { &self.atoms }
    fn vecs(&self) -> &V { &self.vecs }
    fn special_cases(&self) -> &S { &self.special_cases }
}
impl <A: Stringify<AtomId>, V: Stringify<Vec<String>>, S: Stringify<SpecialCase>> ExpressionStringifyInterface<A,V,S> for ExpressionStringify<A,V,S> {
    fn atoms(&self) -> &A { &self.atoms }
    fn vecs(&self) -> &V { &self.vecs }
    fn special_cases(&self) -> &S { &self.special_cases }
}
impl <A: Stringifier<AtomId>, V: Stringifier<Vec<String>>, S: Stringifier<SpecialCase>> ExpressionDestringifyInterface<A,V,S> for ExpressionStringifier<A,V,S> {
    fn atoms(&self) -> &A { &self.atoms }
    fn vecs(&self) -> &V { &self.vecs }
    fn special_cases(&self) -> &S { &self.special_cases }
}
impl <A: Destringify<AtomId>, V: Destringify<Vec<String>>, S: Destringify<SpecialCase>> ExpressionDestringifyInterface<A,V,S> for ExpressionDestringify<A,V,S> {
    fn atoms(&self) -> &A { &self.atoms }
    fn vecs(&self) -> &V { &self.vecs }
    fn special_cases(&self) -> &S { &self.special_cases }
}

// Implement Stringify and Destringifiy for the stringification structs
impl <A: Stringifier<AtomId>, V: Stringifier<Vec<String>>, S: Stringifier<SpecialCase>> Stringify<Expression> for ExpressionStringifier<A,V,S>  { 
    fn stringify(&self, expr: &Expression) -> Result<String,()> { self.to_text(expr) }
}
impl <A: Stringifier<AtomId>, V: Stringifier<Vec<String>>, S: Stringifier<SpecialCase>> Stringify<Expression> for ExpressionStringify<A,V,S>  { 
    fn stringify(&self, expr: &Expression) -> Result<String,()> { self.to_text(expr) }
}
impl <A: Stringifier<AtomId>, V: Stringifier<Vec<String>>, S: Stringifier<SpecialCase>> Destringify<Expression> for ExpressionStringifier<A,V,S> { 
    fn destringify(&self, string: &String) -> Result<Expression,()> { self.from_text(string) }
}
impl <A: Stringifier<AtomId>, V: Stringifier<Vec<String>>, S: Stringifier<SpecialCase>> Destringify<Expression> for ExpressionDestringify<A,V,S> { 
    fn destringify(&self, string: &String) -> Result<Expression,()> { self.from_text(string) }
}

// Implement Stringifier for the one stringifier type
impl <A: Stringifier<AtomId>, V: Stringifier<Vec<String>>, S: Stringifier<SpecialCase>> Stringifier<Expression> for ExpressionStringifier<A,V,S> {}
