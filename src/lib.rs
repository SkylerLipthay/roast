#![feature(plugin_registrar, rustc_private)]

#[macro_use]
extern crate rustc;
#[macro_use]
extern crate syntax;

use rustc::plugin::Registry;

mod config;
mod gen;
mod lint;
mod util;

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_lint_pass(Box::new(lint::Lint));
}
