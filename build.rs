fn main() {
    // Use system-wide static library from /usr/local/lib
    println!("cargo:rustc-link-search=native=/usr/local/lib");
    println!("cargo:rustc-link-lib=static=uxn11");
    println!("cargo:rustc-link-lib=X11");
    println!("cargo:rustc-link-lib=util");
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
    // Diagnostics: check for c/libuxn11.a
    let lib_path = std::path::Path::new("c/libuxn11.a");
    if lib_path.exists() {
        println!("cargo:warning=Found static library: c/libuxn11.a");
    } else {
        println!("cargo:warning=Missing static library: c/libuxn11.a");
    }
}
