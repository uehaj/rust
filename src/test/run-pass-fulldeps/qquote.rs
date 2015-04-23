// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// ignore-pretty

#![feature(quote, rustc_private)]

extern crate syntax;

use syntax::ast;
use syntax::codemap;
use syntax::parse;
use syntax::print::pprust;

trait FakeExtCtxt {
    fn call_site(&self) -> codemap::Span;
    fn cfg(&self) -> ast::CrateConfig;
    fn ident_of(&self, st: &str) -> ast::Ident;
    fn name_of(&self, st: &str) -> ast::Name;
    fn parse_sess(&self) -> &parse::ParseSess;
}

impl FakeExtCtxt for parse::ParseSess {
    fn call_site(&self) -> codemap::Span {
        codemap::Span {
            lo: codemap::BytePos(0),
            hi: codemap::BytePos(0),
            expn_id: codemap::NO_EXPANSION,
        }
    }
    fn cfg(&self) -> ast::CrateConfig { Vec::new() }
    fn ident_of(&self, st: &str) -> ast::Ident {
        parse::token::str_to_ident(st)
    }
    fn name_of(&self, st: &str) -> ast::Name {
        parse::token::intern(st)
    }
    fn parse_sess(&self) -> &parse::ParseSess { self }
}

fn main() {
    let cx = parse::new_parse_sess();

    quote_ty!(&cx, isize).and_then(|ty| {
        assert_eq!(pprust::ty_to_string(&ty), "isize")
    });

    quote_pat!(&cx, Some(_)).and_then(|pat| {
        assert_eq!(pprust::pat_to_string(&pat), "Some(_)")
    });

    let arm = quote_arm!(&cx, (ref x, ref y) => (x, y),);
    assert_eq!(pprust::arm_to_string(&arm), " (ref x, ref y) => (x, y),");

    quote_expr!(&cx, 23).and_then(|expr| {
        assert_eq!(pprust::expr_to_string(&expr), "23")
    });

    quote_stmt!(&cx, let x = 20;).unwrap().and_then(|stmt| {
        assert_eq!(pprust::stmt_to_string(&stmt), "let x = 20;")
    });

    let attr = quote_attr!(&cx, #![cfg(foo = "bar")]);
    assert_eq!(pprust::attr_to_string(&attr), "#![cfg(foo = \"bar\")]");

    quote_item!(&cx, static x : isize = 10;).unwrap().and_then(|item| {
        assert_eq!(pprust::item_to_string(&item), "static x: isize = 10;")
    });
}
