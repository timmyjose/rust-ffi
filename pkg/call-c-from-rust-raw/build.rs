fn main() {
    // generate libmathlib.a
    cc::Build::new().file("c_src/math.c").compile("mathlib");
    println!("cargo:rerun-if-changed=c_src/math.c");
}