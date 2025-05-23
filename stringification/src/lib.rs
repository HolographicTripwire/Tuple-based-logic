pub mod atom;
pub mod terms;

/// Types which allow attempting a conversion between Strings and Objects of the template type
pub trait Textualizer<Object>: Sync + Send {
    fn to_text(&self, object: &Object) -> Result<String,()>;
    fn from_text(&self, string: &String) -> Result<Object,()>;
}
