use std::collections::{HashMap, HashSet};

pub struct VariableAssignments {
    vars: HashSet<String>,
    var_to_val: HashMap<String,String>,
    vars_to_vals: HashMap<(String,String),Vec<String>>
}
impl VariableAssignments {
    pub fn new() -> Self { Self {
        vars: HashSet::new(),
        var_to_val: HashMap::new(), 
        vars_to_vals: HashMap::new()
    }}

    pub fn add_var_to_val(&mut self, k: String, v: String) -> Result<(),()> {
        if self.vars.contains(&k) { return Err(()); }
        self.vars.insert(k.clone());
        self.var_to_val.insert(k, v);
        return Ok(());
    }
    pub fn get_va1_from_var(&self, s: &String) -> Option<&String>
        { self.var_to_val.get(s) }

    pub fn add_vars_to_vals(&mut self, k1: String, k2: String, v: Vec<String>) -> Result<(),()> { 
        if self.vars.contains(&k1) || self.vars.contains(&k2) { return Err(()); }
        self.vars_to_vals.insert((k1,k2), v);
        return Ok(());
    }
    pub fn get_vals_from_vars(&self, s1: &String, s2: &String) -> Option<&Vec<String>> 
        { self.vars_to_vals.get(&(s1.clone(),s2.clone())) }
}
