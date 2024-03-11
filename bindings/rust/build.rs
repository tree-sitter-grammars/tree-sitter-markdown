fn main() {
    let block_dir = std::path::Path::new("tree-sitter-markdown").join("src");
    let inline_dir = std::path::Path::new("tree-sitter-markdown-inline").join("src");

    let mut config = cc::Build::new();
    config.include(&block_dir);

    for path in &[
        block_dir.join("parser.c"),
        block_dir.join("scanner.c"),
        inline_dir.join("parser.c"),
        inline_dir.join("scanner.c"),
    ] {
        config.file(path);
        println!("cargo:rerun-if-changed={}", path.to_str().unwrap());
    }

    config.compile("tree-sitter-markdown");
}
