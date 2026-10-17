#!/usr/bin/env node
// Bounded native CST/runtime parity cases for the shared TypeScript eraser.
import { checkProjectData, writeProjectData } from './project-data.mjs';
import { eraseTypeScript } from '../src/typescript-erasure.js';
import { selfTranslate } from '../src/self-translation.js';
import { projectTypeScriptTyping } from '../src/typescript-portable-typing.js';

const inputs = [
  ['annotated-function', 'function add(a: number, b: number): number { return a + b; }\n'],
  ['annotated-string-function', 'function identity(value: string): string { return value; }\n'],
  ['annotated-boolean-function', 'function identity(value: boolean): boolean { return value; }\n'],
  ['exported-interface-and-alias', 'export interface Entry { value: string }\ntype Amount = number;\nconst amount: Amount = 3;\n'],
  ['generic-optional-and-Unicode', 'function identity<T>(value?: T): T { return value!; }\nconst 汉字: number = 3 as number;\nconst result = 汉字 > 0 ? identity<number>(汉字) : 0;\n'],
  ['comments-and-compile-time-shape', '// shape\nexport interface Entry { value: number }\n\n// runtime\nexport function add(a: number, b: number): number { return a + b; }\n'],
  ['carriage-returns-and-Unicode-type', 'type 类型 = number;\r\nfunction add(a: 类型, b: 类型): 类型 { return a + b; }\r\n'],
  ['type-only-import', 'import type { Entry } from "./entry.js";\nconst amount: number = 3;\n'],
  ['ordinary-JavaScript', 'function add(a, b) { return a + b; }\n'],
  ['enum-runtime', 'enum Entry { One }\n'],
  ['namespace-runtime', 'namespace Entry { export const value = 1; }\n'],
  ['parameter-property-runtime', 'class Entry { constructor(private value: number) {} }\n'],
  ['mixed-type-import', 'import { type Entry, value } from "./entry.js";\n'],
];

export function typeScriptErasureFixture() {
  return { schemaVersion: 1, generatedBy: 'js/scripts/generate-typescript-erasure-fixtures.mjs',
    rules: 'js/src/data/typescript-erasure.lino',
    cases: inputs.map(([id, source]) => {
      let erased;
      let error;
      try { erased = eraseTypeScript(source); } catch (failure) { error = { reason: failure.reason, span: failure.span }; }
      return { id, source, ...(erased ? { erased, typing: projectTypeScriptTyping(source).code } : { error }),
        javascript: selfTranslate(source, 'TypeScript', 'JavaScript').code,
        ...(['annotated-function', 'annotated-string-function', 'annotated-boolean-function'].includes(id)
          ? { rust: selfTranslate(source, 'TypeScript', 'Rust').code } : {}) };
    }) };
}

const path = new URL('../../parity/fixtures/typescript-erasure.json', import.meta.url);
const fixture = typeScriptErasureFixture();
if (process.argv.includes('--check')) {
  checkProjectData(path, fixture);
} else writeProjectData(path, fixture);
