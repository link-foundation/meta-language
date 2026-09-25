import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { appendFile, readFile } from 'node:fs/promises';
import { test } from 'node:test';

import {
  LinkNetwork,
  LinkType,
  grammarProvenance,
  languageCandidatesForPath,
  languageForPath,
} from '../src/index.js';

const parityJson = async (name) =>
  JSON.parse(await readFile(new URL(`../../parity/${name}`, import.meta.url), 'utf8'));
const inventoryBytes = await readFile(new URL('../../parity/language-grammar-inventory.json', import.meta.url));
const inventory = JSON.parse(inventoryBytes);
const inventoryDigest = createHash('sha256').update(inventoryBytes).digest('hex');
// Tree-sitter languages and built-in grammar languages together cover the
// inventory.
const expected = {
  languages: {
    ...(await parityJson('fixtures/default-cst-expected.json')).languages,
    ...(await parityJson('fixtures/builtin-cst-expected.json')).languages,
  },
};

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

/**
 * Every embedded region with its grammar root, in source order. A region of
 * the document's own language over the whole document (the natural-language
 * annotation of a text) embeds nothing.
 */
function embeddedRoots(network, language, source) {
  const length = encoder.encode(source).length;
  return network
    .links()
    .filter((link) => link.metadata().linkType === LinkType.Region)
    .filter((region) => {
      const { language: regionLanguage, span } = region.metadata();
      return !(regionLanguage === language && span.byteRange.start === 0 && span.byteRange.end === length);
    })
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
    const regions = embeddedRoots(network, language.name, language.source);
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

const CST_POSITIVE_ASSERTIONS = [
  'ordinaryPublicParseApi', 'realGrammarNodes', 'independentExpectedStructure', 'grammarVersionRecorded',
  'hierarchy', 'namedFields', 'childOrder', 'tokens', 'commentsAndTrivia', 'exactUtf8Spans',
  'errorAndMissingNodes', 'embeddedLanguageBoundaries', 'exactReconstruction', 'allAliases', 'extensionDispatch',
];

const languageSlug = (name) => name.normalize('NFKD').toLowerCase()
  .replaceAll('+', '-plus').replaceAll('#', '-sharp')
  .replace(/[^a-z0-9]+/gu, '-').replace(/^-|-$/gu, '');

async function recordPositiveCstObservations(language, testName) {
  if (!process.env.ISSUE_195_OBSERVATION_FILE) return;
  const records = CST_POSITIVE_ASSERTIONS.map((assertionId) => JSON.stringify({
    testId: `i195-cst-${languageSlug(language)}-javascript-positive`,
    assertionId,
    fixtureId: `inventory:${language}`,
    fixtureDigest: inventoryDigest,
    runtime: 'javascript',
    commit: process.env.ISSUE_195_COMMIT,
    outcome: 'passed',
    testName,
  }));
  await appendFile(process.env.ISSUE_195_OBSERVATION_FILE, `${records.join('\n')}\n`);
}

/** The Syntax links below `root` with their child Syntax and SourceToken links. */
function syntaxTree(network, root) {
  const nodes = [];
  const visit = (link) => {
    const children = link.references().map((id) => network.link(id));
    const node = {
      link,
      children: children.filter((child) => child.metadata().linkType === LinkType.Syntax),
      tokens: children.filter((child) => child.metadata().linkType === LinkType.SourceToken),
    };
    nodes.push(node);
    node.children.forEach(visit);
  };
  visit(root);
  return nodes;
}

const TEST_NAME = 'every JavaScript inventory language parses to its complete lossless default CST';

test(TEST_NAME, async () => {
  const failures = [];
  for (const language of inventory.languages) {
    const want = expected.languages[language.name];
    const problems = [];
    const check = (condition, message) => {
      if (!condition) problems.push(message);
    };
    const bytes = Buffer.from(language.source, 'utf8');
    // ordinaryPublicParseApi, exactReconstruction
    const network = LinkNetwork.parse(language.source, language.name);
    const recovery = LinkNetwork.parse(language.recoverySource, language.name);
    check(network.reconstructText() === language.source, 'positive reconstruction');
    check(recovery.reconstructText() === language.recoverySource, 'recovery reconstruction');
    // independentExpectedStructure, realGrammarNodes, hierarchy, namedFields
    check(want !== undefined, 'independent expectation');
    const root = documentRoot(network);
    const rows = syntaxRows(network, root);
    const difference = want && firstDifference(rows, publicRows(language.name, language.source, want.positive));
    check(!difference, `positive rows ${difference}`);
    check(rows.length > 1 && rows.every(([depth], index) => index === 0 || depth <= rows[index - 1][0] + 1), 'hierarchy');
    check(!want || rows.filter((row) => row[1]).length === publicRows(language.name, language.source, want.positive).filter((row) => row[1]).length, 'fields');
    // grammarVersionRecorded
    const recorded = network.parseGrammars()
      .filter((grammar) => grammar.language === language.name)
      .map(({ id, version, parserSha256 }) => ({ id, version, parserSha256 }));
    check(recorded.length > 0 && JSON.stringify(recorded) === JSON.stringify(grammarProvenance(language.name)), 'grammar provenance');
    check(!want || Object.keys(want.grammars).every((id) => recorded.some((grammar) =>
      grammar.id === id && grammar.version === want.grammars[id].version && grammar.parserSha256 === want.grammars[id].parserSha256)),
    'expected grammar versions');
    // childOrder, tokens, commentsAndTrivia, exactUtf8Spans
    const trivia = network.links().filter((link) => link.metadata().linkType === LinkType.Trivia)
      .map((link) => `${link.metadata().span.byteRange.start}:${link.metadata().span.byteRange.end}`);
    const isBoundary = (offset) => offset === bytes.length || (offset < bytes.length && (bytes[offset] & 0xc0) !== 0x80);
    for (const node of syntaxTree(network, root)) {
      const { span, flags, term } = node.link.metadata();
      const { start, end } = span.byteRange;
      check(start <= end && end <= bytes.length && isBoundary(start) && isBoundary(end), `UTF-8 span ${term} ${start}:${end}`);
      node.children.forEach((child, index) => {
        const previous = node.children[index - 1]?.metadata().span.byteRange;
        const { start: childStart, end: childEnd } = child.metadata().span.byteRange;
        check(childStart >= start && childEnd <= end && (!previous || previous.end <= childStart), `child order ${term} ${childStart}`);
      });
      if (node.children.length === 0 && start < end) {
        const text = bytes.subarray(start, end).toString('utf8');
        check(node.tokens.some((token) => token.metadata().term === text), `token ${term} ${start}:${end}`);
        if (flags.isExtra) check(trivia.includes(`${start}:${end}`), `trivia ${term} ${start}:${end}`);
      }
    }
    check(!want || rows.filter((row) => row[6].includes('X')).length === want.positive.filter((row) => row[6].includes('X')).length, 'extras');
    // errorAndMissingNodes
    check(network.verifyFullMatch().isClean(), 'positive source is clean');
    check(!recovery.verifyFullMatch().isClean(), 'recovery source is diagnosed');
    const recoveryRows = syntaxRows(recovery, documentRoot(recovery));
    const recoveryDifference = want && firstDifference(recoveryRows, publicRows(language.name, language.recoverySource, want.recovery));
    check(!recoveryDifference, `recovery rows ${recoveryDifference}`);
    check(recoveryRows.some((row) => /[EM]/u.test(row[6])), 'recovery has error or missing nodes');
    // embeddedLanguageBoundaries
    const regions = embeddedRoots(network, language.name, language.source);
    check(want && JSON.stringify(regions.map(({ language: name, span }) => [name, span.start, span.end]))
      === JSON.stringify(want.embedded.map(({ language: name, startByte, endByte }) => [name, startByte, endByte])), 'embedded boundaries');
    regions.forEach((region, index) => {
      const regionDifference = firstDifference(syntaxRows(network, region.root, region.span.start), want.embedded[index].rows);
      check(!regionDifference, `embedded ${want.embedded[index].path} ${regionDifference}`);
    });
    // allAliases
    for (const alias of language.aliases) {
      const aliased = LinkNetwork.parse(language.source, alias);
      check(!firstDifference(syntaxRows(aliased, documentRoot(aliased)), rows), `alias ${alias}`);
      check(aliased.reconstructText() === language.source, `alias ${alias} reconstruction`);
    }
    // extensionDispatch: every extension offers the language, and a path
    // that selects it parses to the same tree.
    const extensions = inventory.extensionDispatch[language.name];
    for (const extension of extensions) {
      const path = `fixture${extension}`;
      check(languageCandidatesForPath(path).includes(language.name), `extension ${extension}`);
      const dispatched = languageForPath(path);
      const viaPath = LinkNetwork.parse(language.source, dispatched);
      if (dispatched === language.name) {
        check(!firstDifference(syntaxRows(viaPath, documentRoot(viaPath)), rows), `extension ${extension} rows`);
      }
    }
    check(extensions.length === 0 || extensions.some((extension) => languageForPath(`fixture${extension}`) === language.name),
      'an extension selects the language');
    if (problems.length) failures.push(`${language.name}: ${problems.join('; ')}`);
    else await recordPositiveCstObservations(language.name, TEST_NAME);
  }
  assert.deepEqual(failures, []);
});
