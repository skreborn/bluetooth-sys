use bindgen;

use std::{env, path::PathBuf};

fn main() {
  println!("cargo:rustc-link-lib=bluetooth");
  println!("cargo:rerun-if-changed=wrapper.h");

  bindgen::Builder::default()
    .header("wrapper.h")
    .parse_callbacks(Box::new(bindgen::CargoCallbacks))
    .generate()
    .expect("Failed to generate bindings.")
    .write_to_file(PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs"))
    .expect("Failed to write generated output.");
}
