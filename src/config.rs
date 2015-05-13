use rustc::lint::Context;
use rustc::session::Session;
use std::env;
use std::path::PathBuf;
use syntax::ast;
use syntax::attr::AttrMetaMethods;
use syntax::codemap::Span;

use util;

pub struct Config {
    pub output_path: PathBuf
}

impl Config {
    pub fn from_context(cx: &Context) -> Config {
        let mut config = Config::default();
        if let Some(mut sm) = ConfigModifier::new(&mut config, cx) {
            sm.apply();
        }
        config
    }
}

impl Default for Config {
    fn default() -> Config {
        Config {
            output_path: PathBuf::from("roast.js")
        }
    }
}

struct ConfigModifier<'a> {
    attr: &'a ast::Attribute,
    config: &'a mut Config,
    sess: &'a Session
}

impl<'a> ConfigModifier<'a> {
    pub fn new(config: &'a mut Config, cx: &'a Context) -> Option<ConfigModifier<'a>> {
        config_attr(cx.krate).map(move |attr| ConfigModifier {
            attr: attr,
            config: config,
            sess: cx.sess()
        })
    }

    pub fn apply(&mut self) {
        use syntax::ast::*;

        match self.attr.node.value.node {
            MetaList(_, ref items) => for item in items {
                self.apply_meta_item(item);
            },
            _ => self.sess.span_err(self.attr.span, "attribute must be parameter list")
        }
    }

    fn apply_meta_item(&mut self, meta_item: &ast::MetaItem) {
        use syntax::ast::*;

        match meta_item.node {
            MetaNameValue(ref name, ref value) => self.apply_value(meta_item.span, name, value),
            _ => self.sess.span_err(self.attr.span, "item must be name-value pair")
        }
    }

    fn apply_value(&mut self, span: Span, name: &str, val: &ast::Lit) {
        match name {
            "output_path" if util::lit_to_str(val).is_some() => {
                self.config.output_path = PathBuf::from(util::lit_to_str(val).unwrap())
            },
            "output_path_env" if util::lit_to_str(val).is_some() => {
                if let Ok(val) = env::var(util::lit_to_str(val).unwrap()) {
                    self.config.output_path = PathBuf::from(val)
                }
            },
            _ => self.sess.span_err(span, "invalid or unrecognized item")
        }
    }
}

fn config_attr(krate: &ast::Crate) -> Option<&ast::Attribute> {
    krate.attrs.iter().find(|a| a.check_name("roast"))
}
