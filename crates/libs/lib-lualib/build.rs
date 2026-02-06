// extern crate cc;
use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=lualib-src");

    if cfg!(target_os = "windows") {
        println!(r"cargo:rustc-link-search=native=../../target/release");
        println!("cargo:rustc-link-lib=dylib=moon");
        println!("cargo:rustc-link-lib=moon");
    } else if cfg!(target_os = "macos") {
        println!("cargo:rustc-cdylib-link-arg=-undefined");
        println!("cargo:rustc-cdylib-link-arg=dynamic_lookup");
    } else if cfg!(target_os = "linux") {
        // Link rust.so against ltask.so so send_integer_message/send_message resolve at load time.
        // LTASK_DIR = dir containing ltask.so (default: repo root, ../../../../../ from this crate).
        let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let ltask_dir = env::var("LTASK_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|_| manifest_dir.join("../../../../../"));
        let ltask_dir_str = ltask_dir.to_string_lossy();
        println!("cargo:rustc-link-search=native={}", ltask_dir_str);
        println!("cargo:rustc-link-lib=dylib:+verbatim=ltask.so");
        // At runtime rust.so often lives in luaclib/ and ltask.so in repo root; $ORIGIN/.. finds it.
        println!("cargo:rustc-cdylib-link-arg=-Wl,-rpath,$ORIGIN/..");
    }
}
