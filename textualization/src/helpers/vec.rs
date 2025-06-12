use parsertools::{pred, AstBounds, Parser, TokenBounds};

pub fn vec_concat_parser<'a,T:'a + TokenBounds,A: 'a + AstBounds>(vec: Vec<Parser<'a,T,Vec<A>>>) -> Parser<'a,T,Vec<A>> {
    vec.into_iter()
        .reduce(|acc,next| vec_concat_parser_inner(acc, next))
        .unwrap_or(pred(|_| Some(vec![])))
}

fn vec_concat_parser_inner<'a,T:'a + TokenBounds,A: 'a + AstBounds>(left: Parser<'a,T,Vec<A>>, right: Parser<'a,T,Vec<A>>) -> Parser<'a,T,Vec<A>> {
    left.then(right).map(|(l,r)| [l,r].concat())
}