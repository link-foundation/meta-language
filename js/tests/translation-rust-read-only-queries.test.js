import assert from 'node:assert/strict';
import { test } from 'node:test';
import { runInNewContext } from 'node:vm';
import { readProjectData } from '../scripts/project-data.mjs';
import { rustReadOnlyQueryFixture } from '../scripts/generate-rust-read-only-query-fixtures.mjs';
import { parseRust } from '../src/translation/rust.js';
import { checkProgram } from '../src/translation/check.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';
import { createTypedProgramLinks, emitTypedProgramLinks } from '../src/typed-program-links.js';
import { LinkNetwork } from '../src/network.js';
import * as decisions from '../src/translation/rust-read-only-query-rules.js';

const fixture = readProjectData(new URL('../../parity/fixtures/rust-read-only-queries.lino', import.meta.url));
function output(code) {
  const values = [];
  runInNewContext(code, { console: { log: value => values.push(String(value)) } }, { timeout: 1000 });
  return values;
}
test('five standard source queries retain exact projections and source-free execution', () => {
  assert.deepEqual(rustReadOnlyQueryFixture(), fixture);
  assert.equal(fixture.cases.length, 5);
  for (const entry of fixture.cases) {
    assert.deepEqual(JSON.parse(JSON.stringify(parseRust(entry.source))), entry.surface, entry.id);
    assert.deepEqual(output(entry.javascript), entry.output, entry.id);
    const archive = createTypedProgramLinks(entry.source, 'Rust');
    const reloaded = LinkNetwork.fromLino(archive.network.toLino());
    assert.deepEqual(output(emitTypedProgramLinks(reloaded, archive.root, 'JavaScript').code), entry.output, entry.id);
    assert.equal(emitTypedProgramLinks(reloaded, archive.root, 'Rust').code, entry.rust, entry.id);
  }
});
test('inclusive f64 membership preserves every NaN position, infinities and signed zero', () => {
  const source = 'fn test(low:f64,high:f64,value:f64)->bool{(low..=high).contains(&value)}fn main(){}';
  const replay = {};
  runInNewContext(emitJavaScript(checkProgram(parseRust(source))).text, replay, { timeout: 1000 });
  const values = [-Infinity,-1,-0,0,0.5,1,Infinity,NaN];
  for (const low of values) for (const high of values) for (const value of values) {
    assert.equal(replay.test(low,high,value), low <= value && value <= high);
  }
});
test('range endpoints and argument evaluate once in order without capturing source names', () => {
  const entry = fixture.cases.find(entry => entry.id === 'range-once-evaluation-and-hygiene');
  const replay = { console: { log() {} } }, trace = [];
  runInNewContext(entry.javascript, replay, { timeout: 1000 });
  for (const [name,value] of [['lower',0],['upper',1],['argument',0.5]]) replay[name] = () => { trace.push(name); return value; };
  assert.equal(replay.test(true), true);
  assert.deepEqual(trace, ['lower','upper','argument']);
  assert.equal(replay.test(false), false);
  assert.deepEqual(trace, ['lower','upper','argument'], 'the original short-circuit leaves every range operand unevaluated');
});
test('unproved owners, ownership layers and unsupported range operations keep exact refusals', () => {
  assert.equal(fixture.invalid.length, 18);
  for (const entry of fixture.invalid) assert.throws(() => checkProgram(parseRust(entry.source)), error => {
    assert.equal(error.kind, entry.kind, entry.id);
    assert.equal(error.reason, entry.reason, entry.id);
    assert.deepEqual(error.span ?? null, entry.span, entry.id);
    return true;
  });
});
test('generated policy decisions refuse unknown owner and layer combinations', () => {
  for (const proven of [false,true]) for (const shadowed of [false,true]) for (const borrowed of [false,true]) {
    for (const layer of ['value','reference','box','multiple']) {
      assert.equal(decisions.acceptRustInclusiveRangeOperand('float','binary64 scalar',proven,layer,borrowed,shadowed), proven && !shadowed && layer === (borrowed ? 'reference' : 'value'));
    }
  }
  assert.equal(decisions.acceptRustReadOnlyQueryOwner('string empty','declaration','String'), false);
  assert.equal(decisions.acceptRustReadOnlyQueryOwner('inclusive range','primitive','f32'), false);
  assert.equal(decisions.acceptRustStringEmptyReceiver('string','owned string',true,false,true), false);
  assert.equal(decisions.acceptRustStringEmptyReceiver('string','owned string',true,true,false), false);
});
