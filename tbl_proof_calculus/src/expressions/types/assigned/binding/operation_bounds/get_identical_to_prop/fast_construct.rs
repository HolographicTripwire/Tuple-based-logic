use crate::expressions::types::assigned::{
    TblExpression,
    at_path_enum::TblExpressionAtPathEnum,
    binding::bounds::{
        TblExpressionBoundAtomExactValue, TblExpressionBoundCompoundExactLength,
        TblExpressionIdentityBound,
    },
    compound::TblExpressionCompound,
    subexpressions::iterators::depth_first::counterclockwise::CounterclockwiseDepthFirstLocatedTblSubexpressionIterator,
};

/// [PropositionIdentityBounds] for [TblProposition] which is fast to construct
/// To see [PropositionIdentityBounds] for [TblProposition] which fast to perform lookups with, see [TblExpressionFastLookupIdentityBounds]
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct TblFastConstructGetBoundsForExprIdenticalToExpr(Box<[TblExpressionIdentityBound]>);
pub type TblFastConstructGetBoundsForPropIdenticalToProp =
    TblFastConstructGetBoundsForExprIdenticalToExpr;
// Construction
impl<'a, C: TblExpressionCompound> From<&'a TblExpression<C>>
    for TblFastConstructGetBoundsForPropIdenticalToProp
{
    fn from(expr: &'a TblExpression<C>) -> Self {
        let bounds = CounterclockwiseDepthFirstLocatedTblSubexpressionIterator::new(expr)
            .map(|v| match v.into() {
                TblExpressionAtPathEnum::Atom(atom) => {
                    TblExpressionBoundAtomExactValue::new(atom.path, *atom.obj).into()
                }
                TblExpressionAtPathEnum::Compound(compound) => {
                    TblExpressionBoundCompoundExactLength::new(compound.path, compound.obj.len())
                        .into()
                }
            })
            .collect();
        Self(bounds)
    }
}
mod retrieval {
    use std::collections::HashSet;

    use proof_calculus::utils::collections::binding::{
        binders::{Binder, GetBinder},
        bounds::{GetBounds, UniqueGetBounds},
    };

    use crate::expressions::types::assigned::binding::{
        bounds::{TblExpressionIdentityBound, TblPropositionIdentityBound},
        operation_bounds::get_identical_to_prop::fast_construct::TblFastConstructGetBoundsForExprIdenticalToExpr,
    };

    impl<B: GetBinder<TblExpressionIdentityBound>> GetBounds<B>
        for TblFastConstructGetBoundsForExprIdenticalToExpr
    {
        fn get_from<'binder>(&self, binder: &'binder B) -> HashSet<&'binder <B as Binder>::Value> {
            binder.get_intersection(self.0.iter())
        }
    }
    impl<B: GetBinder<TblPropositionIdentityBound>> UniqueGetBounds<B>
        for TblFastConstructGetBoundsForExprIdenticalToExpr
    {
    }
}
mod operational_retrieval {
    use proof_calculus::{
        propositions::types::assigned::binding::bounds::GetBoundsForPropIdenticalToProp,
        utils::collections::binding::binders::GetBinder,
    };

    use crate::{
        expressions::types::assigned::{
            binding::{
                bounds::TblPropositionIdentityBound,
                operation_bounds::get_identical_to_prop::fast_construct::TblFastConstructGetBoundsForExprIdenticalToExpr,
            },
            compound::TblExpressionCompound,
        },
        proof_calculus_derived::aliases::propositions::types::assigned::TblProposition,
    };

    impl<'prop, C: 'prop + TblExpressionCompound, B: GetBinder<TblPropositionIdentityBound>>
        GetBoundsForPropIdenticalToProp<'prop, TblProposition<C>, B>
        for TblFastConstructGetBoundsForExprIdenticalToExpr
    {
    }
}

// impl TblFastConstructGetBoundsForExprIdenticalToExpr {
//     pub fn bounds(&self) -> &Box<[TblExpressionIdentityBound]> {
//         &self.0
//     }
// }
