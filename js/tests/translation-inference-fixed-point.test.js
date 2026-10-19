import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { test } from 'node:test';

import { parseJavaScript } from '../src/translation/javascript.js';
import { preservesTranslationBehavior } from './support/translation-behavior.js';

const mutuallyRecursiveRecords = `function f(x) { return x ? g(false) : {value: 1}; }
function g(x) { return x ? f(false) : {value: 2}; }
console.log(f(false).value, f(true).value, g(false).value, g(true).value);
`;

test('equal mutually recursive record returns reach a finite fixed point and preserve behavior', () => {
  // A synchronous infinite solver cannot be interrupted by node:test's timer.
  // Keep the exact regression inside a child with an actual bounded deadline.
  const parser = new URL('../src/translation/javascript.js', import.meta.url).href;
  const code = `import { parseJavaScript } from ${JSON.stringify(parser)};
const program = parseJavaScript(${JSON.stringify(mutuallyRecursiveRecords)});
console.log(JSON.stringify(program.items.map(item => item.ret)));`;
  const child = spawnSync(process.execPath, ['--input-type=module', '-e', code], { encoding: 'utf8', timeout: 2000 });
  assert.ifError(child.error);
  assert.equal(child.status, 0, child.stderr);
  assert.deepEqual(JSON.parse(child.stdout), Array.from({ length: 2 }, () => ({ kind: 'record', fields: [{ name: 'value', type: { kind: 'float' } }] })));
  preservesTranslationBehavior(mutuallyRecursiveRecords, '1 2 2 1\n');
});

test('record joins preserve constraints across field ordering and unresolved fields', () => {
  const source = `function f(x) { return x ? g(false) : {value: 1, active: false}; }
function g(x) { return x ? f(false) : {active: true, value: 2}; }
console.log(f(false).value, f(true).active, g(false).value, g(true).active);
`;
  preservesTranslationBehavior(source, '1 true 2 false\n');
  assert.throws(() => parseJavaScript("function f(x) { return x ? g(false) : {value: 1}; } function g(x) { return x ? f(false) : {value: 'two'}; }"),
    error => error.kind === 'type' && /number.*string|string.*number/u.test(error.message));
});

test('nullable and absent return branches keep their distinct source values', () => {
  preservesTranslationBehavior(`function f(x) { return x ? g(false) : undefined; }
function g(x) { return x ? f(false) : 1; }
console.log(f(false), f(true), g(false), g(true));
`, 'undefined 1 1 undefined\n');
  preservesTranslationBehavior('function f(x) { if (x) return {value: 1}; }\nconsole.log(f(false) === undefined, f(true)?.value ?? 0);\n', 'true 1\n');
});
