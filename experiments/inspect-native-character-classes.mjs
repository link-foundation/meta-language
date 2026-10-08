// Compare the native importer's diagnostics before and after this regression
// fix, without running the catalog or an upstream corpus. The finite input is
// the shared character-class fixture. Run from the repository root:
//   node experiments/inspect-native-character-classes.mjs [baseline-commit]
import { execFileSync } from 'node:child_process';
import { readFileSync } from 'node:fs';
import { importTreeSitterNative } from '../js/src/grammar-importers/tree-sitter-native.js';

const baseline = process.argv[2] ?? '2df5bf793990e6e6503d9b479ad1fb54a697a4c4';
const moduleUrl = new URL('../js/src/grammar-importers/tree-sitter-native.js', import.meta.url);
const source = execFileSync('git', ['show', `${baseline}:js/src/grammar-importers/tree-sitter-native.js`], { encoding: 'utf8' });
const standalone = source.replace(/from (['"])(\.{1,2}\/[^'"]+)\1/gu,
  (_, quote, dependency) => `from ${quote}${new URL(dependency, moduleUrl).href}${quote}`);
const previous = await import(`data:text/javascript;base64,${Buffer.from(standalone).toString('base64')}`);
const fixture = JSON.parse(readFileSync(new URL('../parity/fixtures/native-character-classes.json', import.meta.url), 'utf8'));
const reports = fixture.cases.map(({ id, pattern }) => {
  const grammar = { name: 'character_classes', rules: { source_file: { type: 'PATTERN', value: pattern } } };
  return { id, before: previous.importTreeSitterNative(grammar).report.unsupported, after: importTreeSitterNative(grammar).report.unsupported };
});
console.log(JSON.stringify({ baseline, reports }, null, 2));
if (reports.some(({ after }) => after.length > 0)) process.exitCode = 1;
