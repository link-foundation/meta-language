#!/usr/bin/env node
// Bounded host-adapter fixtures, including independently specified identities
// and complete emitted identity functions for every compound signature kind.
import { readFileSync, writeFileSync } from 'node:fs';
import { checkProgram } from '../src/translation/check.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { typeKey } from '../src/translation/types.js';

const bool = { kind: 'bool' };
const string = { kind: 'string' };
const optional = (element, absence) => ({ kind: 'option', element, absence });
const record = (fields, open = false) => ({ kind: 'record', fields: fields.map(([name, type]) => ({ name, type })), ...(open ? { open: true } : {}) });
const cases = [
  ['boolean', bool, 'bool'], ['float', { kind: 'float' }, 'float'], ['signed integer', { kind: 'fixed', bits: 32, signed: true }, 'i32'],
  ['nullable boolean', optional(bool, 'null'), 'option:null:bool'],
  ['undefined boolean', optional(bool, 'undefined'), 'option:undefined:bool'],
  ['mixed nested absence', optional(optional(bool, 'null'), 'undefined'), 'option:both:bool'],
  ['equal nested absence', optional(optional(bool, 'null'), 'null'), 'option:null:bool'],
  ['empty tuple', { kind: 'tuple', elements: [] }, 'tuple:[]'],
  ['singleton tuple', { kind: 'tuple', elements: [bool] }, 'tuple:[bool]'],
  ['ordered tuple', { kind: 'tuple', elements: [bool, string] }, 'tuple:[bool,string]'],
  ['empty record', record([]), 'record:[]'],
  ['open record', record([], true), 'record:open:[]'],
  ['sorted record', record([['z', bool], ['a', string]]), 'record:[["a","string"],["z","bool"]]'],
  ['quoted record field', record([['a"b\\c', bool]]), 'record:[["a\\"b\\\\c","bool"]]'],
  ['UTF-16 record field order', record([['\uE000', bool], ['😀', string]]), 'record:[["😀","string"],["\uE000","bool"]]'],
  ['readonly map', { kind: 'map', key: string, value: bool }, 'map:string:bool'],
  ['readonly set', { kind: 'set', element: string }, 'set:string'],
  ['zero parameter function', { kind: 'function', params: [], ret: bool }, 'function:():bool'],
  ['callable function', { kind: 'function', params: [string, bool], ret: string }, 'function:(string,bool):string'],
  ['array of nullable records', { kind: 'array', element: optional(record([['value', bool]]), 'both') }, 'array:option:both:record:[["value","bool"]]'],
].map(([name, type, identity]) => {
  if (typeKey(type) !== identity) throw new Error(`${name}: canonical identity differs from independently specified fixture`);
  return { name, type, identity };
});
const emissionNames = ['nullable boolean', 'undefined boolean', 'empty tuple', 'singleton tuple', 'ordered tuple', 'empty record', 'sorted record', 'readonly map', 'readonly set', 'zero parameter function', 'callable function', 'array of nullable records'];
const emissionCases = cases.filter(({ name }) => emissionNames.includes(name)).map(({ name, type }) => {
  const surface = { language: 'JavaScript', items: [{ k: 'fn', name: 'identity', params: [{ name: 'value', type }], ret: type, body: { k: 'name', path: ['value'] } }], main: null };
  return { name, surface, rust: emitRust(checkProgram(surface)).text };
});
const data = `${JSON.stringify({ cases, emissionCases, optionalPrelude: '/// A nullable value retains the distinct JavaScript null and undefined cases.\n#[derive(Clone, Debug, PartialEq)]\npub enum MlOptional<T> { Value(T), Null, Undefined }' }, null, 2)}\n`;
const target = new URL('../../parity/fixtures/translation-structured-types.json', import.meta.url);
if (process.argv.includes('--check')) {
  if (readFileSync(target, 'utf8') !== data) throw new Error('structured type fixtures have drifted');
} else writeFileSync(target, data);
