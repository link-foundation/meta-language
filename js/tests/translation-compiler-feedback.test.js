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
