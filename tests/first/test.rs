#![feature(plugin, custom_attribute)]
#![plugin(roast)]
#![roast(output_path_env = "ROAST")]

#[roast]
fn use_numbers(a_foo: i32, b_foo: i32) -> i32 {
    a_foo + b_foo
}

fn main() {
}
