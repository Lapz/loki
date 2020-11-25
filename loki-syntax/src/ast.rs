
// This file is automatically generated based on the file `./generated.rs.tera` when `cargo xtask ast` is run
// Do not edit manually

//! This module contains auto-generated Rust AST. Like `SyntaxNode`s, AST nodes
//! are generic over ownership: `X<'a>` things are `Copy` references, `XNode`
//! are Arc-based. You can switch between the two variants using `.owned` and
//! `.borrowed` functions. Most of the code works with borowed mode, and only
//! this mode has all AST accessors.
#![allow(bad_style, missing_docs, unreachable_pub)]
#![cfg_attr(rustfmt, rustfmt_skip)]


use TokenKind::*;
/// The kind of syntax node, e.g. `IDENT`, `USE_KW`, or `STRUCT_DEF`.
#[derive(Debug,Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum TokenKind {
    // Technical SyntaxKinds: they appear temporally during parsing,
    // but never end up in the final tree
    #[doc(hidden)]
    EOF,
    SEMI, // ;
    COMMA, // ,
    L_PAREN, // (
    R_PAREN, // )
    L_CURLY, // {
    R_CURLY, // }
    L_BRACK, // [
    R_BRACK, // ]
    L_ANGLE, // <
    R_ANGLE, // >
    AMP, // &
    PIPE, // |
    PLUS, // +
    STAR, // *
    SLASH, // /
    CARET, // ^
    PERCENT, // %
    UNDERSCORE, // _
    EXPONENTIAL, // ^
    DOT, // .
    QUESTION, // ?
    MINUS, // -
    FRETURN, // ->
    COLON, // :
    COLON_COLON, // ::
    EQ, // =
    EQEQ, // ==
    FAT_ARROW, // =>
    EXCL, // !
    NEQ, // !=
    LTEQ, // <=
    GTEQ, // >=
    PLUSEQ, // +=
    MINUSEQ, // -=
    STAREQ, // *=
    SLASHEQ, // /=
    AMPAMP, // &&
    PIPEPIPE, // ||
    CLASS_KW, // class
    EXTENDS_KW, // extends
    TYPE_KW, // type
    AS_KW, // as
    MATCH_KW, // match
    ENUM_KW, // enum
    FN_KW, // fn
    LET_KW, // let
    IF_KW, // if
    ELSE_KW, // else
    FOR_KW, // for
    WHILE_KW, // while
    RETURN_KW, // return
    BREAK_KW, // break
    CONTINUE_KW, // continue
    DO_KW, // do
    TRUE_KW, // true
    FALSE_KW, // false
    OR_KW, // or
    AND_KW, // and
    EXPORT_KW, // export
    IMPORT_KW, // import
    FROM_KW, // from
    PRINT_KW, // print
    NIL_KW, // nil
    SELF_KW, // self
    INT_NUMBER,
    FLOAT_NUMBER,
    CHAR,
    STRING,
    ERROR,
    IDENT,
    COMMENT,
    // Technical kind so that we can cast from u16 safely
    #[doc(hidden)]
    __LAST,
}



impl TokenKind {
    pub fn text(&self) -> &'static str {
        use TokenKind::*;
        match self {
            SEMI => ";",
            COMMA => ",",
            L_PAREN => "(",
            R_PAREN => ")",
            L_CURLY => "{",
            R_CURLY => "}",
            L_BRACK => "[",
            R_BRACK => "]",
            L_ANGLE => "<",
            R_ANGLE => ">",
            AMP => "&",
            PIPE => "|",
            PLUS => "+",
            STAR => "*",
            SLASH => "/",
            CARET => "^",
            PERCENT => "%",
            UNDERSCORE => "_",
            EXPONENTIAL => "^",
            DOT => ".",
            QUESTION => "?",
            MINUS => "-",
            FRETURN => "->",
            COLON => ":",
            COLON_COLON => "::",
            EQ => "=",
            EQEQ => "==",
            FAT_ARROW => "=>",
            EXCL => "!",
            NEQ => "!=",
            LTEQ => "<=",
            GTEQ => ">=",
            PLUSEQ => "+=",
            MINUSEQ => "-=",
            STAREQ => "*=",
            SLASHEQ => "/=",
            AMPAMP => "&&",
            PIPEPIPE => "||",
            CLASS_KW => "class",
            EXTENDS_KW => "extends",
            TYPE_KW => "type",
            AS_KW => "as",
            MATCH_KW => "match",
            ENUM_KW => "enum",
            FN_KW => "fn",
            LET_KW => "let",
            IF_KW => "if",
            ELSE_KW => "else",
            FOR_KW => "for",
            WHILE_KW => "while",
            RETURN_KW => "return",
            BREAK_KW => "break",
            CONTINUE_KW => "continue",
            DO_KW => "do",
            TRUE_KW => "true",
            FALSE_KW => "false",
            OR_KW => "or",
            AND_KW => "and",
            EXPORT_KW => "export",
            IMPORT_KW => "import",
            FROM_KW => "from",
            PRINT_KW => "print",
            NIL_KW => "nil",
            SELF_KW => "self",
            INT_NUMBER => "INT_NUMBER",
            FLOAT_NUMBER => "FLOAT_NUMBER",
            CHAR => "CHAR",
            STRING => "STRING",
            ERROR => "ERROR",
            IDENT => "IDENT",
            COMMENT => "COMMENT",
        
            _ => ""
        }
    }
}




    // ArgList
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArgList {

    pub args:Vec<Expr>,

}


    // ArrayExpr
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArrayExpr {

    pub exprs:Vec<Expr>,

}

// ArrayType
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArrayType;


// BinExpr
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BinExpr;


// BindPat
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BindPat;



    // Block
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Block {

    pub expr:Option<Expr>,

    pub statements:Vec<Stmt>,

}

// BlockExpr
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BlockExpr;


// BreakExpr
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BreakExpr;


// CallExpr
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CallExpr;


// CastExpr
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CastExpr;


// ClassDef
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ClassDef;


// ClassLit
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ClassLit;


// ClosureExpr
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ClosureExpr;


// Condition
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Condition;


// ContinueExpr
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ContinueExpr;


// EnumDef
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EnumDef;


// EnumVariant
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EnumVariant;



    // EnumVariantList
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EnumVariantList {

    pub variants:Vec<EnumVariant>,

}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expr {

    ArrayExpr,
    ParenExpr,
    ClosureExpr,
    IfExpr,
    ForExpr,
    WhileExpr,
    ContinueExpr,
    BreakExpr,
    BlockExpr,
    ReturnExpr,
    MatchExpr,
    ClassLit,
    CallExpr,
    IndexExpr,
    FieldExpr,
    CastExpr,
    PrefixExpr,
    BinExpr,
    Literal,
}


// ExprStmt
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExprStmt;


// ExternImportDef
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExternImportDef;


// FieldExpr
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FieldExpr;


// FnDef
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FnDef;


// FnType
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FnType;


// ForExpr
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ForExpr;


// IdentType
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IdentType;


// IfExpr
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IfExpr;


// IndexExpr
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IndexExpr;


// LetStmt
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LetStmt;


// Literal
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Literal;


// LiteralPat
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LiteralPat;



    // MatchArm
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MatchArm {

    pub expr:Option<Expr>,

    pub pats:Vec<Pat>,

}


    // MatchArmList
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MatchArmList {

    pub arms:Vec<MatchArm>,

}

// MatchExpr
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MatchExpr;


// Name
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Name;


// NameRef
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NameRef;


// NamedField
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NamedField;


// NamedFieldDef
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NamedFieldDef;



    // NamedFieldDefList
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NamedFieldDefList {

    pub fields:Vec<NamedFieldDef>,

}


    // NamedFieldList
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NamedFieldList {

    pub fields:Vec<NamedField>,

}

// Param
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Param;



    // ParamList
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParamList {

    pub params:Vec<Param>,

}

// ParenExpr
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParenExpr;


// ParenType
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParenType;


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Pat {

    BindPat,
    PlaceholderPat,
    TuplePat,
    LiteralPat,
}


// PlaceholderPat
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PlaceholderPat;


// PrefixExpr
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PrefixExpr;


// RetType
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RetType;


// ReturnExpr
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ReturnExpr;



    // SourceFile
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SourceFile {

    pub functions:Vec<FnDef>,

}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Stmt {

    ExprStmt,
    LetStmt,
}



    // TuplePat
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TuplePat {

    pub args:Vec<Pat>,

}


    // TypeAliasDef
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeAliasDef {

    pub type_params:Option<TypeParamList>,

    pub name:Name,

    pub ty:TypeRef,

}


    // TypeParam
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeParam {

    pub name:Name,

}


    // TypeParamList
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeParamList {

    pub type_params:Vec<TypeParam>,

}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TypeRef {

    ParenType,
    ArrayType,
    FnType,
    IdentType,
}


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Visibility {

    Public,
    Private,
}


// WhileExpr
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WhileExpr;
