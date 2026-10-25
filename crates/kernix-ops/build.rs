//! Build the optional SDK bridge only for explicitly enabled macOS consumers.
fn main() {
    println!("cargo:rerun-if-changed=src/accelerate.c");
    #[cfg(feature = "accelerate")]
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("macos") {
        cc::Build::new()
            .file("src/accelerate.c")
            .compile("kernix_accelerate_0_12");
        println!("cargo:rustc-link-lib=framework=Accelerate");
    }
}
