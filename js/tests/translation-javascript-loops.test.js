import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { test } from 'node:test';
import { runInNewContext } from 'node:vm';
import { format } from 'node:util';

import { translateProgram } from '../src/program-translation.js';
import { checkProgram } from '../src/translation/check.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { parseJavaScript } from '../src/translation/javascript.js';
import { readJavaScriptTemporalBindingRead } from '../src/translation/frontend-rules.js';
import { isLiteralBinding } from '../src/translation/javascript-bindings.js';
import { acceptJavaScriptLiteralBinding } from '../src/translation/javascript-scoped-binding-emission-rules.js';
import { LinkNetwork } from '../src/network.js';
import { decodeLinkNetworkBinary, encodeLinkNetworkBinary } from '../src/link-network-binary.js';
import { decodeTypedProgramLinks, encodeTypedProgramLinks } from '../src/typed-program-links.js';

const SUM = `function sum(n) {
  let total = 0n;
  for (let i = 1n; i <= n; i++) {
    total += i;
  }
  return total;
}
console.log(sum(10n));
`;

test('a loop is a generated function of the variables it uses', () => {
  const { items } = parseJavaScript(SUM);
  const loop = items.find((item) => item.name === 'ml_sum_loop1');
  assert.equal(loop.k, 'fn');
  assert.equal(loop.generated, true);
  assert.deepEqual(loop.params.map((param) => param.name), ['n', 'total', 'i']);
  // The one variable the loop assigns and the statements after it read is its result: no result type is needed.
  assert.equal(items.some((item) => item.k === 'data'), false);
});

test('a loop that returns, or leaves live variables it assigns, returns a generated data type', () => {
  const { items } = parseJavaScript(`function divisor(n) {
  for (let d = 2n; d * d <= n; d++) {
    if (n % d === 0n) return d;
  }
  return n;
}
function gcd(a, b) {
  while (b !== 0n) {
    const r = a % b;
    a = b;
    b = r;
  }
  return a;
}
console.log(divisor(91n) + gcd(1071n, 462n));
`);
  const result = items.find((item) => item.name === 'ml_divisor_loop1_result');
  assert.deepEqual(result.ctors.map((ctor) => ctor.name), ['ml_divisor_loop1_done', 'ml_divisor_loop1_return']);
  // gcd's loop assigns a and b, but only a is read after it.
  assert.equal(items.some((item) => item.name === 'ml_gcd_loop2_result'), false);
});

test('mutable bindings JavaScript could not run as translated are rejected', () => {
  const cases = [
    ['function f(n) { var x = n; return x; }', 'var declaration: var bindings are hoisted to the function and shared by its blocks; use let or const at 16..19'],
    ['function f(n) { const x = n; x = 1n; return x; }', 'assignment of constant x: assigning a const binding throws a TypeError; declare it with let at 29..33'],
    ['function f(n) { let x; x = n; return x; }', 'let x without a value: an uninitialised let holds undefined, which is not a portable value; give it an initial value at 16..20'],
    ['function f(n) { let [a] = n; return a; }', 'let destructuring: declare each binding with its own let at 16..20'],
    ['function f(n) { x = 1n; let x = n; return x; }', 'assignment of x before its declaration: the binding is in its temporal dead zone, where assigning it throws a ReferenceError at 16..20'],
    ['function f(n) { break; }', 'break outside a loop or switch at 16..21'],
    ['function f(n) { outer: while (true) { break outer; } return n; }', 'label outer: labels are outside the portable core; a break or continue applies to the innermost loop at 16..21'],
    ['function f(n) { for (x of n) {} return n; }', 'for…of without const or let: declare the loop variable with const or let, so each iteration has its own at 16..21'],
    ['function f(n) { for (const x in n) {} return n; }', 'for…in loop: iteration over the keys of objects is outside the portable core; count with for (let i = …; …; …) at 16..21'],
    ['function f(n) { let ml_x = n; return ml_x; }', "reserved identifier: ml_x uses the translator's reserved ml_ prefix at 20..24"],
    ['function f(n) { while (n > 0n) { if (n === 3n) return n; n--; } }', 'the function returns a bigint on one path and finishes without a return value, returning undefined, on another; return a value on every path at 54..55'],
    ['function f(n) { while (true) { return n; } return 0n; }', 'unreachable statement: statements after return, throw, break, continue or a complete if are never executed at 43..54'],
    ['function f(n) { n <<= 1n; return n; }', '<<= assignment: the portable compound assignments are +=, -=, *=, /=, %=, &&= and ||= at 16..22'],
    ['function f(n) { for (let i = 0n; i < n; i++) { g = i; } return n; }', 'assignment of g: only local variables declared with let, and parameters, are assignable at 47..51'],
  ];
  for (const [source, message] of cases) {
    if (source === 'function f(n) { let ml_x = n; return ml_x; }' || source === 'function f(n) { let x; x = n; return x; }' || source === 'function f(n) { while (n > 0n) { if (n === 3n) return n; n--; } }') {
      const observed = (text) => {
        const output = [];
        runInNewContext(text, { console: { log: (...values) => output.push(format(...values)) } });
        return output;
      };
      const runnable = `${source}\n${source.includes('while') ? 'console.log(f(4n)); console.log(f(0n));' : 'console.log(f(17));'}`;
      assert.deepEqual(observed(emitJavaScript(checkProgram(parseJavaScript(runnable))).text), observed(runnable));
      assert.equal(translateProgram(runnable, 'JavaScript', 'Rust').diagnostic, null);
      continue;
    }
    assert.throws(() => parseJavaScript(source), (error) => error.message === message, source);
  }
  for (const source of [
    'function read(){let first=NaN,NaN=1;return first;}console.log(read());',
    'function read(){let first=Infinity,Infinity=1;return first;}console.log(read());',
    'function read(){const result=NaN;let first=0,NaN=1;return result;}console.log(read());',
    'function read(){let first=undefined,undefined=1;return first;}console.log(read());',
    'function read(){const result=NaN\nlet first=0,NaN=1;return result;}console.log(read());',
    'function read(NaN){{const result=NaN;let first=0,NaN=1;return result;}}console.log(read(9));',
    'function read(){for(let NaN of [NaN]){console.log(NaN);}return 1;}console.log(read());',
    'function read(){for(const [NaN] of [[NaN]]){console.log(NaN);}return 1;}console.log(read());',
    'function read(){for(let first=NaN,NaN=1;first<2;first++){}return 1;}console.log(read());',
    'function read(NaN){{NaN=2;let first=0,NaN=1;return first;}}console.log(read(9));',
  ]) {
    assert.throws(() => runInNewContext(source), error => error.name === 'ReferenceError', source);
    assert.throws(() => parseJavaScript(source), error => error.kind === 'unsupported'
      && /temporal dead zone/u.test(error.reason), source);
  }
  // Parsing a closure is not its execution. A later capture needs an actual
  // invocation-order proof, even where this particular original invokes late.
  const deferred = 'function read(){let first=()=>NaN,NaN=1;return first();}console.log(read());';
  const output = [];
  runInNewContext(deferred, { console: { log: value => output.push(value) } });
  assert.deepEqual(output, [1]);
  assert.throws(() => parseJavaScript(deferred), error => error.kind === 'unsupported'
    && /invocation-order proof/u.test(error.reason));
  const eager = 'function read(){const fn=()=>{let first=NaN,NaN=1;return first;};return fn();}console.log(read());';
  assert.throws(() => runInNewContext(eager), error => error.name === 'ReferenceError');
  assert.throws(() => parseJavaScript(eager), error => error.kind === 'unsupported'
    && /temporal dead zone/u.test(error.reason) && !/invocation-order proof/u.test(error.reason));
  for (const source of [
    'function read(){return NaN;}let first=0,NaN=1;console.log(read());',
    'let first=0,NaN=1;function read(){return NaN;}console.log(read());',
    'function read(value=NaN){return value;}let first=0,NaN=1;console.log(read());',
  ]) {
    const original = [];
    // A strict lexical block models the module's lexical binding environment;
    // a Script's immutable global NaN property must not replace that oracle.
    runInNewContext(`'use strict';{${source}}`, { console: { log: value => original.push(value) } });
    assert.deepEqual(original, [1], source);
    assert.throws(() => parseJavaScript(source), error => error.kind === 'unsupported'
      && /mutable module binding requires module-capture representation/u.test(error.reason), source);
  }
  for (const source of [
    'function read(){let first=0\nInfinity,NaN;return first;}',
    'function read(){let first={let:2};first.let\nNaN,Infinity;return first.let;}',
  ]) assert.throws(() => parseJavaScript(source), source);
});

test('actual lexical owners preserve initialized bindings, shadows and initializer order through Links', () => {
  const observed = (source) => {
    const output = [];
    runInNewContext(source, { console: { log: (...values) => output.push(format(...values)) } }, { timeout: 1000 });
    return output;
  };
  for (const [source, expected] of [
    ['function read(){let NaN=1,first=NaN;return first;}console.log(read());', ['1']],
    ['function read(){let first=1\nlet second=first+1;return second;}console.log(read());', ['2']],
    ['function read(){let first=1\n+2,second=first+1;return second;}console.log(read());', ['4']],
    ['function read(){let first={let:2},second=first.let;return second;}console.log(read());', ['2']],
    ['function read(){let first=[1,2],second={x:3,y:4};return first.length+second.x;}console.log(read());', ['5']],
    ['function read(){const [NaN,other]=[1,2];let first=NaN,Infinity=other;return first+Infinity;}console.log(read());', ['3']],
    ['function read(){let total=0;{const NaN=3;total=NaN;}let first=0,NaN=1;return total+NaN;}console.log(read());', ['4']],
    ['function read(){let reader=(NaN)=>NaN,NaN=1;return reader(3);}console.log(read());', ['3']],
    ['function read(){let NaN=1,reader=()=>NaN;return reader();}console.log(read());', ['1']],
    ['function read(){let total=0;for(let NaN=0;NaN<2;NaN++){total+=NaN;}let first=0,NaN=3;return total+NaN;}console.log(read());', ['4']],
    ['function read(){let total=0;for(const NaN of [1,2]){total+=NaN;}let first=0,NaN=3;return total+NaN;}console.log(read());', ['6']],
    ['function read(){let total=0;for(const [x] of [[1],[2]]){total+=x;}return total;}console.log(read());', ['3']],
    ['function pair(a,b){console.log(a);return b;}function read(){let first=pair(1,2),second=pair(first,3);return second;}console.log(read());', ['1', '2', '3']],
  ]) {
    assert.deepEqual(observed(source), expected, 'the unchanged original source is the execution oracle');
    assert.deepEqual(observed(emitJavaScript(checkProgram(parseJavaScript(source))).text), expected, source);
  }
  // Nearest non-let shadows stop lookup; otherwise the outer initializer end
  // still constrains an actual earlier read in a child scope.
  assert.deepEqual(readJavaScriptTemporalBindingRead([-1, 0], [0, 1, 0, 1, 1, 0], ['NaN', 'NaN'], [30, -1], [1], ['NaN'], [10]), []);
  assert.deepEqual(readJavaScriptTemporalBindingRead([-1, 0], [0, 1, 0, 1, 0, 0], ['NaN'], [30], [1], ['NaN'], [10]), [0, 0]);
  assert.deepEqual(readJavaScriptTemporalBindingRead([-1, 0], [0, 1, 0, 1, 0, 0], ['NaN'], [30], [1], ['NaN'], [30]), []);
  assert.deepEqual(readJavaScriptTemporalBindingRead([-1, 0, 1], [0, 0, 0, 0, 0, 1, 0, 1, 0], ['NaN'], [30], [2], ['NaN'], [10]), [0, 0]);
  assert.deepEqual(readJavaScriptTemporalBindingRead([-1, 0, 1], [0, 1, 0, 1, 0, 1, 1, 0, 0], ['NaN'], [30], [2], ['NaN'], [10]), [0, 1]);
  assert.deepEqual(readJavaScriptTemporalBindingRead([-1, 0], [0, 1, 0, 1, 0, 2], ['NaN'], [30], [1], ['NaN'], [10]), [0, 2]);
  assert.deepEqual(readJavaScriptTemporalBindingRead([-1, 0], [0, 1, 0, 1, 0, 2], ['NaN'], [30], [1], ['NaN'], [40]), [0, 2]);
  assert.deepEqual(readJavaScriptTemporalBindingRead([-1, 0], [0, 1, 0, 1, 1, 2], ['NaN', 'NaN'], [30, -1], [1], ['NaN'], [40]), []);
  for (const [source, expected] of [
    ['function read(NaN){return NaN;}let first=0,NaN=1;console.log(read(2));', ['2']],
    ['const NaN=3;function read(){return NaN;}console.log(read());', ['3']],
  ]) {
    assert.deepEqual(observed(`'use strict';{${source}}`), expected);
    const emitted = emitJavaScript(checkProgram(parseJavaScript(source))).text;
    assert.deepEqual(observed(`'use strict';{${emitted}}`), expected, 'nearest parameter and existing constant ownership remain unchanged');
  }
  const source = 'function read(){let NaN=1,first=NaN;return first;}console.log(read()===1);';
  const checked = checkProgram(parseJavaScript(source));
  const archive = encodeTypedProgramLinks(checked);
  const rust = emitRust(checked).text;
  for (const program of [checked,
    decodeTypedProgramLinks(LinkNetwork.fromLino(archive.network.toLino()), archive.root),
    decodeTypedProgramLinks(decodeLinkNetworkBinary(encodeLinkNetworkBinary(archive.network)), archive.root),
  ]) {
    assert.deepEqual(observed(emitJavaScript(program).text), observed(source));
    assert.equal(emitRust(program).text, rust, 'text and binary reload retain the actual initialized binding');
  }
});

test('literal bindings share complete recursive admission without admitting spreads, effects or mutable module captures', () => {
  const primitiveKinds = new Set(['num', 'bool', 'str', 'none']);
  for (const kind of [...primitiveKinds, 'record', 'array', 'name', 'call', 'unit', 'unknown']) {
    for (const hasSpread of [false, true]) for (const allChildrenAdmitted of [false, true]) {
      assert.equal(acceptJavaScriptLiteralBinding(kind, hasSpread, allChildrenAdmitted),
        primitiveKinds.has(kind) || kind === 'record' && !hasSpread && allChildrenAdmitted,
        `${kind}:${hasSpread}:${allChildrenAdmitted}`);
    }
  }
  for (const value of [
    {k: 'num', value: '3'}, {k: 'num', value: '2', suffix: 'n'}, {k: 'bool', value: false},
    {k: 'str', value: ''}, {k: 'none', absence: 'null'}, {k: 'none', absence: 'undefined'},
    {k: 'record', entries: []},
    {k: 'record', entries: [{name: 'nested', value: {k: 'record', entries: [
      {name: 'leaf', value: {k: 'record', entries: [{name: 'value', value: {k: 'none', absence: 'null'}}]}},
    ]}}]},
  ]) assert.equal(isLiteralBinding(value), true);
  for (const value of [
    {k: 'array', items: [{value: {k: 'num', value: '3'}}]}, {k: 'name', path: ['value']},
    {k: 'call', fn: 'create', args: []},
    {k: 'record', entries: [{spread: true, value: {k: 'record', entries: []}}]},
    {k: 'record', entries: [{name: 'nested', value: {k: 'record', entries: [{name: 'leaf', value: {k: 'call', fn: 'create', args: []}}]}}]},
    {k: 'record', entries: [{spread: true, get value() { throw new Error('a spread must stop before its value'); }}]},
    {k: 'record', entries: [{name: 'first', value: {k: 'call', fn: 'create', args: []}},
      {get spread() { throw new Error('a rejected child must stop before the next entry'); }}]},
  ]) assert.equal(isLiteralBinding(value), false);

  const observed = (source) => {
    const output = [];
    runInNewContext(`'use strict';{${source}}`, {console: {log: (...values) => output.push(format(...values))}}, {timeout: 1000});
    return output;
  };
  for (const [source, expected] of [
    ['const NaN=3;function read(){return NaN;}console.log(read());', ['3']],
    ['const NaN=null;function read(){return NaN===null;}console.log(read());', ['true']],
    ['const missing=undefined;function read(){return missing===undefined;}console.log(read());', ['true']],
    ['const Infinity=false;function read(){return Infinity;}console.log(read());', ['false']],
    ['const NaN="word";function read(){return NaN;}console.log(read());', ['word']],
    ['const NaN={value:3};function read(){return NaN.value;}console.log(read());', ['3']],
    ['const NaN={outer:{inner:{value:7}}};function read(){return NaN.outer.inner.value;}console.log(read());', ['7']],
    ['const NaN=3;function read(NaN){return NaN;}console.log(read(9));', ['9']],
    ['function first(){const NaN=3;return NaN;}function second(){return NaN;}console.log(first(),Number.isNaN(second()));', ['3 true']],
  ]) {
    assert.deepEqual(observed(source), expected, 'the unchanged original source independently specifies the result');
    const checked = checkProgram(parseJavaScript(source));
    const archive = encodeTypedProgramLinks(checked);
    const rust = emitRust(checked).text;
    for (const program of [checked,
      decodeTypedProgramLinks(LinkNetwork.fromLino(archive.network.toLino()), archive.root),
      decodeTypedProgramLinks(decodeLinkNetworkBinary(encodeLinkNetworkBinary(archive.network)), archive.root),
    ]) {
      assert.deepEqual(observed(emitJavaScript(program).text), expected, source);
      assert.equal(emitRust(program).text, rust, 'both source-free archives preserve the admitted initializer');
    }
  }
  for (const [source, expected, earlierConstruct] of [
    ['const base={value:3};const NaN={...base};function read(){return NaN.value;}console.log(read());', ['3'], 'namespace after a top-level statement'],
    ['const base={value:3};const NaN={nested:{...base}};function read(){return NaN.nested.value;}console.log(read());', ['3'], 'namespace after a top-level statement'],
    ['const NaN={...{value:3}};function read(){return NaN.value;}console.log(read());', ['3']],
    ['const NaN={nested:{...{value:3}}};function read(){return NaN.nested.value;}console.log(read());', ['3']],
    ['function create(){console.log("init");return 3;}const NaN=create();function read(){return NaN;}console.log(read());', ['init', '3']],
    ['const base=3;const NaN=base+1;function read(){return NaN;}console.log(read());', ['4']],
    ['const NaN=[3];function read(){return NaN[0];}console.log(read());', ['3']],
    ['let NaN=3;function read(){return NaN;}console.log(read());', ['3']],
    ['let Infinity=4;function read(){return Infinity;}console.log(read());', ['4']],
    ['let undefined=5;function read(){return undefined;}console.log(read());', ['5']],
  ]) {
    assert.deepEqual(observed(source), expected, 'refusal does not change the original module observations');
    assert.throws(() => parseJavaScript(source), error => error.kind === 'unsupported'
      && (earlierConstruct ? error.details.construct === earlierConstruct
        : /module binding|module-capture/u.test(error.reason)), source);
  }
});

test('a program of loops is a semantic translation in every target, and Rust and JavaScript run lifted loops as loops', () => {
  for (const target of ['Rust', 'Lean', 'Rocq']) {
    assert.equal(translateProgram(SUM, 'JavaScript', target).contract.support, 'semantic-translation', target);
  }
  assert.match(translateProgram(SUM, 'JavaScript', 'Rust').code, /pub fn ml_sum_loop1\(mut n: [^)]*\) -> crate::ml::Big \{\n {4}loop \{/u);
  // A hundred thousand iterations would overflow the stack as a hundred thousand calls.
  const long = SUM.replace('sum(10n)', 'sum(100000n)');
  const { text } = emitJavaScript(checkProgram(parseJavaScript(long)));
  assert.match(text, /for \(;;\) \{/u);
  assert.equal(execFileSync(process.execPath, ['--input-type=module', '-e', text], { encoding: 'utf8' }), '5000050000n\n');
});
