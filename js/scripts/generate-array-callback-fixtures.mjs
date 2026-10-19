// Capture complete actual profiles from independent source/value/trace premises.
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';
import { runInNewContext } from 'node:vm';
import { checkProjectData, writeProjectData } from './project-data.mjs';
import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { emitLean } from '../src/translation/emit-lean.js';
import { emitRocq } from '../src/translation/emit-rocq.js';
import { encodeTypedProgramLinks, decodeTypedProgramLinks } from '../src/typed-program-links.js';
import { recheckLinkedTypedProgram } from '../src/typed-program-checked-projection.js';
import { LinkNetwork } from '../src/network.js';
import { encodeLinkNetworkBinary, decodeLinkNetworkBinary } from '../src/link-network-binary.js';

const destination = new URL('../../parity/fixtures/array-callbacks.lino', import.meta.url);
const digest = source => createHash('sha256').update(source).digest('hex');

/** Normalize cross-realm values without merging undefined, arrays or scalars. */
export function readArrayCallbackObservation(value) {
  if (value === undefined) return { kind: 'undefined' };
  if (value === null) return { kind: 'null' };
  if (Array.isArray(value)) return { kind: 'array', items: Array.from(value, readArrayCallbackObservation) };
  assert.ok(['number', 'string', 'boolean'].includes(typeof value), 'callback observations retain actual primitive or array results');
  assert.ok(typeof value !== 'number' || Number.isFinite(value), 'the independent callback fixture has finite Number premises');
  return { kind: typeof value, value };
}

export function observeArrayCallbackProgram(source) {
  const output = [];
  const context = { console: { log: (...args) => output.push(args.map(String).join(' ')) } };
  runInNewContext(source, context, { timeout: 1000 });
  output.push(readArrayCallbackObservation(context.probe()));
  return output;
}

/** These nine original function sources and results precede the extraction. */
export function readArrayCallbackFixtureCases() {
  const rows = [
    ['map', '(value,index,source)=>{console.log(index);return value+1;}', [0, 3, 1, 4], [0, 1, 2, 3]],
    ['filter', '(value,index,source)=>{console.log(index);return value>0;}', [2, 3], [0, 1, 2, 3]],
    ['find', '(value,index,source)=>{console.log(index);return value>0;}', 2, [0, 1]],
    ['findIndex', '(value,index,source)=>{console.log(index);return value>0;}', 1, [0, 1]],
    ['some', '(value,index,source)=>{console.log(index);return value>0;}', true, [0, 1]],
    ['every', '(value,index,source)=>{console.log(index);return value>0;}', false, [0]],
    ['flatMap', '(value,index,source)=>{console.log(index);return [value,value];}', [-1, -1, 2, 2, 0, 0, 3, 3], [0, 1, 2, 3]],
    ['forEach', '(value,index,source)=>{console.log(index);}', undefined, [0, 1, 2, 3]],
    ['reduce', '(acc,value,index,source)=>{console.log(index);return acc+value;}', 4, [0, 1, 2, 3]],
  ];
  return rows.map(([operation, callback, value, trace]) => {
    const source = '/** @param {number[]} values */ function run(values){return values.' + operation + '(' + callback + (operation === 'reduce' ? ',0' : '') + ');}';
    return { id: 'array-callback-' + operation, operation, source,
      programSource: source + '\nfunction probe(){return run([-1,2,0,3]);}',
      expectedValue: readArrayCallbackObservation(value), expectedTrace: trace,
      expectedOutput: [...trace.map(String), readArrayCallbackObservation(value)] };
  });
}

/** Separate admitted true-profile cases retain the original bodies with fewer supplied parameters. */
export function readArrayCallbackReferenceFixtureCases() {
  return readArrayCallbackFixtureCases().map(row => {
    assert.equal(row.source.split(',source)').length, 2, 'the original callback has one actual unused receiver parameter');
    const source = row.source.replace(',source)', ')');
    return { ...row, id: row.id + '-reference-value-parameters', source,
      programSource: source + '\nfunction probe(){return run([-1,2,0,3]);}', referenceProfiles: [true] };
  });
}

export function readArrayCallbackTargetEmission(program, target) {
  const emit = { JavaScript: emitJavaScript, Rust: value => emitRust(value, { arrayConsumption: true }), Lean: emitLean, Rocq: emitRocq }[target];
  assert.ok(emit, 'an actual registered target emitter is selected');
  try {
    const emitted = emit(program);
    return { emitted: { ...emitted, preludes: emitted.preludes, definitions: emitted.definitions } };
  }
  catch (error) {
    assert.ok(['parse', 'type', 'unsupported'].includes(error.kind), 'unexpected failures cannot become supported target diagnostics');
    return { diagnostic: { kind: error.kind, reason: error.reason, span: error.span ?? null, details: error.details ?? null } };
  }
}

export function buildArrayCallbackFixtures() {
  const cases = [...readArrayCallbackFixtureCases(), ...readArrayCallbackReferenceFixtureCases()].map(row => {
    assert.deepEqual(observeArrayCallbackProgram(row.programSource), row.expectedOutput, row.id + ': independent original observations');
    const profiles = (row.referenceProfiles ?? [false, true]).map(referenceSemantics => {
      const surface = parseJavaScript(row.programSource, { referenceSemantics });
      let checked;
      try { checked = checkProgram(surface); }
      catch (error) {
        if (!referenceSemantics || row.referenceProfiles) throw error;
        assert.equal(error.kind, 'type', 'the retained third-receiver gap is the original typed refusal');
        assert.equal(error.reason, '.' + row.operation + '() callback parameters differ');
        return { referenceSemantics, surface, checkDiagnostic: { kind: error.kind, reason: error.reason, span: error.span ?? null, details: error.details ?? null } };
      }
      assert.ok(!referenceSemantics || row.referenceProfiles, 'the original third-receiver reference gap remains explicit');
      const packet = encodeTypedProgramLinks(checked);
      const text = packet.network.toLino(), binary = encodeLinkNetworkBinary(packet.network);
      const targets = Object.fromEntries(['JavaScript', 'Rust', 'Lean', 'Rocq'].map(target => [target, readArrayCallbackTargetEmission(checked, target)]));
      assert.ok(targets.JavaScript.emitted, 'the actual JavaScript profile has an operational encoding');
      assert.deepEqual(observeArrayCallbackProgram(targets.JavaScript.emitted.text), row.expectedOutput,
        row.id + ': internal literal calls exercise the actual emitted reference ABI');
      for (const network of [LinkNetwork.fromLino(text), decodeLinkNetworkBinary(binary)]) {
        const restored = decodeTypedProgramLinks(network, packet.root);
        assert.deepEqual(restored, checked, 'complete checked declarations, types, effects and source records survive');
        assert.equal(recheckLinkedTypedProgram(restored, { wholeProgram: true }), restored);
        for (const target of Object.keys(targets)) assert.deepEqual(readArrayCallbackTargetEmission(restored, target), targets[target]);
      }
      return { referenceSemantics, surface,
        checkedOwners: [...checked.declarations.values()].map(owner => ({ kind: owner.k, name: owner.name, fullName: owner.fullName, modulePath: owner.modulePath })),
        checkedArchive: { root: packet.root.asU64(), text, textSha256: digest(text), binaryHex: Buffer.from(binary).toString('hex'), binarySha256: digest(binary) },
        targets, observedOutput: row.expectedOutput };
    });
    return { ...row, sourceSha256: digest(row.source), programSourceSha256: digest(row.programSource), profiles };
  });
  return { schemaVersion: 1, model: 'complete array callback profiles 1', cases,
    boundary: 'All nine original sources retain false-profile checked archives and exact true-profile third-receiver type refusals. Nine separate true-profile cases omit the unused receiver parameter and retain complete checked archives and independent values/traces. Target refusals remain refusals; native execution requires CI.' };
}

export function generateArrayCallbackFixtures({ check = false } = {}) {
  const fixture = buildArrayCallbackFixtures();
  if (check) checkProjectData(destination, fixture);
  else writeProjectData(destination, fixture);
  return fixture;
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  const args = process.argv.slice(2);
  assert.ok(args.length === 0 || args.length === 1 && args[0] === '--check', 'callback fixture generator accepts only freshness mode');
  generateArrayCallbackFixtures({ check: args.includes('--check') });
}
