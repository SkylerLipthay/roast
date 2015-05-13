use rustc::lint::{Context, LintPass, LintArray};
use syntax::{ast, visit};
use syntax::attr::AttrMetaMethods;
use std::cell::RefCell;

use config::Config;
use gen::Generator;
use util;

pub struct Lint;

impl LintPass for Lint {
    fn get_lints(&self) -> LintArray {
        lint_array!()
    }

    fn check_crate(&mut self, cx: &Context, krate: &ast::Crate) {
        let config = Config::from_context(cx);
        let generator = Generator::new(cx, util::absolutize(config.output_path.to_path_buf())).unwrap();

        let roast_cx = RoastContext { cx: cx, generator: RefCell::new(generator) };
        let mut visitor = CrateVisitor { roast_cx: &roast_cx };
        visit::walk_crate(&mut visitor, krate);
    }
}

pub struct RoastContext<'a, 'tcx: 'a> {
    cx: &'a Context<'a, 'tcx>,
    generator: RefCell<Generator<'a, 'tcx>>
}

struct CrateVisitor<'a, 'tcx: 'a> {
    roast_cx: &'a RoastContext<'a, 'tcx>
}

impl<'a, 'tcx, 'v> visit::Visitor<'v> for CrateVisitor<'a, 'tcx> {
    fn visit_item(&mut self, item: &'v ast::Item) {
        if attrs_has_roast(&item.attrs) {
            self.visit_roast_item(item);
        } else {
            self.visit_non_roast_item(item);
        }

        visit::walk_item(self, item);
    }
}

impl<'a, 'tcx, 'v> CrateVisitor<'a, 'tcx> {
    fn visit_roast_item(&self, item: &'v ast::Item) {
        match self.roast_cx.generator.borrow_mut().generate(item) {
            Ok(_) => (),
            Err((span, msg)) => self.roast_cx.cx.sess().span_err(span, msg)
        };

        attrs_mark_used(&item.attrs);
    }

    fn visit_non_roast_item(&self, item: &'v ast::Item) {
        use syntax::ast::*;

        match item.node {
            ItemImpl(_, _, _, _, _, _) => {
                let mut visitor = ImplVisitor { roast_cx: self.roast_cx };
                visit::walk_item(&mut visitor, item)
            },
            _ => ()
        }
    }
}

#[allow(dead_code)] // TODO: support methods
struct ImplVisitor<'a, 'tcx: 'a> {
    roast_cx: &'a RoastContext<'a, 'tcx>
}

impl<'a, 'tcx, 'v> visit::Visitor<'v> for ImplVisitor<'a, 'tcx> {
    fn visit_impl_item(&mut self, impl_item: &'v ast::ImplItem) {
        if attrs_has_roast(&impl_item.attrs) {
            self.visit_roast_impl_item(impl_item);
        }

        visit::walk_impl_item(self, impl_item);
    }
}

impl<'a, 'tcx, 'v> ImplVisitor<'a, 'tcx> {
    fn visit_roast_impl_item(&mut self, impl_item: &'v ast::ImplItem) {
        use syntax::ast::*;

        match impl_item.node {
            MethodImplItem(_, _) => println!("MethodImplItem FOUND"), // TODO: support methods
            _ => return
        };

        attrs_mark_used(&impl_item.attrs);
    }
}

fn attrs_has_roast(attrs: &Vec<ast::Attribute>) -> bool {
    attrs.iter().any(|a| &a.name()[..] == "roast")
}

fn attrs_mark_used(attrs: &Vec<ast::Attribute>) {
    attrs.iter().any(|a| a.check_name("roast"));
}
