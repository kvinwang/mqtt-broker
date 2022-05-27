fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "emscripten" {
        let emcc_flags = "-s ERROR_ON_UNDEFINED_SYMBOLS=0 --no-entry";
        println!(r#"cargo:rustc-env=EMMAKEN_CFLAGS={emcc_flags}"#);
        println!(r#"cargo:rustc-env=EMCC_CFLAGS={emcc_flags}"#);
    }
}

