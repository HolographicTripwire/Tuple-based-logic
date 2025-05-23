pub mod atom;
pub mod terms;

// Types which allow attempting conversions between structs of type Object and Strings
pub trait Stringifier<Object>: Stringify<Object> + Destringify<Object> {}

/// Types which allow attempting a conversion of a struct of type Object to a String
pub trait Stringify<Object>: Sync + Send {
    fn stringify(&self, object: &Object) -> Result<String,()>;
}
/// Types which allow attempting a conversion of a String to a struct of type Object
pub trait Destringify<Object>: Send + Sync {
    fn destringify(&self, string: &String) -> Result<Object,()>;
}
