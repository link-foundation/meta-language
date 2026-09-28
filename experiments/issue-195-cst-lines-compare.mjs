// Compares the canonical CST lines of the native CLI with the public parse for one file.
//   node experiments/issue-195-cst-lines-compare.mjs LANGUAGE GRAMMAR_DIR FILE
import { execFileSync } from 'node:child_process';
import { readFileSync } from 'node:fs';
import { LinkNetwork } from '../js/src/index.js';
import { cliCstToLines, diagnosticProblems, documentGrammarRoots, renderCstLines, triviaProblems }
  from '../js/tests/support/cst-lines.js';

const [language, grammarDir, file] = process.argv.slice(2);
const cli = process.env.TREE_SITTER_CLI ?? '/tmp/tscli/ts';
let output;
try {
  output = execFileSync(cli, ['parse', '--cst', file], { cwd: grammarDir, encoding: 'utf8', maxBuffer: 1 << 28 });
} catch (error) {
  output = error.stdout;
}
const source = readFileSync(file, 'utf8');
const expected = cliCstToLines(output, source);
const network = LinkNetwork.parse(source, language);
const { text, rendered } = renderCstLines(documentGrammarRoots(network, language), language);
if (text === expected) console.log('CST equal');
else {
  const a = text.split('\n');
  const b = expected.split('\n');
  const index = a.findIndex((line, position) => line !== b[position]);
  console.log(`CST differs at line ${index}\n  actual   ${a[index]}\n  expected ${b[index]}`);
}
console.log('trivia', triviaProblems(network, source, expected));
console.log('diagnostics', diagnosticProblems(network, rendered, expected));
