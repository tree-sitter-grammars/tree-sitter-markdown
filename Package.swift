// swift-tools-version:5.3

import PackageDescription

let package = Package(
    name: "TreeSitterMarkdown",
    platforms: [.macOS(.v10_13), .iOS(.v11)],
    products: [
        .library(name: "TreeSitterMarkdown", targets: ["TreeSitterMarkdown", "TreeSitterMarkdownInline", "TreeSitterMarkdownQueries", "TreeSitterMarkdownInlineQueries"]),
    ],
    dependencies: [],
    targets: [
        .target(name: "TreeSitterMarkdown",
                path: "tree-sitter-markdown",
                exclude: [
                    "corpus",
                    "grammar.js",
                ],
                sources: [
                    "src/parser.c",
                    "src/scanner.cc",
                ],
                publicHeadersPath: "bindings/swift",
                cSettings: [.headerSearchPath("src")]),
        .target(name: "TreeSitterMarkdownQueries",
                path: "tree-sitter-markdown",
                exclude: [
                    "corpus",
                    "grammar.js",
                    "src",
                ],
                sources: [
                    "bindings/swift/TreeSitterMarkdownQueries/Query.swift",
                ],
                resources: [
                    .process("queries")
                ]),
        .target(name: "TreeSitterMarkdownInline",
                path: "tree-sitter-markdown-inline",
                exclude: [
                    "corpus",
                    "grammar.js",
                ],
                sources: [
                    "src/parser.c",
                    "src/scanner.cc",
                ],
                publicHeadersPath: "bindings/swift",
                cSettings: [.headerSearchPath("src")]),
        .target(name: "TreeSitterMarkdownInlineQueries",
                path: "tree-sitter-markdown-inline",
                exclude: [
                    "corpus",
                    "grammar.js",
                    "src",
                ],
                sources: [
                    "bindings/swift/TreeSitterMarkdownInlineQueries/Query.swift",
                ],
                resources: [
                    .process("queries")
                ]),
    ]
)
