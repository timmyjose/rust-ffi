use std::process::Command;

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();

    let status = Command::new("cc")
        .args([
            "-dynamiclib",
            "-fPIC",
            "c_src/math.c",
            "-o",
            &format!("{out_dir}/libmathlib.dylib"),
        ])
        .status()
        .expect("failed to build libmath");

    assert!(status.success(), "libmath build failed");

    println!("cargo:rustc-link-lib=dylib=mathlib");
    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rerun-if-changed=c_src/math.c");
}