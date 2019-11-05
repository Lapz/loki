mod pratt;

mod source_file;
mod type_alias;
mod type_params;
mod types;
mod visibility;
use crate::ast::{
    self,
    TokenKind::{self, *},
};
use crate::{Parser, ParserResult};
use loki_errors::pos::Position;
impl<'a> Parser<'a> {
    pub fn parse(&mut self) -> ParserResult<()> {
        Ok(())
    }
    pub(crate) fn token_is_ahead<F>(&self, mut check: F) -> bool
    where
        F: FnMut(TokenKind) -> bool,
    {
        self.past_tokens
            .back()
            .as_ref()
            .map_or(false, |token| check(token.value.kind))
    }

    pub(crate) fn peek_token(&mut self) -> TokenKind {
        self.past_tokens
            .back()
            .map_or(TokenKind::EOF, |token| token.value.kind)
    }

    pub(crate) fn current(&self) -> TokenKind {
        self.past_tokens
            .back()
            .map(|token| token.value.kind)
            .unwrap_or(EOF)
    }

    pub(crate) fn at(&self, check: TokenKind) -> bool {
        self.current() == check
    }

    pub fn bump(&mut self) {
        if self.at(TokenKind::EOF) {
            return;
        }
        let token = self.next();
        match token {
            Some(token) => {
                // let text = &self.input[token.start.absolute as usize
                //     ..token.start.absolute as usize + token.value.len as usize];
                // self.builder.token(token.value.kind.into(), text.into());
                self.past_tokens.push_front(token);
                // self.lookahead = self.iter.next()
            }
            None => {}
        }
    }

    pub(crate) fn matches(&self, kind: Vec<TokenKind>) -> bool {
        for kind in kind {
            if kind == self.current() {
                return true;
            }
        }
        false
    }

    pub(crate) fn expect(&mut self, expected: TokenKind) {
        if self.token_is_ahead(|t| t == expected) {
            self.bump();
        } else {
            self.parser_error(
                format!("Expected `{}`", expected.text()),
                format!(
                    "Expected `{}` but instead found `{}`",
                    expected.text(),
                    self.current().text()
                ),
            );
        }
    }

    pub(crate) fn expect_with_msg<T: Into<String>>(&mut self, expected: TokenKind, msg: T) {
        if self.token_is_ahead(|t| t == expected) {
            self.bump();
        } else {
            self.parser_error(format!("Expected `{}`", expected.text()), msg);
        }
    }

    pub(crate) fn expected(&mut self, expected: TokenKind) -> bool {
        if self.token_is_ahead(|t| t == expected) {
            self.bump();
            true
        } else {
            self.parser_error(
                format!("Expected `{}`", expected.text()),
                format!(
                    "Expected `{}` but instead found `{}`",
                    expected.text(),
                    self.current().text()
                ),
            );
            false
        }
    }

    fn recover(&mut self) {
        if self.at(T!["{"]) || self.at(T!["}"]) {
            return;
        }

        self.bump();
    }

    fn recover_until(&mut self, token: TokenKind) {
        while !self.lookahead.is_none() && !self.at(token) {
            self.bump();
        }

        self.bump(); // eat the token as well
    }

    pub(crate) fn parser_error(
        &mut self,
        message: impl Into<String>,
        additional_info: impl Into<String>,
    ) {
        self.recover();

        self.reporter
            .error(self.file_id, message, additional_info, self.current_span());
    }

    fn current_span(&self) -> (Position, Position) {
        self.past_tokens
            .back()
            .map(|token| (token.start, token.end))
            .unwrap_or_else(|| {
                let token = self.past_tokens.front().unwrap();
                (token.start, token.end)
            })
    }

    fn current_string(&self) -> &str {
        self.past_tokens
            .back()
            .map(|token| {
                &self.input[token.start.absolute as usize
                    ..token.start.absolute as usize + token.value.len as usize]
            })
            .unwrap_or("")
    }

    pub(crate) fn ident(&mut self) -> ast::Name {
        self.expect(IDENT);
        unimplemented!()
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn peek_works() {
        let mut files = loki_errors::Files::new();
        let input = "fn foo() {}";
        let file_id = files.add("testing", input);

        let reporter = loki_errors::Reporter::new(files);
        let mut parser = crate::Parser::new(input, reporter, file_id);

        assert_eq!(parser.peek_token(), crate::TokenKind::FN_KW);
    }

    #[test]
    fn is_ahead_works() {
        let mut files = loki_errors::Files::new();
        let input = "fn foo() {}";
        let file_id = files.add("testing", input);

        let reporter = loki_errors::Reporter::new(files);
        let parser = crate::Parser::new(input, reporter, file_id);

        assert_eq!(parser.token_is_ahead(|t| t == T![fn]), true);
    }

    #[test]
    fn current_works() {
        let mut files = loki_errors::Files::new();
        let input = "fn foo() {}";
        let file_id = files.add("testing", input);

        let reporter = loki_errors::Reporter::new(files);
        let parser = crate::Parser::new(input, reporter, file_id);
        assert_eq!(parser.current(), crate::TokenKind::FN_KW);
    }

    #[test]
    fn at_works() {
        let mut files = loki_errors::Files::new();
        let input = "fn foo() {}";
        let file_id = files.add("testing", input);

        let reporter = loki_errors::Reporter::new(files);
        let parser = crate::Parser::new(input, reporter, file_id);
        assert_eq!(parser.current(), crate::TokenKind::FN_KW);
    }

    #[test]
    fn bump_works() {
        let mut files = loki_errors::Files::new();
        let input = "fn foo() {}";
        let file_id = files.add("testing", input);

        let reporter = loki_errors::Reporter::new(files);
        let mut parser = crate::Parser::new(input, reporter, file_id);
        parser.bump();
        assert_eq!(parser.current(), crate::TokenKind::IDENT);
    }
}
