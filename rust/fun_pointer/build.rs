use cbindgen;

use std::env;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    cbindgen::Builder::new()
      .with_language(cbindgen::Language::C)
      .with_crate(crate_dir)
      .generate()
      .expect("Unable to generate fun_pointer")
      .write_to_file("../../c/fun_pointer/fun_pointer.h");
}