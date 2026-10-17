import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import test from 'node:test';
import vm from 'node:vm';

import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';
import { encodeTypedProgramLinks, decodeTypedProgramLinks, emitTypedProgramLinks } from '../src/typed-program-links.js';
import { translateProgram, readTranslationProvenance } from '../src/program-translation.js';
import { recheckLinkedTypedProgram } from '../src/typed-program-checked-projection.js';
import { acceptOwnedArrayAppendUses, acceptOwnedArrayFinalUse } from '../src/translation/owned-array-append-rules.js';
import { readOwnedArrayFinalUses } from '../src/translation/owned-array-moves.js';

const check = source => checkProgram(parseJavaScript(source, { referenceSemantics: false }));
const run = (program, expression) => vm.runInNewContext(emitJavaScript(program).text + '\n' + expression, {}, { timeout: 1000 });

test('fresh local pushes become pure checked array values and survive source-free Links recheck', () => {
  const source = '/** @param {number} count @returns {number[]} */ function build(count) { const values=[]; for(let index=0;index<count;index+=1) { values.push(index); } return values; }';
  const program = check(source);
  assert.equal(run(program, 'build(8).join(",");'), '0,1,2,3,4,5,6,7');
  const automatic = checkProgram(parseJavaScript(source));
  assert.equal(run(automatic, 'build(8).join(",");'), '0,1,2,3,4,5,6,7');
  assert.ok(emitRust(automatic).text.includes('crate::ml_array::append('));
  const archive = encodeTypedProgramLinks(program);
  const replay = decodeTypedProgramLinks(archive.network, archive.root);
  recheckLinkedTypedProgram(replay, { wholeProgram: true });
  assert.equal(run(replay, 'build(8).join(",");'), '0,1,2,3,4,5,6,7');
  assert.ok(emitRust(program).text.includes('crate::ml_array::append('));
  const variadic = check(source.replace('values.push(index);', 'values.push(index,index+1);'));
  assert.equal(run(variadic, 'build(3).join(",");'), '0,1,1,2,2,3');
});

test('construction refuses preexisting aliases, unknown calls, captures and loop-carried escapes', () => {
  const cases = [
    'const alias=values; values.push(1); return alias;',
    'observe(values); values.push(1); return values;',
    'const capture=()=>values; values.push(1); return capture();',
    'let index=0; while(index<2) { values.push(index); const alias=values; index+=1; } return values;',
    'values.push(values); return values;',
  ];
  for (const body of cases) {
    const source = '/** @param {number[]} input */ function observe(input) { return input.length; }\n/** @returns {number[]} */ function build() { const values=[]; ' + body + ' }';
    assert.throws(() => check(source), /method call \.push\(\)|recursive|expected|type|array/u, body);
  }
});

test('a reader is admitted from its actual scalar declaration and shadowed readers remain unsafe', () => {
  const source = '/** @param {number[]} values @param {number} index @returns {number} */ function read(values,index) { return values[index] ?? -1; }\n/** @returns {number[]} */ function build() { const pending=[2]; let index=0; while(index<3) { const next=read(pending,index); pending.push(next+1); index+=1; } return pending; }';
  assert.equal(run(check(source), 'build().join(",");'), '2,3,4,5');
  const shadow = source.replace('const pending=[2];', 'const pending=[2]; const read=(values,index)=>values.length;');
  assert.throws(() => check(shadow), /method call \.push\(\)/u);
  const escapes = source.replace('return values[index] ?? -1;', 'const alias=values; return alias.length;');
  assert.throws(() => check(escapes), /method call \.push\(\)/u);
  const namespace = "const Other={ /** @param {number[]} values @param {number} index @returns {number} */ read(values,index) { const alias=values; return alias.length; } };\n";
  const boundary = source.indexOf('/** @returns {number[]} */');
  const qualified = source.slice(0,boundary) + namespace + source.slice(boundary).replace('read(pending,index)', 'Other.read(pending,index)');
  assert.throws(() => check(qualified), /method call \.push\(\)/u);
  const preserved = source.slice(0,boundary) + namespace + source.slice(boundary);
  assert.equal(run(check(preserved), 'build().join(",");'), '2,3,4,5');
  const earlyNamespace = "const Other={ /** @param {number[]} values @param {number} index @returns {number} */ read(values,index) { return values[index] ?? -1; } };\n";
  const beforeActualDeclaration = earlyNamespace + source.slice(boundary) + '\n' + escapes.slice(0,boundary);
  assert.throws(() => check(beforeActualDeclaration), { message: 'function value ali: functions and namespaces are only portable when a function is called at 448..451' });
  const completeDeclaration = earlyNamespace + source.slice(boundary) + '\n' + escapes.slice(0,escapes.indexOf('/** @returns {number[]} */'));
  assert.throws(() => check(completeDeclaration), /method call \.push\(\)/u);
  const loopingReader = source.replace('return values[index] ?? -1;', 'let cursor=0; while(cursor<values.length) { if(cursor===index) return values[cursor] ?? -1; cursor+=1; } return -1;');
  assert.equal(run(check(loopingReader), 'build().join(",");'), '2,3,4,5');
});

test('the exact loop allocation relation distinguishes a final handoff from a carried alias', () => {
  assert.equal(acceptOwnedArrayAppendUses([], [], [], [0], [0], [2], 1), true);
  assert.equal(acceptOwnedArrayAppendUses([], [], [], [0], [0], [0], 1), false);
  assert.equal(acceptOwnedArrayAppendUses([], [7], [7], [0], [1], [2], 1), false);
  assert.equal(acceptOwnedArrayAppendUses([7], [7], [7], [0], [1], [2], 1), true);
  assert.throws(() => acceptOwnedArrayAppendUses([], [7], [], [0], [1], [2], 1), RangeError);
  const source = '/** @returns {number} */ function build() { let count=0; for(let index=0;index<3;index+=1) { const values=[]; values.push(index); const finished=values; count+=finished.length; } return count; }';
  assert.equal(run(check(source), 'build();'), 3);
});

test('final consumption follows actual continuation edges, including branches, joins and cycles', () => {
  // Binding 4 at event 0 has no later read in one alternative; a read of
  // binding 4 at event 1 must not be confused with that disjoint path.
  assert.equal(acceptOwnedArrayFinalUse(0, [4, 4, 8], [0, 1, 2], [1, 1, 0], [2, 2]), true);
  assert.equal(acceptOwnedArrayFinalUse(0, [4, 4, 8], [0, 1, 2], [1, 1, 0], [1, 2]), false);
  assert.equal(acceptOwnedArrayFinalUse(0, [4, 8], [0, 1], [1, 1], [1, 1]), true);
  const type = { kind: 'array', element: { kind: 'float' } };
  const variable = name => ({ k: 'var', name, type });
  const left = variable('original');
  const next = variable('updated');
  const append = { k: 'append', left, right: { k: 'array', items: [{ k: 'length', array: variable('original'), type: { kind: 'float' } }], type }, type };
  const body = { k: 'let', name: 'updated', value: append, body: { k: 'recur', args: [next], type } , type };
  const final = readOwnedArrayFinalUses(body);
  assert.ok(final.has(left));
  assert.ok(final.has(next));
  const aliased = { ...body, body: { k: 'tuple', items: [variable('original'), next], type: { kind: 'tuple', elements: [type, type] } } };
  assert.equal(readOwnedArrayFinalUses(aliased).has(left), false);
});

test('explicit array consumption preserves the historical copy profile and source-free alias values', () => {
  const source = '/** @returns {number} */function preserve(){const original=[1];const alias=original;const updated=[...original,2];return alias.length*10+updated.length;}';
  const program = check(source);
  const copied = emitRust(program);
  const consumed = emitRust(program, { arrayConsumption: true });
  assert.ok(copied.text.includes('crate::ml_array::append(original.clone(),'));
  assert.equal(copied.text.includes('let (ml_append_right, ml_append_left)'), false);
  assert.ok(consumed.text.includes('let (ml_append_right, ml_append_left)'));
  assert.ok(consumed.text.includes('let alias = original.clone();'));
  assert.deepEqual(consumed.assumptions, copied.assumptions);
  assert.deepEqual(consumed.encodings, copied.encodings);
  assert.equal(run(program, 'preserve();'), 12);
  const archive = encodeTypedProgramLinks(program);
  const replay = decodeTypedProgramLinks(archive.network, archive.root);
  recheckLinkedTypedProgram(replay, { wholeProgram: true });
  assert.equal(emitRust(replay).text, copied.text);
  assert.equal(emitRust(replay, { arrayConsumption: true }).text, consumed.text);
  assert.equal(run(replay, 'preserve();'), 12);
  assert.equal(vm.runInNewContext(source + '\npreserve();', {}, { timeout: 1000 }), 12);
  const publicTranslation = translateProgram(source, 'JavaScript', 'Rust');
  assert.equal(publicTranslation.diagnostic, null);
  readTranslationProvenance(publicTranslation.code, 'Rust');
  assert.equal(publicTranslation.code.slice(publicTranslation.code.indexOf('\n') + 1), consumed.text);
  assert.equal(emitTypedProgramLinks(archive.network, archive.root, 'Rust').code, consumed.text);
});

test('reserved append locals retain the exact refusal while legal source names exercise real Rust temporary collisions', () => {
  const original = '/** @param {number} count @returns {number[]} */ function build(count) { const values=[]; for(let index=0;index<count;index+=1) { if(index%2===0) values.push(index,index+1); } return values; }\n/** @returns {number} */ function hygiene(){const ml_append_right=[1];const ml_append_left=2;ml_append_right.push(ml_append_left);return ml_append_right.length;}\n/** @returns {number} */function preserve(){const original=[1];const alias=original;const updated=[...original,2];return alias.length*10+updated.length;}\n/** @returns {number} */function readAfter(){const original=[1];const updated=[...original,2];return original.length*10+updated.length;}\nconst values=build(100001);console.log(values.length,values[0]??-1,values[100001]??-1);console.log(hygiene(),preserve(),readAfter());';
  assert.equal(createHash('sha256').update(original).digest('hex'), '83e09c13448b22e6d48879c61bb462dfe2b9915081f302f91ff063f6588ad17b', 'the complete original native fixture remains an unchanged refusal input');
  assert.throws(() => parseJavaScript(original), error => {
    assert.deepEqual({kind:error.kind,reason:error.reason,span:error.span}, {
      kind:'unsupported',reason:"reserved identifier: ml_append_right uses the translator's reserved ml_ prefix",span:{start:243,end:258},
    });
    return true;
  });
  // These admitted source spellings become the actual append temporary names
  // under the existing Rust snake-case binder transform, preserving collision coverage.
  const source = original.replaceAll('ml_append_right', 'mlAppendRight').replaceAll('ml_append_left', 'mlAppendLeft');
  const observe = code => {
    const lines = [];
    vm.runInNewContext(code, {console:{log:(...values) => lines.push(values.map(String).join(' '))}}, {timeout:1000});
    return lines.join('\n') + '\n';
  };
  assert.equal(observe(original), '100002 0 100001\n2 12 12\n', 'the independent original source retains every large-loop and alias observation');
  const program = checkProgram(parseJavaScript(source));
  recheckLinkedTypedProgram(program, {wholeProgram:true});
  const emitted = emitRust(program, {arrayConsumption:true});
  assert.match(emitted.text, /let \(ml_append_right, ml_append_left\) = \(vec!\[[^\]]+\], [A-Za-z_][A-Za-z_0-9]*\); crate::ml_array::append/u);
  const hygiene = emitted.definitions.find(text => text.startsWith('pub fn hygiene('));
  assert.ok(hygiene);
  assert.ok(hygiene.includes('let ml_append_right ='), 'the legal source array name really collides with the append temporary spelling');
  assert.ok(hygiene.includes('let ml_append_left ='), 'the legal source argument name really collides with the other append temporary spelling');
  assert.ok(hygiene.includes('let (ml_append_right, ml_append_left)'), 'the simultaneous initializer retains both actual collision sites');
  const readAfter = emitted.definitions.find(text => text.startsWith('pub fn read_after('));
  assert.ok(readAfter.includes('original.clone()'));
  assert.equal(readAfter.includes('let (ml_append_right, ml_append_left)'), false);
  const javascript = emitJavaScript(program);
  // The native CI test executes all 100001 iterations. Its immutable JavaScript
  // append lowering is checked here with bounded inputs to avoid quadratic work.
  const declarations = [...javascript.preludes, ...javascript.definitions].join('\n\n');
  assert.equal(observe(declarations + '\nconst values=build(8);console.log(values.length,values[0]??-1,values[7]??-1);console.log(hygiene(),preserve(),readAfter());'), '8 0 7\n2 12 12\n');
  assert.ok(javascript.text.includes('100001'), 'the complete emitted program retains the original large-loop input');
});

test('the complete five-decision source translates together with its actual required-column closure', () => {
  const required = readFileSync(new URL('../src/translation/fact-column-rules.js', import.meta.url), 'utf8');
  const source = readFileSync(new URL('../src/translation/owned-array-append-rules.js', import.meta.url), 'utf8');
  const closure = required + '\n' + source.replace("import { readRequiredNumberFact } from './fact-column-rules.js';", '');
  const program = check(closure);
  const emitted = emitRust(program, { arrayConsumption: true });
  assert.equal(emitted.definitions.length, program.declarations.size);
  for (const name of ['acceptOwnedArrayAppendShape', 'acceptOwnedArrayRead', 'acceptOwnedArrayReaderResult',
    'acceptOwnedArrayAppendUses', 'acceptOwnedArrayFinalUse', 'readRequiredNumberFact', 'readRequiredTextFact', 'readRequiredBooleanFact']) {
    assert.ok(program.declarations.has(name), name);
  }
  assert.equal(run(program, 'acceptOwnedArrayAppendUses([], [7], [7], [0], [1], [2], 1);'), false);
});
