// Runs the JavaScript and Rust `meta-language grammar` commands over the shared
// importer corpus and prints every disagreement in exit status, standard output
// or standard error. Usage: node experiments/interchange-cli-differential.mjs
// [path to the Rust binary, default rust/target/debug/meta-language].
import { spawnSync } from 'node:child_process';
import { mkdtempSync, readFileSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';

const binary = process.argv[2] ?? 'rust/target/debug/meta-language';
const corpus = JSON.parse(readFileSync('parity/fixtures/grammar-importers.json', 'utf8'));
const directory = mkdtempSync(join(tmpdir(), 'interchange-'));
const extension = { abnf: 'abnf', bnf: 'bnf', ebnf: 'ebnf', pest: 'pest', 'tree-sitter-json': 'json' };

function run(command, args) {
  const result = spawnSync(command[0], [...command.slice(1), 'grammar', ...args], { encoding: 'utf8' });
  return { exitCode: result.status, stdout: result.stdout, stderr: result.stderr };
}

const runs = [[], ['help'], ['formats'], ['bogus'], ['import'], ['import', '--from', 'xyz', 'f'], ['import', '--from', 'bnf', '/missing']];
for (const testCase of corpus.cases) {
  const file = join(directory, `${testCase.id.replace(':', '-')}.${extension[testCase.format]}`);
  writeFileSync(file, testCase.source);
  const native = `${file}.native`;
  runs.push(['import', '--from', testCase.format, file]);
  runs.push(['validate', '--from', testCase.format, file]);
  runs.push(['round-trip', '--from', testCase.format, ...testCase.accepts.flatMap((a) => ['--accept', a]), ...testCase.rejects.flatMap((r) => ['--reject', r]), file]);
  for (const to of ['abnf', 'bnf', 'ebnf', 'gbnf', 'pest', 'tree-sitter-json', 'native']) runs.push(['convert', '--from', testCase.format, '--to', to, file]);
  const imported = run(['node', 'js/src/cli.js'], ['import', '--from', testCase.format, file]);
  writeFileSync(native, imported.stdout);
  runs.push(['export', '--to', 'native', native]);
  runs.push(['round-trip', '--from', 'native', native]);
  runs.push(['rename', '--from', testCase.format, '--rule', testCase.start, '--name', 'renamed', file]);
}
const files = corpus.cases.map((c) => `${c.format}:${join(directory, `${c.id.replace(':', '-')}.${extension[c.format]}`)}`);
runs.push(['merge', '--source', files[0], '--source', files[1]]);
runs.push(['merge', ...files.flatMap((f) => ['--source', f])]);
for (const entry of corpus.malformed) {
  const file = join(directory, `malformed-${entry.format}-${runs.length}.${extension[entry.format]}`);
  writeFileSync(file, entry.source);
  runs.push(['import', '--from', entry.format, file]);
}
let differences = 0;
for (const args of runs) {
  const js = run(['node', 'js/src/cli.js'], args);
  const rust = run([binary], args);
  for (const key of ['exitCode', 'stdout', 'stderr']) {
    if (js[key] !== rust[key]) {
      differences += 1;
      console.log(`--- ${args.join(' ')} [${key}]\njs:   ${JSON.stringify(js[key])}\nrust: ${JSON.stringify(rust[key])}`);
    }
  }
}
console.log(`${runs.length} runs, ${differences} differences`);
