// Copyright 2025 Jelly Terra <jellyterra@symboltics.com>
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0
// that can be found in the LICENSE file and https://mozilla.org/MPL/2.0/.

use std::num::ParseIntError;

// Atoms

pub struct Ident {
    pub lit: String,
}

pub struct Nat {
    pub val: Result<u128, ParseIntError>,
}

pub enum UnaryOperator {
    Invert
}

pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    And,
    Or,
}

// Types

pub enum RecordTypeKind {
    Struct,
    Tuple,
}

pub struct RecordType {
    pub kind: RecordTypeKind,
    pub fields: Vec<Type>,
    pub idents: Vec<Ident>,
}

pub struct UnionType {
    pub variants: Vec<Type>,
    pub idents: Vec<Ident>,
}

pub struct FuncType {
    pub params: RecordType,
    pub result: Type,
}

pub enum Type {
    Record(Box<RecordType>),
    Union(Box<UnionType>),
    Func(Box<FuncType>),
    Ident(Ident),
}

// Expressions

pub struct TupleExpr {
    pub exprs: Vec<Expr>,
}

pub struct ApplyExpr {
    pub func: Expr,
    pub params: RecordType,
}

pub struct BlockExpr {
    pub stmts: Vec<Stmt>,
    pub expr: Option<Expr>,
}

pub enum Expr {
    Tuple(Box<TupleExpr>),
    Apply(Box<ApplyExpr>),
    Block(Box<BlockExpr>),
    Ident(Ident),
}

// Declarations

pub struct LetDecl {
    pub ident: Ident,
    pub expr: Expr,
}

pub struct VarDecl {
    pub ident: Ident,
    pub expr: Expr,
}

pub struct FuncDecl {
    pub ident: Ident,
    pub params: RecordType,
    pub result: Type,
    pub block: BlockExpr,
}

pub struct TypeAliasDecl {
    pub origin: Type,
    pub alias: Ident,
}

pub enum Decl {
    Let(LetDecl),
    Var(VarDecl),
    Func(FuncDecl),
    TypeAlias(TypeAliasDecl),
}

// Statement

pub enum Stmt {
    Decl(Decl),
    Expr(Expr),
}
