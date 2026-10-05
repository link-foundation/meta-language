// Tries candidate sources on the native Regex grammar and its tree-sitter-regex
// oracle: SAME, DIFF, REJECT, or ORACLE-ERROR with the native outcome, to pick
// the hand-written matches and rejections of the fixture.
//   node experiments/native-regex-candidates.mjs
import { readFileSync } from 'node:fs';

import { compileGrammar } from '../src/grammar.js';
import { parseGrammarLinks } from '../src/grammar-links.js';
import { nativeOracleKinds } from '../scripts/build-language-catalog.mjs';
import { nativeRows, oracleRecovers, oracleRows } from '../scripts/native-grammar-rows.mjs';

const text = readFileSync(new URL('../../parity/grammars/native/regex.lino', import.meta.url), 'utf8');
const compiled = compileGrammar(parseGrammarLinks(text));
const options = { hidden: [], anonymous: ['unnamed_token'], extras: [], oracleKinds: nativeOracleKinds(text) };
const candidates = [
  '', 'a', 'abc', 'a|b|c', '^a$', '\\bword\\B', 'a*b+c?', 'a*?b+?c??', 'a{2}', 'a{2,}', 'a{2,5}?', 'a{,5}', '[abc]', '[^a-z0-9_]',
  '[\\d\\s]', '[-a]', '[a-]', '(a)(b)', '(?:a|b)', '(?<year>\\d{4})-\\k<year>', '(?P<n>x)', '(?=a)', '(?!a)', '(?<=a)', '(?<!a)',
  '\\1', '\\cA', '\\n\\t', '\\u00e9', '\\u{1F600}', '\\p{L}', '\\P{Script=Greek}', '[[:alpha:]]', '(?i)abc', '(?i-m:abc)', '(?-s)x',
  '.', 'caf[eé]', '^(?<word>[a-zé]+)\\s*(\\d{2,4})?$|caf[eé]', 'a\nb', 'x\\.y', '\\/', 'a{', '{1}',
  '(ab[c', '(', ')', '[', ']', '(?', '(?<', '(?<a', '(?:', 'a|(', '[a', '\\', '(?P<>a)', '*', '+a', '?', 'a**', 'a{2}{3}',
  '(?<n>a', 'a)', '[[:alpha:]', '\\k<', '\\p{', '(?=', '(?i', '[]', '[^]',
];
for (const source of candidates) {
  const outcome = compiled.parseTree(source);
  let kind;
  if (oracleRecovers(source, 'Regex')) kind = `ORACLE-ERROR native ${outcome.ok ? 'ACCEPTS' : 'rejects'}`;
  else if (!outcome.ok) kind = 'REJECT';
  else kind = (JSON.stringify(oracleRows(source, 'Regex')) === JSON.stringify(nativeRows(outcome.tree, source, options)) ? 'SAME' : 'DIFF') + (outcome.ambiguities?.length ? '-AMBIGUOUS' : '');
  console.log(`${kind} ${JSON.stringify(source)}`);
}
