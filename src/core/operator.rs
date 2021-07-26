//! The operators that exist within jodin code

use std::cmp::Ordering;

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
