extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let magick_header_path = env!("MAGICK_HEADER_PATH");
    let magick_link_path = env!("MAGICK_LINK_PATH");
    println!("cargo:cargo:rustc-link-search={}", magick_link_path);
    let magick_objects: Vec<_> = env!("MAGICK_OBJECTS")
        .split(' ')
        .map(String::from)
        .collect();
    for obj in magick_objects {
        println!("cargo:rustc-link-lib={}", obj);
    }

    let binding = bindgen::builder()
        .clang_arg(format!("-I{}", magick_header_path))
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("failed to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    binding
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
