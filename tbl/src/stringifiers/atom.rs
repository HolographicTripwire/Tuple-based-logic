use bimap::BiHashMap;
use tbl_stringification::atom::AtomStringifier;
use tbl_structures::atoms::AtomId;

pub fn construct_symbols(vec: Vec<(usize,&str)>) -> AtomStringifier {
    AtomStringifier::new(BiHashMap::from_iter(
        vec.iter()
        .map(|(int,str)| -> (AtomId, String) {
            (AtomId::try_from(*int).expect("Atom id out of range when constructing symbols"), str.to_string())
        })
    ))
}
