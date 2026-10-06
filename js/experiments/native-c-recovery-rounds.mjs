// Counts the executor steps of a plain parse and of each automatic repair
// round for a native grammar, with a large step limit, to see which round
// exhausts the default budget.
//   node experiments/native-c-recovery-rounds.mjs [GRAMMAR.lino] SOURCE...
import { readFileSync } from 'node:fs';
import { Executor } from '../src/grammar-runtime/executor.js';
import { loadProgram } from '../src/grammar-runtime/load.js';
import { parseGrammarLinks } from '../src/index.js';

const args = process.argv.slice(2);
const grammarPath = args[0]?.endsWith('.lino') ? args.shift() : new URL('../../parity/grammars/native/c.lino', import.meta.url);
const program = loadProgram(parseGrammarLinks(readFileSync(grammarPath, 'utf8')), {});
for (const source of args) {
  const bytes = Buffer.from(source, 'utf8');
  const round = (points) => {
    const budget = { steps: 0, limit: 50_000_000 };
    const executor = new Executor(program, bytes, 0, bytes.length, {}, budget, 1000);
    executor.repairPoints = points;
    const started = performance.now();
    const outcome = executor.run(program.start);
    return { outcome, steps: budget.steps, ms: Math.round(performance.now() - started) };
  };
  const points = new Set();
  let { outcome, steps, ms } = round(points);
  console.log(JSON.stringify(source), 'plain', steps, 'steps', ms, 'ms ok', outcome.ok, 'farthest', outcome.farthest, 'elementFarthest', outcome.elementFarthest);
  while (!outcome.ok && points.size < 32) {
    const point = outcome.elementFarthest >= 0 ? outcome.elementFarthest : outcome.farthest;
    if (points.has(point)) break;
    points.add(point);
    ({ outcome, steps, ms } = round(new Set(points)));
    console.log('  round', [...points].join(','), steps, 'steps', ms, 'ms ok', outcome.ok, 'elementFarthest', outcome.elementFarthest);
  }
}
