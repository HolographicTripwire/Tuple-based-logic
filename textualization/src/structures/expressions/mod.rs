pub mod patterns;
pub mod special_cases;


pub use special_cases::{SpecialCase};

use tbl_structures::{atoms::AtomId, propositions::Expression};

use crate::{Destringify, Stringifier, Stringify};

trait ExpressionParseInterface {
    // Simple getters
    fn atoms(&self) -> &Box<dyn Stringify<AtomId>>;
    fn vecs(&self) -> &Box<dyn Stringify<Vec<String>>>;
    fn special_cases(&self) -> &Box<dyn Stringify<SpecialCase>>;
    /// This function will be used by [ExpressionTextualizer] and [ExpressionTextualize] as a proxy for [Textualize::stringify]. 
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
trait ExpressionUnparseInterface {
    fn atoms(&self) -> &Box<dyn Destringify<AtomId>>;
    fn vecs(&self) -> &Box<dyn Destringify<Vec<String>>>;
    fn special_cases(&self) -> &Box<dyn Destringify<SpecialCase>>;
    /// This function will be used by [ExpressionStringifier] and [ExpressionDestringifier] as a proxy for [Detextualize::destringify]. 
    fn from_text(&self, string: &String) -> Result<Expression,()> {
        // Remove whitespace on either side of the string
        let trimmed = string.trim().to_string();
        
        // Try to interpret the provided string with each of our inner stringifiers
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

/// A struct that can both Textualize and Destringify expressions
#[derive(Clone)]
pub struct ExpressionParser {
    atoms: Box<dyn Stringifier<AtomId>>,
    vecs: Box<dyn Stringifier<Vec<String>>>,
    special_cases: Box<dyn Stringifier<SpecialCase>>,
}
/// A struct that can stringify, but not destringify, expressions
#[derive(Clone)]
pub struct ExpressionParse {
    atoms: Box<dyn Stringify<AtomId>>,
    vecs: Box<dyn Stringify<Vec<String>>>,
    special_cases: Box<dyn Stringify<SpecialCase>>,
}
/// A struct that can destringify, but not stringify, expressions
#[derive(Clone)]
pub struct ExpressionUnparse {
    atoms: Box<dyn Destringify<AtomId>>,
    vecs: Box<dyn Destringify<Vec<String>>>,
    special_cases: Box<dyn Destringify<SpecialCase>>,
}

// Implement new for Expression stringification structs
impl ExpressionParser {
    pub fn new(atoms: Box<dyn Stringifier<AtomId>>, vecs: Box<dyn Stringifier<Vec<String>>>, special_cases: Box<dyn Stringifier<SpecialCase>>) -> Self 
        { Self {atoms, vecs, special_cases} }
}
impl ExpressionParse {
    pub fn new(atoms: Box<dyn Stringify<AtomId>>, vecs: Box<dyn Stringify<Vec<String>>>, special_cases: Box<dyn Stringify<SpecialCase>>) -> Self
        { Self {atoms, vecs, special_cases} }
}
impl ExpressionUnparse {
    pub fn new(atoms: Box<dyn Destringify<AtomId>>, vecs: Box<dyn Destringify<Vec<String>>>, special_cases: Box<dyn Destringify<SpecialCase>>) -> Self
        { Self {atoms, vecs, special_cases} }
}

// Implement the expression stringifier interface for Expression stringification structs
impl ExpressionParseInterface for ExpressionParser {
    fn atoms(&self) -> &Box<dyn Stringify<AtomId>> 
        { unsafe { &*(&self.atoms as *const Box<dyn Stringifier<AtomId>> as *const Box<dyn Stringify<AtomId>>) } }
    fn vecs(&self) -> &Box<dyn Stringify<Vec<String>>> 
        { unsafe { &*(&self.vecs as *const Box<dyn Stringifier<Vec<String>>> as *const Box<dyn Stringify<Vec<String>>>) } }
    fn special_cases(&self) -> &Box<dyn Stringify<SpecialCase>> 
        { unsafe { &*(&self.special_cases as *const Box<dyn Stringifier<SpecialCase>> as *const Box<dyn Stringify<SpecialCase>>) } }
}
impl ExpressionParseInterface for ExpressionParse {
    fn atoms(&self) -> &Box<dyn Stringify<AtomId>> { &self.atoms }
    fn vecs(&self) -> &Box<dyn Stringify<Vec<String>>> { &self.vecs }
    fn special_cases(&self) -> &Box<dyn Stringify<SpecialCase>> { &self.special_cases }
}
impl ExpressionUnparseInterface for ExpressionParser {
    fn atoms(&self) -> &Box<dyn Destringify<AtomId>> 
        { unsafe { &*(&self.atoms as *const Box<dyn Stringifier<AtomId>> as *const Box<dyn Destringify<AtomId>>) } }
    fn vecs(&self) -> &Box<dyn Destringify<Vec<String>>> 
        { unsafe { &*(&self.vecs as *const Box<dyn Stringifier<Vec<String>>> as *const Box<dyn Destringify<Vec<String>>>) } }
    fn special_cases(&self) -> &Box<dyn Destringify<SpecialCase>> 
        { unsafe { &*(&self.special_cases as *const Box<dyn Stringifier<SpecialCase>> as *const Box<dyn Destringify<SpecialCase>>) } }
}
impl ExpressionUnparseInterface for ExpressionUnparse {
    fn atoms(&self) -> &Box<dyn Destringify<AtomId>> { &self.atoms }
    fn vecs(&self) -> &Box<dyn Destringify<Vec<String>>> { &self.vecs }
    fn special_cases(&self) -> &Box<dyn Destringify<SpecialCase>> { &self.special_cases }
}

// Implement Textualize and Destringifiy for the stringification structs
impl Stringify<Expression> for ExpressionParser  { 
    fn stringify(&self, expr: &Expression) -> Result<String,()> { self.to_text(expr) }
}
impl Stringify<Expression> for ExpressionParse  { 
    fn stringify(&self, expr: &Expression) -> Result<String,()> { self.to_text(expr) }
}
impl Destringify<Expression> for ExpressionParser { 
    fn destringify(&self, string: &String) -> Result<Expression,()> { self.from_text(string) }
}
impl Destringify<Expression> for ExpressionUnparse { 
    fn destringify(&self, string: &String) -> Result<Expression,()> { self.from_text(string) }
}

// Implement Textualizer for the one stringifier type
impl Stringifier<Expression> for ExpressionParser {}
