use std::env;

fn main() {
    if env::var("TARGET").expect("target").ends_with("windows-msvc") {
        println!("cargo:rustc-link-arg-bins=/MANIFEST:EMBED");
        println!("cargo:rustc-link-arg-bins=/MANIFESTUAC:level='requireAdministrator'");
    }
    println!("cargo:rerun-if-changed=build.rs");
}