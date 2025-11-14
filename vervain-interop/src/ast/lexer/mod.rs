/*
░██    ░██ ░██████████ ░█████████  ░██    ░██    ░███    ░██████░███    ░██ 
░██    ░██ ░██         ░██     ░██ ░██    ░██   ░██░██     ░██  ░████   ░██ 
░██    ░██ ░██         ░██     ░██ ░██    ░██  ░██  ░██    ░██  ░██░██  ░██ 
░██    ░██ ░█████████  ░█████████  ░██    ░██ ░█████████   ░██  ░██ ░██ ░██ 
 ░██  ░██  ░██         ░██   ░██    ░██  ░██  ░██    ░██   ░██  ░██  ░██░██ 
  ░██░██   ░██         ░██    ░██    ░██░██   ░██    ░██   ░██  ░██   ░████ 
   ░███    ░██████████ ░██     ░██    ░███    ░██    ░██ ░██████░██    ░███ 
*/

// THE VERVAIN COMPILER
// COPYRIGHT EST. 2025
// WRITTEN BY @NVTTLES

use super::logos::Logos;
use regex::Regex;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token {
    // Keywords
    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("func")]
    Function,
    #[token("while")]
    While,
    #[token("for")]
    For,
    #[token("return")]
    Return,
    #[token("break")]
    Break,
    #[token("lambda")]
    LambdaKeyword,
    #[token("private")]
    Private,
    #[token("public")]
    Public,
    #[token("end")]
    End,

    // Literals
    #[token("~~")]
    Comment,
    #[regex(r#""([^"]*)""#)]
    DoubleString,
    #[regex(r"[0-9]+(\.[0-9]+)?")]
    Number,
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier,
    #[token("true")]
    True,
    #[token("false")]
    False,

    // Operators
    #[token("+")]
    Add,
    #[token("-")]
    Subtract,
    #[token("*")]
    Multiply,
    #[token("/")]
    Divide,
    #[token("%")]
    Modulo,
    #[token("**")]
    Power,
    #[token("+=")]
    AddResult,
    #[token("-=")]
    SubtractResult,
    #[token("*=")]
    MultiplyResult,
    #[token("/=")]
    DivideResult,
    #[token("%=")]
    ModuloResult,
    #[token(">=")]
    GreaterEqual,
    #[token("<=")]
    LessEqual,
    #[token(">")]
    GreaterThan,
    #[token("<")]
    LessThan,
    #[token("==")]
    Equal,
    #[token("~=")]
    NotEqual,
    #[token("!")]
    Not,
    #[token("&&")]
    And,
    #[token("||")]
    Or,
    #[token("λ")]
    LambdaSymbol,
    #[token("Σ")]
    Sumate,
    #[token("√")]
    SquareRoot,
    #[token("!")]
    Factorial,
    #[token("@")]
    At,

    // Separators
    #[token(",")]
    Comma,
    #[token(";")]
    Semicolon,
    #[token(":")]
    TypeColon,
    #[token("::")]
    TypeAssertion,
    #[token("(")]
    LeftParen,
    #[token(")")]
    RightParen,
    #[token("[")]
    LeftBracket,
    #[token("]")]
    RightBracket,
    #[token("{")]
    LeftBrace,
    #[token("}")]
    RightBrace,

    // Anything else
    #[error]
    Error,
}

pub struct Lexer<'a> {
    source: &'a str,
    lexer: logos::Lexer<'a, Token>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            lexer: Token::lexer(source),
            source,
        }
    }

    pub fn next(&mut self) -> Option<Token> {
        self.lexer.next()
    }

    pub fn slice(&self) -> &'a str {
        self.lexer.slice()
    }
}