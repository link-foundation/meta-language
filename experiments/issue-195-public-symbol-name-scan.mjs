// Lists, for every vendored grammar, the visible symbols whose public symbol
// (ts_language_symbol_for_name returns public_symbol_map[i]) carries a
// different name. web-tree-sitter's Node.type reads the public symbol's name
// while the native ts_node_type reads the symbol's own, so these are the
// symbols on which the two runtimes print different kinds.
import { readdirSync, readFileSync } from 'node:fs';
import { gunzipSync } from 'node:zlib';
import { Language, Parser } from '../js/node_modules/web-tree-sitter/web-tree-sitter.js';

const root = new URL('..', import.meta.url).pathname;
const grammars = `${root}js/src/vendor/grammars`;
await Parser.init({ wasmBinary: gunzipSync(readFileSync(`${root}js/src/vendor/web-tree-sitter/web-tree-sitter.wasm.gz`)) });
for (const file of readdirSync(grammars).filter((name) => name.endsWith('.wasm.gz')).sort()) {
  const language = await Language.load(gunzipSync(readFileSync(`${grammars}/${file}`)));
  const found = [];
  language.types.forEach((name, id) => {
    if (!name || !language.nodeTypeIsVisible(id)) return;
    const named = language.nodeTypeIsNamed(id);
    const publicId = language.idForNodeType(name, named);
    if (publicId !== null && language.types[publicId] !== name) found.push(`${id} ${JSON.stringify(name)} -> ${publicId} ${JSON.stringify(language.types[publicId])}`);
  });
  if (found.length) console.log(`${file.replace('.wasm.gz', '')}: ${found.join('; ')}`);
}
