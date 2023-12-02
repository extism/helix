fn main() {
    let mut path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    path.push("bindings.rs");
    std::process::Command::new("python3")
        .arg("gen.py")
        .arg(path)
        .status()
        .unwrap();
}
