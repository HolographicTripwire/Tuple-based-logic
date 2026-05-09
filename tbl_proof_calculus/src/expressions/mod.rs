pub mod assignments;
pub mod paths;
pub mod types;

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum TblExpressionLength {
    Unit,
    Compound(usize),
}
