import assert from 'node:assert/strict';
import test from 'node:test';
import vm from 'node:vm';
import { readFileSync } from 'node:fs';
import { readProjectData } from '../../js/scripts/project-data.mjs';
import { LinkNetwork, encodeLinkNetworkBinary, decodeLinkNetworkBinary } from '../../js/src/index.js';
import { checkProgram } from '../../js/src/translation/check.js';
import { parseJavaScript } from '../../js/src/translation/javascript.js';
import { emitJavaScript } from '../../js/src/translation/emit-javascript.js';
import { emitRust } from '../../js/src/translation/emit-rust.js';
import { createFreshNumberArrayCollector, checkFreshNumberArrayCollector, encodeFreshNumberArrayCollector,
  decodeFreshNumberArrayCollector, FRESH_NUMBER_ARRAY_COLLECTOR_POLICY as policy } from './model.mjs';
import { freshNumberArrayCollectorFixtures } from './generate-fixtures.mjs';
import { acceptFreshNumberArrayCollectorPattern } from './rules.mjs';

const create = source => createFreshNumberArrayCollector(source, { model: policy.model });
const execute = (source, name, input) => {
  const context = vm.createContext({});
  vm.runInContext(source, context, { timeout: 1000 });
  return Array.from(context[name](input));
};

test('five complete unchanged sources execute in original order and after value-state translation', () => {
  for (const entry of freshNumberArrayCollectorFixtures()) {
    const { program, proof } = create(entry.source);
    assert.deepEqual(execute(entry.source, entry.name, entry.input), entry.expected, entry.id);
    assert.deepEqual(execute(emitJavaScript(program).text, entry.name, entry.input), entry.expected, entry.id);
    assert.equal(checkFreshNumberArrayCollector(program, proof).source, entry.source);
    assert.equal(proof.parameter, program.declarations.get(entry.name).params[0]);
    assert.equal(proof.collector, program.declarations.get(entry.name).body);
    assert.equal(proof.collector.name, proof.bindings[2].name);
    assert.equal(proof.tokens[policy.declarations.collectorKeyword].value, 'const');
    assert.equal(program.referenceSemantics, undefined);
    assert.equal(proof.assumptions.length, 4);
    for (const input of [[], [0], [1, 2, 3, 4, 5]]) {
      assert.deepEqual(execute(emitJavaScript(program).text, entry.name, input), execute(entry.source, entry.name, input));
    }
  }
});

test('text and binary Links reload retain actual source binders and checked owner identities', () => {
  for (const entry of freshNumberArrayCollectorFixtures()) {
    const { program, proof } = create(entry.source), packet = encodeFreshNumberArrayCollector(program, proof);
    for (const network of [LinkNetwork.fromLino(packet.network.toLino()), decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))]) {
      const restored = decodeFreshNumberArrayCollector(network, packet.root);
      assert.equal(checkFreshNumberArrayCollector(restored.program, restored.proof).source, entry.source);
      assert.equal(restored.proof.declaration, restored.program.declarations.get(entry.name));
      assert.equal(restored.proof.parameter, restored.proof.declaration.params[0]);
      assert.equal(restored.proof.collector, restored.proof.declaration.body);
      assert.deepEqual(execute(emitJavaScript(restored.program).text, entry.name, entry.input), entry.expected);
    }
  }
});

test('all existing default push refusals remain exact and selection must be explicit', () => {
  const source = freshNumberArrayCollectorFixtures()[0].source;
  assert.throws(() => createFreshNumberArrayCollector(source), /select the exact experimental source model/u);
  assert.throws(() => createFreshNumberArrayCollector(source, { model: 'another collector' }), /select the exact/u);
  for (const referenceSemantics of [undefined, true, false]) {
    assert.throws(() => checkProgram(parseJavaScript(source, { referenceSemantics })), /\.push\(\).*has no checked operation/u);
  }
});

test('aliases, used lengths, argument order effects and unknown complete-source forms refuse', () => {
  const source = freshNumberArrayCollectorFixtures()[0].source;
  for (const invalid of [
    source.replace('result.push(', 'const alias=result;result.push('),
    source.replace('result.push(', 'const size=result.push('),
    source.replace('result.push(values[index]??-1);', 'return result.push(values[index]??-1);'),
    source.replace('result.push(values[index]??-1);', 'result.push(1,2);'),
    source.replace('result.push(values[index]??-1);', 'result.push(result.push(1));'),
    source.replace('result.push(values[index]??-1);', 'result.push(other());'),
    source.replace('result.push(values[index]??-1);', 'result.push(result[0]);'),
    source.replace('result.push(values[index]??-1);', 'result.push();'),
    source.replace('const result=[];', 'const result=values;'),
    source.replace('const result=[];', 'const result=Object.freeze([]);'),
    source.replace('const result=[];', 'const result={get push(){return other;}};'),
    source.replace('const result=[];', 'const result=[];result.push=other;'),
    source.replace('return result;', 'return result===values;'),
    source.replace('index+=1', 'index+=2'),
    source.replace('index<values.length', 'index<=values.length'),
    source.replace('const result=[];', 'const result=[];consume(result);'),
    source + '\nfunction extra(){return 1;}',
    'Array.prototype.push=other;\n' + source,
    source.replace('const result=[];', 'const result=[];const closure=()=>result;'),
    source.replace('result.push(values[index]??-1);', 'box.result.push(values[index]??-1);'),
    source.replace('const result=[];', 'const result=[];values[0]=1;'),
    source.replace('let index=0', 'let result=0'),
  ]) assert.throws(() => create(invalid), /Fresh Number array collector|unsupported|syntax/u, invalid);
});

test('copied owners, changed source intervals, premises and represented core edits do not pass', () => {
  const source = freshNumberArrayCollectorFixtures()[0].source;
  for (const corrupt of [
    ({ proof }) => { proof.parameter = structuredClone(proof.parameter); },
    ({ proof }) => { proof.collector = structuredClone(proof.collector); },
    ({ proof }) => { proof.tokens[30].binding = structuredClone(proof.bindings[2]); },
    ({ proof }) => { proof.tokens[30].span.start += 1; },
    ({ proof }) => { proof.gaps[0].units.push(32); },
    ({ proof }) => { proof.lowering[1].to = 'result=[];'; },
    ({ proof }) => { proof.assumptions.pop(); },
    ({ proof }) => { proof.assumptions[0].meaning = 'no assumptions'; },
    ({ proof }) => { proof.assumptions[1].owner = structuredClone(proof.parameter); },
    ({ proof }) => { proof.extra = true; },
    ({ program }) => { program.main.effects.push({ k: 'print' }); },
    ({ proof }) => { Object.defineProperty(proof.tokens, 'extra', { value: true }); },
  ]) {
    const result = create(source); corrupt(result);
    assert.throws(() => encodeFreshNumberArrayCollector(result.program, result.proof), /Fresh Number array collector/u);
  }
});

test('the complete generated decision and all selected artifacts are automatically emitted, never substituted', () => {
  const artifact = readProjectData(new URL('./data/rule-translation.lino', import.meta.url));
  assert.equal(artifact.source, readFileSync(new URL('./rules.mjs', import.meta.url), 'utf8'));
  const emitted = emitRust(checkProgram(parseJavaScript(artifact.source)));
  assert.equal(emitted.text, artifact.completeRust);
  assert.deepEqual(emitted.definitions, artifact.definitions);
  assert.ok(artifact.completeRust.includes('pub fn accept_fresh_number_array_collector_pattern'));
  const fixtures = readProjectData(new URL('./data/fixtures.lino', import.meta.url));
  for (const entry of fixtures.examples) {
    const translated = emitRust(create(entry.source).program);
    assert.equal(translated.text, entry.completeRust);
    assert.deepEqual(translated.definitions, entry.definitions);
    assert.deepEqual(execute(entry.source, entry.name, entry.input), entry.expected);
  }
});

test('portable pattern identity codes are bounded integers and every lexical use keeps its binder', () => {
  const codes = [1, 1000, 1001, 1002, 1003, 1001, 1002, 1003];
  const pattern = [1, -1, -2, -3, -4, -2, -3, -4];
  assert.equal(acceptFreshNumberArrayCollectorPattern(codes, pattern), true);
  for (const value of [NaN, Infinity, -1, 1000.5, 1256, 9007199254740992]) {
    const changed = [...codes]; changed[1] = value;
    assert.equal(acceptFreshNumberArrayCollectorPattern(changed, pattern), false);
  }
  for (const index of [2, 3, 4, 5, 6, 7]) {
    const changed = [...codes]; changed[index] = 1000;
    assert.equal(acceptFreshNumberArrayCollectorPattern(changed, pattern), false);
  }
});
