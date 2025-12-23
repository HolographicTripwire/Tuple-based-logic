use tbl_structures::expressions::Expression;


/// Convert atomicity to string
pub(crate) fn stringify_atomicity(is_atomic: bool) -> &'static str {
    if is_atomic { "atomic" } else { "not-atomic" }
}
/// Convert length of an expression to string
pub(crate) fn stringify_length(expr: &Expression) -> String {
    match expr.len() {
        Some(len) => len.to_string(),
        None => stringify_atomicity(true).to_string()
    }
}
