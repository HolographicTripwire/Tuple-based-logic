use parsertools::{lazy, pred, AstBounds, Parser, TokenBounds};

pub fn vec_concat_parser_transformer<'a,T:'a + TokenBounds,A: 'a + AstBounds, I: IntoIterator<Item = Parser<'a,T,Vec<A>>>>(vec: I) -> Parser<'a,T,Vec<A>> {
    vec.into_iter()
        .reduce(|acc,next| vec_concat_parser_transformer_inner(acc, next))
        .unwrap_or(pred(|_| Some(vec![])))
}

fn vec_concat_parser_transformer_inner<'a,T:'a + TokenBounds,A: 'a + AstBounds>(left: Parser<'a,T,Vec<A>>, right: Parser<'a,T,Vec<A>>) -> Parser<'a,T,Vec<A>> {
    left.then(right).map(|(l,r)| [l,r].concat())
}

pub fn iter_disjunction_parser_transformer<'a,T:'a + TokenBounds,A: 'a + AstBounds, I: IntoIterator<Item = Parser<'a,T,A>>>(vec: I) -> Parser<'a,T,A> {
    vec.into_iter()
        .reduce(|acc,next| acc.or(next))
        .unwrap_or(pred(|_| None))
}

pub fn n_repeat_parser_transformer<'a,T: 'a + TokenBounds, A: 'a + AstBounds>(parser: Parser<'a,T,A>, n: usize) -> Parser<'a,T,A> {
    if n == 0 { panic!("Attempted to repeat parser 0 times") }
    else if n == 1 { parser }
    else { n_repeat_parser_transformer(parser.clone(), n-1).then(parser).map(|(a,_)| a) }
}

pub fn multiple_of_n_repeat_parser_transformer<'a,T: 'a + TokenBounds, A: 'a + AstBounds>(parser: Parser<'a,T,A>, n: usize) -> Parser<'a,T,A> {
    let n_repeat = n_repeat_parser_transformer(parser.clone(),n);
    n_repeat.clone().then(lazy(move || n_repeat_parser_transformer(parser.clone(), n))).map(|(a,_)| a)
        .or(n_repeat)
}
