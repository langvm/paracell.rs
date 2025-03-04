// Copyright 2025 Jelly Terra <jellyterra@symboltics.com>
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0
// that can be found in the LICENSE file and https://mozilla.org/MPL/2.0/.

use lalrpop_util::lalrpop_mod;
use util_macro::AsVariant;

lalrpop_mod!(pub grammar, "/flow/grammar.rs");

#[derive(Clone, Debug)]
pub struct Nat {
    pub val: u128,
}

#[derive(Clone, Debug)]
pub struct Ident {
    pub lit: String,
}

#[derive(Clone, Debug)]
pub enum UnaryOperator {
    Invert,
    Not,
}

#[derive(Clone, Debug)]
pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    And,
    Or,

    Pipe,
}

#[derive(Clone, Debug)]
pub struct RecordType {
    pub fields: Vec<TypedIdent>,
}

#[derive(Clone, Debug)]
pub struct UnionType {
    pub variants: Vec<TypedIdent>,
}

#[derive(Clone, Debug)]
pub struct FuncType {
    pub params: RecordType,
    pub result: Item,
}

#[derive(Clone, Debug)]
pub struct ApplyExpr {
    pub func: Item,
    pub params: Tuple,
}

#[derive(Clone, Debug)]
pub struct TypedIdent {
    pub ident: Ident,
    pub ty: Item,
}

#[derive(Clone, Debug)]
pub struct LetDecl {
    pub ident: Ident,
    pub expr: Item,
}

#[derive(Clone, Debug)]
pub struct VarDecl {
    pub ident: Ident,
    pub expr: Item,
}

#[derive(Clone, Debug)]
pub struct Func {
    pub params: Vec<TypedIdent>,
    pub result: Item,
    pub block: Block,
}

#[derive(Clone, Debug)]
pub struct TypeAliasDecl {
    pub ident: Ident,
    pub ty: Item,
}

#[derive(Clone, Debug)]
pub struct Tuple {
    pub elems: Vec<Item>,
}

#[derive(Clone, Debug)]
pub struct TypeTuple {
    pub elems: Vec<Item>,
}

#[derive(Clone, Debug)]
pub struct UnaryOpExpr {
    pub op: UnaryOperator,
    pub expr: Item,
}

#[derive(Clone, Debug)]
pub struct BinaryOpExpr {
    pub op: BinaryOperator,
    pub left: Item,
    pub right: Item,
}

#[derive(Clone, Debug)]
pub struct Pattern {
    pub pattern: Item,
    pub expr: Item,
}

#[derive(Clone, Debug)]
pub struct Match {
    pub expr: Item,
    pub patterns: Vec<Pattern>,
}

#[derive(Clone, Debug)]
pub struct Block {
    pub elems: Vec<Item>,
}

#[derive(AsVariant, Clone, Debug)]
pub enum Item {
    Nat(Nat),
    Ident(Ident),
    Tuple(Tuple),
    Block(Box<Block>),
    Func(Box<Func>),
    Match(Box<Match>),
    TypeTuple(Box<TypeTuple>),

    RecordType(Box<RecordType>),
    UnionType(Box<UnionType>),
    FuncType(Box<FuncType>),

    UnaryOpExpr(Box<UnaryOpExpr>),
    BinaryOpExpr(Box<BinaryOpExpr>),
    ApplyExpr(Box<ApplyExpr>),

    LetDecl(Box<LetDecl>),
    VarDecl(Box<VarDecl>),
    TypeAliasDecl(Box<TypeAliasDecl>),
}
