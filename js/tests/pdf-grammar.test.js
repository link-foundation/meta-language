import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { test } from 'node:test';

import { LinkNetwork, LinkType } from '../src/index.js';

const { cases } = JSON.parse(
  await readFile(new URL('../../parity/fixtures/pdf-grammar-cases.json', import.meta.url), 'utf8'),
);

/** The Syntax tree of a public PDF network as {term, field, named, flags, span, children}. */
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

const bytesOf = (node, bytes) => bytes.slice(node.span.byteRange.start, node.span.byteRange.end);
const textOf = (node, bytes) => decoder.decode(bytesOf(node, bytes));
const syntaxChildren = (node) => node.children.filter((child) => child.named && !child.flags.isExtra);
const field = (node, name) => node.children.find((child) => child.field === name);

/** A name without its solidus, with `#XX` escapes decoded, as UTF-8 text. */
function decodeName(node, bytes) {
  const raw = bytesOf(node, bytes);
  const decoded = [];
  for (let index = 1; index < raw.length; index += 1) {
    const hex = String.fromCharCode(raw[index + 1] ?? 0, raw[index + 2] ?? 0);
    if (raw[index] === 0x23 && /^[0-9A-Fa-f]{2}$/u.test(hex)) {
      decoded.push(Number.parseInt(hex, 16));
      index += 2;
    } else {
      decoded.push(raw[index]);
    }
  }
  return decoder.decode(new Uint8Array(decoded));
}

function entries(dictionary, bytes) {
  const projected = new Map();
  for (const entry of syntaxChildren(dictionary)) {
    projected.set(decodeName(field(entry, 'key'), bytes), project(field(entry, 'value'), bytes));
  }
  return [...projected];
}

/** Projects a PDF object node to the shape of the pdf-lib oracle. */
function project(node, bytes) {
  const text = textOf(node, bytes);
  switch (node.term) {
    case 'null': return null;
    case 'boolean': return text === 'true';
    case 'integer':
    case 'real': return { number: Number(text) };
    case 'name': return { name: decodeName(node, bytes) };
    case 'literal_string': return { string: text.slice(1, -1) };
    case 'hex_string': return { hex: text.slice(1, -1) };
    case 'indirect_reference':
      return { ref: [Number(textOf(field(node, 'object_number'), bytes)), Number(textOf(field(node, 'generation'), bytes))] };
    case 'array': return syntaxChildren(node).map((child) => project(child, bytes));
    case 'dictionary': return { dictionary: entries(node, bytes) };
    case 'stream': {
      const data = field(node, 'data');
      return { stream: entries(field(node, 'dictionary'), bytes), data: data ? textOf(data, bytes) : '' };
    }
    default: throw new Error(`unexpected PDF object ${node.term}`);
  }
}

/** The indirect objects of a PDF CST, later definitions replacing earlier ones, by object number. */
function indirectObjects(tree, bytes) {
  const objects = new Map();
  for (const object of tree.children.filter((child) => child.term === 'indirect_object')) {
    const ref = [
      Number(textOf(field(object, 'object_number'), bytes)),
      Number(textOf(field(object, 'generation'), bytes)),
    ];
    objects.set(ref.join(' '), { ref, value: project(field(object, 'value'), bytes) });
  }
  return [...objects.values()].sort((left, right) => left.ref[0] - right.ref[0]);
}

function leaves(node) {
  return node.children.length === 0 && node.term !== 'pdf_file' ? [node] : node.children.flatMap(leaves);
}

function terms(node) {
  return [node.term, ...node.children.flatMap(terms)];
}

test('the PDF grammar CST describes the indirect objects pdf-lib reads', () => {
  const seen = new Set();
  let clean = 0;
  for (const { source, objects } of cases) {
    const network = LinkNetwork.parse(source, 'PDF');
    assert.equal(network.reconstructText(), source);
    const tree = syntaxTree(network);
    const bytes = encoder.encode(source);
    assert.equal(tree.term, 'pdf_file');
    if (objects === null) {
      assert.ok(tree.flags.hasError, `pdf-lib rejects ${JSON.stringify(source)}, so its CST has errors`);
    }
    if (!tree.flags.hasError) {
      clean += 1;
      assert.notEqual(objects, null, `pdf-lib reads the clean ${JSON.stringify(source)}`);
      assert.deepEqual(indirectObjects(tree, bytes), objects, `objects of ${JSON.stringify(source)}`);
      terms(tree).forEach((term) => seen.add(term));
    }
    let covered = 0;
    for (const leaf of leaves(tree)) {
      const { start, end } = leaf.span.byteRange;
      assert.equal(start, covered, `leaves of ${JSON.stringify(source)} are contiguous`);
      covered = end;
      if (leaf.term === 'whitespace') {
        assert.ok(leaf.flags.isExtra && !leaf.named);
        assert.match(textOf(leaf, bytes), /^[\0\t\n\f\r ]+$/u);
      }
      if (leaf.term === 'comment') {
        assert.ok(leaf.flags.isExtra && leaf.named);
        assert.match(textOf(leaf, bytes), /^%[^\r\n]*$/u);
      }
    }
    assert.equal(covered, bytes.length);
  }
  assert.ok(clean >= 150, `${clean} clean cases`);
  for (const term of [
    'header', 'indirect_object', 'stream', 'content_stream', 'operation', 'operator', 'inline_image',
    'image_data', 'stream_data', 'dictionary', 'dictionary_entry', 'array', 'indirect_reference',
    'integer', 'real', 'boolean', 'null', 'name', 'literal_string', 'hex_string', 'cross_reference_table',
    'cross_reference_subsection', 'cross_reference_entry', 'trailer', 'start_cross_reference',
    'end_of_file', 'comment', 'whitespace',
  ]) {
    assert.ok(seen.has(term), `a clean case has a ${term} node`);
  }
});

function fieldRows(tree) {
  const rows = [];
  const walk = (node, depth) => {
    const flag = node.flags.isError ? ' (error)' : node.flags.isMissing ? ' (missing)' : '';
    rows.push(`${'  '.repeat(depth)}${node.field ? `${node.field}: ` : ''}${node.term}${flag}`);
    node.children.filter((child) => !child.flags.isExtra).forEach((child) => walk(child, depth + 1));
  };
  walk(tree, 0);
  return rows;
}

test('the PDF grammar CST records fields and keeps malformed input in error and missing nodes', () => {
  const tree = syntaxTree(LinkNetwork.parse(
    '%PDF-1.7\n1 0 obj\n<< /A [1 ) /B >>\n2 0 obj\n<< /Length 2 >>\nstream\nq Q\nendstream\n',
    'PDF',
  ));
  assert.deepEqual(fieldRows(tree), [
    'pdf_file',
    '  header',
    '  indirect_object',
    '    object_number: integer',
    '    generation: integer',
    '    obj',
    '    value: dictionary',
    '      <<',
    '      dictionary_entry',
    '        key: name',
    '        value: array',
    '          [',
    '          integer',
    '          ERROR (error)',
    '            )',
    '          name',
    '          ] (missing)',
    '      >>',
    '    endobj (missing)',
    '  indirect_object',
    '    object_number: integer',
    '    generation: integer',
    '    obj',
    '    value: stream',
    '      dictionary: dictionary',
    '        <<',
    '        dictionary_entry',
    '          key: name',
    '          value: integer',
    '        >>',
    '      stream',
    '      data: content_stream',
    '        operation',
    '          operator: operator',
    '        operation',
    '          operator: operator',
    '      endstream',
    '    endobj (missing)',
    '  end_of_file (missing)',
  ]);
  assert.ok(tree.flags.hasError && !tree.flags.isError);
});
