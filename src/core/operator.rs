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
}
