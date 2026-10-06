// Compares the keywords each native grammar import extracts with those of
// the upstream generated parser: the tokens `ts_lex_keywords` of its
// src/parser.c accepts. PARSERS maps native grammar ids to parser.c paths:
//   PARSERS=native-c=/path/c/src/parser.c,native-java=... node experiments/native-keywords-compare.mjs
import { readFileSync } from 'node:fs';

import { importSource } from '../scripts/import-native-grammars.mjs';

const read = (path) => JSON.parse(readFileSync(new URL(`../../${path}`, import.meta.url), 'utf8'));
const { sources } = read('parity/grammars/sources.json');
const naming = read('parity/naming/grammar-name-expansions.json');
const words = new Map(naming.words.map(({ word, replacement }) => [word, replacement]));
const unescape = (text) => text.replace(/\\(.)/gu, '$1');

function upstreamKeywords(parser) {
  const text = readFileSync(parser, 'utf8');
  const names = text.slice(text.indexOf('ts_symbol_names'));
  const symbols = new Map([...names.slice(0, names.indexOf('};')).matchAll(/\[(\w+)\] = "((?:[^"\\]|\\.)*)"/gu)].map(([, symbol, name]) => [symbol, unescape(name)]));
  const start = text.indexOf('static bool ts_lex_keywords');
  if (start < 0) return new Set();
  const body = text.slice(start, text.indexOf('\n}\n', start));
  return new Set([...body.matchAll(/ACCEPT_TOKEN\((\w+)\)/gu)].map(([, symbol]) => symbols.get(symbol) ?? symbol));
}

const decode = (text) => text.replace(/%([0-9A-F]{2})/gu, (_, hex) => String.fromCharCode(Number.parseInt(hex, 16)));
for (const pair of (process.env.PARSERS ?? '').split(',').filter(Boolean)) {
  const [native, parser] = pair.split('=');
  const entry = sources.find((item) => item.native === native);
  const { text } = importSource(entry, words, naming.grammars[entry.language]);
  const ours = new Set([...text.matchAll(/\(literal ([^ ()]+)\)+ \(not \(ref word_characters\)\)/gu)].map(([, literal]) => decode(literal)));
  const theirs = upstreamKeywords(parser);
  const missing = [...theirs].filter((keyword) => !ours.has(keyword)).sort();
  const extra = [...ours].filter((keyword) => !theirs.has(keyword)).sort();
  console.log(`${native}: ${ours.size} ours, ${theirs.size} upstream; missing [${missing.join(' ')}] extra [${extra.join(' ')}]`);
}
