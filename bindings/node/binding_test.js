/// <reference types="node" />

const assert = require("node:assert");
const { test } = require("node:test");

test("can load block grammar", () => {
  const parser = new (require("tree-sitter"))();
  assert.doesNotThrow(() => parser.setLanguage(require(".")));
});

test("can load inline grammar", () => {
  const parser = new (require("tree-sitter"))();
  assert.doesNotThrow(() => parser.setLanguage(require(".").inline));
});
