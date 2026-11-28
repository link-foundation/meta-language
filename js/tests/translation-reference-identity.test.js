import assert from 'node:assert/strict';
import test from 'node:test';
import vm from 'node:vm';
import {readProjectData} from '../scripts/project-data.mjs';
import {generateReferenceIdentityRules} from '../scripts/generate-reference-identity-rules.mjs';
import {acceptReferenceIdentityComparison, readReferenceIdentityComparisonError, renderReferenceIdentityComparison} from '../src/translation/reference-identity-rules.js';
import {ReferenceHeap} from '../src/translation/reference-model.js';
import {parseJavaScript} from '../src/translation/javascript.js';
import {checkProgram} from '../src/translation/check.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
import {emitRust} from '../src/translation/emit-rust.js';
import {validateLinkedTypedProgram} from '../src/typed-program-links-validation.js';
import {encodeTypedProgramLinks, decodeTypedProgramLinks, TYPED_PROGRAM_LINKS_SCHEMA as schema} from '../src/typed-program-links.js';
import {recheckLinkedTypedProgram} from '../src/typed-program-checked-projection.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary, decodeLinkNetworkBinary} from '../src/link-network-binary.js';
const policy = readProjectData(new URL('../src/translation/data/reference-identity-rules.lino', import.meta.url));
const printed = source => {const lines = []; vm.runInNewContext(source, {console: {log: value => lines.push(String(value))}}, {timeout: 1000}); return lines.join('\n') + (lines.length ? '\n' : '');};
const reload = (network, codec) => codec === 'text' ? LinkNetwork.fromLino(network.toLino()) : decodeLinkNetworkBinary(encodeLinkNetworkBinary(network));
function nodes(program) {const result = [], seen = new WeakSet(); function walk(value) {if (!value || typeof value !== 'object' || seen.has(value)) return; seen.add(value); if (value.k && value.type) result.push(value); for (const child of value instanceof Map ? value.values() : Object.values(value)) walk(child);} walk(program); return result;}
const comparison = program => nodes(program).find(value => value.k === 'binary' && value.domain?.reference === true);

test('reference identity sources and the actual generated native closure remain fresh', async () => {
  const [major, minor] = process.versions.node.split('.').map(Number);
  assert.ok(major === 22 && minor >= 15 || major === 24, 'uncovered Node ' + process.versions.node + '; ' + policy.assertionVersions.Node);
  await generateReferenceIdentityRules({check: true, native: true});
});

test('general reference admission excludes every primitive optional value and cross-domain owner', () => {
  const kinds = ['array', 'record', 'map', 'set', 'option', 'tuple', 'cell', 'data', 'unit', 'float', 'fixed', 'string', 'bool', 'unknown'];
  for (const left of kinds) for (const right of kinds) for (const operation of ['eq', 'ne', 'lt', 'add', 'unknown'])
    for (const a of [false, true]) for (const b of [false, true]) for (const same of [false, true]) {
      const expected = ['eq', 'ne'].includes(operation) && ['array', 'record', 'map', 'set'].includes(left) && left === right && a && b && same;
      assert.equal(acceptReferenceIdentityComparison(operation, left, right, a, b, same), expected);
    }
  assert.equal(renderReferenceIdentityComparison('unknown', '&left', '&right'), '');
  assert.notEqual(readReferenceIdentityComparisonError('eq', 'array', 'array', true, true, true, false, true, true), '');
});

test('the existing linked identity operation distinguishes equal payloads and survives payload mutation', () => {
  const heap = new ReferenceHeap(), first = heap.allocate('array', [1]), alias = heap.alias(first), other = heap.allocate('array', [1]);
  assert.equal(heap.execute(policy.linkedOperation.id, first, alias), true);
  assert.equal(heap.execute(policy.linkedOperation.id, first, other), false);
  heap.write(alias, [9]);
  assert.equal(heap.execute(policy.linkedOperation.id, first, alias), true);
  assert.equal(heap.execute(policy.linkedOperation.id, first, other), false);
  assert.deepEqual(heap.read(first), [9]); assert.deepEqual(heap.read(other), [1]);
});

test('complete original sources retain independent identity and effect oracles through both semantic codecs', () => {
  for (const row of policy.cases) {
    assert.equal(printed(row.source), row.expected, row.name + ' independent Node');
    const checked = checkProgram(parseJavaScript(row.source)), packet = encodeTypedProgramLinks(checked), original = emitRust(checked);
    assert.ok(comparison(checked), row.name + ' uses the existing reference-qualified Binary');
    for (const program of [checked, ...['text', 'binary'].map(codec => recheckLinkedTypedProgram(decodeTypedProgramLinks(reload(packet.network, codec), packet.root), {wholeProgram: true}))]) {
      assert.equal(printed(emitJavaScript(program).text), row.expected, row.name);
      const emitted = emitRust(program);
      for (const field of ['text', 'definitions', 'preludes', 'mappings', 'assumptions', 'encodings']) assert.deepEqual(emitted[field], original[field]);
      assert.deepEqual(comparison(program).domain, comparison(program).left.type);
      const reverse = checkProgram(parseJavaScript(emitJavaScript(program, {identifierModel: 'ordinary JavaScript binding identifiers 1'}).text));
      assert.equal(printed(emitJavaScript(reverse).text), row.expected, row.name + ' emitted JavaScript reparses');
    }
  }
});

test('ordinary value arrays optional references ordering and cross-domain comparisons retain their refusals', () => {
  for (const row of policy.refusals) assert.throws(() => checkProgram(parseJavaScript(row.source, row.options)), row.name);
  const source = 'console.log(1===1);console.log(1!==2);console.log(1<2);console.log("a"==="a");';
  assert.equal(printed(source), 'true\ntrue\ntrue\ntrue\n');
  const checked = checkProgram(parseJavaScript(source)); assert.equal(printed(emitJavaScript(checked).text), printed(source));
  assert.equal(nodes(checked).some(value => value.domain?.reference === true), false);
});

test('source-free validation rejects forged missing domains and changed reference operations before emission', () => {
  const checked = checkProgram(parseJavaScript(policy.cases[0].source));
  for (const change of [
    value => {delete value.domain;}, value => {value.domain = {...value.domain, reference: false};},
    value => {value.domain = {kind: 'float'};}, value => {value.op = 'lt';},
    value => {value.type = {kind: 'float'};}, value => {value.right.type = {...value.right.type, reference: false};},
  ]) {
    const changed = structuredClone(checked); change(comparison(changed));
    assert.throws(() => validateLinkedTypedProgram(changed));
    assert.throws(() => recheckLinkedTypedProgram(changed, {wholeProgram: true}));
  }
});

test('default decoding refuses changed reference operations domains and Boolean results in both codecs', () => {
  const checked = checkProgram(parseJavaScript(policy.cases[0].source)), packet = encodeTypedProgramLinks(checked);
  for (const codec of ['text', 'binary']) for (const mutation of ['operation', 'domain', 'result', 'reference']) {
    const network = reload(packet.network, codec), rows = network.fieldRelations();
    const field = (parent, name) => {const row = rows.find(row => row.parent.equals(parent) && row.label === (schema.fields[name] ?? name)); assert.ok(row); return row;};
    const binary = rows.find(row => row.label === schema.fields.k && network.link(row.child).metadata().definition === 'binary').parent;
    if (mutation === 'operation') {const link = network.link(field(binary, 'op').child); link.setMetadata(link.metadata().withDefinition('lt'));}
    else if (mutation === 'domain' || mutation === 'result') {
      const destination = mutation === 'domain' ? field(binary, 'domain') : field(binary, 'type');
      const child = mutation === 'domain' ? field(binary, 'type').child : field(binary, 'domain').child;
      destination.link.setReferences([destination.parent, destination.link.references()[1], child]);
    } else {const domain = field(binary, 'domain').child, link = network.link(field(domain, 'reference').child); link.setMetadata(link.metadata().withDefinition('false'));}
    assert.throws(() => decodeTypedProgramLinks(reload(network, codec), packet.root), codec + ' ' + mutation);
  }
});
