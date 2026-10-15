import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {existsSync, readFileSync} from 'node:fs';
import test from 'node:test';
import {readProjectData} from '../scripts/project-data.mjs';
import {readCompilerFeedbackCorrections} from '../scripts/compiler-style-feedback.mjs';

const root = new URL('../../', import.meta.url);
const receipt = readProjectData(new URL('parity/self-translation/ci-38698267304-reviewed-diagnostics.lino', root));

test('reviewed bulk twenty diagnostics retain the exact failed source and no invented machine suggestions', () => {
  assert.equal(createHash('sha256').update(readFileSync(new URL('parity/self-translation/ci-38698267304-reviewed-diagnostics.lino', root))).digest('hex'), '32d3b46dcd3e38a5241ea843a7c4dcced520012a153e43dd5d4222b49851ec5d');
  assert.equal(receipt.head, '21b09298ea44723fc79f8621ec6e22dcc15c9d37');
  assert.equal(receipt.execution, '50bb2b4d0f33fc1ab5ace38dc6d2029c6167c2e0');
  assert.equal(receipt.diagnostics.length, 18);
  assert.equal(receipt.sources.length, 15);
  assert.deepEqual(receipt.logs.map(log => log.sha256), [
    'c946c1b77cb3d3aa822d8ee4ad1ebc21f7a88677a2534b141ae02fb3b4cf9a8d',
    '3b7a9c5d9944b17c8e15ce48f66894a6f20df7216dcbb0ad17533a00dd07994d',
  ]);
  for (const source of receipt.sources) {
    const bytes = Buffer.from(source.text, 'utf8');
    assert.equal(source.bytes, bytes.length);
    assert.equal(source.sha256, createHash('sha256').update(bytes).digest('hex'));
    assert.equal(source.text, bytes.toString('utf8'));
  }
  const corrections = readCompilerFeedbackCorrections({feedback: receipt, allowedPaths: receipt.sources.map(source => source.path)});
  assert.deepEqual(corrections.actions, []);
  assert.equal(corrections.spans, 0);
  assert.match(receipt.formatDiagnostic, /failed to resolve mod `strings`/u);
});

test('the actual frontend rules imported by the test shim resolve every generated child explicitly', () => {
  const file = new URL('rust/src/translation/frontend_rules.rs', root);
  const source = readFileSync(file, 'utf8');
  const children = [...source.matchAll(/(?:#\[path = "([^"]+)"\]\n)?mod (\w+);/gu)];
  assert.deepEqual(children.map(match => match[2]), ['strings', 'rust_strings', 'continuations', 'continuations_2']);
  for (const [, path, name] of children) {
    assert.equal(path, `frontend_rules/${name}.rs`);
    assert.ok(existsSync(new URL(path, file)), `the actual generated ${name} child exists`);
  }
});
