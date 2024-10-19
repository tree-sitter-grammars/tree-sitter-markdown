// swift-tools-version:5.3
import PackageDescription

let package = Package(
    name: "TreeSitterMarkdown",
    platforms: [.macOS(.v10_13), .iOS(.v11)],
    products: [
        .library(name: "TreeSitterMarkdown", targets: ["TreeSitterMarkdown", "TreeSitterMarkdownInline"]),
    ],
    dependencies: [
        .package(url: "https://github.com/ChimeHQ/SwiftTreeSitter", from: "0.8.0"),
    ],
    targets: [
        .target(
            name: "TreeSitterMarkdown",
            path: "tree-sitter-markdown",
            sources: [
                "src/parser.c",
                "src/scanner.c",
            ],
            resources: [
                .copy("queries")
            ],
            publicHeadersPath: "bindings/swift",
            cSettings: [.headerSearchPath("src")]
        ),
        .target(
            name: "TreeSitterMarkdownInline",
            path: "tree-sitter-markdown-inline",
            exclude: [
                "test",
                "grammar.js",
            ],
            sources: [
                "src/parser.c",
                "src/scanner.c",
            ],
            resources: [
                .copy("queries")
            ],
            publicHeadersPath: "bindings/swift",
            cSettings: [.headerSearchPath("src")]
        ),
        .testTarget(
            name: "TreeSitterMarkdownTests",
            dependencies: [
                "SwiftTreeSitter",
                "TreeSitterMarkdown",
                "TreeSitterMarkdownInline",
            ],
            path: "bindings/swift/TreeSitterMarkdownTests"
        )
    ],
    cLanguageStandard: .c11
)
