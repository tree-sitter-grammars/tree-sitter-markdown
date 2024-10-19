/// <reference types="node" />

const assert = require("node:assert");
const { test } = require("node:test");

const Parser = require("tree-sitter");

test("can load block grammar", () => {
  const parser = new Parser();
  assert.doesNotThrow(() => parser.setLanguage(require(".")));
});

test("can load inline grammar", () => {
  const parser = new Parser();
  assert.doesNotThrow(() => parser.setLanguage(require(".").inline));
});
