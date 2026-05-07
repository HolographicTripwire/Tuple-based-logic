pub mod types;
pub mod paths;
pub mod assignments;

#[derive(PartialEq,Eq,Clone,Copy,Debug,Hash)]
pub enum TblExpressionLength {
    Unit,
    Compound(usize)
}