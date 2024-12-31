// Copyright 2024 Jelly Terra
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0
// that can be found in the LICENSE file and https://mozilla.org/MPL/2.0/.

use std::num::ParseIntError;

pub struct Ident {
    pub lit: String,
}

pub struct Nat {
    pub val: Result<u128, ParseIntError>,
}

// Types

pub enum Type {
    Nat,
    Ident(Ident),
}

// Tokens

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

// Expressions

pub struct IdentExpr {
    pub ident: Ident,
}

pub struct TupleExpr {
    pub exprs: Vec<Expr>,
}

pub struct UnaryOperatorExpr {
    pub op: UnaryOperator,
    pub expr: Expr,
}

pub struct BinaryOperatorExpr {
    pub op: BinaryOperator,
    pub left: Expr,
    pub right: Expr,
}

pub struct Case {
    pub cond: Expr,
    pub expr: Expr,
}

pub struct ForeachExpr {
    pub item: Ident,
    pub collection: Expr,
}

pub struct MatchExpr {
    pub expr: Expr,
    pub patterns: Vec<Case>,
}

pub struct ApplyExpr {
    pub func: Expr,
    pub params: TupleExpr,
}

pub enum Expr {
    Ident(Box<IdentExpr>),
    Unary(Box<UnaryOperatorExpr>),
    Binary(Box<BinaryOperatorExpr>),
    Match(Box<MatchExpr>),
    Tuple(Box<TupleExpr>),
    Apply(Box<ApplyExpr>),
}

// Statements

pub struct LetDecl {
    pub ident: Ident,
    pub expr: Expr,
}

pub struct VarDecl {
    pub ident: Ident,
    pub expr: Expr,
}

pub struct FuncDecl {
    pub params: Vec<Type>,
    pub expr: Expr,
}

pub enum Decl {
    Let(LetDecl),
    Var(VarDecl),
    Func(FuncDecl),
}
