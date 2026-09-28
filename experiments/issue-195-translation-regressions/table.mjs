// Runs the JavaScript → Rust regressions from the 2026-09-28 audit through the
// public translator and prints the translation kind each one gets.
//   node experiments/issue-195-translation-regressions/table.mjs [target]
import { translateProgram } from '../../js/src/program-translation.js';

const target = process.argv[2] ?? 'rust';
const cases = [
  'console.log(6n * 7n);\n',
  'console.log(42);',
  'console.log(42);\n',
  'console.log(6 * 7);',
  'function answer() { return 42; } console.log(answer());',
  'const inc = x => x + 1; console.log(inc(41));',
  'async function answer() { return 42; } console.log(await answer());',
];
for (const source of cases) {
  const result = translateProgram(source, 'javascript', target);
  const kind = result.contract?.support ?? 'none';
  console.log(JSON.stringify(source), '→', kind, result.diagnostic?.message ?? '');
}
