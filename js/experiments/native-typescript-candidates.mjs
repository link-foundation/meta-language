// Sorts candidate sources for the native TypeScript or TSX fixture
// (GRAMMAR=tsx picks TSX): MATCH (the oracle accepts and both trees agree),
// REJECT (the oracle recovers, the native grammar rejects and recovers), or
// what keeps them out of the fixture.
//   node experiments/native-typescript-candidates.mjs
import { readFileSync } from 'node:fs';

import { compileGrammar } from '../src/grammar.js';
import { parseGrammarLinks } from '../src/grammar-links.js';
import { nativeOracleKinds } from '../scripts/build-language-catalog.mjs';
import { nativeRows, oracleRecovers, oracleRows } from '../scripts/native-grammar-rows.mjs';

const grammar = process.env.GRAMMAR ?? 'typescript';
const language = grammar === 'tsx' ? 'TSX' : 'TypeScript';
const text = readFileSync(new URL(`../../parity/grammars/native/${grammar}.lino`, import.meta.url), 'utf8');
const compiled = compileGrammar(parseGrammarLinks(text));
const options = { hidden: [], anonymous: ['unnamed_token'], extras: ['comment', 'html_comment'], oracleKinds: nativeOracleKinds(text) };
const inventory = JSON.parse(readFileSync(new URL('../../parity/language-grammar-inventory.json', import.meta.url), 'utf8'));
const entry = inventory.languages.find(({ name }) => name === language);
const shared = [
  '', 'x;', 'let x: number = 1;', 'const f = (a: string, b?: number): void => {};', 'function f<T extends object>(a: T, ...b: T[]): T { return a; }',
  'interface A extends B { x: number; readonly y?: string; [k: string]: unknown }', 'type U = A | B & C;', 'type F = (a: number) => string;',
  'enum E { A = 1, B, C = "c" }', 'const enum E { A }', 'namespace N { export const x = 1; }', 'declare module "m" { export function f(): void; }',
  'abstract class A<T> implements I { private x: T; protected abstract m(): void; constructor(public y: number) { super(); } }',
  'let x = y as unknown as string;', 'let x = y satisfies Z;', 'let x = y!;', 'type K = keyof typeof o;', 'type M = { [P in keyof T]?: T[P] };',
  'type C<T> = T extends string ? "s" : never;', 'type L = `a${B}c`;', 'type T = [a: number, b?: string, ...rest: boolean[]];',
  'import type { A } from "a";', 'export type { B };', 'import x = require("x");', 'export = x;', 'declare global { interface Window { a: 1 } }',
  'function assert(x: unknown): asserts x is string {}', 'function f(this: Window) {}', '@dec class A { @prop() x = 1; }',
  '!g<T>();', 'await g<T>;', 'typeof g<number>(x);', 'for (const [k, v] of m) {}', 'a?.b ?? c;', 'x = `a${b}c`;', 'async function f(): Promise<void> { await g<number>(); }',
  'let x = 1;　let y;', 'let a; let b;', 'x; y;', 'x;﻿', '// c\nlet x = 1 /* d */;\n',
  entry.source, entry.recoverySource,
  'const = 1;', 'let x: = 1;', 'interface A {', 'type = number;', 'function f(', 'function f(): {', 'class A {', 'enum E {', 'let x = <;',
  'namespace {', 'type T = [;', '}', 'x = 1 +;', 'if (x', 'import {', 'let x: number[;',
];
const extra = grammar === 'tsx'
  ? ['x = <div className="a">{b}</div>;', 'const C = <T,>(a: T) => <span>{a}</span>;', 'x = <A.B c={1} {...d} />;', 'x = <></>;', 'x = <div>;', 'x = <a></b>;']
  : ['let x = <string>y;', 'const f = <T>(a: T) => a;', 'x = a < b > c;'];
for (const source of [...shared, ...extra]) {
  let kind;
  if (oracleRecovers(source, language)) {
    if (compiled.parseTree(source).ok) kind = 'NATIVE-ACCEPTS';
    else kind = compiled.parseTree(source, { errorRecovery: true }).rejection?.reason === 'recovered' ? 'REJECT' : 'NO-RECOVERY';
  } else {
    const outcome = compiled.parseTree(source);
    if (!outcome.ok) kind = 'NATIVE-REJECTS';
    else kind = JSON.stringify(nativeRows(outcome.tree, source, options)) === JSON.stringify(oracleRows(source, language)) ? 'MATCH' : 'DIFF';
  }
  console.log(kind, JSON.stringify(source));
}
