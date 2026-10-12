import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { mkdtempSync, mkdirSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import test from 'node:test';
import { applyRecordedCompilerFeedback } from '../scripts/compiler-style-feedback.mjs';
import { DecoratorSet } from '../src/decorators.js';

const sha256 = (source) => createHash('sha256').update(source).digest('hex');

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
    writeFileSync(path, JSON.stringify(feedback));
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
    writeFileSync(path, JSON.stringify(feedback));
    assert.throws(apply, /stale/u);
    feedback.sources['/ci/generated.rs'].sha256 = sha256(source);
    span.suggestion_applicability = 'MaybeIncorrect';
    writeFileSync(path, JSON.stringify(feedback));
    assert.throws(apply, /no applicable machine suggestions/u);
    feedback.diagnostics = [{ rendered: 'help: replace true with false', loggedMachineApplicability: false }];
    writeFileSync(path, JSON.stringify(feedback));
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
      writeFileSync(feedbackFile, JSON.stringify({ schemaVersion: 1, diagnostics,
        sources: { '/ci/generated.rs': { relativePath, sha256: sha256(source), text: source } } }));
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
