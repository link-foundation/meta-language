// Parses each wave 2 row with its vendored WebAssembly grammar (through the
// package's UTF-8 web-tree-sitter runtime) and reports the root kind, whether
// the positive source is error-free and whether the recovery source produces
// ERROR or MISSING nodes.
//   node experiments/issue-195-wave2/probe.mjs
import { readFile } from 'node:fs/promises';
import { createRequire } from 'node:module';
import { gunzipSync } from 'node:zlib';
import { ROWS } from './rows.mjs';

const root = new URL('../../', import.meta.url);
const require = createRequire(new URL('js/package.json', root));
const { Language, Parser } = await import(require.resolve('web-tree-sitter'));
await Parser.init({
  wasmBinary: gunzipSync(await readFile(new URL('js/src/vendor/web-tree-sitter/tree-sitter.wasm.gz', root))),
});

const encoder = new TextEncoder();
function utf8Input(text) {
  const bytes = encoder.encode(text);
  return (index) => {
    let end = Math.min(bytes.length, index + 4096);
    while (end < bytes.length && (bytes[end] & 0xc0) === 0x80) end -= 1;
    return String.fromCharCode(...bytes.subarray(index, end));
  };
}

const parser = new Parser();
for (const [name, , , grammar, , source, recovery] of ROWS) {
  const wasm = await readFile(new URL(`js/src/vendor/grammars/${grammar}.wasm.gz`, root));
  parser.setLanguage(await Language.load(gunzipSync(wasm)));
  const positive = parser.parse(utf8Input(source)).rootNode;
  const broken = parser.parse(utf8Input(recovery)).rootNode;
  const ok = !positive.hasError && broken.hasError;
  console.log(`${ok ? 'ok ' : 'BAD'} ${name}: root=${positive.type} positiveError=${positive.hasError} recoveryError=${broken.hasError}`);
  if (positive.hasError) console.log(`    ${positive.toString().slice(0, 700)}`);
}
