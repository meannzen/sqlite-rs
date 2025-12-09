use super::token::Token;

#[derive(Debug)]
pub struct Lexer {
    input: Vec<char>,
    pos: usize,
    read_pos: usize,
    ch: Option<char>,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut l = Lexer {
            input: input.chars().collect(),
            pos: 0,
            read_pos: 0,
            ch: None,
        };
        l.read_char();
        l
    }

    fn read_char(&mut self) {
        if self.read_pos >= self.input.len() {
            self.ch = None;
        } else {
            self.ch = Some(self.input[self.read_pos]);
        }
        self.pos = self.read_pos;
        self.read_pos += 1;
    }

    fn peek(&self) -> Option<char> {
        if self.read_pos >= self.input.len() {
            None
        } else {
            Some(self.input[self.read_pos])
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.ch {
            if !c.is_whitespace() {
                break;
            }
            self.read_char();
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();

        let tok = match self.ch? {
            '*' => Token::Asterisk,
            ',' => Token::Comma,
            ';' => Token::Semicolon,
            '(' => Token::LParen,
            ')' => Token::RParen,
            '=' => Token::Eq,
            '!' if self.peek() == Some('=') => {
                self.read_char();
                Token::Neq
            }
            '<' if self.peek() == Some('=') => {
                self.read_char();
                Token::Lte
            }
            '>' if self.peek() == Some('=') => {
                self.read_char();
                Token::Gte
            }
            '<' => Token::Lt,
            '>' => Token::Gt,
            '.' => Token::Dot,
            '\'' => self.read_string(),
            c if c.is_ascii_digit() => return Some(self.read_number()),
            c if c.is_ascii_alphabetic() || c == '_' => return Some(self.read_identifier()),
            _ => return None,
        };

        self.read_char();
        Some(tok)
    }

    fn read_string(&mut self) -> Token {
        self.read_char(); // consume '
        let start = self.pos;
        while self.ch != Some('\'') && self.ch.is_some() {
            self.read_char();
        }
        let s: String = self.input[start..self.pos].iter().collect();
        self.read_char(); // consume closing '
        Token::StringLiteral(s)
    }

    fn read_number(&mut self) -> Token {
        let start = self.pos;
        while let Some(c) = self.ch {
            if !c.is_ascii_digit() && c != '.' {
                break;
            }
            self.read_char();
        }
        let num_str: String = self.input[start..self.pos].iter().collect();

        if num_str.contains('.') {
            Token::Float(num_str.parse().unwrap())
        } else {
            Token::Integer(num_str.parse().unwrap())
        }
    }

    fn read_identifier(&mut self) -> Token {
        let start = self.pos;
        while let Some(c) = self.ch {
            if !c.is_ascii_alphanumeric() && c != '_' {
                break;
            }
            self.read_char();
        }
        let ident: String = self.input[start..self.pos].iter().collect();

        match ident.to_uppercase().as_str() {
            "SELECT" => Token::Select,
            "FROM" => Token::From,
            "WHERE" => Token::Where,
            "ORDER" => Token::Order,
            "BY" => Token::By,
            "LIMIT" => Token::Limit,
            "ASC" => Token::Asc,
            "DESC" => Token::Desc,
            "JOIN" => Token::Join,
            "INNER" => Token::Inner,
            "LEFT" => Token::Left,
            "RIGHT" => Token::Right,
            "ON" => Token::On,
            "AND" => Token::And,
            "OR" => Token::Or,
            "COUNT" => Token::Count,
            "SUM" => Token::Sum,
            "AVG" => Token::Avg,
            "MIN" => Token::Min,
            "MAX" => Token::Max,
            _ => Token::Identifier(ident),
        }
    }
}
