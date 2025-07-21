fn main() {
    cc::Build::new()
        .file("c/uxn11.c")
        .warnings(true)
        .flag_if_supported("-Wall")
        .flag_if_supported("-Wextra")
        .flag_if_supported("-O2")
        .compile("uxn11");
    println!("cargo:rustc-link-lib=static=uxn11");
    println!("cargo:rustc-link-search=native={}", std::env::var("OUT_DIR").unwrap());
    println!("cargo:rerun-if-changed=c/uxn11.c");
    // Generate Rust bindings from C header using bindgen
    let bindings = bindgen::Builder::default()
        .header("c/uxn11.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");
    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("uxn11_bindings.rs"))
        .expect("Couldn't write bindings!");
    // Verbose output for diagnostics
    println!("cargo:warning=build.rs completed");
}
