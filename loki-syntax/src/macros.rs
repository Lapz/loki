#[macro_export]
macro_rules! T {
    (while) => {
        $crate::TokenKind::WHILE_KW
    };
    (for) => {
        $crate::TokenKind::FOR_KW
    };
    (break) => {
        $crate::TokenKind::BREAK_KW
    };
    (continue) => {
        $crate::TokenKind::CONTINUE_KW
    };
    (do) => {
        $crate::TokenKind::DO_KW
    };
    (return) => {
        $crate::TokenKind::RETURN_KW
    };
    (print) => {
        $crate::TokenKind::PRINT
    };
    (class) => {
        $crate::TokenKind::CLASS_KW
    };
    (true) => {
        $crate::TokenKind::TRUE_KW
    };
    (false) => {
        $crate::TokenKind::FALSE_KW
    };
    (self) => {
        $crate::TokenKind::SELF_KW
    };
    (enum) => {
        $crate::TokenKind::ENUM_KW
    };

    (extends) => {
        $crate::TokenKind::EXTENDS
    };
    (export) => {
        $crate::TokenKind::EXPORT_KW
    };
    (match) => {
        $crate::TokenKind::MATCH
    };
    (else) => {
        $crate::TokenKind::ELSE_KW
    };
    (if) => {
        $crate::TokenKind::IF_KW
    };
    (let) => {
        $crate::TokenKind::LET_KW
    };
    (type) => {
        $crate::TokenKind::TYPE_KW
    };
    (::) => {
        $crate::TokenKind::COLON_COLON
    };
    ("{") => {
        $crate::TokenKind::L_CURLY
    };

    (,) => {
        $crate::TokenKind::COMMA
    };
    ("}") => {
        $crate::TokenKind::R_CURLY
    };
    ("[") => {
        $crate::TokenKind::L_BRACK
    };
    ("]") => {
        $crate::TokenKind::R_BRACK
    };
    ("(") => {
        $crate::TokenKind::L_PAREN
    };
    (")") => {
        $crate::TokenKind::R_PAREN
    };

    (=) => {
        $crate::TokenKind::EQ
    };
    (:) => {
        $crate::TokenKind::COLON
    };
    (.) => {
        $crate::TokenKind::DOT
    };
    (;) => {
        $crate::TokenKind::SEMI
    };
    (_) => {
        $crate::TokenKind::UNDERSCORE
    };
    (+=) => {
        $crate::TokenKind::PLUSEQ
    };
    (!) => {
        $crate::TokenKind::EXCL
    };
    ("//") => {
        $crate::TokenKind::COMMENT
    };

    (+) => {
        $crate::TokenKind::PLUS
    };
    (-) => {
        $crate::TokenKind::MINUS
    };
    (*) => {
        $crate::TokenKind::STAR
    };
    (/) => {
        $crate::TokenKind::SLASH
    };

    (&&) => {
        $crate::TokenKind::AMPAMP
    };
    (==) => {
        $crate::TokenKind::EQEQ
    };
    (=>) => {
        $crate::TokenKind::FAT_ARROW
    };
    (<) => {
        $crate::TokenKind::L_ANGLE
    };
    (<=) => {
        $crate::TokenKind::LTEQ
    };
    (>) => {
        $crate::TokenKind::R_ANGLE
    };
    (>=) => {
        $crate::TokenKind::GTEQ
    };
    (!=) => {
        $crate::TokenKind::NEQ
    };

    (-=) => {
        $crate::TokenKind::MINUSEQ
    };
    (*=) => {
        $crate::TokenKind::STAREQ
    };
    (/=) => {
        $crate::TokenKind::SLASHEQ
    };
    (||) => {
        $crate::TokenKind::PIPEPIPE
    };

    (->) => {
        $crate::TokenKind::FRETURN
    };

    (fn) => {
        $crate::TokenKind::FN_KW
    };

    (nil) => {
        $crate::TokenKind::NIL_KW
    };
}

// #[macro_export]
// macro_rules! test_parser {
//     ($f_name:ident,$test:expr) => {
//         #[test]
//         fn $f_name() {
//             let parser_output = $crate::utils::parse($test).parse_program();
//             insta::assert_snapshot_matches!($crate::utils::dump_debug(&parser_output));
//         }
//     };
// }
