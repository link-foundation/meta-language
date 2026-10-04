// Probes which semantic re-checks hold for the shared `commands` of
// parity/fixtures/grammar-importers.json: each successful command's output is
// re-imported in the output format and run over the samples of its case.
import { readFileSync } from 'node:fs';

import { grammarImporter, parseNativeGrammar, parseWithGrammar } from '../js/src/index.js';

const corpus = JSON.parse(readFileSync(new URL('../parity/fixtures/grammar-importers.json', import.meta.url), 'utf8'));
const accepts = (grammar, text) => {
  try {
    parseWithGrammar(grammar, text);
    return true;
  } catch {
    return false;
  }
};
for (const command of corpus.commands.filter(({ exitCode }) => exitCode === 0)) {
  const caseIds = Object.values(command.files).flatMap((value) => (typeof value === 'object' && 'case' in value ? [value.case] : []));
  const to = command.args[command.args.indexOf('--to') + 1];
  const format = command.args[0] === 'import' ? 'native' : command.args.includes('--to') ? to : null;
  if (!format || caseIds.length === 0) continue;
  let grammar;
  try {
    grammar = format === 'native' ? parseNativeGrammar(command.stdout) : grammarImporter(format)(command.stdout);
  } catch (error) {
    console.log(command.id, 'reimport failed', error.message);
    continue;
  }
  const kase = corpus.cases.find(({ id }) => id === caseIds[0]);
  const bad = [...kase.accepts.filter((s) => !accepts(grammar, s)).map((s) => `+${s}`), ...kase.rejects.filter((s) => accepts(grammar, s)).map((s) => `-${s}`)];
  console.log(command.id, format, grammar.start ?? grammar.startRule?.name, bad.length ? `MISMATCH ${JSON.stringify(bad)}` : 'ok');
}
