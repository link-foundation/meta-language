#!/usr/bin/env node
// Small source-free graphs with independently executed JavaScript behavior.
import { readFileSync, writeFileSync } from 'node:fs';
import vm from 'node:vm';
import { fileURLToPath } from 'node:url';
import { createTypedProgramLinks, emitTypedProgramLinks } from '../src/typed-program-links.js';

const inputs = [
  ['javascript-add', 'JavaScript', 'function add(a,b){return a+b;} console.log(add(2,3));', ['5']],
  ['rust-add', 'Rust', 'fn add(a:i32,b:i32)->i32{a+b} fn main(){println!("{}",add(2,3));}', ['5']],
  ['javascript-two-functions', 'JavaScript', 'function add(a,b){return a+b;} function multiply(a,b){return a*b;} console.log(add(2,3));', ['5']],
  ['javascript-lexical-let', 'JavaScript', 'function calculate(value){const doubled=value*2;return doubled+1;} console.log(calculate(3));', ['7']],
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

export function typedProgramLinksFixture() {
  return { schemaVersion: 1, generatedBy: 'js/scripts/generate-typed-program-links-fixtures.mjs',
    cases: inputs.map(([id, language, source, output]) => {
      const archive = createTypedProgramLinks(source, language);
      const javascript = emitTypedProgramLinks(archive.network, archive.root, 'JavaScript').code;
      const rust = emitTypedProgramLinks(archive.network, archive.root, 'Rust').code;
      if (JSON.stringify(execute(javascript)) !== JSON.stringify(output)) throw new Error(`${id} emitted the wrong observable result`);
      return { id, language, source, root: archive.root.asU64(), lino: archive.network.toLino(), javascript, rust, output };
    }),
    resourceCases: [sharedDagFixture()],
    invalidMachineIntegerWidths: [0, 7, 129, 130],
    mutations: [
      { id: 'arithmetic-operation', fixture: 'javascript-add', field: 'operation', from: 'add', to: 'mul', output: ['6'] },
      { id: 'binding-rename', fixture: 'javascript-add', relation: 'binding reference', targetField: 'name', to: 'renamed', output: ['5'] },
      { id: 'declaration-retarget', fixture: 'javascript-two-functions', relation: 'declaration reference', targetDeclaration: 'multiply', output: ['6'] },
      { id: 'literal-injection', fixture: 'javascript-add', field: 'value', from: '2', to: '0); injected(); (0', rejection: 'floating-point literal' },
    ] };
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  const path = new URL('../../parity/fixtures/typed-program-links.json', import.meta.url);
  const text = JSON.stringify(typedProgramLinksFixture(), null, 2) + '\n';
  if (process.argv.includes('--check')) {
    if (readFileSync(path, 'utf8') !== text) throw new Error('typed program link fixtures are stale');
  } else writeFileSync(path, text);
}
