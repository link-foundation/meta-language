// Reports whether each file parses cleanly and lists its parse issues.
// Usage: node experiments/issue-195-clean.mjs <Language> <file>...
import { readFileSync } from 'node:fs';
import { analyzeProgram } from '../js/src/program-representation.js';

const [language, ...files] = process.argv.slice(2);
for (const file of files) {
  const source = readFileSync(file, 'utf8');
  const program = analyzeProgram(source, language);
  const issues = program.diagnostics.filter(({ kind }) => kind !== 'missing-project-context');
  console.log(file, issues.length === 0 ? 'clean' : issues.map(({ kind, term, start, end }) => `${kind}:${term}@${start} ${JSON.stringify(source.slice(start, end))}`).join('; '));
}
