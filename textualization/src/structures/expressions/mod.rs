pub mod patterns;
pub mod special_cases;


pub use special_cases::{SpecialCase};

use tbl_structures::{atoms::AtomId, propositions::Expression};

use crate::{Detextualize, Textualizer, Textualize};

trait ExpressionTextualizeInterface<A: Textualize<AtomId>, V: Textualize<Vec<String>>, S: Textualize<SpecialCase>> {
    // Simple getters
    fn atoms(&self) -> &A;
    fn vecs(&self) -> &V;
    fn special_cases(&self) -> &S;
    /// This function will be used by [ExpressionTextualizer] and [ExpressionTextualize] as a proxy for [Textualize::stringify]. 
    fn to_text(&self, expr: &Expression) -> Result<String,()> {
        match expr {
            Expression::Atomic(atom_id) => self.atoms().textualize(&atom_id),
            Expression::Tuple(expr_components) => {
                // Convert each expression in the tuple to a string
                let string_components: Vec<String> = expr_components
                    .iter()
                    .map(|expr| -> Result<String,()> { self.to_text(expr) })
                    .collect::<Result<Vec<String>,()>>()?;
                let vecified_whole = self.vecs().textualize(&string_components)?;
                // Pair expressions with strings
                let special_case = SpecialCase { 
                    expr_components: expr_components.clone(),
                    string_components, 
                    vecified_whole: vecified_whole.clone()
                };
                
                // If there are any optional rules, apply them
                if let Ok(string) = self.special_cases().textualize(&special_case) { Ok(string) }
                // Otherwise just treat this vec as we would any other vec
                else { Ok(vecified_whole) }
            },
        }
    }
}
trait ExpressionDetextualizeInterface<A: Detextualize<AtomId>, V: Detextualize<Vec<String>>, S: Detextualize<SpecialCase>> {
    fn atoms(&self) -> &A;
    fn vecs(&self) -> &V;
    fn special_cases(&self) -> &S;
    /// This function will be used by [ExpressionTextualizer] and [ExpressionDetextualize] as a proxy for [Detextualize::destringify]. 
    fn from_text(&self, string: &String) -> Result<Expression,()> {
        // Remove whitespace on either side of the string
        let trimmed = string.trim().to_string();
        
        // Try to interpret the provided string with each of our inner textualizers
        let atom_result = self.atoms().detextualize(&trimmed);
        let tuple_result = self.vecs().detextualize(&trimmed);
        let optional_rules_result = self.special_cases().detextualize(&trimmed);
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

/// A struct that can both Textualize and Detextualize expressions
pub struct ExpressionTextualizer<A: Textualizer<AtomId>, V: Textualizer<Vec<String>>, S: Textualizer<SpecialCase>> {
    atoms: A,
    vecs: V,
    special_cases: S,
}
/// A struct that can stringify, but not destringify, expressions
pub struct ExpressionTextualize<A: Textualize<AtomId>, V: Textualize<Vec<String>>, S: Textualize<SpecialCase>> {
    atoms: A,
    vecs: V,
    special_cases: S,
}
/// A struct that can destringify, but not stringify, expressions
pub struct ExpressionDetextualize<A: Detextualize<AtomId>, V: Detextualize<Vec<String>>, S: Detextualize<SpecialCase>> {
    atoms: A,
    vecs: V,
    special_cases: S,
}

// Implement new for Expression stringification structs
impl <A: Textualizer<AtomId>, V: Textualizer<Vec<String>>, S: Textualizer<SpecialCase>> ExpressionTextualizer<A,V,S> {
    pub fn new(atoms: A, vecs: V, special_cases: S) -> Self { Self {atoms, vecs, special_cases} }
}
impl <A: Textualize<AtomId>, V: Textualize<Vec<String>>, S: Textualize<SpecialCase>> ExpressionTextualize<A,V,S> {
    pub fn new(atoms: A, vecs: V, special_cases: S) -> Self { Self {atoms, vecs, special_cases} }
}
impl <A: Detextualize<AtomId>, V: Detextualize<Vec<String>>, S: Detextualize<SpecialCase>> ExpressionDetextualize<A,V,S> {
    pub fn new(atoms: A, vecs: V, special_cases: S) -> Self { Self {atoms, vecs, special_cases} }
}

// Implement the expression stringifier interface for Expression stringification structs
impl <A: Textualizer<AtomId>, V: Textualizer<Vec<String>>, S: Textualizer<SpecialCase>> ExpressionTextualizeInterface<A,V,S> for ExpressionTextualizer<A,V,S> {
    fn atoms(&self) -> &A { &self.atoms }
    fn vecs(&self) -> &V { &self.vecs }
    fn special_cases(&self) -> &S { &self.special_cases }
}
impl <A: Textualize<AtomId>, V: Textualize<Vec<String>>, S: Textualize<SpecialCase>> ExpressionTextualizeInterface<A,V,S> for ExpressionTextualize<A,V,S> {
    fn atoms(&self) -> &A { &self.atoms }
    fn vecs(&self) -> &V { &self.vecs }
    fn special_cases(&self) -> &S { &self.special_cases }
}
impl <A: Textualizer<AtomId>, V: Textualizer<Vec<String>>, S: Textualizer<SpecialCase>> ExpressionDetextualizeInterface<A,V,S> for ExpressionTextualizer<A,V,S> {
    fn atoms(&self) -> &A { &self.atoms }
    fn vecs(&self) -> &V { &self.vecs }
    fn special_cases(&self) -> &S { &self.special_cases }
}
impl <A: Detextualize<AtomId>, V: Detextualize<Vec<String>>, S: Detextualize<SpecialCase>> ExpressionDetextualizeInterface<A,V,S> for ExpressionDetextualize<A,V,S> {
    fn atoms(&self) -> &A { &self.atoms }
    fn vecs(&self) -> &V { &self.vecs }
    fn special_cases(&self) -> &S { &self.special_cases }
}

// Implement Textualize and Destringifiy for the stringification structs
impl <A: Textualizer<AtomId>, V: Textualizer<Vec<String>>, S: Textualizer<SpecialCase>> Textualize<Expression> for ExpressionTextualizer<A,V,S>  { 
    fn textualize(&self, expr: &Expression) -> Result<String,()> { self.to_text(expr) }
}
impl <A: Textualizer<AtomId>, V: Textualizer<Vec<String>>, S: Textualizer<SpecialCase>> Textualize<Expression> for ExpressionTextualize<A,V,S>  { 
    fn textualize(&self, expr: &Expression) -> Result<String,()> { self.to_text(expr) }
}
impl <A: Textualizer<AtomId>, V: Textualizer<Vec<String>>, S: Textualizer<SpecialCase>> Detextualize<Expression> for ExpressionTextualizer<A,V,S> { 
    fn detextualize(&self, string: &String) -> Result<Expression,()> { self.from_text(string) }
}
impl <A: Textualizer<AtomId>, V: Textualizer<Vec<String>>, S: Textualizer<SpecialCase>> Detextualize<Expression> for ExpressionDetextualize<A,V,S> { 
    fn detextualize(&self, string: &String) -> Result<Expression,()> { self.from_text(string) }
}

// Implement Textualizer for the one stringifier type
impl <A: Textualizer<AtomId>, V: Textualizer<Vec<String>>, S: Textualizer<SpecialCase>> Textualizer<Expression> for ExpressionTextualizer<A,V,S> {}
