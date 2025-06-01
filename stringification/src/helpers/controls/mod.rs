use either::Either;
use enum_iterator::{all, Sequence};

use crate::{Destringify, Stringifier, Stringify};

/// This trait allows an enum to be used as a Control for an implementer of the Controls trait
pub trait Control: Sequence + Clone {}

#[derive(Clone)]
pub struct ControlString<C: Control>(pub C,pub String);

pub struct ControlSequence<C: Control>(pub Vec<Either<C,String>>);

pub trait Controls<C>: Clone + Send + Sync {
    fn escape_string(&self) -> &String;
    fn string_from_control(&self, control: &C) -> &String;
}

impl <C: Control, Cs: Controls<C>> Stringifier<ControlSequence<C>> for Cs {}
impl <C: Control, Cs: Controls<C>> Stringify<ControlSequence<C>> for Cs {
    fn stringify(&self, sequence: &ControlSequence<C>) -> Result<String,()> {
        let mut string = "".to_string();
        for s in &sequence.0 { match s {
            Either::Left(c) => string += self.string_from_control(&c),
            Either::Right(s) => string += s.as_str(),
        }}
        Ok(string)
    }
}
impl <C: Control, Cs: Controls<C>> Destringify<ControlSequence<C>> for Cs {
    fn destringify(&self, string: &String) -> Result<ControlSequence<C>,()> {
        let escape_string = self.escape_string();
        let mut sequence: Vec<Either<C, String>> = Vec::new();
        let mut escaping = false;
        let mut current_string = "".to_string();
        for char in string.chars() {
            current_string.push(char);
            if let Ok(_) = pop_escape_from_string_end(self,&mut current_string) { 
                escaping = true;
                pop_n_from_end(&mut current_string,escape_string.len());
            } else if let Some(ControlString(c,s)) = pop_control_from_string_end(self, &mut current_string)? {
                if current_string.len() > 0 { sequence.push(Either::Right(current_string)) }
                sequence.push(Either::Left(c));
                current_string = "".to_owned();
            }
        }
        if current_string.len() > 0 { sequence.push(Either::Right(current_string)) }
        Ok(ControlSequence(sequence))
    }
}

fn pop_n_from_end(string: &mut String, n: usize) -> std::string::Drain<'_> {
    string.drain(string.len()-n..string.len())
}

fn pop_escape_from_string_end<C: Control, Cs: Controls<C>>(controls: &Cs, string: &mut String) -> Result<(),()> {
    let escape_string = controls.escape_string();
    if string.ends_with(escape_string) {
        pop_n_from_end(string, escape_string.len());
        Ok(())
    } else { Err(()) }
}

fn pop_control_from_string_end<C: Control, Cs: Controls<C>>(controls: &Cs, string: &mut String) -> Result<Option<ControlString<C>>,()> {
    Ok(match string_ends_with(controls,string)? {
        Some(ControlString(c,s)) => {
            pop_n_from_end(string,s.len());
            Some(ControlString(c,s))
        }, None => None,
    })
}

fn string_ends_with<C: Control, Cs: Controls<C>>(controls: &Cs, string: &String) -> Result<Option<ControlString<C>>,()> {
    let ends_with = get_controls_and_strings(controls).iter()
        .filter_map(|ControlString(c, s)| -> Option<ControlString<C>> 
            { if string.ends_with(s) { Some(ControlString(c.clone(),s.clone())) } else { None } }
        ).collect::<Vec<ControlString<C>>>();
    // If there are multiple plausible control strings that this string could start with
    if ends_with.len() > 1 { return Err(()) }
    Ok(ends_with.get(0).cloned())
}

fn get_controls<C: Control, Cs: Controls<C>>(_: &Cs) -> Vec<C> { all::<C>().collect::<Vec<C>>() }
fn get_controls_and_strings<C: Control, Cs: Controls<C>>(controls: &Cs) -> Vec<ControlString<C>> { 
    get_controls(controls)
        .iter()
        .map(|c| -> ControlString<C> { ControlString(c.clone(), controls.string_from_control(c).clone()) })
        .collect()
}
