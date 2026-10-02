// Prints the JavaScript import error of every malformed source in
// parity/fixtures/grammar-importers.json next to the Rust CLI's error for the
// same file, to see whether the two runtimes word import errors alike.
// Usage: node experiments/interchange-cli-error-messages-probe.mjs <rust-binary>
import { execFileSync } from 'node:child_process';
import { mkdtempSync, readFileSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';

import * as ml from '../js/src/index.js';

const corpus = JSON.parse(readFileSync(new URL('../parity/fixtures/grammar-importers.json', import.meta.url)));
const importers = { abnf: ml.importAbnf, bnf: ml.importBnf, ebnf: ml.importEbnf, pest: ml.importPest, 'tree-sitter-json': ml.importTreeSitterJson };
const rustFormat = { abnf: 'abnf', bnf: 'bnf', ebnf: 'ebnf', pest: 'peg', 'tree-sitter-json': 'tree-sitter' };
const directory = mkdtempSync(join(tmpdir(), 'malformed-'));
for (const [index, { format, source }] of corpus.malformed.entries()) {
  let js;
  try { importers[format](source); js = 'accepted'; } catch (error) { js = error.message; }
  const file = join(directory, `${index}.txt`);
  writeFileSync(file, source);
  let rust;
  try {
    execFileSync(process.argv[2], ['import-grammar', '--format', rustFormat[format], file], { stdio: 'pipe' });
    rust = 'accepted';
  } catch (error) { rust = String(error.stderr).trim(); }
  console.log(`${format}\n  js:   ${js}\n  rust: ${rust}`);
}
