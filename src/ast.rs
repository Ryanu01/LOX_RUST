pub enum Expression {
    Literal {
        value: LiteralValue
    },

    Unary {
        operator: UnaryOperator,
        expression: Box<Expression>
    },

    Binary {
        left: Box<Expression>,
        operator: BinaryOperator,
        right: Box<Expression>
    },

    Grouping {
        expression: Box<Expression>
    }
}

#[allow(unused)]
pub enum LiteralValue {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}

#[allow(unused)]
pub enum UnaryOperator {
    Bang,
    Minus
}

#[allow(unused)]
pub enum BinaryOperator {
    EqualEqual,
    BangEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Minus,
    Plus,
    Slash,
    Star
}