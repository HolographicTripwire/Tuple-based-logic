use bimap::{BiHashMap};
use tbl_structures::atoms::AtomId;
use tbl_stringification::{atom::AtomTextualizer, Textualizer};

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
        // Strip the parentheses and return the inner string
        let inner =  { 
            if s.starts_with('(') && s.ends_with(')') {
                s[1..s.len()-1].to_string()
            } else { return Err(()) }
        };
        // Break into substrings
        let mut substrings = Vec::new(); // The substrings that we will return from this function
        let mut current_substring = "".to_string(); // The current substring, which will be added to substrings when we reach a delimiter
        let mut parentheses_level = 0; // How deep into parentheses we are (e.g. if we had just read "((a,b),(c" we would be at level 2)
        let mut escaping = false; // If an escape character has just preceded the one we're at currently
        for char in inner.chars() {
            // Commas (,) are used as delimiters
            if char == ',' && parentheses_level == 0 && !escaping {
                substrings.push(current_substring);
                current_substring = "".to_string();
            // Any parenthesised expression should be contained
            } else if char == '(' && !escaping {
                current_substring.push(char);
                parentheses_level += 1;
            } else if char == ')' && !escaping {
                current_substring.push(char);
                parentheses_level -= 1;
                // Parentheses level cannot be allowed to go below 0
                if parentheses_level < 0 { return Err(()) }
            // Use backslash (\) as an escape character
            } else if char == '\\' && !escaping {
                escaping = true;
            // All non-control characters can 
            } else {
                current_substring.push(char);
                escaping = false;
            }
        }
        // Push final substring, which will not be delimited like the others
        substrings.push(current_substring);
        // Then return all the substrings (each of which will be converted to a Term later)
        Ok(substrings)
    }
}
