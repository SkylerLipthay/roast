extern crate compiletest_rs as compiletest;
extern crate tempdir;
extern crate tempfile;

use std::{fs, env, path};
use std::io::Read;

fn read_to_string<R: Read>(reader: &mut R) -> String {
    let mut contents = String::new();
    reader.read_to_string(&mut contents).unwrap();
    contents
}

fn run_test(name: &'static str, mode: &'static str) {
    let mut config = compiletest::default_config();
    let temp_dir = tempdir::TempDir::new("roast").unwrap();

    config.mode = mode.parse().ok().expect("Invalid mode");
    config.build_base = temp_dir.path().to_path_buf();
    config.src_base = path::PathBuf::from(format!("tests/{}", name));
    config.target = "x86_64-apple-darwin".to_string(); // TODO: support all platforms
    config.target_rustcflags = Some("-L target/debug".to_string());

    let mut temp_file = tempfile::NamedTempFile::new().unwrap();

    env::set_var("ROAST", temp_file.path().to_str().unwrap());
    compiletest::run_tests(&config);

    let mut cmp_file_path = config.src_base.clone();
    cmp_file_path.push("output.js");
    let mut cmp_file = fs::File::open(cmp_file_path).unwrap();

    assert_eq!(read_to_string(&mut temp_file), read_to_string(&mut cmp_file));
}

#[test]
fn compile_test() {
    run_test("first", "run-pass");
}
