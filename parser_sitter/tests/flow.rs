// Copyright 2025 Jelly Terra <jellyterra@symboltics.com>
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0
// that can be found in the LICENSE file and https://mozilla.org/MPL/2.0/.

use paracell_parser_sitter::flow::grammar::*;

#[test]
fn test_parse_nat() {
    let v = match parse("0xEF") {
        Ok(Expr::Nat(v)) => v.to_usize(),
        Err(e) => panic!("{:?}", e),
        _ => panic!(),
    };
    assert_eq!(v, 0xEF);
    let v = match parse("1024") {
        Ok(Expr::Nat(v)) => v.to_usize(),
        Err(e) => panic!("{:?}", e),
        _ => panic!(),
    };
    assert_eq!(v, 1024);
    let v = match parse("0o644") {
        Ok(Expr::Nat(v)) => v.to_usize(),
        Err(e) => panic!("{:?}", e),
        _ => panic!(),
    };
    assert_eq!(v, 0o644);
    let v = match parse("0b1001") {
        Ok(Expr::Nat(v)) => v.to_usize(),
        Err(e) => panic!("{:?}", e),
        _ => panic!(),
    };
    assert_eq!(v, 0b1001);
}

#[test]
fn test_parse_ident() {
    let v = match parse("_IdentOfRecord32Bit_") {
        Ok(Expr::Ident(v)) => v.ident,
        Err(e) => panic!("{:?}", e),
        _ => panic!(),
    };
    assert_eq!(v, "_IdentOfRecord32Bit_");
}

#[test]
fn test_parse_type() {
    let v = match parse("") {
        Ok(v) => {}
        Err(e) => panic!("{:?}", e),
        _ => panic!(),
    };
}
