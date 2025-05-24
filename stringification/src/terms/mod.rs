mod functions;
mod special_cases;

pub use functions::FunctionStringifier;
pub use special_cases::{SpecialCase, NoSpecialCasesStringifier};

use tbl_structures::{atoms::AtomId, propositions::Term};

use crate::{Destringify, Stringifier, Stringify};

trait TermStringifyInterface<A: Stringify<AtomId>, V: Stringify<Vec<String>>, S: Stringify<SpecialCase>> {
    // Simple getters
    fn atoms(&self) -> &A;
    fn vecs(&self) -> &V;
    fn special_cases(&self) -> &S;
    /// This function will be used by [TermStringifier] and [TermStringify] as a proxy for [Stringify::stringify]. 
    fn to_text(&self, term: &Term) -> Result<String,()> {
        match term {
            Term::Atomic(atom_id) => self.atoms().stringify(atom_id),
            Term::Tuple(terms) => {
                // Convert each term in the tuple to a string
                let strings: Result<Vec<String>,()> = terms.iter().map(|term| -> Result<String,()> { self.to_text(term) }).collect();
                // Pair terms with strings
                let term_and_string = SpecialCase(terms.clone(),strings?);
                
                // If there are any optional rules, apply them
                if let Ok(string) = self.special_cases().stringify(&term_and_string) { Ok(string) }
                // Otherwise just treat this vec as we would any other vec
                else { self.vecs().stringify(&term_and_string.1) }
            },
        }
    }
}
trait TermDestringifyInterface<A: Destringify<AtomId>, V: Destringify<Vec<String>>, S: Destringify<SpecialCase>> {
    fn atoms(&self) -> &A;
    fn vecs(&self) -> &V;
    fn special_cases(&self) -> &S;
    /// This function will be used by [TermStringifier] and [TermDestringify] as a proxy for [Destringify::destringify]. 
    fn from_text(&self, string: &String) -> Result<Term,()> {
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
        else if let Ok(atom) = atom_result { Ok(Term::Atomic(atom)) }
        else if let Ok(strings) = tuple_result {
            let terms: Result<Vec<Term>,()> = strings.iter()
                .map(|s| -> Result<Term,()> { self.from_text(s) })
                .collect();
            Ok(Term::Tuple(terms?)) 
        } else if let Ok(SpecialCase(_, strings)) = optional_rules_result {
            let terms: Result<Vec<Term>,()> = strings.iter()
                .map(|string| -> Result<Term,()> { self.from_text(string) })
                .collect();
            Ok(Term::Tuple(terms?))
        }
        // Throw an error if this string has no interpretations
        else { Err(()) }
    }
}

/// A struct that can both Stringify and Destringify terms
pub struct TermStringifier<A: Stringifier<AtomId>, V: Stringifier<Vec<String>>, S: Stringifier<SpecialCase>> {
    atoms: A,
    vecs: V,
    special_cases: S,
}
/// A struct that can stringify, but not destringify, terms
pub struct TermStringify<A: Stringify<AtomId>, V: Stringify<Vec<String>>, S: Stringify<SpecialCase>> {
    atoms: A,
    vecs: V,
    special_cases: S,
}
/// A struct that can destringify, but not stringify, terms
pub struct TermDestringify<A: Destringify<AtomId>, V: Destringify<Vec<String>>, S: Destringify<SpecialCase>> {
    atoms: A,
    vecs: V,
    special_cases: S,
}

// Implement new for Term stringification structs
impl <A: Stringifier<AtomId>, V: Stringifier<Vec<String>>, S: Stringifier<SpecialCase>> TermStringifier<A,V,S> {
    pub fn new(atoms: A, vecs: V, special_cases: S) -> Self { Self {atoms, vecs, special_cases} }
}
impl <A: Stringify<AtomId>, V: Stringify<Vec<String>>, S: Stringify<SpecialCase>> TermStringify<A,V,S> {
    pub fn new(atoms: A, vecs: V, special_cases: S) -> Self { Self {atoms, vecs, special_cases} }
}
impl <A: Destringify<AtomId>, V: Destringify<Vec<String>>, S: Destringify<SpecialCase>> TermDestringify<A,V,S> {
    pub fn new(atoms: A, vecs: V, special_cases: S) -> Self { Self {atoms, vecs, special_cases} }
}

// Implement the term stringifier interface for Term stringification structs
impl <A: Stringifier<AtomId>, V: Stringifier<Vec<String>>, S: Stringifier<SpecialCase>> TermStringifyInterface<A,V,S> for TermStringifier<A,V,S> {
    fn atoms(&self) -> &A { &self.atoms }
    fn vecs(&self) -> &V { &self.vecs }
    fn special_cases(&self) -> &S { &self.special_cases }
}
impl <A: Stringify<AtomId>, V: Stringify<Vec<String>>, S: Stringify<SpecialCase>> TermStringifyInterface<A,V,S> for TermStringify<A,V,S> {
    fn atoms(&self) -> &A { &self.atoms }
    fn vecs(&self) -> &V { &self.vecs }
    fn special_cases(&self) -> &S { &self.special_cases }
}
impl <A: Stringifier<AtomId>, V: Stringifier<Vec<String>>, S: Stringifier<SpecialCase>> TermDestringifyInterface<A,V,S> for TermStringifier<A,V,S> {
    fn atoms(&self) -> &A { &self.atoms }
    fn vecs(&self) -> &V { &self.vecs }
    fn special_cases(&self) -> &S { &self.special_cases }
}
impl <A: Destringify<AtomId>, V: Destringify<Vec<String>>, S: Destringify<SpecialCase>> TermDestringifyInterface<A,V,S> for TermDestringify<A,V,S> {
    fn atoms(&self) -> &A { &self.atoms }
    fn vecs(&self) -> &V { &self.vecs }
    fn special_cases(&self) -> &S { &self.special_cases }
}

// Implement Stringify and Destringifiy for the stringification structs
impl <A: Stringifier<AtomId>, V: Stringifier<Vec<String>>, S: Stringifier<SpecialCase>> Stringify<Term> for TermStringifier<A,V,S>  { 
    fn stringify(&self, term: &Term) -> Result<String,()> { self.to_text(term) }
}
impl <A: Stringifier<AtomId>, V: Stringifier<Vec<String>>, S: Stringifier<SpecialCase>> Stringify<Term> for TermStringify<A,V,S>  { 
    fn stringify(&self, term: &Term) -> Result<String,()> { self.to_text(term) }
}
impl <A: Stringifier<AtomId>, V: Stringifier<Vec<String>>, S: Stringifier<SpecialCase>> Destringify<Term> for TermStringifier<A,V,S> { 
    fn destringify(&self, string: &String) -> Result<Term,()> { self.from_text(string) }
}
impl <A: Stringifier<AtomId>, V: Stringifier<Vec<String>>, S: Stringifier<SpecialCase>> Destringify<Term> for TermDestringify<A,V,S> { 
    fn destringify(&self, string: &String) -> Result<Term,()> { self.from_text(string) }
}

// Implement Stringifier for the one stringifier type
impl <A: Stringifier<AtomId>, V: Stringifier<Vec<String>>, S: Stringifier<SpecialCase>> Stringifier<Term> for TermStringifier<A,V,S> {}
