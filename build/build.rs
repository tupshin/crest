#[cfg(not(feature = "nightly"))]
fn main() {
    extern crate syntex;
    extern crate serde_codegen;

    use std::env;
    use std::path::Path;

    let out_dir = env::var_os("OUT_DIR").unwrap();

    let src = Path::new("build/type.rs.in");
    let dst = Path::new(&out_dir).join("type.rs.out");

    let mut registry = syntex::Registry::new();

    serde_codegen::register(&mut registry);
    registry.expand("", &src, &dst).unwrap();
}

#[cfg(feature = "nightly")]
fn main() {}
