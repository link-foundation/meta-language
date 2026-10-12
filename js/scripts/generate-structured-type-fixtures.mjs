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
  ['false reference remains readonly', { kind: 'array', element: bool, reference: false }, 'array:bool'],
  ['reference array', { kind: 'array', element: bool, reference: true }, 'reference:array:bool'],
  ['reference record', { ...record([['z', bool], ['a', string]]), reference: true }, 'reference:record:[["a","string"],["z","bool"]]'],
  ['reference map', { kind: 'map', key: string, value: bool, reference: true }, 'reference:map:string:bool'],
  ['reference set', { kind: 'set', element: string, reference: true }, 'reference:set:string'],
  ['boolean cell', { kind: 'cell', element: bool }, 'cell:bool'],
  ['optional reference cell', optional({ kind: 'cell', element: { kind: 'array', element: bool, reference: true } }, 'both'), 'option:both:cell:reference:array:bool'],
].map(([name, type, identity]) => {
  if (typeKey(type) !== identity) throw new Error(`${name}: canonical identity differs from independently specified fixture`);
  return { name, type, identity };
});
const emissionNames = ['nullable boolean', 'undefined boolean', 'empty tuple', 'singleton tuple', 'ordered tuple', 'empty record', 'sorted record', 'readonly map', 'readonly set', 'zero parameter function', 'callable function', 'array of nullable records', 'reference array', 'reference record', 'reference map', 'reference set', 'boolean cell', 'optional reference cell'];
const emissionCases = cases.filter(({ name }) => emissionNames.includes(name)).map(({ name, type }) => {
  const surface = { language: 'JavaScript', items: [{ k: 'fn', name: 'identity', params: [{ name: 'value', type }], ret: type, body: { k: 'name', path: ['value'] } }], main: null };
  const checked = checkProgram(surface);
  if (typeKey(checked.declarations.get('identity').params[0].type) !== typeKey(type)) throw new Error(`${name}: checker changed the parameter type identity`);
  const rust = emitRust(checked).text;
  const referenceTargetTypes = {
    'reference array': 'std::rc::Rc<std::cell::RefCell<Vec<bool>>>',
    'reference record': 'std::rc::Rc<std::cell::RefCell<(String, bool)>>',
    'reference map': 'std::rc::Rc<std::cell::RefCell<Vec<(String, bool)>>>',
    'reference set': 'std::rc::Rc<std::cell::RefCell<Vec<String>>>',
    'boolean cell': 'std::rc::Rc<std::cell::RefCell<bool>>',
    'optional reference cell': 'crate::MlOptional<std::rc::Rc<std::cell::RefCell<std::rc::Rc<std::cell::RefCell<Vec<bool>>>>>>',
  };
  const targetType = referenceTargetTypes[name];
  if (targetType && !rust.includes(`pub fn identity(value: ${targetType}) -> ${targetType}`)) throw new Error(`${name}: Rust signature differs from independently specified reference encoding`);
  return { name, surface, rust };
});
const traitCases = [
  ['nullable boolean retains equality', optional(bool, 'both'), true],
  ['nullable Number omits total equality', optional({ kind: 'float' }, 'both'), false],
  ['nested record map Number omits total equality', record([['values', { kind: 'map', key: string, value: { kind: 'float' } }]]), false],
  ['reference cell Number omits total equality', { kind: 'cell', element: { kind: 'float' } }, false],
].map(([name, type, totalEquality]) => {
  const surface = { language: 'JavaScript', items: [{ k: 'data', name: 'Box', ctors: [{ name: 'Wrap', fields: [{ type }] }] }], main: null };
  const rust = emitRust(checkProgram(surface)).text;
  const derive = totalEquality ? '#[derive(Clone, Debug, PartialEq, Eq)]\npub enum Box' : '#[derive(Clone, Debug, PartialEq)]\npub enum Box';
  if (!rust.includes(derive)) throw new Error(`${name}: Rust data traits differ from independent expectation`);
  return { name, surface, rust, totalEquality };
});
const callableData = { language: 'JavaScript', items: [{ k: 'data', name: 'Box', ctors: [{ name: 'Wrap', fields: [{ type: optional(record([['callback', { kind: 'function', params: [], ret: bool }]]), 'both') }] }] }], main: null };
const data = `${JSON.stringify({ cases, emissionCases, traitCases, callableData, optionalPrelude: '/// A nullable value retains the distinct JavaScript null and undefined cases.\n#[derive(Clone, Debug, PartialEq, Eq)]\npub enum MlOptional<T> { Value(T), Null, Undefined }' }, null, 2)}\n`;
const target = new URL('../../parity/fixtures/translation-structured-types.json', import.meta.url);
if (process.argv.includes('--check')) {
  if (readFileSync(target, 'utf8') !== data) throw new Error('structured type fixtures have drifted');
} else writeFileSync(target, data);
