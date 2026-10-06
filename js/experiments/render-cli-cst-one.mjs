// Renders one file the way `tree-sitter parse --cst` does (see the generator).
//   node experiments/render-cli-cst-one.mjs <file> [grammar-id]
import { readFile } from 'node:fs/promises';
import { gunzipSync } from 'node:zlib';
import { Language, Parser } from 'web-tree-sitter';
import { renderCliCst } from '../scripts/generate-default-cst-expectations.mjs';
await Parser.init();
const p = new Parser();
p.setLanguage(await Language.load(gunzipSync(await readFile(new URL(`../src/vendor/grammars/${process.argv[3] ?? 'css'}.wasm.gz`, import.meta.url)))));
const text = await readFile(process.argv[2], 'utf8');
process.stdout.write(renderCliCst(p.parse(text), text));
