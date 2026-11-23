import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { DecoratorSet } from '../src/decorators.js';
import { readProjectData } from './project-data.mjs';
import { formatProductRuleSource } from './product-rule-style-corrections.mjs';

const review = new URL('../../parity/self-translation/native-source-style-review-137557959068.lino', import.meta.url);
let modules;

// Keep the existing checked generation and apply only the reviewed edits to
// the exact Rust partition diagnosed by CI. The review does not attest to a
// successful native build or runtime execution.
export function applyReviewedNativeSourceStyle(path, source) {
  if (!modules) {
    const { schemaVersion, reviewKind, diagnostics, files } = readProjectData(review);
    assert.equal(schemaVersion, 1);
    assert.equal(reviewKind, 'Reviewed rendered CI diagnostics. No machine applicability metadata was reported or invented.');
    const owners = new Map(diagnostics.map((diagnostic) => [diagnostic.logLine, diagnostic.path]));
    assert.equal(owners.size, diagnostics.length, 'each diagnostic occupies one line in the captured log');
    const reviewedModules = new Map(files.map((file) => [file.path, file]));
    assert.equal(reviewedModules.size, files.length, 'each source partition has one review');
    const cited = new Set();
    for (const file of files) {
      assert.ok(file.edits.length, `empty review: ${file.path}`);
      for (const edit of file.edits) {
        assert.equal(owners.get(edit.logLine), file.path, 'the cited diagnostic belongs to the edited partition');
        cited.add(edit.logLine);
      }
    }
    assert.equal(cited.size, owners.size, 'every reviewed diagnostic has a correction');
    modules = reviewedModules;
  }
  const file = modules.get(path);
  if (!file) return source;
  assert.equal(createHash('sha256').update(source).digest('hex'), file.sha256, `reviewed source is stale: ${path}`);

  let end = 0;
  const fragments = [];
  for (const edit of file.edits) {
    assert.ok(Number.isSafeInteger(edit.offset) && edit.offset >= end && edit.from.length > 0, `overlapping reviewed edit: ${path}`);
    assert.equal(source.slice(edit.offset, edit.offset + edit.from.length), edit.from, `the reviewed span must match exactly: ${path}`);
    fragments.push(source.slice(end, edit.offset), edit.to);
    end = edit.offset + edit.from.length;
  }
  fragments.push(source.slice(end));
  const decorators = new DecoratorSet([{
    id: 'reviewed-native-source-style', level: 'emitter', order: 0,
    when: [['format', 'Rust'], ['path', path]],
    actions: [{ op: 'replace', field: 'source', from: source, to: fragments.join('') }],
  }]);
  return formatProductRuleSource(decorators.decorate('emitter', { format: 'Rust', path, source }).source);
}
