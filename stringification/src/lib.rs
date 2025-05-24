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


trait MyTrait {
    fn do_something(&self);
    fn clone_box(&self) -> Box<dyn MyTrait>; // Method to clone the trait object
}

#[derive(Clone)]
struct MyStruct;

impl MyTrait for MyStruct {
    fn do_something(&self) {
        println!("Doing something!");
    }

    fn clone_box(&self) -> Box<dyn MyTrait> {
        Box::new(self.clone()) // Clone the concrete type
    }
}

struct Container {
    my_obj: Box<dyn MyTrait>,
}

impl Container {
    // Getter that returns a clone of my_obj
    fn get_my_obj(&self) -> Box<dyn MyTrait> {
        self.my_obj.clone_box() // Use the clone_box method
    }
}
