// Writes CHECKOUT/src/grammar.json for a tree-sitter checkout that commits
// only grammar.js, as `tree-sitter generate` does: node evaluates grammar.js
// after the CLI's DSL (crates/generate/src/dsl.js at the CLI release, passed
// as DSL), and the JSON is pretty-printed in insertion order, as serde_json
// with preserve_order prints it.
//
//   node js/experiments/generate-tree-sitter-grammar-json.mjs CHECKOUT DSL MAJOR.MINOR.PATCH
import { execFileSync } from 'node:child_process';
import { readFileSync, writeFileSync } from 'node:fs';
import { join, resolve } from 'node:path';

const [checkout, dsl, version] = process.argv.slice(2);
const [major, minor, patch] = version.split('.').map(Number);
const prelude = `globalThis.TREE_SITTER_CLI_VERSION_MAJOR = ${major};
         globalThis.TREE_SITTER_CLI_VERSION_MINOR = ${minor};
         globalThis.TREE_SITTER_CLI_VERSION_PATCH = ${patch};`;
const stdout = execFileSync('node', ['--input-type=module', '-'], {
  input: prelude + readFileSync(dsl, 'utf8'),
  env: { ...process.env, TREE_SITTER_GRAMMAR_PATH: resolve(checkout, 'grammar.js') },
  maxBuffer: 1 << 28,
}).toString();
const json = stdout.slice(stdout.lastIndexOf('\n') + 1);
writeFileSync(join(checkout, 'src/grammar.json'), JSON.stringify(JSON.parse(json), null, 2));
