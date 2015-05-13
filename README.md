# Roast

Roast is an experimental Rust-to-JavaScript transpiler built for the purpose of avoiding code duplication between the server and client codebases of API-driven web applications. As it stands, Roast is unfit for entire projects (or projects at all, really).

## Installation

If you're using Cargo, just add Roast to your `Cargo.toml`:

```toml
[dependencies]
roast = "0.0.1"
```

## Example

```rust
#![feature(plugin, custom_attribute)]
#![plugin(roast)]

// Optional configure attribute (output path defaults to "roast.js"):
#![roast(output_path = "my_roast.js")]

#[roast]
fn use_numbers(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    println!("use_numbers: {}", use_numbers(20, 40));
}
```

`cargo build` would generate `my_roast.js` in the crate's root folder:

```js
function useNumbers(a, b) {
  return a + b;
}
```

This example happens to be an exhaustive demonstration of all supported Rust syntax.

## Supported syntax

* [`ItemFn`](http://doc.rust-lang.org/syntax/ast/enum.Item_.html#variant.ItemFn), [`FnDecl`](http://doc.rust-lang.org/syntax/ast/struct.FnDecl.html) with arguments of [primitive types](http://doc.rust-lang.org/rustc/middle/astconv_util/fn.ast_ty_to_prim_ty.html) and [plain identifiers](http://doc.rust-lang.org/syntax/ast/enum.Pat_.html#variant.PatIdent)
* Function [`Block`](http://doc.rust-lang.org/syntax/ast/struct.Block.html)s with only a [return expression](http://doc.rust-lang.org/syntax/ast/struct.Block.html#structfield.expr) (statements are unsupported)
* Binary operator expressions ([`ExprBinary`](http://doc.rust-lang.org/syntax/ast/enum.Expr_.html#variant.ExprBinary))
* Identifier-only path expressions ([`ExprPath`](http://doc.rust-lang.org/syntax/ast/enum.Expr_.html#variant.ExprPath)) with no support for lexical scope filtering yet

Syntax will be supported as use cases arise.
