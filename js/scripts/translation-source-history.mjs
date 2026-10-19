// Exact source inverses keep historical assertions pinned to their own inputs.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {readProjectData} from './project-data.mjs';

const historyFile = new URL('../../parity/self-translation/translation-source-history.lino', import.meta.url);
const historyHash = '0d97c149fb8e5d8cdb3036631c5da0d873e46b97a2269f22874d582e506a853a';
const digest = value => createHash('sha256').update(value).digest('hex');
let history;

export function restoreTranslationSourceHistory(source, file, revision) {
  assert.equal(typeof source, 'string', 'a complete source string is required');
  assert.ok(Buffer.byteLength(source) <= 1024 * 1024, 'bounded complete source required');
  if (!history) {
    const bytes = readFileSync(historyFile);
    assert.equal(digest(bytes), historyHash, 'the whole reviewed inverse policy remains exact');
    history = readProjectData(historyFile);
    assert.equal(history.schemaVersion, 1);
    assert.equal(history.sources.length, 6);
  }
  const entries = history.sources.filter(row => row.file === file && row.revision === revision);
  assert.equal(entries.length, 1, 'one exact recorded source and predecessor revision are required');
  const entry = entries[0];
  let bytes = Buffer.from(source);
  assert.equal(bytes.toString('utf8'), source, 'complete source retains valid UTF8');
  if (digest(bytes) === entry.previous.sha256) {
    assert.equal(bytes.length, entry.previous.bytes);
    assert.equal(source, entry.previous.sourceText);
    return source;
  }
  assert.equal(bytes.length, entry.current.bytes, 'unreviewed current source changes refuse');
  assert.equal(digest(bytes), entry.current.sha256, 'the complete current source must match its reviewed input');
  let limit = bytes.length;
  for (const edit of entry.edits.toReversed()) {
    assert.ok(Number.isSafeInteger(edit.start) && Number.isSafeInteger(edit.end)
      && edit.start >= 0 && edit.end >= edit.start && edit.end <= limit, 'ordered disjoint source intervals required');
    assert.equal(bytes.subarray(edit.start, edit.end).toString('utf8'), edit.currentText);
    bytes = Buffer.concat([bytes.subarray(0, edit.start), Buffer.from(edit.previousText), bytes.subarray(edit.end)]);
    limit = edit.start;
  }
  assert.equal(bytes.length, entry.previous.bytes, 'the complete original source byte count is restored');
  assert.equal(digest(bytes), entry.previous.sha256, 'every original source byte and context is restored');
  assert.equal(bytes.toString('utf8'), entry.previous.sourceText);
  return bytes.toString('utf8');
}
