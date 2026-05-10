use std::{collections::HashMap, rc::Rc};

use crate::expressions::{
    assignments::implementations::btree::constructors::BTreeTblExpressionAssignmentConstructor,
    paths::TblSubexpressionInExpressionPath,
    types::unassigned::{
        binding::bounds::UnassignedTblExpressionEquivalenceBound, variable::TblExpressionVariable,
    },
};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TblFastConstructGetBoundsForUexprsEquivalentToUexpr {
    get_bounds: Box<[UnassignedTblExpressionEquivalenceBound]>,
    elem_to_map_constructor: Rc<BTreeTblExpressionAssignmentConstructor>,
    map_to_elem_constructor:
        HashMap<TblSubexpressionInExpressionPath, Box<[TblExpressionVariable]>>,
}
pub type TblFastConstructGetBoundsForUpropsEquivalentToUprop =
    TblFastConstructGetBoundsForUexprsEquivalentToUexpr;

mod construction {
    use std::{collections::BTreeMap, rc::Rc};

    use crate::expressions::{
        paths::TblSubexpressionInExpressionPath,
        types::{
            assigned::binding::bounds::{
                TblExpressionBoundAtomExactValue, TblExpressionBoundCompoundExactLength,
                TblExpressionBoundValueDuplicated,
            },
            unassigned::{
                UnassignedTblExpression,
                at_path_enum::UnassignedTblExpressionAtPathEnum,
                binding::{
                    bounds::UnassignedTblExpressionBoundVariableExistsAtLocation,
                    operation_bounds::get_equivalent_to_uprop::fast_construct::TblFastConstructGetBoundsForUpropsEquivalentToUprop,
                },
                compound::UnassignedTblExpressionCompound,
                subexpressions::iterators::depth_first::counterclockwise::CounterclockwiseDepthFirstLocatedUnassignedTblSubexpressionIterator,
                variable::TblExpressionVariable,
            },
        },
    };

    impl<'a, C: UnassignedTblExpressionCompound> From<&'a UnassignedTblExpression<C>>
        for TblFastConstructGetBoundsForUpropsEquivalentToUprop
    {
        fn from(expr: &'a UnassignedTblExpression<C>) -> Self {
            let mut first_var_instances: BTreeMap<
                TblExpressionVariable,
                TblSubexpressionInExpressionPath,
            > = BTreeMap::new();
            let mut bounds = Vec::new();
            for expr in
                CounterclockwiseDepthFirstLocatedUnassignedTblSubexpressionIterator::new(expr)
            {
                match expr.into() {
                    UnassignedTblExpressionAtPathEnum::Atom(atom) => bounds
                        .push(TblExpressionBoundAtomExactValue::new(atom.path, *atom.obj).into()),
                    UnassignedTblExpressionAtPathEnum::Variable(variable) => {
                        match first_var_instances.get(variable.obj) {
                            Some(path) => bounds.push(
                                TblExpressionBoundValueDuplicated::new(
                                    path.clone(),
                                    variable.path.clone(),
                                )
                                .0
                                .into(),
                            ),
                            None => {
                                first_var_instances.insert(*variable.obj, variable.path.clone());
                            }
                        };
                        bounds.push(
                            UnassignedTblExpressionBoundVariableExistsAtLocation::new(
                                variable.path,
                            )
                            .into(),
                        );
                    }
                    UnassignedTblExpressionAtPathEnum::Compound(compound) => bounds.push(
                        TblExpressionBoundCompoundExactLength::new(
                            compound.path,
                            compound.obj.len(),
                        )
                        .into(),
                    ),
                }
            }
            Self {
                get_bounds: bounds.into(),
                elem_to_map_constructor: Rc::new(first_var_instances.into()),
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

    use crate::expressions::types::unassigned::binding::{
        bounds::UnassignedTblExpressionEquivalenceBound,
        operation_bounds::get_equivalent_to_uprop::fast_construct::TblFastConstructGetBoundsForUexprsEquivalentToUexpr,
    };

    impl<B: GetBinder<UnassignedTblExpressionEquivalenceBound>> GetBounds<B>
        for TblFastConstructGetBoundsForUexprsEquivalentToUexpr
    {
        fn get_from<'binder>(&self, binder: &'binder B) -> HashSet<&'binder <B as Binder>::Value> {
            binder.get_intersection(self.get_bounds.iter())
        }
    }
    impl<B: GetBinder<UnassignedTblExpressionEquivalenceBound>> UniqueGetBounds<B>
        for TblFastConstructGetBoundsForUexprsEquivalentToUexpr
    {
    }
}
mod operational_retrieval {
    use proof_calculus::{
        propositions::{
            assignments::PropositionalAssignmentConstructor,
            types::unassigned::binding::bounds::{
                GetBoundsForConstructibleUpropsEquivalentToUprop,
                GetBoundsForUpropsEquivalentToUprop,
            },
        },
        utils::collections::binding::binders::GetBinder,
    };
    use std::collections::HashSet;

    use crate::{
        expressions::{
            assignments::implementations::btree::constructors::BTreeTblPropositionAssignmentConstructor,
            types::unassigned::{
                binding::{
                    bounds::UnassignedTblExpressionEquivalenceBound,
                    operation_bounds::get_equivalent_to_uprop::fast_construct::TblFastConstructGetBoundsForUexprsEquivalentToUexpr,
                },
                compound::UnassignedTblExpressionCompound,
            },
        },
        proof_calculus_derived::aliases::propositions::{
            assignments::{
                TblPartialPropositionalAssignment, TblPartialPropositionalAssignmentConstructor,
                TblPropositionalAssignmentConstructor,
            },
            types::unassigned::UnassignedTblProposition,
        },
    };

    impl<
        'prop,
        C: 'prop + UnassignedTblExpressionCompound,
        B: GetBinder<UnassignedTblExpressionEquivalenceBound>,
    > GetBoundsForUpropsEquivalentToUprop<'prop, UnassignedTblProposition<C>, B>
        for TblFastConstructGetBoundsForUexprsEquivalentToUexpr
    {
    }
    impl<
        'elem,
        MapUcompound: UnassignedTblExpressionCompound,
        ElemUcompound: 'elem + UnassignedTblExpressionCompound,
        ElemToMapAssignment: TblPartialPropositionalAssignment<ElemUcompound, MapUcompound>,
        MapToElemAssignment: TblPartialPropositionalAssignment<MapUcompound, ElemUcompound>,
        B: GetBinder<UnassignedTblExpressionEquivalenceBound>,
    >
        GetBoundsForConstructibleUpropsEquivalentToUprop<
            'elem,
            UnassignedTblProposition<MapUcompound>,
            UnassignedTblProposition<ElemUcompound>,
            ElemToMapAssignment,
            MapToElemAssignment,
            B,
        > for TblFastConstructGetBoundsForUexprsEquivalentToUexpr
    where
        BTreeTblPropositionAssignmentConstructor: TblPartialPropositionalAssignmentConstructor<
                ElemUcompound,
                MapUcompound,
                ElemToMapAssignment,
            > + TblPartialPropositionalAssignmentConstructor<
                MapUcompound,
                ElemUcompound,
                MapToElemAssignment,
            >,
    {
        type ElemToMapConstructor = BTreeTblPropositionAssignmentConstructor;

        fn get_from_with_elem_to_map_constructors<'binder>(
            &self,
            binder: &'binder B,
        ) -> HashSet<(&'binder B::Value, Self::ElemToMapConstructor)> {
            todo!()
        }

        type MapToElemConstructor = BTreeTblPropositionAssignmentConstructor;

        fn get_from_with_map_to_elem_constructors<'binder>(
            &self,
            binder: &'binder B,
        ) -> HashSet<(&'binder B::Value, Self::MapToElemConstructor)> {
            todo!()
        }
    }
}
// impl TblFastConstructGetBoundsForUexprsEquivalentToUexpr {
//     pub fn bounds(&self) -> &Box<[UnassignedTblExpressionEquivalenceBound]> {
//         &self.get_bounds
//     }
// }
