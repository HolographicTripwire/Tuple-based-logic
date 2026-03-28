use trait_aliases::trait_aliases;

use crate::{stringification::Style, structures::Proposition};

trait_aliases!{
    pub trait PropositionStyle<P: Proposition> = Style<P>;
}
