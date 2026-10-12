#!/usr/bin/env node
// Source-free algebraic examples with independent JavaScript source behavior.
import assert from 'node:assert/strict';
import { fileURLToPath } from 'node:url';
import vm from 'node:vm';
import { checkProjectData, writeProjectData } from './project-data.mjs';
import { LinkId } from '../src/primitives.js';
import { LinkNetwork } from '../src/network.js';
import { createTypedProgramLinks, decodeTypedProgramLinks, emitTypedProgramLinks, TYPED_PROGRAM_LINKS_SCHEMA } from '../src/typed-program-links.js';

const referenceLabels = TYPED_PROGRAM_LINKS_SCHEMA.references;

const inputs = [
  {
    id: 'choice', output: ['7', '99', '3'],
    rust: 'enum Choice{Empty,Value(i32),Pair(i32,i32)} enum Other{Value(i32)} fn read(item:Choice,shadow:i32)->i32{match item{Choice::Empty=>shadow,Choice::Value(shadow)=>shadow,Choice::Pair(shadow,_)=>shadow}} fn main(){println!("{}",read(Choice::Value(7),99));println!("{}",read(Choice::Empty,99));println!("{}",read(Choice::Pair(3,8),99));}',
    javascript: `/** @typedef {{ $:'Empty' } | { $:'Value', field0:number } | { $:'Pair', field0:number, field1:number }} Choice */
/** @typedef {{ $:'Value', field0:number }} Other */
/** @param {Choice} item @param {number} shadow @returns {number} */
function read(item,shadow){switch(item.$){case 'Empty':return shadow;case 'Value':return item.field0;case 'Pair':return item.field0;}}
console.log(read({$:'Value',field0:7},99));console.log(read({$:'Empty'},99));console.log(read({$:'Pair',field0:3,field1:8},99));`,
  },
  {
    id: 'tree', output: ['9'],
    rust: 'enum Tree{Leaf(i32),Branch(Box<Tree>,Box<Tree>)} fn sum(tree:Tree)->i32{match tree{Tree::Leaf(value)=>value,Tree::Branch(left,right)=>sum(*left)+sum(*right)}} fn main(){println!("{}",sum(Tree::Branch(Box::new(Tree::Leaf(2)),Box::new(Tree::Branch(Box::new(Tree::Leaf(3)),Box::new(Tree::Leaf(4)))))));}',
    javascript: `/** @typedef {{ $:'Leaf', field0:number } | { $:'Branch', field0:Tree, field1:Tree }} Tree */
/** @param {Tree} tree @returns {number} */
function sum(tree){if(tree.$==='Leaf')return tree.field0;return sum(tree.field0)+sum(tree.field1);}
console.log(sum({$:'Branch',field0:{$:'Leaf',field0:2},field1:{$:'Branch',field0:{$:'Leaf',field0:3},field1:{$:'Leaf',field0:4}}}));`,
  },
  {
    id: 'natural', output: ['2'],
    rust: 'enum Natural{Zero,Succ(Box<Natural>)} fn count(value:Natural)->i32{match value{Natural::Zero=>0,Natural::Succ(previous)=>1+count(*previous)}} fn main(){println!("{}",count(Natural::Succ(Box::new(Natural::Succ(Box::new(Natural::Zero))))));}',
    javascript: `/** @typedef {{ $:'Zero' } | { $:'Succ', field0:Natural }} Natural */
/** @param {Natural} value @returns {number} */
function count(value){if(value.$==='Zero')return 0;return 1+count(value.field0);}
console.log(count({$:'Succ',field0:{$:'Succ',field0:{$:'Zero'}}}));`,
  },
];

function execute(code) {
  const output = [];
  vm.runInNewContext(code, { console: { log: (...values) => output.push(values.map(String).join(' ')) }, process: { argv: [] } }, { timeout: 1000 });
  return output;
}

function graphCase(input, language) {
  const source = input[language === 'Rust' ? 'rust' : 'javascript'];
  const archive = createTypedProgramLinks(source, language);
  const network = LinkNetwork.fromLino(archive.network.toLino());
  const javascript = emitTypedProgramLinks(network, archive.root, 'JavaScript').code;
  const rust = emitTypedProgramLinks(network, archive.root, 'Rust').code;
  assert.deepEqual(execute(input.javascript), input.output, 'the independent original JavaScript source defines the observations');
  assert.deepEqual(execute(javascript), input.output, 'source-free target execution retains the observations');
  const decoded = decodeTypedProgramLinks(network, archive.root);
  assert.equal(decoded.items[0], decoded.declarations.get(decoded.items[0].fullName));
  assert.ok([...network.links()].every(link => !link.metadata().definition?.includes(source)), 'the graph carries no original source');
  return { id: `${language.toLowerCase()}-${input.id}`, program: input.id, language, source,
    oracleSource: input.javascript, output: [...input.output], root: archive.root.asU64(), lino: network.toLino(), javascript, rust };
}

function graph(case_) {
  const network = LinkNetwork.fromLino(case_.lino);
  const relations = network.fieldRelations();
  const field = (parent, label) => {
    const value = relations.find(relation => relation.parent.equals(parent) && relation.label === label)?.child;
    assert.ok(value, `missing linked ${label}`); return value;
  };
  const scalar = id => network.link(id).metadata().definition;
  const records = kind => relations.filter(relation => relation.label === 'construct kind' && scalar(relation.child) === kind).map(relation => relation.parent);
  const datatype = records('data')[0];
  const constructors = network.link(field(datatype, 'constructors')).references();
  const constructor = name => constructors.find(id => scalar(field(id, 'name')) === name);
  const match = records('match')[0];
  const cases = network.link(field(match, 'cases')).references();
  const pattern = name => cases.map(caseId => field(caseId, 'pattern')).find(id => field(id, referenceLabels.constructor).equals(constructor(name)));
  const binders = patternId => network.link(field(patternId, 'pattern bindings')).references();
  return { network, root: LinkId.fromU64(case_.root), field, scalar, records, datatype, constructors, constructor, match, cases, pattern, binders };
}

/** Apply only explicit link edits; no source or decoded host program participates. */
export function applyAlgebraicProgramEdits(network, edits) {
  for (const edit of edits) {
    const link = network.link(LinkId.fromU64(edit.link));
    assert.ok(link, 'an edit references an existing link');
    if (edit.kind === 'references') link.setReferences(edit.values.map(LinkId.fromU64));
    else if (edit.kind === 'definition') link.setMetadata(link.metadata().withDefinition(edit.value));
    else if (edit.kind === 'language') link.setMetadata(link.metadata().withLanguage(edit.value));
    else if (edit.kind === 'term') link.setMetadata(link.metadata().withTerm(edit.value));
    else throw new TypeError('unknown linked algebraic edit');
  }
}

const definition = (link, value) => ({ kind: 'definition', link: link.asU64(), value });
const references = (link, values) => ({ kind: 'references', link: link.asU64(), values: values.map(id => id.asU64()) });
function replaceField(graph, parent, label, child) {
  const relation = graph.network.fieldRelations().find(value => value.parent.equals(parent) && value.label === label);
  assert.ok(relation, `missing ${label} relation`);
  return references(relation.link.id(), [parent, relation.link.references()[1], child]);
}

function edits(cases) {
  const choice = cases.find(case_ => case_.id === 'rust-choice');
  const tree = cases.find(case_ => case_.id === 'rust-tree');
  const c = graph(choice);
  const t = graph(tree);
  const binding = c.binders(c.pattern('Value'))[0];
  assert.equal(c.network.link(binding).metadata().term, 'program record', 'a case binder is an owned linked record');
  const mapping = c.field(c.network.link(c.root).references()[0], 'declaration mapping');
  const mapKey = c.network.link(mapping).references().find(id => c.network.link(id).references()[1].equals(c.datatype));
  assert.ok(mapKey, 'the datatype has an explicit declaration mapping entry');
  const literal = t.network.fieldRelations().find(relation => relation.label === 'value' && t.scalar(relation.child) === '2')?.child;
  assert.ok(literal, 'the selected original tree leaf is linked');
  const ignoredSlot = [...c.network.links()].find(link => link.metadata().term === 'null value')?.id();
  assert.ok(ignoredSlot, 'the selected ignored slot uses an existing linked null primitive');
  return [
    { id: 'constructor-rename', case: choice.id, edits: [definition(c.field(c.constructor('Value'), 'name'), 'Picked')],
      output: choice.output, oracleSource: choice.oracleSource.replace("$:'Value', field0:number", "$:'Picked', field0:number")
        .replace("case 'Value'", "case 'Picked'").replace("{$:'Value',field0:7}", "{$:'Picked',field0:7}"),
      nativeOracle: choice.source.replace('Value(i32)', 'Picked(i32)').replaceAll('Choice::Value', 'Choice::Picked') },
    { id: 'datatype-rename', case: choice.id, edits: [
      definition(c.field(c.datatype, 'name'), 'Selection'), definition(c.field(c.datatype, 'full name'), 'Selection'),
      definition(c.network.link(mapKey).references()[0], 'Selection'),
    ], output: choice.output, oracleSource: choice.oracleSource.replaceAll('Choice', 'Selection'), nativeOracle: choice.source.replaceAll('Choice', 'Selection') },
    { id: 'pattern-binder-shadow', case: choice.id, edits: [definition(c.field(binding, 'name'), 'item')],
      output: choice.output, oracleSource: choice.oracleSource,
      nativeOracle: choice.source.replace('Choice::Value(shadow)=>shadow', 'Choice::Value(item)=>item') },
    { id: 'constructor-field-rename', case: choice.id, edits: [definition(c.field(c.network.link(c.field(c.constructor('Value'), 'fields')).references()[0], 'name'), 'payload')],
      output: choice.output, oracleSource: choice.oracleSource.replace("$:'Value', field0:number", "$:'Value', payload:number")
        .replace("case 'Value':return item.field0", "case 'Value':return item.payload").replace("{$:'Value',field0:7}", "{$:'Value',payload:7}"), nativeOracle: choice.source },
    { id: 'ignored-constructor-slot', case: choice.id, edits: [references(c.field(c.pattern('Pair'), 'pattern bindings'), [c.binders(c.pattern('Pair'))[0], ignoredSlot])],
      output: choice.output, oracleSource: choice.oracleSource, nativeOracle: choice.source },
    { id: 'recursive-tree-leaf-edit', case: tree.id, edits: [definition(literal, '6')], output: ['13'],
      oracleSource: tree.oracleSource.replace('field0:2', 'field0:6'), nativeOracle: tree.source.replace('Tree::Leaf(2)', 'Tree::Leaf(6)') },
  ];
}

function invalidEdits(cases) {
  const choice = cases.find(case_ => case_.id === 'rust-choice');
  const c = graph(choice);
  const valuePattern = c.pattern('Value');
  const valueBinder = c.binders(valuePattern)[0];
  const pairPattern = c.pattern('Pair');
  const pairBinder = c.binders(pairPattern)[0];
  const value = c.records('ctor').find(parent => c.network.fieldRelations().some(relation => relation.parent.equals(parent) && relation.label === 'arguments')
    && c.field(parent, referenceLabels.constructor).equals(c.constructor('Value')));
  const variable = c.network.fieldRelations().find(relation => relation.label === referenceLabels.binding && relation.child.equals(valueBinder))?.parent;
  assert.ok(variable, 'the selected pattern body uses its actual owned binder');
  const other = c.records('data').find(parent => !parent.equals(c.datatype));
  assert.ok(other, 'a distinct owned datatype has a structurally similar constructor');
  const foreignConstructor = c.network.link(c.field(other, 'constructors')).references()[0];
  const parameters = c.network.link(c.field(c.records('fn')[0], 'parameters')).references();
  const invalid = [
    { id: 'foreign-datatype-reference', edits: [replaceField(c, value, referenceLabels.datatype, c.records('fn')[0])] },
    { id: 'foreign-constructor-owner', edits: [replaceField(c, value, referenceLabels.constructor, foreignConstructor)] },
    { id: 'constructor-arity', edits: [references(c.field(value, 'arguments'), [])] },
    { id: 'structural-cycle', edits: [references(c.field(value, 'arguments'), [value])] },
    { id: 'constructor-argument-type', edits: [replaceField(c, c.network.link(c.field(value, 'arguments')).references()[0], 'type', c.field(parameters[0], 'type'))] },
    { id: 'pattern-arity', edits: [references(c.field(pairPattern, 'pattern bindings'), [pairBinder])] },
    { id: 'duplicate-pattern-bindings', edits: [references(c.field(pairPattern, 'pattern bindings'), [pairBinder, pairBinder])] },
    { id: 'pattern-binding-type', edits: [replaceField(c, valueBinder, 'type', c.field(parameters[0], 'type'))] },
    { id: 'foreign-pattern-scope', edits: [definition(c.field(pairBinder, 'name'), c.scalar(c.field(valueBinder, 'name'))), replaceField(c, variable, referenceLabels.binding, pairBinder)] },
    { id: 'nonexhaustive-match', edits: [references(c.field(c.match, 'cases'), c.cases.slice(0, 2))] },
    { id: 'duplicate-constructor-name', edits: [definition(c.field(c.constructor('Pair'), 'name'), 'Value')] },
    { id: 'foreign-constructor-metadata', edits: [{ kind: 'language', link: c.constructor('Value').asU64(), value: 'foreign typed model' }] },
    { id: 'counterfeit-binding-metadata', edits: [{ kind: 'term', link: valueBinder.asU64(), value: 'string value' }] },
  ];
  return invalid.map(entry => ({ ...entry, case: choice.id }));
}

export function typedProgramAlgebraicFixture() {
  const cases = inputs.flatMap(input => ['Rust', 'JavaScript'].map(language => graphCase(input, language)));
  // Recorded observations remain independent arrays, as the previous JSON
  // storage decoded them; shared Links identities belong to the program graphs.
  const mutations = edits(cases).map(mutation => ({ ...mutation, output: [...mutation.output] }));
  for (const mutation of mutations) {
    const original = cases.find(case_ => case_.id === mutation.case);
    const archive = graph(original);
    applyAlgebraicProgramEdits(archive.network, mutation.edits);
    assert.deepEqual(execute(mutation.oracleSource), mutation.output);
    mutation.javascript = emitTypedProgramLinks(archive.network, archive.root, 'JavaScript').code;
    mutation.rust = emitTypedProgramLinks(archive.network, archive.root, 'Rust').code;
    assert.deepEqual(execute(mutation.javascript), mutation.output, mutation.id);
  }
  const invalid = invalidEdits(cases);
  for (const mutation of invalid) for (const target of ['JavaScript', 'Rust']) {
    const archive = graph(cases.find(case_ => case_.id === mutation.case));
    applyAlgebraicProgramEdits(archive.network, mutation.edits);
    assert.throws(() => emitTypedProgramLinks(archive.network, archive.root, target), undefined, mutation.id);
  }
  return { schemaVersion: 1, generatedBy: 'js/scripts/generate-typed-program-algebraic-fixtures.mjs', cases, mutations, invalid };
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  const path = new URL('../../parity/fixtures/typed-program-algebraic.json', import.meta.url);
  const fixture = typedProgramAlgebraicFixture();
  if (process.argv.includes('--check')) checkProjectData(path, fixture);
  else writeProjectData(path, fixture);
}
