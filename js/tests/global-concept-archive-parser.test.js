import assert from 'node:assert/strict';
import test from 'node:test';
import { Grammar, compileGrammar } from '../src/grammar.js';
import { parseGrammarLinks } from '../src/grammar-links.js';
import { createGlobalConceptSpace, createGlobalConceptSpaceParser,
  globalConceptSpaceToLinks, readGlobalConceptSpaceLinks } from '../src/global-concept-space.js';
import { readStructuredDataLino, structuredDataToLino } from '../src/structured-data-links.js';
import { encodeFormalConceptDefinition } from '../src/formal-concept-definition.js';
import { readGrammarSourceDefaultMatching } from '../src/translation/grammar-selection-rules.js';
import { isNativeGrammar, nativeGrammarText } from '../src/native-grammar-parser.js';
import { parseProgrammingLanguage } from '../src/programming-language-parser.js';
import { languageEntry } from '../src/language-catalog.js';
import { nativeRows } from '../scripts/native-grammar-rows.mjs';
import { readProjectData } from '../scripts/project-data.mjs';
import { authoredGrammarSelectionListing, grammarSelectionFixture,
  grammarSelectionLiteral } from './support/grammar-selection-cases.js';

const encoder = new TextEncoder();

function archivedRecursiveFixture() {
  const fixture = grammarSelectionFixture();
  const grammars = new Map(fixture.grammars.slice(0, 4).map(({ name, listing }) => [name, parseGrammarLinks(listing)]));
  const sources = [...grammars].map(([id, grammar]) => ({ id, language: id, edition: fixture.chain.edition, grammar }));
  const space = createGlobalConceptSpace(sources, { records: [] });
  const expected = compileGrammar(grammars.get(fixture.chain.root), {
    grammarName: fixture.chain.root, globalConceptSpace: space, resolveGrammar: name => grammars.get(name),
  }).parseTree(fixture.chain.source);
  assert.equal(expected.ok, true, JSON.stringify(expected.rejection));
  const archive = globalConceptSpaceToLinks(space), mappings = { ...space.mappings };
  for (const grammar of grammars.values()) grammar.rules.clear();
  grammars.clear();
  sources.length = 0;
  space.sources.length = 0;
  space.concepts.length = 0;
  return { archive, mappings, expected, chain: fixture.chain };
}

function archivedGrammar(listing, id = 'root-source', language = id, edition = 'authored-1') {
  const grammar = parseGrammarLinks(listing);
  const space = createGlobalConceptSpace([{ id, language, edition, grammar }], { records: [] });
  return globalConceptSpaceToLinks(space);
}

function embeddedArchiveTrees(node, found = []) {
  if (node.type === 'embed') found.push(node);
  for (const child of node.children ?? (node.root ? [node.root] : [])) embeddedArchiveTrees(child, found);
  return found;
}

function archivedTreeBytes(node) {
  if (node.type === 'missing') return [];
  if (node.type === 'token' || node.type === 'error') return node.text === null
    ? [...node.hex.matchAll(/../gu)].map(([byte]) => Number.parseInt(byte, 16)) : [...encoder.encode(node.text)];
  return (node.children ?? [node.root]).flatMap(archivedTreeBytes);
}

test('one validated archive executes the exact recursive CST after all original grammar maps are discarded', () => {
  const { archive, mappings, expected, chain } = archivedRecursiveFixture();
  const parser = createGlobalConceptSpaceParser(archive, chain.root, { records: [] });
  const actual = parser.parseTree(chain.source);
  assert.deepEqual(actual, expected);
  assert.deepEqual(parser.parse(chain.source), expected.tree);
  const embedded = embeddedArchiveTrees(actual.tree);
  assert.deepEqual(embedded.map(({ language }) => language), chain.languages);
  for (const tree of embedded) {
    assert.equal(tree.selection.provenance.edition, chain.edition);
    const origin = `${tree.language}:document`;
    assert.equal(tree.selection.provenance.rules.find(rule => rule.origin === origin).concept, mappings[origin]);
  }
  assert.equal(embedded[0].selection.provenance.rules[0].concept, embedded[3].selection.provenance.rules[0].concept);
  const stored = readStructuredDataLino(structuredDataToLino(actual.tree));
  assert.deepEqual(stored, actual.tree);
  assert.deepEqual(archivedTreeBytes(stored), [...encoder.encode(chain.source)]);
  const decoded = readGlobalConceptSpaceLinks(archive, { records: [] });
  assert.equal(Object.hasOwn(decoded, 'grammars'), false, 'the existing decoder return shape stays unchanged');
  assert.equal(decoded.sources.some(source => Object.hasOwn(source, 'grammar')), false);
  assert.equal(globalConceptSpaceToLinks(decoded), archive);
});

test('archive source identifiers remain exact even when two sources declare the same language', () => {
  const sources = ['one', 'two'].map((id, index) => ({ id, language: 'same-language', edition: String(index + 1),
    grammar: parseGrammarLinks(authoredGrammarSelectionListing(grammarSelectionLiteral(id))) }));
  const archive = globalConceptSpaceToLinks(createGlobalConceptSpace(sources, { records: [] }));
  for (const id of ['one', 'two']) {
    const outcome = createGlobalConceptSpaceParser(archive, id, { records: [] }).parseTree(id);
    assert.equal(outcome.ok, true);
    assert.equal(outcome.tree.language, id);
    assert.equal(outcome.tree.selection.provenance.edition, id === 'one' ? '1' : '2');
  }
  for (const id of ['same-language', 'ONE', '', null]) assert.throws(
    () => createGlobalConceptSpaceParser(archive, id, { records: [] }),
    error => error.reason === 'selection' && /exact existing source identifier/u.test(error.message));
  for (const option of ['resolveGrammar', 'globalConceptSpace', 'grammarName']) {
    assert.throws(() => createGlobalConceptSpaceParser(archive, 'one', { records: [], [option]: null }), /owns parser option/u);
    const parser = createGlobalConceptSpaceParser(archive, 'one', { records: [] });
    assert.throws(() => parser.parseTree('one', { [option]: null }), /owns parser option/u);
    assert.throws(() => parser.parse('one', { [option]: null }), /owns parser option/u);
  }
});

test('archived any and all selections retain every witness and explicit unknown failures', () => {
  const fixture = grammarSelectionFixture();
  const sources = fixture.grammars.slice(4).map(({ name: id, listing }) => ({ id, language: id, grammar: parseGrammarLinks(listing) }));
  for (const name of ['any', 'all', 'allRejected', 'unknown', 'escaped']) sources.push({ id: name, language: name,
    grammar: parseGrammarLinks(fixture[name].listing) });
  const archive = globalConceptSpaceToLinks(createGlobalConceptSpace(sources, { records: [] }));
  const any = createGlobalConceptSpaceParser(archive, 'any', { records: [] }).parseTree('π');
  assert.equal(any.ok, true);
  const selected = embeddedArchiveTrees(any.tree)[1];
  assert.deepEqual(selected.selection.candidates.map(item => item.language), ['first', 'second']);
  assert.deepEqual(selected.selection.alternatives.map(item => item.language), ['second']);
  assert.deepEqual(any.ambiguities.find(item => item.rule === 'any-grammar').languages, ['first', 'second']);
  const all = createGlobalConceptSpaceParser(archive, 'all', { records: [] }).parseTree('π');
  assert.equal(all.ok, true);
  assert.deepEqual(embeddedArchiveTrees(all.tree)[1].selection.alternatives.map(item => item.language), ['second']);
  assert.equal(all.ambiguities.some(item => item.rule === 'any-grammar'), false);
  assert.equal(createGlobalConceptSpaceParser(archive, 'allRejected', { records: [] }).parseTree('π').ok, false);
  for (const [id, reason] of [['unknown', 'unknownGrammar'], ['escaped', 'unsupportedEmbeddingMapping']]) {
    const outcome = createGlobalConceptSpaceParser(archive, id, { records: [] }).parseTree('π');
    assert.equal(outcome.ok, false);
    assert.equal(outcome.rejection.reason, reason);
  }
});

test('archive compilation runs every existing proof and final redirect validation before exposing grammars', () => {
  const fixture = readProjectData(new URL('../../parity/fixtures/global-concept-space.lino', import.meta.url));
  const archive = fixture.expectedLinks, options = { records: fixture.records };
  assert.equal(typeof createGlobalConceptSpaceParser(archive, 'first', options).parseTree, 'function');
  const changed = archive.replace(/\(literal %CE%B1\)/u, '(literal %CE%B2)');
  assert.notEqual(changed, archive);
  assert.throws(() => createGlobalConceptSpaceParser(changed, 'first', options), /unproved global concept correspondence/u);
  const removed = archive.split('\n').filter(line => !line.startsWith('(concept-proof ')).join('\n');
  assert.notEqual(removed, archive);
  assert.throws(() => createGlobalConceptSpaceParser(removed, 'first', options), /lacks its proof witnesses/u);
  const forged = archive.replace('(first%3Aroot second%3Apage)', '(first%3Aroot first%3Aroot)');
  assert.notEqual(forged, archive);
  assert.throws(() => createGlobalConceptSpaceParser(forged, 'first', options), /proof/u);
  assert.throws(() => createGlobalConceptSpaceParser(`${archive}(concept-redirect missing nowhere)\n`, 'first', options), /redirect/u);
});

test('archive roots preserve explicit and implicit starts and refuse counterfeit start or rule-order metadata', () => {
  const sources = [
    { id: 'explicit', language: 'explicit', grammar: new Grammar('chosen', [
      ['fallback', { expression: { kind: 'literal', value: 'x' } }], ['chosen', { expression: { kind: 'literal', value: 'π' } }],
    ]) },
    { id: 'implicit', language: 'implicit', grammar: new Grammar(null, [
      ['first', { expression: { kind: 'literal', value: 'π' } }], ['second', { expression: { kind: 'literal', value: 'x' } }],
    ]) },
  ];
  const archive = globalConceptSpaceToLinks(createGlobalConceptSpace(sources, { records: [] }));
  for (const [id, root] of [['explicit', 'chosen'], ['implicit', 'first']]) {
    const result = createGlobalConceptSpaceParser(archive, id, { records: [] }).parseTree('π');
    assert.equal(result.ok, true);
    assert.equal(result.tree.root.kind, root);
  }
  const parser = createGlobalConceptSpaceParser(archive, 'explicit', { records: [] });
  for (const method of ['parseTree', 'parse']) assert.throws(
    () => parser[method]('π', { startRule: 'absent' }),
    error => error.reason === 'reference' && error.message === 'undefined start rule absent');
  const start = archive.replace('(start defined chosen)', '(start defined absent)');
  assert.notEqual(start, archive);
  assert.throws(() => createGlobalConceptSpaceParser(start, 'explicit', { records: [] }), /start or rule order/u);
  const order = archive.replace('(rule-order fallback chosen)', '(rule-order chosen chosen)');
  assert.notEqual(order, archive);
  assert.throws(() => createGlobalConceptSpaceParser(order, 'explicit', { records: [] }), /duplicate grammar origin rule order/u);
});

test('archive recursion keeps the shared budgets and same-region cycle diagnostics', () => {
  const { archive, chain } = archivedRecursiveFixture();
  const parser = createGlobalConceptSpaceParser(archive, chain.root, { records: [] });
  for (const [options, reason] of [[{ stepLimit: 0 }, 'stepLimit'], [{ memoryLimit: 0 }, 'memoryBudget'], [{ maxDepth: 0 }, 'nestingDepth']]) {
    const result = parser.parseTree(chain.source, options);
    assert.equal(result.ok, false);
    assert.equal(result.rejection.reason, reason);
    assert.equal(result.rejection.limit, 0);
  }
  const cycle = grammarSelectionFixture().cyclic;
  const cyclicArchive = archivedGrammar(cycle.listing, cycle.name);
  const result = createGlobalConceptSpaceParser(cyclicArchive, cycle.name, { records: [] }).parseTree(cycle.source);
  assert.equal(result.ok, false);
  assert.equal(result.rejection.reason, 'nonConsumingEmbeddingCycle');
});

test('archived raw PEG formats preserve possessive repetition and explicit generalized overrides', () => {
  for (const format of ['', 'bnf', 'ebnf', 'abnf', 'antlr', 'lark', 'gbnf', 'tree-sitter', 'meta-language', 'inferred']) {
    assert.equal(readGrammarSourceDefaultMatching(format), 'generalized');
  }
  assert.equal(readGrammarSourceDefaultMatching('peg'), 'peg');
  const listing = '(grammar (start document))\n(rule document normal (seq (repeat0 (literal a)) (literal a)))\n';
  const grammar = parseGrammarLinks(listing);
  const specifications = [
    { id: 'peg-default', format: 'peg', matching: null, accepted: false },
    { id: 'peg-explicit', format: 'peg', matching: 'peg', accepted: false },
    { id: 'generalized', format: null, matching: null, accepted: true },
    { id: 'override', format: 'peg', matching: 'generalized', accepted: true },
  ];
  const sources = specifications.map(({ id, format, matching }) => ({ id, language: id,
    grammar: new Grammar(grammar.start, grammar.rules, format, matching === null ? null : { matching }) }));
  const meaning = encodeFormalConceptDefinition(undefined, { model: 'source text value' });
  const space = createGlobalConceptSpace(sources, { records: [],
    formalDefinitions: Object.fromEntries(sources.map(({ id }) => [`${id}:document`, meaning])),
    equivalences: [['peg-default:document', 'peg-explicit:document']],
  });
  assert.deepEqual(space.failures, []);
  assert.equal(space.mappings['peg-default:document'], space.mappings['peg-explicit:document']);
  assert.notEqual(space.mappings['peg-default:document'], space.mappings['generalized:document']);
  assert.notEqual(space.mappings['peg-default:document'], space.mappings['override:document']);
  assert.equal(space.sources.find(({ id }) => id === 'peg-default').context.matching, null,
    'the original declared matching remains absent rather than a fabricated declaration');
  const expected = sources.map(({ id, grammar }, index) => {
    const ordinary = compileGrammar(grammar).parseTree('a');
    assert.equal(ordinary.ok, specifications[index].accepted, id);
    const qualified = compileGrammar(grammar, { grammarName: id, globalConceptSpace: space }).parseTree('a');
    assert.equal(qualified.ok, ordinary.ok, id);
    if (!ordinary.ok) assert.equal(ordinary.rejection.reason, 'syntax');
    return qualified;
  });
  const archive = globalConceptSpaceToLinks(space);
  assert.ok(archive.includes('(grammar-source-format peg-default peg)'));
  assert.ok(!archive.includes('(grammar-source-format generalized '));
  for (const { grammar } of sources) grammar.rules.clear();
  grammar.rules.clear();
  const reloaded = readGlobalConceptSpaceLinks(archive, { records: [] });
  assert.deepEqual(reloaded, space);
  assert.equal(globalConceptSpaceToLinks(reloaded), archive);
  for (let index = 0; index < specifications.length; index += 1) {
    const { id } = specifications[index];
    assert.deepEqual(createGlobalConceptSpaceParser(archive, id, { records: [] }).parseTree('a'), expected[index]);
  }
});

test('archive format defaults and declared matching cannot bypass current concept witnesses', () => {
  const listing = '(grammar (format peg) (start document))\n(rule document normal (seq (repeat0 (literal a)) (literal a)))\n';
  const sources = ['first', 'second'].map(id => ({ id, language: id, grammar: parseGrammarLinks(listing) }));
  sources[1].grammar = new Grammar(sources[1].grammar.start, sources[1].grammar.rules, 'peg', { matching: 'peg' });
  const meaning = encodeFormalConceptDefinition(undefined, { model: 'source text value' });
  const archive = globalConceptSpaceToLinks(createGlobalConceptSpace(sources, { records: [],
    formalDefinitions: { 'first:document': meaning, 'second:document': meaning },
    equivalences: [['first:document', 'second:document']],
  }));
  const original = readGlobalConceptSpaceLinks(archive, { records: [] });
  // This exact format edit preserves semantics because the second source has
  // actual explicit PEG matching. Format spelling alone cannot split a proof.
  const explicitFormatEdit = archive.replace('(grammar-source-format second peg)', '(grammar-source-format second bnf)');
  assert.notEqual(explicitFormatEdit, archive);
  const unchanged = readGlobalConceptSpaceLinks(explicitFormatEdit, { records: [] });
  assert.equal(unchanged.sources.find(({ id }) => id === 'second').sourceFormat, 'bnf');
  assert.deepEqual(unchanged.mappings, original.mappings);
  assert.deepEqual(unchanged.proofs, original.proofs);
  assert.equal(createGlobalConceptSpaceParser(explicitFormatEdit, 'second', { records: [] }).parseTree('a').ok, false);
  for (const changed of [
    archive.replace('(grammar-source-format first peg)', '(grammar-source-format first bnf)'),
    archive.replace('(grammar-source-format first peg)\n', ''),
    archive.replace('(grammar-context second (matching peg))', '(grammar-context second (matching generalized))'),
  ]) {
    assert.notEqual(changed, archive);
    for (const id of ['first', 'missing']) assert.throws(
      () => createGlobalConceptSpaceParser(changed, id, { records: [] }), /unproved global concept correspondence|one-to-one proof/u);
  }
  for (const changed of [
    archive.replace('(grammar-source-format first peg)', '(grammar-source-format first unknown)'),
    archive.replace('(grammar-source-format first peg)', '(grammar-source-format first (peg))'),
    archive.replace('(grammar-source-format first peg)', '(grammar-source-format first peg generalized)'),
    `${archive}(grammar-source-format first peg)\n`,
    `${archive}(grammar-source-format absent peg)\n`,
    archive.replace('(grammar-context second (matching peg))', '(grammar-context second (matching unknown))'),
  ]) {
    assert.notEqual(changed, archive);
    assert.throws(() => readGlobalConceptSpaceLinks(changed, { records: [] }), /format|matching|word/u);
    assert.throws(() => createGlobalConceptSpaceParser(changed, 'missing', { records: [] }), /format|matching|word/u);
  }
  for (const format of ['unknown', '', 0, false]) {
    const grammar = new Grammar('document', sources[0].grammar.rules, format);
    assert.throws(() => createGlobalConceptSpace([{ id: 'unsupported', language: 'unsupported', grammar }], { records: [] }), /format/u);
  }
});

test('one existing native JavaScript fixture executes from the archived shipped grammar with exact CST rows', () => {
  const fixture = readProjectData(new URL('../../parity/fixtures/native-grammars/javascript.lino', import.meta.url));
  const sample = fixture.matches.find(({ source }) => source.length > 0 && source.length <= 48);
  assert.ok(sample, 'the existing independent fixture supplies one bounded source');
  const grammar = parseGrammarLinks(nativeGrammarText('native-javascript'));
  const ordinary = compileGrammar(grammar).parseTree(sample.source);
  assert.equal(ordinary.ok, true, JSON.stringify(ordinary.rejection));
  assert.deepEqual(nativeRows(ordinary.tree, sample.source, fixture), sample.rows);
  const space = createGlobalConceptSpace([{ id: 'javascript-pinned', language: fixture.language, grammar }], { records: [] });
  // The qualified root additionally checks every production and the full
  // scanner/declaration context against the archive using the shared step
  // budget. Bound that work explicitly in both runs; the ordinary fixture
  // above still uses unchanged defaults and the same source and oracle rows.
  const limits = { stepLimit: 2_000_000 };
  const expected = compileGrammar(grammar, { ...limits, grammarName: 'javascript-pinned', globalConceptSpace: space }).parseTree(sample.source);
  assert.equal(expected.ok, true, JSON.stringify(expected.rejection));
  const archive = globalConceptSpaceToLinks(space);
  grammar.rules.clear();
  const actual = createGlobalConceptSpaceParser(archive, 'javascript-pinned', { ...limits, records: [] }).parseTree(sample.source);
  assert.deepEqual(actual, expected);
  assert.deepEqual(nativeRows(actual.tree.root, sample.source, fixture), sample.rows);
  assert.deepEqual(archivedTreeBytes(actual.tree), [...encoder.encode(sample.source)]);
});

test('one archive retains complete C JavaScript Rust and TypeScript grammars and independent concrete rows', () => {
  const fixtures = ['c', 'javascript', 'rust', 'typescript'].map(id => {
    const fixture = readProjectData(new URL(`../../parity/fixtures/native-grammars/${id}.lino`, import.meta.url));
    const sample = fixture.matches.find(({ source, rows }) => source.length > 0
      && encoder.encode(source).length <= 64 && rows.length >= 4);
    assert.ok(sample, id + ': an existing independent fixture supplies a bounded nontrivial CST');
    assert.equal(languageEntry(fixture.language).grammars[0].id, 'native-' + id);
    assert.equal(isNativeGrammar('native-' + id), true);
    return { id: 'native-' + id, fixture, sample };
  });
  const sources = fixtures.map(({ id, fixture }) => ({ id, language: fixture.language,
    grammar: parseGrammarLinks(nativeGrammarText(id)) }));
  const space = createGlobalConceptSpace(sources, { records: [] });
  const limits = { stepLimit: 4_000_000 };
  const expected = fixtures.map(({ id, fixture, sample }, index) => {
    const grammar = sources[index].grammar;
    const ordinary = compileGrammar(grammar).parseTree(sample.source);
    assert.equal(ordinary.ok, true, id + ': ' + JSON.stringify(ordinary.rejection));
    assert.deepEqual(nativeRows(ordinary.tree, sample.source, fixture), sample.rows);
    const parsed = parseProgrammingLanguage(sample.source, fixture.language);
    assert.equal(parsed.tree.flags.hasError, false, id + ': default public CST uses no recovery');
    assert.equal(parsed.tokens.map(token => token.text).join(''), sample.source,
      id + ': all physical default CST tokens remain in source order');
    assert.deepEqual(publicArchiveConcreteRows(parsed.tree), sample.rows,
      id + ': every public CST kind field flag and byte interval retains its independent oracle');
    const qualified = compileGrammar(grammar, { ...limits, grammarName: id, globalConceptSpace: space }).parseTree(sample.source);
    assert.equal(qualified.ok, true, id + ': ' + JSON.stringify(qualified.rejection));
    return qualified;
  });
  const archive = globalConceptSpaceToLinks(space), mappings = { ...space.mappings };
  for (const { grammar } of sources) grammar.rules.clear();
  sources.length = 0;
  space.sources.length = 0;
  space.concepts.length = 0;
  const restored = readGlobalConceptSpaceLinks(archive, { records: [] });
  assert.deepEqual(restored.mappings, mappings);
  assert.equal(globalConceptSpaceToLinks(restored), archive);
  for (const [index, { id, fixture, sample }] of fixtures.entries()) {
    const actual = createGlobalConceptSpaceParser(archive, id, { ...limits, records: [] }).parseTree(sample.source);
    assert.deepEqual(actual, expected[index], id + ': the complete outcome survives removal of all original grammar maps');
    assert.deepEqual(nativeRows(actual.tree.root, sample.source, fixture), sample.rows);
    assert.deepEqual(archivedTreeBytes(actual.tree), [...encoder.encode(sample.source)]);
    assert.deepEqual(readStructuredDataLino(structuredDataToLino(actual)), actual,
      id + ': every qualified result field remains inspectable Links data');
  }
});

function publicArchiveConcreteRows(node, depth = 0, rows = []) {
  if (node.gap) return rows;
  const flags = node.flags;
  rows.push([depth, node.field ?? null, node.term, node.named ? 1 : 0,
    node.span.byteRange.start, node.span.byteRange.end,
    (flags.isError ? 'E' : '') + (flags.isMissing ? 'M' : '') + (flags.isExtra ? 'X' : '')]);
  for (const child of node.children) publicArchiveConcreteRows(child, depth + 1, rows);
  return rows;
}
