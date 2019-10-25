#[macro_use]
mod macros;
mod ast;
mod lexer;
mod parser;
mod token;

use loki_errors::{
    pos::{CharPosition, Position, Span},
    FileId, Files, Reporter,
};

use std::collections::VecDeque;

pub type ParserResult<T> = Result<T, ()>;

use crate::ast::TokenKind;
use crate::token::Token;
pub struct Parser<'a> {
    /// The input string
    input: &'a str,
    /// The file id for the file being parsed
    file_id: FileId,
    /// The position for each char in the stream
    chars: CharPosition<'a>,
    /// The char ahead in the stream
    lookahead: Option<(Position, char)>,
    /// Tokens in the stream
    past_tokens: VecDeque<Span<Token>>,
    reporter: Reporter,
    /// The very first character
    start: Position,
    /// The last character
    end: Position,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str, reporter: Reporter, file_id: FileId) -> Self {
        let mut chars = CharPosition::new(input);
        let end = chars.pos;
        let mut past_tokens = VecDeque::new();
        let mut parser = Self {
            input,
            end,
            file_id,
            start: end,
            reporter,
            lookahead: chars.next(),
            past_tokens: VecDeque::new(),
            chars,
        };

        past_tokens.push_back(parser.next().unwrap());

        parser.past_tokens = past_tokens;

        parser
    }
}
