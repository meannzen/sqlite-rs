#[derive(Debug, PartialEq)]
pub enum Expr {
    Identifier(String),
    String(String),
    Integer(i64),
    Float(f64),
    Star,
    Call {
        name: String,
        args: Vec<Expr>,
    },
    Binary {
        left: Box<Expr>,
        op: String,
        right: Box<Expr>,
    },
    Column {
        table: Option<String>,
        name: String,
    },
}

#[derive(Debug, PartialEq)]
pub struct Select {
    pub columns: Vec<Expr>,
    pub table: String,
    pub joins: Vec<Join>,
    pub where_clause: Option<Expr>,
    pub order_by: Vec<(Expr, OrderDir)>,
    pub limit: Option<i64>,
}

#[derive(Debug, PartialEq)]
pub struct Join {
    pub kind: JoinKind,
    pub table: String,
    pub condition: Expr,
}

#[derive(Debug, PartialEq)]
pub enum JoinKind {
    Inner,
    Left,
    Right,
}

#[derive(Debug, PartialEq)]
pub enum OrderDir {
    Asc,
    Desc,
}
