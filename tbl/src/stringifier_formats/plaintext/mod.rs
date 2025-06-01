use tbl_stringification::helpers::controls::StringifierControls;

pub mod atom;
pub mod expression;
pub mod special_cases;

pub (self) static STRINGIFIER_CONTROLS: StringifierControls = StringifierControls::default();
