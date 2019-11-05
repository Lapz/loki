use crate::ast::{TokenKind::*, *};
use crate::parser::Parser;
use crate::T;

impl<'a> Parser<'a> {
    pub(crate) fn parse_type_alias(&mut self, has_visibility: bool) {
        let visibility = if has_visibility {
            self.parse_visibility()
        } else {
            Visibility::Private
        };

        self.expect(T![type]);

        let ident = self.ident();

        let type_params = if self.token_is_ahead(|t| t == L_ANGLE) {
            self.parse_type_params(true)
        } else {
            Vec::new()
        };

        self.expect(EQ);

        // self.parse_type();

        self.expect(T![;])
    }
}
