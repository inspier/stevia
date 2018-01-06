use ast2::prelude::*;

use std::slice;
use std::cmp;

/// Re-exports commonly used items of this module.
pub mod prelude {
    pub use super::{
		ChildsIter	
	};
}

/// Iterator over immutable child expressions.
#[derive(Debug, Clone)]
pub enum ChildsIter<'p> {
    Inl(InlChildsIter<'p>),
    Ext(ExtChildsIter<'p>)
}

#[derive(Debug, Clone)]
pub struct InlChildsIter<'p> {
    childs: [Option<&'p Expr>; 3],
    cur: usize
}

impl<'p> InlChildsIter<'p> {
    fn from_array(childs: [Option<&'p Expr>; 3]) -> InlChildsIter {
        InlChildsIter{childs, cur: 0}
    }

    pub fn none() -> InlChildsIter<'p> {
        InlChildsIter::from_array([None; 3])
    }

	pub fn unary(fst: &'p Expr) -> InlChildsIter<'p> {
		InlChildsIter::from_array([Some(fst), None, None])
	}

	pub fn binary(fst: &'p Expr, snd: &'p Expr) -> InlChildsIter<'p> {
		InlChildsIter::from_array([Some(fst), Some(snd), None])
	}

	pub fn ternary(fst: &'p Expr, snd: &'p Expr, trd: &'p Expr) -> InlChildsIter<'p> {
		InlChildsIter::from_array([Some(fst), Some(snd), Some(trd)])
	}
}

impl<'p> Iterator for InlChildsIter<'p> {
    type Item = &'p Expr;

    fn next(&mut self) -> Option<Self::Item> {
        let elem: Option<Self::Item> = self.childs[self.cur];
        self.cur = cmp::min(self.cur + 1, 2);
        elem
    }
}

#[derive(Debug, Clone)]
pub struct ExtChildsIter<'p> {
    childs: slice::Iter<'p, Expr>
}

impl<'p> ExtChildsIter<'p> {
    fn from_slice(childs: &'p [Expr]) -> ExtChildsIter {
        ExtChildsIter{childs: childs.into_iter()}
    }
}

impl<'p> Iterator for ExtChildsIter<'p> {
    type Item = &'p Expr;

    fn next(&mut self) -> Option<Self::Item> {
        self.childs.next()
    }
}

impl<'p> ChildsIter<'p> {
	pub fn none() -> ChildsIter<'p> {
        ChildsIter::Inl(InlChildsIter::none())
	}

	pub fn unary(fst: &'p Expr) -> ChildsIter<'p> {
        ChildsIter::Inl(InlChildsIter::unary(fst))
	}

	pub fn binary(fst: &'p Expr, snd: &'p Expr) -> ChildsIter<'p> {
        ChildsIter::Inl(InlChildsIter::binary(fst, snd))
	}

	pub fn ternary(
		fst: &'p Expr,
		snd: &'p Expr,
		trd: &'p Expr) -> ChildsIter<'p>
	{
		ChildsIter::Inl(InlChildsIter::ternary(fst, snd, trd))
	}

	pub fn nary(childs: &'p [Expr]) -> ChildsIter<'p> {
		ChildsIter::Ext(ExtChildsIter::from_slice(childs))
	}
}

impl<'p> Iterator for ChildsIter<'p> {
	type Item = &'p Expr;

	fn next(&mut self) -> Option<Self::Item> {
		use self::ChildsIter::*;
		match *self {
			Inl(ref mut iter) => iter.next(),
			Ext(ref mut iter) => iter.next()
		}
	}
}
