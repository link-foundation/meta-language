// Probes each candidate LiNo compatibility-matrix sample against the official
// links-notation parser and meta-language's lossless LiNo reading.
import { Parser } from '../js/node_modules/links-notation/src/index.js';
import { LinkNetwork } from '../js/src/index.js';
import { parseLinoCst } from '../js/src/lino-grammar.js';

const samples = {
  named: 'papa: loves mama\n',
  anonymous: '(loves mama)\n',
  arity: '(a)\n(a b)\n(a b c d e f g)\n',
  shared: '(a b)\n(c b)\n',
  recursive: '(loop: loop next)\n',
  forward: '(a: b)\n(b: c)\n',
  identity: '(x: a b)\n(y: a b)\n',
  ordering: '(a b c)\n(c b a)\n',
  indentation: 'parent:\n  child one\n  child two\n',
  nested: '(outer\n  (inner a\n    b)\n  c)\n',
  quoting: '("a b" \'c d\' `e f`)\n',
  escaping: '(""say "hi" now"" \'\'it\'s\'\')\n',
  comments: '# heading\n\na: b  # trailing\n  (c d)\n',
  unicode: '(любовь: 愛 💞)\n',
  mappings: 'a: b\r\n(c ü)\r\n',
};
const countLinks = (links) => links.reduce((n, l) => n + 1 + countLinks((l.values ?? []).filter((v) => v.values?.length || v.id !== undefined && v.values)), 0);
for (const [name, src] of Object.entries(samples)) {
  let official;
  try { official = new Parser().parse(src); } catch (e) { official = `ERROR ${e.message}`; }
  const n = LinkNetwork.parse(src, 'LiNo');
  const cstError = JSON.stringify(parseLinoCst(src)).includes('"ERROR"');
  const rel = n.links().filter((l) => l.metadata().linkType === 'Relation');
  const lino = n.toLino();
  console.log(name, {
    lossless: n.reconstructText() === src, cstError,
    official: typeof official === 'string' ? official : official.map(String),
    relations: rel.map((l) => l.metadata().term ?? '-'),
    encoded: LinkNetwork.fromLino(lino).toLino() === lino,
  });
}
