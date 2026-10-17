import assert from 'node:assert/strict';
import test from 'node:test';
import vm from 'node:vm';
import { readProjectData } from '../scripts/project-data.mjs';
import { rustArrayResultFixture } from '../scripts/generate-rust-array-result-fixtures.mjs';
import { parseRust } from '../src/translation/rust.js';
import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';
import { LinkNetwork } from '../src/network.js';
import { createTypedProgramLinks, encodeTypedProgramLinks, emitTypedProgramLinks } from '../src/typed-program-links.js';
import { acceptRustSourceArrayResultModel, acceptRustSourceArrayResult, acceptRustFreshArrayResultTail } from '../src/translation/rust-array-rules.js';

const fixture = readProjectData(new URL('../../parity/fixtures/rust-array-results.lino', import.meta.url));
const options = { arrayResultModel: fixture.arrayResultModel };
function observed(source, name, inputs, parameterTypes = []) {
  const context = {};
  vm.runInNewContext(source, context, { timeout: 1000 });
  return JSON.parse(JSON.stringify(inputs.map(input => context[name](...input.map((value, index) => parameterTypes[index]?.kind === 'fixed' ? BigInt(value) : value))), (_, value) => typeof value === 'bigint' ? Number(value) : value));
}
test('fresh scalar vector result profile replays actual Rust definitions and source-free Links', () => {
  assert.deepEqual(rustArrayResultFixture(), fixture);
  for (const entry of fixture.cases) {
    const surface = parseRust(entry.source, options);
    assert.deepEqual(JSON.parse(JSON.stringify(surface)), entry.surface, entry.id);
    const provenance = { project: entry.id, premises: fixture.premises };
    const archive = createTypedProgramLinks(entry.source, 'Rust', { rustArrayResultModel: fixture.arrayResultModel, provenance });
    const lino = archive.network.toLino();
    const network = LinkNetwork.fromLino(lino);
    const field = (parent, label) => network.fieldRelations().find(relation => relation.parent.equals(parent) && relation.label === label)?.child;
    const selected = field(archive.root, 'provenance');
    const profile = field(selected, 'source profile');
    const model = field(profile, 'rust array result model');
    assert.equal(network.link(model).metadata().definition, fixture.arrayResultModel, entry.id);
    const user = field(selected, 'user provenance');
    assert.equal(network.link(field(user, 'project')).metadata().definition, entry.id, entry.id);
    assert.ok(network.links().every(link => !link.metadata().definition?.includes(entry.source)), entry.id);
    const javascript = emitTypedProgramLinks(network, archive.root, 'JavaScript').code;
    assert.equal(javascript, entry.javascript, entry.id);
    assert.deepEqual(observed(javascript, entry.name, entry.inputs, entry.surface.items[0].params.map(parameter => parameter.type)), entry.expected, entry.id);
    assert.ok(emitTypedProgramLinks(network, archive.root, 'Rust').code.includes(entry.rustDefinitions), entry.id);
    const replay = checkProgram(parseRust(entry.rustDefinitions + '\n' + entry.main, options));
    assert.equal(emitRust(replay).definitions.join('\n\n'), entry.rustDefinitions, entry.id);
    assert.deepEqual(observed(emitJavaScript(replay).text, entry.name, entry.inputs, entry.surface.items[0].params.map(parameter => parameter.type)), entry.expected, entry.id);
  }
});
test('number and boolean constructor algorithms check Rust to JavaScript and JavaScript to Rust', () => {
  for (const id of fixture.bidirectionalCases) {
    const entry = fixture.cases.find(candidate => candidate.id === id);
    const reverse = checkProgram(parseJavaScript(entry.javascriptDefinitions));
    const archive = encodeTypedProgramLinks(reverse);
    const network = LinkNetwork.fromLino(archive.network.toLino());
    const returnedDefinitions = emitRust(reverse).definitions.join('\n\n');
    assert.ok(emitTypedProgramLinks(network, archive.root, 'Rust').code.includes(returnedDefinitions), id);
    assert.equal(returnedDefinitions, entry.rustDefinitions, id);
    const returned = checkProgram(parseRust(returnedDefinitions + '\nfn main(){}', options));
    assert.deepEqual(observed(emitJavaScript(returned).text, entry.name, entry.inputs, entry.surface.items[0].params.map(parameter => parameter.type)), entry.expected, id);
  }
});
test('legacy profile and unsupported fresh result premises keep explicit refusals', () => {
  const legacy = readProjectData(new URL('../../parity/fixtures/rust-array-source.lino', import.meta.url));
  const original = legacy.invalid.find(entry => entry.id === fixture.legacyRefusal);
  assert.equal(original.source, 'fn values()->Vec<f64>{vec![1f64]} fn main(){}');
  assert.throws(() => parseRust(original.source), error => {
    assert.equal(error.kind, original.kind);
    assert.equal(error.reason, original.reason);
    assert.deepEqual(error.span, original.span);
    return true;
  });
  assert.doesNotThrow(() => checkProgram(parseRust(original.source, options)));
  for (const entry of fixture.invalid) assert.throws(() => checkProgram(parseRust(entry.source, options)), error => {
    assert.equal(error.kind, entry.kind, entry.id);
    assert.equal(error.reason, entry.reason, entry.id);
    assert.deepEqual(error.span ?? null, entry.span, entry.id);
    return true;
  });
  assert.throws(() => parseRust('fn main(){}', { arrayResultModel: 'unknown' }), /Rust array result model/u);
  assert.throws(() => createTypedProgramLinks('fn main(){}', 'Rust', { rustArrayResultModel: 'unknown' }), /Rust array result model/u);
  assert.throws(() => createTypedProgramLinks('function value(){return 1;}', 'JavaScript', { rustArrayResultModel: fixture.arrayResultModel }), /requires Rust source/u);
});
test('generated profile decisions require fresh owned scalar results and constructor tails', () => {
  const model = fixture.arrayResultModel;
  assert.equal(acceptRustSourceArrayResultModel(''), true);
  assert.equal(acceptRustSourceArrayResultModel(model), true);
  assert.equal(acceptRustSourceArrayResultModel('unknown'), false);
  for (const kind of ['bool', 'float', 'fixed', 'unit']) {
    assert.equal(acceptRustSourceArrayResult(model, kind, true, true), true);
    assert.equal(acceptRustSourceArrayResult('', kind, true, true), false);
    assert.equal(acceptRustSourceArrayResult(model, kind, false, true), false);
    assert.equal(acceptRustSourceArrayResult(model, kind, true, false), false);
  }
  for (const kind of ['string', 'array', 'named', 'tuple']) assert.equal(acceptRustSourceArrayResult(model, kind, true, true), false);
  assert.equal(acceptRustFreshArrayResultTail('array', true, false), true);
  for (const kind of ['let', 'if']) assert.equal(acceptRustFreshArrayResultTail(kind, false, true), true);
  for (const kind of ['name', 'app', 'match', 'abort']) assert.equal(acceptRustFreshArrayResultTail(kind, true, true), false);
});
