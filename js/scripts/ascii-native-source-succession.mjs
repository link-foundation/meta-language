import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { readProjectData, serializeProjectData } from './project-data.mjs';
import { asciiHostPin, replayAsciiHostEdits } from './ascii-text-predicate-host-history.mjs';
import { restoreRequiredFieldNativeHostSource } from './required-field-native-host-succession.mjs';

const root = new URL('../../', import.meta.url);
const expressionPath = 'rust/src/translation/rust/expressions.rs';
const castPath = 'rust/src/translation/rust/expressions/cast.rs';
const castImport = 'use super::cast_syntax_rules::{accept_rust_cast_following_token, read_rust_cast_syntax_message};\n';
const castWrapper = castImport + '\nimpl RustParser {\n';
const artifactPath = 'parity/self-translation/ascii-text-predicate-rules-native.lino';
const artifactSha256 = '2ada2dd04d5982b6f10f8cdd685999ba176e262a465803eecc2f794997dfd947';

/** Reassemble the exact original expression host before applying its archived pin. */
export function restoreAsciiNativeHostSource(source, path, cast = null) {
  if (path !== expressionPath || !source.includes('include!("expressions/cast.rs");')) {
    return restoreRequiredFieldNativeHostSource(source, path, 'ASCII character host');
  }
  cast ??= readFileSync(new URL(castPath, root), 'utf8');
  assert.ok(cast.startsWith(castWrapper) && cast.endsWith('\n}\n'));
  const methods = cast.slice(castWrapper.length, -2);
  const importAnchor = 'use super::indirection_rules::accept_rust_source_box_owner;\n';
  const methodAnchor = '    pub(super) fn unary(';
  return replayAsciiHostEdits(source, [
    { before: 'include!("expressions/cast.rs");\n\n', after: '' },
    { before: importAnchor, after: castImport + importAnchor },
    { before: methodAnchor, after: methods + '\n' + methodAnchor },
  ]).source;
}

/** Restore the reviewed lint edits for the immutable generated ABI receipt. */
export function restoreAsciiNativeArtifact(source, path) {
  if (path !== artifactPath || asciiHostPin(source).sha256 !== artifactSha256) return source;
  const file = new URL(path, root);
  const artifact = readProjectData(file);
  assert.equal(serializeProjectData(artifact, file), source);
  const partition = artifact.files.find(row => row.path === 'ascii_text_predicate_rules/part_2.rs');
  assert.ok(partition);
  // UTF-16 coordinates in the pinned partition. Apply each group from the
  // end of the source so preceding coordinates stay valid.
  const edits = [
    // The callback's empty-input guard.
    [5700, ' return', ''],
    [5725, ';', ''],
    [5732, '\n', ''],
    [5734, '  ', 'else'],
    // The loop's alternate branch.
    [16217, '\n', ''],
    [16219, ' '.repeat(14), 'else'],
    // The single-method vector and its original formatting.
    [28707, '', '\n' + ' '.repeat(72)],
    [28718, '', '.clone()'],
    [28720, '', '\n' + ' '.repeat(71)],
    [28728, '', ',\n' + ' '.repeat(68)],
    // The code point's clone and lazy fallback.
    [39535, '', ' '.repeat(32) + '.clone()\n'],
    [39577, '', '_else'],
    [39578, '', '|| '],
  ];
  for (const [start, current, previous] of edits.toReversed()) {
    const end = start + current.length;
    assert.equal(partition.source.slice(start, end), current);
    partition.source = partition.source.slice(0, start) + previous + partition.source.slice(end);
  }
  const { bytes, sha256 } = asciiHostPin(partition.source);
  partition.bytes = bytes;
  partition.sourceSha256 = sha256;
  return serializeProjectData(artifact, file);
}
