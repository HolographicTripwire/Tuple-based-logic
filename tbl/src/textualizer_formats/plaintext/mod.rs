use std::sync::{Arc, LazyLock};

use tbl_textualization::structures::{atoms::AtomStyle, expressions::{functional::SpecialCasesBuilder, raw::RawExpressionStyle, ExpressionStyle, SpecialCases}};

const RAW_EXPRESSION_STYLE: LazyLock<RawExpressionStyle> = LazyLock::new(|| {
    let atom_style = AtomStyle::from_strs("#");
    RawExpressionStyle::from_strs(atom_style, "(", ")", ", ")
});

const SPECIAL_CASES: LazyLock<SpecialCases> = LazyLock::new(|| SpecialCasesBuilder::new(RAW_EXPRESSION_STYLE.clone())
    // Built in atoms
    .add_variadic_atomic_infix_function(000,"∧", " ∧ ")  // Conjunction
    .add_atomic_prefix_function(001, 1..1,"∀","∀")  // Universal quantiifer
    .add_atomic_infix_function(002, 2..2, "→"," → ")  // Implication
    .add_atomic_prefix_function(003,1..1, "¬","¬")  // Negation
    .add_variadic_atomic_infix_function(004,"=", " = ")  // Identity
    .add_variadic_atomic_outfix_function(005,"⟨⟩","⟨","⟩") // Verbatim
    .add_variadic_atomic_infix_function(006,"⌢","⌢")  // Concatenation
    .add_atomic_prefix_function(007,1..1, "⚛","⚛")  // Atomicity
    // Non-built-in atoms
    .build()
);

pub const EXPRESSION_STYLE: LazyLock<ExpressionStyle> = LazyLock::new(|| -> ExpressionStyle {
    ExpressionStyle::new(RAW_EXPRESSION_STYLE.clone(), Arc::new(SPECIAL_CASES.clone()))
});
