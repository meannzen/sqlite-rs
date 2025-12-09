#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Select,
    From,
    Where,
    Order,
    By,
    Limit,
    Asc,
    Desc,
    Join,
    Inner,
    Left,
    Right,
    On,
    Asterisk,
    Comma,
    Semicolon,
    Dot,
    Identifier(String),
    StringLiteral(String),
    Integer(i64),
    Float(f64),
    Eq,
    Neq,
    Lt,
    Gt,
    Lte,
    Gte,
    LParen,
    RParen,
    And,
    Or,
    Count,
    Sum,
    Avg,
    Min,
    Max,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Token::Identifier(s) => write!(f, "ID({})", s),
            Token::StringLiteral(s) => write!(f, "'{}'", s),
            Token::Integer(n) => write!(f, "{}", n),
            Token::Float(n) => write!(f, "{}", n),
            _ => write!(f, "{:?}", self),
        }
    }
}
