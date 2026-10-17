import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import vm from 'node:vm';
import { createHash } from 'node:crypto';
import { format as formatConsole } from 'node:util';
import { test } from 'node:test';
import { readProjectData } from '../scripts/project-data.mjs';
import { readCallableSourceModule } from '../scripts/callable-source-module.mjs';
import { readPortableSourceParameterStyles, partitionPortableRustLibrary } from '../scripts/portable-source-library-style.mjs';
import { digest, runStages } from '../scripts/translation-stage-lib.mjs';
import { LinkNetwork } from '../src/network.js';
import { decodeLinkNetworkBinary, encodeLinkNetworkBinary } from '../src/link-network-binary.js';
import { createTypedProgramLinks, decodeTypedProgramLinks, emitTypedProgramLinks, readTypedProgramBindingIdentities, TYPED_PROGRAM_LINKS_SCHEMA } from '../src/typed-program-links.js';
import { recheckLinkedTypedProgram } from '../src/typed-program-checked-projection.js';
import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { emitLean } from '../src/translation/emit-lean.js';
import { emitRocq } from '../src/translation/emit-rocq.js';
import { statementUses } from '../src/translation/javascript-lower.js';
import { createSurfaceReferenceFacts } from '../src/translation/surface-reference-facts.js';
import { CALLABLE_SOURCE_POLICY } from '../src/translation/callable-source-policy.js';
import {
  constructCallableFunctionType, constructCallableSurfaceParameters, readCallableApplicationConstraints,
  readCallableCapturedBindings, readCallableClosureConflicts,
  readCallableClosureScope, readCallableFunctionConstraints, readCallableLambdaInference,
  readCallableOwnedParameters, readCallableParameterTypes, readCallableStatementUsage,
  readCallableMutableCellNames,
  renameCallableSourceExpression,
} from '../src/translation/callable-source.js';
import { planCallableSurfaceParameters, readCallableApplicationError } from '../src/translation/callable-source-rules.js';
import { acceptCallableTypeWhitespace, planCallableTypeSyntax, readCallableLinkedApplicationError } from '../src/translation/callable-source-rules.js';
import { readCompositeType } from '../src/translation/javascript-bindings.js';
import { unsupported } from '../src/translation/diagnostics.js';

function output(code) {
  const lines = [];
  vm.runInNewContext(code, { console: { log: (...values) => lines.push(values.map(String).join(' ')) }, process: { argv: [] } }, { timeout: 1000 });
  return lines;
}

function reload(saved, binary = false) {
  return { root: saved.root, network: binary ? decodeLinkNetworkBinary(encodeLinkNetworkBinary(saved.network)) : LinkNetwork.fromLino(saved.network.toLino()) };
}

function field(network, parent, label) {
  const spelling = TYPED_PROGRAM_LINKS_SCHEMA.fields[label] ?? label.replace(/[A-Z]/gu, letter => ' ' + letter.toLowerCase());
  return network.fieldRelations().find(relation => relation.parent.equals(parent) && relation.label === spelling)?.child;
}

function constructs(network, kind) {
  return network.fieldRelations().filter(relation => relation.label === 'construct kind' && network.link(relation.child).metadata().definition === kind).map(relation => relation.parent);
}

function redirect(network, parent, label, target) {
  const relation = network.fieldRelations().find(item => item.parent.equals(parent) && item.label === label);
  assert.ok(relation, 'the actual semantic relation exists');
  relation.link.setReferences([parent, relation.link.references()[1], target]);
}

const fixtureNames = [
  'check/js-higher-order',
  ...['negative-closure', 'negative-functionValue', 'error-namespace-method-value', 'error-arrow-bare', 'error-arrow-identifier', 'error-arrow-empty', 'error-computed-call', 'accept-parenthesised-call'].map(name => 'frontend-javascript/frontend-javascript-' + name),
];
const frozen = readProjectData(new URL('../../parity/fixtures/translation-stages.lino', import.meta.url));
for (const name of fixtureNames) {
  test('callable source preserves every original stage: ' + name, () => {
    const fixture = frozen.programs.find(entry => entry.name === name);
    assert.ok(fixture, 'the original source remains registered');
    const actual = runStages(fixture.extension, fixture.source);
    const record = stage => stage.error ? { error: stage.error } : { sha256: digest(stage.value) };
    assert.deepEqual(record(actual.parse), fixture.parse);
    assert.ok(actual.check, 'parsing succeeds before the original checked result');
    assert.deepEqual(record(actual.check), fixture.check);
    if (fixture.emit) {
      assert.deepEqual(Object.keys(actual.emit), Object.keys(fixture.emit));
      for (const target of Object.keys(fixture.emit)) assert.deepEqual(record(actual.emit[target]), fixture.emit[target], target);
    } else assert.equal(actual.emit, undefined, 'the original check refusal remains a refusal');
  });
}

test('callable fact transport retains actual aliases without changing its default shape', () => {
  const shared = { kind: 'float' };
  const value = { first: shared, second: shared, absent: undefined, nullValue: null };
  const plain = createSurfaceReferenceFacts(value);
  const retained = createSurfaceReferenceFacts(value, { retainValues: true });
  assert.deepEqual(Object.keys(plain), ['forms', 'texts', 'starts', 'counts', 'labels', 'targets', 'root']);
  const { values, ...facts } = retained;
  assert.deepEqual(facts, plain);
  assert.equal(values[retained.root], value);
  assert.equal(values.filter(item => item === shared).length, 1);
  assert.ok(values.includes(undefined));
  assert.ok(values.includes(null));
});

test('callable statement capture plans preserve existing branch loop and shadowing semantics', () => {
  const name = value => ({ k: 'name', path: [value] });
  const read = value => ({ s: 'expr', expr: name(value) });
  const statements = [
    { s: 'const', name: 'local', value: name('first') },
    { s: 'block', body: [{ s: 'let', name: 'hidden', value: name('second') }, read('hidden')] },
    read('hidden'),
    { s: 'block', inline: true, body: [{ s: 'let', name: 'inline', value: name('third') }] }, read('inline'),
    { s: 'if', cond: name('condition'), then: [{ s: 'assign', name: 'outer', value: name('left') }], else: [read('right')] },
    { s: 'switch', discriminant: name('tag'), clauses: [{ body: [read('branch')] }] },
    { s: 'while', cond: name('again'), body: [read('loop')] },
    { s: 'doWhile', cond: name('after'), body: [read('body')] },
    { s: 'for', init: [{ s: 'let', name: 'at', value: name('initial') }], cond: name('at'), update: [{ s: 'assign', name: 'at', value: name('next') }], body: [read('at'), read('element')] },
    { s: 'assignReference', object: name('record'), index: name('key'), value: name('field') },
    { s: 'return', expr: { k: 'lambda', params: [{ name: 'shadow' }], body: { k: 'let', name: 'bound', value: name('initializer'), body: { k: 'array', items: [name('bound'), name('shadow'), name('captured')] } } } },
  ];
  const expected = statementUses(statements, new Set(['shadow']));
  const actual = readCallableStatementUsage(statements, new Set(['shadow']));
  assert.deepEqual([...actual.free], [...expected.free]);
  assert.deepEqual([...actual.assigned], ['outer']);
  assert.deepEqual([...actual.assigned], [...expected.assigned]);
  assert.ok(actual.free.has('hidden') && !actual.free.has('inline') && !actual.free.has('at'));
});

test('callable capture conflicts preserve actual closure identities and nested scope boundaries', () => {
  const nested = { k: 'lambda', captures: ['outer'], body: { s: 'assign', name: 'inside', value: 1 } };
  const first = { k: 'lambda', captures: ['inside', 'outer'], body: nested };
  const statements = [first, { s: 'assign', name: 'outer', value: 1 }];
  assert.deepEqual(readCallableClosureConflicts(statements), [{ closure: first, capture: 'outer' }]);
  for (const source of [
    'function value(){let n=1;const read=()=>n;n=2;return read();}',
    'function value(){let n=1;const change=()=>{n=2;return n;};return change();}',
  ]) assert.throws(() => parseJavaScript(source, { referenceSemantics: false }), /mutable closure capture/u);
});

test('callable type plans retain original parameter owners annotations and constraint identities', () => {
  const float = { kind: 'float' }, boolean = { kind: 'bool' };
  const parameters = [{ name: 'inferred' }, { name: 'nullable', type: null }, { name: 'typed', type: boolean }];
  const expected = { kind: 'function', params: [float, boolean, float], ret: boolean };
  const plan = readCallableParameterTypes(parameters, expected);
  assert.deepEqual(plan.map(entry => entry.type), [float, boolean, boolean]);
  assert.deepEqual(plan.map(entry => entry.declared), [false, false, true]);
  plan.forEach((entry, index) => assert.equal(entry.parameter, parameters[index]));
  assert.ok(!Object.hasOwn(parameters[0], 'type'));
  assert.equal(parameters[1].type, null);
  const lambda = { k: 'lambda', params: parameters, ret: null, body: { k: 'unit' } };
  const owned = readCallableOwnedParameters(lambda);
  owned.forEach((entry, index) => { assert.equal(entry.owner, lambda); assert.equal(entry.parameter, parameters[index]); assert.equal(entry.index, index); });
  const inferred = readCallableLambdaInference(lambda, expected);
  assert.equal(inferred.ret, boolean);
  assert.equal(inferred.declaredRet, false);
  const actual = constructCallableFunctionType([{ type: float }, { type: boolean }], float);
  assert.equal(actual.params[0], float);
  assert.equal(actual.ret, float);
  const other = { kind: 'function', params: [boolean, float], ret: boolean };
  const constraints = readCallableFunctionConstraints(actual, other);
  assert.equal(constraints[0][0], float);
  assert.equal(constraints[0][1], boolean);
  assert.equal(constraints[2][0], float);
  assert.equal(readCallableFunctionConstraints(actual, { ...other, params: [] }), null);
  assert.equal(readCallableApplicationConstraints(actual, [float, boolean], undefined).ret, float);
  const candidate = { kind: 'function', params: [], ret: boolean };
  assert.equal(readCallableApplicationConstraints(actual, [], candidate).pairs[0][1], candidate);
  assert.equal(readCallableApplicationError('float', 0, 0), 'the called value is not a function');
  assert.equal(readCallableApplicationError('function', 2, 1), 'function expects 2 arguments');
});

test('callable closure scope plans preserve lexical order temporal shadowing and exact token spans', () => {
  const outer = { locals: new Set(['outer', 'shadow']), tdz: new Set(['shadow', 'later']), mutable: new Set(['outer']) };
  const tokens = [{ value: 'shadow', start: 7, end: 13 }, { value: 'fresh', start: 15, end: 20 }];
  const scope = readCallableClosureScope(outer, tokens);
  assert.deepEqual([...scope.locals], ['outer', 'shadow', 'fresh']);
  assert.deepEqual([...scope.tdz], ['later']);
  assert.deepEqual([...scope.mutable], ['outer', 'shadow', 'fresh']);
  assert.deepEqual([...outer.tdz], ['shadow', 'later'], 'entry planning leaves the outer lexical state intact');
  assert.deepEqual(constructCallableSurfaceParameters(tokens), [
    { name: 'shadow', type: null, span: { start: 7, end: 13 } },
    { name: 'fresh', type: null, span: { start: 15, end: 20 } },
  ]);
});

test('callable checked capture plans respect let initializers nested lambdas and match binders', () => {
  const type = { kind: 'string' };
  const variable = name => ({ k: 'var', name, type });
  const body = { k: 'let', name: 'local', value: variable('outside'), body: {
    k: 'match', scrutinee: variable('scrutinee'), cases: [{ pattern: { k: 'bind', name: 'bound' }, body: {
      k: 'lambda', params: [{ name: 'parameter' }], body: { k: 'array', items: [variable('local'), variable('bound'), variable('parameter'), variable('captured')] },
    } }],
  } };
  assert.deepEqual([...readCallableCapturedBindings(body, [])], [['outside', type], ['scrutinee', type], ['captured', type]]);
});

test('callable source free parameter renames keep captured and shadowed bindings distinct in both formats', () => {
  const source = 'function choose(base){const outer=()=>base;const inner=base=>base+1;return outer()+inner(2);}console.log(choose(5));';
  const saved = createTypedProgramLinks(source, 'JavaScript');
  for (const binary of [false, true]) {
    const { network, root } = reload(saved, binary);
    assert.deepEqual(output(emitTypedProgramLinks(network, root, 'JavaScript').code), ['8']);
    const inner = constructs(network, 'lambda').find(lambda => network.link(field(network, lambda, 'params')).references().length === 1);
    const parameter = network.link(field(network, inner, 'params')).references()[0];
    assert.ok(readTypedProgramBindingIdentities(network, root).some(identity => identity.equals(parameter)));
    const name = network.link(field(network, parameter, 'name'));
    name.setMetadata(name.metadata().withDefinition('selected'));
    assert.deepEqual(output(emitTypedProgramLinks(network, root, 'JavaScript').code), ['8']);
    assert.match(emitTypedProgramLinks(network, root, 'Rust').code, /selected: f64/u);
    assert.ok(network.links().every(link => !link.metadata().definition?.includes(source)));
    // Retain the outer binding's spelling before redirecting its use to a
    // different lexical owner. The intended failure compares identities;
    // an unrelated renamed spelling would fail earlier as an unbound name.
    name.setMetadata(name.metadata().withDefinition('base'));
    assert.deepEqual(output(emitTypedProgramLinks(network, root, 'JavaScript').code), ['8']);
    const outer = constructs(network, 'lambda').find(lambda => network.link(field(network, lambda, 'params')).references().length === 0);
    redirect(network, field(network, outer, 'body'), 'binding reference', parameter);
    assert.throws(() => decodeTypedProgramLinks(network, root), /outside its scope/u);
  }
});

test('callable source free argument edits and reused parameter descriptors cannot bypass checking', () => {
  const saved = createTypedProgramLinks('function value(base){const first=x=>x+base;const second=x=>x*base;return first(2)+second(3);}console.log(value(4));', 'JavaScript');
  for (const binary of [false, true]) {
    const { network, root } = reload(saved, binary);
    assert.deepEqual(output(emitTypedProgramLinks(network, root, 'JavaScript').code), ['18']);
    const application = constructs(network, 'apply')[0];
    network.link(field(network, application, 'args')).setReferences([]);
    for (const target of ['JavaScript', 'Rust']) assert.throws(() => emitTypedProgramLinks(network, root, target), /wrong number of arguments/u);
    const reused = reload(saved, binary);
    const lambdas = constructs(reused.network, 'lambda');
    const firstParameters = reused.network.link(field(reused.network, lambdas[0], 'params')).references();
    const secondParameters = reused.network.link(field(reused.network, lambdas[1], 'params')).references();
    // Keep every variable target owned after the descriptor edit, so the
    // decoder reaches the multiple-owner proof rather than a dangling edge.
    for (const relation of reused.network.fieldRelations().filter(item => item.label === 'binding reference' && item.child.equals(secondParameters[0]))) redirect(reused.network, relation.parent, 'binding reference', firstParameters[0]);
    reused.network.link(field(reused.network, lambdas[1], 'params')).setReferences(firstParameters);
    assert.throws(() => decodeTypedProgramLinks(reused.network, reused.root), /multiple lexical owners/u);
  }
});

test('callable shared projection plans recheck actual lambdas applications and function references', () => {
  assert.deepEqual(Object.keys(CALLABLE_SOURCE_POLICY.expressionProjections).slice(0, 3), ['functionRef', 'apply', 'lambda']);
  assert.deepEqual(Object.keys(CALLABLE_SOURCE_POLICY.expressionProjections).slice(3), ['cellAllocate', 'cellRead', 'cellWrite']);
  const saved = createTypedProgramLinks('function add(value){return value+1;}function run(value){const callback=x=>add(x)+value;return callback(2);}const fn=add;console.log(run(4)+fn(1));', 'JavaScript');
  for (const binary of [false, true]) {
    const { network, root } = reload(saved, binary);
    const program = decodeTypedProgramLinks(network, root);
    recheckLinkedTypedProgram(program, { wholeProgram: true });
    assert.deepEqual(output(emitTypedProgramLinks(network, root, 'JavaScript').code), ['9']);
    assert.match(emitTypedProgramLinks(network, root, 'Rust').code, /std::rc::Rc::new/u);
  }
});

test('the complete callable kernel translates through checked links without semantic body replacements', () => {
  const source = readCallableSourceModule(readFileSync(new URL('../src/translation/callable-source-rules.js', import.meta.url), 'utf8'));
  const program = checkProgram(parseJavaScript(source, { referenceSemantics: false }));
  const emitted = emitRust(program, { arrayConsumption: true });
  for (const name of ['read_callable_free_references', 'read_callable_statement_uses', 'read_callable_capture_conflicts', 'plan_callable_parameter_ownership', 'plan_callable_type_constraints', 'plan_callable_application_constraints', 'plan_callable_lambda_inference', 'render_callable_lambda']) assert.ok(emitted.definitions.some(definition => definition.includes('pub fn ' + name + '(')), name);
  const header = emitted.text.split('\n').find(line => line.startsWith('#![allow('));
  const raw = [header, ...emitted.preludes, ...emitted.definitions].join('\n\n');
  const library = readPortableSourceParameterStyles(raw);
  assert.match(library.source, /pub fn read_required_number_fact\(values: &\[f64\]/u);
  assert.match(library.source, /pub fn read_required_text_fact\(values: &\[String\]/u);
  assert.doesNotMatch(library.source, /read_required_number_fact\(starts\.clone\(\)/u);
  const parts = partitionPortableRustLibrary(library.source, {directory: 'callable_source_rules'});
  assert.ok(parts.length > 2, 'the complete source is retained across bounded modules');
  assert.ok(parts.every(part => part.source.split('\n').length < 1000));
  const declarations = text => [...text.matchAll(/^pub (?:fn|enum|struct|mod) ([A-Za-z_][A-Za-z_0-9]*)\b/gmu)].map(match => match[1]);
  assert.deepEqual(declarations(parts.map(part => part.source).join('\n')), declarations(raw), 'every emitted declaration survives immutable loans and whole-item placement');
});

test('callable cell plans retain shared and independent mutable identities through source free formats', () => {
  const statements = [
    { s: 'assign', name: 'assigned', value: 1 },
    { k: 'lambda', captures: ['readOnly', 'assigned'], assignedCaptures: [], body: {
      k: 'lambda', captures: ['nested'], assignedCaptures: ['nested'], body: { k: 'unit' },
    } },
  ];
  assert.deepEqual([...readCallableMutableCellNames(statements)], ['assigned', 'nested']);
  for (const [source, expected] of [
    ['function counter(){let n=0;return ()=>{n=n+1;return n;};}const next=counter();console.log(next(),next());', ['1 2']],
    ['function counter(){let n=0;return ()=>{n=n+1;return n;};}const first=counter();const second=counter();console.log(first(),first(),second());', ['1 2 1']],
    ['function value(){let n=0;const add=()=>{n=n+1;return n;};const read=()=>n;add();add();return read();}console.log(value());', ['2']],
  ]) {
    assert.deepEqual(output(source), expected);
    const saved = createTypedProgramLinks(source, 'JavaScript');
    for (const binary of [false, true]) {
      const { network, root } = reload(saved, binary);
      recheckLinkedTypedProgram(decodeTypedProgramLinks(network, root), { wholeProgram: true });
      assert.deepEqual(output(emitTypedProgramLinks(network, root, 'JavaScript').code), expected);
      assert.match(emitTypedProgramLinks(network, root, 'Rust').code, /std::cell::RefCell/u);
      assert.ok(constructs(network, 'cellAllocate').length > 0);
      assert.ok(constructs(network, 'cellRead').length > 0);
      assert.ok(constructs(network, 'cellWrite').length > 0);
    }
  }
});

test('callable required fact columns reject malformed parallel data instead of inventing primitive values', () => {
  assert.throws(() => planCallableSurfaceParameters([0], [1], [], [], 0), /required Number fact column/u);
  assert.throws(() => planCallableSurfaceParameters([0], [], [], [], 0), /required Number fact column/u);
  assert.deepEqual(planCallableSurfaceParameters([], [], [], [], -1), [], 'an absent optional node keeps its existing explicit sentinel');
});

test('callable reconstruction plans preserve lexical shadowing raw cell carriers and metadata identities', () => {
  const span = { start: 1, end: 2 }, type = { kind: 'float' }, data = { name: 'metadata' };
  const named = name => ({ k: 'name', path: [name], span });
  const input = { k: 'array', items: [
    named('value'),
    { k: 'cellRead', cell: named('value'), span },
    { k: 'lambda', params: [{ name: 'value', default: named('value') }], body: named('value'), captures: ['value'], span },
    named('ordinary'),
  ], type, data, span };
  const result = renameCallableSourceExpression(input, [
    { source: 'value', ir: 'old', reference: true },
    { source: 'value', ir: 'actual', reference: true },
    { source: 'ordinary', ir: 'renamed' },
  ]);
  assert.equal(result.items[0].k, 'cellRead');
  assert.deepEqual(result.items[0].cell.path, ['actual']);
  assert.equal(result.items[1].cell.k, 'name', 'an existing cell carrier never becomes a value read');
  assert.deepEqual(result.items[1].cell.path, ['actual']);
  assert.equal(result.items[2].params[0].default.k, 'cellRead', 'defaults retain the outer lexical environment');
  assert.deepEqual(result.items[2].body.path, ['value']);
  assert.equal(result.items[2].body.k, 'name', 'the lambda parameter shadows the outer shared cell');
  assert.deepEqual(result.items[3].path, ['renamed']);
  assert.equal(result.type, type);
  assert.equal(result.data, data);
  assert.equal(result.span, span);
  assert.deepEqual(input.items[0].path, ['value'], 'reconstruction leaves the actual input graph unchanged');
});

test('callable type syntax plans preserve the complete original function forms and exact child slices', () => {
  const archived = readFileSync(new URL('./fixtures/callable-type-original.js', import.meta.url), 'utf8');
  assert.equal(createHash('sha256').update(archived).digest('hex'), 'f58d274c170817ca33e85676db540edabde497298a1cf990bf01dd8904e12777', 'the complete original development oracle remains byte-for-byte authenticated');
  const original = vm.runInNewContext(archived.replaceAll('export function ', 'function ') + '\nreadCompositeType;', {unsupported});
  const parser = { type: text => text };
  const where = {start: 7, end: 41};
  const observe = (read, name) => {
    try { return JSON.stringify(read(parser, name, where)); }
    catch (error) { return JSON.stringify({kind: error.kind, construct: error.construct, reason: error.reason, span: error.span}); }
  };
  for (const name of [
    '(number) => bigint', '(named: number, other: (bigint)=>string) => boolean', '() => number',
    '(name : number : malformed) => string', '(name::number) => string', '((number) => string) => bigint',
    'function(number, string):bigint', 'function ((number)=>string, [number, bigint]): boolean ',
    'function\u00a0( number , ):\u2028string', 'function(): ', 'function():', 'function(n): function(m): number',
    '( number )', '((number)=>bigint)', '(number) => ', '(number)\u2029=>\ufeffnumber ',
    '(number', '(number))', 'function /*not whitespace*/(number):string', 'number', 'number|string',
    '{field: number}', 'Map<string, number>', 'function(name:number):number',
  ]) assert.equal(observe(readCompositeType, name), observe(original, name), name);
  for (const unit of [0, 8, 9, 13, 14, 32, 133, 160, 5760, 6158, 8191, 8192, 8202, 8203, 8232, 8233, 8239, 8287, 12288, 65279, 65535]) {
    assert.equal(acceptCallableTypeWhitespace(unit), /\s/u.test(String.fromCharCode(unit)), String(unit));
  }
  assert.throws(() => planCallableTypeSyntax([40, undefined]), /required Number fact column/u);
});

test('callable JSDoc function types drive genuine checked callbacks and all four target renderers', () => {
  const consoleOutput = code => {
    const lines = [];
    vm.runInNewContext(code, { console: { log: (...values) => lines.push(formatConsole(...values)) }, process: { argv: [] } }, { timeout: 1000 });
    return lines;
  };
  for (const type of ['(bigint)=>bigint', 'function(bigint):bigint', '((value: bigint) => bigint)']) {
    const source = '/** @param {' + type + '} callback @returns {bigint} */function use(callback){return callback(2n);}function increment(value){return value+1n;}console.log(use(increment));';
    assert.deepEqual(consoleOutput(source), ['3n']);
    const saved = createTypedProgramLinks(source, 'JavaScript');
    for (const binary of [false, true]) {
      const {network, root} = reload(saved, binary);
      const program = recheckLinkedTypedProgram(decodeTypedProgramLinks(network, root), {wholeProgram: true});
      assert.deepEqual(consoleOutput(emitTypedProgramLinks(network, root, 'JavaScript').code), ['3n']);
      assert.ok(emitTypedProgramLinks(network, root, 'Rust').code.length > 0);
      for (const emit of [emitLean, emitRocq]) assert.ok(emit(program).text.length > 0);
    }
  }
  assert.equal(readCallableLinkedApplicationError('number', 0, 0), 'a linked application does not target a function');
  assert.equal(readCallableLinkedApplicationError('function', 1, 0), 'a linked application has the wrong number of arguments');
  assert.equal(readCallableLinkedApplicationError('function', 1, 1), '');
});
