import XCTest
import SwiftTreeSitter
import TreeSitterMarkdown
import TreeSitterMarkdownInline

final class TreeSitterXMLTests: XCTestCase {
    func testCanLoadBlockGrammar() throws {
        let parser = Parser()
        let language = Language(language: tree_sitter_markdown())
        XCTAssertNoThrow(try parser.setLanguage(language),
                         "Error loading Markdown block grammar")
    }

    func testCanLoadInlineGrammar() throws {
        let parser = Parser()
        let language = Language(language: tree_sitter_markdown_inline())
        XCTAssertNoThrow(try parser.setLanguage(language),
                         "Error loading Markdown inline grammar")
    }
}
