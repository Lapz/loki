use crate::ast;
use crate::parser::Parser;
use crate::T;

use crate::{Span, Token, TokenKind::*};

impl<'a> Parser<'a> {
    pub(crate) fn parse_visibility(&mut self) -> ast::Visibility {
        self.expect(T![export]);
        ast::Visibility::Public
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn it_works() {
        let mut files = loki_errors::Files::new();
        let input = "export fn foo() {}";
        let file_id = files.add("testing", input);

        let reporter = loki_errors::Reporter::new(files);
        let mut parser = crate::Parser::new(input, reporter, file_id);
        parser.parse_visibility();
    }
}
