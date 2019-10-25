mod ast;
mod lexer;
mod parser;
mod token;

use loki_errors::{
    pos::{CharPosition, Position, Span},
    Files, Reporter,
};

pub type ParserResult<T> = Result<T, ()>;

use crate::token::Token;

pub struct Parser<'a> {
    /// The input string
    input: &'a str,
    /// The position for each char in the stream
    chars: CharPosition<'a>,
    /// The char ahead in the stream
    lookahead: Option<(Position, char)>,
    /// Tokens in the stream
    past_tokens: Vec<Span<Token>>,
    reporter: Reporter,
    /// The very first character
    start: Position,
    /// The last character
    end: Position,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str, reporter: Reporter) -> Self {
        let mut chars = CharPosition::new(input);
        let end = chars.pos;

        Self {
            input,
            end,
            start: end,
            reporter,
            lookahead: chars.next(),
            past_tokens: Vec::new(),
            chars,
        }
    }
}
