use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    // Keywords
    #[token("function")]
    Function,
    #[token("let")]
    Let,
    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("scoop")]
    Scoop,
    #[token("show.text")]
    ShowText,

    // Symbols and delimiters
    #[token("=>")]
    Arrow,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,

    // Literals
    #[regex(r#""([^"\\]|\\.)*""#)]
    StringLiteral,
    #[regex(r"[0-9]+")]
    Number,

    // Identifiers
    #[regex(r"[A-Za-z_][A-Za-z0-9_]*")]
    Identifier,

    // Skip comments and whitespace
    #[regex(r"//[^\n]*", logos::skip)]
    Comment,
    #[regex(r"\s+", logos::skip)]
    Whitespace,

    // Catch any unrecognized characters
    #[error]
    Error,
}
