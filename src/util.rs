use std::{env, path};
use syntax::ast;

pub fn snake_to_camel_case(s: &str) -> String {
    use std::ascii::*;

    let mut result = String::new();
    let mut at_new_word = false;

    for c in s.chars() {
        if c == '_' {
            at_new_word = true;
        } else if at_new_word {
            result.push(c.to_ascii_uppercase());
            at_new_word = false;
        } else {
            result.push(c);
        }
    }

    return result;
}

pub fn absolutize(pb: path::PathBuf) -> path::PathBuf {
    if pb.as_path().is_absolute() {
        pb
    } else {
        env::current_dir().unwrap().join(&pb.as_path()).clone()
    }
}

pub fn lit_to_str<'a>(lit: &'a ast::Lit) -> Option<&'a str> {
    use syntax::ast::*;

    match lit.node {
        LitStr(ref val, _) => Some(val),
        _ => None
    }
}
