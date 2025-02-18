// Copyright 2025 Jelly Terra <jellyterra@symboltics.com>
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0
// that can be found in the LICENSE file and https://mozilla.org/MPL/2.0/.

use std::cell::RefCell;
use std::collections::HashMap;

pub struct RecordType {
    pub elems: Vec<Type>,
    pub names: HashMap<String, usize>,
}

pub struct UnionType {
    pub types: Vec<Type>,
}

pub enum PrimitiveType {
    Nat(Nat),
}

pub struct Nat {
    pub width: Option<usize>,
}

pub enum Type {
    Primitive(PrimitiveType),
    Record(RefCell<RecordType>),
    Union(RefCell<UnionType>),
}

pub struct FuncType {
    pub params: RecordType,
    pub results: RecordType,
}

pub struct StateType {
    pub typ: Type,
}
