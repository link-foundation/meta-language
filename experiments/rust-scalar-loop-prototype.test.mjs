// Isolated JavaScript-first prototype. No native execution or full-module claim.
import assert from 'node:assert/strict';
import test from 'node:test';
import vm from 'node:vm';
import {readFileSync} from 'node:fs';
import {readProjectData} from '../js/scripts/project-data.mjs';
import {rustFunctionDefinitions} from '../js/scripts/generate-self-translation-report.mjs';
import {parseRust} from '../js/src/translation/rust.js';
import {parseJavaScript} from '../js/src/translation/javascript.js';
import {checkProgram} from '../js/src/translation/check.js';
import {emitRust} from '../js/src/translation/emit-rust.js';
import {emitJavaScript} from '../js/src/translation/emit-javascript.js';
import {LinkNetwork} from '../js/src/network.js';
import {createTypedProgramLinks, decodeTypedProgramLinks, emitTypedProgramLinks, encodeTypedProgramLinks} from '../js/src/typed-program-links.js';

const model = 'closed primitive scalar loop 1';
const options = {scalarLoopModel: model};
const linkedOptions = {rustScalarLoopModel: model};
const suffix = '\nfn main(){}';
const moduleSource = readFileSync(new URL('../rust/src/link_network_binary_rules/part_1.rs', import.meta.url), 'utf8');
const originalName = 'ml_link_network_binary_integer_byte_loop2';
const authentic = rustFunctionDefinitions(moduleSource).get(originalName);
assert.equal(Buffer.byteLength(authentic), 512);
const cases = [
  {name: 'advance', source: 'fn advance(mut value:f64,limit:f64,mut at:f64)->f64{loop{return if at<limit{(value,at)=(value+1f64,at+1f64);continue;}else{value};}}',
    original: 'function advance(value,limit,at){for(;;){if(at<limit){[value,at]=[value+1,at+1];continue;}return value;}}', observations: [[[3,5,0],8], [[0,20000,0],20000]]},
  {name: 'swap', source: 'fn swap(mut a:f64,mut b:f64,limit:f64,mut at:f64)->f64{loop{return if at<limit{(a,b,at)=(b,a,at+1f64);continue;}else{a};}}',
    original: 'function swap(a,b,limit,at){for(;;){if(at<limit){[a,b,at]=[b,a,at+1];continue;}return a;}}', observations: [[[0,-0,1,0],-0], [[0,-0,2,0],0], [[1,2,1,0],2], [[1,2,2,0],1]]},
  {name: 'toggle', source: 'fn toggle(mut flag:bool,limit:f64,mut at:f64)->bool{loop{return if at<limit{(flag,at)=(!flag,at+1f64);continue;}else{flag};}}',
    original: 'function toggle(flag,limit,at){for(;;){if(at<limit){[flag,at]=[!flag,at+1];continue;}return flag;}}', observations: [[[true,1,0],false], [[true,2,0],true]]},
  {name: originalName, source: authentic,
    original: `function ${originalName}(value,index,at){for(;;){if(at<index){const value_2=Math.floor(value/256);const at_2=at+1;[value,at]=[value_2,at_2];continue;}return value;}}`,
    observations: [[[65536,1,0],256], [[65536,2,0],1], [[-65537,1,0],-257], [[NaN,1,0],NaN], [[Infinity,1,0],Infinity], [[-Infinity,1,0],-Infinity], [[-0,0,0],-0], [[-0,1,0],-0], [[5e-324,1,0],0], [[-5e-324,1,0],-0], [[257,NaN,0],257]]},
];
function observe(code, entry) {
  const context = {};
  vm.runInNewContext(code, context, {timeout: 1000});
  for (const [arguments_, expected] of entry.observations) assert.ok(Object.is(context[entry.name](...arguments_), expected), `${entry.name}(${arguments_}): exact scalar value including NaN and signed zero`);
}
test('ordinary scalar state lowers iteratively before exact authentic generated-name replay', () => {
  assert.ok(moduleSource.includes(authentic));
  for (const entry of cases) {
    observe(entry.original, entry);
    const surface = parseRust(entry.source + suffix, options);
    assert.equal(surface.items[0].name, entry.name);
    assert.equal(surface.items[0].generated, undefined);
    const program = checkProgram(surface);
    const javascript = emitJavaScript(program).text;
    assert.match(javascript, /for \(;;\)/u);
    observe(javascript, entry);
    const definitions = emitRust(program).definitions.join('\n\n');
    const replay = checkProgram(parseRust(definitions + suffix, options));
    assert.equal(replay.items[0].name, entry.name);
    observe(emitJavaScript(replay).text, entry);
    const actualJavaScriptDefinitions = emitJavaScript(program).definitions.join('\n\n');
    assert.throws(() => checkProgram(parseJavaScript(actualJavaScriptDefinitions)), /reserved identifier/u);
    const reverse = checkProgram(parseJavaScript(actualJavaScriptDefinitions, options));
    assert.equal(reverse.items[0].name, entry.name);
    assert.equal(reverse.items[0].sourceSignature, undefined);
    assert.equal(reverse.items[0].sourceLoop.interface.model, 'JavaScript Number and Boolean scalar loop values 1');
    observe(emitJavaScript(reverse).text, entry);
    const reverseRust = emitRust(reverse).definitions.join('\n\n');
    observe(emitJavaScript(checkProgram(parseRust(reverseRust + suffix, options))).text, entry);
  }
});
test('owned header, state, write order and profile survive checked source-free Links reload', () => {
  for (const entry of cases) {
    const imported = createTypedProgramLinks(entry.source + suffix, 'Rust', {...linkedOptions, provenance: {caller: 'prototype'}});
    const network = LinkNetwork.fromLino(imported.network.toLino());
    assert.ok(network.links().every(link => !link.metadata().definition?.includes(entry.source)));
    const declaration = decodeTypedProgramLinks(network, imported.root).declarations.get(entry.name);
    assert.equal(declaration.sourceLoop.sourceName, entry.name);
    assert.equal(declaration.sourceSignature.model, 'Rust source function signature 1');
    observe(emitTypedProgramLinks(network, imported.root, 'JavaScript').code, entry);
    assert.ok(emitTypedProgramLinks(network, imported.root, 'Rust').code.includes(`pub fn ${entry.name}(`));
    assert.ok(network.links().some(link => link.metadata().definition === model));
    assert.ok(network.fieldRelations().some(row => row.label === 'source loop proof'));
    const actualJavaScript = emitJavaScript(decodeTypedProgramLinks(network, imported.root)).definitions.join('\n\n');
    const reverse = createTypedProgramLinks(actualJavaScript, 'JavaScript', {javaScriptScalarLoopModel: model, provenance: {caller: 'reverse prototype'}});
    const reverseNetwork = LinkNetwork.fromLino(reverse.network.toLino());
    assert.equal(decodeTypedProgramLinks(reverseNetwork, reverse.root).declarations.get(entry.name).sourceLoop.language, 'JavaScript');
    observe(emitTypedProgramLinks(reverseNetwork, reverse.root, 'JavaScript').code, entry);
    assert.ok(emitTypedProgramLinks(reverseNetwork, reverse.root, 'Rust').code.includes(`pub fn ${entry.name}(`));
  }
});
test('JavaScript reverse refuses unproved snapshots, owners, state order and interface meanings', () => {
  const actual = emitJavaScript(checkProgram(parseRust(cases[1].source + suffix, options))).definitions.join('\n\n');
  const invalid = [
    actual.replaceAll('ml_scalar_write0', 'ml_scalar_write00'),
    actual.replace('const ml_scalar_write0 = b;', 'const ml_unrelated = b;'),
    actual.replace('const ml_scalar_write0 = b;', 'const ml_scalar_write9 = b; const ml_scalar_write0 = b;'),
    actual.replace('const ml_scalar_write1 = a;', 'const ml_scalar_write1 = ml_scalar_write0;'),
    actual.replace('[a, b, limit, at] =', '[b, a, limit, at] ='),
    actual.replace('const ml_scalar_write0 = b;', 'const ml_scalar_write0 = [b];'),
    actual.replace('const ml_scalar_write0 = b;', 'const ml_scalar_write0 = foreign(b);'),
    actual.replace('const ml_scalar_write0 = b;', 'console.log(b); const ml_scalar_write0 = b;'),
    actual.replace('for (;;)', 'for (;at<limit;)'),
    actual.replace('continue;', 'break;'),
    actual.replace('function swap(a,', 'async function swap(a,'),
    actual.replace('function swap(a,', 'function swap(a = 0,'),
  ];
  for (const source of invalid) assert.throws(() => checkProgram(parseJavaScript(source, options)), source);
  const imported = createTypedProgramLinks(actual, 'JavaScript', {javaScriptScalarLoopModel: model});
  for (const edit of [item => {item.sourceLoop.interface.parameters[0].kind = 'bool';},
    item => {item.sourceLoop.language = 'Rust';}, item => {item.sourceLoop.state[0].name = 'other';},
    item => {item.sourceLoop.transitions[0].writes[0].binding = 'b';}]) {
    const program = decodeTypedProgramLinks(LinkNetwork.fromLino(imported.network.toLino()), imported.root);
    edit(program.declarations.get('swap')); assert.throws(() => encodeTypedProgramLinks(program));
  }
  assert.throws(() => parseJavaScript(actual, {scalarLoopModel: 'another model'}), /scalar loop model/u);
  assert.throws(() => createTypedProgramLinks(cases[1].source + suffix, 'Rust', {javaScriptScalarLoopModel: model}), /requires JavaScript source/u);
});
test('source-free checked scalar loop edits change the actual parallel transition', () => {
  const imported = createTypedProgramLinks(cases[1].source + suffix, 'Rust', linkedOptions);
  const program = decodeTypedProgramLinks(LinkNetwork.fromLino(imported.network.toLino()), imported.root);
  const entry = program.declarations.get('swap');
  assert.equal(entry.body.then.name, 'ml_scalar_write0');
  entry.body.then.value = {k: 'binary', op: 'add', domain: {kind: 'float'}, semantics: 'ieee', left: {k: 'var', name: 'b', type: {kind: 'float'}},
    right: {k: 'lit', value: '1', type: {kind: 'float'}}, type: {kind: 'float'}};
  const edited = encodeTypedProgramLinks(program), network = LinkNetwork.fromLino(edited.network.toLino());
  const expected = {...cases[1], observations: [[[1,2,1,0],3], [[1,2,2,0],2]]};
  const javascript = emitJavaScript(decodeTypedProgramLinks(network, edited.root)).definitions.join('\n\n');
  observe(javascript, expected);
  const reverse = checkProgram(parseJavaScript(javascript, options));
  observe(emitJavaScript(reverse).text, expected);
  observe(emitJavaScript(checkProgram(parseRust(emitRust(reverse).definitions.join('\n\n') + suffix, options))).text, expected);
});
test('default and existing profiles retain exact refusal boundaries and unproved owners remain excluded', () => {
  for (const previous of [{}, {arrayResultModel: 'immutable fresh scalar vector results 1', completionModel: 'closed guarded function returns 1'}]) {
    assert.throws(() => parseRust(authentic + suffix, previous), error => {
      assert.equal(error.reason, 'mutable parameter: mutation is outside the portable core');
      assert.deepEqual(error.span, {start: 54, end: 57}); return true;
    });
    assert.throws(() => checkProgram(parseRust('fn ml_original()->f64{1f64}' + suffix, previous)), /reserved identifier/u);
  }
  const refused = [
    'fn keep(mut value:&f64)->f64{loop{return if true{(value,)=(value,);continue;}else{*value};}}',
    'fn keep(mut value:String)->String{loop{return if true{(value,)=(value,);continue;}else{value};}}',
    'fn keep(mut value:Vec<f64>)->f64{loop{return if true{(value,)=(value,);continue;}else{1f64};}}',
    'fn keep(mut value:u32)->u32{loop{return if true{(value,)=(value,);continue;}else{value};}}',
    'mod f64{} fn keep(mut value:f64)->f64{loop{return if true{(value,)=(value,);continue;}else{value};}}',
    'fn keep(value:f64)->f64{loop{return if true{(value,)=(value,);continue;}else{value};}}',
    'fn keep(mut value:f64)->f64{loop{return if true{(value,value)=(value,value);continue;}else{value};}}',
    'fn keep(mut value:f64)->f64{loop{return if true{(value,)=(value,);break;}else{value};}}',
    'fn keep(mut value:f64)->f64{loop{return if true{loop{continue;}}else{value};}}',
    'fn keep(mut value:f64)->f64{loop{return if true{println!("x");continue;}else{value};}}',
    'fn keep(mut value:f64)->f64{loop{return if true{let value=1f64;(value,)=(value,);continue;}else{value};}}',
    'fn foreign(value:f64)->f64{value} fn keep(mut value:f64)->f64{loop{return if true{(value,)=(foreign(value),);continue;}else{value};}}',
    'fn keep(mut value:f64)->f64{loop{if value<3f64{(value,)=(value+1f64,);continue;}else{value}}}',
    'fn keep(mut value:f64)->f64{loop{return if true{let pointer=&value;(value,)=(value,);continue;}else{value};}}',
    'fn keep(mut value:f64)->f64{loop{return if true{(value,)=(*(&value),);continue;}else{value};}}',
    'fn keep(mut value:f64)->f64{loop{return if true{let boxed=Box::new(value);(value,)=(value,);continue;}else{value};}}',
    'fn keep(mut value:f64)->f64{loop{return if true{(value,)=(Box::new(value),);continue;}else{value};}}',
    'fn keep(mut value:f64)->f64{loop{return if true{(value,)=(value,);continue;}else{Box::new(value)};}}',
  ];
  for (const source of refused) assert.throws(() => checkProgram(parseRust(source + suffix, options)), source);
  assert.throws(() => createTypedProgramLinks('function keep(){return 1;}', 'JavaScript', linkedOptions), /requires Rust source/u);
  assert.throws(() => parseRust(cases[0].source + suffix, {scalarLoopModel: 'another model'}), /scalar loop model/u);
});
test('forged name, state, continuation, write order and previous-state claims cannot survive graph checking', () => {
  const imported = createTypedProgramLinks(cases[1].source + suffix, 'Rust', linkedOptions);
  const fresh = () => decodeTypedProgramLinks(LinkNetwork.fromLino(imported.network.toLino()), imported.root);
  const edits = [
    value => { value.sourceLoop.sourceName = 'other'; },
    value => { value.generated = true; },
    value => { value.generated = null; },
    value => { value.params[0].default = {k: 'lit', value: '0', type: {kind: 'float'}}; },
    value => { value.sourceLoop.state[0].mutable = false; },
    value => { value.sourceLoop.transitions[0].arguments[0] = 'a'; },
    value => { value.sourceLoop.transitions[0].writes.reverse(); },
    value => { value.sourceLoop.transitions = []; },
    value => { value.sourceSignature.parameters[0].owner.path = ['f32']; },
    value => { value.sourceLoop.state[0].name = 'b'; },
    value => {
      let node = value.body.then;
      while (node.k === 'let' && node.name !== 'ml_scalar_write1') node = node.body;
      node.value = {k: 'var', name: 'ml_scalar_write0', type: {kind: 'float'}};
    },
  ];
  for (const edit of edits) {
    const program = fresh(); edit(program.declarations.get('swap'));
    assert.throws(() => encodeTypedProgramLinks(program));
  }
});
test('the profile makes no termination claim and preserves iterative divergence', () => {
  const source = 'fn forever(mut value:f64)->f64{loop{(value,)=(value+1f64,);continue;}}' + suffix;
  const emitted = emitJavaScript(checkProgram(parseRust(source, options))).text;
  for (const code of ['function forever(value){for(;;){value+=1;}}', emitted]) {
    assert.throws(() => vm.runInNewContext(code + '\nforever(0);', {}, {timeout: 30}), /Script execution timed out/u);
  }
});

test('paired draft fixtures retain actual complete sources, surfaces and definitions', () => {
 const fixture=readProjectData(new URL('../parity/fixtures/rust-scalar-loops.lino',import.meta.url));
 assert.equal(fixture.actualSource,authentic);assert.equal(fixture.cases.length,cases.length);
 for(const [index,entry] of cases.entries()) {
  const stored=fixture.cases[index],surface=parseRust(entry.source+suffix,options),program=checkProgram(surface);
  assert.equal(stored.source,entry.source+suffix);assert.equal(stored.originalJavaScript,entry.original);
  assert.deepEqual(stored.surface,JSON.parse(JSON.stringify(surface)));
  const definitions=emitJavaScript(program).definitions.join('\n\n');
  assert.equal(stored.javaScriptDefinitions,definitions);
  assert.deepEqual(stored.javaScriptReverseSurface,JSON.parse(JSON.stringify(parseJavaScript(definitions,options))));
  assert.equal(stored.rustDefinitions,emitRust(program).definitions.join('\n\n'));
  assert.equal(stored.reverseRustDefinitions,emitRust(checkProgram(parseJavaScript(definitions,options))).definitions.join('\n\n'));
 }
});
