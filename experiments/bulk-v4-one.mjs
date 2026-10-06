// Experiment: import pinned grammars-v4 files (from the bulk cache) and parse
// the given sources, printing each rejection.
// usage: node experiments/bulk-v4-one.mjs <dir under grammars-v4> <file.g4,...> <source> [<source>...]
import { readFileSync } from 'node:fs';
import { join } from 'node:path';
import { compileGrammar, importAntlr } from '../js/src/index.js';

const [dir, files, ...sources] = process.argv.slice(2);
const base = '/tmp/meta-language-grammar-bulk/raw.githubusercontent.com/antlr/grammars-v4/7df52be94698550d219d299d04105c6bafadd9c3';
const text = files.split(',').map((file) => readFileSync(join(base, dir, file), 'utf8')).join('\n');
const compiled = compileGrammar(importAntlr(text));
for (const source of sources) {
  const result = compiled.parseTree(source.replace(/\\n/gu, '\n'));
  console.log(JSON.stringify(source), '->', result.tree ? 'accepted' : JSON.stringify(result.rejection ?? result.diagnostics).slice(0, 300));
}
