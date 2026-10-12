#!/usr/bin/env node
import { structuredDataToLino } from '../src/structured-data-links.js';
import { readProjectData, resolveProjectDataPath } from './project-data.mjs';
// Three reported syntax failures, retained verbatim with bounded controls.
import assert from 'node:assert/strict';
import { readFileSync, writeFileSync } from 'node:fs';
import { grammarSourceOf, corpusCases } from './import-native-grammars.mjs';
import { oracleSnapshot } from './native-grammar-rows.mjs';

const target = new URL('../../parity/fixtures/native-grammars/ruby-expression-syntax-ci.json', import.meta.url);
const fixture = readProjectData(target);
const naming = readProjectData(new URL('../../parity/naming/grammar-name-expansions.json', import.meta.url));
for (const decision of fixture.decisions) assert.ok(naming.grammars.ruby.sourceTransforms.some(value => JSON.stringify(value) === JSON.stringify(decision)), 'every expression decision is registered source data');
const entry = grammarSourceOf('native-ruby');
const corpus = corpusCases(entry);
const snapshot = item => {
  const oracle = oracleSnapshot(item.source, fixture.language);
  assert.equal(oracle.recovers, false, item.title ?? item.source);
  return { ...item, rows: oracle.rows };
};
const cases = fixture.selections.map(selection => {
  const item = corpus.find(value => value.file === selection.file && value.title === selection.title);
  assert.ok(item, 'every selected failure is retained verbatim from the pinned corpus');
  return snapshot(item);
});
const controls = fixture.controls.map(({ source }) => snapshot({ source }));
for (const { source } of fixture.rejections) assert.equal(oracleSnapshot(source, fixture.language).recovers, true, source);
const result = { ...fixture, source: entry.id, revision: entry.revision, sourceSha256: entry.sha256, corpusSha256: entry.corpus.sha256, cases, controls };
const rendered = structuredDataToLino(result);
if (process.argv.includes('--check')) assert.equal(readFileSync(resolveProjectDataPath(target), 'utf8'), rendered, 'Ruby expression syntax fixture drift');
else writeFileSync(resolveProjectDataPath(target), rendered);
