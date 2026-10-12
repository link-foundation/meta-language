import assert from 'node:assert/strict';
import vm from 'node:vm';
import { test } from 'node:test';
import { LinkNetwork } from '../src/network.js';
import { createTypedProgramLinks, decodeTypedProgramLinks, encodeTypedProgramLinks, emitTypedProgramLinks, readTypedProgramBindingIdentities } from '../src/typed-program-links.js';
import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';

function run(code) {
  const output = [];
  vm.runInNewContext(code, { console: { log: (...args) => output.push(args.map(String).join(' ')) }, process: { argv: [] } }, { timeout: 1_000 });
  return output;
}
function roundTrip(archive) { return { root: archive.root, network: LinkNetwork.fromLino(archive.network.toLino()) }; }
function field(network, parent, label) {
  return network.fieldRelations().find((relation) => relation.parent.asU64() === parent.asU64() && relation.label === label)?.child;
}
function scalarByField(network, label, value) {
  return network.fieldRelations().find((relation) => relation.label === label && network.link(relation.child).metadata().definition === value)?.child;
}
function construct(network, kind) {
  return network.fieldRelations().find((relation) => relation.label === 'construct kind' && network.link(relation.child).metadata().definition === kind)?.parent;
}
function replaceField(network, parent, label, child) {
  const relation = network.fieldRelations().find((item) => item.parent.equals(parent) && item.label === label);
  assert.ok(relation, `missing ${label} relation`);
  relation.link.setReferences([parent, relation.link.references()[1], child]);
}

for (const [language, source] of [
  ['JavaScript', 'function add(a,b) { return a+b; } console.log(add(2,3));'],
  ['Rust', 'fn add(a: i32,b: i32)->i32 { a+b } fn main() { println!("{}", add(2,3)); }'],
]) {
  test(`${language} source translates through reloaded typed links into JavaScript and Rust`, () => {
    const archive = roundTrip(createTypedProgramLinks(source, language, { provenance: { project: 'reusable numeric module', edition: 'current fixture' } }));
    const program = decodeTypedProgramLinks(archive.network, archive.root);
    assert.equal(program.items[0], program.declarations.get('add'), 'declaration identity and shared references survive serialization');
    assert.ok(archive.network.fieldRelations().some(({ label }) => label === 'declaration reference'));
    assert.ok(archive.network.fieldRelations().some(({ label }) => label === 'binding reference'));
    assert.deepEqual(run(emitTypedProgramLinks(archive.network, archive.root, 'JavaScript').code), ['5']);
    assert.match(emitTypedProgramLinks(archive.network, archive.root, 'Rust').code, /fn add/u);
    assert.ok(archive.network.links().every((link) => !link.metadata().definition?.includes(source)), 'source text is absent from the semantic graph');
  });
}

test('editing one linked operation changes source-free emitted behavior in both targets', () => {
  const archive = roundTrip(createTypedProgramLinks('function calculate(a,b) { return a+b; } console.log(calculate(2,3));', 'JavaScript'));
  const operation = scalarByField(archive.network, 'operation', 'add');
  archive.network.link(operation).setMetadata(archive.network.link(operation).metadata().withDefinition('mul'));
  assert.deepEqual(run(emitTypedProgramLinks(archive.network, archive.root, 'JavaScript').code), ['6']);
  assert.match(emitTypedProgramLinks(archive.network, archive.root, 'Rust').code, /a \* b/u);
});

test('binding identity permits a declaration rename without capture or stale variable names', () => {
  const archive = roundTrip(createTypedProgramLinks('function add(a,b) { return a+b; } console.log(add(2,3));', 'JavaScript'));
  const binding = archive.network.fieldRelations().find(({ label }) => label === 'binding reference');
  const name = field(archive.network, binding.child, 'name');
  archive.network.link(name).setMetadata(archive.network.link(name).metadata().withDefinition('renamed'));
  const emitted = emitTypedProgramLinks(archive.network, archive.root, 'JavaScript');
  assert.match(emitted.code, /function add\(renamed, b\)/u);
  assert.deepEqual(run(emitted.code), ['5']);
});

test('owned binding identities include unused parameters, locals and callback parameters', () => {
  const examples = [
    ['function calculate(value,unused){const constant=1;const increment=()=>value+constant;return increment();}const result=calculate(2,3);console.log(result);', ['constant', 'increment', 'result', 'unused', 'value']],
    ['/** @param {number[]} values */function ones(values){return values.map((unused)=>1);}console.log(ones([1,2]).join(","));', ['unused', 'values']],
  ];
  for (const [source, expected] of examples) {
    const archive = roundTrip(createTypedProgramLinks(source, 'JavaScript'));
    const names = readTypedProgramBindingIdentities(archive.network, archive.root)
      .map((id) => archive.network.link(field(archive.network, id, 'name')).metadata().definition).sort();
    assert.deepEqual(names, expected);
  }
});

test('ignored metadata and disconnected relations cannot counterfeit binder identities', () => {
  const program = checkProgram(parseJavaScript('console.log({left:1});'));
  const record = program.main.effects[0].expr.arg.body;
  const publicField = record.type.fields[0];
  // The same public descriptor can be placed in metadata with convincing
  // function/lambda tags. These are not semantic declarations or operands.
  program.extra = { k: 'fn', params: [publicField] };
  program.main.effects[0].expr.arg.value.extra = {
    k: 'lambda', params: [publicField], body: { k: 'lit', type: { kind: 'float' }, value: '0' },
    type: { kind: 'function', params: [publicField.type], ret: { kind: 'float' } },
  };
  const archive = roundTrip(encodeTypedProgramLinks(program));
  const names = readTypedProgramBindingIdentities(archive.network, archive.root)
    .map((id) => archive.network.link(field(archive.network, id, 'name')).metadata().definition);
  assert.deepEqual(names, ['ml_field1']);
  const fieldName = scalarByField(archive.network, 'name', 'left');
  const descriptor = archive.network.fieldRelations().find((relation) => relation.child.equals(fieldName)).parent;
  const decoy = archive.network.insertLink([], archive.network.link(descriptor).metadata());
  archive.network.insertFieldRelation(decoy, 'binding reference', descriptor);
  assert.deepEqual(readTypedProgramBindingIdentities(archive.network, archive.root)
    .map((id) => archive.network.link(field(archive.network, id, 'name')).metadata().definition), names);
  assert.ok(readTypedProgramBindingIdentities(archive.network, archive.root).every((id) => !id.equals(descriptor)));
});

test('sequential effects and lexical shadowing remain structured after source-free reload', () => {
  const source = 'function calculate(value) { const doubled = value*2; return doubled+1; } const input=3; console.log(calculate(input));';
  const archive = roundTrip(createTypedProgramLinks(source, 'JavaScript'));
  assert.deepEqual(run(emitTypedProgramLinks(archive.network, archive.root, 'JavaScript').code), ['7']);
  const effects = archive.network.fieldRelations().filter(({ label }) => label === 'effects');
  assert.equal(effects.length, 1);
});

test('let binding types survive when the body returns a different type', () => {
  const archive = roundTrip(createTypedProgramLinks('function positive(value){const doubled=value*2;return doubled>0;}console.log(positive(3));', 'JavaScript'));
  assert.deepEqual(run(emitTypedProgramLinks(archive.network, archive.root, 'JavaScript').code), ['true']);
  assert.match(emitTypedProgramLinks(archive.network, archive.root, 'Rust').code, /fn positive\(value: f64\) -> bool/u);
});

for (const [description, source, expected] of [
  ['escaping counter', 'function counter(){let n=0;return ()=>{n=n+1;return n;};}const next=counter();console.log(next(),next());', '1 2'],
  ['shared closure cell', 'function f(){let n=0;const add=()=>{n=n+1;return n;};const read=()=>n;add();add();return read();}console.log(f());', '2'],
  ['independent closure cells', 'function counter(){let n=0;return ()=>{n=n+1;return n;};}const first=counter();const second=counter();console.log(first(),first(),second());', '1 2 1'],
]) {
  test(`${description} retains mutable binding identity in source-free links`, () => {
    const archive = roundTrip(createTypedProgramLinks(source, 'JavaScript'));
    assert.deepEqual(run(emitTypedProgramLinks(archive.network, archive.root, 'JavaScript').code), [expected]);
    assert.match(emitTypedProgramLinks(archive.network, archive.root, 'Rust').code, /RefCell/u);
    assert.ok(archive.network.links().every((link) => !link.metadata().definition?.includes(source)));
  });
}

test('editing a captured-cell increment changes source-free behavior and rejects invalid cell types', () => {
  const archive = roundTrip(createTypedProgramLinks('function counter(){let n=0;return ()=>{n=n+1;return n;};}const next=counter();console.log(next(),next());', 'JavaScript'));
  const increment = scalarByField(archive.network, 'value', '1');
  archive.network.link(increment).setMetadata(archive.network.link(increment).metadata().withDefinition('2'));
  assert.deepEqual(run(emitTypedProgramLinks(archive.network, archive.root, 'JavaScript').code), ['2 4']);
  assert.match(emitTypedProgramLinks(archive.network, archive.root, 'Rust').code, /\+ 2f64/u);
  const cellKind = scalarByField(archive.network, 'kind', 'cell');
  archive.network.link(cellKind).setMetadata(archive.network.link(cellKind).metadata().withDefinition('array'));
  assert.throws(() => emitTypedProgramLinks(archive.network, archive.root, 'JavaScript'), /reference cell/u);
});

test('declaration retargeting preserves checked call signatures and uses actual linked identity', () => {
  const archive = roundTrip(createTypedProgramLinks('function add(a,b){return a+b;} function multiply(a,b){return a*b;} console.log(add(2,3));', 'JavaScript'));
  const reference = archive.network.fieldRelations().find(({ label }) => label === 'declaration reference');
  const declarations = decodeTypedProgramLinks(archive.network, archive.root).declarations;
  assert.equal(declarations.size, 2);
  const declarationName = scalarByField(archive.network, 'full name', 'multiply');
  const target = archive.network.fieldRelations().find(({ child }) => child.asU64() === declarationName.asU64()).parent;
  reference.link.setReferences([reference.parent, reference.link.references()[1], target]);
  assert.deepEqual(run(emitTypedProgramLinks(archive.network, archive.root, 'JavaScript').code), ['6']);
});

test('function reference retargeting rejects incompatible callable signatures', () => {
  const archive = roundTrip(createTypedProgramLinks('function g(a){return a+1;}function h(a,b){return a+b;}const fn=g;console.log(fn(2));', 'JavaScript'));
  assert.deepEqual(run(emitTypedProgramLinks(archive.network, archive.root, 'JavaScript').code), ['3']);
  const functionReference = construct(archive.network, 'functionRef');
  const targetName = scalarByField(archive.network, 'full name', 'h');
  const target = archive.network.fieldRelations().find((relation) => relation.child.equals(targetName)).parent;
  replaceField(archive.network, functionReference, 'declaration reference', target);
  for (const language of ['JavaScript', 'Rust']) {
    assert.throws(() => emitTypedProgramLinks(archive.network, archive.root, language), /inconsistent function reference type/u);
  }
});

for (const [changedField, message] of [
  ['branch', 'conditional branch types'], ['result', 'conditional result type'], ['condition', 'conditional condition type'],
]) {
  test(`conditional ${changedField} type edits cannot bypass checked semantics`, () => {
    const archive = roundTrip(createTypedProgramLinks('/** @param {boolean} flag */function choose(flag){if(flag){return 1;}return 2;}console.log(choose(true)+3);', 'JavaScript'));
    assert.deepEqual(run(emitTypedProgramLinks(archive.network, archive.root, 'JavaScript').code), ['4']);
    const conditional = construct(archive.network, 'if');
    const textKind = scalarByField(archive.network, 'kind', 'string');
    const textType = archive.network.fieldRelations().find((relation) => relation.child.equals(textKind)).parent;
    const owner = changedField === 'branch' ? field(archive.network, conditional, 'then')
      : changedField === 'condition' ? field(archive.network, conditional, 'condition') : conditional;
    replaceField(archive.network, owner, 'type', textType);
    for (const language of ['JavaScript', 'Rust']) {
      assert.throws(() => emitTypedProgramLinks(archive.network, archive.root, language), new RegExp(message, 'u'));
    }
  });
}

test('unary policy and text conversion result edits retain their checked types', () => {
  const source = 'function neg(a){return -a;}console.log(neg(2));';
  const numeric = roundTrip(createTypedProgramLinks(source, 'JavaScript'));
  const policy = field(numeric.network, construct(numeric.network, 'unary'), 'semantics');
  numeric.network.link(policy).setMetadata(numeric.network.link(policy).metadata().withDefinition('exact'));
  assert.throws(() => emitTypedProgramLinks(numeric.network, numeric.root, 'JavaScript'), /inconsistent numeric semantics/u);
  const text = roundTrip(createTypedProgramLinks(source, 'JavaScript'));
  assert.deepEqual(run(emitTypedProgramLinks(text.network, text.root, 'JavaScript').code), ['-2']);
  replaceField(text.network, construct(text.network, 'toString'), 'type', field(text.network, construct(text.network, 'unary'), 'type'));
  assert.throws(() => emitTypedProgramLinks(text.network, text.root, 'JavaScript'), /inconsistent text conversion result type/u);
});

test('admitted math edits change behavior while incompatible result types are refused', () => {
  const archive = roundTrip(createTypedProgramLinks('function root(value){return Math.sqrt(value);}console.log(root(9));', 'JavaScript'));
  const operation = field(archive.network, construct(archive.network, 'math'), 'operation');
  archive.network.link(operation).setMetadata(archive.network.link(operation).metadata().withDefinition('floor'));
  assert.deepEqual(run(emitTypedProgramLinks(archive.network, archive.root, 'JavaScript').code), ['9']);
  archive.network.link(operation).setMetadata(archive.network.link(operation).metadata().withDefinition('isInteger'));
  for (const target of ['JavaScript', 'Rust']) assert.throws(() => emitTypedProgramLinks(archive.network, archive.root, target), /inconsistent math result type/u);
});

test('collection operation edits preserve the checked lookup result type', () => {
  const archive = roundTrip(createTypedProgramLinks('const values=new Map([[1,2]]);console.log(values.get(1));', 'JavaScript'));
  assert.deepEqual(run(emitTypedProgramLinks(archive.network, archive.root, 'JavaScript').code), ['2']);
  const operation = field(archive.network, construct(archive.network, 'collectionRead'), 'operation');
  archive.network.link(operation).setMetadata(archive.network.link(operation).metadata().withDefinition('has'));
  assert.throws(() => emitTypedProgramLinks(archive.network, archive.root, 'JavaScript'), /inconsistent collection read result type/u);
});

test('optional Number predicates retain false for explicitly absent nonnumeric values', () => {
  const archive = roundTrip(createTypedProgramLinks('console.log(Number.isFinite(undefined));console.log(Number.isNaN(null));', 'JavaScript'));
  assert.deepEqual(run(emitTypedProgramLinks(archive.network, archive.root, 'JavaScript').code), ['false', 'false']);
  assert.match(emitTypedProgramLinks(archive.network, archive.root, 'Rust').code, /false/u);
});

test('positional index edits remain numeric fields rather than interpolated target source', () => {
  const archive = roundTrip(createTypedProgramLinks('/** @param {[number,number]} pair */function first(pair){return pair[0];}console.log(first([2,3]));', 'JavaScript'));
  assert.deepEqual(run(emitTypedProgramLinks(archive.network, archive.root, 'JavaScript').code), ['2']);
  const index = field(archive.network, construct(archive.network, 'tupleIndex'), 'index');
  archive.network.link(index).setMetadata(archive.network.link(index).metadata().withTerm('string value')
    .withDefinition('0]; globalThis.injectionObserved=true; [0'));
  for (const target of ['JavaScript', 'Rust']) assert.throws(() => emitTypedProgramLinks(archive.network, archive.root, target), /positional field index is invalid/u);
});

test('constructor immutability survives source-free replay and boolean edits change descriptors', () => {
  const declaration = "/** @typedef {{ $:'value', value:number } | { $:'none' }} Choice */";
  const literal = "{$:'value',value:1}";
  const inspect = (code) => JSON.parse(vm.runInNewContext(`${code}\n(() => {
    const value = make();
    const descriptor = Object.getOwnPropertyDescriptor(value, 'value');
    return JSON.stringify({ frozen: Object.isFrozen(value), extensible: Object.isExtensible(value),
      writable: descriptor.writable, configurable: descriptor.configurable, enumerable: descriptor.enumerable,
      assigned: Reflect.set(value, 'value', 2), value: value.value });
  })()`, {}, { timeout: 1000 }));
  const expected = (frozen) => ({ frozen, extensible: !frozen, writable: !frozen, configurable: !frozen,
    enumerable: true, assigned: !frozen, value: frozen ? 1 : 2 });
  const ordinary = `${declaration}function make(){return ${literal};}`;
  const frozen = `${declaration}function make(){return Object.freeze(${literal});}`;
  assert.deepEqual(inspect(ordinary), expected(false));
  assert.deepEqual(inspect(frozen), expected(true));
  const archive = roundTrip(createTypedProgramLinks(frozen, 'JavaScript'));
  const program = decodeTypedProgramLinks(archive.network, archive.root);
  assert.equal(program.sourceLanguage, 'JavaScript');
  assert.equal(program.declarations.get('make').body.frozen, true);
  assert.deepEqual(inspect(emitTypedProgramLinks(archive.network, archive.root, 'JavaScript').code), expected(true));
  const immutability = field(archive.network, construct(archive.network, 'ctor'), 'constructor immutability');
  assert.ok(immutability, 'constructor immutability is an inspectable boolean relation');
  archive.network.link(immutability).setMetadata(archive.network.link(immutability).metadata().withDefinition('false'));
  const edited = roundTrip(archive);
  assert.equal(decodeTypedProgramLinks(edited.network, edited.root).declarations.get('make').body.frozen, false);
  assert.deepEqual(inspect(emitTypedProgramLinks(edited.network, edited.root, 'JavaScript').code), expected(false));
  const ordinaryArchive = roundTrip(createTypedProgramLinks(ordinary, 'JavaScript'));
  assert.deepEqual(inspect(emitTypedProgramLinks(ordinaryArchive.network, ordinaryArchive.root, 'JavaScript').code), expected(false));
  archive.network.link(immutability).setMetadata(archive.network.link(immutability).metadata().withTerm('string value').withDefinition('false'));
  assert.throws(() => emitTypedProgramLinks(archive.network, archive.root, 'JavaScript'), /immutability flag must be a boolean/u);
});

test('shared type paths are bounded before canonical type equality expands them', () => {
  const program = checkProgram(parseJavaScript('function add(a,b){return a+b;}'));
  let shared = { kind: 'float' };
  for (let index = 0; index < 17; index += 1) shared = { kind: 'tuple', elements: [shared, shared] };
  program.declarations.get('add').params[0].type = shared;
  assert.throws(() => encodeTypedProgramLinks(program), /semantic resource limit/u);
});

test('malformed semantic edits cannot bypass typing, identity or structural bounds', () => {
  const archive = roundTrip(createTypedProgramLinks('function add(a,b) { return a+b; } console.log(add(2,3));', 'JavaScript'));
  const operation = scalarByField(archive.network, 'construct kind', 'float');
  // Type kinds use their own `kind` field, not the expression construct kind.
  const typeKind = scalarByField(archive.network, 'kind', 'float');
  assert.equal(operation, undefined);
  archive.network.link(typeKind).setMetadata(archive.network.link(typeKind).metadata().withDefinition('string'));
  assert.throws(() => emitTypedProgramLinks(archive.network, archive.root, 'Rust'), TypeError);
  const cyclic = roundTrip(createTypedProgramLinks('function add(a,b){return a+b;} console.log(add(2,3));', 'JavaScript'));
  const body = field(cyclic.network, cyclic.network.link(cyclic.root).references()[0], 'items');
  cyclic.network.link(body).setReferences([body]);
  assert.throws(() => decodeTypedProgramLinks(cyclic.network, cyclic.root), /structural cycle/u);
  assert.throws(() => decodeTypedProgramLinks(cyclic.network, cyclic.root, { nodeLimit: 2 }), /resource limit/u);
});

test('generic typed codec retains host data sharing and refuses callback payloads', () => {
  const program = checkProgram(parseJavaScript('function add(a,b){return a+b;}'));
  const archive = roundTrip(encodeTypedProgramLinks(program));
  const restored = decodeTypedProgramLinks(archive.network, archive.root);
  assert.equal(restored.items[0].ret, restored.items[0].params[0].type);
  program.callback = () => 1;
  assert.throws(() => encodeTypedProgramLinks(program), /host callbacks/u);
});

test('edited literal data cannot become host source code during emission', () => {
  const archive = roundTrip(createTypedProgramLinks('console.log(3);', 'JavaScript'));
  const literal = scalarByField(archive.network, 'value', '3');
  archive.network.link(literal).setMetadata(archive.network.link(literal).metadata().withDefinition('0); injected(); (0'));
  assert.throws(() => emitTypedProgramLinks(archive.network, archive.root, 'JavaScript'), /floating-point literal is invalid/u);
});

test('floating-point literal edits admit canonical specials and both exponent spellings', () => {
  for (const [value, expected] of [['1e2', '100'], ['1E2', '100'], ['NaN', 'NaN'], ['Infinity', 'Infinity'], ['-Infinity', '-Infinity']]) {
    const archive = roundTrip(createTypedProgramLinks('console.log(3);', 'JavaScript'));
    const literal = scalarByField(archive.network, 'value', '3');
    archive.network.link(literal).setMetadata(archive.network.link(literal).metadata().withDefinition(value));
    assert.deepEqual(run(emitTypedProgramLinks(archive.network, archive.root, 'JavaScript').code), [expected]);
  }
  for (const value of ['nan', 'NAN', 'infinity', '-infinity']) {
    const archive = roundTrip(createTypedProgramLinks('console.log(3);', 'JavaScript'));
    const literal = scalarByField(archive.network, 'value', '3');
    archive.network.link(literal).setMetadata(archive.network.link(literal).metadata().withDefinition(value));
    for (const target of ['JavaScript', 'Rust']) assert.throws(() => emitTypedProgramLinks(archive.network, archive.root, target), /floating-point literal is invalid/u);
  }
});
