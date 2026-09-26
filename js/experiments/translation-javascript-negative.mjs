// JavaScript constructs outside the portable core must raise precise obligations.
import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';

const doc = '/**\n * @param {bigint} n\n * @returns {bigint}\n */\n';
const tree = "/** @typedef {{ $: 'leaf' } | { $: 'node', value: bigint }} T */\n";
const cases = {
  numberLiteral: 'console.log(String(1 + 2));',
  decimal: 'console.log(String(1.5));',
  numberParam: '/**\n * @param {number} n\n * @returns {bigint}\n */\nfunction f(n) { return 1n; }',
  untyped: 'function f(n) { return n; }',
  noReturns: '/** @param {bigint} n */\nfunction f(n) { return n; }',
  letBinding: `${doc}function f(n) { let x = n; return x; }`,
  loop: `${doc}function f(n) { for (;;) {} }`,
  missingReturn: `${doc}function f(n) { if (n > 0n) return n; }`,
  bareReturn: `${doc}function f(n) { return; }`,
  asiReturn: `${doc}function f(n) { return\n n; }`,
  unreachable: `${doc}function f(n) { return n; return n; }`,
  looseEquality: `${doc}function f(n) { return n == 0n ? 1n : n; }`,
  nullish: `${doc}function f(n) { return n ?? 0n; }`,
  bitwise: `${doc}function f(n) { return n & 1n; }`,
  shift: `${doc}function f(n) { return n << 1n; }`,
  power: `${doc}function f(n) { return n ** 2n; }`,
  unaryPlus: `${doc}function f(n) { return +n; }`,
  assignment: `${doc}function f(n) { n = 1n; return n; }`,
  increment: `${doc}function f(n) { n++; return n; }`,
  arrow: 'const g = (x) => x;',
  closure: `${doc}function f(n) { const g = (x) => x; return n; }`,
  functionValue: `${doc}function f(n) { const g = f; return n; }`,
  array: 'console.log(String([1n]));',
  nullValue: `${doc}function f(n) { return null; }`,
  mathGlobal: `${doc}function f(n) { return Math.max(n, 1n); }`,
  consoleFormat: "console.log('a', 1n);",
  consoleError: "console.error('a');",
  otherImport: "import fs from 'node:fs';",
  asyncFunction: `${doc}async function f(n) { return n; }`,
  generator: `${doc}function* f(n) { return n; }`,
  tdzOwn: `${doc}function f(n) { const x = x; return n; }`,
  tdzLater: `${doc}function f(n) { const y = x; const x = n; return y; }`,
  namespaceAfterEffect: "console.log('x');\nconst A = { /** @returns {bigint} */ f() { return 1n; } };",
  throwString: `${doc}function f(n) { throw 'bad'; }`,
  computedMessage: `${doc}function f(n) { throw new Error(String(n)); }`,
  expressionStatement: `${doc}function f(n) { f(n); return n; }`,
  caseDeclaration: `${doc}function f(n) { switch (n) { case 0n: const x = 1n; return x; default: return n; } }`,
  fallThrough: `${doc}function f(n) { switch (n) { case 0n: if (n > 0n) return 1n; default: return n; } }`,
  identityOfObjects: `${tree}import assert from 'node:assert/strict';\nassert.equal({ $: 'leaf' }, { $: 'leaf' });`,
  objectEquality: `${tree}/**\n * @param {T} t\n * @returns {boolean}\n */\nfunction f(t) { return t === t; }`,
  printObject: `${tree}console.log({ $: 'leaf' });`,
  stringOfObject: `${tree}console.log('x' + { $: 'leaf' });`,
  fieldOutsideSwitch: `${tree}/**\n * @param {T} t\n * @returns {bigint}\n */\nfunction f(t) { return t.value; }`,
  unknownTag: `${tree}console.log(String({ $: 'branch' } === 1n));`,
  missingField: `${tree}/** @returns {T} */\nfunction f() { return { $: 'node' }; }`,
  assertMessage: "import assert from 'node:assert/strict';\nassert(true, 'message');",
  looseAssertEqual: "import assert from 'node:assert';\nassert.equal(1n, 1n);",
  assertThrows: "import assert from 'node:assert/strict';\nassert.throws(1n);",
  optionalChain: `${tree}/**\n * @param {T} t\n * @returns {bigint}\n */\nfunction f(t) { switch (t?.$) { default: return 0n; } }`,
  topLevelIf: "if (true) console.log('x');",
  exportDefault: 'export default 1n;',
};
for (const [name, source] of Object.entries(cases)) {
  try {
    checkProgram(parseJavaScript(source));
    console.log(`${name}: ACCEPTED`);
  } catch (error) {
    console.log(`${name}: ${error.kind ?? 'CRASH'} ${error.message}`);
  }
}
