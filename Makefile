all install uninstall clean test:
	$(MAKE) -C tree-sitter-markdown $@
	$(MAKE) -C tree-sitter-markdown-inline $@

.PHONY: all install uninstall clean test
