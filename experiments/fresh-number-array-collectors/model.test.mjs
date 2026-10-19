import assert from 'node:assert/strict';
import test from 'node:test';
import vm from 'node:vm';
import { readFileSync } from 'node:fs';
import { readProjectData } from '../../js/scripts/project-data.mjs';
import { LinkNetwork, encodeLinkNetworkBinary, decodeLinkNetworkBinary } from '../../js/src/index.js';
import { TYPED_PROGRAM_LINKS_SCHEMA as schema, encodeTypedProgramLinks, decodeTypedProgramLinks } from '../../js/src/typed-program-links.js';
import { checkProgram } from '../../js/src/translation/check.js';
import { parseJavaScript } from '../../js/src/translation/javascript.js';
import { emitJavaScript } from '../../js/src/translation/emit-javascript.js';
import { emitRust } from '../../js/src/translation/emit-rust.js';
import { createFreshNumberArrayCollector, checkFreshNumberArrayCollector, encodeFreshNumberArrayCollector,
  decodeFreshNumberArrayCollector, FRESH_NUMBER_ARRAY_COLLECTOR_POLICY as policy } from './model.mjs';
import { freshNumberArrayCollectorFixtures } from './generate-fixtures.mjs';
import { acceptFreshNumberArrayCollectorPattern, acceptFreshNumberArrayCollectorBinding, acceptFreshNumberArrayCollectorFunctionName } from './rules.mjs';

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
    assert.deepEqual(proof.sourceContext, policy.sourceContext);
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
      assert.deepEqual(restored.proof.sourceContext, policy.sourceContext);
      assert.deepEqual(execute(emitJavaScript(restored.program).text, entry.name, entry.input), entry.expected);
    }
  }
});

test('the experimental model remains explicit while original fresh collectors also use production append proofs', () => {
  const source = freshNumberArrayCollectorFixtures()[0].source;
  assert.throws(() => createFreshNumberArrayCollector(source), /select the exact experimental source model/u);
  assert.throws(() => createFreshNumberArrayCollector(source, { model: 'another collector' }), /select the exact/u);
  // CI 38911564946 job 117015296609 showed that the former unconditional
  // default refusal is obsolete for owned construction. Explicit reference
  // selection still retains the original operation and its exact refusal.
  for (const entry of freshNumberArrayCollectorFixtures()) for (const referenceSemantics of [undefined, true, false]) {
    if (referenceSemantics === true) {
      const operation = /\b[A-Za-z][A-Za-z_0-9]*\.push\([^;]+\)/u.exec(entry.source);
      assert.ok(operation, entry.id);
      assert.throws(() => checkProgram(parseJavaScript(entry.source, { referenceSemantics })), error => error.kind === 'unsupported'
        && error.reason === 'method call .push(): .push() has no checked operation on array:float'
        && error.span.start === operation.index && error.span.end === operation.index + operation[0].length, entry.id);
      continue;
    }
    const program = checkProgram(parseJavaScript(entry.source, { referenceSemantics }));
    assert.deepEqual(execute(entry.source, entry.name, entry.input), entry.expected);
    assert.deepEqual(execute(emitJavaScript(program).text, entry.name, entry.input), entry.expected);
    const packet = encodeTypedProgramLinks(program);
    for (const network of [LinkNetwork.fromLino(packet.network.toLino()), decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))]) {
      const restored = decodeTypedProgramLinks(network, packet.root);
      assert.deepEqual(execute(emitJavaScript(restored).text, entry.name, entry.input), entry.expected);
    }
  }
  for (const invalid of [
    source.replace('result.push(', 'const alias=result;result.push('),
    source.replace('result.push(', 'const size=result.push('),
    source.replace('result.push(values[index]??-1);', 'return result.push(values[index]??-1);'),
    source.replace('const result=[];', 'const result=values;'),
  ]) {
    assert.throws(() => checkProgram(parseJavaScript(invalid, { referenceSemantics: false })), /\.push\(\)|type|expected/u);
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
    ({ proof }) => { proof.assumptions[1].premiseOwner = structuredClone(proof.parameter); },
    ({ proof }) => { proof.sourceContext.sourceGoal = 'Module'; },
    ({ proof }) => { proof.sourceContext.initialGlobalRealm = 'arbitrary host realm'; },
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

test('premise relations use their actual operation-owner role and saved core LinkIds', () => {
  const { program, proof } = create(freshNumberArrayCollectorFixtures()[0].source);
  const packet = encodeFreshNumberArrayCollector(program, proof);
  const fields = packet.network.fieldRelations();
  const field = (parent, name) => fields.find(row => row.parent.equals(parent) && row.label === (schema.fields[name] ?? name.replace(/[A-Z]/gu, letter => ' ' + letter.toLowerCase())));
  const body = packet.network.link(packet.root).references()[0];
  const mapping = field(body, 'declarations');
  const declarations = packet.network.link(mapping.child).references().map(pair => packet.network.link(pair).references()[1]);
  const mainDeclaration = declarations.find(id => packet.network.link(field(id, 'name').child).metadata().definition === proof.declaration.name);
  const parameters = packet.network.link(field(mainDeclaration, 'params').child).references();
  const collector = field(mainDeclaration, 'body').child;
  const loop = declarations.find(id => !id.equals(mainDeclaration));
  const owners = fields.filter(row => row.label === 'premise owner');
  assert.equal(owners.length, 4);
  assert.equal(fields.filter(row => row.label === schema.fields.owner).length, 0,
    'collector premises are not registered source-type-owner facts');
  assert.deepEqual(owners.map(row => row.child.asU64().toString()),
    [mainDeclaration, parameters[0], collector, loop].map(id => id.asU64().toString()));
  assert.ok(owners.every(row => row.child.equals(field(row.parent, 'premiseOwner').child)));
});

test('direct checks and saved declaration edges reject split ordered-item and mapping ownership', () => {
  const source = freshNumberArrayCollectorFixtures()[0].source;
  for (const splitMap of [false, true]) {
    const { program, proof } = create(source);
    const declaration = proof.declaration, copied = { ...declaration };
    if (splitMap) {
      program.declarations.set(declaration.fullName, copied);
      proof.declaration = copied;
      proof.assumptions[0].premiseOwner = copied;
    } else program.items[program.items.indexOf(declaration)] = copied;
    assert.throws(() => checkFreshNumberArrayCollector(program, proof), /ordered items.*unowned declaration/u);
  }
  for (const splitMap of [false, true]) {
    const { program, proof } = create(source);
    const packet = encodeFreshNumberArrayCollector(program, proof), network = packet.network;
    const fields = network.fieldRelations();
    const field = (parent, name) => fields.find(row => row.parent.equals(parent) && row.label === (schema.fields[name] ?? name));
    const body = network.link(packet.root).references()[0], mapping = field(body, 'declarations');
    const pair = network.link(mapping.child).references()[0];
    const [key, declaration] = network.link(pair).references();
    const original = network.link(declaration);
    const copied = network.insertLink(original.references(), original.metadata().clone());
    for (const row of fields.filter(row => row.parent.equals(declaration))) network.insertFieldRelation(copied, row.label, row.child);
    if (splitMap) {
      network.link(pair).setReferences([key, copied]);
      for (const row of fields.filter(row => row.child.equals(declaration) && ['declaration', 'premise owner'].includes(row.label))) {
        row.link.setReferences([row.parent, row.link.references()[1], copied]);
      }
    } else {
      const items = field(body, 'items');
      network.link(items.child).setReferences(network.link(items.child).references().map(id => id.equals(declaration) ? copied : id));
    }
    const restored = LinkNetwork.fromLino(network.toLino());
    assert.throws(() => decodeFreshNumberArrayCollector(restored, packet.root), /declaration|binding|identity|owner/u);
  }
});

test('every source binder refuses unconditional reserved words using portable policy and an independent syntax oracle', () => {
  const source = freshNumberArrayCollectorFixtures()[0].source;
  for (const name of policy.unconditionallyReservedBindings) {
    assert.equal(acceptFreshNumberArrayCollectorBinding(name), false, name);
    for (const binder of ['collect', 'values', 'result', 'index']) {
      const invalid = source.replace(new RegExp('\\b' + binder + '\\b', 'gu'), name);
      assert.throws(() => new vm.Script(invalid), error => error.name === 'SyntaxError', binder + ':' + name);
      assert.throws(() => create(invalid), /Fresh Number array collector|unsupported|syntax/u, binder + ':' + name);
    }
  }
  for (const [binder, name] of [['collect', 'enumCount'], ['values', 'Array'], ['result', 'undefined'], ['result', 'NaN']]) {
    assert.equal(acceptFreshNumberArrayCollectorBinding(name), true, name);
    const valid = source.replace(new RegExp('\\b' + binder + '\\b', 'gu'), name);
    assert.doesNotThrow(() => new vm.Script(valid));
    const selected = create(valid);
    assert.deepEqual(execute(emitJavaScript(selected.program).text, selected.proof.declaration.name, [3, 4]), [3, 4]);
  }
});

test('global function declaration restrictions do not become restrictions on valid local binders', () => {
  const source = freshNumberArrayCollectorFixtures()[0].source;
  const selected = create(source);
  const { loweredSource } = checkFreshNumberArrayCollector(selected.program, selected.proof);
  for (const name of policy.forbiddenGlobalFunctionNames) {
    assert.equal(acceptFreshNumberArrayCollectorBinding(name), true, 'the local spelling remains valid');
    assert.equal(acceptFreshNumberArrayCollectorFunctionName(name), false, name);
    const original = source.replace(/\bcollect\b/gu, name);
    assert.throws(() => execute(original, name, [3, 4]), error => error.name === 'SyntaxError',
      'actual Script declaration instantiation rejects the original global function');
    // Retain the actual former mismatch as a counterexample. The ordinary
    // value program is compiled independently; it is never selected here.
    const candidate = emitJavaScript(checkProgram(parseJavaScript(loweredSource.replace(/\bcollect\b/gu, name))));
    assert.throws(() => execute(candidate.text, name, [3, 4]), error => error.name === 'TypeError',
      'the emitted Script would reach a different failure when called by the source name');
    assert.throws(() => create(original), /global function declaration conflicts/u);

    for (const binder of ['values', 'result', 'index']) {
      const local = source.replace(new RegExp('\\b' + binder + '\\b', 'gu'), name);
      const localProgram = create(local);
      assert.deepEqual(execute(local, 'collect', [3, 4]), [3, 4]);
      assert.deepEqual(execute(emitJavaScript(localProgram.program).text, 'collect', [3, 4]), [3, 4]);
    }
  }
  const ordinary = source.replace(/\bcollect\b/gu, 'collectValues');
  assert.equal(acceptFreshNumberArrayCollectorFunctionName('collectValues'), true);
  assert.deepEqual(execute(ordinary, 'collectValues', [3, 4]), [3, 4]);
  assert.deepEqual(execute(emitJavaScript(create(ordinary).program).text, 'collectValues', [3, 4]), [3, 4]);
});
