use tbl_structures::expressions::Expression;


/// Convert atomicity to string
pub(crate) fn stringify_atomicity(is_atomic: bool) -> &'static str {
    if is_atomic { "atomic" } else { "not-atomic" }
}
/// Convert length of an expression to string
pub(crate) fn stringify_length(expr: &Expression) -> String {
    match expr.as_slice() {
        Ok(tuple) => tuple.len().to_string(),
        Err(()) => stringify_atomicity(true).to_string()
    }
}
