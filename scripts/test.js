#!/usr/bin/env node

const { execSync } = require("child_process");
const { join } = require("path");

const parsers = ["tree-sitter-markdown", "tree-sitter-markdown-inline"];

for (const dir of parsers) {
  console.log(`testing ${dir}`);
  try {
    execSync("npx tree-sitter-cli test", {
      stdio: "inherit",
      cwd: join(__dirname, "..", dir)
    });
  } catch(error) {
    process.exitCode |= parsers.indexOf(dir) + 1;
  }
}
