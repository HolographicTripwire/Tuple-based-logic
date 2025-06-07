use std::sync::LazyLock;

use tbl_textualization::structures::TblLexer;

pub mod atom;
pub mod expression;
pub mod special_cases;

pub (self) static TBL_LEXER: LazyLock<Box<TblLexer>> = LazyLock::new(|| -> Box<TblLexer> { 
    Box::new(TblLexer::default())
});
