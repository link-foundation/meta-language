#!/usr/bin/env node
// Small source-free graphs with independently executed JavaScript behavior.
import vm from 'node:vm';
import { fileURLToPath } from 'node:url';
import { checkProjectData, writeProjectData } from './project-data.mjs';
import { createTypedProgramLinks, decodeTypedProgramLinks, encodeTypedProgramLinks, emitTypedProgramLinks } from '../src/typed-program-links.js';

const inputs = [
  ['javascript-add', 'JavaScript', 'function add(a,b){return a+b;} console.log(add(2,3));', ['5']],
  ['rust-add', 'Rust', 'fn add(a:i32,b:i32)->i32{a+b} fn main(){println!("{}",add(2,3));}', ['5']],
  ['javascript-two-functions', 'JavaScript', 'function add(a,b){return a+b;} function multiply(a,b){return a*b;} console.log(add(2,3));', ['5']],
  ['javascript-lexical-let', 'JavaScript', 'function calculate(value){const doubled=value*2;return doubled+1;} console.log(calculate(3));', ['7']],
  ['javascript-string-map', 'JavaScript', '/** @param {string} value */ function trim(value){return value.trim();}console.log(trim(" hi "));', ['hi']],
  ['javascript-string-test', 'JavaScript', '/** @param {string} value */ function starts(value){return value.startsWith("a");}console.log(starts("abc"));', ['true']],
  ['javascript-array-length', 'JavaScript', 'function values(){return [1,2];}console.log(values().length);', ['2']],
  ['javascript-array-append', 'JavaScript', 'function values(){return [1,...[2,3]];}console.log(values().length);', ['3']],
  ['javascript-string-length', 'JavaScript', '/** @param {string} value */ function length(value){return value.length;}console.log(length("😀"));', ['2']],
  ['javascript-math', 'JavaScript', 'function root(value){return Math.sqrt(value);}console.log(root(9));', ['3']],
  ['javascript-math-spread', 'JavaScript', 'function maximum(){return Math.max(...[2,10]);}console.log(maximum());', ['10']],
  ['javascript-function-print', 'JavaScript', 'function f(value){console.log(value);return value+1;}console.log(f(2));', ['2', '3']],
  ['rust-exact-cast', 'Rust', 'fn widen(value:i32)->i64{value as i64}fn main(){println!("{}",widen(2));}', ['2']],
  ['javascript-abort-branch', 'JavaScript', 'function f(value){if(value<0){throw new Error("negative");}return value;}console.log(f(2));', ['2']],
  ['javascript-operation-string-data', 'JavaScript', 'console.log("sqrt((globalThis.injectionObserved=true,9))||Math.sqrt");', ['sqrt((globalThis.injectionObserved=true,9))||Math.sqrt']],
];

function execute(code) {
  const output = [];
  vm.runInNewContext(code, { console: { log: (...values) => output.push(values.map(String).join(' ')) }, process: { argv: [] } }, { timeout: 1000 });
  return output;
}

function sharedDagFixture() {
  const { network, root } = createTypedProgramLinks(inputs[0][2], inputs[0][1]);
  const relations = network.fieldRelations();
  const bodyRelation = relations.find((relation) => relation.label === 'body'
    && relations.some((kind) => kind.parent.equals(relation.child) && kind.label === 'construct kind'
      && network.link(kind.child).metadata().definition === 'binary'));
  const fields = relations.filter((relation) => relation.parent.equals(bodyRelation.child));
  let child = bodyRelation.child;
  // Only seventeen additional records are allocated, but their ordered walk
  // expands into more than 100000 expression paths before target emission.
  for (let index = 0; index < 17; index += 1) {
    const parent = network.insertLink([], network.link(bodyRelation.child).metadata());
    for (const field of fields) network.insertFieldRelation(parent, field.label,
      ['left', 'right'].includes(field.label) ? child : field.child);
    child = parent;
  }
  const relation = [...network.links()].find((link) => {
    const refs = link.references();
    return refs.length === 3 && refs[0].equals(bodyRelation.parent) && refs[2].equals(bodyRelation.child)
      && network.link(refs[1]).metadata().term === 'body';
  });
  relation.setReferences([relation.references()[0], relation.references()[1], child]);
  return { id: 'shared-dag-traversal', root: root.asU64(), lino: network.toLino(), rejection: 'resource limit' };
}

function derivedIndexFixture() {
  const archive = createTypedProgramLinks(inputs[1][2], inputs[1][1]);
  const program = decodeTypedProgramLinks(archive.network, archive.root);
  const function_ = program.declarations.get('add');
  const element = function_.ret;
  function_.body = { k: 'index', array: { k: 'array', items: function_.params.map((parameter) => ({ k: 'var', name: parameter.name, type: parameter.type })), type: { kind: 'array', element } },
    index: { k: 'lit', value: '0', type: { kind: 'float' } }, type: element };
  const edited = encodeTypedProgramLinks(program);
  const javascript = emitTypedProgramLinks(edited.network, edited.root, 'JavaScript').code;
  const rust = emitTypedProgramLinks(edited.network, edited.root, 'Rust').code;
  const output = ['2'];
  if (JSON.stringify(execute(javascript)) !== JSON.stringify(output)) throw new Error('derived native array index changed its checked behavior');
  return { id: 'native-array-index', derivedFrom: 'rust-add', root: edited.root.asU64(), lino: edited.network.toLino(), javascript, rust, output };
}

function operationMutationFixtures() {
  const inputs = [
    ['math', 'math', 'sqrt', 'function root(value){return Math.sqrt(value);}console.log(root(9));', ['3'], ['JavaScript', 'Rust']],
    ['string-map', 'stringMap', 'trim', '/** @param {string} value */ function trim(value){return value.trim();}console.log(trim(" hi "));', ['hi'], ['JavaScript', 'Rust']],
    ['string-test', 'stringTest', 'startsWith', '/** @param {string} value */ function starts(value){return value.startsWith("a");}console.log(starts("abc"));', ['true'], ['JavaScript', 'Rust']],
    ['method', 'method', 'split', 'console.log("a,b".split(",").length);', ['2'], ['JavaScript']],
    ['collection-read', 'collectionRead', 'has', 'function lookup(){return new Map([["key",3]]).has("key");}console.log(lookup());', ['true'], ['JavaScript']],
    ['collection-items', 'collectionItems', 'values', 'console.log(Array.from(new Set([2,3]).values()).length);', ['2'], ['JavaScript']],
    ['optional-predicate', 'optionalPredicate', 'isNaN', '/** @param {number|null} value */ function check(value){return Number.isNaN(value);}console.log(check(null));', ['false'], ['JavaScript']],
  ];
  return inputs.map(([id, kind, operation, source, output, runtimes]) => {
    const archive = createTypedProgramLinks(source, 'JavaScript');
    if (JSON.stringify(execute(emitTypedProgramLinks(archive.network, archive.root, 'JavaScript').code)) !== JSON.stringify(output)) throw new Error(`operation fixture ${id} changed behavior`);
    return { id, kind, operation, replacement: 'sqrt((globalThis.injectionObserved=true,9))||Math.sqrt', runtimes,
      root: archive.root.asU64(), lino: archive.network.toLino() };
  });
}

function roundingMutationFixture() {
  const archive = createTypedProgramLinks('function divide(a,b){return a/b;}console.log(divide(8n,2n));', 'JavaScript');
  const output = execute(emitTypedProgramLinks(archive.network, archive.root, 'JavaScript').code);
  if (JSON.stringify(output) !== JSON.stringify(['4n'])) throw new Error('division fixture changed behavior');
  return { id: 'division-rounding-injection', root: archive.root.asU64(), lino: archive.network.toLino(),
    replacement: 'trunc\";globalThis.injectionObserved=true;//' };
}

function constructorImmutabilityFixtures() {
  const declaration = "/** @typedef {{ $:'value', value:number } | { $:'none' }} Choice */";
  const mutable = createTypedProgramLinks(`${declaration}function make(){return {$:'value',value:1};}`, 'JavaScript');
  const frozen = createTypedProgramLinks(`${declaration}function make(){return Object.freeze({$:'value',value:1});}`, 'JavaScript');
  const editedProgram = decodeTypedProgramLinks(frozen.network, frozen.root);
  editedProgram.declarations.get('make').body.frozen = false;
  const edited = encodeTypedProgramLinks(editedProgram);
  const algebraicProgram = decodeTypedProgramLinks(edited.network, edited.root);
  algebraicProgram.sourceLanguage = 'Rust';
  const algebraic = encodeTypedProgramLinks(algebraicProgram);
  return [['mutable-absent', mutable, false], ['frozen-explicit', frozen, true],
    ['mutable-edited-false', edited, false], ['algebraic-origin-default', algebraic, true]].map(([id, archive, isFrozen]) => {
    const javascript = emitTypedProgramLinks(archive.network, archive.root, 'JavaScript').code;
    const descriptors = JSON.parse(vm.runInNewContext(`${javascript}\n(() => {
      const value = make(); const property = Object.getOwnPropertyDescriptor(value, 'value');
      return JSON.stringify({ frozen: Object.isFrozen(value), extensible: Object.isExtensible(value),
        writable: property.writable, configurable: property.configurable, enumerable: property.enumerable,
        assigned: Reflect.set(value, 'value', 2), value: value.value });
    })()`, {}, { timeout: 1000 }));
    const expected = { frozen: isFrozen, extensible: !isFrozen, writable: !isFrozen, configurable: !isFrozen,
      enumerable: true, assigned: !isFrozen, value: isFrozen ? 1 : 2 };
    if (JSON.stringify(descriptors) !== JSON.stringify(expected)) throw new Error(`constructor immutability fixture ${id} changed descriptors`);
    return { id, root: archive.root.asU64(), lino: archive.network.toLino(), javascript, descriptors };
  });
}

export function typedProgramLinksFixture() {
  return { schemaVersion: 1, generatedBy: 'js/scripts/generate-typed-program-links-fixtures.mjs',
    cases: inputs.map(([id, language, source, output]) => {
      const archive = createTypedProgramLinks(source, language);
      const javascript = emitTypedProgramLinks(archive.network, archive.root, 'JavaScript').code;
      const rust = emitTypedProgramLinks(archive.network, archive.root, 'Rust').code;
      if (JSON.stringify(execute(javascript)) !== JSON.stringify(output)) throw new Error(`${id} emitted the wrong observable result`);
      return { id, language, source, root: archive.root.asU64(), lino: archive.network.toLino(), javascript, rust, output };
    }),
    derivedCases: [derivedIndexFixture()],
    operationMutations: operationMutationFixtures(),
    roundingMutations: [roundingMutationFixture()],
    constructorImmutabilityCases: constructorImmutabilityFixtures(),
    resourceCases: [sharedDagFixture()],
    invalidMachineIntegerWidths: [0, 7, 129, 130],
    invalidFloatingPointSpellings: ['nan', 'infinity', '-infinity', 'nAn', 'iNfInItY'],
    validFloatingPointSpellings: [
      { value: 'NaN', output: ['NaN'] }, { value: 'Infinity', output: ['Infinity'] },
      { value: '-Infinity', output: ['-Infinity'] }, { value: '1e3', output: ['1003'] },
      { value: '1E3', output: ['1003'] }, { value: '-0', output: ['3'] },
    ],
    mutations: [
      { id: 'arithmetic-operation', fixture: 'javascript-add', field: 'operation', from: 'add', to: 'mul', output: ['6'] },
      { id: 'binding-rename', fixture: 'javascript-add', relation: 'binding reference', targetField: 'name', to: 'renamed', output: ['5'] },
      { id: 'declaration-retarget', fixture: 'javascript-two-functions', relation: 'declaration reference', targetDeclaration: 'multiply', output: ['6'] },
      { id: 'literal-injection', fixture: 'javascript-add', field: 'value', from: '2', to: '0); injected(); (0', rejection: 'floating-point literal' },
    ] };
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  const path = new URL('../../parity/fixtures/typed-program-links.json', import.meta.url);
  const fixture = typedProgramLinksFixture();
  if (process.argv.includes('--check')) checkProjectData(path, fixture);
  else writeProjectData(path, fixture);
}
