#!/usr/bin/env node

const { execSync } = require("child_process");
const { join } = require("path");

for (const dir of ["tree-sitter-markdown", "tree-sitter-markdown-inline"]) {
  console.log(`building ${dir}`);
  execSync("npx tree-sitter-cli generate --no-bindings", {
    stdio: "inherit",
    cwd: join(__dirname, "..", dir)
  });
}
