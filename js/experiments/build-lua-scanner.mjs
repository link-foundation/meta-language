// Generate executable scanner data for tree-sitter-lua 0.5.0's long
// strings and block comments. The grammar owns their three separate tokens;
// the shared scanner family retains the opening level until the closing token.
// node js/experiments/build-lua-scanner.mjs > parity/grammars/scanners/lua.lino
import { scannerFamilies } from '../scripts/scanner-families.mjs';
import { fileURLToPath } from 'node:url';
import { resolve } from 'node:path';

export const LUA_SCANNERS = ['string', 'comment'].map((kind) => ({
  family: 'split-counted-delimiter',
  name: `block_${kind}`,
  startToken: `block_${kind}_start`,
  contentToken: `block_${kind}_content`,
  endToken: `block_${kind}_end`,
  prefix: kind === 'string' ? '[' : '--[',
  marker: '=', opening: '[', closing: ']', suffix: ']',
  skipWhitespace: true,
  contentStops: ['\0'],
  // The pinned upstream scanner stores both counts in uint8_t.
  countModulo: 256,
}));

if (process.argv[1] && fileURLToPath(import.meta.url) === resolve(process.argv[1])) {
  process.stdout.write(scannerFamilies(LUA_SCANNERS));
}
