import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { mkdtempSync, mkdirSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import test from 'node:test';
import { applyRecordedCompilerFeedback, readCompilerFeedbackCorrections, retainCompilerFeedback } from '../scripts/compiler-style-feedback.mjs';
import { readProjectData, readProjectFileBytes, writeProjectData } from '../scripts/project-data.mjs';
import { DecoratorSet } from '../src/decorators.js';

const sha256 = (source) => createHash('sha256').update(source).digest('hex');

test('retained compiler feedback stores inspectable LiNo without altering diagnostic or source provenance', () => {
  const root = mkdtempSync(join(tmpdir(), 'meta-language-compiler-feedback-storage-'));
  try {
    const source = '// α byte offsets\nfn sample() -> i32 { 2 }\n';
    const path = join(root, 'generated.rs');
    const relativePath = 'rust/src/generated.rs';
    writeFileSync(path, source);
    const diagnostics = [{ reason: 'compiler-message', message: {
      level: 'error', code: { code: 'E0308', explanation: null },
      message: 'mismatched types', spans: [{ file_name: path, byte_start: 0,
        byte_end: 0, suggestion_applicability: null, suggested_replacement: null }], children: [],
    } }];
    const actual = retainCompilerFeedback({ root, group: 'frontend-rules', round: 0,
      stderr: diagnostics.map(item => JSON.stringify(item)).join('\n') + '\n', inputs: [{ path, relativePath }] });
    assert.deepEqual(actual, diagnostics);
    const feedbackFile = join(root, 'issue-195-artifacts/compiler-feedback/frontend-rules/round-0.json');
    const stored = readProjectData(feedbackFile);
    assert.deepEqual(stored.diagnostics, diagnostics);
    assert.deepEqual(stored.sources[path], { relativePath, sha256: sha256(source), text: source });
    assert.equal(stored.compiler, 'clippy-driver');
    assert.ok(readProjectFileBytes(feedbackFile).includes(Buffer.from('(term: structured%20data)')),
      'the retained snapshot has the structured data graph root');
    assert.equal(readCompilerFeedbackCorrections({ feedback: stored, allowedPaths: [relativePath] }).actions.length, 0,
      'storing genuine diagnostics cannot promote an unproven suggestion');
    retainCompilerFeedback({ root, group: 'decimal-format-draft', round: 0, compiler: 'rustc',
      stderr: diagnostics.map(item => JSON.stringify(item)).join('\n') + '\n', inputs: [{ path, relativePath }] });
    const native = readProjectData(join(root, 'issue-195-artifacts/compiler-feedback/decimal-format-draft/round-0.lino'));
    assert.equal(native.compiler, 'rustc', 'native feedback must not claim that Clippy produced it');
    assert.deepEqual(native.diagnostics, diagnostics);
    assert.deepEqual(native.sources[path], stored.sources[path]);
    assert.throws(() => retainCompilerFeedback({ root, group: 'unknown', round: 0, compiler: 'invented',
      stderr: '', inputs: [] }), /actual compiler kind/u);
  } finally { rmSync(root, { recursive: true, force: true }); }
});

test('compiler feedback replays exact byte ranges in unique contexts and refuses stale or unproven suggestions', () => {
  const root = mkdtempSync(join(tmpdir(), 'meta-language-compiler-feedback-test-'));
  try {
    mkdirSync(join(root, 'rust/src'), { recursive: true });
    const relativePath = 'rust/src/generated.rs';
    const source = '// α byte offsets\nfn first() {\n    let marker = true;\n}\nfn second() {\n    let marker = true;\n}\n';
    writeFileSync(join(root, relativePath), source);
    writeFileSync(join(root, 'styles.lino'), '');
    const start = Buffer.from(source).indexOf(Buffer.from('true'));
    const span = { file_name: '/ci/generated.rs', byte_start: start, byte_end: start + 4,
      suggested_replacement: 'false', suggestion_applicability: 'MachineApplicable' };
    const feedback = { schemaVersion: 1,
      diagnostics: [{ spans: [], children: [{ spans: [span], children: [] }] }],
      sources: { '/ci/generated.rs': { relativePath, sha256: sha256(source), text: source } } };
    const path = join(root, 'feedback.json');
    const apply = () => applyRecordedCompilerFeedback({ root, feedbackFile: path,
      decoratorFile: 'styles.lino', allowedPaths: [relativePath] });
    writeProjectData(path, feedback);
    apply();
    const styles = DecoratorSet.fromLino(readFileSync(join(root, 'styles.lino'), 'utf8'));
    const changed = styles.decorate('emitter', { format: 'Rust', scope: 'formatted-module', source }).source;
    assert.equal(changed, source.replace('true', 'false'));
    assert.match(changed, /fn second\(\) \{\n    let marker = true;/u, 'the identical line in another function stays unchanged');
    assert.equal(styles.decorate('emitter', { format: 'Rust', scope: 'formatted-module', source: changed }).source, changed, 'the recorded replacement converges');
    writeFileSync(join(root, relativePath), source + '// changed\n');
    assert.throws(apply, /stale/u);
    writeFileSync(join(root, relativePath), source);
    feedback.sources['/ci/generated.rs'].sha256 = '0'.repeat(64);
    writeProjectData(path, feedback);
    assert.throws(apply, /stale/u);
    feedback.sources['/ci/generated.rs'].sha256 = sha256(source);
    span.suggestion_applicability = 'MaybeIncorrect';
    writeProjectData(path, feedback);
    assert.throws(apply, /no applicable machine suggestions/u);
    feedback.diagnostics = [{ rendered: 'help: replace true with false', loggedMachineApplicability: false }];
    writeProjectData(path, feedback);
    assert.throws(apply, /no applicable machine suggestions/u);
    assert.equal(readFileSync(join(root, relativePath), 'utf8'), source, 'offline replay records data and never edits generated Rust');
  } finally { rmSync(root, { recursive: true, force: true }); }
});

test('compiler feedback keeps multipart suggestions atomic when nested alternatives overlap', () => {
  const root = mkdtempSync(join(tmpdir(), 'meta-language-compiler-feedback-multipart-'));
  try {
    mkdirSync(join(root, 'rust/src'), { recursive: true });
    const relativePath = 'rust/src/generated.rs';
    const source = 'fn sample() -> i32 {\n    let result = 1 + (2);\n    result\n}\n';
    const bytes = Buffer.from(source);
    const span = (start, end, replacement) => ({ file_name: '/ci/generated.rs',
      byte_start: start, byte_end: end, suggested_replacement: replacement,
      suggestion_applicability: 'MachineApplicable' });
    const bindingStart = bytes.indexOf(Buffer.from('let result'));
    const bindingEnd = bytes.indexOf(Buffer.from(';')) + 1;
    const returnStart = bytes.lastIndexOf(Buffer.from('result'));
    const parenthesesStart = bytes.indexOf(Buffer.from('(2)'));
    const multipart = { spans: [span(bindingStart, bindingEnd, ''),
      span(returnStart, returnStart + 6, '1 + (2)')], children: [] };
    const nested = { spans: [span(parenthesesStart, parenthesesStart + 3, '2')], children: [] };
    for (const [diagnostics, expected] of [
      [[multipart, nested], 'fn sample() -> i32 {\n\n    1 + (2)\n}\n'],
      [[nested, multipart], source.replace('(2)', '2')],
    ]) {
      writeFileSync(join(root, relativePath), source);
      writeFileSync(join(root, 'styles.lino'), '');
      const feedbackFile = join(root, 'feedback.json');
      writeProjectData(feedbackFile, { schemaVersion: 1, diagnostics,
        sources: { '/ci/generated.rs': { relativePath, sha256: sha256(source), text: source } } });
      const result = applyRecordedCompilerFeedback({ root, feedbackFile,
        decoratorFile: 'styles.lino', allowedPaths: [relativePath] });
      const styles = DecoratorSet.fromLino(readFileSync(join(root, 'styles.lino'), 'utf8'));
      assert.equal(styles.decorate('emitter', { format: 'Rust', scope: 'formatted-module', source }).source, expected);
      assert.equal(result.deferred, 1, 'the conflicting complete suggestion is deferred');
      if (diagnostics[0] === multipart) {
        const misaligned = source.replace('let result =', 'let result  =');
        assert.equal(styles.decorate('emitter', { format: 'Rust', scope: 'formatted-module', source: misaligned }).source, misaligned,
          'a changed binding context refuses the entire multipart replacement');
        assert.equal(result.actions, 1, 'a multipart suggestion remains one literal transaction');
      }
    }
  } finally { rmSync(root, { recursive: true, force: true }); }
});

test('a unique body correction still retains its complete owning signature', () => {
  const relativePath = 'rust/src/generated.rs';
  const source = 'pub fn borrowed(\n    units: &[f64],\n) -> Option<f64> {\n    optional_at(&units, 0)\n}\n';
  const start = Buffer.from(source).indexOf(Buffer.from('&units'));
  const feedback = { schemaVersion: 1,
    diagnostics: [{ spans: [{ file_name: '/ci/generated.rs', byte_start: start, byte_end: start + 1,
      suggested_replacement: '', suggestion_applicability: 'MachineApplicable' }] }],
    sources: { '/ci/generated.rs': { relativePath, sha256: sha256(source), text: source } } };
  const { actions } = readCompilerFeedbackCorrections({ feedback, allowedPaths: [relativePath] });
  assert.equal(actions.length, 1);
  assert.ok(actions[0].from.startsWith('pub fn borrowed(\n    units: &[f64],\n) -> Option<f64> {\n'));
  const styles = new DecoratorSet([{ id: 'ownership-regression', level: 'emitter', actions }]);
  const changed = styles.decorate('emitter', { source }).source;
  assert.equal(changed, source.replace('&units', 'units'));
  const owned = source.replace('units: &[f64]', 'units: Vec<f64>');
  assert.equal(styles.decorate('emitter', { source: owned }).source, owned,
    'the same function body with an owned parameter must retain its borrow');
  const other = owned.replace('fn borrowed(', 'fn owned(');
  assert.equal(styles.decorate('emitter', { source: other + source }).source, other + changed,
    'adding an identical body in another owned function cannot acquire the old correction');
});

test('compiler corrections refuse byte ranges that split UTF-8 source characters', () => {
  const source = '// α\nfn sample() {}\n';
  const start = Buffer.from(source).indexOf(Buffer.from('α')) + 1;
  const feedback = { schemaVersion: 1,
    diagnostics: [{ spans: [{ file_name: '/ci/generated.rs', byte_start: start, byte_end: start,
      suggested_replacement: 'x', suggestion_applicability: 'MachineApplicable' }] }],
    sources: { '/ci/generated.rs': { relativePath: 'rust/src/generated.rs', sha256: sha256(source), text: source } } };
  assert.throws(() => readCompilerFeedbackCorrections({ feedback, allowedPaths: ['rust/src/generated.rs'] }), /UTF-8 source boundary/u);
});

test('several signature edits preserve full contexts after descending byte offsets shift', () => {
  const source = 'pub fn sample(\n    first: i64,\n    mut units: &[f64],\n) -> Option<f64> {\n    optional_at(units, 0)\n}\n';
  const relativePath = 'rust/src/generated.rs';
  const bytes = Buffer.from(source);
  const span = (from, to) => {
    const start = bytes.indexOf(Buffer.from(from));
    return { file_name: '/ci/generated.rs', byte_start: start, byte_end: start + Buffer.byteLength(from),
      suggested_replacement: to, suggestion_applicability: 'MachineApplicable' };
  };
  const feedback = { schemaVersion: 1,
    diagnostics: [{ spans: [span('i64', 'i32')] }, { spans: [span('mut ', '')] }],
    sources: { '/ci/generated.rs': { relativePath, sha256: sha256(source), text: source } } };
  const { actions } = readCompilerFeedbackCorrections({ feedback, allowedPaths: [relativePath] });
  assert.equal(actions.length, 2);
  assert.ok(actions.every(action => action.from.startsWith('pub fn sample(\n')));
  assert.ok(actions.every(action => action.from.endsWith(') -> Option<f64> {')), 'the complete signatures use adjusted source offsets');
  const styles = new DecoratorSet([{ id: 'signature-offsets', level: 'emitter', actions }]);
  assert.equal(styles.decorate('emitter', { source }).source, source.replace('i64', 'i32').replace('mut ', ''));
});

test('the retained CI borrow correction is machine-applicable and bound to the owned Unicode decoder', () => {
  const feedbackFile = new URL('../../parity/self-translation/ci-38240388521-frontend-rules-feedback.json', import.meta.url);
  const raw = readProjectFileBytes(feedbackFile);
  const review = readProjectData(new URL('../../parity/self-translation/ci-38240388521-frontend-rule-style-correction.json', import.meta.url));
  assert.equal(sha256(raw), review.compilerFeedbackSha256);
  assert.equal(review.kind, 'reviewed decorator collision correction');
  const feedback = readProjectData(feedbackFile);
  assert.equal(feedback.runId, '38240388521');
  assert.equal(feedback.head, '688b2f376011d1504623c422c734fc26135f0f66');
  const { actions, spans } = readCompilerFeedbackCorrections({ feedback,
    allowedPaths: Object.values(feedback.sources).map(source => source.relativePath) });
  assert.equal(spans, 1);
  assert.equal(actions.length, 1);
  assert.ok(actions[0].from.startsWith('pub fn decode_unicode_escape(\n    units: Vec<f64>,\n'));
  assert.ok(actions[0].to.includes('optional_at(\n                        &units,'));
  const original = Object.values(feedback.sources).find(source => source.relativePath === 'rust/src/translation/frontend_rules.rs').text;
  const styles = new DecoratorSet([{ id: 'actual-borrow-correction', level: 'emitter', actions }]);
  const changed = styles.decorate('emitter', { source: original }).source;
  assert.notEqual(changed, original);
  const differentOwnership = original.replace('pub fn decode_unicode_escape(\n    units: Vec<f64>,', 'pub fn decode_unicode_escape(\n    units: &[f64],');
  assert.equal(styles.decorate('emitter', { source: differentOwnership }).source, differentOwnership);
  const recorded = DecoratorSet.fromLino(readFileSync(new URL('../../parity/self-translation/frontend-rules-decorators.lino', import.meta.url), 'utf8'));
  assert.ok(!recorded.decorators.find(entry => entry.id === 'compiler-feedback-style').actions
    .some(action => action.from === review.removedAction.from && action.to === review.removedAction.to));
});

test('reviewed slice corrections cannot remove borrows after the parameter becomes owned', () => {
  const review = readProjectData(new URL('../../parity/self-translation/ci-38240388521-frontend-rule-style-correction.json', import.meta.url));
  const actions = review.reviewedStyleActions.filter(action => action.from.startsWith('pub fn ml_read_rust_number_bounds_'));
  assert.equal(actions.length, 3);
  const styles = new DecoratorSet([{ id: 'reviewed-slice-regression', level: 'emitter', actions }]);
  for (const action of actions) {
    assert.ok(action.from.includes('(units: &[f64],'));
    assert.equal(styles.decorate('emitter', { source: action.from }).source, action.to);
    const owned = action.from.replace('(units: &[f64],', '(units: Vec<f64>,');
    assert.equal(styles.decorate('emitter', { source: owned }).source, owned);
  }
});
