use crate::{ast::*, lexer::Lexer, token::Token};

pub struct Parser {
    lexer: Lexer,
    current: Option<Token>,
    peek: Option<Token>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let mut p = Parser {
            lexer,
            current: None,
            peek: None,
        };
        p.next();
        p.next();
        p
    }

    fn next(&mut self) {
        self.current = self.peek.take();
        self.peek = self.lexer.next_token();
    }

    fn expect(&mut self, expected: Token) -> bool {
        if self.current == Some(expected.clone()) {
            self.next();
            true
        } else {
            false
        }
    }

    fn curr(&self) -> &Option<Token> {
        &self.current
    }

    pub fn parse_select(&mut self) -> Result<Select, String> {
        if !self.expect(Token::Select) {
            return Err("Expected SELECT".into());
        }

        let columns = self.parse_columns()?;

        if !self.expect(Token::From) {
            return Err("Expected FROM".into());
        }

        let table = self.expect_identifier("Expected table name")?;

        let mut joins = Vec::new();
        let mut where_clause = None;
        let mut order_by = Vec::new();
        let mut limit = None;

        while matches!(
            self.curr(),
            Some(Token::Join | Token::Inner | Token::Left | Token::Right)
        ) {
            joins.push(self.parse_join()?);
        }

        if self.current == Some(Token::Where) {
            self.next();
            where_clause = Some(self.parse_expr()?);
        }

        if self.current == Some(Token::Order) {
            self.next();
            self.expect(Token::By);
            order_by = self.parse_order_by()?;
        }

        if self.current == Some(Token::Limit) {
            self.next();
            limit = match self.current.take() {
                Some(Token::Integer(n)) => {
                    self.next();
                    Some(n)
                }
                _ => return Err("Expected integer after LIMIT".into()),
            };
        }

        Ok(Select {
            columns,
            table,
            joins,
            where_clause,
            order_by,
            limit,
        })
    }

    fn parse_columns(&mut self) -> Result<Vec<Expr>, String> {
        let mut cols = Vec::new();

        loop {
            cols.push(if *self.curr() == Some(Token::Asterisk) {
                self.next();
                Expr::Star
            } else if matches!(
                self.curr(),
                Some(Token::Count | Token::Sum | Token::Avg | Token::Min | Token::Max)
            ) {
                self.parse_function_call()?
            } else {
                self.parse_expr()?
            });

            if self.current != Some(Token::Comma) {
                break;
            }
            self.next(); // consume comma
        }

        Ok(cols)
    }

    fn parse_function_call(&mut self) -> Result<Expr, String> {
        let name = match self.current.take() {
            Some(Token::Count) => "COUNT".to_string(),
            Some(Token::Sum) => "SUM".to_string(),
            Some(Token::Avg) => "AVG".to_string(),
            Some(Token::Min) => "MIN".to_string(),
            Some(Token::Max) => "MAX".to_string(),
            _ => return Err("Expected function".into()),
        };
        self.next();

        self.expect(Token::LParen);
        let arg = if *self.curr() == Some(Token::Asterisk) {
            self.next();
            Expr::Star
        } else {
            self.parse_expr()?
        };
        self.expect(Token::RParen);

        Ok(Expr::Call {
            name,
            args: vec![arg],
        })
    }

    fn parse_expr(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_primary()?;

        while matches!(
            self.curr(),
            Some(Token::Eq | Token::Neq | Token::Lt | Token::Gt | Token::Lte | Token::Gte)
        ) {
            let op = match self.current.take() {
                Some(Token::Eq) => "=",
                Some(Token::Neq) => "!=",
                Some(Token::Lt) => "<",
                Some(Token::Gt) => ">",
                Some(Token::Lte) => "<=",
                Some(Token::Gte) => ">=",
                _ => unreachable!(),
            };
            self.next();
            let right = self.parse_primary()?;
            left = Expr::Binary {
                left: Box::new(left),
                op: op.to_string(),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_primary(&mut self) -> Result<Expr, String> {
        match self.current.take() {
            Some(Token::Identifier(id)) => {
                self.next();
                if self.current == Some(Token::Dot) {
                    self.next();
                    let col = self.expect_identifier("Expected column name")?;
                    Ok(Expr::Column {
                        table: Some(id),
                        name: col,
                    })
                } else {
                    Ok(Expr::Identifier(id))
                }
            }
            Some(Token::StringLiteral(s)) => {
                self.next();
                Ok(Expr::String(s))
            }
            Some(Token::Integer(n)) => {
                self.next();
                Ok(Expr::Integer(n))
            }
            Some(Token::Float(n)) => {
                self.next();
                Ok(Expr::Float(n))
            }
            _ => Err(format!("Unexpected token: {:?}", self.current)),
        }
    }

    fn expect_identifier(&mut self, msg: &str) -> Result<String, String> {
        match self.current.take() {
            Some(Token::Identifier(s)) => {
                self.next();
                Ok(s)
            }
            _ => Err(msg.into()),
        }
    }

    fn parse_join(&mut self) -> Result<Join, String> {
        let kind = if self.current == Some(Token::Left) {
            self.next();
            JoinKind::Left
        } else if self.current == Some(Token::Right) {
            self.next();
            JoinKind::Right
        } else if self.current == Some(Token::Inner) {
            self.next();
            JoinKind::Inner
        } else {
            JoinKind::Inner
        };

        self.expect(Token::Join);
        let table = self.expect_identifier("Expected table name after JOIN")?;

        self.expect(Token::On);
        let condition = self.parse_expr()?;

        Ok(Join {
            kind,
            table,
            condition,
        })
    }

    fn parse_order_by(&mut self) -> Result<Vec<(Expr, OrderDir)>, String> {
        let mut items = Vec::new();
        loop {
            let expr = self.parse_primary()?;
            let dir = if self.current == Some(Token::Asc) {
                self.next();
                OrderDir::Asc
            } else if self.current == Some(Token::Desc) {
                self.next();
                OrderDir::Desc
            } else {
                OrderDir::Asc
            };
            items.push((expr, dir));

            if self.current != Some(Token::Comma) {
                break;
            }
            self.next();
        }
        Ok(items)
    }
}
