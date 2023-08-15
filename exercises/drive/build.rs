use std::env;

fn main() {
    // 设置环境变量
    println!("cargo:rustc-env=TEST_FOO=1234567890");

    // 设置特征标识符
    #[cfg(not(feature = "pass"))]
    println!("cargo:rustc-cfg=feature=\"not_pass\"");
}