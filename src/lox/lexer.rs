use std::collections::HashMap;
use std::fmt;

use lazy_static::lazy_static;
use strum_macros::Display;

use super::error::LoxError;

#[derive(Clone, Display)]
pub enum TokenType {
    // Single-character tokens
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star,

    // One or two character tokens
    Bang, BangEqual, Equal, EqualEqual, Greater,
    GreaterEqual, Less, LessEqual,

    // Literals
    Identifier, String, Number,

    // Keywords
    And, Class, Else, False, Fun, For, If, Nil, Or,
    Print, Return, Super, This, True, Var, While,

    // End of file.
    EOF,
}

pub enum Literal {
    String(String),
    Number(f64),
}


lazy_static! {
    static ref KEYWORDS: HashMap<String, TokenType> = {
        let mut m = HashMap::new();

        m.insert("and".to_string(), TokenType::And);
        m.insert("class".to_string(), TokenType::Class);
        m.insert("else".to_string(), TokenType::Else);
        m.insert("false".to_string(), TokenType::False);
        m.insert("for".to_string(), TokenType::For);
        m.insert("fun".to_string(), TokenType::Fun);
        m.insert("if".to_string(), TokenType::If);
        m.insert("nil".to_string(), TokenType::Nil);
        m.insert("or".to_string(), TokenType::Or);
        m.insert("print".to_string(), TokenType::Print);
        m.insert("return".to_string(), TokenType::Return);
        m.insert("super".to_string(), TokenType::Super);
        m.insert("this".to_string(), TokenType::This);
        m.insert("true".to_string(), TokenType::True);
        m.insert("var".to_string(), TokenType::Var);
        m.insert("while".to_string(), TokenType::While);

        m
    };
}

pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<Literal>,
    line: usize,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let literal_str = match &self.literal {
            Some(Literal::String(s)) => s.clone(),
            Some(Literal::Number(n)) => n.to_string(),
            None => "".to_string(),
        };
        write!(f, "{} {} {}", self.token_type, self.lexeme, literal_str)
    }
}

pub struct Lexer {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> (&Vec<Token>, bool) {
        let mut had_error = false;

        while !self.is_at_end() {
            self.start = self.current;
            if let Err(error) = self.scan_token() {
                println!("{}", error);
                had_error = true;
            }
        }

        self.tokens.push(
            Token {
                token_type: TokenType::EOF,
                lexeme: "".to_string(),
                literal: None,
                line: self.line,
            }
        );

        (&self.tokens, had_error)
    }

    fn scan_token(&mut self) -> Result<(), LoxError> {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen, None),
            ')' => self.add_token(TokenType::RightParen, None),
            '{' => self.add_token(TokenType::LeftBrace, None),
            '}' => self.add_token(TokenType::RightBrace, None),
            ',' => self.add_token(TokenType::Comma, None),
            '.' => self.add_token(TokenType::Dot, None),
            '-' => self.add_token(TokenType::Minus, None),
            '+' => self.add_token(TokenType::Plus, None),
            ';' => self.add_token(TokenType::Semicolon, None),
            '*' => self.add_token(TokenType::Star, None),
            '!' => {
                let token_type = if self.match_next('=') { TokenType::BangEqual } else { TokenType::Bang };
                self.add_token(token_type,None);
            },
            '=' => {
                let token_type = if self.match_next('=') { TokenType::EqualEqual } else { TokenType::Equal };
                self.add_token(token_type, None);
            },
            '<' => {
                let token_type = if self.match_next('=') { TokenType::LessEqual } else { TokenType::Less };
                self.add_token(token_type,None);
            },
            '>' => {
                let token_type = if self.match_next('=') { TokenType::GreaterEqual } else { TokenType::Greater };
                self.add_token(token_type,None);
            },
            '/' => {
                if self.match_next('/') {
                    while (self.peek() != '\n') && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash, None);
                }
            },
            _ if c == '_' || c.is_ascii_alphabetic() => self.scan_identifier(),
            '"' => self.scan_string()?,
            _ if c.is_ascii_digit() => self.scan_number(),
            ' ' | '\r' | '\t' => {},
            '\n' => self.line += 1,
            _ => return Err(LoxError::new(self.line, None, "Unexpected character.")),
        }

        Ok(())
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<Literal>) {
        let lexeme = self.source[self.start..self.current].to_string();
        self.tokens.push(
            Token {
                token_type,
                lexeme,
                literal,
                line: self.line,
            }
        );
    }

    fn scan_identifier(&mut self) {
        while self.peek() == '_' || self.peek().is_ascii_alphanumeric() {
            self.advance();
        }

        let lexeme = &self.source[self.start..self.current];
        let identifir_type = KEYWORDS.get(lexeme).unwrap_or(&TokenType::Identifier).clone();

        self.add_token(identifir_type, None);
    }

    fn scan_string(&mut self) -> Result<(), LoxError> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err(LoxError::new(self.line, None, "Unterminated string."));
        }

        self.advance();

        let literal = Literal::String(self.source[(self.start + 1)..(self.current - 1)].to_string());
        self.add_token(TokenType::String, Some(literal));

        Ok(())
    }

    fn scan_number(&mut self) {
        while self.peek().is_numeric() {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_numeric() {
            self.advance();

            while self.peek().is_numeric() {
                self.advance();
            }
        }

        let literal = Literal::Number(self.source[self.start..self.current].parse::<f64>().unwrap());
        self.add_token(TokenType::Number, Some(literal));
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source.chars().nth(self.current - 1).unwrap()
    }

    fn match_next(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        
        self.source.chars().nth(self.current).unwrap()
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }

        self.source.chars().nth(self.current + 1).unwrap()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}
