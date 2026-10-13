import assert from 'node:assert/strict';
import test from 'node:test';
import vm from 'node:vm';
import { readFileSync } from 'node:fs';
import { readProjectData } from '../scripts/project-data.mjs';
import { rustScalarParameterFixture } from '../scripts/generate-rust-scalar-parameter-fixtures.mjs';
import { parseRust } from '../src/translation/rust.js';
import { checkProgram } from '../src/translation/check.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';
import { LinkNetwork } from '../src/network.js';
import { createTypedProgramLinks, emitTypedProgramLinks } from '../src/typed-program-links.js';
import { acceptRustUnwrittenScalarParameter, detectRustSourceAssignmentToken } from '../src/translation/rust-scalar-parameter-rules.js';
import { readRustScalarParameterBody } from '../src/translation/rust-scalar-parameters.js';
import { TokenCursor, tokenize } from '../src/translation/lexer.js';

const fixture = readProjectData(new URL('../../parity/fixtures/rust-scalar-parameters.lino', import.meta.url));
function output(source) {
  const observed = [];
  vm.runInNewContext(source, { console: { log: value => observed.push(value) } }, { timeout: 1000 });
  return observed;
}

test('unwritten scalar parameters retain source observations and source-free linked semantics', () => {
  assert.deepEqual(rustScalarParameterFixture(), fixture);
  for (const entry of fixture.cases) {
    assert.deepEqual(JSON.parse(JSON.stringify(parseRust(entry.source))), entry.surface, entry.id);
    assert.deepEqual(output(entry.javascript), entry.expectedOutput, entry.id);
    assert.deepEqual(output(entry.replayJavaScript), entry.expectedOutput, entry.id);
    const archive = createTypedProgramLinks(entry.source, 'Rust');
    const network = LinkNetwork.fromLino(archive.network.toLino());
    assert.deepEqual(output(emitTypedProgramLinks(network, archive.root, 'JavaScript').code), entry.expectedOutput, entry.id);
    assert.ok(emitTypedProgramLinks(network, archive.root, 'Rust').code.includes(entry.definitions), entry.id);
  }
});

test('written, borrowed, nominal and conservatively unproved parameters retain the original mut diagnostic', () => {
  for (const entry of fixture.invalid) assert.throws(() => checkProgram(parseRust(entry.source)), error => {
    assert.equal(error.kind, entry.kind, entry.id);
    assert.equal(error.reason, entry.reason, entry.id);
    assert.deepEqual(error.span, entry.span, entry.id);
    return true;
  });
  assert.throws(() => parseRust('fn changed(mut value:i32)->i32{loop{value}} fn main(){}'), /loop loop: loops over mutable state/u);
  assert.throws(() => parseRust('fn changed(mut value:i32)->i32{unknown!(value)} fn main(){}'), /unknown!: outside the portable core/u);
  assert.throws(() => parseRust('fn main(){let mut value=1;}'), /mutable binding: mutation is outside the portable core/u);
});

test('the actual generated source part with a written mutable loop parameter remains refused', () => {
  const original = readFileSync(new URL('../../rust/src/resource_description_ntriples_rules/part_1.rs', import.meta.url), 'utf8');
  assert.throws(() => parseRust(original + '\nfn main(){}'), error => {
    assert.equal(error.kind, 'unsupported');
    assert.equal(error.reason, 'mutable parameter: mutation is outside the portable core');
    assert.equal(original.slice(error.span.start, error.span.end), 'mut');
    return true;
  });
});

test('portable premises require every source proof and classify only assignment punctuation', () => {
  for (const kind of ['bool', 'float', 'fixed']) {
    assert.equal(acceptRustUnwrittenScalarParameter(kind, true, false, true, false), true);
    for (const evidence of [[false, false, true, false], [true, true, true, false], [true, false, false, false], [true, false, true, true]]) assert.equal(acceptRustUnwrittenScalarParameter(kind, ...evidence), false);
  }
  for (const kind of ['unit', 'tuple', 'string', 'array', 'named', 'function', '']) assert.equal(acceptRustUnwrittenScalarParameter(kind, true, false, true, false), false);
  for (const token of ['=', '+=', '-=', '*=', '/=', '%=', '&=', '|=', '^=', '<<=', '>>=']) {
    assert.equal(detectRustSourceAssignmentToken('punct', token), true);
    assert.equal(detectRustSourceAssignmentToken('string', token), false);
  }
  for (const token of ['==', '!=', '=>', '<=', '>=', '..=', '+']) assert.equal(detectRustSourceAssignmentToken('punct', token), false);
  for (const [source, expected] of [['{if true{{1}}else{2}}', { closed: true, possibleWrite: false }], ['{if true{value=1}', { closed: false, possibleWrite: true }], ['{ "}" == "=" }', { closed: true, possibleWrite: false }], ['{value=1} {next}', { closed: true, possibleWrite: true }]]) {
    const cursor = new TokenCursor(tokenize(source, 'Rust').tokens, 'Rust');
    assert.deepEqual(readRustScalarParameterBody(cursor), expected, source);
    assert.equal(cursor.index, 0, source);
  }
});
