// PRIVATE unexecuted draft. Every refusal below remains part of this profile.
import assert from 'node:assert/strict';
import test from 'node:test';
import vm from 'node:vm';
import {readProjectData} from '../scripts/project-data.mjs';
import { tokenize } from '../src/translation/lexer.js';
import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { JAVASCRIPT_ARRAY_BINDING_POLICY } from '../src/translation/array-binding-policy.js';
import { findJavaScriptRecordParameterPattern, acceptJavaScriptRecordParameterBinding,
  planJavaScriptRecordParameters, acceptJavaScriptRecordParameterProfile, acceptJavaScriptStandardBuiltinOwner,
  findJavaScriptRecordParameterStrictDirective } from '../src/translation/javascript-record-parameter-rules.js';
import { readNamedJavaScriptRecordParameters } from '../src/translation/javascript-record-parameters.js';
import { LinkNetwork, createTypedProgramLinks, decodeTypedProgramLinks,
  encodeTypedProgramLinks, encodeLinkNetworkBinary, decodeLinkNetworkBinary } from '../src/index.js';
import { recheckLinkedTypedProgram } from '../src/typed-program-checked-projection.js';
import { readJavaScriptRecordParameterSourceClosure, readJavaScriptRecordParameterDeclaredOwners,
  readOriginalJavaScriptRecordParameterDeclaration } from '../scripts/generate-javascript-record-parameter-rules.mjs';

const reserved = JAVASCRIPT_ARRAY_BINDING_POLICY.reservedBindings;
const facts = source => {
  const tokens = tokenize(source, 'JavaScript').tokens;
  return { tokens, kinds: tokens.map(token => token.kind), values: tokens.map(token => token.value) };
};
const directiveFacts = source => {
  const input = facts(source);
  return { ...input, raws: input.tokens.map(token => source.slice(token.start, token.end)),
    lineBreaks: input.tokens.map((token, index) => /[\n\r\u2028\u2029]/u.test(source.slice(
      index === 0 ? 0 : input.tokens[index - 1].end, token.start))) };
};
const directiveCases = [
  ['"use strict";}', 0], ["'ordinary';'use strict';}", 2],
  ["'ordinary'\n'use strict'\nreturn a;}", 1], ['"use strict"/*\n*/return a;}', 0],
  ['"use strict"\rreturn a;}', 0], ['"use strict"\u2028return a;}', 0], ['"use strict"\u2029return a;}', 0],
  ['"use\\x20strict";return a;}', -1], ['("use strict");return a;}', -1],
  ['"use strict"+"";return a;}', -1], ['"use strict"\n+"";return a;}', -1],
  ['"use strict".length;return a;}', -1], [';"use strict";return a;}', -1],
  ['"ordinary"\n+1;"use strict";return a;}', -1],
  ['"use strict"', -2], ['"ordinary";', -2],
];
const plan = source => { const input = facts(source); return planJavaScriptRecordParameters(input.kinds, input.values, 0, reserved); };
const observe = (source, expression, globals = {}) => vm.runInNewContext(source + '\nJSON.stringify(' + expression + ');', globals, { timeout: 1000 });
const archives = network => [LinkNetwork.fromLino(network.toLino()), decodeLinkNetworkBinary(encodeLinkNetworkBinary(network))];
const nodes = root => {
  const result = [], seen = new WeakSet();
  const visit = value => {
    if (!value || typeof value !== 'object' || seen.has(value)) return;
    seen.add(value);
    if (value.k && value.type) result.push(value);
    for (const child of value instanceof Map ? value.values() : Object.values(value)) visit(child);
  };
  visit(root);
  return result;
};

test('active Node selects the bounded source assertion profile and reports uncovered versions', () => {
  const [major, minor] = process.versions.node.split('.').map(Number);
  assert.ok((major === 22 && minor >= 15) || major === 24,
    'uncovered Node ' + process.versions.node + '; supported >=22.15.0 <23 || >=24 <25');
  assert.equal(acceptJavaScriptRecordParameterProfile('ordinary named function declaration', false, 0, false, false), true);
  for (const input of [['closure', false, 0, false, false], ['ordinary named function declaration', true, 0, false, false],
    ['ordinary named function declaration', false, 1, false, false], ['ordinary named function declaration', false, 0, true, false],
    ['ordinary named function declaration', false, 0, false, true]]) assert.equal(acceptJavaScriptRecordParameterProfile(...input), false);
});

test('actual local module context and captured owners cannot normalize to standard collection constructors or console output', () => {
  for (const name of ['Map', 'Set', 'console']) {
    assert.equal(acceptJavaScriptStandardBuiltinOwner(name, false), true);
    assert.equal(acceptJavaScriptStandardBuiltinOwner(name, true), false);
  }
  assert.equal(acceptJavaScriptStandardBuiltinOwner('Unknown', false), false);
  const sources = [
    'function f({Map}) { return new Map([[1,2]]).size; }\nconsole.log(f({Map:0}));',
    'function g({Set}) { return new Set([1,2]).size; }\nconsole.log(g({Set:0}));',
    'function h({console}) { console.log(7); return 1; }\nh({console:0});',
    'function f(Map){return new Map([[1,2]]).size;}console.log(f(0));',
    'function f(console){console.log(7);return 1;}f(0);',
    'function f(){return new Map([[1,2]]).size;}const Map=0;console.log(f());',
    'function f(){console.log(7);return 1;}const console=0;f();',
    'function f({Map}){const g=()=>new Map([[1,2]]);return g().size;}console.log(f({Map:0}));',
  ];
  for (const source of sources) {
    assert.throws(() => vm.runInNewContext(source, { console: { log() {} } }, { timeout: 1000 }),
      error => error?.name === 'TypeError', source);
    assert.throws(() => parseJavaScript(source), /shadowed standard collection owner|shadowed standard console owner/u, source);
  }
  const deadZone = 'function f(){{console.log(7);const console=0;}return 1;}f();';
  assert.throws(() => vm.runInNewContext(deadZone, {}, { timeout: 1000 }), error => error?.name === 'ReferenceError');
  assert.throws(() => parseJavaScript(deadZone), /shadowed standard console owner/u);
  for (const name of ['Map', 'Set', 'console']) {
    const source = name === 'console' ? 'function f(){console.log(7);return 1;}'
      : 'function f(){return new ' + name + '().size;}';
    const external = { k: 'const', name, type: { kind: 'float' } };
    assert.throws(() => parseJavaScript(source, { externals: [external] }), /shadowed standard/u, name + ' external');
    assert.throws(() => parseJavaScript(source, { literalBindings: new Map([
      [name, { k: 'num', value: '0', type: { kind: 'float' } }],
    ]) }), /shadowed standard/u, name + ' literal');
  }
  const fields = 'function mapField({Map}){return Map;}function setField({Set}){return Set;}'
    + 'function consoleField({console}){return console;}'
    + 'console.log(mapField({Map:2}));console.log(setField({Set:3}));console.log(consoleField({console:4}));';
  const expression = '[mapField({Map:2}),setField({Set:3}),consoleField({console:4})]';
  assert.equal(observe(fields, expression, { console: { log() {} } }), '[2,3,4]');
  const packet = createTypedProgramLinks(fields, 'JavaScript');
  for (const network of archives(packet.network)) {
    const program = recheckLinkedTypedProgram(decodeTypedProgramLinks(network, packet.root), { wholeProgram: true });
    assert.equal(observe(emitJavaScript(program).text, expression, { console: { log() {} } }), '[2,3,4]');
  }
  const standard = 'function standard(){console.log(7);return new Map([[1,2]]).size+new Set([1,1,2]).size;}console.log(standard());';
  const standardPacket = createTypedProgramLinks(standard, 'JavaScript');
  const texts = [standard, ...archives(standardPacket.network).map(network => emitJavaScript(recheckLinkedTypedProgram(
    decodeTypedProgramLinks(network, standardPacket.root), { wholeProgram: true })).text)];
  for (const text of texts) {
    const output = [];
    vm.runInNewContext(text, { console: { log(value) { output.push(String(value)); } } }, { timeout: 1000 });
    assert.deepEqual(output, ['7', '3']);
  }
});

test('non-simple formal lists reject actual Use Strict Directives while raw escapes and continuing expressions retain their meanings', () => {
  for (const [source, expected] of directiveCases) {
    const input = directiveFacts(source);
    assert.equal(findJavaScriptRecordParameterStrictDirective(input.kinds, input.values, input.raws, input.lineBreaks, 0), expected, source);
  }
  const input = directiveFacts('"ordinary"\nreturn a;}');
  for (const start of [-1, NaN, Infinity, 0.5, input.kinds.length]) {
    assert.equal(findJavaScriptRecordParameterStrictDirective(input.kinds, input.values, input.raws, input.lineBreaks, start), -2);
  }
  assert.equal(findJavaScriptRecordParameterStrictDirective(input.kinds, input.values.slice(1), input.raws, input.lineBreaks, 0), -2);
  const sparse = input.lineBreaks.slice();
  delete sparse[1];
  assert.throws(() => findJavaScriptRecordParameterStrictDirective(input.kinds, input.values, input.raws, sparse, 0),
    /missing required Boolean fact column value/u);
  for (const body of ['"use strict";return a;', "'ordinary';'use strict';return a;",
    "'ordinary'\n'use strict'\nreturn a;", '"use strict"/*\n*/return a;']) {
    const source = 'function f({a}){' + body + '}console.log(f({a:2}));';
    assert.throws(() => new vm.Script(source), error => error?.name === 'SyntaxError', source);
    assert.throws(() => parseJavaScript(source), /record parameter strict directive/u, source);
  }
  for (const body of ['("use strict");return a;', '"use\\x20strict";return a;', '"use strict"+"";return a;',
    '"use strict"\n+"";return a;', ';"use strict";return a;', '"use strict".length;return a;']) {
    const source = 'function f({a}){' + body + '}console.log(f({a:2}));';
    assert.doesNotThrow(() => new vm.Script(source), source);
    assert.equal(observe(source, 'f({a:2})', { console: { log() {} } }), '2');
    const packet = createTypedProgramLinks(source, 'JavaScript');
    for (const network of archives(packet.network)) {
      const program = recheckLinkedTypedProgram(decodeTypedProgramLinks(network, packet.root), { wholeProgram: true });
      assert.equal(observe(emitJavaScript(program).text, 'f({a:2})', { console: { log() {} } }), '2');
    }
  }
  const ordinary = 'function ordinary(a){"use strict";return a;}console.log(ordinary(2));';
  assert.doesNotThrow(() => new vm.Script(ordinary));
  assert.equal(observe(emitJavaScript(checkProgram(parseJavaScript(ordinary))).text, 'ordinary(2)', { console: { log() {} } }), '2');
});

test('actual record plans retain ordered source token identities and distinct BoundNames', () => {
  const source = 'before,{left:a,right:b,left:c,},after)';
  const input = facts(source), before = structuredClone(input);
  assert.equal(findJavaScriptRecordParameterPattern(input.kinds, input.values, 0), 2);
  const result = planJavaScriptRecordParameters(input.kinds, input.values, 0, reserved);
  assert.deepEqual(result, [input.tokens.length - 1, 3,
    0, 0, 0, 1, 1, 2, 15, 3, 3, 5, 7, 9, 11, 13, 0, 2, 17]);
  assert.deepEqual(input, before);
  assert.ok(plan('{left,right:alias},tail)')[0] > 0);
  assert.ok(plan('{left},)')[0] > 0);
  for (const name of ['class', 'arguments', 'eval', 'ml_hidden']) {
    assert.equal(acceptJavaScriptRecordParameterBinding('identifier', name, [], reserved), false);
  }
  assert.equal(acceptJavaScriptRecordParameterBinding('identifier', 'same', ['same'], reserved), false);
  assert.equal(acceptJavaScriptRecordParameterBinding('identifier', 'alias', ['field'], reserved), true);
});

test('the selected whole list refuses every default rest nested array computed empty or duplicate binding', () => {
  for (const source of ['{a=1})', '{a}={})', 'before=effect(),{a})', '{a},after=effect())',
    '[a],{b})', '{a:[b]})', '{a:{b}})', '{...rest})', '{[key]:a})', '{"a":a})', '{})',
    '{a},a)', '{a:x,b:x})', '{a:class})', '{a},...rest)', '{a},,b)']) {
    assert.ok(plan(source)[0] < 0, source);
  }
  const input = facts('{a})');
  for (const start of [-1, NaN, Infinity, 0.5, input.kinds.length]) {
    assert.ok(planJavaScriptRecordParameters(input.kinds, input.values, start, reserved)[0] < 0);
  }
  assert.ok(planJavaScriptRecordParameters(input.kinds.slice(1), input.values, 0, reserved)[0] < 0);
  for (const source of ['a=({literal:1}),b)', 'value,other)', 'callback=()=>({a:1}))',
    'expression=/{a,b}/,after)', 'expression=/[,{}()]/,after)', 'text="{a,b}",after)',
    'text=`{a,b}`,after)', 'value=call({a:1,b:2}),after)']) {
    const input = facts(source);
    assert.equal(findJavaScriptRecordParameterPattern(input.kinds, input.values, 0), -1, source);
  }
});

test('every truncated actual token column refuses before any required-column access becomes out of bounds', () => {
  for (const source of ['{a})', '{a:b})', '{a:b,c:d},tail)', 'before,{a:b},after)', '{a,})']) {
    const input = facts(source);
    const close = input.tokens.findIndex(token => token.kind === 'punct' && token.value === ')');
    assert.ok(close >= 0);
    for (let end = 0; end <= close; end += 1) {
      const kinds = input.kinds.slice(0, end), values = input.values.slice(0, end);
      assert.doesNotThrow(() => planJavaScriptRecordParameters(kinds, values, 0, reserved), source + ' tokens=' + end);
      assert.ok(planJavaScriptRecordParameters(kinds, values, 0, reserved)[0] < 0, source + ' tokens=' + end);
    }
    assert.ok(planJavaScriptRecordParameters(input.kinds.slice(0, close + 1), input.values.slice(0, close + 1), 0, reserved)[0] > 0);
  }
  // Sparse host columns are outside the dense primitive-fact transport. They
  // must abort instead of substituting a token or admitting an unknown value.
  const input = facts('{a:b})'), kinds = input.kinds.slice();
  delete kinds[3];
  assert.throws(() => planJavaScriptRecordParameters(kinds, input.values, 0, reserved), /missing required text fact column value/u);
});

test('bounded source transport rebases the second actual function and owns whole-module external and literal-name hygiene', () => {
  const source = '// prefix π😀\nconst record_parameter_value0=9;\nfunction unrelated(value){return value;}\n'
    + 'function selected(before,{left:a},after){return before+a+after;}';
  const input = facts(source), before = structuredClone(input.tokens);
  const name = input.tokens.findIndex(token => token.value === 'selected');
  const open = name + 1, close = input.tokens.findIndex((token, index) => index > open && token.value === ')');
  assert.equal(input.tokens[open].value, '(');
  const cursor = { tokens: input.tokens, index: open + 1, peek(offset = 0) { return this.tokens[this.index + offset]; } };
  const parser = { cursor, recordParameterTokenOwner: input.tokens, recordParameterOrdinal: 0,
    recordParameterNames: new Set([...input.tokens.filter(token => token.kind === 'identifier').map(token => token.value),
      'record_parameter_value1', 'record_parameter_value2']),
    matching(index) { assert.equal(index, open); return close; } };
  const syntax = readNamedJavaScriptRecordParameters(parser, null, false);
  assert.equal(cursor.index, close + 1);
  assert.equal(parser.recordParameterOrdinal, 4);
  assert.deepEqual(syntax.parameters.map(parameter => parameter.name), ['before', 'record_parameter_value3', 'after']);
  assert.deepEqual(syntax.sourceNames, ['before', 'a', 'after']);
  assert.deepEqual(syntax.sourcePlan, [close + 1, 3, 0, 0, open + 1,
    1, 1, open + 3, open + 7, 1, open + 4, open + 6, 0, 2, open + 9]);
  assert.deepEqual(syntax.parameters[1].span, { start: input.tokens[open + 3].start, end: input.tokens[open + 7].end });
  assert.equal(source.slice(syntax.parameters[1].span.start, syntax.parameters[1].span.end), '{left:a}');
  assert.deepEqual(input.tokens, before, 'no fake identifier Token or rewritten source is introduced');
  cursor.index = open + 1;
  parser.recordParameterTokenOwner = input.tokens.slice();
  assert.throws(() => readNamedJavaScriptRecordParameters(parser, null, false), /actual formal interval|token\/name inventory/u);
  assert.equal(cursor.index, open + 1);
  parser.recordParameterTokenOwner = input.tokens;
  for (const ordinal of [-1, NaN, Infinity, 0.5]) {
    parser.recordParameterOrdinal = ordinal;
    assert.throws(() => readNamedJavaScriptRecordParameters(parser, null, false), /token\/name inventory/u);
    assert.equal(cursor.index, open + 1);
  }
  parser.recordParameterOrdinal = Number.MAX_SAFE_INTEGER;
  assert.throws(() => readNamedJavaScriptRecordParameters(parser, null, false), /ordinal exceeds its safe integer range/u);
  assert.equal(cursor.index, open + 1);

  // Exercise the real parser constructor's external/literal reservation too.
  const surface = parseJavaScript(source + '\nconsole.log(selected(1,{left:2},3));', {
    externals: [{ k: 'const', name: 'record_parameter_value1', type: { kind: 'float' } }],
    literalBindings: new Map([['record_parameter_value2', { k: 'num', value: '5', type: { kind: 'float' } }]]),
  });
  assert.equal(surface.items.find(item => item.k === 'fn' && item.name === 'selected').params[1].name, 'record_parameter_value3');
});

test('named declarations bind required fields and aliases without flattening logical arguments', () => {
  const source = 'function selected({left:a,right:b},scale){return (a+b)*scale;}\n'
    + 'function repeated({left:a,left:b}){return a===b;}\n'
    + 'function mixed(before,{left,right},after){return before+left+right+after;}\n'
    + 'console.log(selected({left:2,right:3},4));\n'
    + 'console.log(repeated({left:7}));\nconsole.log(mixed(1,{left:2,right:3},4));';
  const program = checkProgram(parseJavaScript(source));
  assert.equal(program.declarations.get('selected').params.length, 2);
  assert.equal(program.declarations.get('repeated').params.length, 1);
  assert.equal(program.declarations.get('mixed').params.length, 3);
  const expression = '[selected({left:2,right:3},4),selected({left:-2,right:3},2),repeated({left:7}),mixed(1,{left:2,right:3},4)]';
  const expected = '[20,2,true,10]';
  const globals = { console: { log() {} } };
  assert.equal(observe(source, expression, globals), expected);
  const packet = createTypedProgramLinks(source, 'JavaScript');
  for (const network of archives(packet.network)) {
    const restored = recheckLinkedTypedProgram(decodeTypedProgramLinks(network, packet.root), { wholeProgram: true });
    assert.equal(observe(emitJavaScript(restored).text, expression, globals), expected);
    assert.ok(emitRust(restored, { arrayConsumption: true }).text.includes('fn main()'));
  }
});

test('all actual arguments evaluate before ordered callee field reads and each actual receiver is read once', () => {
  const source = 'function selected({left:a,right:b},scale){return (a+b)*scale;}\n'
    + 'console.log(selected({left:2,right:3},4));';
  const code = emitJavaScript(checkProgram(parseJavaScript(source))).text;
  const expression = 'selected(provide(),scale())';
  const run = text => {
    const order = [];
    const provide = () => { order.push('argument:record'); return {
      get left() { order.push('binding:left'); return 2; },
      get right() { order.push('binding:right'); return 3; },
    }; };
    const scale = () => { order.push('argument:scale'); return 4; };
    const value = observe(text, expression, { provide, scale, console: { log() {} } });
    return { value, order };
  };
  const expected = { value: '20', order: ['argument:record', 'argument:scale', 'binding:left', 'binding:right'] };
  assert.deepEqual(run(source), expected);
  assert.deepEqual(run(code), expected);
  const repeated = 'function repeated({left:a,left:b}){return a===b;}console.log(repeated({left:7}));';
  for (const text of [repeated, emitJavaScript(checkProgram(parseJavaScript(repeated))).text]) {
    let reads = 0;
    const provide = () => ({ get left() { reads += 1; return reads; } });
    assert.equal(observe(text, 'repeated(provide())', { provide, console: { log() {} } }), 'false');
    assert.equal(reads, 2, 'each actual repeated property occurrence retains its own ordered read');
  }
  const multiple = 'function ordered({left:a},{right:b}){return a+b;}console.log(ordered({left:2},{right:3}));';
  for (const text of [multiple, emitJavaScript(checkProgram(parseJavaScript(multiple))).text]) {
    const order = [];
    const first = () => { order.push('argument:first'); return { get left() { order.push('binding:left'); return 2; } }; };
    const second = () => { order.push('argument:second'); return { get right() { order.push('binding:right'); return 3; } }; };
    assert.equal(observe(text, 'ordered(first(),second())', { first, second, console: { log() {} } }), '5');
    assert.deepEqual(order, ['argument:first', 'argument:second', 'binding:left', 'binding:right']);
  }
  // Getter traces are JS oracle/transport controls only. This does not admit
  // accessor/proxy records to the Rust represented-data invocation premise.
});

test('hygiene and actual source scopes retain mutable bindings and historical refusal inputs', () => {
  const source = 'function selected({left:record_parameter_value0,right:b}){return record_parameter_value0+b;}\nconsole.log(selected({left:2,right:3}));';
  const code = emitJavaScript(checkProgram(parseJavaScript(source))).text;
  assert.equal(observe(code, 'selected({left:2,right:3})', { console: { log() {} } }), '5');
  const corrections=readProjectData(new URL('../../parity/fixtures/record-parameter-writes.lino',import.meta.url));
  for(const row of corrections.correctedAssertions){
    assert.equal(row.previousAssertion,'checkProgram(parseJavaScript(source)) throws /record parameter|mutable|capture|var declaration/');
    assert.equal(observe(row.source,row.call),row.value);
    const program=checkProgram(parseJavaScript(row.source)),packet=encodeTypedProgramLinks(program);
    assert.equal(observe(emitJavaScript(program).text,row.call),row.value);
    for(const network of archives(packet.network))assert.equal(observe(emitJavaScript(decodeTypedProgramLinks(network,packet.root)).text,row.call),row.value);
  }
  assert.throws(()=>checkProgram(parseJavaScript('function f({a}){var a=2;return a;}')),/var declaration/u);
  for (const source of ['function f({a}){const a=2;return a;}', 'function f({a}){let a=2;return a;}']) {
    assert.throws(() => parseJavaScript(source), /record parameter|redeclar|already|duplicate/u, source);
  }
  const shadowed = 'function selected({a}){if(a>0){const a=7;return a;}else{return a;}}console.log(selected({a:1}));';
  for (const text of [shadowed, emitJavaScript(checkProgram(parseJavaScript(shadowed))).text]) {
    assert.equal(observe(text, '[selected({a:1}),selected({a:-2})]', { console: { log() {} } }), '[7,-2]');
  }
});

test('actual owned callers constrain required fields and reject null missing or unowned invocations even for unused bindings', () => {
  const source = 'function selected({left:a,right:b}){return a+b;}\n'
    + 'function forward(value){return selected(value);}\nconsole.log(forward({right:3,left:2}));';
  const packet = createTypedProgramLinks(source, 'JavaScript');
  for (const network of archives(packet.network)) {
    const program = recheckLinkedTypedProgram(decodeTypedProgramLinks(network, packet.root), { wholeProgram: true });
    assert.equal(program.declarations.get('selected').params[0].type.kind, 'record');
    assert.deepEqual(program.declarations.get('selected').params[0].type.fields.map(field => field.name), ['left', 'right']);
    assert.equal(observe(emitJavaScript(program).text, 'forward({left:2,right:3})', { console: { log() {} } }), '5');
  }
  for (const argument of ['null', 'undefined', '', '{}', '{right:3}', '3', 'true', '"text"', '[1,2]']) {
    for (const body of ['return a;', 'return 1;']) {
      const source = 'function selected({left:a}){' + body + '}console.log(selected(' + argument + '));';
      assert.throws(() => checkProgram(parseJavaScript(source)), undefined, source);
    }
  }
  for (const source of [
    'function selected({left:a}){return a;}console.log(selected(external()));',
    'function selected({left:a}){return a;}console.log(selected({get left(){return 2;}}));',
    'function selected({left:a}){return a;}console.log(selected(Object.create({left:2})));',
    'function selected({left:a}){return a;}console.log(selected(new Proxy({left:2},{})));',
    'function selected({constructor:owner}){return owner;}console.log(selected({}));',
  ]) assert.throws(() => checkProgram(parseJavaScript(source)), undefined, source);
  // No fabricated origin flag is used: current source calls, real parameter
  // types, every field operation and the whole linked program are checked.
});

test('existing typed field laws preserve present null values String or Array length and Map or Set size', () => {
  for (const [source, expression, expected] of [
    ['function selected({left:a}){return a===null;}console.log(selected({left:null}));', 'selected({left:null})', 'true'],
    ['function selected({length}){return length;}console.log(selected("A😀"));', 'selected("A😀")', '3'],
    ['function selected({length}){return length;}console.log(selected([1,2,3]));', 'selected([1,2,3])', '3'],
    ['function selected({size}){return size;}console.log(selected(new Map([["second",2],["first",1],["second",3]])));', 'selected(new Map([["second",2],["first",1],["second",3]]))', '2'],
    ['function selected({size}){return size;}console.log(selected(new Set([1,1,2])));', 'selected(new Set([1,1,2]))', '2'],
  ]) {
    assert.equal(observe(source, expression, { console: { log() {} } }), expected);
    const packet = createTypedProgramLinks(source, 'JavaScript');
    for (const network of archives(packet.network)) {
      const program = recheckLinkedTypedProgram(decodeTypedProgramLinks(network, packet.root), { wholeProgram: true });
      assert.equal(observe(emitJavaScript(program).text, expression, { console: { log() {} } }), expected);
      assert.ok(emitRust(program, { arrayConsumption: true }).text.includes('fn main()'));
    }
  }
  // These are already checked field meanings; a pattern alone is not a proof
  // that its receiver has record type, getters or arbitrary JS object coercion.
});

test('ordinary callable parameter contexts retain original syntax and concrete-call coverage separately from the named profile',()=>{
 const legacy=readProjectData(new URL('../../parity/fixtures/record-callable-parameters.lino',import.meta.url)).legacy;
 for(const row of legacy){
  assert.doesNotThrow(()=>parseJavaScript(row.source),row.source);
  const source=row.source+'console.log('+row.call+');';
  assert.doesNotThrow(()=>checkProgram(parseJavaScript(source)),row.source);
  assert.equal(observe(source,row.call,{console:{log(){}}}),JSON.stringify(Number(row.value)));
 }
});
test('async and documentation profiles retain their separate named-adapter refusal boundaries', () => {
  for (const source of ['async function f({a}){return a;}',
    '/** @param {{a:number}} options */function f({a}){return a;}']) {
    assert.throws(() => parseJavaScript(source), /destructured parameter|record parameter/u, source);
  }
  for (const [old, expected] of [
    ['function f(value=3){return value;}console.log(f());console.log(f(7));', [3,7]],
    ['function f(value=({literal:1})){return value.literal;}console.log(f());console.log(f({literal:7}));', [1,7]],
  ]) {
    // Native source logs numbers; the target emits the same console text via
    // its explicit String encoding. Preserve the raw numeric control, and
    // compare actual console text for both programs on the retained inputs.
    for (const [index, code] of [old, emitJavaScript(checkProgram(parseJavaScript(old))).text].entries()) {
      const output = [];
      vm.runInNewContext(code, { console: { log(value) { output.push(value); } } }, { timeout: 1000 });
      if (index === 0) assert.deepEqual(output, expected);
      assert.deepEqual(output.map(String), expected.map(String));
    }
  }
  for (const source of ['function ordinary(value=/{a,b}/){return 1;}', 'function ordinary(value=/[,{}()]/){return 1;}']) {
    assert.throws(() => parseJavaScript(source), /regular expression/u, source);
  }
});

test('the optional scalar-loop route cannot bypass actual object pattern entry bindings', () => {
  const source = 'function selected({left:a}){for(;;){return a;}}console.log(selected({left:2}));';
  const selected = checkProgram(parseJavaScript(source, { scalarLoopModel: 'closed primitive scalar loop 1' }));
  assert.equal(selected.declarations.get('selected').sourceLoop, undefined);
  assert.ok(nodes(selected.declarations.get('selected').body).some(node => node.k === 'recordField' && node.field === 'left'));
  const packet = encodeTypedProgramLinks(selected);
  for (const network of archives(packet.network)) {
    const restored = recheckLinkedTypedProgram(decodeTypedProgramLinks(network, packet.root), { wholeProgram: true });
    assert.equal(observe(emitJavaScript(restored).text, 'selected({left:2})', { console: { log() {} } }), '2');
  }
});

test('ordinary source CST and source-free semantic rechecks are separate controls', () => {
  const source = '// original source π\nfunction f({left:a,right:b}) { return a+b; }\nconsole.log(f({left:2,right:3}));\n';
  const concrete = LinkNetwork.parse(source, 'JavaScript');
  assert.ok(concrete.links().every(link => !link.metadata().flags.hasError && !link.metadata().flags.isMissing));
  assert.deepEqual(Buffer.from(concrete.reconstructText()), Buffer.from(source));
  for (const network of archives(concrete)) assert.deepEqual(Buffer.from(network.reconstructText()), Buffer.from(source));
  const packet = createTypedProgramLinks(source, 'JavaScript');
  for (const network of archives(packet.network)) {
    const program = recheckLinkedTypedProgram(decodeTypedProgramLinks(network, packet.root), { wholeProgram: true });
    assert.equal(observe(emitJavaScript(program).text, 'f({left:2,right:3})', { console: { log() {} } }), '5');
  }
});

test('coherent source-free field and binder edits recheck and execute while malformed ownership and projection edits refuse', () => {
  const source = 'function selected({left:a,right:b}){return a+b;}console.log(selected({left:2,right:3}));';
  const packet = createTypedProgramLinks(source, 'JavaScript');
  for (const first of archives(packet.network)) {
    const program = decodeTypedProgramLinks(first, packet.root);
    const selected = program.declarations.get('selected');
    const originalName = selected.params[0].name, changedName = 'renamed_record_argument';
    const variables = nodes(selected.body).filter(node => node.k === 'var' && node.name === originalName);
    assert.equal(variables.length, 2);
    selected.params[0].name = changedName;
    variables.forEach(node => { node.name = changedName; });
    const fields = nodes(selected.body).filter(node => node.k === 'recordField');
    assert.deepEqual(fields.map(node => [node.field, node.index]), [['left', 0], ['right', 1]]);
    fields[0].field = 'right';
    fields[0].index = 1;
    assert.strictEqual(recheckLinkedTypedProgram(program, { wholeProgram: true }), program);
    const edited = encodeTypedProgramLinks(program);
    for (const second of archives(edited.network)) {
      const restored = recheckLinkedTypedProgram(decodeTypedProgramLinks(second, edited.root), { wholeProgram: true });
      assert.equal(restored.declarations.get('selected').params[0].name, changedName);
      assert.equal(observe(emitJavaScript(restored).text, 'selected({left:2,right:3})', { console: { log() {} } }), '6');
      assert.ok(emitRust(restored, { arrayConsumption: true }).text.includes('fn main()'));
    }
  }
  for (const edit of [
    (program, field) => { field.field = 'absent'; },
    (program, field) => { field.index = 1; },
    (program, field) => { field.object.name = 'unowned_receiver'; },
    (program, field) => { field.type = { kind: 'bool' }; },
  ]) {
    const program = decodeTypedProgramLinks(LinkNetwork.fromLino(packet.network.toLino()), packet.root);
    const field = nodes(program.declarations.get('selected').body).find(node => node.k === 'recordField');
    assert.ok(field);
    edit(program, field);
    assert.throws(() => recheckLinkedTypedProgram(program, { wholeProgram: true }));
    assert.throws(() => encodeTypedProgramLinks(program));
  }
});

test('all complete shared decisions translate and replay every selected control through both source-free codecs before native generation', () => {
  const closure = readJavaScriptRecordParameterSourceClosure();
  const surface = parseJavaScript(closure.source, { referenceSemantics: false });
  assert.deepEqual(surface.items.filter(owner => owner.k === 'fn' && !owner.generated).map(owner => owner.name).sort(),
    readJavaScriptRecordParameterDeclaredOwners(closure.source).sort());
  const checked = checkProgram(surface), saved = encodeTypedProgramLinks(checked);
  const names = ['findJavaScriptRecordParameterPattern', 'acceptJavaScriptRecordParameterBinding',
    'planJavaScriptRecordParameters', 'acceptJavaScriptRecordParameterProfile',
    'acceptJavaScriptStandardBuiltinOwner', 'findJavaScriptRecordParameterStrictDirective'];
  const expectedRust = emitRust(checked, { arrayConsumption: true }).text;
  for (const network of archives(saved.network)) {
    const program = recheckLinkedTypedProgram(decodeTypedProgramLinks(network, saved.root), { wholeProgram: true });
    assert.equal(emitRust(program, { arrayConsumption: true }).text, expectedRust);
    const context = vm.createContext({});
    const api = vm.runInContext(emitJavaScript(program).text + '\n({' + names.join(',') + '});', context, { timeout: 1000 });
    for (const source of ['before,{left:a,right:b,left:c,},after)', '{left,right:alias},tail)', '{left},)',
      '{a=1})', '{a}={})', 'before=effect(),{a})', '{a},after=effect())', '[a],{b})', '{a:[b]})',
      '{a:{b}})', '{...rest})', '{[key]:a})', '{"a":a})', '{})', '{a},a)', '{a:x,b:x})', '{a:class})',
      '{a},...rest)', '{a},,b)', 'expression=/{a,b}/,after)', 'expression=/[,{}()]/,after)',
      'text="{a,b}",after)', 'text=`{a,b}`,after)', 'value=call({a:1,b:2}),after)',
      'a=({literal:1}),b)', 'value,other)', 'callback=()=>({a:1}))']) {
      const input = facts(source);
      for (let end = 0; end <= input.tokens.length; end += 1) {
        const kinds = input.kinds.slice(0, end), values = input.values.slice(0, end);
        assert.deepEqual(Array.from(api.planJavaScriptRecordParameters(kinds, values, 0, reserved)),
          planJavaScriptRecordParameters(kinds, values, 0, reserved), source + ' tokens=' + end);
        assert.equal(api.findJavaScriptRecordParameterPattern(kinds, values, 0),
          findJavaScriptRecordParameterPattern(kinds, values, 0), source + ' tokens=' + end);
      }
    }
    const input = facts('{a})');
    for (const start of [-1, NaN, Infinity, 0.5, input.kinds.length]) {
      assert.deepEqual(Array.from(api.planJavaScriptRecordParameters(input.kinds, input.values, start, reserved)),
        planJavaScriptRecordParameters(input.kinds, input.values, start, reserved));
      assert.equal(api.findJavaScriptRecordParameterPattern(input.kinds, input.values, start), -1);
    }
    assert.deepEqual(Array.from(api.planJavaScriptRecordParameters(input.kinds.slice(1), input.values, 0, reserved)),
      planJavaScriptRecordParameters(input.kinds.slice(1), input.values, 0, reserved));
    assert.equal(api.acceptJavaScriptRecordParameterBinding('identifier', 'alias', ['field'], reserved), true);
    for (const name of ['class', 'arguments', 'eval', 'ml_hidden', 'same']) {
      assert.equal(api.acceptJavaScriptRecordParameterBinding('identifier', name, ['same'], reserved), false);
    }
    assert.equal(api.acceptJavaScriptRecordParameterProfile('ordinary named function declaration', false, 0, false, false), true);
    for (const input of [['closure', false, 0, false, false], ['ordinary named function declaration', true, 0, false, false],
      ['ordinary named function declaration', false, 1, false, false], ['ordinary named function declaration', false, 0, true, false],
      ['ordinary named function declaration', false, 0, false, true]]) {
      assert.equal(api.acceptJavaScriptRecordParameterProfile(...input), false);
    }
    for (const name of ['Map', 'Set', 'console', 'Unknown']) for (const shadowed of [false, true]) {
      assert.equal(api.acceptJavaScriptStandardBuiltinOwner(name, shadowed), acceptJavaScriptStandardBuiltinOwner(name, shadowed));
    }
    for (const [source, expected] of directiveCases) {
      const input = directiveFacts(source);
      assert.equal(api.findJavaScriptRecordParameterStrictDirective(input.kinds, input.values, input.raws, input.lineBreaks, 0), expected, source);
      for (let end = 0; end <= input.tokens.length; end += 1) {
        const args = [input.kinds.slice(0, end), input.values.slice(0, end), input.raws.slice(0, end), input.lineBreaks.slice(0, end), 0];
        assert.equal(api.findJavaScriptRecordParameterStrictDirective(...args), findJavaScriptRecordParameterStrictDirective(...args),
          source + ' tokens=' + end);
      }
    }
    const directive = directiveFacts('"ordinary"\nreturn a;}');
    for (const start of [-1, NaN, Infinity, 0.5, directive.kinds.length]) {
      assert.equal(api.findJavaScriptRecordParameterStrictDirective(directive.kinds, directive.values, directive.raws, directive.lineBreaks, start), -2);
    }
    assert.equal(api.findJavaScriptRecordParameterStrictDirective(directive.kinds, directive.values.slice(1), directive.raws, directive.lineBreaks, 0), -2);
    const sparse = directive.lineBreaks.slice();
    delete sparse[1];
    assert.throws(() => api.findJavaScriptRecordParameterStrictDirective(directive.kinds, directive.values, directive.raws, sparse, 0),
      /missing required Boolean fact column value/u);
  }
});

test('actual complete project helper declarations retain their original owners and accepted headers without claiming whole-body translation', () => {
  for (const [path, name] of [
    ['js/src/grammar-interchange.js', 'formatsCommand'], ['js/src/decorators.js', 'renderDecorator'],
    ['js/src/grammar-emitters/pest.js', 'emitRepeat'], ['js/src/grammar-emitters/tree-sitter-json.js', 'emitRepeat'],
    ['js/src/program-binding-extents.js', 'resolveRustPath'],
  ]) {
    const owner = readOriginalJavaScriptRecordParameterDeclaration(path, name);
    const concrete = LinkNetwork.parse(owner.declaration, 'JavaScript');
    assert.ok(concrete.links().every(link => !link.metadata().flags.hasError && !link.metadata().flags.isMissing));
    for (const network of archives(concrete)) assert.deepEqual(Buffer.from(network.reconstructText()), Buffer.from(owner.declaration));
    assert.ok(owner.context.includes(owner.declaration));
    assert.equal(owner.context.slice(owner.range.start, owner.range.end), owner.declaration);
    const input = facts(owner.declaration), at = input.tokens.findIndex(token => token.value === 'function');
    assert.equal(input.tokens[at + 1].value, name);
    assert.equal(input.tokens[at + 2].value, '(');
    assert.ok(findJavaScriptRecordParameterPattern(input.kinds, input.values, at + 3) >= 0);
    assert.ok(planJavaScriptRecordParameters(input.kinds, input.values, at + 3, reserved)[0] > 0);
    // These are actual original header/CST controls, not whole-body success.
  }
});

// Prepared owner controls; no execution during the whole requirements draft.
test('BigInt and Boolean owners require actual unshadowed standard bindings',()=>{
 for(const name of ['BigInt','Boolean']){
  assert.equal(acceptJavaScriptStandardBuiltinOwner(name,false),true);
  assert.equal(acceptJavaScriptStandardBuiltinOwner(name,true),false);
 }
});
