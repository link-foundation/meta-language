import assert from 'node:assert/strict';
import test from 'node:test';
import {readProjectData} from '../scripts/project-data.mjs';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {runInNewContext} from 'node:vm';
import {relocateCheckedModuleImports} from '../../experiments/rust-text-certificate-adapter/overlay.mjs';
import {arrayCallbackRulesPolicy,readArrayCallbackSourceModule,readArrayCallbackLibraryStyles} from '../scripts/generate-array-callback-rules.mjs';
import {buildArrayCallbackFixtures,readArrayCallbackFixtureCases,readArrayCallbackReferenceFixtureCases,observeArrayCallbackProgram} from '../scripts/generate-array-callback-fixtures.mjs';
import * as rules from '../src/translation/array-callback-rules.js';
import {DecoratorSet} from '../src/decorators.js';
import {parseJavaScript} from '../src/translation/javascript.js';
import {checkProgram} from '../src/translation/check.js';
import {emitRust} from '../src/translation/emit-rust.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
import {typeError,unsupported} from '../src/translation/diagnostics.js';
import {BOOL,FLOAT,UNIT,array,sameType,typeKey} from '../src/translation/types.js';
import {readArrayFromMapCallback} from '../src/translation/array-from.js';
import {readArrayFromMapCallbackError} from '../src/translation/array-from-rules.js';
import {encodeTypedProgramLinks,decodeTypedProgramLinks} from '../src/typed-program-links.js';
import {recheckLinkedTypedProgram} from '../src/typed-program-checked-projection.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {restoreTranslationSourceHistory} from '../scripts/translation-source-history.mjs';

// Frozen original source seams and exact current inverse, captured before integration.
const historical = {
  "hashes": {
    "check.js": "e4a95a59a9f9878d2842928db794a4973eaa56daae8e11a970dfafd4f15f7551",
    "emit-rust.js": "69315a498f97e54b16f2fb8f3d7454d84e818ddce7bb89904b4ae85279af1f79",
    "javascript-infer.js": "3b4afafb648b8205408c4ca8832803bcb3ec20fb4ac3dc989d5ddec1548a5471",
    "javascript-surface-rules.js": "794d918304dac11b384a3d29a1fd83235cacd51cd2cfe19ca630cfa2833ac40c"
  },
  "callback": "      const callback = (index, params, ret) => {\n        const value = this.expr(args[index], env, path, { kind: 'function', params, ret });\n        if (value.type.kind !== 'function') throw typeError(`.${node.op}() requires a function`, node.span);\n        if (value.type.params.length > params.length || value.type.params.some((type, index) => !sameType(type, params[index]))) throw typeError(`.${node.op}() callback parameters differ`, node.span);\n        if (ret && !sameType(value.type.ret, ret)) throw typeError(`.${node.op}() callback result must be ${typeKey(ret)}`, node.span);\n        return value;\n      };\n",
  "checkerBranches": "      if (['map', 'filter', 'find', 'findIndex', 'some', 'every', 'flatMap', 'forEach'].includes(node.op)) {\n        count(1);\n        const predicate = ['filter', 'find', 'findIndex', 'some', 'every'].includes(node.op);\n        const fn = callback(0, [element, FLOAT, object.type], predicate ? BOOL : undefined);\n        let type = node.op === 'map' ? array(fn.type.ret) : object.type;\n        if (node.op === 'flatMap') {\n          if (fn.type.ret.kind !== 'array') throw typeError('flatMap callback must return an array', node.span);\n          type = fn.type.ret;\n        }\n        if (node.op === 'find') type = { kind: 'option', element, absence: 'undefined' };\n        if (node.op === 'findIndex') type = FLOAT;\n        if (node.op === 'some' || node.op === 'every') type = BOOL;\n        if (node.op === 'forEach') type = UNIT;\n        return { k: 'method', object, op: node.op, args: [fn], type, span: node.span };\n      }\n      if (node.op === 'reduce') {\n        count(1, 2);\n        if (!args[1]) {\n          const fn = callback(0, [element, element, FLOAT, object.type], element);\n          return { k: 'method', object, op: node.op, args: [fn], type: element, span: node.span };\n        }\n        const initial = this.expr(args[1], env, path, expected);\n        const fn = callback(0, [initial.type, element, FLOAT, object.type], initial.type);\n        return { k: 'method', object, op: node.op, args: [fn, initial], type: initial.type, span: node.span };\n      }\n",
  "mapChecker": "      if (node.op === 'map') {\n        count(1);\n        const params = [element, FLOAT, object.type];\n        const fn = this.expr(args[0], env, path, { kind: 'function', params, ret: undefined });\n        const selection = readArrayFromMapCallback(fn.type, params);\n        const error = readArrayFromMapCallbackError(selection.status, selection.pairs.every(([left, right]) => sameType(left, right)));\n        if (error) throw typeError(error, node.span);\n        return { k: 'method', object, op: node.op, args: [fn], type: array(selection.ret), span: node.span };\n      }\n",
  "renderer": "      const available = e.op === 'reduce' ? [`ml_accumulator${suffix}`, value, `(${index} as f64)`, `${values}.clone()`] : [value, `(${index} as f64)`, `${values}.clone()`];\n      const call = `${callback}(${available.slice(0, fn.type.params.length).join(', ')})`;\n      const head = `let ${values} = ${this.expr(e.object)}; let ${callback} = ${this.expr(fn)};`;\n      const iterator = `${values}.iter().cloned().enumerate()`;\n      if (e.op === 'some' || e.op === 'every') return `{ ${head} ${iterator}.${e.op === 'some' ? 'any' : 'all'}(|(${index}, ${value})| ${call}) }`;\n      if (e.op === 'findIndex') return `{ ${head} ${iterator}.position(|(${index}, ${value})| ${call}).map_or(-1f64, |ml_position| ml_position as f64) }`;\n      if (e.op === 'flatMap') return `{ ${head} ${iterator}.flat_map(|(${index}, ${value})| ${call}).collect::<Vec<_>>() }`;\n      if (e.op === 'filter' || e.op === 'find') {\n        const predicateArgs = available.slice(0, fn.type.params.length).map((argument, index) => index === 0 ? `${value}.clone()` : argument);\n        const predicate = `${callback}(${predicateArgs.join(', ')})`;\n        if (e.op === 'filter') return `{ ${head} ${iterator}.filter_map(|(${index}, ${value})| if ${predicate} { Some(${value}) } else { None }).collect::<Vec<_>>() }`;\n        return `{ ${head} ${iterator}.find_map(|(${index}, ${value})| if ${predicate} { Some(${value}) } else { None }) }`;\n      }\n      if (e.op === 'forEach') return `{ ${head} for (${index}, ${value}) in ${iterator} { ${call}; } () }`;\n      if (e.op === 'reduce') {\n        if (e.args.length === 1) return `{ ${head} let mut ml_iterator${suffix} = ${iterator}; let ml_initial${suffix} = ml_iterator${suffix}.next().map(|(_, ml_first)| ml_first).expect(\"Reduce of empty array with no initial value\"); ml_iterator${suffix}.fold(ml_initial${suffix}, |ml_accumulator${suffix}, (${index}, ${value})| ${call}) }`;\n        return `{ ${head} ${iterator}.fold(${this.expr(e.args[1])}, |ml_accumulator${suffix}, (${index}, ${value})| ${call}) }`;\n      }",
  "mapRenderer": "      if (e.op === 'map') return renderArrayFromMapping('Rust', this.expr(e.object), this.expr(fn), fn.type.params.length, String(suffix));\n",
  "prefixBytes": 5994,
  "prefixSha256": "31c9614138653e1ea7944849c56a0b942930a839ff4c70a0b0d1770994c57b0a"
};
const integration = {
  "checkerImport": "import { readArrayCallbackResultForm, readArrayCallbackSignatureError, readArrayCallbackParameterRole,\n  readArrayCallbackArgumentCountError, readArrayCallbackCheckedResultContext,\n  readArrayCallbackResultError, readArrayCallbackCheckedArgumentPosition } from './array-callback-rules.js';\n",
  "emitterImport": "import { renderArrayCallbackExpression, renderArrayCallbackReferenceExpression, readArrayCallbackCheckedArgumentPosition } from './array-callback-rules.js';\n",
  "callback": "      const callback = (index, params, ret) => {\n        const value = this.expr(args[index], env, path, { kind: 'function', params, ret });\n        const kindError = readArrayCallbackSignatureError(node.op, value.type.kind, 0, params.length, true, true, '');\n        if (kindError) throw typeError(kindError, node.span);\n        const parametersMatch = value.type.params.length > params.length\n          || value.type.params.every((type, index) => sameType(type, params[index]));\n        const parameterError = readArrayCallbackSignatureError(node.op, value.type.kind,\n          value.type.params.length, params.length, parametersMatch, true, '');\n        if (parameterError) throw typeError(parameterError, node.span);\n        if (ret) {\n          const resultMatches = sameType(value.type.ret, ret);\n          const resultError = readArrayCallbackSignatureError(node.op, value.type.kind,\n            value.type.params.length, params.length, true, resultMatches, resultMatches ? '' : typeKey(ret));\n          if (resultError) throw typeError(resultError, node.span);\n        }\n        return value;\n      };\n",
  "checkerBranches": "      const callbackForm = readArrayCallbackResultForm(node.op);\n      if (callbackForm) {\n        const countError = readArrayCallbackArgumentCountError(node.op, args.length);\n        if (countError) throw unsupported(`method call .${node.op}()`, countError, node.span);\n        const initialPosition = readArrayCallbackCheckedArgumentPosition(node.op, Boolean(args[1]), 0);\n        const initial = initialPosition === 1 ? this.expr(args[1], env, path, expected) : null;\n        const accumulator = initial ? initial.type : element;\n        const params = [];\n        for (let position = 0; ; position += 1) {\n          const role = readArrayCallbackParameterRole(node.op, position);\n          if (!role) break;\n          params.push(role === 'accumulator' ? accumulator : role === 'element' ? element : role === 'index' ? FLOAT : object.type);\n        }\n        const context = readArrayCallbackCheckedResultContext(node.op);\n        const fn = callback(0, params, context === 'bool' ? BOOL : context === 'accumulator' ? accumulator : undefined);\n        const resultError = readArrayCallbackResultError(node.op, fn.type.ret.kind);\n        if (resultError) throw typeError(resultError, node.span);\n        const type = callbackForm === 'receiver' ? object.type\n          : callbackForm === 'optional element' ? { kind: 'option', element, absence: 'undefined' }\n          : callbackForm === 'float' ? FLOAT : callbackForm === 'bool' ? BOOL\n          : callbackForm === 'unit' ? UNIT : callbackForm === 'accumulator' ? accumulator\n          : callbackForm === 'mapped array' ? array(fn.type.ret) : fn.type.ret;\n        return { k: 'method', object, op: node.op, args: initial ? [fn, initial] : [fn], type, span: node.span };\n      }\n",
  "renderer": "      const parameterCount = fn.type.params.length;\n      const object = this.expr(e.object);\n      const callbackText = this.expr(fn);\n      const hasInitial = readArrayCallbackCheckedArgumentPosition(e.op, e.args.length !== 1, 0) === 1;\n      const initial = hasInitial ? this.expr(e.args[1]) : '';\n      const referenceResult = fn.type.ret.kind === 'array' && fn.type.ret.reference === true;\n      const rendered = renderArrayCallbackReferenceExpression(e.op, object, callbackText, parameterCount, String(suffix), referenceResult)\n        || renderArrayCallbackExpression(e.op, object, callbackText, parameterCount, String(suffix), initial, hasInitial);\n      if (rendered) return rendered;",
  "emitterPrefix": "      this.temporaries += 1;\n      const suffix = this.temporaries;\n      const fn = e.args[0];\n"
};

const digest = value => createHash('sha256').update(value).digest('hex');
const translationRoot = new URL('../src/translation/', import.meta.url);
const readSource = name => restoreTranslationSourceHistory(readFileSync(new URL(name, translationRoot), 'utf8'),
  'js/src/translation/' + name, '81107a5945651f17c962e4bb81fb560f96c65ccd');
const operations = ['filter', 'find', 'findIndex', 'some', 'every', 'flatMap', 'forEach', 'reduce'];

function replaceOnce(source, from, to) {
  assert.equal(source.split(from).length, 2, 'the exact reviewed source seam occurs once');
  return source.replace(from, to);
}
function readOriginalSources() {
  let checker = replaceOnce(readSource('check.js'), integration.checkerImport, '');
  checker = replaceOnce(checker, integration.callback, historical.callback);
  checker = replaceOnce(checker, integration.checkerBranches, historical.checkerBranches);
  let emitter = replaceOnce(readSource('emit-rust.js'), integration.emitterImport, '');
  emitter = replaceOnce(emitter, integration.renderer, historical.renderer);
  const bindings = ['values', 'callback', 'index', 'value'].map(name =>
    '      const ' + name + ' = ' + String.fromCharCode(96) + 'ml_' + name + '$' + '{suffix}' + String.fromCharCode(96) + ';\n').join('');
  emitter = replaceOnce(emitter, integration.emitterPrefix,
    integration.emitterPrefix.replace('      const fn = e.args[0];\n', bindings + '      const fn = e.args[0];\n'));
  assert.equal(digest(checker), historical.hashes['check.js'], 'every original checker byte is restored');
  assert.equal(digest(emitter), historical.hashes['emit-rust.js'], 'every original emitter byte is restored');
  return {'check.js': checker, 'emit-rust.js': emitter};
}
async function readOriginalModules() {
  return Object.fromEntries(await Promise.all(Object.entries(readOriginalSources()).map(async ([name, source]) => {
    const relocated = relocateCheckedModuleImports(source, new URL(name, translationRoot).href, value => value);
    return [name, await import('data:text/javascript;base64,' + Buffer.from(relocated).toString('base64'))];
  })));
}

test('callback extraction preserves complete original sources map blocks and unchanged inference owners', () => {
  readOriginalSources();
  for (const name of ['javascript-infer.js', 'javascript-surface-rules.js'])
    assert.equal(digest(readSource(name)), historical.hashes[name]);
  assert.ok(readSource('check.js').includes(historical.mapChecker));
  assert.ok(readSource('emit-rust.js').includes(historical.mapRenderer));
  const source = Buffer.from(readArrayCallbackSourceModule());
  assert.equal(digest(source.subarray(0, historical.prefixBytes)), historical.prefixSha256, 'all eight original pure functions retain their exact prefix');
  assert.equal(digest(source.subarray(0, 7249)), 'a5fa80c7683dc2a127b2500b464e049ccf23d29979ff596aa14999c369e5c554',
    'all twelve previous shared functions retain every original byte');
  assert.equal([...source.toString().matchAll(/^export function /gmu)].length, 13);
});

test('all 224 original callback renderings retain complete template bytes and checked JavaScript behavior', () => {
  const original = new Function('e', 'suffix', 'values', 'callback', 'index', 'value', 'fn', historical.renderer);
  const checked = checkProgram(parseJavaScript(readArrayCallbackSourceModule(), {referenceSemantics: false}));
  const emitted = emitRust(checked, {arrayConsumption: true}), context = {};
  assert.deepEqual(emitted.preludes, []);
  assert.equal(checked.declarations.size, 14);
  runInNewContext(emitJavaScript(checked).text, context, {timeout: 1000});
  let count = 0;
  for (const operation of operations) for (let parameterCount = 0; parameterCount < 7; parameterCount += 1)
    for (const suffix of ['1', '27']) for (const hasInitial of [false, true]) {
      const args = [{text: 'makeCallback()', type: {params: Array(parameterCount).fill(FLOAT)}}, ...(hasInitial ? [{text: 'makeInitial()'}] : [])];
      const expression = {op: operation, object: {text: 'makeReceiver()'}, args};
      const expected = original.call({expr: value => value.text}, expression, suffix, 'ml_values' + suffix,
        'ml_callback' + suffix, 'ml_index' + suffix, 'ml_value' + suffix, args[0]);
      const input = [operation, 'makeReceiver()', 'makeCallback()', parameterCount, suffix, 'makeInitial()', hasInitial];
      assert.equal(rules.renderArrayCallbackExpression(...input), expected);
      assert.equal(context.renderArrayCallbackExpression(...input), expected);
      count += 1;
    }
  assert.equal(count, 224);
  assert.equal(rules.renderArrayCallbackExpression('unrepresented', '', '', 0, '', '', false), '');
  const packet = encodeTypedProgramLinks(checked);
  for (const network of [LinkNetwork.fromLino(packet.network.toLino()), decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))]) {
    const restored = decodeTypedProgramLinks(network, packet.root);
    assert.deepEqual(restored, checked);
    assert.equal(recheckLinkedTypedProgram(restored, {wholeProgram: true}), restored);
    assert.deepEqual(emitRust(restored, {arrayConsumption: true}), emitted);
  }
});

test('actual callback programs retain both reference profiles full checked archives and original target behavior', async () => {
  const originals = await readOriginalModules(), fixture = buildArrayCallbackFixtures();
  assert.equal(fixture.cases.length, 18);
  let corrected = 0;
  for (const row of fixture.cases) {
    assert.deepEqual(row.profiles.map(profile => profile.referenceSemantics), row.referenceProfiles ?? [false, true]);
    for (const profile of row.profiles) {
      if (profile.checkDiagnostic) {
        for (const checker of [originals['check.js'].checkProgram, checkProgram]) assert.throws(() => checker(structuredClone(profile.surface)), error => {
          assert.deepEqual({kind: error.kind, reason: error.reason, span: error.span ?? null, details: error.details ?? null}, profile.checkDiagnostic);
          return true;
        });
        assert.equal(profile.referenceSemantics, true);
        assert.equal(profile.checkDiagnostic.reason, '.' + row.operation + '() callback parameters differ');
        continue;
      }
      const original = originals['check.js'].checkProgram(structuredClone(profile.surface));
      const current = checkProgram(structuredClone(profile.surface));
      assert.deepEqual(current, original, row.id + ': complete original checked records');
      assert.deepEqual(observeArrayCallbackProgram(emitJavaScript(current).text), row.expectedOutput);
      let output;
      try {
        const emitted = originals['emit-rust.js'].emitRust(original, {arrayConsumption: true});
        output = {emitted: {...emitted, preludes: emitted.preludes, definitions: emitted.definitions}};
      }
      catch (error) {
        if (!error.kind) throw error;
        output = {diagnostic: {kind: error.kind, reason: error.reason, span: error.span ?? null, details: error.details ?? null}};
      }
      const method = current.declarations.get('run').body;
      if (method.k === 'method' && method.op === 'flatMap' && method.args[0].type.ret.reference === true) {
        const previous = 'ml_values1.iter().cloned().enumerate().flat_map(|(ml_index1, ml_value1)| ml_callback1(ml_value1, (ml_index1 as f64))).collect::<Vec<_>>()';
        const replacement = 'std::rc::Rc::new(std::cell::RefCell::new(ml_values1.iter().cloned().enumerate().flat_map(|(ml_index1, ml_value1)| { let ml_callback_values1 = ml_callback1(ml_value1, (ml_index1 as f64)).borrow().clone(); ml_callback_values1 }).collect::<Vec<_>>()))';
        const inverse = structuredClone(profile.targets.Rust);
        inverse.emitted.text = replaceOnce(inverse.emitted.text, replacement, previous);
        let definitions = 0;
        inverse.emitted.definitions = inverse.emitted.definitions.map(definition => {
          if (!definition.includes(replacement)) return definition;
          definitions += 1;
          return replaceOnce(definition, replacement, previous);
        });
        assert.equal(definitions, 1);
        assert.deepEqual(inverse, output, row.id + ': exact inverse retains every historical target field and byte');
        corrected += 1;
      } else assert.deepEqual(profile.targets.Rust, output, row.id + ': complete original Rust target behavior');
      assert.equal(digest(profile.checkedArchive.text), profile.checkedArchive.textSha256);
      assert.equal(digest(Buffer.from(profile.checkedArchive.binaryHex, 'hex')), profile.checkedArchive.binarySha256);
      assert.equal(profile.checkedOwners.length, original.declarations.size);
    }
  }
  assert.equal(corrected, 1, 'only the actual reference-array flatMap result needs target representation repair');
});

test('all original whole-program callback arity inputs retain exact errors in both profiles', async () => {
  const originals = await readOriginalModules();
  let count = 0;
  for (const row of [...readArrayCallbackFixtureCases(), ...readArrayCallbackReferenceFixtureCases()]) for (const referenceSemantics of row.referenceProfiles ?? [false, true]) {
    const surface = parseJavaScript(row.source, {referenceSemantics});
    const owner = surface.items.find(item => item.k === 'fn' && item.name === 'run');
    assert.equal(owner.body.k, 'method');
    for (let size = 0; size <= 4; size += 1) {
      const candidate = structuredClone(surface);
      candidate.items.find(item => item.k === 'fn' && item.name === 'run').body.args =
        Array.from({length: size}, (_, index) => owner.body.args[index] ?? owner.body.args[0]);
      const run = checker => {
        try { return {value: checker(structuredClone(candidate))}; }
        catch (error) { return {error: {kind: error.kind, reason: error.reason, span: error.span, details: error.details}}; }
      };
      const actual = run(checkProgram);
      assert.deepEqual(actual, run(originals['check.js'].checkProgram));
      const error = rules.readArrayCallbackArgumentCountError(row.operation, size);
      if (error) assert.equal(actual.error.reason, 'method call .' + row.operation + '(): ' + error);
      else if (!referenceSemantics || row.referenceProfiles) assert.ok(actual.value, row.operation + ':' + referenceSemantics + ':' + size + ':' + actual.error?.reason);
      else assert.equal(actual.error.reason, '.' + row.operation + '() callback parameters differ');
      count += 1;
    }
  }
  assert.equal(count, 135, 'all45 original false inputs,45 original true inputs and45 separate admitted true inputs remain');
});

test('callback signature decisions preserve original refusal order owners and reduce expected context', () => {
  const method = source => source.slice(source.indexOf('  method(node, env, path, expected) {'), source.indexOf('  arrayLiteral(node, env, path, expected) {'));
  const dependencies = {typeError, unsupported, sameType, typeKey, BOOL, FLOAT, UNIT, array, readArrayFromMapCallback, readArrayFromMapCallbackError, ...rules};
  const load = source => new Function(...Object.keys(dependencies), 'return ({' + method(source) + '}).method;')(...Object.values(dependencies));
  const original = load(readOriginalSources()['check.js']), current = load(readSource('check.js'));
  const receiver = {kind: 'array', element: FLOAT}, span = {start: 3, end: 19};
  for (const operation of operations) for (const size of [0, 1, 3, 4, 5])
    for (const wrongParameter of [false, true]) for (const result of [BOOL, FLOAT, {kind: 'array', element: FLOAT}])
      for (const hasInitial of [false, true]) {
        const expected = {kind: 'array', element: FLOAT};
        const supplied = operation === 'reduce' ? [FLOAT, FLOAT, FLOAT, receiver] : [FLOAT, FLOAT, receiver];
        const callback = {k: 'mock', type: {kind: 'function', params: Array.from({length: size}, (_, index) => wrongParameter ? BOOL : supplied[index] ?? FLOAT), ret: result}};
        const object = {k: 'mock', type: receiver}, initial = {k: 'mock', type: FLOAT};
        const node = {k: 'method', object, op: operation, args: [callback, ...(operation === 'reduce' && hasInitial ? [initial] : [])], span};
        const run = implementation => {
          const trace = [], host = {referenceValue: value => value, expr: (...args) => {trace.push(args); return args[0];}};
          try { return {value: implementation.call(host, node, 'environment', 'path', expected), trace}; }
          catch (error) { return {error: {kind: error.kind, reason: error.reason, span: error.span}, trace}; }
        };
        assert.deepEqual(run(current), run(original), operation);
      }
  for (const operation of operations) for (const kind of ['array', 'float', 'unit', 'function'])
    assert.equal(rules.readArrayCallbackResultError(operation, kind), operation === 'flatMap' && kind !== 'array' ? 'flatMap callback must return an array' : '');
  assert.equal(rules.readArrayCallbackCheckedResultContext('flatMap'), '');
  assert.equal(rules.readArrayCallbackExpectedResultForm('flatMap'), 'array');
});

test('callback source policy retains the entire module without importing runtime semantics', () => {
  const policy = arrayCallbackRulesPolicy(), source = readArrayCallbackSourceModule();
  assert.equal(policy.source, source);
  assert.equal(policy.sourceClosure.source, source);
  assert.equal(policy.sourceSha256, digest(source));
  assert.equal(policy.sourceBytes, Buffer.byteLength(source));
  assert.deepEqual(policy.sourceClosure.removedImports, []);
  assert.equal(policy.sourceClosure.inputs.length, 1);
  assert.equal(policy.sourceClosure.inputs[0].source, source);
  const native = readFileSync(new URL('../../rust/src/translation/array_callback_rules/part_1.rs', import.meta.url), 'utf8');
  const call = '(self::read_array_callback_result_form(operation))';
  const prior = replaceOnce(native, call + '.is_empty()', call + ' == ""');
  const styled = readArrayCallbackLibraryStyles(prior);
  assert.equal(styled.actions.length, 1);
  assert.equal(styled.source, native, 'the actual generated String producer permits exactly the empty comparison style');
  const comments = prior.replace(call + ' == ""', call + ' /* retained context */ == ""');
  assert.deepEqual(readArrayCallbackLibraryStyles(comments), {source: comments, actions: []});
  for (const changed of [prior.replace('String::from("mapped array")', 'String::from("changed")'),
    prior.replace('if operation == "reduce" {\n        2f64', 'if operation == "reduce" {\n        7f64'),
    'type String = OtherString;\n' + prior,
    prior.replace('read_array_callback_result_form(operation: &str)', 'unowned_result_form(operation: &str)')]) {
    assert.notEqual(changed, prior, 'every refusal changes the actual authenticated context');
    assert.deepEqual(readArrayCallbackLibraryStyles(changed), {source: changed, actions: []});
  }
  const literal = '\npub fn retain_literal() -> String { String::from(r#"' + call + ' == \"\""#) }';
  const opaque = readArrayCallbackLibraryStyles(prior + literal);
  assert.equal(opaque.actions.length, 1);
  assert.equal(opaque.source, native + literal, 'comparison spelling inside raw string data stays opaque');
  const recorded = DecoratorSet.fromLino(readFileSync(new URL('../../parity/self-translation/array-callback-rules-decorators.lino', import.meta.url), 'utf8'));
  assert.equal(recorded.decorators.length, 1);
  const owner = recorded.decorators[0];
  assert.equal(owner.actions.at(-1).op, 'replace');
  assert.equal(owner.actions.at(-1).field, 'source');
  assert.equal(owner.actions.at(-1).from.includes(call + ' == ""'), true);
  assert.equal(owner.actions.at(-1).to.includes(call + '.is_empty()'), true);
  const referenceActions = owner.actions.filter(action => action.from?.startsWith('pub fn render_array_callback_reference_expression('));
  assert.equal(referenceActions.length, 2, 'only the new complete renderer needs additional loan and lint styles');
  const historicalActions = owner.actions.filter(action => !referenceActions.includes(action));
  assert.equal(historicalActions.length, 39);
  assert.equal(digest(new DecoratorSet([{...owner, actions: historicalActions.slice(0, -1)}]).toLino()),
    '026555406f9d97fe4f2629698053c20352c64ac0b1f4c89bb3bf45f34024fccd', 'every existing recorded action retains its exact original serialization');

});

test('the generated callback Links fixture retains every original and admitted reference profile', () => {
  const fixture = readProjectData(new URL('../../parity/fixtures/array-callbacks.lino', import.meta.url));
  assert.deepEqual(fixture, buildArrayCallbackFixtures());
  assert.equal(fixture.cases.length, 18);
  const profiles = fixture.cases.flatMap(row => row.profiles);
  assert.equal(profiles.length, 27);
  assert.equal(profiles.filter(profile => profile.checkDiagnostic).length, 9);
  assert.equal(profiles.filter(profile => profile.checkedArchive).length, 18);
  for (const row of fixture.cases.slice(0, 9)) assert.deepEqual(row.profiles.map(profile => profile.referenceSemantics), [false, true]);
});

test('source-free callback archives reject actual missing operands and changed type owners', () => {
  const surface = parseJavaScript(readArrayCallbackFixtureCases()[0].source, {referenceSemantics: false});
  const original = checkProgram(surface);
  encodeTypedProgramLinks(original);
  const edits = [
    ['unknown operation', method => {method.op = 'unrepresented';}],
    ['missing callback', method => {method.args[0] = null;}],
    ['extra callback parameter', method => {method.args[0].type.params.push(BOOL);}],
    ['changed callback result', method => {method.args[0].type.ret = BOOL;}],
    ['changed method result', method => {method.type = FLOAT;}],
  ];
  for (const [name, edit] of edits) {
    const changed = structuredClone(original);
    const declaration = [...changed.declarations.values()].find(owner => owner.k === 'fn' && owner.name === 'run');
    assert.equal(declaration.body.k, 'method');
    edit(declaration.body);
    // Generic encoding retains historical tuple-free admission; this fixture
    // explicitly promises whole-program checked reconstruction after reload.
    for (const format of ['text', 'binary']) assert.throws(() => {
      const packet = encodeTypedProgramLinks(changed);
      const network = format === 'text' ? LinkNetwork.fromLino(packet.network.toLino())
        : decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network));
      recheckLinkedTypedProgram(decodeTypedProgramLinks(network, packet.root), {wholeProgram: true});
    }, name + ': ' + format);
  }
});

test('callback method spans remain inside reference allocations through complete archive rechecks', () => {
  const fixture = readProjectData(new URL('../../parity/fixtures/array-callbacks.lino', import.meta.url));
  let methods = 0;
  const allocations = [];
  for (const row of fixture.cases) for (const profile of row.profiles) {
    if (profile.checkDiagnostic) continue;
    const source = profile.surface.items.find(owner => owner.k === 'fn' && owner.name === 'run');
    assert.equal(source.body.k, 'method');
    assert.ok(source.body.span, 'the original source method owns its complete span');
    const checked = checkProgram(structuredClone(profile.surface));
    const checkSpans = program => {
      const body = program.declarations.get('run').body;
      const method = body.k === 'referenceAllocate' ? body.value : body;
      assert.equal(method.k, 'method');
      assert.deepEqual(method.span, source.body.span, row.id + ': inner checked method span');
      assert.deepEqual(body.span, source.body.span, row.id + ': complete result span');
      return body.k === 'referenceAllocate';
    };
    if (checkSpans(checked)) allocations.push(row.id);
    const packet = encodeTypedProgramLinks(checked);
    for (const network of [LinkNetwork.fromLino(packet.network.toLino()), decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))]) {
      const restored = decodeTypedProgramLinks(network, packet.root);
      assert.deepEqual(restored, checked);
      assert.equal(recheckLinkedTypedProgram(restored, {wholeProgram: true}), restored);
      checkSpans(restored);
    }
    methods += 1;
  }
  assert.equal(methods, 18);
  assert.deepEqual(allocations, ['array-callback-map-reference-value-parameters', 'array-callback-filter-reference-value-parameters'],
    'map and filter retain both span owners; flatMap keeps its already-reference callback result');
});

test('reference flatMap snapshots callback arrays and retains fresh results nested identities and call order', () => {
  const source = readArrayCallbackSourceModule();
  const module = checkProgram(parseJavaScript(source, {referenceSemantics: false}));
  const context = {};
  runInNewContext(emitJavaScript(module).text, context, {timeout: 1000});
  for (const operation of [...operations, 'map', 'unrepresented']) for (const reference of [false, true])
    for (const count of [0, 1, 2, 3]) for (const suffix of ['1', '27']) {
      const args = [operation, 'makeReceiver()', 'makeCallback()', count, suffix, reference];
      const actual = rules.renderArrayCallbackReferenceExpression(...args);
      assert.equal(context.renderArrayCallbackReferenceExpression(...args), actual);
      if (operation !== 'flatMap' || !reference) assert.equal(actual, '');
      else {
        const call = 'ml_callback' + suffix + '(' + ['ml_value' + suffix, '(ml_index' + suffix + ' as f64)', 'ml_values' + suffix + '.clone()'].slice(0, count).join(', ') + ')';
        const snapshot = '{ let ml_callback_values' + suffix + ' = ' + call + '.borrow().clone(); ml_callback_values' + suffix + ' }';
        assert.ok(actual.includes('.flat_map(|(ml_index' + suffix + ', ml_value' + suffix + ')| ' + snapshot + ')'));
        for (const once of ['makeReceiver()', 'makeCallback()', call]) assert.equal(actual.split(once).length, 2);
        assert.ok(actual.includes('std::rc::Rc::new(std::cell::RefCell::new('));
      }
    }
  const programs = [
    '/** @param {number[]} values */ function run(values){return values.flatMap((value,index)=>{console.log(index);return [value,value];});}\nfunction probe(){return run([-1,2,0,3]);}',
    '/** @param {number[][]} values */ function run(values){return values.flatMap((value,index)=>{console.log(index);return [value,value];});}\nfunction probe(){const item=[7];return run([item,item]);}',
    '/** @param {number[]} values */ function run(values){return values.flatMap((value,index)=>{console.log(index);return values;});}\nfunction probe(){return run([4,5]);}',
  ];
  for (let index = 0; index < programs.length; index += 1) {
    const input = programs[index], checked = checkProgram(parseJavaScript(input, {referenceSemantics: true}));
    const method = checked.declarations.get('run').body;
    assert.equal(method.k, 'method');
    assert.equal(method.op, 'flatMap');
    assert.equal(method.args[0].type.ret.kind, 'array');
    assert.equal(method.args[0].type.ret.reference, true);
    assert.deepEqual(method.type, method.args[0].type.ret);
    const rust = emitRust(checked, {arrayConsumption: true});
    assert.match(rust.text, /\.borrow\(\)\.clone\(\); ml_callback_values\d+ \}\)\.collect::<Vec<_>>\(\)\)\)/u);
    const expected = observeArrayCallbackProgram(input);
    assert.deepEqual(observeArrayCallbackProgram(emitJavaScript(checked).text), expected);
    const packet = encodeTypedProgramLinks(checked);
    for (const network of [LinkNetwork.fromLino(packet.network.toLino()), decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))]) {
      const restored = decodeTypedProgramLinks(network, packet.root);
      assert.deepEqual(restored, checked);
      assert.equal(recheckLinkedTypedProgram(restored, {wholeProgram: true}), restored);
      assert.deepEqual(emitRust(restored, {arrayConsumption: true}), rust);
      assert.deepEqual(observeArrayCallbackProgram(emitJavaScript(restored).text), expected);
    }
    for (const text of [input, emitJavaScript(checked).text]) {
      const runtime = {console: {log() {}}};
      runInNewContext(text, runtime, {timeout: 1000});
      const result = runtime.probe();
      if (index === 1) {
        assert.equal(result[0], result[1]);
        assert.equal(result[0], result[2]);
        result[0].push(8);
        assert.deepEqual(Array.from(result, value => Array.from(value)), [[7, 8], [7, 8], [7, 8], [7, 8]]);
      }
      const receiver = index === 1 ? [[7]] : [4, 5];
      const fresh = runtime.run(receiver);
      assert.notEqual(fresh, receiver);
      fresh.push(index === 1 ? [9] : 9);
      assert.equal(receiver.length, index === 1 ? 1 : 2);
    }
  }
});
