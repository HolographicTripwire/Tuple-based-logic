use std::rc::Rc;

use crate::expressions::{
    assignments::implementations::btree::constructors::BTreeTblExpressionAssignmentConstructor,
    types::assigned::binding::bounds::TblExpressionInsertionBound,
};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TblFastConstructGetBoundsForExprsSubsumedByUexpr {
    get_bounds: Box<[TblExpressionInsertionBound]>,
    map_to_elem_constructor: Rc<BTreeTblExpressionAssignmentConstructor>,
}
pub type TblFastConstructGetBoundsForPropsSubsumedByUprop =
    TblFastConstructGetBoundsForExprsSubsumedByUexpr;

mod construction {
    use std::{collections::BTreeMap, rc::Rc};

    use crate::expressions::{
        paths::TblSubexpressionInExpressionPath,
        types::{
            assigned::binding::{
                bounds::{
                    TblExpressionBoundAtomExactValue, TblExpressionBoundCompoundExactLength,
                    TblExpressionBoundValueDuplicated,
                },
                operation_bounds::get_subsumed_by_uprop::fast_construct::TblFastConstructGetBoundsForPropsSubsumedByUprop,
            },
            unassigned::{
                UnassignedTblExpression, at_path_enum::UnassignedTblExpressionAtPathEnum,
                compound::UnassignedTblExpressionCompound,
                subexpressions::iterators::depth_first::counterclockwise::CounterclockwiseDepthFirstLocatedUnassignedTblSubexpressionIterator,
                variable::TblExpressionVariable,
            },
        },
    };

    impl<'a, C: UnassignedTblExpressionCompound> From<&'a UnassignedTblExpression<C>>
        for TblFastConstructGetBoundsForPropsSubsumedByUprop
    {
        fn from(expr: &'a UnassignedTblExpression<C>) -> Self {
            let mut first_var_instances: BTreeMap<
                TblExpressionVariable,
                TblSubexpressionInExpressionPath,
            > = BTreeMap::new();
            let get_bounds =
                CounterclockwiseDepthFirstLocatedUnassignedTblSubexpressionIterator::new(expr)
                    .filter_map(|v| match v.into() {
                        UnassignedTblExpressionAtPathEnum::Atom(atom) => {
                            Some(TblExpressionBoundAtomExactValue::new(atom.path, *atom.obj).into())
                        }
                        UnassignedTblExpressionAtPathEnum::Variable(variable) => {
                            match first_var_instances.get(variable.obj) {
                                Some(path) => Some(
                                    TblExpressionBoundValueDuplicated::new(
                                        path.clone(),
                                        variable.path,
                                    )
                                    .0
                                    .into(),
                                ),
                                None => {
                                    first_var_instances.insert(*variable.obj, variable.path);
                                    None
                                }
                            }
                        }
                        UnassignedTblExpressionAtPathEnum::Compound(compound) => Some(
                            TblExpressionBoundCompoundExactLength::new(
                                compound.path,
                                compound.obj.len(),
                            )
                            .into(),
                        ),
                    })
                    .collect();
            Self {
                get_bounds,
                map_to_elem_constructor: Rc::new(first_var_instances.into()),
            }
        }
    }
}
mod retrieval {
    use std::collections::HashSet;

    use proof_calculus::utils::collections::binding::{
        binders::{Binder, GetBinder},
        bounds::{GetBounds, UniqueGetBounds},
    };

    use crate::expressions::types::assigned::binding::{
        bounds::TblExpressionInsertionBound,
        operation_bounds::get_subsumed_by_uprop::fast_construct::TblFastConstructGetBoundsForExprsSubsumedByUexpr,
    };

    impl<B: GetBinder<TblExpressionInsertionBound>> GetBounds<B>
        for TblFastConstructGetBoundsForExprsSubsumedByUexpr
    {
        fn get_from<'binder>(&self, binder: &'binder B) -> HashSet<&'binder <B as Binder>::Value> {
            binder.get_intersection(self.get_bounds.iter())
        }
    }
    impl<B: GetBinder<TblExpressionInsertionBound>> UniqueGetBounds<B>
        for TblFastConstructGetBoundsForExprsSubsumedByUexpr
    {
    }
}
mod operational_retrieval {
    use std::{collections::HashSet, rc::Rc};

    use proof_calculus::{
        propositions::types::assigned::binding::bounds::{
            GetBoundsForConstructiblePropsSubsumedByUprop, GetBoundsForPropsSubsumedByUprop,
        },
        utils::collections::binding::{binders::GetBinder, bounds::GetBounds},
    };

    use crate::{
        expressions::{
            assignments::implementations::btree::constructors::BTreeTblExpressionAssignmentConstructor,
            types::{
                assigned::{
                    TblExpression,
                    binding::{
                        bounds::TblExpressionInsertionBound,
                        operation_bounds::get_subsumed_by_uprop::fast_construct::TblFastConstructGetBoundsForPropsSubsumedByUprop,
                    },
                    compound::TblExpressionCompound,
                },
                unassigned::{UnassignedTblExpression, compound::UnassignedTblExpressionCompound},
            },
        },
        proof_calculus_derived::aliases::propositions::{
            assignments::{TblPropositionalAssignment, TblPropositionalAssignmentConstructor},
            types::unassigned::UnassignedTblProposition,
        },
    };

    impl<
        'elem,
        ElemCompound: 'elem + UnassignedTblExpressionCompound,
        B: GetBinder<TblExpressionInsertionBound>,
    > GetBoundsForPropsSubsumedByUprop<'elem, UnassignedTblProposition<ElemCompound>, B>
        for TblFastConstructGetBoundsForPropsSubsumedByUprop
    {
    }
    impl<
        'elem,
        MapCompound: TblExpressionCompound,
        ElemUcompound: 'elem + UnassignedTblExpressionCompound,
        Assignment: TblPropositionalAssignment<ElemUcompound, MapCompound>,
        B: GetBinder<TblExpressionInsertionBound>,
    >
        GetBoundsForConstructiblePropsSubsumedByUprop<
            'elem,
            TblExpression<MapCompound>,
            UnassignedTblExpression<ElemUcompound>,
            Assignment,
            B,
        > for TblFastConstructGetBoundsForPropsSubsumedByUprop
    where
        BTreeTblExpressionAssignmentConstructor:
            TblPropositionalAssignmentConstructor<ElemUcompound, MapCompound, Assignment>,
    {
        type ElemToMapConstructor = Rc<BTreeTblExpressionAssignmentConstructor>;
        fn get_from_with_elem_to_map_constructors<'binder>(
            &self,
            binder: &'binder B,
        ) -> HashSet<(&'binder B::Value, Self::ElemToMapConstructor)> {
            self.get_from(binder)
                .iter()
                .map(|v| (*v, self.map_to_elem_constructor.clone()))
                .collect()
        }
    }
}
// impl TblFastConstructGetBoundsForExprsSubsumedByUexpr {
//     pub fn get_bounds(&self) -> &Box<[TblExpressionInsertionBound]> {
//         &self.get_bounds
//     }
//     pub fn assignment_constructor(&self) -> &BTreeTblExpressionAssignmentConstructor {
//         &self.assignment_constructor
//     }
// }
