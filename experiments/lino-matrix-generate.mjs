// Drafts the features of parity/fixtures/lino-compatibility-matrix.json with
// the official links-notation 0.22 reading of each sample; the drafted links
// were then reviewed by hand before being committed.
import { Parser } from '../js/node_modules/links-notation/src/index.js';
const canonical = (link) => (link.values.length === 0 && link.id !== null
  ? link.id : [link.id, ...link.values.map(canonical)]);
const features = [
  ['named-links', 'Named links', 'papa: loves mama\n', ['mama', 'grandma']],
  ['anonymous-links', 'Anonymous links', '(loves mama)\n', ['loves', 'adores']],
  ['arbitrary-arity', 'Links of any arity', '(a)\n(a b)\n(a b c d e f g)\n', ['g', 'h']],
  ['shared-references', 'Shared references', '(a b)\n(c b)\n', ['c', 'd']],
  ['recursive-references', 'Recursive references', '(loop: loop next)\n', ['next', 'again']],
  ['forward-references', 'Forward references', '(a: b)\n(b: c)\n', ['c', 'd']],
  ['identity', 'Identity of equal-content links', '(x: a b)\n(y: a b)\n', ['y', 'z']],
  ['ordering', 'Ordering of values and links', '(a b c)\n(c b a)\n', ['a', 'z']],
  ['indentation', 'Indented definitions', 'parent:\n  child one\n  child two\n', ['two', 'three']],
  ['nested-multiline-groups', 'Nested multiline groups', '(outer\n  (inner a\n    b)\n  c)\n', ['inner', 'core']],
  ['quoting', 'Quoted references', '("a b" \'c d\' `e f`)\n', ['"a b"', '"x y"']],
  ['escaping', 'Escaped delimiters in quoted references', '(""say "hi" now"" \'\'it\'s\'\')\n', ['\'\'it\'s\'\'', '\'\'it\'\'\'\'s\'\'']],
  ['comments-and-trivia', 'Comments, blank lines and trivia', '# heading\n\na: b  # trailing\n  (c d)\n', ['d', 'e']],
  ['unicode', 'Unicode references', '(любовь: 愛 💞)\n', ['愛', 'ai']],
  ['source-mappings', 'Exact source mappings across line endings and multibyte text', 'a: b\r\n(c ü)\r\n', ['ü', 'ö']],
];
const out = features.map(([id, name, source, [replace, withText]]) => ({
  id, name, source, edit: { replace, with: withText }, links: new Parser().parse(source).map(canonical),
}));
console.log(JSON.stringify(out, null, 2));
