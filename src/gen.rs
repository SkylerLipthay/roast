use rustc::lint::Context;
use rustc::middle::ty::ctxt;
use rustc::middle::astconv_util::ast_ty_to_prim_ty;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use syntax::ast;
use syntax::codemap::Span;

use util;

pub struct Generator <'a, 'tcx: 'a> {
    cx: &'a Context<'a, 'tcx>,
    file: File
}

pub type GeneratorError = (Span, &'static str);
pub type GeneratorResult = Result<(), GeneratorError>;

impl<'a, 'tcx, 'v> Generator<'a, 'tcx> {
    pub fn new(cx: &'a Context<'a, 'tcx>, path: PathBuf)
        -> Result<Generator<'a, 'tcx>, Box<Error>> {
        Ok(Generator { cx: cx, file: try!(File::create(&path)) })
    }

    pub fn generate<G: Generate>(&mut self, g: &G) -> GeneratorResult {
        g.generate(self)
    }

    fn write(&mut self, s: &str) {
        self.file.write(&s.as_bytes()).unwrap();
    }

    fn tcx(&self) -> &'a ctxt<'tcx> {
        self.cx.tcx
    }
}

trait Generate {
    fn generate(&self, generator: &mut Generator) -> GeneratorResult;
}

impl Generate for ast::Item {
    fn generate(&self, generator: &mut Generator) -> GeneratorResult {
        use syntax::ast::*;

        match self.node {
            ItemFn(ref fn_decl, _, _, _, ref block) => {
                let fn_name = util::snake_to_camel_case(self.ident.name.as_str());

                generator.write(&format!("function {}(", fn_name));
                try!(fn_decl.generate(generator));
                generator.write(") {\n");
                try!(block.generate(generator));
                generator.write("}\n");

                Ok(())
            },
            _ => Err((self.span, "unsupported item"))
        }
    }
}

impl Generate for ast::FnDecl {
    fn generate(&self, generator: &mut Generator) -> GeneratorResult {
        use syntax::ast::*;

        // TODO: check return type
        for (index, arg) in self.inputs.iter().enumerate() {
            // TODO: support non-primitives
            if ast_ty_to_prim_ty(generator.tcx(), &*arg.ty).is_none() {
                return Err((arg.ty.span, "unsupported argument"));
            }

            match arg.pat.node {
                PatIdent(_, ref si, _) => {
                    if index > 0 {
                        generator.write(", ");
                    }
                    generator.write(&util::snake_to_camel_case(si.node.name.as_str()));
                },
                _ => return Err((arg.pat.span, "only plain identifiers are supported"))
            }
        }

        Ok(())
    }
}

impl Generate for ast::Block {
    fn generate(&self, generator: &mut Generator) -> GeneratorResult {
        // TODO: statements

        if let Some(ref e) = self.expr {
            generator.write("  return ");
            try!(e.generate(generator));
            generator.write(";\n");
        }

        Ok(())
    }
}

impl Generate for ast::Expr {
    fn generate(&self, generator: &mut Generator) -> GeneratorResult {
        use syntax::ast::*;

        match self.node {
            ExprBinary(ref op, ref a, ref b) => {
                let op = try!(js_bin_op(op).ok_or_else(|| {
                    (op.span, "unsupported binary operator")
                }));
                try!(a.generate(generator));
                generator.write(&format!(" {} ", op));
                try!(b.generate(generator));
            },
            ExprPath(_, ref path) => {
                // TODO: yeah this is freaking awful and should immediately consider scoping. For
                // example: only allow paths that are function parameters.
                let name = path.segments.first().unwrap().identifier.name.as_str();
                generator.write(&util::snake_to_camel_case(name));
            },
            _ => return Err((self.span, "unsupported expression"))
        }

        Ok(())
    }
}

fn js_bin_op(bin_op: &ast::BinOp) -> Option<&'static str> {
    use syntax::ast::*;

    Some(match bin_op.node {
        BiAdd => "+",
        BiSub => "-",
        BiMul => "*",
        BiDiv => "/",
        BiRem => "%",
        BiAnd => "&&",
        BiOr => "||",
        BiBitXor => "^",
        BiBitAnd => "&",
        BiBitOr => "|",
        BiShl => "<<",
        BiShr => ">>",
        BiEq => "===",
        BiLt => "<",
        BiLe => "<=",
        BiNe => "!=",
        BiGe => ">=",
        BiGt => ">"
    })
}
