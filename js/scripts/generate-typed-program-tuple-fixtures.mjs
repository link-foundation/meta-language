#!/usr/bin/env node
// Independent source programs define the tuple observations; native compilers
// run only in CI. The generator edits semantic links without source envelopes.
import assert from 'node:assert/strict';
import { fileURLToPath } from 'node:url';
import vm from 'node:vm';
import { checkProjectData, writeProjectData } from './project-data.mjs';
import { LinkNetwork } from '../src/network.js';
import { LinkId } from '../src/primitives.js';
import { createTypedProgramLinks, decodeTypedProgramLinks, emitTypedProgramLinks, TYPED_PROGRAM_LINKS_SCHEMA } from '../src/typed-program-links.js';
import { parseRust } from '../src/translation/rust.js';

const copyJavaScript = `/** @returns {[number,boolean]} */
function make(){return Object.freeze([5,true]);}
/** @param {[number,boolean]} pair @returns {number} */
function choose(pair){const value=pair[0];const flag=pair[1];if(flag)return value+2;return value;}
const pair=make();console.log(choose(pair)===7);console.log(pair[0]===5);`;
const copyRust = 'fn make()->(f64,bool){(5.0,true)} fn choose(pair:(f64,bool))->f64{let(value,flag)=pair;if flag{value+2.0}else{value}} fn main(){let pair=make();println!("{}",choose(pair)==7.0);println!("{}",pair.0==5.0);}';
const nestedJavaScript = `/** @returns {[[number,boolean],number]} */
function make(){return Object.freeze([Object.freeze([5,true]),2]);}
/** @param {[[number,boolean],number]} pair @returns {number} */
function choose(pair){const value=pair[0][0];const flag=pair[0][1];const increment=pair[1];if(flag)return value+increment;return value;}
const pair=make();console.log(choose(pair)===7);console.log(pair[0][0]===5);`;

export const tupleSourcePairs = [
  { id: 'copy-choice', output: ['true', 'true'], javascript: copyJavaScript, rust: copyRust },
  { id: 'nested-copy', output: ['true', 'true'], javascript: nestedJavaScript,
    rust: 'fn make()->((f64,bool),f64){((5.0,true),2.0)} fn choose(pair:((f64,bool),f64))->f64{let value=pair.0.0;let flag=pair.0.1;let increment=pair.1;if flag{value+increment}else{value}} fn main(){let pair=make();println!("{}",choose(pair)==7.0);println!("{}",pair.0.0==5.0);}' },
  { id: 'triple-copy', output: ['true', 'true'],
    javascript: `/** @returns {[number,boolean,number]} */ function make(){return Object.freeze([5,true,2]);}
/** @param {[number,boolean,number]} pair @returns {number} */ function choose(pair){const value=pair[0];const flag=pair[1];const increment=pair[2];if(flag)return value+increment;return value;}
const pair=make();console.log(choose(pair)===7);console.log(pair[0]===5);`,
    rust: 'fn make()->(f64,bool,f64){(5.0,true,2.0)} fn choose(pair:(f64,bool,f64))->f64{let(value,flag,increment)=pair;if flag{value+increment}else{value}} fn main(){let pair=make();println!("{}",choose(pair)==7.0);println!("{}",pair.0==5.0);}' },
  { id: 'one-and-group', output: ['true', 'true'],
    javascript: `/** @returns {[number]} */ function make(){return Object.freeze([5]);}
/** @param {number} value @returns {number} */ function group(value){return(value);}
const pair=make();console.log(pair[0]===5);console.log(group(pair[0])===5);`,
    rust: 'fn make()->(f64,){(5.0,)} fn group(value:f64)->f64{(value)} fn main(){let pair=make();println!("{}",pair.0==5.0);println!("{}",group(pair.0)==5.0);}' },
  { id: 'unit-and-group', output: ['true'],
    javascript: '/** @param {number} value @returns {number} */ function group(value){return(value);} function unit(){return;} const unitValue=unit();console.log(group(5)===5);',
    rust: 'fn unit()->(){()} fn group(value:f64)->f64{(value)} fn main(){let ()=unit();println!("{}",group(5.0)==5.0);}' },
];

export const tupleSourceRefusals = [
  ...[
    ['mutable-product', copyJavaScript.replace('Object.freeze([5,true])', '[5,true]')],
    ['mutable-nested-product', nestedJavaScript.replace('Object.freeze([5,true])', '[5,true]')],
    ['mutable-nested-through-let', nestedJavaScript.replace('return Object.freeze([Object.freeze([5,true]),2]);', 'const inner=[5,true];return Object.freeze([inner,2]);')],
    ['mutable-nested-through-if', nestedJavaScript.replace('return Object.freeze([Object.freeze([5,true]),2]);', 'const inner=true?[5,true]:Object.freeze([5,true]);return Object.freeze([inner,2]);')],
    ['hole', copyJavaScript.replace('[5,true]', '[5,,true]')],
    ['spread', copyJavaScript.replace('Object.freeze([5,true])', 'Object.freeze([...[5,true]])')],
    ['missing-slot', copyJavaScript.replace('Object.freeze([5,true])', 'Object.freeze([5])')],
    ['excess-slot', copyJavaScript.replace('Object.freeze([5,true])', 'Object.freeze([5,true,9])')],
    ['dynamic-index', copyJavaScript + '\n/** @param {[number,boolean]} value @param {number} index */ function lookup(value,index){return value[index];}console.log(lookup(pair,0));'],
    ['identity-observer', copyJavaScript + '\nconsole.log(pair===pair);'],
    ['prototype-observer', copyJavaScript + '\nconsole.log(Object.getPrototypeOf(pair));'],
    ['descriptor-observer', copyJavaScript + "\nconsole.log(Object.getOwnPropertyDescriptor(pair,'0').writable);"],
    ['escaped-product', copyJavaScript + '\nglobalThis.savedProduct=pair;'],
    ['external-caller', copyJavaScript + '\nglobalThis.consumeProduct(pair);'],
    ['exported-product-entry', copyJavaScript.replace('function make()', 'export function make()')],
    ['zero-product-scope', '/** @returns {[]} */ function make(){return Object.freeze([]);}const pair=make();'],
  ].map(([id, source]) => ({ id: `javascript-${id}`, language: 'JavaScript', source })),
  ...[
    ['owned-string', 'fn make()->(String,bool){(String::from("x"),true)}fn main(){let pair=make();println!("{}",pair.1);}'],
    ['owned-vector', 'fn make()->(Vec<f64>,bool){(vec![5.0],true)}fn main(){let pair=make();println!("{}",pair.1);}'],
    ['owned-box', 'fn make()->(Box<f64>,bool){(Box::new(5.0),true)}fn main(){let pair=make();println!("{}",pair.1);}'],
    ['mutable-borrow', 'fn read(pair:(&mut f64,bool))->bool{pair.1}fn main(){let mut value=5.0;println!("{}",read((&mut value,true)));}'],
    ['shared-borrow', 'fn read(pair:(&f64,bool))->bool{pair.1}fn main(){let value=5.0;println!("{}",read((&value,true)));}'],
    ['tuple-struct', 'struct Pair(f64,bool);fn make()->Pair{Pair(5.0,true)}fn main(){let pair=make();println!("{}",pair.0==5.0);}'],
    ['mutable-product-binding', copyRust.replace('let pair=make();', 'let mut pair=make();pair.0=9.0;')],
  ].map(([id, source]) => ({ id: `rust-${id}`, language: 'Rust', source })),
];

export function runTupleJavaScript(code) {
  const output = [];
  vm.runInNewContext(code, { console: { log: (...values) => output.push(values.map(String).join(' ')) }, process: { argv: [] } }, { timeout: 1000 });
  return output;
}
export function applyTupleProgramEdits(network, edits) {
  for (const edit of edits) {
    const link = network.link(LinkId.fromU64(edit.link));
    assert.ok(link, 'a tuple edit references an existing link');
    if (edit.kind === 'definition') link.setMetadata(link.metadata().withDefinition(edit.value));
    else if (edit.kind === 'references') link.setReferences(edit.values.map(LinkId.fromU64));
    else throw new TypeError('unknown tuple link edit');
  }
}
export function tupleProjectionSyntaxWitness() {
  const source = '// \u{1F388}\nfn pick(pair:((f64,bool),bool))->bool{pair.0.1}fn main(){println!("{}",pick(((5.0,true),false)));}';
  const start = source.indexOf('pair.0.1');
  const outer = parseRust(source).items[0].body;
  assert.equal(outer.k, 'tupleIndex');
  assert.equal(outer.index, 1);
  assert.deepEqual(outer.span, { start, end: start + 8 });
  assert.equal(outer.object.k, 'tupleIndex');
  assert.equal(outer.object.index, 0);
  assert.deepEqual(outer.object.span, { start, end: start + 6 });
  return { source, start, inner: { start, end: start + 6 }, outer: { start, end: start + 8 } };
}
const definition = (link, value) => ({ kind: 'definition', link: link.asU64(), value });
const references = (link, values) => ({ kind: 'references', link: link.asU64(), values: values.map(id => id.asU64()) });
const label = name => TYPED_PROGRAM_LINKS_SCHEMA.fields[name] ?? name.replace(/[A-Z]/gu, letter => ` ${letter.toLowerCase()}`);
function inspect(case_) {
  const network = LinkNetwork.fromLino(case_.lino);
  const relations = network.fieldRelations();
  const scalar = id => network.link(id).metadata().definition;
  const field = (owner, label) => { const relation = relations.find(value => value.parent.equals(owner) && value.label === label); assert.ok(relation, label); return relation.child; };
  const records = kind => relations.filter(value => value.label === TYPED_PROGRAM_LINKS_SCHEMA.fields.k && scalar(value.child) === kind).map(value => value.parent);
  const replaceField = (owner, label, child) => { const relation = relations.find(value => value.parent.equals(owner) && value.label === label); assert.ok(relation, label); return references(relation.link.id(), [owner, relation.link.references()[1], child]); };
  return { network, root: LinkId.fromU64(case_.root), field, scalar, records, replaceField };
}

function graphCase(input, language) {
  const source = input[language === 'Rust' ? 'rust' : 'javascript'];
  const archive = createTypedProgramLinks(source, language);
  const network = LinkNetwork.fromLino(archive.network.toLino());
  const javascript = emitTypedProgramLinks(network, archive.root, 'JavaScript').code;
  const rust = emitTypedProgramLinks(network, archive.root, 'Rust').code;
  assert.deepEqual(runTupleJavaScript(input.javascript), input.output, 'independent original JavaScript observations');
  assert.deepEqual(runTupleJavaScript(javascript), input.output, 'source-free tuple target observations');
  decodeTypedProgramLinks(network, archive.root);
  assert.ok([...network.links()].every(link => !link.metadata().definition?.includes(source)), 'semantic links retain no original source envelope');
  return { id: `${language.toLowerCase()}-${input.id}`, program: input.id, language, source,
    oracleSource: input.javascript, rustOracle: input.rust, output: [...input.output], root: archive.root.asU64(), lino: network.toLino(), javascript, rust };
}

export function typedProgramTupleFixture() {
  const cases = tupleSourcePairs.flatMap(input => ['Rust', 'JavaScript'].map(language => graphCase(input, language)));
  const chosen = cases.find(case_ => case_.id === 'javascript-copy-choice');
  const graph = inspect(chosen);
  const tuple = graph.records('tuple')[0];
  const items = graph.field(tuple, label('items'));
  const slots = graph.network.link(items).references();
  assert.equal(slots.length, 2);
  const literal = graph.field(slots[0], label('value'));
  assert.equal(graph.scalar(literal), '5');
  const projection = graph.records('tupleIndex').find(value => graph.scalar(graph.field(value, label('index'))) === '0');
  assert.ok(projection, 'a scalar projection from the first tuple slot is present');
  const mutations = [{ id: 'linked-copy-number-edit', case: chosen.id, edits: [definition(literal, '9')], output: ['false', 'false'],
    oracleSource: chosen.oracleSource.replace('[5,true]', '[9,true]'), rustOracle: chosen.rustOracle.replace('(5.0,true)', '(9.0,true)') }];
  for (const mutation of mutations) {
    const edited = inspect(chosen);
    applyTupleProgramEdits(edited.network, mutation.edits);
    mutation.javascript = emitTypedProgramLinks(edited.network, edited.root, 'JavaScript').code;
    mutation.rust = emitTypedProgramLinks(edited.network, edited.root, 'Rust').code;
    assert.deepEqual(runTupleJavaScript(mutation.oracleSource), mutation.output);
    assert.deepEqual(runTupleJavaScript(mutation.javascript), mutation.output);
  }
  const invalid = [
    { id: 'linked-tuple-missing-slot', edits: [references(items, slots.slice(0, 1))] },
    { id: 'linked-tuple-excess-slot', edits: [references(items, [...slots, slots[0]])] },
    { id: 'linked-index-outside-layout', edits: [definition(graph.field(projection, label('index')), '2')] },
    { id: 'linked-negative-index', edits: [definition(graph.field(projection, label('index')), '-1')] },
    { id: 'linked-fractional-index', edits: [definition(graph.field(projection, label('index')), '0.5')] },
    { id: 'linked-index-result-type', edits: [definition(graph.field(projection, label('index')), '1')] },
    { id: 'linked-frozen-nonboolean', edits: [graph.replaceField(tuple, TYPED_PROGRAM_LINKS_SCHEMA.fields.frozen, literal)] },
    { id: 'linked-immutable-model-unfrozen', edits: [definition(graph.field(tuple, TYPED_PROGRAM_LINKS_SCHEMA.fields.frozen), 'false')] },
    { id: 'linked-unknown-product-model', edits: [definition(graph.field(tuple, label('productModel')), 'mutable value product')] },
  ].map(value => ({ ...value, case: chosen.id }));
  for (const mutation of invalid) {
    const edited = inspect(chosen);
    applyTupleProgramEdits(edited.network, mutation.edits);
    assert.throws(() => decodeTypedProgramLinks(edited.network, edited.root), undefined, mutation.id);
    for (const language of ['JavaScript', 'Rust']) assert.throws(() => emitTypedProgramLinks(edited.network, edited.root, language), undefined, mutation.id);
  }
  const refusals = tupleSourceRefusals.map(input => {
    assert.throws(() => createTypedProgramLinks(input.source, input.language), undefined, input.id);
    return { ...input };
  });
  return { schemaVersion: 1, generatedBy: 'js/scripts/generate-typed-program-tuple-fixtures.mjs', cases, mutations, invalid, refusals, syntaxWitness: tupleProjectionSyntaxWitness() };
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  if (process.argv.includes('--source-check')) {
    for (const input of tupleSourcePairs) assert.deepEqual(runTupleJavaScript(input.javascript), input.output, input.id);
    tupleProjectionSyntaxWitness();
    console.log(`independent tuple JavaScript source oracles: ${tupleSourcePairs.length} programs`);
  } else {
    const file = new URL('../../parity/fixtures/typed-program-tuples.lino', import.meta.url);
    const fixture = typedProgramTupleFixture();
    if (process.argv.includes('--check')) checkProjectData(file, fixture);
    else writeProjectData(file, fixture);
  }
}
