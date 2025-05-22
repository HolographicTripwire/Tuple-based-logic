use bimap::{BiHashMap};
use shared::atoms::AtomId;
use tuple_based_logic_textualizer::{atom::AtomTextualizer, Textualizer};

mod numerical;
mod plaintext;

pub (self) fn construct_symbols(vec: Vec<(usize,&str)>) -> AtomTextualizer {
    AtomTextualizer::new(BiHashMap::from_iter(
        vec.iter()
        .map(|(int,str)| -> (AtomId, String) {
            (AtomId::try_from(*int).expect("Atom id out of range when constructing symbols"), str.to_string())
        })
    ))
}

pub struct VecTextualizer();

impl Textualizer<Vec<String>> for VecTextualizer {
    fn to_text(&self, strings: &Vec<String>) -> Result<String,()> {
        Ok("(".to_string() + &strings.join(", ") + ")")
    }

    fn from_text(&self, s: &String) -> Result<Vec<String>,()> {
        todo!()
    }
}
