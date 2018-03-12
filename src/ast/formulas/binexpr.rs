use ast::prelude::*;
use ast::formulas::checks;
use ast::terms::ExprMarker;

use std::marker::PhantomData;

pub mod prelude {
    pub use super::{
        BinBoolExpr
    };
}

/// Generic binary formula expression.
/// 
/// Used by concrete binary formula expressions as base template.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BinBoolExpr<M> {
    /// The two child expressions.
    pub children: P<BinExprChildren>,
    /// Marker to differentiate bool expressions from each
    /// other using the type system.
    marker: PhantomData<M>
}

impl<M> BinBoolExpr<M> {
    /// Returns a new binary formula expression with the given child expressions.
    /// 
    /// # Errors
    /// 
    /// - If `lhs` or `rhs` are not of bool type.
    pub fn new<E1, E2>(lhs: E1, rhs: E2) -> Result<Self, String>
        where E1: Into<AnyExpr>,
              E2: Into<AnyExpr>
    {
        let lhs = lhs.into();
        let rhs = rhs.into();
        checks::expect_bool_ty(&lhs)?;
        checks::expect_bool_ty(&rhs)?;
        Ok(Self{ children: BinExprChildren::new_boxed(lhs, rhs), marker: PhantomData })
    }
}

impl<M> BoolExpr for BinBoolExpr<M> where Self: Into<AnyExpr> {}

impl<M> Children for BinBoolExpr<M> {
    fn children(&self) -> ChildrenIter {
        self.children.children()
    }
}

impl<M> ChildrenMut for BinBoolExpr<M> {
    fn children_mut(&mut self) -> ChildrenIterMut {
        self.children.children_mut()
    }
}

impl<M> IntoChildren for BinBoolExpr<M> {
    fn into_children(self) -> IntoChildrenIter {
        self.children.into_children()
    }
}

impl<M> HasType for BinBoolExpr<M> {
    fn ty(&self) -> Type {
        Type::Bool
    }
}

impl<M> HasKind for BinBoolExpr<M>
    where M: ExprMarker
{
    fn kind(&self) -> ExprKind {
        M::EXPR_KIND
    }
}

impl<M> HasArity for BinBoolExpr<M> {
    fn arity(&self) -> usize {
        2
    }
}
