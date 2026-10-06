// Rewrites the vendored Lean parser's symbol names the way MSVC without
// /utf-8 compiles them: `×` (a code page 1252 character) becomes the single
// byte 0xD7 and characters outside code page 1252 become `?`. Building and
// running the default CST suite against it must fail the name check.
//   node experiments/issue-195-lean-windows/simulate-msvc-names.mjs rust/vendor/tree-sitter-lean/src/parser.c.gz
import { readFileSync, writeFileSync } from 'node:fs';
import { gunzipSync, gzipSync } from 'node:zlib';

const path = process.argv[2];
const parser = gunzipSync(readFileSync(path)).toString('latin1');
const start = parser.indexOf('static const char * const ts_symbol_names[]');
const end = parser.indexOf('};', start);
const names = parser.slice(start, end)
  .replaceAll('\\u00d7', '\\xd7')
  .replace(/\\u[0-9a-fA-F]{4}/g, '?');
writeFileSync(path, gzipSync(Buffer.from(parser.slice(0, start) + names + parser.slice(end), 'latin1'), { level: 9 }));
