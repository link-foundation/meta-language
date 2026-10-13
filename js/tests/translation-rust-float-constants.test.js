import assert from 'node:assert/strict';
import test from 'node:test';
import vm from 'node:vm';
import { readProjectData } from '../scripts/project-data.mjs';
import { rustFloatConstantFixture } from '../scripts/generate-rust-float-constant-fixtures.mjs';
import { parseRust } from '../src/translation/rust.js';
import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { LinkNetwork } from '../src/network.js';
import { createTypedProgramLinks, emitTypedProgramLinks } from '../src/typed-program-links.js';
import { acceptRustFloatConstantName, acceptRustFloatConstantOwner } from '../src/translation/rust-float-predicate-rules.js';

const fixture = readProjectData(new URL('../../parity/fixtures/rust-float-constants.lino', import.meta.url));
function bits(value) {
  if (Number.isNaN(value)) return 'NaN';
  const view = new DataView(new ArrayBuffer(8));
  view.setFloat64(0, value, false);
  return view.getBigUint64(0, false).toString(16).padStart(16, '0');
}
function observe(code, entry) {
  const context = {};
  vm.runInNewContext(code, context, {timeout: 1000});
  assert.equal(bits(context.apply()), entry.expectedBits, entry.id);
  assert.equal(context.classify(), entry.expectedNaN, entry.id);
  assert.equal(bits(context.absolute()), entry.expectedAbsoluteBits, entry.id);
}
test('exact primitive constants preserve independent bits, class, source spans and source-free links', () => {
  assert.deepEqual(rustFloatConstantFixture(), fixture);
  for (const entry of fixture.cases) {
    const surface = parseRust(entry.source);
    assert.deepEqual(JSON.parse(JSON.stringify(surface)), entry.surface, entry.id);
    const literal = surface.items.find(item => item.name === 'apply').body;
    assert.equal(entry.source.slice(literal.span.start, literal.span.end), entry.spelling, entry.id);
    observe(entry.originalJavaScript, entry);
    observe(emitJavaScript(checkProgram(parseJavaScript(entry.originalJavaScript))).text, entry);
    observe(entry.javascript, entry);
    const archive = createTypedProgramLinks(entry.source, 'Rust');
    const native = LinkNetwork.fromLino(archive.network.toLino());
    const shared = LinkNetwork.fromLino(entry.archive);
    for (const network of [native, shared]) {
      assert.ok(network.links().every(link => !link.metadata().definition?.includes(entry.source)));
      assert.equal(emitTypedProgramLinks(network, archive.root, 'Rust').code, entry.rust, entry.id);
      observe(emitTypedProgramLinks(network, archive.root, 'JavaScript').code, entry);
    }
  }
});
test('actual emitted algorithm definitions reimport in both directions', () => {
  for (const entry of fixture.cases) {
    const replay = checkProgram(parseRust(entry.rustDefinitions + '\nfn main(){}'));
    assert.equal(emitRust(replay).definitions.join('\n\n'), entry.rustDefinitions, entry.id);
    observe(emitJavaScript(replay).text, entry);
    const reverse = checkProgram(parseJavaScript(entry.javascriptDefinitions));
    assert.equal(emitRust(reverse).definitions.join('\n\n'), entry.reverseDefinitions, entry.id);
    observe(emitJavaScript(checkProgram(parseRust(entry.reverseDefinitions + '\nfn main(){}'))).text, entry);
  }
});
test('custom, shadowed, qualified, f32 and call syntax retain precise refusals', () => {
  for (const entry of fixture.invalid) assert.throws(() => checkProgram(parseRust(entry.source)), error => {
    assert.equal(error.kind, entry.kind, entry.id);
    assert.equal(error.reason, entry.reason, entry.id);
    assert.deepEqual(error.span ?? null, entry.span, entry.id);
    return true;
  });
  for (const name of ['NAN', 'INFINITY', 'NEG_INFINITY']) assert.equal(acceptRustFloatConstantName(name), true);
  for (const name of ['MAX', 'MIN', 'nan', 'Infinity']) assert.equal(acceptRustFloatConstantName(name), false);
  assert.equal(acceptRustFloatConstantOwner('f64', false), true);
  for (const [owner, shadowed] of [['f64', true], ['f32', false], ['std::f64', false], ['Value', false]]) assert.equal(acceptRustFloatConstantOwner(owner, shadowed), false);
});
test('a source-free linked constant edit changes its sign in both emissions', () => {
  const entry = fixture.cases.find(row => row.id === fixture.edit.case);
  const network = LinkNetwork.fromLino(entry.archive);
  const scalar = network.links().find(link => link.metadata().definition === fixture.edit.from);
  assert.ok(scalar);
  scalar.setMetadata(scalar.metadata().withDefinition(fixture.edit.to));
  const changed = LinkNetwork.fromLino(network.toLino());
  const archive = createTypedProgramLinks(entry.source, 'Rust');
  const code = emitTypedProgramLinks(changed, archive.root, 'JavaScript').code;
  const context = {};
  vm.runInNewContext(code, context, {timeout: 1000});
  assert.equal(bits(context.apply()), fixture.edit.expectedBits);
  assert.match(emitTypedProgramLinks(changed, archive.root, 'Rust').code, /f64::NEG_INFINITY/u);
});
