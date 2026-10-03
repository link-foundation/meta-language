// Times automatic recovery on large JSON inputs with 0, 1, 10 and 100 errors.
import { readFileSync } from 'node:fs';
import { compileGrammar, parseGrammarLinks } from '../js/src/index.js';

const links = readFileSync(new URL('../parity/grammars/native/json.lino', import.meta.url), 'utf8');
const parser = compileGrammar(parseGrammarLinks(links), { errorRecovery: true });
const items = Array.from({ length: 2000 }, (_, index) => `{"k${index}": [${index}, true, "v"]}`);
for (const errors of [0, 1, 10, 100]) {
  const broken = items.map((item, index) => (errors && index % Math.floor(items.length / errors) === 0 ? item.replace(':', '') : item));
  const source = `[${broken.join(',\n')}]`;
  const started = performance.now();
  const outcome = parser.parseTree(source);
  console.log(errors, 'errors', source.length, 'bytes', outcome.rejection?.reason ?? 'ok', (performance.now() - started).toFixed(0), 'ms');
}
