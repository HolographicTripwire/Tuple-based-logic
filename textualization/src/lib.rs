pub mod helpers;
pub mod structures;

// Types which allow attempting conversions between structs of type Object and Strings
pub trait Textualizer<Object>: Textualize<Object> + Detextualize<Object> {}

/// Types which allow attempting a conversion of a struct of type Object to a String
pub trait Textualize<Object>: Sync + Send {
    fn textualize(&self, object: &Object) -> Result<String,()>;
}
/// Types which allow attempting a conversion of a String to a struct of type Object
pub trait Detextualize<Object>: Send + Sync {
    fn detextualize(&self, string: &String) -> Result<Object,()>;
}
