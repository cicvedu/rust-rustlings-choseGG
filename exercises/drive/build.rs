fn main() {
    #[cfg(feature = "pass")]
    println!("cargo:rustc-cfg=feature=\"pass\"");
}