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

/**
 * The failed checks of a complete lossless default CST: `subject` names the
 * parse language, its positive and recovery sources, aliases, extensions and
 * independent expectation `want` ({ grammars, positive, recovery, embedded }).
 */
function positiveCstProblems({ name, source, recoverySource, aliases, extensions, want }) {
  const problems = [];
  const check = (condition, message) => {
    if (!condition) problems.push(message);
  };
  const bytes = Buffer.from(source, 'utf8');
  // ordinaryPublicParseApi, exactReconstruction
  const network = LinkNetwork.parse(source, name);
  const recovery = LinkNetwork.parse(recoverySource, name);
  check(network.reconstructText() === source, 'positive reconstruction');
  check(recovery.reconstructText() === recoverySource, 'recovery reconstruction');
  // independentExpectedStructure, realGrammarNodes, hierarchy, namedFields
  check(want !== undefined, 'independent expectation');
  if (!want) return problems;
  const root = documentRoot(network);
  const rows = syntaxRows(network, root);
  const wantRows = publicRows(name, source, want.positive);
  const difference = firstDifference(rows, wantRows);
  check(!difference, `positive rows ${difference}`);
  check(rows.length > 1 && rows.every(([depth], index) => index === 0 || depth <= rows[index - 1][0] + 1), 'hierarchy');
  check(rows.filter((row) => row[1]).length === wantRows.filter((row) => row[1]).length, 'fields');
  // grammarVersionRecorded
  const recorded = network.parseGrammars()
    .filter((grammar) => grammar.language === name)
    .map(({ id, version, parserSha256 }) => ({ id, version, parserSha256 }));
  check(recorded.length > 0 && JSON.stringify(recorded) === JSON.stringify(grammarProvenance(name)), 'grammar provenance');
  check(Object.keys(want.grammars).every((id) => recorded.some((grammar) =>
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
  check(rows.filter((row) => row[6].includes('X')).length === wantRows.filter((row) => row[6].includes('X')).length, 'extras');
  // errorAndMissingNodes
  check(network.verifyFullMatch().isClean(), 'positive source is clean');
  check(!recovery.verifyFullMatch().isClean(), 'recovery source is diagnosed');
  const recoveryRows = syntaxRows(recovery, documentRoot(recovery));
  const recoveryDifference = firstDifference(recoveryRows, publicRows(name, recoverySource, want.recovery));
  check(!recoveryDifference, `recovery rows ${recoveryDifference}`);
  // embeddedLanguageBoundaries
  problems.push(...embeddedProblems(network, name, source, want.embedded, 'embedded'));
  // allAliases
  for (const alias of aliases) {
    const aliased = LinkNetwork.parse(source, alias);
    check(!firstDifference(syntaxRows(aliased, documentRoot(aliased)), rows), `alias ${alias}`);
    check(aliased.reconstructText() === source, `alias ${alias} reconstruction`);
  }
  // extensionDispatch: every extension offers the language, and a path that
  // selects it parses to the same tree.
  for (const extension of extensions) {
    const path = `fixture${extension}`;
    check(languageCandidatesForPath(path).includes(name), `extension ${extension}`);
    const dispatched = languageForPath(path);
    const viaPath = LinkNetwork.parse(source, dispatched);
    if (dispatched === name) {
      check(!firstDifference(syntaxRows(viaPath, documentRoot(viaPath)), rows), `extension ${extension} rows`);
    }
  }
  check(extensions.length === 0 || extensions.some((extension) => languageForPath(`fixture${extension}`) === name),
    'an extension selects the language');
  return problems;
}

/** Differences between the embedded regions of `network` and `want`. */
function embeddedProblems(network, language, source, want, label) {
  const regions = embeddedRoots(network, language, source);
  const boundaries = JSON.stringify(regions.map(({ language: name, span }) => [name, span.start, span.end]));
  if (boundaries !== JSON.stringify(want.map(({ language: name, startByte, endByte }) => [name, startByte, endByte]))) {
    return [`${label} boundaries ${boundaries}`];
  }
  return regions.flatMap((region, index) => {
    const difference = firstDifference(syntaxRows(network, region.root, region.span.start), want[index].rows);
    return difference ? [`${label} ${want[index].path} ${difference}`] : [];
  });
}

const TEST_NAME = 'every JavaScript inventory language parses to its complete lossless default CST';

test(TEST_NAME, async () => {
  const failures = [];
  for (const language of inventory.languages) {
    const want = expected.languages[language.name];
    const problems = positiveCstProblems({
      name: language.name,
      source: language.source,
      recoverySource: language.recoverySource,
      aliases: language.aliases,
      extensions: inventory.extensionDispatch[language.name],
      want,
    });
    if (want) {
      const recovery = LinkNetwork.parse(language.recoverySource, language.name);
      if (!syntaxRows(recovery, documentRoot(recovery)).some((row) => /[EM]/u.test(row[6]))) {
        problems.push('recovery has error or missing nodes');
      }
    }
    if (problems.length) failures.push(`${language.name}: ${problems.join('; ')}`);
    else await recordPositiveCstObservations(language.name, TEST_NAME);
  }
  assert.deepEqual(failures, []);
});

const evidenceBytes = await readFile(new URL('../../parity/fixtures/issue-195-evidence.json', import.meta.url));
const evidenceDigest = createHash('sha256').update(evidenceBytes).digest('hex');
const embeddedExpectations = (await parityJson('fixtures/default-cst-expected.json')).embeddedFixtures;
const EMBEDDED_TEST_NAME = 'every JavaScript embedded-language path parses to its complete lossless default CST';

test(EMBEDDED_TEST_NAME, async () => {
  const failures = [];
  for (const fixture of JSON.parse(evidenceBytes).embedded) {
    const label = `${fixture.host} -> ${fixture.target}`;
    const want = embeddedExpectations[label];
    const host = inventory.languages.find(({ name }) => name === fixture.parseLanguage);
    const target = fixture.regionLanguage ?? fixture.target;
    const problems = positiveCstProblems({
      name: host.name,
      source: fixture.source,
      recoverySource: fixture.recoverySource,
      aliases: host.aliases,
      extensions: inventory.extensionDispatch[host.name],
      want: want && {
        grammars: expected.languages[host.name].grammars,
        positive: want.positive.rows,
        recovery: want.recovery.rows,
        embedded: want.positive.embedded,
      },
    });
    if (want) {
      // The fixture's region is an independently expected boundary parsed by
      // the target grammar, whose version the network records.
      const network = LinkNetwork.parse(fixture.source, host.name);
      const start = encoder.encode(fixture.source.slice(0, fixture.source.indexOf(fixture.regionSource))).length;
      const end = start + encoder.encode(fixture.regionSource).length;
      const region = embeddedRoots(network, host.name, fixture.source)
        .find(({ span }) => span.start === start && span.end === end);
      if (region?.language !== target || region.root?.metadata().term !== fixture.root) problems.push('fixture region');
      const recorded = JSON.stringify(network.parseGrammars()
        .filter((grammar) => grammar.language === target)
        .map(({ id, version, parserSha256 }) => ({ id, version, parserSha256 })));
      if (recorded !== JSON.stringify(grammarProvenance(target))) problems.push('target grammar provenance');
      // errorAndMissingNodes: the recovery source's error is inside its
      // embedded region.
      const recovery = LinkNetwork.parse(fixture.recoverySource, host.name);
      problems.push(...embeddedProblems(recovery, host.name, fixture.recoverySource, want.recovery.embedded, 'recovery embedded'));
      const recoveryRegions = embeddedRoots(recovery, host.name, fixture.recoverySource);
      if (!recoveryRegions.some((candidate) => candidate.language === target &&
        syntaxRows(recovery, candidate.root, candidate.span.start).some((row) => /[EM]/u.test(row[6])))) {
        problems.push('recovery region has error or missing nodes');
      }
      // allAliases: every spelling that selects the target language.
      fixture.spellings.forEach((spelling, index) => {
        const spelled = LinkNetwork.parse(spelling, host.name);
        const difference = firstDifference(syntaxRows(spelled, documentRoot(spelled)), want.spellings[index].rows);
        if (difference || spelled.reconstructText() !== spelling) problems.push(`spelling ${index} ${difference}`);
        problems.push(...embeddedProblems(spelled, host.name, spelling, want.spellings[index].embedded, `spelling ${index}`));
        if (!embeddedRoots(spelled, host.name, spelling).some(({ language }) => language === target)) {
          problems.push(`spelling ${index} selects ${target}`);
        }
      });
    }
    if (problems.length) failures.push(`${label}: ${problems.join('; ')}`);
    else await recordEmbeddedCstObservations(fixture);
  }
  assert.deepEqual(failures, []);
});

async function recordEmbeddedCstObservations({ host, target }) {
  if (!process.env.ISSUE_195_OBSERVATION_FILE) return;
  const records = CST_POSITIVE_ASSERTIONS.map((assertionId) => JSON.stringify({
    testId: `i195-embed-${languageSlug(host)}-${languageSlug(target)}-javascript-positive`,
    assertionId,
    fixtureId: `planned:embedded:${host}:${target}`,
    fixtureDigest: evidenceDigest,
    runtime: 'javascript',
    commit: process.env.ISSUE_195_COMMIT,
    outcome: 'passed',
    testName: EMBEDDED_TEST_NAME,
  }));
  await appendFile(process.env.ISSUE_195_OBSERVATION_FILE, `${records.join('\n')}\n`);
}
