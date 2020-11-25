use crate::ast::{TokenKind::*, *};
use crate::parser::Parser;
use crate::T;

impl<'a> Parser<'a> {
    pub(crate) fn parse_type_params(&mut self, allow_types: bool) -> Vec<TypeParam> {
        let mut type_param_list: Vec<TypeParam> = Vec::new();

        self.bump();

        while !self.at(EOF) && !self.at(T![>]) {
            if allow_types {
                // self.parse_type();
            } else {
                self.type_param();
            }

            if !self.at(T![>]) && !self.expected(T![,]) {
                break;
            }
        }

        self.expect_with_msg(T![>], "Expected `>` to close type params");

        type_param_list
    }

    fn type_param(&mut self) -> TypeParam {
        let name = self.ident();

        TypeParam { name }
    }
}
