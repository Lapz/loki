use crate::ast::{TokenKind::*, *};
use crate::parser::{Parser, ParserResult};
use crate::T;

impl<'a> Parser<'a> {
    pub(crate) fn parse_type(&mut self) -> ParserResult<TypeRef> {
        match self.current() {
            IDENT | T![self] => self.parse_ident_type(),
            T!["["] => self.parse_array_type(),
            T!["("] => self.parse_paren_type(),
            T![fn] => self.parse_fn_type(),
            _ => {
                self.parser_error(
                    "Expected a type parameter",
                    format!(
                        "Expected a type parameter but instead found `{}`",
                        self.current_string()
                    ),
                );

                Err(())
            }
        }
    }

    fn parse_ident_type(&mut self) -> TypeRef {
        if self.matches(vec![IDENT, T![self]]) {
            self.bump();
        } else {
            self.parser_error(
                "Expected an identifier or `void`",
                format!(
                    "Expected an identifier or `void` but instead found `{}`",
                    self.current_string()
                ),
            )
        }

        if self.token_is_ahead(|t| t == T![<]) {
            self.parse_type_params(false);
        }

        TypeRef::IdentType
    }

    fn parse_array_type(&mut self) -> TypeRef {
        self.bump(); //Eat `[`

        self.parse_type();

        self.expect(T!["]"]);

        TypeRef::ArrayType
    }

    fn parse_paren_type(&mut self) -> TypeRef {
        self.bump(); //Eat `(`

        while !self.at(EOF) && !self.at(T![")"]) {
            self.parse_type();
            if !self.at(T![")"]) && !self.expected(T![,]) {
                break;
            }
        }

        self.expect(T![")"]);

        TypeRef::ParenType
    }

    fn parse_fn_type(&mut self) -> TypeRef {
        self.bump(); //Eat `fn`

        self.expect(T!["("]);

        while !self.at(EOF) && !self.at(T![")"]) {
            self.parse_type();
            if !self.at(T![")"]) && !self.expected(T![,]) {
                break;
            }
        }

        self.expect(T![")"]);

        if self.token_is_ahead(|t| t == T![->]) {
            self.bump();

            self.parse_type();
        }

        TypeRef::FnType
    }
}
// Literal Table
// Var Table
// Expr Id
