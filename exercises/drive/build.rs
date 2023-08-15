use std::env;

fn main() {
    // 获取环境变量的值
    let test_foo = env::var("TEST_FOO").unwrap_or_else(|_| String::from("1234567890"));

    // 设置环境变量
    println!("cargo:rustc-env=TEST_FOO={}", test_foo);

    // 设置特征标识符
    #[cfg(not(feature = "pass"))]
    println!("cargo:rustc-cfg=feature=\"not_pass\"");
}