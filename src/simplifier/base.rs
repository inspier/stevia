use ast2::prelude::*;

use std::mem;
use std::ops::BitOrAssign;

pub mod prelude {
    pub use super::{
        BaseTransformer,
        Transformer,
        TransformResult,
        AnyTransformer,
        AnyExprAndTransformResult
    };
}

/// Describes whether the result of a transformation actually transformed
/// the input or did nothing to it.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TransformResult {
    /// States that the transformation had no effect on the input.
    Identity,
    /// States that the transformation transformed the input.
    Transformed
}

impl BitOrAssign for TransformResult {
    /// Assigns this `TransformResult` to `rhs`.
    /// 
    /// This works equivalent to boolean or-assign
    /// where `Identity` is equal to `false` and
    /// `Transformed` is equal to `true`.
    fn bitor_assign(&mut self, rhs: TransformResult) {
        match rhs {
            TransformResult::Transformed => *self = rhs,
            TransformResult::Identity    => ()
        }
    }
}

/// Simple struct to store a transformed expression
/// and a state indicating if it was actually transformed.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AnyExprAndTransformResult {
    /// States if `expr` actually got transformed.
    pub result: TransformResult,
    /// The (probably) transformed expression.
    pub expr: AnyExpr
}

impl AnyExprAndTransformResult {
    /// Creates a new `AnyExprAndTransformResult` with the given expression and state.
    pub fn new(result: TransformResult, expr: AnyExpr) -> AnyExprAndTransformResult {
        AnyExprAndTransformResult{expr, result}
    }

    /// Creates a new non-transformed `AnyExprAndTransformResult` for the given expression.
    pub fn identity<E>(expr: E) -> AnyExprAndTransformResult
        where E: Into<AnyExpr>
    {
        AnyExprAndTransformResult::new(TransformResult::Identity, expr.into())
    }
}

pub trait Transformer: Copy {
    fn transform_cond(self, cond: expr::IfThenElse) -> AnyExprAndTransformResult {
        AnyExprAndTransformResult::identity(cond)
    }

    fn transform_var(self, bool_const: expr::Symbol) -> AnyExprAndTransformResult {
        AnyExprAndTransformResult::identity(bool_const)
    }

    fn transform_bool_const(self, bool_const: expr::BoolConst) -> AnyExprAndTransformResult {
        AnyExprAndTransformResult::identity(bool_const)
    }

    fn transform_bool_equals(self, bool_equals: expr::BoolEquals) -> AnyExprAndTransformResult {
        AnyExprAndTransformResult::identity(bool_equals)
    }

    fn transform_and(self, and: expr::And) -> AnyExprAndTransformResult {
        AnyExprAndTransformResult::identity(and)
    }

    fn transform_or(self, or: expr::Or) -> AnyExprAndTransformResult {
        AnyExprAndTransformResult::identity(or)
    }

    fn transform_not(self, not: expr::Not) -> AnyExprAndTransformResult {
        AnyExprAndTransformResult::identity(not)
    }
}

/// Simple transformer that does nothing.
/// 
/// This is useful for testing as long as there are no other
/// real transformers to test the system.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct NoopTransformer;
impl Transformer for NoopTransformer {}
impl AutoImplAnyTransformer for NoopTransformer {}
impl Default for NoopTransformer {
    fn default() -> Self {
        NoopTransformer
    }
}

/// Expression transformers that may transform `AnyExpr` instances.
pub trait AnyTransformer: Copy {
    /// Transforms the given mutable `AnyExpr` inplace.
    /// 
    /// Returns a state indicating whether the given expression was actually transformed.
    fn transform_any_expr(self, expr: &mut AnyExpr) -> TransformResult;

    /// Consumed the given `AnyExpr` and transforms it.
    /// 
    /// Returns the resulting expression after the transformation and a state
    /// indicating whether the consumed expression was actually transformed.
    fn into_transform_any_expr(self, expr: AnyExpr) -> AnyExprAndTransformResult;
}

/// Implement this to activate automatic default implementation
/// of the `AnyTransformer` trait.
pub trait AutoImplAnyTransformer {}

impl<T> AnyTransformer for T where T: Transformer + AutoImplAnyTransformer {
    fn transform_any_expr(self, expr: &mut AnyExpr) -> TransformResult {
        let temp = AnyExpr::from(expr::BoolConst::f());
		let input = mem::replace(expr, temp);
		let AnyExprAndTransformResult{result, expr: transformed} =
            self.into_transform_any_expr(input);
        mem::replace(expr, transformed);
        result
    }

    fn into_transform_any_expr(self, expr: AnyExpr) -> AnyExprAndTransformResult {
        use self::AnyExpr::*;
        match expr {
            IfThenElse(expr) => self.transform_cond(expr),
            Symbol(expr) => self.transform_var(expr),
            BoolConst(expr) => self.transform_bool_const(expr),
            BoolEquals(expr) => self.transform_bool_equals(expr),
            Not(expr) => self.transform_not(expr),
            And(expr) => self.transform_and(expr),
            Or(expr) => self.transform_or(expr),
            _ => unimplemented!()
        }
    }
}

macro_rules! create_base_transformer {
    (struct $name:ident; $(($id:ident, $trans:ty)),+) => {
        /// The base transformer including a collection of sub-transformers.
        /// 
        /// This traverses the expression tree and performs transformations
        /// using all given transformers.
        #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
        pub struct $name {
            $($id: $trans),*
        }

        impl $name {
            pub fn new($($id: $trans),*) -> Self {
                $name{
                    $($id),*
                }
            }

            fn forward_transform_any_expr(self, expr: &mut AnyExpr) -> TransformResult {
                let mut result = TransformResult::Identity;
                $(result |= self.$id.transform_any_expr(expr));*;
                result
            }

            pub fn traverse_transform_any_expr(self, expr: &mut AnyExpr) -> TransformResult {
                let mut result = TransformResult::Identity;
                for child in expr.childs_mut() {
                    result |= self.traverse_transform_any_expr(child);
                }
                result |= self.forward_transform_any_expr(expr);
                result
            }
        }

        impl AnyTransformer for $name {
            fn transform_any_expr(self, expr: &mut AnyExpr) -> TransformResult {
                self.traverse_transform_any_expr(expr)
            }

            fn into_transform_any_expr(self, expr: AnyExpr) -> AnyExprAndTransformResult {
                let mut expr = expr;
                let result = self.transform_any_expr(&mut expr);
                AnyExprAndTransformResult::new(result, expr)
            }
        }
    }
}

create_base_transformer!{
    struct BaseTransformer;

    (_0, NoopTransformer)
}