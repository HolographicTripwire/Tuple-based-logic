use std::sync::LazyLock;

use tbl_stringification::structures::TblStringifierLexer;

pub mod atom;
pub mod expression;
pub mod special_cases;

pub (self) static STRINGIFIER_LEXER: LazyLock<Box<TblStringifierLexer>> = LazyLock::new(|| -> Box<TblStringifierLexer> { 
    Box::new(TblStringifierLexer::default())
});
