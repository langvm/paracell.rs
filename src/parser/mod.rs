// Copyright 2024 Jelly Terra
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0
// that can be found in the LICENSE file and https://mozilla.org/MPL/2.0/.

use lalrpop_util::lalrpop_mod;

pub mod ast;

lalrpop_mod!(pub cee, "flow.lalrpop");
lalrpop_mod!(pub sexpr, "sexpr.lalrpop");
