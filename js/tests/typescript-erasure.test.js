import assert from 'node:assert/strict';
import { test } from 'node:test';
import vm from 'node:vm';
import { eraseTypeScript } from '../src/typescript-erasure.js';
import { selfTranslate } from '../src/self-translation.js';

test('TypeScript annotations erase through the native tree and translated JavaScript executes', () => {
  const source = 'function add(a: number, b: number): number { return a + b; }\n';
  const result = selfTranslate(source, 'TypeScript', 'JavaScript');
  assert.ok(result.items.every(({ status }) => status === 'translated'));
  assert.equal(vm.runInNewContext(`${result.code}\nadd(5, 8)`), 13);
  assert.equal(selfTranslate(result.code, 'JavaScript', 'TypeScript').code, source);
  const rust = selfTranslate(source, 'TypeScript', 'Rust');
  assert.ok(rust.items.every(({ status }) => status === 'translated'));
  assert.match(rust.code, /pub fn add\(a: f64, b: f64\) -> f64/u);
});

test('type aliases and exported interfaces keep provenance with zero target definitions', () => {
  const source = 'export interface Entry { value: string }\ntype Amount = number;\nconst amount: Amount = 3;\n';
  const result = selfTranslate(source, 'TypeScript', 'JavaScript');
  assert.equal(result.items.filter(({ status }) => status === 'translated').length, 3);
  assert.equal(vm.runInNewContext(`${result.code}\namount`), 3);
  assert.equal(selfTranslate(result.code, 'JavaScript', 'TypeScript').code, source);
});

test('generic, optional, assertion and Unicode spans erase while runtime punctuation remains', () => {
  const source = 'function identity<T>(value?: T): T { return value!; }\nconst 汉字: number = 3 as number;\nconst result = 汉字 > 0 ? identity<number>(汉字) : 0;\n';
  const result = eraseTypeScript(source);
  assert.ok(result.erasures.length >= 7);
  assert.match(result.code, /汉字 > 0 \? identity/u, 'the conditional operator is preserved');
  assert.equal(vm.runInNewContext(`${result.code}\nresult`), 3);
  const translation = selfTranslate(source, 'TypeScript', 'JavaScript');
  assert.equal(selfTranslate(translation.code, 'JavaScript', 'TypeScript').code, source);
});

test('runtime TypeScript constructs are carried with a precise obligation', () => {
  for (const source of ['enum Entry { One }\n', 'namespace Entry { export const value = 1; }\n',
    'class Entry { constructor(private value: number) {} }\n']) {
    const result = selfTranslate(source, 'TypeScript', 'JavaScript');
    assert.ok(result.items.some(({ status }) => status === 'carried'));
    assert.ok(result.items.some(({ reason }) => reason?.includes('runtime construct needs explicit translation')));
    assert.match(result.code, /meta-language:carried/u);
    assert.equal(selfTranslate(result.code, 'JavaScript', 'TypeScript').code, source);
  }
});

test('ordinary JavaScript syntax remains unchanged in both JavaScript family directions', () => {
  const source = 'function add(a, b) { return a + b; }\n';
  assert.equal(selfTranslate(source, 'JavaScript', 'TypeScript').code, source);
  assert.equal(selfTranslate(source, 'TypeScript', 'JavaScript').code, source);
});

test('comments before erased types and runtime declarations restore with their exact source', () => {
  const source = '// exported compile-time shape\nexport interface Entry { value: number }\n\n// executable function\nexport function add(a: number, b: number): number { return a+b; }\n';
  const translation = selfTranslate(source, 'TypeScript', 'JavaScript');
  assert.equal(selfTranslate(translation.code, 'JavaScript', 'TypeScript').code, source);
});

test('editing emitted JavaScript invalidates erased-source restoration', () => {
  const source = 'function add(a: number, b: number): number { return a + b; }\n';
  const result = selfTranslate(source, 'TypeScript', 'JavaScript');
  const position = result.code.lastIndexOf('return a + b;');
  const edited = result.code.slice(0, position) + result.code.slice(position).replace('return a + b;', 'return a - b;');
  const reverse = selfTranslate(edited, 'JavaScript', 'TypeScript');
  assert.notEqual(reverse.code, source);
  assert.match(reverse.code, /return a - b;/u);
  assert.equal(vm.runInNewContext(`${reverse.code}\nadd(5, 8)`), -3);
});
