use dyn_clone::DynClone;

pub mod helpers;
pub mod structures;

// Types which allow attempting conversions between structs of type Object and Strings
pub trait Stringifier<Object>: Stringify<Object> + Destringify<Object> {}

/// Types which allow attempting a conversion of a struct of type Object to a String
pub trait Stringify<Object>: DynClone + Sync + Send {
    fn stringify(&self, object: &Object) -> Result<String,()>;
}
/// Types which allow attempting a conversion of a String to a struct of type Object
pub trait Destringify<Object>: DynClone + Send + Sync {
    fn destringify(&self, string: &String) -> Result<Object,()>;
}

dyn_clone::clone_trait_object!(<Object> Stringifier<Object>);
dyn_clone::clone_trait_object!(<Object> Stringify<Object>);
dyn_clone::clone_trait_object!(<Object> Destringify<Object>);
