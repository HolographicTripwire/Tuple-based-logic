pub mod types;
pub mod paths;
pub mod assignments;

pub enum TblExpressionLength {
    Unit,
    Compound(usize)
}