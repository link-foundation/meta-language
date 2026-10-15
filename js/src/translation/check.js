import {captureRustRuntimeShell,checkRustRuntimeShellContainers} from './rust-runtime-shell.js';
import {checkRustPureUnitCalls} from './rust-pure-unit-calls.js';
import {checkRustEmptyOptionProof} from './rust-empty-options.js';
// Resolves names, checks types, and fixes the exact operator semantics of a
// surface program produced by one of the language frontends. The result is
// the portable-core program the emitters consume: every expression carries
// its type and every operator carries the source language's semantics, so a
// target can never silently substitute its own (for example, Lean's Euclidean
// `Int` division for JavaScript's truncating BigInt division).

import { typeError, unsupported } from './diagnostics.js';
import { checkRustSourceSignature } from './rust-source-signatures.js';
import { validateRustTypeNamespace, checkRustTypeNamespaceRecordShapes } from './rust-type-namespace.js';
import {prepareRustPrimitiveOwnerProof,checkRustPrimitiveOwnerProof} from './rust-primitive-owners.js';
import {prepareRustOwnedLocalStateProof,checkRustOwnedLocalStateProof,checkRustOwnedLocalStateContainers} from './rust-owned-local-state.js';
import {checkRustConstFunctionProof,checkRustConstFunctionContainers} from './rust-const-functions.js';
import { acceptArgumentCount, acceptCheckedTypeQueryOperand, acceptTypeQueryOperand, readTypeQueryResult } from './frontend-rules.js';
import { normaliseProof } from './proof.js';
import { referenceOperation } from './reference-model.js';
import { acceptTupleIndex, readNumberDisplayRefusal, readScalarCellOperation, readTuplePosition } from './primitive-value-rules.js';
import { checkRustScalarLoopProof } from './rust-scalar-loops.js';
import { acceptOptionalAbsence } from './structured-type-rules.js';
import { acceptProgramTupleProductModel } from '../typed-program-link-rules.js';
import {
  BOOL, FLOAT, INT, NAT, STRING, UNIT, array, data, fixedBounds, immutableProductType, isFloat, isNatural, isNumeric, sameType, tuple, typeKey,
} from './types.js';

const COMPARISONS = new Set(['eq', 'ne', 'lt', 'le', 'gt', 'ge']);
const ARITHMETIC = new Set(['add', 'sub', 'mul', 'div', 'rem']);
// The styles of the values console.log shows: its argument, or those util.format reads.
const CONSOLE_STYLES = new Set(['js-console', 'js-format-first', 'js-format-number', 'js-format-integer']);
// Environment entry recording, for a local, the constructor it is known to
// have matched and the locals naming its fields.
const KNOWN = Symbol('known constructors');
const PRODUCT_VALUES = Symbol('known immutable product values');
/** Natural numeral patterns above this unfold to equality tests rather than successor chains. */
const NAT_PATTERN_UNFOLD = 16n;

// The Math and Number functions whose value is a boolean.
const PREDICATES = new Set(['isInteger', 'isSafeInteger', 'isFinite', 'isNaN']);

/** Binds a local, forgetting constructor facts the new binding shadows. */
function bindLocal(env, name, type, immutableProduct = false) {
  env.set(name, type);
  const products = env.get(PRODUCT_VALUES);
  if (immutableProduct || products?.has(name)) {
    const updated = new Map(products);
    if (immutableProduct) updated.set(name, true);
    else updated.delete(name);
    env.set(PRODUCT_VALUES, updated);
  }
  const known = env.get(KNOWN);
  if (known && [...known].some(([variable, fact]) => variable === name || fact.binds.includes(name))) {
    env.set(KNOWN, new Map([...known].filter(([variable, fact]) => variable !== name && !fact.binds.includes(name))));
  }
}

// A nested product needs a proof about the value returned, not only its type.
// Unknown parameter/call origins remain unsupported by this local proof.
function immutableProductValue(value, bindings = new Map()) {
  if (value.type.kind !== 'tuple') return true;
  switch (value.k) {
    case 'tuple':
      return value.frozen === true && value.items.every(item => immutableProductValue(item, bindings));
    case 'let':
      return immutableProductValue(value.body, new Map(bindings).set(value.name, immutableProductValue(value.value, bindings)));
    case 'if':
      return immutableProductValue(value.then, bindings) && immutableProductValue(value.else, bindings);
    case 'var':
      return bindings.get(value.name) === true;
    default:
      return false;
  }
}

export function checkProgram(surface) {
  checkRustConstFunctionContainers(surface);
  checkRustRuntimeShellContainers(surface);
