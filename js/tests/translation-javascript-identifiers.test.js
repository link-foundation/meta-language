// Future complete controls. Never execute during the whole requirements draft.
import assert from 'node:assert/strict';
import test from 'node:test';
import { runInNewContext } from 'node:vm';
import { readProjectData } from '../scripts/project-data.mjs';
import { tokenize, TokenCursor } from '../src/translation/lexer.js';
import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';
import { encodeTypedProgramLinks, emitTypedProgramLinks } from '../src/typed-program-links.js';
import { createJavaScriptIdentifierProgramLinks } from '../src/javascript-identifier-program.js';
import { LinkNetwork } from '../src/network.js';
import { encodeLinkNetworkBinary, decodeLinkNetworkBinary } from '../src/link-network-binary.js';
import { readJavaScriptModuleSyntaxError } from '../../experiments/javascript-program/syntax-source.mjs';
const fixture = readProjectData(new URL('../../parity/fixtures/javascript-escaped-identifiers.lino', import.meta.url));
const [major, minor] = process.versions.node.split('.').map(Number);
assert.ok(major === 22 && minor >= 15 || major === 24, 'uncovered Node ' + process.versions.node);
for (const row of fixture.lexicalCases) test('escaped identifier source: ' + row.source, () => {
  const { tokens } = tokenize(row.source, 'JavaScript');
  assert.equal(tokens.length, row.tokens.length + 1);
  const cursor = new TokenCursor(tokens, 'JavaScript');
  for (const expected of row.tokens) {
    const token = cursor.peek();
    assert.deepEqual({ kind: token.kind, value: token.value, raw: token.raw, start: token.start, end: token.end },
      { kind: expected.kind, value: expected.value, raw: expected.raw, start: expected.start, end: expected.end });
    assert.equal(cursor.is(expected.value), expected.matchesValue); cursor.next();
  }
  assert.equal(cursor.peek().kind, 'eof');
});
for (const row of fixture.lexicalErrors) test('identifier lexical refusal: ' + row.source, () => assert.throws(() => tokenize(row.source, 'JavaScript'), error => {
  assert.equal(error.kind, row.kind); assert.equal(error.reason, row.reason); assert.deepEqual(error.span, row.span); return true;
}));
for (const row of fixture.refusals) test('strict module identifier refusal: ' + row.name, () => {
  const original = readJavaScriptModuleSyntaxError(row.source);
  assert.equal(original.source, row.source); assert.notEqual(original.exitCode, 0); assert.equal(original.signal, null);
  assert.ok(original.stderr.includes('SyntaxError'));
  assert.throws(() => checkProgram(parseJavaScript(row.source)), error => error.kind === row.kind && error.reason === row.reason);
});
test('escaped identifiers retain original values, aliases and full graphs across both codecs and four emissions', () => {
  const program = checkProgram(parseJavaScript(fixture.source)), packet = encodeTypedProgramLinks(program), js = emitJavaScript(program);
  for (const actual of [packet, createJavaScriptIdentifierProgramLinks()]) for (const target of ['JavaScript', 'Rust', 'Lean', 'Rocq']) {
    const expected = emitTypedProgramLinks(actual.network, actual.root, target);
    for (const network of [LinkNetwork.fromLino(actual.network.toLino()), decodeLinkNetworkBinary(encodeLinkNetworkBinary(actual.network))])
      assert.deepEqual(emitTypedProgramLinks(network, actual.root, target), expected);
  }
  const names = fixture.cases.map(row => js.mappings.find(mapping => mapping.kind === 'function' && mapping.source === row.entry).target);
  const functions = runInNewContext('"use strict";\n' + js.text.replace(/^export /gmu, '') + '\n[' + names.join(',') + '];');
  for (const [index, row] of fixture.cases.entries()) {
    assert.equal(String(runInNewContext('"use strict";\n' + row.source.replace(/^export /gmu, '') + '\ncompute();')), row.value, row.name);
    assert.equal(String(functions[index]()), row.value, row.name);
  }
});
