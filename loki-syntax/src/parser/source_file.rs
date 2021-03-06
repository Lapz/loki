use crate::ast::{TokenKind::*, *};
use crate::parser::Parser;
use crate::T;

impl<'a> Parser<'a> {
    pub(crate) fn parse_program(&mut self) -> SourceFile {
        while !self.at(EOF) && !self.at(ERROR) {
            // println!("{:?}", self.past_tokens);
            let has_visibility = self.has_visibility();

            if has_visibility {
                match self.peek_token() {
                    T![type] => self.parse_type_alias(true),
                    T![fn] => unimplemented!(),
                    T![enum] => unimplemented!(),
                    T![class] => unimplemented!(),
                    _ => {
                        self.recover();
                    }
                }
            } else {
                match self.current() {
                    T![type] => self.parse_type_alias(false),
                    T![fn] => unimplemented!(),
                    T![enum] => unimplemented!(),
                    T![class] => unimplemented!(),
                    _ => {
                        self.recover();
                    }
                }
            }

            self.bump();
        }

        SourceFile {
            functions: Vec::new(),
        }
    }

    pub(crate) fn has_visibility(&self) -> bool {
        match self.current() {
            T![export] => true,
            e => {
                println!("{:?}", e);
                false
            }
        }
    }
}
