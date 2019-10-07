fn main() {
    let kind = "dylib";
    let lib = "user32";
    let prefix = "";
    println!("cargo:rustc-link-lib={}={}{}", kind, prefix, lib);

    let kind = "dylib";
    let lib = "Gdi32";
    let prefix = "";
    println!("cargo:rustc-link-lib={}={}{}", kind, prefix, lib);
}

