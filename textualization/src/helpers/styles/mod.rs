use std::fmt::Display;

pub trait Style<Sb: Stylable>: Sized + Clone {
    fn stringify(&self, stylable: &Sb) -> String;
}

pub trait Stylable: Sized + Clone {
    fn styled<S: Style<Self>>(&self, style: &S) -> Styled<Self,S>
        { Styled::new(self.clone(), style.clone()) }
}
impl <Sb: Sized + Clone> Stylable for Sb {}

pub struct Styled<Sb: Stylable, S: Style<Sb>> {
    stylable: Sb,
    style: S
}
impl <Sb: Stylable, S: Style<Sb>> Styled<Sb,S> {
    fn new(stylable: Sb, style: S) -> Self { Self { stylable, style } }
}
impl <Sb: Stylable, S: Style<Sb>> Display for Styled<Sb,S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self.style.stringify(&self.stylable))
    }
}
