fn main() {
    internal::run();
}

#[cfg(not(feature = "internal-bindgen-on-build"))]
mod internal {
    pub fn run() {
        let out = std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
        std::fs::remove_dir_all(&out).unwrap();
        std::fs::create_dir(&out).unwrap();

        cc::Build::new()
            .file("src/bridge.c")
            .include("src")
            .compile("bridge");
    }
}

#[cfg(feature = "internal-bindgen-on-build")]
mod internal {
    pub fn run() {
        bindgen::Builder::default()
            .header("src/stb_image.h")
            .layout_tests(false)
            .prepend_enum_name(true)
            .disable_name_namespacing()
            .generate_comments(false)
            .allowlist_function("^(stbi).*")
            .allowlist_type("^(stbi).*")
            .allowlist_var("^(stbi).*")
            .blocklist_item(".*(from_file).*")
            .ctypes_prefix("cty")
            .allowlist_recursively(false)
            .generate_inline_functions(true)
            .rust_target(bindgen::RustTarget::Nightly)
            .use_core()
            .size_t_is_usize(true)
            .clang_args([
                "-x",
                "c++",
                "-std=c++17",
                "-fno-inline-functions",
                "-Ifpng\\src\\",
            ])
            .generate()
            .expect("unable to generate bindings")
            .write_to_file("src/stb_image.rs")
            .expect("couldn't write bindings in `src/stb_image.rs`");
    }
}
