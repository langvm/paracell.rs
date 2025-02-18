// Copyright 2025 Jelly Terra <jellyterra@symboltics.com>
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0
// that can be found in the LICENSE file and https://mozilla.org/MPL/2.0/.

// use paracell_parser::lalr::flow::*;

const src: &str = "
fn Divide(a: Nat, b: Nat) -> Nat {
    a + b
}
";

#[test]
fn test_parse() {}