use enum_iterator::{all, Sequence};

pub trait Controls<Control: Sequence + Clone> {
    fn get_control_string(&self, control: &Control) -> &String;
    fn pop_from_string(&self, string: &mut String) -> Result<Option<Control>,()> {
        Ok(match self.string_starts_with(string)? {
            Some(control) => {
                let string_to_remove = self.get_control_string(&control);
                string.drain(0..string_to_remove.len());
                Some(control)
            }, None => None,
        })
    }
    fn string_starts_with(&self, string: &String) -> Result<Option<Control>,()> {
        let all_controls = all::<Control>().collect::<Vec<Control>>();
        let starts_with = all_controls.iter()
            .map(|c| -> (&Control,String) { (c,self.get_control_string(c).to_string()) })
            .map(|(c, s)| -> (&Control, bool) { (c,string.starts_with(&s)) } )
            .filter_map(|(c, b)| -> Option<Control> { if b { Some(c.clone()) } else { None } })
            .collect::<Vec<Control>>();
        // If there are multiple plausible control strings that this string could start with
        if starts_with.len() > 1 { return Err(()) }
        Ok(starts_with.get(0).cloned())
    }
}

#[derive(Sequence, Clone, Copy)]
pub enum StringifierControl { Escape, Vec(VecControl), Pattern (ExprPatternControl) }
#[derive(Sequence, Clone, Copy)]
pub enum VecControl { Opener, Closer, Delimiter }
#[derive(Sequence, Clone, Copy)]
pub enum ExprPatternControl { VariableIndicator, VariableEnumerator }

pub struct StringifierControls{
    escape_string: String,
    vec_controls: VecControls,
    pattern_controls: ExprPatternControls,
}
pub struct VecControls {
    opener: String,
    closer: String,
    delimiter: String,
}
pub struct ExprPatternControls {
    variable_indicator: String,
    variable_enumerator: String,
}

impl StringifierControls {
    pub fn new(escape_string: String, vec_controls: VecControls, pattern_controls: ExprPatternControls) -> Self
        { Self { escape_string, vec_controls, pattern_controls } }
}
impl VecControls {
    pub fn new(opener: String, closer: String, delimiter: String) -> Self
        { Self { opener, closer, delimiter } }
}
impl ExprPatternControls {
    pub fn new(variable_indicator: String, variable_enumerator: String) -> Self 
        { Self { variable_indicator, variable_enumerator } }
}

impl Controls<StringifierControl> for StringifierControls {
    fn get_control_string(&self, control: &StringifierControl) -> &String { match control {
        StringifierControl::Escape => &self.escape_string,
        StringifierControl::Vec(vec_control) => self.vec_controls.get_control_string(vec_control),
        StringifierControl::Pattern(pattern_control) => self.pattern_controls.get_control_string(pattern_control),
    }}
}
impl Controls<VecControl> for VecControls {
    fn get_control_string(&self, control: &VecControl) -> &String { match control {
        VecControl::Opener => &self.opener,
        VecControl::Closer => &self.closer,
        VecControl::Delimiter => &self.delimiter,
    }}
}
impl Controls<ExprPatternControl> for ExprPatternControls {
    fn get_control_string(&self, control: &ExprPatternControl) -> &String { match control {
        ExprPatternControl::VariableIndicator => &self.variable_indicator,
        ExprPatternControl::VariableEnumerator => &self.variable_enumerator,
    }}
}

impl Default for VecControls {
    fn default() -> Self { VecControls::new(
        "(".to_string(),
        ")".to_string(),
        ",".to_string(),
    )}
}
impl Default for ExprPatternControls {
    fn default() -> Self { Self {
        variable_indicator: "#".to_string(),
        variable_enumerator: "..".to_string(),
    }}
}
impl Default for StringifierControls {
    fn default() -> Self { Self {     
        escape_string: "\\".to_string(),
        vec_controls: VecControls::default(),
        pattern_controls: ExprPatternControls::default()
    }}
}
