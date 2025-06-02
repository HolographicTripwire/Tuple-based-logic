use std::sync::LazyLock;

use tbl_stringification::structures::TblStringifierControls;

pub mod atom;
pub mod expression;
pub mod special_cases;

pub (self) static STRINGIFIER_CONTROLS: LazyLock<Box<TblStringifierControls>> = LazyLock::new(|| -> Box<TblStringifierControls> { 
    Box::new(TblStringifierControls::default())
});
