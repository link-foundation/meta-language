import assert from 'node:assert/strict';
import vm from 'node:vm';
import test from 'node:test';
import { LinkNetwork } from '../src/network.js';
import { createTypedProgramLinks, decodeTypedProgramLinks, emitTypedProgramLinks } from '../src/typed-program-links.js';
import { encodeTypedProgramLinks } from '../src/typed-program-links.js';
import { checkProgram } from '../src/translation/check.js';
import { parseRust } from '../src/translation/rust.js';
import { readProgramNeutralFieldDefault, readProgramReferenceSourceField, readProgramReferenceTargetField, readProgramReferenceRole, acceptProgramReferenceField } from '../src/typed-program-link-rules.js';

const source = 'enum Value { Empty, Full(i64) } fn make() -> Value { Value::Full(7) } fn main() {}';
const reload = archive => ({ ...archive, network: LinkNetwork.fromLino(archive.network.toLino()) });
const field = (network, parent, label) => network.fieldRelations().find(relation => relation.parent.equals(parent) && relation.label === label);
const scalar = (network, id) => network.link(id).metadata().definition;
const construct = (network, kind) => network.fieldRelations().find(relation => relation.label === 'construct kind' && scalar(network, relation.child) === kind).parent;
const rename = (network, id, name) => network.link(id).setMetadata(network.link(id).metadata().withDefinition(name));
const retarget = (relation, target) => relation.link.setReferences([relation.parent, relation.link.references()[1], target]);
const value = archive => vm.runInNewContext(`${emitTypedProgramLinks(archive.network, archive.root, 'JavaScript').code}\nmake().field0`, {}, { timeout: 1000 });

test('constructor expressions retain their datatype and constructor as distinct owned references', () => {
  const archive = reload(createTypedProgramLinks(source, 'Rust'));
  const expression = construct(archive.network, 'ctor');
  const datatype = field(archive.network, expression, 'data type declaration reference').child;
  const constructor = field(archive.network, expression, 'constructor declaration reference').child;
  assert.equal(scalar(archive.network, field(archive.network, datatype, 'construct kind').child), 'data');
  assert.equal(scalar(archive.network, field(archive.network, constructor, 'name').child), 'Full');
  assert.equal(field(archive.network, expression, 'data'), undefined);
  assert.equal(field(archive.network, expression, 'constructor'), undefined);
  assert.equal(value(archive), 7n);
  assert.ok(archive.network.links().every(link => !link.metadata().definition?.includes(source)));
});

test('one constructor declaration rename updates every source-free construction after a second reload', () => {
  let archive = reload(createTypedProgramLinks(source, 'Rust'));
  const expression = construct(archive.network, 'ctor');
  const constructor = field(archive.network, expression, 'constructor declaration reference').child;
  rename(archive.network, field(archive.network, constructor, 'name').child, 'Packed');
  archive = reload(archive);
  const program = decodeTypedProgramLinks(archive.network, archive.root);
  assert.equal(program.declarations.get('make').body.ctor, 'Packed');
  assert.equal(value(archive), 7n);
  assert.match(emitTypedProgramLinks(archive.network, archive.root, 'Rust').code, /Value::Packed\(7i64\)/u);
});

test('datatype spelling changes preserve owned references through signatures and constructor result types', () => {
  let archive = reload(createTypedProgramLinks(source, 'Rust'));
  const expression = construct(archive.network, 'ctor');
  const datatype = field(archive.network, expression, 'data type declaration reference').child;
  rename(archive.network, field(archive.network, datatype, 'name').child, 'Renamed');
  rename(archive.network, field(archive.network, datatype, 'full name').child, 'Renamed');
  const mapping = archive.network.fieldRelations().find(relation => relation.label === 'declaration mapping').child;
  const pair = archive.network.link(mapping).references().find(id => archive.network.link(id).references()[1].equals(datatype));
  rename(archive.network, archive.network.link(pair).references()[0], 'Renamed');
  archive = reload(archive);
  const program = decodeTypedProgramLinks(archive.network, archive.root);
  const make = program.declarations.get('make');
  assert.equal(make.ret.name, 'Renamed');
  assert.equal(make.body.data, 'Renamed');
  assert.equal(make.body.type.name, 'Renamed');
  assert.equal(value(archive), 7n);
  assert.match(emitTypedProgramLinks(archive.network, archive.root, 'Rust').code, /Renamed::Full\(7i64\)/u);
});

test('a same-named constructor from another datatype cannot replace its actual owned constructor', () => {
  const archive = reload(createTypedProgramLinks('enum Left { Full(i64) } enum Right { Full(i64) } fn make() -> Left { Left::Full(7) } fn main() {}', 'Rust'));
  const expression = construct(archive.network, 'ctor');
  const foreign = archive.network.fieldRelations().find(relation => relation.label === 'full name' && scalar(archive.network, relation.child) === 'Right').parent;
  const constructors = field(archive.network, foreign, 'constructors').child;
  retarget(field(archive.network, expression, 'constructor declaration reference'), archive.network.link(constructors).references()[0]);
  for (const target of ['JavaScript', 'Rust']) assert.throws(() => emitTypedProgramLinks(archive.network, archive.root, target), /different declaration/u);
});

test('datatype references cannot target functions and duplicate scalar/reference fields are refused', () => {
  const archive = reload(createTypedProgramLinks(source, 'Rust'));
  const expression = construct(archive.network, 'ctor');
  const function_ = construct(archive.network, 'fn');
  retarget(field(archive.network, expression, 'data type declaration reference'), function_);
  assert.throws(() => decodeTypedProgramLinks(archive.network, archive.root), /datatype declaration/u);
  const duplicate = reload(createTypedProgramLinks(source, 'Rust'));
  const constructor = construct(duplicate.network, 'ctor');
  const datatype = field(duplicate.network, constructor, 'data type declaration reference').child;
  duplicate.network.insertFieldRelation(constructor, 'data', field(duplicate.network, datatype, 'full name').child);
  assert.throws(() => decodeTypedProgramLinks(duplicate.network, duplicate.root), /duplicate source field/u);
});

test('constructor arity and duplicate declaration names remain checked before any target emission', () => {
  const archive = reload(createTypedProgramLinks(source, 'Rust'));
  const expression = construct(archive.network, 'ctor');
  archive.network.link(field(archive.network, expression, 'arguments').child).setReferences([]);
  assert.throws(() => decodeTypedProgramLinks(archive.network, archive.root), /wrong number of arguments/u);
  const duplicate = reload(createTypedProgramLinks(source, 'Rust'));
  const datatype = construct(duplicate.network, 'data');
  const constructors = duplicate.network.link(field(duplicate.network, datatype, 'constructors').child).references();
  rename(duplicate.network, field(duplicate.network, constructors[0], 'name').child, 'Full');
  assert.throws(() => decodeTypedProgramLinks(duplicate.network, duplicate.root), /duplicate constructors/u);
});

test('normalized field aliases cannot overwrite one another during graph reload', () => {
  const archive = reload(createTypedProgramLinks(source, 'Rust'));
  const datatype = construct(archive.network, 'data');
  archive.network.insertFieldRelation(datatype, 'fullName', field(archive.network, datatype, 'full name').child);
  assert.throws(() => decodeTypedProgramLinks(archive.network, archive.root), /duplicate normalized field/u);
});

test('one constructor record cannot belong to two nominal datatype declarations', () => {
  const archive = reload(createTypedProgramLinks('enum Left { Full(i64) } enum Right { Full(i64) } fn make() -> Left { Left::Full(7) } fn main() {}', 'Rust'));
  const datatypes = ['Left', 'Right'].map(name => archive.network.fieldRelations().find(relation => relation.label === 'full name' && scalar(archive.network, relation.child) === name).parent);
  const constructors = datatypes.map(datatype => field(archive.network, datatype, 'constructors').child);
  archive.network.link(constructors[1]).setReferences(archive.network.link(constructors[0]).references());
  assert.throws(() => decodeTypedProgramLinks(archive.network, archive.root), /multiple datatype owners/u);
});

test('forged effect or proposition roles cannot bypass constructor arity checking', () => {
  const effects = checkProgram(parseRust(source));
  effects.declarations.get('make').body.args = [];
  effects.declarations.get('make').body.effects = [];
  assert.throws(() => encodeTypedProgramLinks(effects), /wrong number of arguments/u);
  const proposition = checkProgram(parseRust(source));
  Object.assign(proposition.declarations.get('make').body, { args: [], p: 'forall', binders: [], body: { p: 'bool', expr: { k: 'lit', type: { kind: 'bool' }, value: true } } });
  assert.throws(() => encodeTypedProgramLinks(proposition), /conflicting expression and proposition/u);
});

test('reference collection bounds ignored pattern metadata before expanding shared paths', () => {
  const program = checkProgram(parseRust('enum Value { Full(i64) } fn read(x: Value) -> i64 { match x { Value::Full(value) => value } } fn main() {}'));
  let repeated = {};
  for (let depth = 0; depth < 18; depth += 1) repeated = { left: repeated, right: repeated };
  program.declarations.get('read').body.cases[0].pattern.extra = repeated;
  assert.throws(() => encodeTypedProgramLinks(program), /reference resource limit/u);
});

test('generated reference and neutral-field policies distinguish datatype names from constructor data fields', () => {
  assert.equal(readProgramReferenceSourceField('datatype declaration reference', '', 'data'), 'name');
  assert.equal(readProgramReferenceSourceField('datatype declaration reference', 'ctor', ''), 'data');
  assert.equal(readProgramReferenceSourceField('constructor declaration reference', 'ctor', ''), 'ctor');
  assert.equal(readProgramReferenceSourceField('constructor declaration reference', '', 'data'), '');
  assert.equal(readProgramReferenceTargetField('datatype declaration reference'), 'fullName');
  assert.equal(readProgramReferenceRole('data type declaration reference'), 'datatype');
  assert.equal(readProgramReferenceRole('datatype declaration reference'), 'datatype');
  assert.equal(acceptProgramReferenceField('data type declaration reference'), true);
  assert.equal(acceptProgramReferenceField('datatype declaration reference'), true);
  assert.equal(readProgramReferenceSourceField('data type declaration reference', '', 'data'), 'name');
  assert.equal(readProgramReferenceSourceField('data type declaration reference', 'ctor', ''), 'data');
  assert.equal(readProgramReferenceTargetField('data type declaration reference'), 'fullName');
  assert.equal(readProgramReferenceTargetField('constructor declaration reference'), 'name');
  assert.equal(readProgramNeutralFieldDefault('expression:ctor', 'frozen'), 'false');
  assert.equal(readProgramNeutralFieldDefault('pattern:ctor', 'frozen'), '');
  assert.equal(readProgramNeutralFieldDefault('declaration:data', 'generated'), 'false');
  assert.equal(readProgramNeutralFieldDefault('expression:ctor', 'generated'), '');
  assert.equal(readProgramNeutralFieldDefault('expression:ctor', 'span'), 'null');
  assert.equal(readProgramNeutralFieldDefault('type:data', 'span'), '');
});

test('the former datatype relation spelling reloads as an explicit alias of the canonical role', () => {
  let archive = reload(createTypedProgramLinks(source, 'Rust'));
  const canonical = archive.network.fieldRelations().filter(relation => relation.label === 'data type declaration reference');
  assert.ok(canonical.length > 0);
  for (const relation of canonical) {
    const label = relation.link.references()[1];
    archive.network.link(label).setMetadata(archive.network.link(label).metadata().withTerm('datatype declaration reference'));
  }
  archive = reload(archive);
  assert.ok(archive.network.fieldRelations().some(relation => relation.label === 'datatype declaration reference'));
  assert.equal(value(archive), 7n);
  assert.equal(decodeTypedProgramLinks(archive.network, archive.root).declarations.get('make').body.data, 'Value');
  assert.match(emitTypedProgramLinks(archive.network, archive.root, 'Rust').code, /Value::Full\(7i64\)/u);
});

test('canonical and alias datatype relations cannot claim the same source field twice', () => {
  const archive = reload(createTypedProgramLinks(source, 'Rust'));
  const expression = construct(archive.network, 'ctor');
  const target = field(archive.network, expression, 'data type declaration reference').child;
  archive.network.insertFieldRelation(expression, 'datatype declaration reference', target);
  assert.throws(() => decodeTypedProgramLinks(archive.network, archive.root), /duplicate source field/u);
});

test('an uninhabited datatype retains its declaration and identity-function signature', () => {
  const archive = reload(createTypedProgramLinks('enum Never {} fn identity(x: Never) -> Never { x } fn main() {}', 'Rust'));
  const program = decodeTypedProgramLinks(archive.network, archive.root);
  assert.deepEqual(program.declarations.get('Never').ctors, []);
  assert.equal(program.declarations.get('identity').ret.name, 'Never');
  assert.equal(vm.runInNewContext(`${emitTypedProgramLinks(archive.network, archive.root, 'JavaScript').code}\ntypeof identity`, {}, { timeout: 1000 }), 'function');
  assert.match(emitTypedProgramLinks(archive.network, archive.root, 'Rust').code, /enum Never/u);
});

test('ordered declaration items cannot be missing, duplicated or replaced by an unowned copy', () => {
  for (const mutation of ['missing', 'duplicate', 'unowned']) {
    const program = checkProgram(parseRust(source));
    if (mutation === 'missing') program.items = [];
    else if (mutation === 'duplicate') program.items.push(program.items[0]);
    else program.items[0] = { ...program.items[0] };
    assert.throws(() => encodeTypedProgramLinks(program), /ordered items|unowned declaration/u, mutation);
  }
});

test('ordered item ownership retains qualified datatype and function declarations inside modules', () => {
  const archive = reload(createTypedProgramLinks('mod Inner { pub enum Value { Full(i64) } pub fn make() -> Value { Value::Full(7) } } fn main() {}', 'Rust'));
  const program = decodeTypedProgramLinks(archive.network, archive.root);
  assert.equal(program.items[0].k, 'module');
  assert.equal(program.items[0].items[0], program.declarations.get('Inner.Value'));
  assert.equal(program.items[0].items[1], program.declarations.get('Inner.make'));
  assert.equal(vm.runInNewContext(`${emitTypedProgramLinks(archive.network, archive.root, 'JavaScript').code}\nInner.make().field0`, {}, { timeout: 1000 }), 7n);
});
