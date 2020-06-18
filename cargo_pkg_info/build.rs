// those doc comments are just there to satisfy `-W missing_docs` since I don't need
// docs on build.rs
//!
///
fn main() {
    if let Ok(s) = std::env::var("PROFILE") {
        // values: debug, release
        println!("cargo:rustc-cfg=profile=\"{}\"", s);
        println!("cargo:rustc-env=profile=\"{}\"", s);
    }
}
