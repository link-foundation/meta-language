import assert from 'node:assert/strict';
import test from 'node:test';
import { compileGrammar } from '../src/grammar.js';
import { parseGrammarLinks, renderGrammarLinks } from '../src/grammar-links.js';
import { Executor } from '../src/grammar-runtime/executor.js';
import { renderSyntaxTree } from '../src/grammar-runtime/syntax-tree.js';
import { createGlobalConceptSpace, globalConceptSpaceToLinks, readGlobalConceptSpaceLinks } from '../src/global-concept-space.js';
import { readStructuredDataLino, structuredDataToLino } from '../src/structured-data-links.js';
import { acceptGrammarSelectionCount, acceptGrammarSelectionMapping,
  acceptGrammarSelectionMode, reportGrammarSelectionAmbiguity } from '../src/translation/grammar-selection-rules.js';
import { authoredGrammarSelectionListing, grammarSelectionCandidate, grammarSelectionExpression,
  grammarSelectionFixture, grammarSelectionLiteral } from './support/grammar-selection-cases.js';

const encoder = new TextEncoder();
const literal = grammarSelectionLiteral;
const candidate = grammarSelectionCandidate;
const select = grammarSelectionExpression;

function authoredGrammar(expression, root = 'document') {
  return parseGrammarLinks(authoredGrammarSelectionListing(expression, root));
}

function authoredChain() {
  const grammars = new Map(grammarSelectionFixture().grammars.slice(0, 4).map(({ name, listing }) => [name, parseGrammarLinks(listing)]));
  const sources = [...grammars].map(([id, grammar]) => ({ id, language: id, edition: 'authored-1', grammar }));
  const space = readGlobalConceptSpaceLinks(globalConceptSpaceToLinks(createGlobalConceptSpace(sources, { records: [] })), { records: [] });
  return { grammars, space, parser: compileGrammar(grammars.get('Markdown'), { grammarName: 'Markdown', resolveGrammar: name => grammars.get(name), globalConceptSpace: space }) };
}

function embeddedTrees(node, result = []) {
  if (node.type === 'embed') result.push(node);
  for (const child of node.children ?? (node.root ? [node.root] : [])) embeddedTrees(child, result);
  return result;
}

function reconstructBytes(node) {
  if (node.type === 'missing') return [];
  if (node.type === 'token' || node.type === 'error') {
    return node.text === null ? [...node.hex.match(/../gu)].map(byte => Number.parseInt(byte, 16)) : [...encoder.encode(node.text)];
  }
  return (node.children ?? [node.root]).flatMap(reconstructBytes);
}

function assertExactLeaves(node, bytes) {
  if (node.type === 'token' || node.type === 'error') assert.deepEqual(reconstructBytes(node), [...bytes.subarray(node.start, node.end)]);
  for (const child of node.children ?? (node.root ? [node.root] : [])) assertExactLeaves(child, bytes);
}

test('linked grammar selection retains a repeated-language CST chain and proven source-qualified identities', () => {
  const { grammars, space, parser } = authoredChain();
  const source = ' m{ j{d{m{π}}}} \n', bytes = encoder.encode(source);
  const outcome = parser.parseTree(bytes);
  assert.equal(outcome.ok, true, JSON.stringify(outcome.rejection));
  assert.equal(outcome.tree.type, 'embed', 'an explicitly qualified root retains a concrete invocation boundary');
  assert.equal(outcome.tree.root.type, 'node', 'the complete original concrete root remains the default inside its boundary');
  assert.deepEqual(reconstructBytes(outcome.tree), [...bytes]);
  assertExactLeaves(outcome.tree, bytes);
  const embedded = embeddedTrees(outcome.tree);
  assert.deepEqual(embedded.map(node => node.language), ['Markdown', 'JavaScript', 'JSON', 'Markdown', 'Python']);
  assert.equal(embedded[0].selection.mode, 'grammar-root');
  for (const node of embedded) {
    assert.equal(node.selection.mapping, 'identity');
    assert.equal(node.selection.provenance.edition, 'authored-1');
    const root = node.selection.provenance.rules.find(rule => rule.rule === 'document');
    assert.equal(root.origin, `${node.language}:document`);
    assert.equal(root.concept, space.mappings[root.origin]);
    assert.deepEqual(root.sourceNames, [{ source: 'authored', name: 'document' }]);
    assert.equal(node.root.start, node.start);
    assert.equal(node.root.end, node.end);
  }
  assert.equal(embedded[3].selection.provenance.rules[0].concept, embedded[0].selection.provenance.rules[0].concept, 'the return to Markdown reuses the root global origin');
  const physical = renderSyntaxTree(outcome.tree);
  assert.match(physical, /EMBED JavaScript/u);
  assert.match(physical, /EMBED Markdown/u);
  const reloadedTree = readStructuredDataLino(structuredDataToLino(outcome.tree));
  assert.deepEqual(reloadedTree, outcome.tree);
  assert.deepEqual(reconstructBytes(reloadedTree), [...bytes]);
  const restored = new Map([...grammars].map(([name, grammar]) => {
    const links = renderGrammarLinks(grammar);
    assert.equal(renderGrammarLinks(parseGrammarLinks(links)), links);
    return [name, parseGrammarLinks(links)];
  }));
  assert.deepEqual(compileGrammar(restored.get('Markdown'), { grammarName: 'Markdown', resolveGrammar: name => restored.get(name), globalConceptSpace: space }).parseTree(source), outcome, 'source-free grammar data reload preserves hierarchy, delimiters, trivia and identities');
  assert.equal(compileGrammar(grammars.get('Markdown'), { resolveGrammar: name => grammars.get(name) }).parseTree(source).tree.type, 'node', 'legacy default root shapes do not gain an invocation wrapper');
});

test('any preserves every accepted grammar witness and reports language ambiguity; all explicitly requires every candidate', () => {
  const grammars = new Map([['first', authoredGrammar(literal('π'), 'first_root')], ['second', authoredGrammar(literal('π'), 'second_root')], ['other', authoredGrammar(literal('x'))]]);
  const options = { resolveGrammar: name => grammars.get(name) };
  const any = compileGrammar(authoredGrammar(select('any-grammar', ['first', 'second'])), options);
  const result = any.parseTree('π');
  assert.equal(result.ok, true);
  assert.deepEqual(result.ambiguities, [{ rule: 'document', start: 0, end: 2 }, { rule: 'any-grammar', start: 0, end: 2, languages: ['first', 'second'] }]);
  const [node] = embeddedTrees(result.tree);
  assert.equal(node.root.kind, 'first_root');
  assert.equal(node.selection.alternatives[0].root.kind, 'second_root');
  assert.deepEqual(reconstructBytes(result.tree), [...encoder.encode('π')], 'parallel witnesses do not duplicate source ownership');
  assert.deepEqual(readStructuredDataLino(structuredDataToLino(result.tree)), result.tree, 'alternative CSTs also survive linked reload');
  assert.equal(any.parseTree('π', { ambiguity: 'reject' }).rejection.reason, 'ambiguity');
  const all = compileGrammar(authoredGrammar(select('all-grammar', ['first', 'second'])), options).parseTree('π');
  assert.equal(all.ok, true);
  assert.deepEqual(all.ambiguities, []);
  assert.equal(embeddedTrees(all.tree)[0].selection.mode, 'all-grammar');
  assert.equal(embeddedTrees(all.tree)[0].selection.alternatives.length, 1);
  const rejected = compileGrammar(authoredGrammar(select('all-grammar', ['first', 'other'])), options).parseTree('π');
  assert.equal(rejected.ok, false);
  assert.equal(rejected.rejection.reason, 'syntax');
});

test('unknown grammar and unsupported escape mapping retain explicit diagnostics and accepted CST witnesses', () => {
  const known = authoredGrammar(literal('π'));
  const options = { resolveGrammar: name => name === 'known' ? known : undefined };
  for (const mode of ['any-grammar', 'all-grammar']) {
    const unknown = compileGrammar(authoredGrammar(select(mode, ['known', 'missing'])), options).parseTree('π');
    assert.equal(unknown.ok, false);
    assert.equal(unknown.rejection.reason, 'unknownGrammar');
    assert.equal(unknown.rejection.diagnostics[0].language, 'missing');
    if (mode === 'any-grammar') {
      assert.deepEqual(reconstructBytes(unknown.tree), [...encoder.encode('π')]);
      assert.deepEqual(embeddedTrees(unknown.tree)[0].selection.candidates.map(item => item.relation), ['accepted', 'unknown']);
    }
  }
  const escaped = compileGrammar(authoredGrammar(`(any-grammar (ref body) ${candidate('known', 'quoted-escape')})`), options).parseTree('π');
  assert.equal(escaped.rejection.reason, 'unsupportedEmbeddingMapping');
  assert.equal(escaped.rejection.diagnostics[0].mapping, 'quoted-escape');
  assert.throws(() => compileGrammar(authoredGrammar('(any-grammar (ref body))')), /at least one candidate/u);
  assert.throws(() => compileGrammar(authoredGrammar(select('any-grammar', ['known', 'known']))), /duplicate selected grammar/u);
});

test('same-region recursive selection diagnoses cycles while consuming recursion remains lossless', () => {
  const cyclic = authoredGrammar(select('any-grammar', ['cycle'], literal('π')));
  const outcome = compileGrammar(cyclic, { grammarName: 'cycle', resolveGrammar: () => cyclic }).parseTree('π');
  assert.equal(outcome.ok, false);
  assert.equal(outcome.rejection.reason, 'nonConsumingEmbeddingCycle');
  assert.deepEqual(outcome.rejection.diagnostics[0], { reason: 'nonConsumingEmbeddingCycle', language: 'cycle', start: 0, end: 2 });
  const recursive = authoredGrammar(`(choice unordered ${literal('x')} (seq ${literal('{')} ${select('any-grammar', ['recursive'])} ${literal('}')}))`);
  const parser = compileGrammar(recursive, { grammarName: 'recursive', resolveGrammar: () => recursive });
  for (const source of ['x', '{x}', '{{{x}}}']) {
    const result = parser.parseTree(source);
    assert.equal(result.ok, true, JSON.stringify(result.rejection));
    assert.deepEqual(reconstructBytes(result.tree), [...encoder.encode(source)]);
  }
});

test('all recursive selections spend the same step and memory budgets and preserve unchanged limit diagnostics', () => {
  const { parser } = authoredChain();
  const original = Executor.prototype.step, budgets = new Set(), memories = new Set();
  let result;
  Executor.prototype.step = function () { budgets.add(this.budget); memories.add(this.budget.memory); return original.call(this); };
  try { result = parser.parseTree('m{j{d{m{π}}}}'); }
  finally { Executor.prototype.step = original; }
  assert.equal(result.ok, true, JSON.stringify(result.rejection));
  assert.equal(budgets.size, 1);
  assert.equal(memories.size, 1);
  for (const [options, reason] of [[{ stepLimit: 0 }, 'stepLimit'], [{ memoryLimit: 0 }, 'memoryBudget'], [{ maxDepth: 0 }, 'nestingDepth']]) {
    const limited = parser.parseTree('m{j{d{m{π}}}}', options);
    assert.equal(limited.ok, false);
    assert.equal(limited.rejection.reason, reason);
    assert.equal(limited.rejection.limit, 0);
  }
});

test('global provenance refuses changed grammar definitions instead of borrowing a known concept identity', () => {
  const { grammars, space } = authoredChain();
  grammars.set('Python', authoredGrammar(literal('x')));
  const parser = compileGrammar(grammars.get('Markdown'), { resolveGrammar: name => grammars.get(name), globalConceptSpace: space });
  const result = parser.parseTree('m{x}');
  assert.equal(result.ok, false);
  assert.equal(result.rejection.reason, 'unprovedGrammarOrigin');
  const python = embeddedTrees(result.tree)[0];
  assert.equal(python.selection.provenance.rules.find(rule => rule.rule === 'document').concept, null);
  grammars.get('Python').rule('document').expression.value = 'π';
  const repeated = parser.parseTree('m{x}');
  assert.equal(repeated.rejection.reason, 'unprovedGrammarOrigin', 'mutating a caller-owned source object cannot counterfeit the already compiled definition');
  assert.equal(embeddedTrees(repeated.tree)[0].selection.provenance.rules.find(rule => rule.rule === 'document').concept, null);
  const altered = authoredGrammar(literal('π'));
  altered.rules.set('document', { ...altered.rule('document'), concept: 'fixture.changed-meaning' });
  grammars.set('Python', altered);
  const changedMeaning = compileGrammar(grammars.get('Markdown'), { resolveGrammar: name => grammars.get(name), globalConceptSpace: space }).parseTree('m{π}');
  assert.equal(changedMeaning.rejection.reason, 'unprovedGrammarOrigin', 'an equal syntax definition cannot erase a changed declared meaning');
  assert.deepEqual(changedMeaning.rejection.diagnostics[0], { reason: 'unprovedGrammarOrigin', language: 'Python', start: 2, end: 4, rule: 'document' });
  const contextual = authoredGrammar(literal('π'));
  contextual.declarations = { ...contextual.declarations, matching: 'peg' };
  grammars.set('Python', contextual);
  const changedContext = compileGrammar(grammars.get('Markdown'), { resolveGrammar: name => grammars.get(name), globalConceptSpace: space }).parseTree('m{π}');
  assert.equal(changedContext.rejection.reason, 'unprovedGrammarOrigin', 'a changed matching policy cannot redefine the stored source implicit default');
  assert.equal(embeddedTrees(changedContext.tree)[0].selection.provenance.rules[0].concept, null);
});

test('portable selection decisions reject unknown modes, zero witnesses and unimplemented mappings', () => {
  for (const mode of ['any-grammar', 'all-grammar']) assert.equal(acceptGrammarSelectionMode(mode), true);
  assert.equal(acceptGrammarSelectionMode('first-guess'), false);
  assert.equal(acceptGrammarSelectionMapping('identity'), true);
  assert.equal(acceptGrammarSelectionMapping('quoted-escape'), false);
  for (const [mode, accepted, total, expected] of [['any-grammar', 0, 2, false], ['any-grammar', 1, 2, true], ['all-grammar', 1, 2, false], ['all-grammar', 2, 2, true], ['unknown', 2, 2, false], ['all-grammar', 0, 0, false]]) assert.equal(acceptGrammarSelectionCount(mode, accepted, total), expected);
  assert.equal(reportGrammarSelectionAmbiguity('any-grammar', 2), true);
  assert.equal(reportGrammarSelectionAmbiguity('all-grammar', 2), false);
});
