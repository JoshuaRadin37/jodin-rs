//! The operators that exist within jodin code

use crate::core::error::{JodinError, JodinErrorType, JodinResult};
use crate::core::literal::Literal;
use num_traits::PrimInt;
use std::cmp::Ordering;
use std::convert::TryInto;

/// The operators
#[derive(Debug)]
pub enum Operator {
    /// !
    Not,
    /// ++
    Increment,
    /// --
    Decrement,
    /// +
    Plus,
    /// -
    Minus,
    /// *
    Star,
    /// %
    Modulo,
    /// /
    Divide,
    /// ^
    Xor,
    /// &
    And,
    /// |
    Or,
    /// &&
    Dand,
    /// ||
    Dor,
    /// new
    New,
    /// []
    Index,
    /// ,
    Comma,
    /// (...)
    Call,
    /// ...
    Ellipsis,
    /// ==
    Equal,
    /// !=
    Nequal,
    /// <
    Lt,
    /// <=
    Lte,
    /// >
    Gt,
    /// >=
    Gte,
    /// <<<
    LShift,
    /// >>>
    RShift,
}

impl Operator {
    /// Gets the precedence of the operator
    pub fn as_precedence(&self) -> Precedence {
        Precedence(self)
    }
}

/// A structure that gets the precedence of an operator
#[derive(Debug)]
pub struct Precedence<'a>(&'a Operator);

impl<'a> Precedence<'a> {
    /// The value associated with a level of precedence
    pub fn value(&self) -> Option<usize> {
        let val = match self.0 {
            Operator::Not => 2,
            Operator::Increment => 1,
            Operator::Decrement => 1,
            Operator::Plus => 4,
            Operator::Minus => 4,
            Operator::Star => 3,
            Operator::Modulo => 3,
            Operator::Divide => 3,
            Operator::Xor => 9,
            Operator::And => 8,
            Operator::Or => 10,
            Operator::Dand => 11,
            Operator::Dor => 12,
            Operator::Index => 1,
            Operator::Comma => 15,
            Operator::Call => 1,
            Operator::Equal => 7,
            Operator::Nequal => 7,
            Operator::Lt => 6,
            Operator::Lte => 6,
            Operator::Gt => 6,
            Operator::Gte => 6,
            Operator::LShift => 5,
            Operator::RShift => 5,
            _default => return None,
        };
        Some(val)
    }
}

impl PartialEq for Precedence<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }
}

impl PartialOrd for Precedence<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self.value(), other.value()) {
            (Some(l), Some(r)) => Some(l.cmp(&r)),
            _ => None,
        }
    }
}

/// A trait to attempt const evaluation of values
pub trait TryConstEvaluation<T> {
    /// Evaluate an operator with two arguments
    fn evaluate_binop<R: TryInto<T, Error = JodinError>>(
        &self,
        lhs: T,
        rhs: R,
    ) -> JodinResult<Literal>;
    /// Evaluate an operator with one argument
    fn evaluate_uniop(&self, lhs: T) -> JodinResult<Literal>;
}

/// Types that implement this trait are integers
pub trait NumType: PrimInt + TryInto<Literal, Error = JodinError> {}

macro_rules! integer {
    ($type_name:ty) => {
        impl NumType for $type_name {}
    };
}

integer!(u8);
integer!(u16);
integer!(u32);
integer!(u64);

integer!(i8);
integer!(i16);
integer!(i32);
integer!(i64);

impl<N: NumType> TryConstEvaluation<N> for Operator {
    fn evaluate_binop<R: TryInto<N>>(&self, lhs: N, rhs: R) -> JodinResult<Literal> {
        let rhs = rhs
            .try_into()
            .map_err(|e| JodinErrorType::IncorrectLiteralType)?;

        match self {
            Operator::Plus => (lhs + rhs).try_into(),
            Operator::Minus => (lhs - rhs).try_into(),
            Operator::Star => (lhs * rhs).try_into(),
            Operator::Modulo => (lhs % rhs).try_into(),
            Operator::Divide => (lhs / rhs).try_into(),
            Operator::Xor => (lhs ^ rhs).try_into(),
            Operator::And => (lhs & rhs).try_into(),
            Operator::Or => (lhs | rhs).try_into(),
            Operator::Equal => (lhs == rhs).try_into(),
            Operator::Nequal => (lhs != rhs).try_into(),
            Operator::Lt => (lhs < rhs).try_into(),
            Operator::Lte => (lhs <= rhs).try_into(),
            Operator::Gt => (lhs > rhs).try_into(),
            Operator::Gte => (lhs >= rhs).try_into(),
            Operator::LShift => {
                let rhs = rhs.to_usize().ok_or(JodinErrorType::IncorrectLiteralType)?;
                (lhs << rhs)
                    .try_into()
                    .map_err(|e| JodinErrorType::IncorrectLiteralType.into())
            }
            Operator::RShift => {
                let rhs = rhs.to_usize().ok_or(JodinErrorType::IncorrectLiteralType)?;
                (lhs >> rhs)
                    .try_into()
                    .map_err(|e| JodinErrorType::IncorrectLiteralType.into())
            }
            _ => return Err(JodinErrorType::InvalidOperatorForConstantExpression.into()),
        }
    }

    fn evaluate_uniop(&self, lhs: N) -> JodinResult<Literal> {
        todo!()
    }
}

impl TryConstEvaluation<char> for Operator {
    fn evaluate_binop<R: TryInto<char, Error = JodinError>>(
        &self,
        lhs: char,
        rhs: R,
    ) -> JodinResult<Literal> {
        todo!()
    }

    fn evaluate_uniop(&self, lhs: char) -> JodinResult<Literal> {
        todo!()
    }
}

impl TryConstEvaluation<f32> for Operator {
    fn evaluate_binop<R: TryInto<f32, Error = JodinError>>(
        &self,
        lhs: f32,
        rhs: R,
    ) -> JodinResult<Literal> {
        todo!()
    }

    fn evaluate_uniop(&self, lhs: f32) -> JodinResult<Literal> {
        todo!()
    }
}

impl TryConstEvaluation<f64> for Operator {
    fn evaluate_binop<R: TryInto<f64, Error = JodinError>>(
        &self,
        lhs: f64,
        rhs: R,
    ) -> JodinResult<Literal> {
        todo!()
    }

    fn evaluate_uniop(&self, lhs: f64) -> JodinResult<Literal> {
        todo!()
    }
}

impl TryConstEvaluation<bool> for Operator {
    fn evaluate_binop<R: TryInto<bool, Error = JodinError>>(
        &self,
        lhs: bool,
        rhs: R,
    ) -> JodinResult<Literal> {
        todo!()
    }

    fn evaluate_uniop(&self, lhs: bool) -> JodinResult<Literal> {
        todo!()
    }
}
