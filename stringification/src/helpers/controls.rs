pub struct VecControls {
    pub opener: String,
    pub closer: String,
    pub delimiter: String,
}
impl Default for VecControls {
    fn default() -> Self { Self {
        opener: "(".to_string(),
        closer: ")".to_string(),
        delimiter: ",".to_string(),
    }}
}

pub struct ExprPatternControls {
    pub variable_indicator: String,
    pub variable_enumerator: String,
}
impl Default for ExprPatternControls {
    fn default() -> Self { Self {
        variable_indicator: "#".to_string(),
        variable_enumerator: "..".to_string(),
    }}
}

pub struct StringifierControls{
    pub escape_string: String,
    pub vec_controls: VecControls,
    pub pattern_controls: ExprPatternControls,
}
impl Default for StringifierControls {
    fn default() -> Self { Self {     
        escape_string: "\\".to_string(),
        vec_controls: VecControls::default(),
        pattern_controls: ExprPatternControls::default()
    }}
}
