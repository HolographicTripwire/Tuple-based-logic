use enum_iterator::{all, Sequence};

pub trait Controls<Control: Sequence + Clone>: Clone {
    fn controls(&self) -> Vec<Control> { all::<Control>().collect::<Vec<Control>>() }
    fn strings(&self) -> Vec<&String> { 
        self.controls()
            .iter()
            .map(|c| -> &String { self.string_from_control(c) })
            .collect() 
    }
    fn controls_and_strings(&self) -> Vec<(Control,&String)> { 
        self.controls()
            .iter()
            .map(|c| -> (Control,&String) { (c.clone(), self.string_from_control(c)) })
            .collect()
    }



    fn string_from_control(&self, control: &Control) -> &String;
    fn control_from_string(&self, string: &String) -> Result<Control,()> {
        let binding = self.controls_and_strings();
        let interpretations = binding
            .iter()
            .filter_map(|(c, s)| -> Option<&Control> { if *s == string { Some(c) } else { None } })
            .collect::<Vec<&Control>>();
        if interpretations.len() > 1 { Err(()) }
        else if let Some(interpretation) = interpretations.get(0) { Ok((*interpretation).clone()) }
        else { Err(()) }
    }



    fn pop_from_string(&self, string: &mut String) -> Result<Option<(Control,&String)>,()> {
        Ok(match self.string_starts_with(string)? {
            Some((c,s)) => {
                string.drain(0..s.len());
                Some((c,s))
            }, None => None,
        })
    }
    fn string_starts_with(&self, string: &String) -> Result<Option<(Control,&String)>,()> {
        let all_controls = all::<Control>().collect::<Vec<Control>>();
        let starts_with = all_controls.iter()
            .map(|c| -> (&Control,&String) { (c,self.string_from_control(c)) })
            .map(|(c, s)| -> (&Control, &String, bool) { (c,s,string.starts_with(s)) } )
            .filter_map(|(c, s, b)| -> Option<(Control,&String)> { if b { Some((c.clone(),s)) } else { None } })
            .collect::<Vec<(Control,&String)>>();
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

#[derive(Clone)]
pub struct StringifierControls{
    escape_string: String,
    vec_controls: VecControls,
    pattern_controls: ExprPatternControls,
}
#[derive(Clone)]
pub struct VecControls {
    opener: String,
    closer: String,
    delimiter: String,
}
#[derive(Clone)]
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
    fn string_from_control(&self, control: &StringifierControl) -> &String { match control {
        StringifierControl::Escape => &self.escape_string,
        StringifierControl::Vec(vec_control) => self.vec_controls.string_from_control(vec_control),
        StringifierControl::Pattern(pattern_control) => self.pattern_controls.string_from_control(pattern_control),
    }}
}
impl Controls<VecControl> for VecControls {
    fn string_from_control(&self, control: &VecControl) -> &String { match control {
        VecControl::Opener => &self.opener,
        VecControl::Closer => &self.closer,
        VecControl::Delimiter => &self.delimiter,
    }}
}
impl Controls<ExprPatternControl> for ExprPatternControls {
    fn string_from_control(&self, control: &ExprPatternControl) -> &String { match control {
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
