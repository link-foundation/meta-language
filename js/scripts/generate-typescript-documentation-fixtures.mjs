#!/usr/bin/env node
// Bounded shared projection cases; expected values are independent of the projection.
import { checkProjectData, writeProjectData } from './project-data.mjs';
import { projectTypeScriptTyping } from '../src/typescript-portable-typing.js';

const inputs = [
  { id: 'destructuring-comment', source: 'function example({value/*name*/}: any): any { return value; }',
    invocation: 'example({value:"retained"})', expectedValue: 'retained' },
  { id: 'quoted-comment-terminator', source: 'function example({"*/": value}: any): any { return value; }',
    invocation: 'example({"*/":"retained"})', expectedValue: 'retained' },
  { id: 'existing-escaped-slash', source: String.raw`function example({"*\/": value}: any): any { return value; }`,
    invocation: 'example({"*/":"retained"})', expectedValue: 'retained' },
  { id: 'Unicode-alias-and-carriage-returns', source: 'type 类型 = number;\r\nfunction 示例(输入: 类型): 类型 { return 输入; }\r\nfunction next(value: string): string { return value; }\r\n',
    invocation: '示例(7) + next("retained").length', expectedValue: 15 },
  { id: 'nested-typed-functions', source: 'function outer(值: string): string { function inner(项: string): string { return 项; } return inner(值); }',
    invocation: 'outer("retained")', expectedValue: 'retained' },
  { id: 'multiline-type', source: 'function example(value: (\r\nstring\r\n)): string { return value; }',
    invocation: 'example("retained")', expectedValue: 'retained' },
  { id: 'annotation-comment-terminator-refusal', source: 'function example(value: "*/"): string { return value; }',
    expectedError: 'TypeScript type cannot be represented as a portable documentation annotation' },
];

export function typeScriptDocumentationFixture() {
  return {
    schemaVersion: 1, generatedBy: 'js/scripts/generate-typescript-documentation-fixtures.mjs',
    documentationPatternEncoding: 'A literal */ becomes *\\/ only in supplemental documentation pattern text; original source text and byte spans remain authoritative.',
    cases: inputs.map((entry) => {
      if (entry.expectedError) return { ...entry };
      const projection = projectTypeScriptTyping(entry.source);
      return { ...entry, projectedSource: projection.code, projections: projection.projections, erasures: projection.erasures };
    }),
  };
}

const output = new URL('../../parity/fixtures/typescript-documentation-patterns.lino', import.meta.url);
if (process.argv.includes('--check')) checkProjectData(output, typeScriptDocumentationFixture());
else writeProjectData(output, typeScriptDocumentationFixture());
