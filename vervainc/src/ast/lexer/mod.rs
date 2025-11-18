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

use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token {
    // Keywords
    #[token("set")]
    Set,
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
    #[regex(r"~~[^\n]*")]
    Comment,
    #[regex(r#""([^"]*)""#)]
    DoubleString,
    #[regex(r"[0-9]+", |lex| lex.slice().parse::<u32>().unwrap())]
    Number(u32),
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Identifier(String),
    #[token("true")]
    True,
    #[token("false")]
    False,

    // Types
    #[token("Number")]
    NumberType,
    #[token("String")]
    StringType,

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
    #[token("=")]
    Equal,
    #[token("==")]
    StrictEqual,
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
}

pub fn lex(source: &str) -> Vec<Token> {
    Token::lexer(source)
        .filter_map(|res| res.ok())
        .collect()
}

#[cfg(test)]
mod test {
    use crate::ast::lexer::{
        lex, 
        Token
    };
    #[test]
    fn attempt_type_asserted_var() {
        let src = "set thing = 10 :: Number";
        assert_eq!(
            lex(&src),
            vec![
                Token::Set,
                Token::Identifier("thing".to_string()),
                Token::Equal,
                Token::Number(10),
                Token::TypeAssertion,
                Token::NumberType
            ])
    }
    #[test]
    fn attempt_operation() {
        let src = "2 + 2";
        assert_eq!(
            lex(&src),
            vec![
                Token::Number(2),
                Token::Add,
                Token::Number(2),
                ])
    }
    #[test]
    fn attempt_set() {
        let src = "set";
        assert_eq!(
            lex(&src),
            vec![Token::Set,])
    }
}