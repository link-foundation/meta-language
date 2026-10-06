// Probes how the official links-notation 0.22 parser reads candidate encodings.
import { Parser } from '../js/node_modules/links-notation/src/index.js';
const canonical = (link) => (link.values.length === 0 && link.id !== null
  ? link.id : [link.id, ...link.values.map(canonical)]);
for (const text of ['a:', '(a:)', '()', '(a)', '(a b)', '(n: a b)', '(n: (m: x) y)', '"a b"', '("a\nb" c)',
  '("x""y" z)', "('\"a' b)", '(`\'"` q)', '("" a)', '(a b c)', '(# x)', '("#x" y)']) {
  try { console.log(JSON.stringify(text), '->', JSON.stringify(new Parser().parse(text).map(canonical))); }
  catch (error) { console.log(JSON.stringify(text), 'ERROR', error.message); }
}
