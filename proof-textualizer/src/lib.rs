pub mod atom;
pub mod term;

pub trait Textualizer<E> {
    fn to_text(&self, e: &E) -> Result<String,()>;
    fn from_text(&self, s: &String) -> Result<E,()>;
}
