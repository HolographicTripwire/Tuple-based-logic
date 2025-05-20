use shared::{atoms::AtomId, term::Term};

struct TermTextualizer<'a> {
    symbols: &'a dyn Textualizer<AtomId>,
    functions: dyn Textualizer<AtomId>,
}

impl Textualizer<Term> for TermTextualizer<'_> {
    fn to_text(&self, e: Term) -> String {
        todo!()
    }

    fn from_text(&self, s: String) -> Term {
        todo!()
    }
}

trait Textualizer<E> {
    fn to_text(&self, e: E) -> String;
    fn from_text(&self, s: String) -> E;
}