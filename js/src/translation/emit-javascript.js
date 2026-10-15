import {checkRustRuntimeShell,recordRustRuntimeShellAssumptions,refuseRustRuntimeShellComposition} from './rust-runtime-shell.js';
import { renderStringMapExpression, renderStringTestExpression } from './frontend-rules.js';
// JavaScript emitter. Every portable integer is a BigInt, so naturals and
// integers stay unbounded and machine integers are range-checked exactly as
// Rust checks them; a Number stays a Number. Data values are plain objects whose `$` property names
// the constructor. Modules become object literals referenced by qualified
// names. Theorems cannot be proved in JavaScript: each becomes an executable
// property, checked over a bounded domain by `--ml-check-theorems`, while the
// proof obligation stays discharged by the source language's kernel.

import { castMessage, overflowMessage, zeroDivisorMessage } from './aborts.js';
import { requireDataObjectFreeze } from './primitive-value-rules.js';
import { renderJavaScriptUnit, renderTupleProjection, renderTupleValue } from './product-emission-rules.js';
import { acceptProgramTupleProductModel } from '../typed-program-link-rules.js';
import { unsupported } from './diagnostics.js';
import { fixedBounds, typeKey } from './types.js';
import { renameFunction, renameMain, renameTheorem, tailLoop } from './ir.js';
import {readJavaScriptScopedBindingEmission} from './javascript-scoped-binding-emission.js';
import {reserveJavaScriptScopedBindingDeclarationNames} from './javascript-scoped-binding-emission-rules.js';
import {checkRustConstFunctionProof} from './rust-const-functions.js';
import {checkRustOwnedLocalStateProof,RUST_OWNED_LOCAL_STATE_POLICY} from './rust-owned-local-state.js';
import {renderRustOwnedLocalTextInput} from './rust-owned-local-state-rules.js';
import { EmitState, withParts } from './emit-common.js';

const KEYWORDS = new Set([
  'break', 'case', 'catch', 'class', 'const', 'continue', 'debugger', 'default', 'delete', 'do', 'else', 'enum',
  'export', 'extends', 'false', 'finally', 'for', 'function', 'if', 'import', 'in', 'instanceof', 'new', 'null',
  'return', 'super', 'switch', 'this', 'throw', 'true', 'try', 'typeof', 'var', 'void', 'while', 'with', 'yield',
  'await', 'let', 'static', 'implements', 'interface', 'package', 'private', 'protected', 'public', 'arguments',
  'eval', 'undefined', 'NaN', 'Infinity', 'globalThis', 'console', 'process', 'BigInt', 'String', 'Object', 'Error',
  'RangeError', 'Math', 'Number', 'Array', 'JSON', 'Symbol', 'main', '__proto__', 'constructor', 'prototype', 'of',
  'async', 'get', 'set',
]);

const HELPERS = {
  ownedTextInput: renderRustOwnedLocalTextInput(),
  natSub: `function ml_natSub(a, b) {
  return a > b ? a - b : 0n;
}`,
  fixed: `function ml_fixed(value, min, max, message) {
  if (value < min || value > max) throw new RangeError(message);
  return value;
}`,
  divide: `// Integer division with the source's rounding; by zero it aborts with the message \`zero\`
// or, when that is null, is total (x / 0 = 0, x % 0 = x); a machine-integer quotient
// out of [min, max] aborts with \`overflow\`, the remainder too.
function ml_divide(a, b, rounding, zero, remainder, bounds) {
  if (b === 0n) {
    if (zero !== null) throw new RangeError(zero);
    return remainder ? a : 0n;
  }
  let q = a / b;
  const r = a - q * b;
  if (r !== 0n) {
    if (rounding === 'floor' && (r < 0n) !== (b < 0n)) q -= 1n;
    if (rounding === 'euclid' && r < 0n) q = b > 0n ? q - 1n : q + 1n;
  }
  if (bounds && (q < bounds[0] || q > bounds[1])) throw new RangeError(bounds[2]);
  return remainder ? a - q * b : q;
}`,
  toNatChecked: `function ml_toNatChecked(value, message) {
  if (value < 0n) throw new RangeError(message);
  return value;
}`,
  abort: `function ml_abort(message) {
  throw new Error(message);
}`,
  assert: `function ml_assert(holds, statement) {
  if (!holds) throw new Error(\`assertion failed: \${statement}\`);
}`,
  equal: `function ml_equal(left, right) {
  if (typeof left !== 'object' || left === null) return left === right;
  const keys = Object.keys(left);
  return keys.length === Object.keys(right).length && keys.every((key) => ml_equal(left[key], right[key]));
}`,
  showNumber: `// console.log prints -0 as -0, where String(-0) is "0".
function ml_showNumber(value) {
  return Object.is(value, -0) ? '-0' : String(value);
}`,
  showOptional: `function ml_showOptional(value) {
  if (value === null) return 'null';
  if (value === undefined) return 'undefined';
  if (typeof value === 'bigint') return String(value) + 'n';
  return Object.is(value, -0) ? '-0' : String(value);
}`,
  at: `// An element of an array; a read outside it, undefined in JavaScript, aborts.
function ml_at(values, index) {
  const at = Number(index);
  if (!Number.isInteger(at) || at < 0 || at >= values.length) throw new RangeError(\`array index \${index} out of range\`);
  return values[at];
}`,
  forall: `function ml_forall(values, property) {
  return values.every(property);
}`,
  domains: `// Bounded domains for executable theorem checks.
const ml_nat = [0n, 1n, 2n, 3n, 4n, 5n, 6n];
const ml_int = [-4n, -3n, -2n, -1n, 0n, 1n, 2n, 3n, 4n];
const ml_small_nat = [0n, 1n, 2n];
const ml_small_int = [-1n, 0n, 1n];
function ml_product(lists) {
  return lists.reduce((rows, list) => rows.flatMap((row) => list.map((value) => [...row, value])), [[]]);
}`,
};

function ident(name) {
  let result = name.replace(/[^A-Za-z0-9_]/gu, '_');
  if (/^[0-9]/u.test(result)) result = `x${result}`;
  if (KEYWORDS.has(result)) result = `${result}_`;
  return result;
}

export function emitJavaScript(program, options = {}) {
  checkRustConstFunctionProof(program);
  const runtimeShell=checkRustRuntimeShell(program);
  if(runtimeShell&&options.scopedBindingModel!==undefined)refuseRustRuntimeShellComposition('scoped target bindings require an independent runtime composition proof');
