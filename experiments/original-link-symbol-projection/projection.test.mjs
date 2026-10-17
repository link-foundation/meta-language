import assert from 'node:assert/strict';
import test from 'node:test';
import vm from 'node:vm';
import {readProjectData} from '../../js/scripts/project-data.mjs';
import {LinkNetwork, encodeLinkNetworkBinary, decodeLinkNetworkBinary} from '../../js/src/index.js';
import {parseJavaScript} from '../../js/src/translation/javascript.js';
import {checkProgram} from '../../js/src/translation/check.js';
import {encodeTypedProgramLinks, emitTypedProgramLinks} from '../../js/src/typed-program-links.js';
import {createPortableRustTextCertificate, encodePortableRustTextCertificate, decodePortableRustTextCertificate, RUST_TEXT_CERTIFICATE_POLICY} from '../rust-text-certificate/bridge.mjs';
import {originalLinkSymbolPolicy as policy, originalLinkSymbolRulesSource} from './policy.mjs';
import {projectOriginalLinkSymbolValues as project} from './rules.mjs';
import {generateOriginalLinkSymbolProjection} from './generate.mjs';

function transport(values) {
 const starts = [], lengths = [], units = [];
 for (const value of values) {
  starts.push(units.length);
  lengths.push(typeof value === 'string' ? value.length : -1);
  if (typeof value === 'string') for (let at = 0; at < value.length; at++) units.push(value.charCodeAt(at));
 }
 return [starts, lengths, units];
}
const argumentsFor = (fixed, source) => [...transport(fixed), ...transport(source)];
function reference(fixed, source) {
 const strings = [null, ...fixed], indices = new Map(fixed.map((value, index) => [value, index + 1]));
 const symbols = source.map(value => {
  if (typeof value !== 'string') return -1;
  if (!indices.has(value)) { indices.set(value, strings.length); strings.push(value); }
  return indices.get(value);
 });
 const names = [], units = [];
 for (const value of strings) {
  names.push(units.length, value === null ? 0 : value.length);
  if (value !== null) {
   for (let index = 0; index < value.length; index++) units.push(value.charCodeAt(index));
   units.push(0);
  }
 }
 return [1, source.length, strings.length, units.length, ...symbols, ...names, ...units];
}
function unpack(result) {
 assert.equal(result[0], 1);
 const symbolEnd = 4 + result[1], nameEnd = symbolEnd + 2 * result[2];
 assert.equal(result.length, nameEnd + result[3]);
 return {symbols: result.slice(4, symbolEnd), names: result.slice(symbolEnd, nameEnd), units: result.slice(nameEnd)};
}
function assertProjection(fixed, source, evaluate = project) {
 const args = argumentsFor(fixed, source), before = structuredClone(args);
 const result = Array.from(evaluate(...args));
 assert.deepEqual(result, reference(fixed, source));
 assert.deepEqual(args, before, 'input vectors remain unchanged');
 return unpack(result);
}
const fixtures = readProjectData(new URL('../rust-text-value-projection/data/fixtures.lino', import.meta.url));
const loads = [packet => ({...packet, network: LinkNetwork.fromLino(packet.network.toLino())}),
 packet => ({...packet, network: decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))})];
function stringsFrom(table) {
 const values = [];
 for (let index = 2; index < table.names.length; index += 2) values.push(String.fromCharCode(...table.units.slice(table.names[index], table.names[index] + table.names[index + 1])));
 return values;
}

test('fixed and first-observed values produce complete independently specified symbol, name and unit tables', () => {
 for (const [fixed, source] of [
  [[], []], [[], [null, undefined, false, 1]], [[], ['', '', 'alpha', 'beta', 'alpha']],
  [['', 'Rust', 'String'], ['String', 'dynamic', 'Rust', 'dynamic', null, '']],
  [['prefix', 'pre'], ['pre', 'prefix', 'prefixes', 'prefixes']],
 ]) assertProjection(fixed, source);
 const out = assertProjection([''], [null, undefined, '', '', 'next', 'next']);
 assert.deepEqual(out.symbols, [-1, -1, 1, 1, 2, 2]);
 assert.deepEqual(out.names, [0, 0, 0, 0, 1, 4]);
});

test('exact UTF16 values retain NUL, surrogate pairs, isolated units and normalization distinctions', () => {
 const fixed = ['', '\0', 'a\0b', 'Ω😀'], source = ['\0', '\0\0', 'a\0', 'a\0b', '\ud800', '\udc00', '😀', 'é', 'e\u0301', 'Ω😀', '\ud800'];
 const out = assertProjection(fixed, source);
 assert.notEqual(out.symbols[7], out.symbols[8], 'normalization does not infer equality');
 assert.notEqual(out.symbols[4], out.symbols[5], 'distinct isolated units remain distinct');
 assert.equal(out.symbols[4], out.symbols[10]);
});

test('the fixed107-symbol dictionary and all thirteen checked dictionaries match through both saved formats', () => {
 assert.equal(RUST_TEXT_CERTIFICATE_POLICY.symbols.length, 107);
 assert.equal(fixtures.cases.length, 13);
 for (const fixture of fixtures.cases) {
  const value = createPortableRustTextCertificate(fixture.source), packet = encodePortableRustTextCertificate(value.program, value.proof);
  for (const load of loads) {
   const saved = load(packet), before = saved.network.toLino(), restored = decodePortableRustTextCertificate(saved.network, saved.root);
   const values = stringsFrom(restored.table), out = assertProjection(RUST_TEXT_CERTIFICATE_POLICY.symbols, [...values, ...values, null]);
   assert.deepEqual(out.names, restored.table.names, fixture.id);
   assert.deepEqual(out.units, restored.table.units, fixture.id);
   assert.equal(saved.network.toLino(), before, 'dictionary projection does not change saved source identities');
   assert.equal(restored.decision[0], 1);
  }
 }
});

test('actual complete source renames change dynamic units without substituting a fixture dictionary', () => {
 const fixture = fixtures.cases.find(row => row.source.includes('choose'));
 const before = createPortableRustTextCertificate(fixture.source).table;
 const after = createPortableRustTextCertificate(fixture.source.replace(/\bchoose\b/gu, 'selectText')).table;
 assert.notDeepEqual(before.units, after.units);
 for (const table of [before, after]) {
  const out = assertProjection(RUST_TEXT_CERTIFICATE_POLICY.symbols, stringsFrom(table));
  assert.deepEqual(out.names, table.names);
  assert.deepEqual(out.units, table.units);
 }
});

test('offsets, exact scalar domains, holes, absent markers and complete unit consumption are checked', () => {
 const base = argumentsFor(['base'], ['query', null]);
 for (const vector of [0, 1, 2, 3, 4, 5]) {
  for (const value of [-0, -2, 0.5, NaN, Infinity, -Infinity, true, '0', null, undefined, 65536, 100001]) {
   const args = structuredClone(base); args[vector][0] = value;
   // 65536 is a valid offset/length integer but is out of bounds here.
   assert.deepEqual(project(...args), vector < 3 ? [-1] : [-2], `vector ${vector}: ${String(value)}`);
  }
  const hole = structuredClone(base); delete hole[vector][0];
  assert.deepEqual(project(...hole), vector < 3 ? [-1] : [-2], 'holes cannot turn into absent values');
 }
 for (const vector of [0, 1, 3, 4]) {
  const args = structuredClone(base); args[vector].pop();
  assert.deepEqual(project(...args), vector < 3 ? [-1] : [-2]);
 }
 for (const vector of [2, 5]) {
  const args = structuredClone(base); args[vector].push(0);
  assert.deepEqual(project(...args), vector < 3 ? [-1] : [-2], 'no trailing unconsumed units');
 }
 const fixedAbsent = argumentsFor([null], []); assert.deepEqual(project(...fixedAbsent), [-1]);
 const sourceAbsent = assertProjection([], [null, false, '', undefined]); assert.deepEqual(sourceAbsent.symbols, [-1, -1, 1, -1]);
});

test('fixed duplicates and refusal precedence remain explicit', () => {
 assert.deepEqual(project(...argumentsFor(['same', 'same'], [])), [-3, 1]);
 assert.deepEqual(project(...argumentsFor(['', ''], ['other'])), [-3, 1]);
 const both = argumentsFor(['same', 'same'], ['query']); both[5][0] = NaN;
 assert.deepEqual(project(...both), [-2], 'source transport is checked before duplicate fixed values');
 both[2][0] = NaN; assert.deepEqual(project(...both), [-1], 'fixed transport is checked first');
});

test('declared record and unit limits refuse before oversized scans or result accumulation', () => {
 for (const index of [0, 3]) {
  const args = argumentsFor([], []); args[index] = Array(policy.maximumStrings + 1); args[index + 1] = Array(policy.maximumStrings + 1);
  assert.deepEqual(project(...args), index === 0 ? [-1] : [-2]);
 }
 for (const index of [2, 5]) {
  const args = argumentsFor([], []); args[index] = Array(policy.maximumUnits + 1);
  assert.deepEqual(project(...args), index === 2 ? [-1] : [-2]);
 }
});

test('the entire checked program executes after source-free typed Links text and binary reload', () => {
 const program = checkProgram(parseJavaScript(originalLinkSymbolRulesSource(), policy.translation));
 const packet = encodeTypedProgramLinks(program);
 for (const load of loads) {
  const saved = load(packet), context = {};
  vm.runInNewContext(emitTypedProgramLinks(saved.network, saved.root, 'JavaScript').code, context, {timeout: 2000});
  assert.equal(typeof context.projectOriginalLinkSymbolValues, 'function');
  assertProjection(['', 'Rust'], ['Rust', '\0', 'dynamic', '', null, 'dynamic'], context.projectOriginalLinkSymbolValues);
  assert.deepEqual(Array.from(context.projectOriginalLinkSymbolValues(...argumentsFor(['same', 'same'], []))), [-3, 1]);
 }
});

test('all automatically emitted definitions and shared source regenerate without native compilation', () => {
 const artifact = generateOriginalLinkSymbolProjection(true);
 assert.equal(artifact.declarationNames.length, 22);
 assert.equal(artifact.declarations.length, 22);
 assert.ok(artifact.declarationNames.includes('projectOriginalLinkSymbolValues'));
 assert.ok(artifact.completeRust.includes('pub fn project_original_link_symbol_values('));
 assert.ok(artifact.completeRust.includes('fn main('));
 assert.equal(artifact.model, policy.model);
});
