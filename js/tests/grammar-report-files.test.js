import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { existsSync, mkdtempSync, readFileSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import test from 'node:test';
import peggy from 'peggy';
import { checkIndependentGrammarCycle } from '../scripts/grammar-interchange-tools.mjs';
import { writeGrammarReport } from '../scripts/grammar-report-files.mjs';
import { readProjectData, readProjectFileBytes } from '../scripts/project-data.mjs';
import { createPipelineGlobalConceptSpace } from '../scripts/run-grammar-bulk-pipeline.mjs';
import { deserializeGrammar } from '../src/grammar.js';
import { renderGrammarLinks } from '../src/grammar-links.js';
import { globalConceptSpaceToLinks, readGlobalConceptSpaceLinks } from '../src/global-concept-space.js';

const digest = value => createHash('sha256').update(value).digest('hex');
const read = file => readProjectData(new URL(`../../${file}`, import.meta.url));

function directory(run) {
  const root = mkdtempSync(join(tmpdir(), 'grammar-report-links-'));
  try { run(root); }
  finally { rmSync(root, { recursive: true, force: true }); }
}

test('a persisted Links report retains the authentic independent target observations and source hashes', () => directory(root => {
  const fixtureFile = new URL('../../parity/fixtures/grammar-interchange-tools.json', import.meta.url);
  const fixture = readProjectData(fixtureFile);
  // One existing fixture runs its actual independent compiler and parser.
  const entry = fixture.cases[0];
  const result = checkIndependentGrammarCycle(entry, peggy);
  assert.deepEqual(result.failures, []);
  assert.equal(result.sourceSha256, digest(entry.source));
  const report = { schemaVersion: 1,
    commit: execFileSync('git', ['rev-parse', 'HEAD'], { encoding: 'utf8' }).trim(),
    fixtureSha256: digest(readProjectFileBytes(fixtureFile)),
    targetTools: [{ name: 'peggy', version: peggy.VERSION }], results: [result] };
  const logical = join(root, 'grammar-interchange-tools.json');
  const written = writeGrammarReport(logical, report);
  assert.equal(written, join(root, 'grammar-interchange-tools.lino'));
  assert.equal(existsSync(logical), false);
  assert.match(readFileSync(written, 'utf8'), /^\(/u);
  assert.deepEqual(readProjectData(logical), report);
  assert.equal(readProjectData(written).results[0].mutation.originalWitnessRejectedByTarget, true);
}));

test('structured concept report data and the native concept model keep distinct linked representations', () => directory(root => {
  const fixture = read('parity/fixtures/global-concept-space.json');
  const sources = fixture.sources.map(({ grammar, ...source }) => ({ ...source,
    grammarLinks: renderGrammarLinks(deserializeGrammar(grammar)) }));
  const space = createPipelineGlobalConceptSpace(sources, {
    records: fixture.records, equivalences: fixture.equivalences, formalDefinitions: fixture.formalDefinitions,
  });
  const data = writeGrammarReport(join(root, 'global-concept-space-data.lino'), space);
  assert.deepEqual(readProjectData(data), space);
  const native = globalConceptSpaceToLinks(space);
  assert.deepEqual(readGlobalConceptSpaceLinks(native, { records: fixture.records }), space);
  assert.notEqual(readFileSync(data, 'utf8'), native);
}));
