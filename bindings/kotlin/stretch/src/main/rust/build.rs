fn main() {
    let os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let pointer_width = std::env::var("CARGO_CFG_TARGET_POINTER_WIDTH").unwrap_or_default();

    // Play requires 64-bit Android shared libraries to support 16 KB pages.
    // max-page-size sets PT_LOAD p_align; the same binary still loads on 4 KB devices.
    // 32-bit ABIs are not part of that requirement and stay at the linker default.
    // The NDK clang driver forwards these -Wl, arguments to lld.
    if os == "android" && pointer_width == "64" {
        println!("cargo:rustc-link-arg-cdylib=-Wl,-z,max-page-size=16384");
        println!("cargo:rustc-link-arg-cdylib=-Wl,-z,common-page-size=16384");
    }
}
