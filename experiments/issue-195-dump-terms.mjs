// Usage: node experiments/issue-195-dump-terms.mjs <Language> <source-file | -e source>
// Prints every syntax node term with its exact source text, to design
// term-driven project analysis.
import { readFileSync } from 'node:fs';
import { analyzeProgram } from '../js/src/program-representation.js';

const [language, flag, value] = process.argv.slice(2);
const source = flag === '-e' ? value.replaceAll('\\n', '\n') : readFileSync(flag, 'utf8');
const program = analyzeProgram(source, language);
for (const { term, start, end } of program.sourceMappings) {
  console.log(`${term} [${start},${end}] ${JSON.stringify(source.slice(start, end)).slice(0, 80)}`);
}
