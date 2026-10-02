// Compares the native JSON grammar draft with the tree-sitter-json oracle
// (the current default parse) row by row over a small corpus.
import { readFileSync } from 'node:fs';
import { LinkNetwork, LinkType, compileGrammar, parseNativeGrammar } from '../src/index.js';

const listing = readFileSync(new URL(process.argv[2] ?? './native-json-draft.listing', import.meta.url), 'utf8');
const parser = compileGrammar(parseNativeGrammar(listing));

function oracleRows(source) {
  const network = LinkNetwork.parse(source, 'JSON');
  const fields = new Map();
  for (const link of network.links()) {
    if (link.metadata().linkType === LinkType.Field && link.references().length === 3) {
      const [parent, label, child] = link.references();
      fields.set(`${parent.asU64()}:${child.asU64()}`, network.link(label).metadata().term);
    }
  }
  const syntax = network.links().filter((link) => link.metadata().linkType === LinkType.Syntax);
  const children = new Set(syntax.flatMap((link) => link.references().map((id) => id.asU64())));
  const root = syntax.find((link) => !children.has(link.id().asU64()));
  const rows = [];
  const visit = (link, depth, field) => {
    const { term, named, span, flags } = link.metadata();
    rows.push([depth, field ?? null, term, named ? 1 : 0, span.byteRange.start, span.byteRange.end,
      `${flags.isError ? 'E' : ''}${flags.isMissing ? 'M' : ''}${flags.isExtra ? 'X' : ''}`]);
    for (const reference of link.references()) {
      const child = network.link(reference);
      if (child.metadata().linkType === LinkType.Syntax) visit(child, depth + 1, fields.get(`${link.id().asU64()}:${reference.asU64()}`));
    }
  };
  visit(root, 0);
  return rows;
}

// Leading trivia of a node belongs before the node, as tree-sitter places extras.
function hoist(node) {
  if (node.type !== 'node') return [node];
  const children = node.children.flatMap(hoist);
  let first = 0;
  while (first < children.length && children[first].trivia) first += 1;
  return [...children.slice(0, first), { ...node, children: children.slice(first) }];
}

function span(node) {
  if (node.type !== 'node') return [node.start, node.end];
  const inner = node.children.filter((child) => !child.trivia).map(span);
  return inner.length === 0 ? [node.start, node.start] : [inner[0][0], inner.at(-1)[1]];
}

function nativeRows(source) {
  const outcome = parser.parseTree(source);
  if (!outcome.ok) return { rejection: outcome.rejection };
  const rows = [];
  const visit = (node, depth) => {
    if (node.type === 'token' && node.trivia && node.kind === null) return;
    const [start, end] = span(node);
    if (node.type === 'node') {
      rows.push([depth, node.field ?? null, node.kind, 1, start, end, '']);
      node.children.flatMap(hoist).forEach((child) => visit(child, depth + 1));
    } else {
      rows.push([depth, node.field ?? null, node.kind ?? node.text, node.kind ? 1 : 0, start, end, node.trivia ? 'X' : '']);
    }
  };
  // tree-sitter's root starts at its first token or comment and ends at the end of the input.
  const root = outcome.tree;
  const leaves = [];
  const collect = (node) => (node.type === 'node' ? node.children.forEach(collect) : leaves.push(node));
  collect(root);
  const first = leaves.find((leaf) => !(leaf.trivia && leaf.kind === null));
  rows.push([0, null, root.kind, 1, first ? first.start : Buffer.byteLength(source), Buffer.byteLength(source), '']);
  root.children.flatMap(hoist).forEach((child) => visit(child, 1));
  return rows;
}

export const CORPUS = [
  '{"name": "meta", "tags": [1, 2.5, true, null], "nested": {"ok": false}}\n',
  '[]', '{}', '""', '0', '-0', '1.', '1.5e10', '-2E-3', '12e3', '"\\u00e9\\n\\"\\\\\\/"',
  '  [1, 2]  \n', '\n\n{"a": 1}\n\n', '// lead\n{"a": /* in */ 1 /* tail */}\n// end\n',
  '[1, /* c */ [2], // x\n 3]', '{"a": /* c */ [1]}', '"Ωmé 漢字 😀"', '{"k": "\\ud83d\\ude00"}',
  '[[[[[]]]]]', '1 2 "three"', '', '   ', '/* only */', '{"a":{"b":{"c":[{"d":null}]}}}',
  '[\t1,\r\n2\f]', '\ufeff[]', '\ufeff {"a": 1}\n',
];

let failures = 0;
for (const source of CORPUS) {
  const want = oracleRows(source);
  const got = nativeRows(source);
  const same = JSON.stringify(want) === JSON.stringify(got);
  if (!same) {
    failures += 1;
    console.log('DIFF', JSON.stringify(source));
    console.log('  oracle', JSON.stringify(want));
    console.log('  native', JSON.stringify(got));
  }
}
console.log(`${CORPUS.length - failures}/${CORPUS.length} identical`);
