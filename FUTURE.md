# Roast's future

A more compilicated example involving structures:

```rust
#[roast]
struct Foo {
    a: i32,
    b: i32
}

impl Foo {
    #[roast]
    fn default() -> Foo {
        Foo { a: 0, b: 0 }
    }

    // A type can have either zero or one `roast_constructor` static method. If
    // no `roast_constructor` attribute is specified for a type, an empty
    // JavaScript constructor will be generated. For instance, if the following
    // `#[roast_constructor]` attribute was erased, `function Foo() {}` would be
    // the JavaScript type declaration. The `roast_constructor` method for a
    // type is only used if the `new Foo(...)` JavaScript call has the same
    // number of provided parameters as the method signature in Rust. If instead
    // of `roast_constructor`, `roast_constructor_strict` is used, the
    // JavaScript constructor throws an exception if the wrong number of
    // parameters is provided.
    #[roast_constructor]
    fn double(half_a: i32, half_b: i32) -> Foo {
        Foo { a: half_a * 2, b: half_b * 2 }
    }

    #[roast]
    fn double_double(quarter_a: i32, quarter_b: i32) -> Foo {
        Foo::double(quarter_a * 2, quarter_b * 2)
    }

    #[roast]
    fn set_a(&mut self, a: i32) {
        self.a = a;
    }

    #[roast]
    fn is_a_greater_than_50(&self) -> bool {
        self.a > 50
    }
}
```

Would generate:

```js
function Foo() {
  if (arguments.length === 2) {
    return Foo.double.apply(arguments);
  }
}

Foo.default = function() {
  this.a = 0;
  this.b = 0;
  return this;
};

Foo.double = function(halfA, halfB) {
  this.a = halfA * 2;
  this.b = halfB * 2;
  return this;
};

Foo.doubleDouble = function(quarterA, quarterB) {
  return Foo.double(quarterA * 2, quarterB * 2);
};

Foo.prototype.setA = function(a) {
  this.a = a;
};

Foo.prototype.isAGreaterThan50 = function() {
  return this.a > 50;
};
```

## Ideas

* Decide what subset of Rust will be supported, and how.
* Look into cases where the Rust type contains a variable with the same name as
  one of its methods. Perhaps we should prefix any private rust variables with
  `__` but somehow avoid the implicated possibility of naming collisions.
* Inline JavaScript macro like `EM_ASM` from Emscripten.
* Proper JavaScript scoping. Can we nest JavaScript declarations in namespacing
  objects based off of the crate &amp; module structure in the source Rust?
* Override export name via attribute (`#[roast_name(fooFunction)]`).
