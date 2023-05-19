use std::fs;
use std::io::{self, Error, Write, BufRead};
use std::marker::PhantomData;

mod lox;

use lox::ast::{AstPrinter, BinaryExpr, UnaryExpr};
use lox::token::Token;
use lox::lexer::Lexer;

use crate::lox::ast::{LiteralExpr, GroupingExpr};
use crate::lox::token::{TokenLiteral, TokenType};

struct Lox {
    had_error: bool,
}

impl Lox {
    fn new() -> Self {
        Self { had_error: false }
    }

    fn run_file(&mut self, path: &str) -> Result<(), Error> {
        let bytes = fs::read(path)?;
        let code = String::from_utf8(bytes).expect("Found invalid UTF-8");

        self.run(code);

        if self.had_error {
            std::process::exit(65);
        }

        Ok(())
    }
    
    fn run_prompt(&mut self) {
        let stdin = io::stdin();
        let reader = stdin.lock();
        let mut stdout = io::stdout();
    
        for line in reader.lines() {
            match line {
                Ok(line) => {
                    print!("> ");
                    stdout.flush().unwrap();

                    self.run(line);

                    self.had_error = false;
                }
                Err(_) => break,
            }
        }
    }
    
    fn run(&mut self, code: String) {
        let mut lexer = Lexer::new(code);
        let (tokens, had_error) = lexer.scan_tokens();
        
        for token in tokens {
            println!("{}", token);
        }

        self.had_error = had_error;
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let expression = BinaryExpr{
        left: Box::new(
            UnaryExpr {
                operator: Token {
                    token_type: TokenType::Minus,
                    lexeme: '-'.to_string(),
                    literal: None,
                    line: 1
                },
                right: Box::new(
                    LiteralExpr {
                        value: Some(TokenLiteral::Number(123.0)),
                        _phantom: PhantomData,
                    },
                ),
            },
        ),
        operator: Token {
            token_type: TokenType::Star,
            lexeme: '*'.to_string(),
            literal: None,
            line: 1,
        },
        right: Box::new(
            GroupingExpr {
                expression: Box::new(
                    LiteralExpr {
                        value: Some(TokenLiteral::Number(45.67)),
                        _phantom: PhantomData,
                    }
                )
            },
        )
    };
    println!("{}", AstPrinter{}.print(&expression));

    let mut lox = Lox::new();
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        1 => lox.run_prompt(),
        2 => lox.run_file(&args[1])?,
        _ => {
            println!("Usage: rlox [script]");
            std::process::exit(64);
        }
    }

    Ok(())
}
