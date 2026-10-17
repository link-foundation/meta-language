import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { format as formatConsole } from 'node:util';
import vm from 'node:vm';
import { test } from 'node:test';
import { readProjectData } from '../scripts/project-data.mjs';
import { callableEmissionRulesPolicy, readCallableEmissionSourceModule } from '../scripts/generate-callable-emission-rules.mjs';
import { readPortableSourceParameterStyles, partitionPortableRustLibrary } from '../scripts/portable-source-library-style.mjs';
import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { emitLean } from '../src/translation/emit-lean.js';
import { emitRocq } from '../src/translation/emit-rocq.js';
import { createSurfaceReferenceFacts } from '../src/translation/surface-reference-facts.js';
import { readCallableFreeReferences } from '../src/translation/callable-source-rules.js';
import { readCallableCapturedBindings } from '../src/translation/callable-source.js';
import { readCallableOutputAnalysis } from '../src/translation/callable-emission.js';
import { planCallableCaptureBindings, readCallableEagerEffects, readCallableOutputTargetRefusal } from '../src/translation/callable-emission-rules.js';
import { createTypedProgramLinks, decodeTypedProgramLinks, emitTypedProgramLinks } from '../src/typed-program-links.js';
import { recheckLinkedTypedProgram } from '../src/typed-program-checked-projection.js';
import { LinkNetwork } from '../src/network.js';
import { encodeLinkNetworkBinary, decodeLinkNetworkBinary } from '../src/link-network-binary.js';

const refusal = 'function values that print or may abort require an output-threaded function type';
const columns = facts => [facts.forms, facts.texts, facts.starts, facts.counts, facts.labels, facts.targets];

function observe(code) {
  const lines = [];
  let message = '';
  try {
    vm.runInNewContext(code, {console: {log: (...values) => lines.push(formatConsole(...values))}, process: {argv: []}}, {timeout: 1000});
  } catch (error) { message = error.message; }
  return {lines, message};
}

function reload(saved, binary) {
  return binary ? decodeLinkNetworkBinary(encodeLinkNetworkBinary(saved.network)) : LinkNetwork.fromLino(saved.network.toLino());
}

function rejectFormal(network, root) {
  for (const [target, emit] of [['Lean', emitLean], ['Rocq', emitRocq]]) {
    const program = recheckLinkedTypedProgram(decodeTypedProgramLinks(network, root), {wholeProgram: true});
    assert.throws(() => emit(program), error => error.kind === 'unsupported' && error.details?.construct === 'effectful function value' && error.reason === 'effectful function value: ' + refusal, target);
  }
}

test('callable capture plans preserve original Map insertion order and last actual type identities', () => {
  const first = {k: 'var', name: 'first', type: {kind: 'float'}};
  const second = {k: 'var', name: 'second', type: {kind: 'int'}};
  const last = {k: 'var', name: 'first', type: {kind: 'string'}};
  const body = {k: 'tuple', items: [first, second, last, second], type: {kind: 'tuple', elements: []}};
  const facts = createSurfaceReferenceFacts(body, {retainValues: true});
  const references = readCallableFreeReferences(...columns(facts), facts.root, [], true);
  const original = new Map();
  for (const identity of references) { const node = facts.values[identity]; original.set(node.name, node.type); }
  const selected = planCallableCaptureBindings(facts.texts, facts.starts, facts.counts, facts.labels, facts.targets, references);
  assert.deepEqual(selected.map(identity => facts.values[identity]), [last, second]);
  assert.equal(facts.values[selected[0]].type, last.type);
  assert.deepEqual([...readCallableCapturedBindings(body, [])], [...original]);
  assert.deepEqual([...original.keys()], ['first', 'second']);
  assert.throws(() => planCallableCaptureBindings([], [0], [1], [], [], [0]), /^RangeError: invalid required text fact column position$/u);
});

test('callable effect plans keep creation pure and retain actual Lambda FunctionRef and Apply obligations', () => {
  const source = "function noisy(value){console.log('called');return value+1;}function make(){return ()=>{console.log('latent');return 7;};}console.log((()=>{console.log('invoked');return 9;})());const selected=noisy;console.log(selected(2));";
  const program = checkProgram(parseJavaScript(source));
  const analysis = readCallableOutputAnalysis(program);
  const make = [...program.declarations.values()].find(entry => entry.k === 'fn' && entry.name === 'make');
  const noisy = [...program.declarations.values()].find(entry => entry.k === 'fn' && entry.name === 'noisy');
  assert.equal(analysis.effectful.has(make.fullName), false, 'creating the returned closure does not print');
  assert.equal(analysis.effectful.has(noisy.fullName), true, 'the named invocation really prints');
  assert.ok(analysis.obligations.some(node => node.k === 'lambda'));
  assert.ok(analysis.obligations.some(node => node.k === 'functionRef'));
  assert.ok(analysis.obligations.some(node => node.k === 'apply'));
  const lambda = analysis.obligations.find(node => node.k === 'lambda');
  const facts = createSurfaceReferenceFacts(lambda, {retainValues: true});
  assert.equal(readCallableEagerEffects(...columns(facts), facts.root, []), false);
  assert.ok(analysis.obligations.every(node => node.span && node.span.end >= node.span.start));
  assert.equal(readCallableOutputTargetRefusal('Rust', analysis.obligations.length), '');
  assert.equal(readCallableOutputTargetRefusal('JavaScript', analysis.obligations.length), '');
  assert.equal(readCallableOutputTargetRefusal('Lean', 0), '');
});

test('effectful callable sources keep creation and invocation traces through both source free formats', () => {
  for (const [source, lines] of [
    ["function make(){console.log('created');return ()=>{console.log('called');return 7;};}const next=make();console.log('ready');console.log(next());", ['created', 'ready', 'called', '7']],
    ["function noisy(value){console.log('called');return value+1;}function use(callback){console.log('before');return callback(2);}const selected=noisy;console.log('ready');console.log(use(selected));", ['ready', 'before', 'called', '3']],
    ["console.log((()=>{console.log('called');return 7;})());", ['called', '7']],
  ]) {
    assert.deepEqual(observe(source), {lines, message: ''});
    const saved = createTypedProgramLinks(source, 'JavaScript');
    for (const binary of [false, true]) {
      const network = reload(saved, binary);
      recheckLinkedTypedProgram(decodeTypedProgramLinks(network, saved.root), {wholeProgram: true});
      assert.deepEqual(observe(emitTypedProgramLinks(network, saved.root, 'JavaScript').code), {lines, message: ''});
      const rust = emitTypedProgramLinks(network, saved.root, 'Rust').code;
      assert.match(rust, /std::rc::Rc::new/u);
      assert.match(rust, /println!/u);
      rejectFormal(network, saved.root);
    }
  }
});

test('callable aborts happen only at invocation and keep the formal target obligation explicit', () => {
  const source = "function make(){return ()=>{throw new Error('stop');};}const next=make();console.log('ready');next();";
  assert.deepEqual(observe(source), {lines: ['ready'], message: 'stop'});
  const saved = createTypedProgramLinks(source, 'JavaScript');
  for (const binary of [false, true]) {
    const network = reload(saved, binary);
    assert.deepEqual(observe(emitTypedProgramLinks(network, saved.root, 'JavaScript').code), {lines: ['ready'], message: 'stop'});
    assert.match(emitTypedProgramLinks(network, saved.root, 'Rust').code, /std::rc::Rc::new/u);
    rejectFormal(network, saved.root);
  }
});

test('the complete callable emission source translates without host callbacks or native semantic replacements', () => {
  assert.deepEqual(readProjectData(new URL('../src/translation/data/callable-emission-rules.lino', import.meta.url)), callableEmissionRulesPolicy());
  const source = readCallableEmissionSourceModule(readFileSync(new URL('../src/translation/callable-emission-rules.js', import.meta.url), 'utf8'));
  const emitted = emitRust(checkProgram(parseJavaScript(source, {referenceSemantics: false})), { arrayConsumption: true });
  for (const name of ['plan_callable_capture_bindings', 'accept_callable_creation_effect_traversal', 'require_callable_output_effect', 'read_callable_eager_effects', 'read_callable_effectful_declarations', 'plan_callable_output_obligations', 'read_callable_output_target_refusal']) assert.ok(emitted.definitions.some(definition => definition.includes('pub fn ' + name + '(')), name);
  const header = emitted.text.split('\n').find(line => line.startsWith('#![allow('));
  const raw = [header, ...emitted.preludes, ...emitted.definitions].join('\n\n');
  const loans = readPortableSourceParameterStyles(raw);
  assert.match(loans.source, /texts: &\[String\]/u);
  const parts = partitionPortableRustLibrary(loans.source, {directory: 'callable_emission_rules'});
  assert.ok(parts.every(part => part.source.split('\n').length < 1000));
});
