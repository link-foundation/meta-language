import { readProjectData, resolveProjectDataPath } from '../scripts/project-data.mjs';

import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { format } from 'node:util';
import { test } from 'node:test';
import { runInNewContext } from 'node:vm';
import { translateProgram } from '../src/program-translation.js';
import { runTestCommand } from './support/child-test-output.js';

test('failed evidence child commands retain both output streams in the test error', () => {
  assert.throws(() => runTestCommand(process.execPath, ['-e',
    'console.log("inner assertion details");console.error("inner diagnostic");process.exit(1);',
  ], { stdio: 'pipe' }), (error) => {
    assert.match(error.message, /Child stdout:\ninner assertion details/u);
    assert.match(error.message, /Child stderr:\ninner diagnostic/u);
    assert.equal(error.cause.status, 1);
    return true;
  });
  assert.equal(runTestCommand(process.execPath, ['-e', 'console.log("observed output");'], { stdio: 'pipe' }), 'observed output\n');
});

test('Rust corpus constructor mappings name the serialized JavaScript tags and preserve output', () => {
  const corpus = readProjectData(new URL('../../parity/fixtures/four-language-conformance.json', import.meta.url)).translationCorpus;
  const read = (file) => readFileSync(resolveProjectDataPath(new URL(`../../${corpus.directory}/${file}`, import.meta.url)), 'utf8');
  const source = read(corpus.sources.Rust.file);
  const result = translateProgram(source, 'Rust', 'JavaScript');
  assert.equal(result.diagnostic, null);
  const constructors = result.semantics.mappings.filter(({ kind }) => kind === 'constructor');
  assert.ok(constructors.length > 0);
  for (const mapping of constructors) {
    assert.ok(result.code.includes(`$: ${JSON.stringify(mapping.target.split('.').at(-1))}`), mapping.target);
  }
  const output = [];
  runInNewContext(result.code, { console: { log: (...values) => output.push(format(...values)) }, process: { argv: [] } }, { timeout: 1000 });
  assert.equal(output.join('\n') + '\n', read(corpus.expectedOutput.file));
});
