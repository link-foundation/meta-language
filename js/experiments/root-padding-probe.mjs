// Reports where each vendored grammar's root node starts and ends when the
// source has leading and trailing whitespace. Tree-sitter excludes the leading
// padding from the root, so an adapter must retain those bytes itself.
import { readFile } from 'node:fs/promises';
import { gunzipSync } from 'node:zlib';
import { Language, Parser } from 'web-tree-sitter';

import { DRAFTS } from './default-cst-source-drafts.mjs';

await Parser.init();
for (const [name, draft] of Object.entries(DRAFTS)) {
  const url = new URL(`../src/vendor/grammars/${draft.grammar}.wasm.gz`, import.meta.url);
  const parser = new Parser();
  parser.setLanguage(await Language.load(gunzipSync(await readFile(url))));
  const text = ` \n ${draft.source} \n `;
  const tree = parser.parse(text);
  const bytes = Buffer.byteLength(text);
  const { startIndex, endIndex } = tree.rootNode;
  console.log(`${startIndex === 0 && endIndex === bytes ? 'covers' : 'gap   '} ${name}: root ${startIndex}-${endIndex} of ${bytes}`);
  tree.delete();
  parser.delete();
}
