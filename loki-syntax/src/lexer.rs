use crate::ast::TokenKind;
use crate::token::Token;
use crate::{Parser, ParserResult};
use loki_errors::{
    pos::{Position, Span},
    Files, Reporter,
};

impl<'a> Parser<'a> {
    /// Function that gets the next token and puts it within the Vec whilst returning the
    /// last token
    pub(crate) fn next(&mut self) -> Option<Span<Token>> {
        let token = self.next_token();
        self.past_tokens.push_back(token);

        match self.past_tokens.pop_front() {
            Some(token) => Some(token),
            None => None,
        }
    }

    pub(crate) fn error(
        &mut self,
        message: impl Into<String>,
        additional_info: impl Into<String>,
        span: (Position, Position),
    ) {
        self.reporter
            .error(self.file_id, message, additional_info, span)
    }

    /// Advances the input return the current position and the char we are at
    pub(crate) fn advance(&mut self) -> Option<(Position, char)> {
        match self.lookahead {
            Some((pos, ch)) => {
                self.start = pos;
                self.end = self.end.shift(ch);
                self.lookahead = self.chars.next();
                Some((pos, ch))
            }

            None => None,
        }
    }

    /// Slice the input return the string contained
    /// between the supplied position
    pub(crate) fn slice(&self, start: Position, end: Position) -> &'a str {
        &self.input[start.absolute as usize..end.absolute as usize]
    }

    /// Advance the input whilst the given function evaluates
    /// to true
    pub(crate) fn take_whilst<F>(
        &mut self,
        start: Position,
        mut terminate: F,
    ) -> (Position, &'a str)
    where
        F: FnMut(char) -> bool,
    {
        while let Some((end, ch)) = self.lookahead {
            if !terminate(ch) {
                return (end, self.slice(start, end));
            }
            self.advance();
        }

        (self.end, self.slice(start, self.end))
    }

    /// Lookahead at the input
    pub(crate) fn peek<F>(&self, mut check: F) -> bool
    where
        F: FnMut(char) -> bool,
    {
        self.lookahead.map_or(false, |(_, ch)| check(ch))
    }

    pub(crate) fn next_token(&mut self) -> Span<Token> {
        while let Some((start, ch)) = self.advance() {
            return match ch {
                '.' => span(TokenKind::DOT, start),
                '?' => span(TokenKind::QUESTION, start),
                ';' => span(TokenKind::SEMI, start),
                '{' => span(TokenKind::L_CURLY, start),
                '}' => span(TokenKind::R_CURLY, start),
                '[' => span(TokenKind::L_BRACK, start),
                ']' => span(TokenKind::R_BRACK, start),
                '(' => span(TokenKind::L_PAREN, start),
                ')' => span(TokenKind::R_PAREN, start),
                ',' => span(TokenKind::COMMA, start),
                '_' => span(TokenKind::UNDERSCORE, start),
                '|' => span(TokenKind::PIPE, start),
                '^' => span(TokenKind::EXPONENTIAL, start),
                '%' => span(TokenKind::PERCENT, start),
                ':' => {
                    if self.peek(|ch| ch == ':') {
                        self.advance();

                        spans(TokenKind::COLON_COLON, start, start.shift(':'))
                    } else {
                        span(TokenKind::COLON, start)
                    }
                }
                '!' => {
                    if self.peek(|ch| ch == '=') {
                        self.advance();
                        spans(TokenKind::NEQ, start, start.shift(':'))
                    } else {
                        span(TokenKind::EXCL, start)
                    }
                }

                '=' => {
                    if self.peek(|ch| ch == '=') {
                        self.advance();
                        spans(TokenKind::EQEQ, start, start.shift('='))
                    } else {
                        span(TokenKind::EQ, start)
                    }
                }

                '>' => {
                    if self.peek(|ch| ch == '=') {
                        self.advance();
                        spans(TokenKind::GTEQ, start, start.shift(':'))
                    } else {
                        span(TokenKind::R_ANGLE, start)
                    }
                }

                '<' => {
                    if self.peek(|ch| ch == '=') {
                        self.advance();
                        spans(TokenKind::LTEQ, start, start.shift(':'))
                    } else {
                        span(TokenKind::L_ANGLE, start)
                    }
                }

                '+' => {
                    if self.peek(|ch| ch == '=') {
                        self.advance();
                        spans(TokenKind::PLUSEQ, start, start.shift('='))
                    } else {
                        span(TokenKind::PLUS, start)
                    }
                }

                '-' => {
                    if self.peek(|ch| ch == '=') {
                        self.advance();
                        spans(TokenKind::MINUSEQ, start, start.shift('='))
                    } else if self.peek(|ch| ch == '>') {
                        self.advance();
                        spans(TokenKind::FRETURN, start, start.shift('>'))
                    } else {
                        span(TokenKind::MINUS, start)
                    }
                }

                '*' => {
                    if self.peek(|ch| ch == '=') {
                        self.advance();

                        spans(TokenKind::STAREQ, start, start.shift('='))
                    } else {
                        span(TokenKind::STAR, start)
                    }
                }

                '"' => self.string_literal(start),
                '/' => {
                    if self.peek(|ch| ch == '=') {
                        self.advance();

                        spans(TokenKind::SLASHEQ, start, start.shift('='))
                    } else if self.peek(|ch| ch == '/') {
                        self.advance();
                        self.line_comment(start)
                    } else if self.peek(|ch| ch == '*') {
                        self.block_comment(start)
                    } else {
                        span(TokenKind::SLASH, start)
                    }
                }

                ch if ch.is_numeric() => self.number(start),
                ch if is_letter_ch(ch) => self.identifier(start),
                ch if ch.is_whitespace() => continue,
                ch => {
                    self.error(
                        "Unknown character",
                        format!("Unknown character `{}`", ch),
                        (start, start),
                    );
                    spans(TokenKind::ERROR, start, start.shift(ch))
                }
            };
        }

        span(TokenKind::EOF, self.end)
    }

    /// Handle a line comment
    pub(crate) fn line_comment(&mut self, start: Position) -> Span<Token> {
        let (end, _) = self.take_whilst(start, |ch| ch != '\n');

        spans(TokenKind::COMMENT, start, end)
    }

    /// Handle  a block comment
    pub(crate) fn block_comment(&mut self, start: Position) -> Span<Token> {
        self.advance(); // Eats the '*'

        let mut depth = 1usize;

        while let Some((_, c)) = self.advance() {
            match c {
                '/' if self.peek(|ch| ch == '*') => {
                    self.advance();
                    depth += 1;
                }

                '*' if self.peek(|ch| ch == '/') => {
                    self.advance();
                    depth -= 1;

                    if depth == 0 {
                        break;
                    }
                }

                _ => (),
            }
        }

        if depth == 0 {
            spans(TokenKind::COMMENT, start, self.end)
        } else {
            spans(TokenKind::ERROR, start, self.end)
        }
    }

    fn string_literal(&mut self, start: Position) -> Span<Token> {
        while let Some((next, ch)) = self.advance() {
            match ch {
                '"' => {
                    let end = next.shift(ch);

                    return spans(TokenKind::STRING, start, end);
                }

                _ => (),
            }
        }

        spans(TokenKind::ERROR, start, self.end)
    }

    /// Handles any identifier.
    // New key words should be added to the look_up_identifier function
    pub(crate) fn identifier(&mut self, start: Position) -> Span<Token> {
        let (end, ident) = self.take_whilst(start, is_letter_ch);
        spans(look_up_identifier(ident), start, end)
    }

    /// Handles number,both int's and floats
    pub(crate) fn number(&mut self, start: Position) -> Span<Token> {
        let (end, _) = self.take_whilst(start, char::is_numeric);

        match self.lookahead {
            Some((end, '.')) => {
                self.advance();

                if self.peek(char::is_numeric) {
                    let (end, _) = self.take_whilst(start, char::is_numeric);

                    spans(TokenKind::FLOAT_NUMBER, start, end)
                } else {
                    spans(TokenKind::ERROR, start, end)
                }
            }

            Some((end, ch)) if ch.is_alphabetic() => spans(TokenKind::ERROR, start, end),

            Some((end, _)) => spans(TokenKind::INT_NUMBER, start, end),
            None => spans(TokenKind::INT_NUMBER, start, end),
        }
    }
}

#[inline]
fn span(token: TokenKind, start: Position) -> Span<Token> {
    Span::new(Token::new(token, 1), start, start)
}

#[inline]
fn spans(token: TokenKind, start: Position, end: Position) -> Span<Token> {
    Span::new(Token::new(token, end.absolute - start.absolute), start, end)
}

#[inline]
fn is_letter_ch(ch: char) -> bool {
    ch.is_alphanumeric() || ch == '_'
}

#[inline]
fn look_up_identifier(id: &str) -> TokenKind {
    match id {
        // Class
        "class" => TokenKind::CLASS_KW,
        "extends" => TokenKind::EXTENDS_KW,
        "print" => TokenKind::PRINT_KW,
        "type" => TokenKind::TYPE_KW,
        "as" => TokenKind::AS_KW,
        "match" => TokenKind::MATCH_KW,
        "enum" => TokenKind::ENUM_KW,
        "export" => TokenKind::EXPORT_KW,
        "self" => TokenKind::SELF_KW,
        // Functions and vars
        "fn" => TokenKind::FN_KW,
        "let" => TokenKind::LET_KW,
        // Control Flow
        "if" => TokenKind::IF_KW,
        "else" => TokenKind::ELSE_KW,
        "for" => TokenKind::FOR_KW,
        "while" => TokenKind::WHILE_KW,
        "return" => TokenKind::RETURN_KW,
        "break" => TokenKind::BREAK_KW,
        "continue" => TokenKind::CONTINUE_KW,
        "do" => TokenKind::DO_KW,

        // Booleans
        "true" => TokenKind::TRUE_KW,
        "false" => TokenKind::FALSE_KW,
        "or" => TokenKind::OR_KW,
        "and" => TokenKind::AND_KW,
        "nil" => TokenKind::NIL_KW,
        _ => TokenKind::IDENT,
    }
}
