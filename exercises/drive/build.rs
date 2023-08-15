use std::env;

fn main() {
    #[cfg(test)]
    if let Ok(test_foo) = env::var("TEST_FOO") {
        println!("cargo:rustc-env=TEST_FOO={}", test_foo);
    }

    #[cfg(test)]
    #[cfg(feature = "pass")]
    if let Ok(test_feature) = env::var("TEST_FEATURE") {
        if test_feature == "pass" {
            println!("cargo:rustc-cfg=feature=\"pass\"");
        }
    }
}