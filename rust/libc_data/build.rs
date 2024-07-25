use cbindgen;

use std::env;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    cbindgen::Builder::new()
      .with_language(cbindgen::Language::C)
      .with_crate(crate_dir)
      .generate()
      .expect("Unable to generate libc_data")
      .write_to_file("../../c/libc_data/libc_data.h");
}