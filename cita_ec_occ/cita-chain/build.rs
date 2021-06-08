use std::{env, path::PathBuf};

use util::build_info::gen_build_info;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    gen_build_info(out_dir.as_ref(), "build_info.rs");

    let library_name = "myjerasure";
    let root = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let library_dir = dunce::canonicalize(root.join("src")).unwrap();
    println!("cargo:rustc-link-lib=static={}", library_name);
    println!("cargo:rustc-link-search=native={}", env::join_paths(&[library_dir]).unwrap().to_str().unwrap());
}
