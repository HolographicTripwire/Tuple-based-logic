use std::collections::{HashMap, HashSet};

use crate::structures::expressions::subexpressions::{TblSubexpressionInExpressionPath, immediate::ImmediateTblSubexpressionInExpressionPath};

mod fast_construct;
mod fast_lookup;

#[derive(Default)]
struct ImportantPathsConstructor(HashMap<TblSubexpressionInExpressionPath,ImmediateTblSubexpressionInExpressionPath>);
impl ImportantPathsConstructor {
    fn try_insert(&mut self, mut path: TblSubexpressionInExpressionPath) -> bool {
        if let Some(last) = path.0.pop() {
            let needs_insert = self.0.get(&path)
                .map(|inner| *inner < last)
                .unwrap_or(true);
            if needs_insert {
                self.0.insert(path.clone(), last);
                self.try_insert(path);
            }
            needs_insert
        } else { false }
    }

    fn contains(&self, path: &TblSubexpressionInExpressionPath) -> bool {
        if let Some((last, front)) = path.0.split_last() {
            self.0.get(&TblSubexpressionInExpressionPath(front.iter().copied().collect()))
                .map(|p| p == last)
                .unwrap_or(false)
        } else { false }
    }

    fn construct(self) -> HashSet<TblSubexpressionInExpressionPath> {
        self.0.into_iter().map(|(mut path,last)| { path.0.push(last); path }).collect()
    }
}