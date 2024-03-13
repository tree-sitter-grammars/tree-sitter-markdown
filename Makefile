all install uninstall clean:
	$(MAKE) -C tree-sitter-markdown $@
	$(MAKE) -C tree-sitter-markdown-inline $@

.PHONY: all install uninstall clean
