use crate::{
    Span, Token,
    TokenKind::{self, *},
};
use std::fmt::Debug;

use crate::ast;
use crate::parser::Parser;
pub trait Rule {
    fn rule(&self) -> RuleToken;
}
pub trait PrefixParser {
    fn parse(&self, parser: &mut Parser);
}
pub trait InfixParser {
    fn parse(&self, parser: &mut Parser, lhs: Span<ast::Expr>);
    fn pred(&self) -> Precedence;
}

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq)]
pub enum Precedence {
    None,
    Assignment,
    Or,
    And,
    Equality,
    Comparison,
    Term,
    Factor,
    Unary,
    Call,
    Primary,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
pub enum RuleToken {
    LParen,
    Ident,
    LBracket,
    Minus,
    Plus,
    Slash,
    Star,
    Literal,
    None,
    Excl,
    Comparison,
    EqEq,
    This,
    AmpAmp,
    PipePipe,
}

impl Precedence {
    pub fn higher(&self) -> Precedence {
        match *self {
            Precedence::None | Precedence::Assignment => Precedence::Or,
            Precedence::Or => Precedence::And,
            Precedence::And => Precedence::Equality,
            Precedence::Equality => Precedence::Comparison,
            Precedence::Comparison => Precedence::Term,
            Precedence::Term => Precedence::Factor,
            Precedence::Factor => Precedence::Unary,
            Precedence::Unary => Precedence::Call,
            Precedence::Call => Precedence::Primary,
            Precedence::Primary => Precedence::Primary,
        }
    }
}

impl Rule for TokenKind {
    fn rule(&self) -> RuleToken {
        match self {
            INT_NUMBER | FLOAT_NUMBER | STRING | T![nil] | T![true] | T![false] => {
                RuleToken::Literal
            }
            IDENT => RuleToken::Ident,
            T![+] => RuleToken::Plus,
            T![!] => RuleToken::Excl,
            T![-] => RuleToken::Minus,
            T!["("] => RuleToken::LParen,
            T![!=] | T![<] | T![>] | T![<=] | T![>=] => RuleToken::Comparison,
            T![&&] => RuleToken::AmpAmp,
            T![||] => RuleToken::PipePipe,
            _ => RuleToken::None,
        }
    }
}
