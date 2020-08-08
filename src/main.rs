fn main() {
    rust_meros::primitives::file::File::new(std::path::Path::new(
        "./testfile",
    ))
    .unwrap();
}
