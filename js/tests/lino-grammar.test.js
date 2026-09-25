import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { test } from 'node:test';

import { LinkNetwork, LinkType } from '../src/index.js';

const { cases } = JSON.parse(
  await readFile(new URL('../../parity/fixtures/lino-grammar-cases.json', import.meta.url), 'utf8'),
);

/** The Syntax tree of a public LiNo network as {term, field, named, text, flags, children}. */
function syntaxTree(network) {
  const fields = new Map();
  const referenced = new Set();
  for (const link of network.links()) {
    if (link.metadata().linkType === LinkType.Field) {
      const [parent, child] = link.references();
      fields.set(`${parent.asU64()}:${child.asU64()}`, link.metadata().term);
    }
    if (link.metadata().linkType === LinkType.Syntax) {
      for (const id of link.references()) referenced.add(id.asU64());
    }
  }
  const visit = (link, field) => {
    const metadata = link.metadata();
    const children = link.references()
      .map((id) => network.link(id))
      .filter((child) => child.metadata().linkType === LinkType.Syntax)
      .map((child) => visit(child, fields.get(`${link.id().asU64()}:${child.id().asU64()}`)));
    return { term: metadata.term, field, named: metadata.named, flags: metadata.flags, span: metadata.span, children };
  };
  const root = network.links().find((link) =>
    link.metadata().linkType === LinkType.Syntax && !referenced.has(link.id().asU64()));
  return visit(root);
}

const decoder = new TextDecoder();
const encoder = new TextEncoder();

function decodeQuoted(text) {
  let count = 0;
  while (text[count] === text[0]) count += 1;
  const quotes = text.slice(0, count);
  return text.slice(count, text.length - count).split(quotes + quotes).join(quotes);
}

/** Projects a `link` or reference node to the official {id, values, children} shape. */
function officialShape(node, bytes) {
  const text = decoder.decode(bytes.slice(node.span.byteRange.start, node.span.byteRange.end));
  if (node.term === 'reference') return { id: text, values: [], children: [] };
  if (node.term === 'quoted_reference') return { id: decodeQuoted(text), values: [], children: [] };
  assert.equal(node.term, 'link');
  const id = node.children.find((child) => child.field === 'id');
  return {
    id: id ? officialShape(id, bytes).id : null,
    values: node.children.filter((child) => child.field === 'value').map((child) => officialShape(child, bytes)),
    children: node.children.filter((child) => child.field === 'child').map((child) => officialShape(child, bytes)),
  };
}

function leaves(node) {
  return node.children.length === 0 && node.term !== 'lino_document' ? [node] : node.children.flatMap(leaves);
}

test('the LiNo grammar CST carries the links of the official links-notation parser', () => {
  for (const { source, links } of cases) {
    const network = LinkNetwork.parse(source, 'LiNo');
    assert.equal(network.reconstructText(), source);
    const tree = syntaxTree(network);
    const bytes = encoder.encode(source);
    assert.equal(tree.term, 'lino_document');
    assert.equal(tree.flags.hasError, links === null, `error state of ${JSON.stringify(source)}`);
    if (links !== null) {
      assert.deepEqual(
        tree.children.filter((child) => child.named).map((child) => officialShape(child, bytes)),
        links,
        `links of ${JSON.stringify(source)}`,
      );
    }
    let covered = 0;
    for (const leaf of leaves(tree)) {
      assert.equal(leaf.span.byteRange.start, covered, `leaves of ${JSON.stringify(source)} are contiguous`);
      covered = leaf.span.byteRange.end;
      if (leaf.term === 'whitespace') {
        assert.ok(leaf.flags.isExtra && !leaf.named);
        assert.match(decoder.decode(bytes.slice(leaf.span.byteRange.start, covered)), /^[ \t\r\n]+$/u);
      }
    }
    assert.equal(covered, bytes.length);
  }
});

test('the LiNo grammar CST records id, value and child fields and recovers per line', () => {
  const recovered = syntaxTree(LinkNetwork.parse('greeting:\n  hello (x: y)\n(broken\nnext: line\n', 'LiNo'));
  const rows = [];
  const walk = (node, depth) => {
    rows.push(`${'  '.repeat(depth)}${node.field ? `${node.field}: ` : ''}${node.term}`);
    node.children.filter((child) => child.term !== 'whitespace').forEach((child) => walk(child, depth + 1));
  };
  walk(recovered, 0);
  assert.deepEqual(rows, [
    'lino_document',
    '  link',
    '    id: reference',
    '    :',
    '    child: link',
    '      value: reference',
    '      value: link',
    '        (',
    '        id: reference',
    '        :',
    '        value: reference',
    '        )',
    '  ERROR',
    '    (',
    '    reference',
    '  link',
    '    id: reference',
    '    :',
    '    value: reference',
  ]);
  assert.ok(recovered.flags.hasError && !recovered.flags.isError);
  assert.ok(recovered.children.find((child) => child.term === 'ERROR').flags.isError);
});
