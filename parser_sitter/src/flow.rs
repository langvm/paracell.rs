// Copyright 2025 Jelly Terra <jellyterra@symboltics.com>
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0
// that can be found in the LICENSE file and https://mozilla.org/MPL/2.0/.

#[rust_sitter::grammar("flow")]
pub mod grammar {
    pub enum Nat {
        Hex(
            #[rust_sitter::leaf(pattern = r"0x[0-9A-F]+", transform = |v| usize::from_str_radix(&v[2..], 16).unwrap()
            )]
            usize
        ),
        Dec(
            #[rust_sitter::leaf(pattern = r"[0-9]+", transform = |v| usize::from_str_radix(v, 10).unwrap()
            )]
            usize
        ),
        Oct(
            #[rust_sitter::leaf(pattern = r"0o[0-7]+", transform = |v| usize::from_str_radix(&v[2..], 8).unwrap()
            )]
            usize
        ),
        Bin(
            #[rust_sitter::leaf(pattern = r"0b[0-1]+", transform = |v| usize::from_str_radix(&v[2..], 2).unwrap()
            )]
            usize
        ),
    }

    impl Nat {
        pub fn to_usize(&self) -> usize {
            match self {
                Nat::Hex(v) => v,
                Nat::Dec(v) => v,
                Nat::Oct(v) => v,
                Nat::Bin(v) => v,
            }.clone()
        }
    }

    pub struct Ident {
        #[rust_sitter::leaf(pattern = r"[a-zA-Z_][a-zA-Z0-9_]*", transform = |v| v.to_string())]
        pub ident: String,
    }

    pub struct TypedIdent {
        pub ident: Ident,
        #[rust_sitter::leaf(text = ":")]
        _1: (),
        pub typ: Type,
    }

    pub struct StructType {
        #[rust_sitter::leaf(text = "struct")]
        _0: (),
        #[rust_sitter::leaf(text = "{")]
        _1: (),
        #[rust_sitter::leaf(text = "}")]
        _3: (),
    }

    pub struct UnionType {
        #[rust_sitter::leaf(text = "union")]
        _0: (),
        #[rust_sitter::leaf(text = "{")]
        _1: (),
        #[rust_sitter::leaf(text = "}")]
        _3: (),
    }

    pub enum Type {
        Struct(StructType),
        Union(UnionType),
        Ident(Ident),
    }

    pub enum Expr {
        Nat(Nat),
        Type(Type),
        Ident(Ident),
    }

    #[rust_sitter::language]
    pub enum TopLevel {
        Expr(Expr),
        Type(Type),
    }
}