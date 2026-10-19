// Admission and inspectable data preparation for the bounded portable rules.
// No renderer, inspector output or original object/source is stored in LiNo.
import { types } from 'node:util';
import { readStructuredDataLino, structuredDataToLino } from '../js/src/structured-data-links.js';
import { renderRecordText } from './issue-199-record-text-rules.mjs';

export const recordTextModels = Object.freeze([
  'ecmascript-ordinary-record-coercion', 'node-console-record-inspection',
]);
const ordinaryPrototype = Object.prototype;
const unsupported = detail => { throw new TypeError(`record text draft: ${detail}`); };
const safeText = text => typeof text === 'string' && text.length <= 80 && /^[\t\n\r\x20-\x7e]*$/u.test(text);
const integerText = text => /^-?(?:0|[1-9][0-9]*)$/u.test(text) && text !== '-0';

// Direct callers must provide inert own data, just like the LiNo decoder does.
// Check before reading any field, array length or element from an unknown tape.
function requireInertData(value, array = false) {
  if (!value || typeof value !== 'object' || types.isProxy(value)) unsupported('effectful tape object');
  const prototype = Object.getPrototypeOf(value);
  if (array ? !Array.isArray(value) || prototype !== Array.prototype
    : prototype !== ordinaryPrototype && prototype !== null) unsupported('unexpected tape prototype');
  const fields = Reflect.ownKeys(value);
  for (const field of fields) {
    const descriptor = Object.getOwnPropertyDescriptor(value, field);
    if (typeof field !== 'string' || !Object.hasOwn(descriptor, 'value')
      || !descriptor.enumerable && !(array && field === 'length')) unsupported('effectful or hidden tape field');
  }
  if (array && (value.length > 32 || fields.length !== value.length + 1
    || fields.some(field => field !== 'length' && (!/^(?:0|[1-9][0-9]*)$/u.test(field) || Number(field) >= value.length)))) {
    unsupported('sparse or decorated tape array');
  }
}

/** Only ordinary own enumerable data descriptors are admitted, without getters. */
export function prepareRecordTextDraft(value) {
  const kinds = [], keys = [], values = [], ends = [];
  const seen = new WeakSet();
  function visit(current, key, depth) {
    if (kinds.length >= 32) unsupported('more than 32 nodes');
    const index = kinds.length;
    keys.push(key); values.push(''); ends.push(0);
    if (typeof current === 'string') {
      if (!safeText(current)) unsupported('string outside the bounded ASCII model');
      kinds.push('string'); values[index] = current;
    } else if (typeof current === 'boolean') {
      kinds.push('boolean'); values[index] = current ? 'true' : 'false';
    } else if (typeof current === 'bigint') {
      const text = current.toString();
      if (text.length > 32) unsupported('integer magnitude exceeds the draft text bound');
      kinds.push('bigint'); values[index] = text;
    } else if (typeof current === 'number') {
      if (!Number.isSafeInteger(current)) unsupported('floating-point formatting requires the separate decimal rules');
      kinds.push('number'); values[index] = Object.is(current, -0) ? '-0' : String(current);
    } else if (current && typeof current === 'object') {
      if (types.isProxy(current)) unsupported('proxy effects');
      if (Object.getPrototypeOf(current) !== ordinaryPrototype) unsupported('nonordinary prototypes');
      if (seen.has(current)) unsupported('cycles or shared record identities');
      if (depth > 2) unsupported('record nesting exceeds the default inspector depth');
      seen.add(current); kinds.push('record');
      for (const field of Reflect.ownKeys(current)) {
        if (!safeText(field)) unsupported('symbol or non-ASCII property key');
        if (field === 'toString' || field === 'valueOf') unsupported('own primitive conversion hooks');
        const descriptor = Object.getOwnPropertyDescriptor(current, field);
        if (!descriptor.enumerable || !Object.hasOwn(descriptor, 'value')) unsupported('hidden properties or accessor effects');
        visit(descriptor.value, field, depth + 1);
      }
    } else unsupported('unsupported scalar or effectful value');
    ends[index] = kinds.length;
  }
  visit(value, '', 0);
  const tape = { schemaVersion: 1, model: 'ordinary ASCII record tree', kinds, keys, values, ends };
  validateRecordTextDraft(tape);
  return tape;
}

/** Reload must not turn untrusted endpoints into loops or hide unused nodes. */
export function validateRecordTextDraft(tape) {
  requireInertData(tape);
  if (Object.keys(tape).sort().join(',') !== 'ends,keys,kinds,model,schemaVersion,values'
    || tape.schemaVersion !== 1 || tape.model !== 'ordinary ASCII record tree') unsupported('unknown tape schema');
  const { kinds, keys, values, ends } = tape;
  for (const items of [kinds, keys, values, ends]) requireInertData(items, true);
  if (kinds.length < 1 || kinds.length > 32
    || [keys, values, ends].some(items => items.length !== kinds.length)
    || kinds[0] !== 'record' || keys[0] !== '') unsupported('inconsistent tape arrays');
  function visit(index, depth) {
    if (!Number.isInteger(ends[index]) || ends[index] <= index || ends[index] > kinds.length
      || !safeText(keys[index]) || !safeText(values[index])) unsupported('invalid node value or subtree endpoint');
    const kind = kinds[index], value = values[index];
    if (kind === 'record') {
      if (depth > 2 || value !== '') unsupported('record depth or opaque record payload');
      let child = index + 1;
      const names = new Set();
      let lastArrayIndex = -1;
      let ordinaryNamesStarted = false;
      while (child < ends[index]) {
        const key = keys[child];
        if (names.has(key) || key === 'toString' || key === 'valueOf') unsupported('duplicate keys or primitive conversion hooks');
        // OrdinaryOwnPropertyKeys places array-index names first in numeric
        // order, followed by other string names in their captured source order.
        if (/^(?:0|[1-9][0-9]*)$/u.test(key) && Number(key) < 4294967295) {
          if (ordinaryNamesStarted || Number(key) <= lastArrayIndex) unsupported('noncanonical array-index property order');
          lastArrayIndex = Number(key);
        } else ordinaryNamesStarted = true;
        names.add(key); visit(child, depth + 1); child = ends[child];
      }
      if (child !== ends[index]) unsupported('child subtree escapes its parent');
    } else {
      if (ends[index] !== index + 1) unsupported('scalar subtree has hidden nodes');
      if (kind === 'boolean') { if (value !== 'true' && value !== 'false') unsupported('noncanonical boolean'); }
      else if (kind === 'bigint') { if (!integerText(value) || value.length > 32) unsupported('noncanonical integer'); }
      else if (kind === 'number') {
        if (value !== '-0' && (!integerText(value) || !Number.isSafeInteger(Number(value)) || String(Number(value)) !== value)) unsupported('noncanonical safe number');
      } else if (kind !== 'string') unsupported('unknown scalar kind');
    }
  }
  visit(0, 0);
  if (ends[0] !== kinds.length) unsupported('disconnected trailing nodes');
  if (!renderRecordText(kinds, keys, values, ends, recordTextModels[1])) unsupported('default inspector multiline layout requires further rules');
  return tape;
}

export function recordTextDraftToLino(tape) {
  return structuredDataToLino(validateRecordTextDraft(tape));
}

export function readRecordTextDraftLino(text) {
  return validateRecordTextDraft(readStructuredDataLino(text, { byteLimit: 128 * 1024, nodeLimit: 2048 }));
}

export function formatRecordTextDraft(tape, model) {
  validateRecordTextDraft(tape);
  if (!recordTextModels.includes(model)) unsupported('undeclared conversion model');
  return renderRecordText(tape.kinds, tape.keys, tape.values, tape.ends, model);
}
