use std::{fmt, ops::{Deref, DerefMut}};
use logos::{Logos, Lexer};


/// Callback for bool literal tokens
fn token_bool_lit<'a>(lex: &mut Lexer<'a, Token<'a>>) -> Option<bool> {
    let slice: &'a str = lex.slice();
    let n = slice.parse::<bool>();
    match n {
        Result::Ok(b) => Some(b),
        Result::Err(_err) => panic!("Expected bool literal")
    }
}

/// Callback for int literal tokens
fn token_int_lit<'a>(lex: &mut Lexer<'a, Token<'a>>) -> Option<i64> {
    let slice: &'a str = lex.slice();
    let sign: usize = if slice.starts_with('~') { 1 } else { 0 };
    let slice_abs = &slice[sign..];
    let n_abs = slice_abs.parse::<i64>();
    match n_abs {
        // Arithmetic magic to encode the negative sign
        Result::Ok(i) => Some(i * (1 - 2 * (sign as i64))),
        Result::Err(_err) => panic!("Expected integer literal")
    }
}


// Tokens
#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token<'a> {

    // Punctuations
    #[token(".")] Dot,
    #[token(",")] Comma,
    #[token(":")] Colon,
    #[token("=")] Equal,
    #[token("(")] LParen,
    #[token(")")] RParen,
    #[token("[")] LBrack,
    #[token("]")] RBrack,
    #[token("|")] Bar,
    #[token("->")] Arrow,
    #[token("=>")] DoubleArrow,

    /// Precedence 7 as multiplication
    #[token("*")]
    Mul,

    // Different precedences in the token enum level is
    // necessary for the parser to disambiguate
    /// Precedence 6
    #[regex(r"\-|\+", |lex| lex.slice())]
    Infix6(&'a str),

    /// Precedence 4
    #[regex(r"<|>|==|!=", |lex| lex.slice())]
    Infix4(&'a str),

    /// Precedence 3
    #[regex(r"(\&\&)|(\|\|)", |lex| lex.slice())]
    Infix3(&'a str),

    /// Identifiers
    #[regex(r"[a-z][0-9a-zA-Z_]*", |lex| lex.slice())]
    ExpId(&'a str),

    /// Type Identifiers
    #[regex(r"[A-Z][0-9a-zA-Z_]*", |lex| lex.slice())]
    TypId(&'a str),

    /// Integer literals
    #[regex(r"\~?[0-9]+", token_int_lit)]
    IntLit(i64),

    /// Bool literals
    #[regex(r"true|false", token_bool_lit)]
    BoolLit(bool),

    /// Unit literal aka null
    #[token("null")]
    UnitLit,

    /***** Keywords *****/
    #[token("if")] If,
    #[token("then")] Then,
    #[token("else")] Else,
    #[token("let")] Let,
    #[token("in")] In,
    #[token("match")] Match,
    #[token("with")] With,
    #[token("end")] End,
    #[regex("???|any")] Any,
    #[regex("??|lambda")] Lambda,
    #[regex("??|forall")] Forall,

    // Built-in types
    #[token("Int")] TInt,
    #[token("Bool")] TBool,
    #[token("Unit")] TUnit,

    // Logos requires one token variant to handle errors,
    // We can also use this variant to define whitespace,
    // or any other matches we wish to skip.
    #[error]
    #[regex("([ \t\n\r]+)|(/\\*[^*]*\\*+(?:[^/*][^*]*\\*+)*/)", logos::skip)]
    Error,
}

impl<'source> fmt::Display for Token<'source> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}

pub type LexicalError = usize;

// Wraps the lexer in a self-defined struct. Otherwise
// Rust complains about implementing Iterator trait for
// Lexer<Token> struct, neither of which I created.
pub struct LexerWrap<'a> { lexer: Lexer<'a, Token<'a>> }

impl<'a> LexerWrap<'a> {
    pub fn new (input: &'a str) -> Self {
        LexerWrap { lexer: Token::lexer(input) }
    }
}

// Deref and DerefMut are implemented so one can call Lexer
// functions on LexerWrap
impl<'a> Deref for LexerWrap<'a> {
    type Target = Lexer<'a, Token<'a>>;
    fn deref(&self) -> &Self::Target {
        &self.lexer
    }
}

impl<'a> DerefMut for LexerWrap<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.lexer
    }
}

// Implements iterator for lalrpop Result type
impl<'a> Iterator for LexerWrap<'a> {
    type Item = Result<(usize, Token<'a>, usize), LexicalError>;
    fn next(&mut self) -> Option<Self::Item> {
        let token_opt = self.lexer.next();
        let span = self.span();
        token_opt.map(|token| Ok((span.start, token, span.end)))
    }
}
