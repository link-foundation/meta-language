import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { test } from 'node:test';

import { LinkNetwork, LinkType } from '../src/index.js';

const parityJson = async (name) =>
  JSON.parse(await readFile(new URL(`../../parity/${name}`, import.meta.url), 'utf8'));
const inventory = await parityJson('language-grammar-inventory.json');
const expected = await parityJson('fixtures/default-cst-expected.json');

const encoder = new TextEncoder();
const decoder = new TextDecoder();

/**
 * Projects the Syntax links of a public network to the grammar rows of
 * `parity/fixtures/default-cst-expected.json`, walking down from `root`, with
 * byte offsets relative to `base`.
 */
function syntaxRows(network, root, base = 0) {
  const fields = new Map();
  for (const link of network.links()) {
    if (link.metadata().linkType === LinkType.Field) {
      const [parent, child] = link.references();
      fields.set(`${parent.asU64()}:${child.asU64()}`, link.metadata().term);
    }
  }
  const rows = [];
  const visit = (link, depth, field) => {
    const { term, named, span, flags } = link.metadata();
    rows.push([
      depth,
      field ?? null,
      term,
      named ? 1 : 0,
      span.byteRange.start - base,
      span.byteRange.end - base,
      `${flags.isError ? 'E' : ''}${flags.isMissing ? 'M' : ''}${flags.isExtra ? 'X' : ''}`,
    ]);
    for (const reference of link.references()) {
      const child = network.link(reference);
      if (child.metadata().linkType === LinkType.Syntax) {
        visit(child, depth + 1, fields.get(`${link.id().asU64()}:${reference.asU64()}`));
      }
    }
  };
  visit(root, 0);
  return rows;
}

/** The grammar root of the document: the first Syntax link no Syntax link references. */
function documentRoot(network) {
  const syntax = network.links().filter((link) => link.metadata().linkType === LinkType.Syntax);
  const children = new Set(syntax.flatMap((link) => link.references().map((id) => id.asU64())));
  return syntax.find((link) => !children.has(link.id().asU64()));
}

/** Every embedded region with its grammar root, in source order. */
function embeddedRoots(network) {
  return network
    .links()
    .filter((link) => link.metadata().linkType === LinkType.Region)
    .map((region) => ({
      language: region.metadata().language,
      span: region.metadata().span.byteRange,
      // A Region link references the host document, its Language link, and
      // then the region's grammar tree.
      root: region.references()
        .slice(2)
        .map((id) => network.link(id))
        .find((link) => link.metadata().linkType === LinkType.Syntax),
    }));
}

/**
 * Applies the documented public projections of the grammar rows: Lean's
 * public root `file` wraps the grammar `module`, and every Rocq `ident`
 * exposes its text as an `identifier` or `primitive_type` token child.
 */
function publicRows(language, source, rows) {
  const bytes = encoder.encode(source);
  if (language === 'Lean') {
    return [
      [0, null, 'file', 1, 0, bytes.length, rows[0]?.[6] ?? ''],
      ...rows.map(([depth, ...row]) => [depth + 1, ...row]),
    ];
  }
  if (language === 'Rocq') {
    return rows.flatMap((row) => {
      if (row[2] !== 'ident') return [row];
      const text = decoder.decode(bytes.subarray(row[4], row[5]));
      const term = ['bool', 'nat', 'Prop', 'Set', 'SProp', 'Type', 'Z'].includes(text)
        ? 'primitive_type'
        : 'identifier';
      return [row, [row[0] + 1, null, term, 1, row[4], row[5], '']];
    });
  }
  return rows;
}

function firstDifference(actual, wanted) {
  const index = actual.findIndex((row, position) =>
    JSON.stringify(row) !== JSON.stringify(wanted[position]));
  return index === -1 && actual.length === wanted.length
    ? undefined
    : `row ${index} (${actual.length} vs ${wanted.length}): actual ${JSON.stringify(actual.slice(index, index + 2))} expected ${JSON.stringify(wanted.slice(index, index + 2))}`;
}

test('JavaScript public networks match grammar-derived rows', () => {
  const differences = [];
  for (const language of inventory.languages) {
    const want = expected.languages[language.name];
    if (!want) continue;
    for (const [kind, key] of [['positive', 'source'], ['recovery', 'recoverySource']]) {
      const source = language[key];
      const network = LinkNetwork.parse(source, language.name);
      const difference = firstDifference(
        syntaxRows(network, documentRoot(network)),
        publicRows(language.name, source, want[kind]),
      );
      if (difference) differences.push(`${language.name} ${kind} ${difference}`);
    }
  }
  assert.deepEqual(differences, []);
});

test('JavaScript embedded regions match grammar-derived rows', () => {
  const differences = [];
  for (const language of inventory.languages) {
    const want = expected.languages[language.name];
    if (!want) continue;
    const network = LinkNetwork.parse(language.source, language.name);
    const regions = embeddedRoots(network);
    assert.deepEqual(
      regions.map(({ language: name, span }) => [name, span.start, span.end]),
      want.embedded.map(({ language: name, startByte, endByte }) => [name, startByte, endByte]),
      language.name,
    );
    for (const [index, region] of regions.entries()) {
      const difference = firstDifference(
        syntaxRows(network, region.root, region.span.start),
        want.embedded[index].rows,
      );
      if (difference) differences.push(`${want.embedded[index].path} ${difference}`);
    }
  }
  assert.deepEqual(differences, []);
});
