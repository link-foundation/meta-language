// Future controls only. Never run during the whole requirements draft.
import assert from 'node:assert/strict';
import test from 'node:test';
import { runInNewContext } from 'node:vm';
import { readProjectData } from '../scripts/project-data.mjs';
import { tokenize, SOURCE_IDENTIFIER_UNICODE_VERSION } from '../src/translation/lexer.js';
import { normalizeUnicodeText } from '../src/translation/unicode-identifiers.js';
import { decodeUnicodeSourceScalar } from '../src/translation/unicode-scalar-rules.js';
import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';
import { createUnicodeScalarProgramLinks } from '../src/unicode-scalar-program.js';
import { encodeTypedProgramLinks, emitTypedProgramLinks } from '../src/typed-program-links.js';
import { LinkNetwork } from '../src/network.js';
import { encodeLinkNetworkBinary, decodeLinkNetworkBinary } from '../src/link-network-binary.js';
const fixture = readProjectData(new URL('../../parity/fixtures/unicode-source-scalars.lino', import.meta.url));
const [major, minor] = process.versions.node.split('.').map(Number);
assert.ok(major === 22 && minor >= 15 || major === 24, 'uncovered Node ' + process.versions.node);
assert.ok(['16.0', '17.0'].includes(process.versions.unicode), 'uncovered independent ICU Unicode ' + process.versions.unicode);
for (const row of fixture.normalizations) test('canonical NFC: ' + row.source, () => {
  assert.equal(row.source.normalize('NFC'), row.expected);
  assert.equal(normalizeUnicodeText(row.source), row.expected);
  assert.equal(normalizeUnicodeText(row.expected), row.expected);
});
for (const row of fixture.identifierSources) for (const language of ['JavaScript', 'Rust']) test(language + ' scalar identifier: ' + row.source, () => {
  const tokens = tokenize(row.source, language).tokens;
  const whole = tokens.length === 2 && tokens[0].kind === 'identifier' && tokens[0].raw === row.source;
  assert.equal(whole, row[language]);
  if (whole) assert.deepEqual([tokens[0].start, tokens[0].end], [0, row.source.length]);
});
test('JavaScript keeps canonically equivalent identifiers distinct through all codecs and emissions', () => {
  assert.equal(SOURCE_IDENTIFIER_UNICODE_VERSION, fixture.unicode);
  const declarations = readProjectData(new URL('../../parity/fixtures/unicode-javascript-identifiers.lino', import.meta.url));
  const program = checkProgram(parseJavaScript(declarations.source)), packet = encodeTypedProgramLinks(program), js = emitJavaScript(program);
  for (const actual of [packet, createUnicodeScalarProgramLinks()]) for (const target of ['JavaScript', 'Rust', 'Lean', 'Rocq']) {
    const expected = emitTypedProgramLinks(actual.network, actual.root, target);
    for (const network of [LinkNetwork.fromLino(actual.network.toLino()), decodeLinkNetworkBinary(encodeLinkNetworkBinary(actual.network))])
      assert.deepEqual(emitTypedProgramLinks(network, actual.root, target), expected);
  }
  const names = declarations.cases.map(row => js.mappings.find(mapping => mapping.kind === 'function' && mapping.source === row.entry).target);
  const functions = runInNewContext('"use strict";\n' + js.text.replace(/^export /gmu, '') + '\n[' + names.join(',') + '];');
  for (const [index, row] of declarations.cases.entries()) {
    assert.equal(String(runInNewContext('"use strict";\n' + row.source.replace(/^export /gmu, '') + '\ncompute();')), row.value, row.name);
    assert.equal(String(functions[index]()), row.value, row.name);
  }
});
test('source scalar decoder refuses lone surrogates and preserves paired widths', () => {
  assert.equal(decodeUnicodeSourceScalar(0xd801, 0xdc00), 0x10400);
  for (const pair of [[0xd801, -1], [0xdc00, -1], [0xd801, 65], [-1, -1], [0x10000, -1]]) assert.equal(decodeUnicodeSourceScalar(...pair), -1);
  assert.throws(() => normalizeUnicodeText('\ud801'), error => error.kind === 'syntax' && error.reason === 'Unicode normalization requires scalar values');
});
