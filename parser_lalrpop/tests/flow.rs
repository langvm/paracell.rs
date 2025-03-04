// Copyright 2025 Jelly Terra <jellyterra@symboltics.com>
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0
// that can be found in the LICENSE file and https://mozilla.org/MPL/2.0/.

use paracell_parser_lalrpop::flow::*;

#[test]
fn test_parse_nat() {
    let hex = grammar::NatParser::new().parse("0xEF").unwrap().val;
    let dec = grammar::NatParser::new().parse("1024").unwrap().val;
    let oct = grammar::NatParser::new().parse("0o644").unwrap().val;
    let bin = grammar::NatParser::new().parse("0b1001").unwrap().val;
    assert_eq!(hex, 0xEF);
    assert_eq!(dec, 1024);
    assert_eq!(oct, 0o644);
    assert_eq!(bin, 0b1001);
}

#[test]
fn test_parse_record() {
    let mut v = grammar::ItemParser::new().parse("
        record {
            A: Nat,
            B: Nat
        }
    ").unwrap().as_RecordType().unwrap().fields;

    let field2 = v.pop().unwrap();
    let field1 = v.pop().unwrap();

    assert_eq!(field1.ident.lit, "A");
    assert_eq!(field2.ident.lit, "B");

    assert_eq!(field1.ty.as_Ident().unwrap().lit, "Nat");
    assert_eq!(field2.ty.as_Ident().unwrap().lit, "Nat");
}

#[test]
fn test_parse_union() {
    let mut v = grammar::ItemParser::new().parse("
        union {
            A: Nat,
            B: Nat
        }
    ").unwrap().as_UnionType().unwrap().variants;

    let field2 = v.pop().unwrap();
    let field1 = v.pop().unwrap();

    assert_eq!(field1.ident.lit, "A");
    assert_eq!(field2.ident.lit, "B");

    assert_eq!(field1.ty.as_Ident().unwrap().lit, "Nat");
    assert_eq!(field2.ty.as_Ident().unwrap().lit, "Nat");
}

#[test]
fn test_parse_unary() {
    let v = match grammar::ItemParser::new().parse("~Bit").unwrap() {
        Item::UnaryOpExpr(v) => v,
        _ => panic!(),
    };

    match v.op {
        UnaryOperator::Invert => {}
        _ => panic!()
    }

    assert_eq!(v.expr.as_Ident().unwrap().lit, "Bit");
}

#[test]
fn test_parse_binary() {
    let v = match grammar::ItemParser::new().parse("1 + 2").unwrap() {
        Item::BinaryOpExpr(v) => v,
        _ => panic!(),
    };

    let left = v.left.as_Nat().unwrap().val;
    let right = v.right.as_Nat().unwrap().val;

    match v.op {
        BinaryOperator::Add => {}
        _ => panic!(),
    }

    assert_eq!(left, 1);
    assert_eq!(right, 2);
}

#[test]
fn test_parse_let() {
    let decl = match grammar::ItemParser::new().parse("let v = 1 + 2").unwrap() {
        Item::LetDecl(v) => v,
        _ => panic!(),
    };

    let expr = decl.expr.as_BinaryOpExpr().unwrap();

    let left = expr.left.as_Nat().unwrap().val;
    let right = expr.right.as_Nat().unwrap().val;

    assert_eq!(decl.ident.lit, "v");
    assert_eq!(left, 1);
    assert_eq!(right, 2);
}

#[test]
fn test_parse_func() {
    let src = "
        fun (a: Nat, b: Nat) -> Nat {
            let v = a + b;
            v
        }
    ";

    let mut v = match grammar::ItemParser::new().parse(src).unwrap() {
        Item::Func(v) => v,
        _ => panic!(),
    };

    let param2 = v.params.pop().unwrap();
    let param1 = v.params.pop().unwrap();

    assert_eq!(param1.ident.lit, "a");
    assert_eq!(param2.ident.lit, "b");

    assert_eq!(param1.ty.as_Ident().unwrap().lit, "Nat");
    assert_eq!(param2.ty.as_Ident().unwrap().lit, "Nat");
}

#[test]
fn test_parse_apply() {
    let mut v = grammar::ItemParser::new().parse("Invoke(1, 2, 3 + 4)").unwrap().as_ApplyExpr().unwrap();

    assert_eq!(v.func.as_Ident().unwrap().lit, "Invoke");

    let arith = v.params.elems.pop().unwrap().as_BinaryOpExpr().unwrap();

    assert_eq!(arith.left.as_Nat().unwrap().val, 3);
    assert_eq!(arith.right.as_Nat().unwrap().val, 4);

    assert_eq!(v.params.elems.pop().unwrap().as_Nat().unwrap().val, 2);
    assert_eq!(v.params.elems.pop().unwrap().as_Nat().unwrap().val, 1);
}

#[test]
fn test_parse_match() {
    let mut v = grammar::ItemParser::new().parse("
        match nat {
            1 => 2,
            3 => 4
        }
    ").unwrap().as_Match().unwrap();

    let pattern2 = v.patterns.pop().unwrap();
    let pattern1 = v.patterns.pop().unwrap();

    assert_eq!(v.expr.as_Ident().unwrap().lit, "nat");
    assert_eq!(pattern1.pattern.as_Nat().unwrap().val, 1);
    assert_eq!(pattern2.pattern.as_Nat().unwrap().val, 3);
    assert_eq!(pattern1.expr.as_Nat().unwrap().val, 2);
    assert_eq!(pattern2.expr.as_Nat().unwrap().val, 4);
}
