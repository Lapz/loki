use crate::ast::TokenKind;
#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub len: u32,
}

impl Token {
    pub const fn new(kind: TokenKind, len: u32) -> Self {
        Token { kind, len }
    }
}
