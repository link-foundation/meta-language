import { readProjectData, resolveProjectDataPath } from '../scripts/project-data.mjs';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import test from 'node:test';
import { elmLayoutCandidate } from '../scripts/elm-layout-candidate.mjs';
import { elmCandidateOutputs } from '../scripts/generate-elm-candidate-fixtures.mjs';
import { sectionLayoutScanner } from '../scripts/section-layout-scanner.mjs';
import { compileGrammar, parseGrammarLinks, renderGrammarLinks } from '../src/index.js';
import { nativeRows, oracleSnapshot } from '../scripts/native-grammar-rows.mjs';

const read = (file) => readFileSync(resolveProjectDataPath(new URL(`../../${file}`, import.meta.url)), 'utf8');
const readData = (file) => readProjectData(new URL(`../../${file}`, import.meta.url));
const candidate = elmLayoutCandidate();
const fixture = readData('parity/fixtures/native-grammars/elm-candidate.json');
const parser = compileGrammar(candidate.grammar);
const text = (tree) => tree.type === 'node' ? tree.children.map(text).join('') : tree.text;

test('Elm candidate covers every pinned external through generated shared Links operations', () => {
  assert.deepEqual(candidate.imported.report.unsupported, []);
  assert.deepEqual(candidate.imported.report.approximations, []);
  assert.equal(candidate.grammar.externalTokens().length, 7);
  assert.equal(candidate.descriptor.maximumDepth, 256);
  assert.equal(candidate.descriptor.indentationModulo, 256);
  assert.equal(candidate.scanner, sectionLayoutScanner(candidate.descriptor));
  assert.equal(renderGrammarLinks(parseGrammarLinks(candidate.text)), candidate.text);
  for (const [file, generated] of elmCandidateOutputs()) assert.equal(read(file), generated, file);
});

test('Elm focused layout queues and lexical content match independent pinned concrete trees', () => {
  for (const entry of fixture.matches) {
    const oracle = oracleSnapshot(entry.source, 'Elm');
    assert.equal(oracle.recovers, false, entry.name);
    assert.deepEqual(oracle.rows, entry.rows, entry.name);
    const outcome = parser.parseTree(entry.source);
    assert.equal(outcome.ok, true, entry.name);
    assert.deepEqual(outcome.ambiguities, [], entry.name);
    assert.deepEqual(nativeRows(outcome.tree, entry.source, fixture), entry.rows, entry.name);
    assert.equal(text(outcome.tree), entry.source, entry.name);
  }
});

test('Elm invalid layout bodies and truncated lexical delimiters are rejected', () => {
  for (const entry of fixture.rejections) {
    assert.equal(oracleSnapshot(entry.source, 'Elm').recovers, true, entry.name);
    assert.equal(parser.parseTree(entry.source).ok, false, entry.name);
  }
});

test('section layout descriptors reject invalid widths, bounds and overlapping tokens', () => {
  for (const change of [
    { maximumDepth: 1 }, { indentationModulo: 0 },
    { sectionEndWord: 'after' }, { commentOpening: '{' },
    { multilineClosing: '"\'"' }, { sectionClosers: [] },
    { endDeclarationToken: candidate.descriptor.openSectionToken },
  ]) assert.throws(() => sectionLayoutScanner({ ...candidate.descriptor, ...change }), TypeError);
});
