// Reuse a composed source snapshot while debugging author-time type errors.
// This experiment neither executes programs nor publishes generated products.
import { readFileSync, writeFileSync } from 'node:fs';
import { readSourceLibraryClosure } from '../../js/scripts/read-source-library-closure.mjs';
import { parseJavaScript } from '../../js/src/translation/javascript.js';
import { checkProgram } from '../../js/src/translation/check.js';

const [mode, snapshot, ...rest] = process.argv.slice(2);
if (!['compose', 'check'].includes(mode) || !snapshot || rest.length) {
  throw new TypeError('use compose or check followed by a scratch snapshot path');
}

if (mode === 'compose') {
  const { source } = readSourceLibraryClosure(
    'js/src/translation/javascript-call-machine.js',
    { aliases: true }
  );
  writeFileSync(snapshot, source, { flag: 'wx' });
}

const source = readFileSync(snapshot, 'utf8');
function countContainers(root, visited = new WeakSet()) {
  if (!root || typeof root !== 'object' || visited.has(root)) return 0;
  visited.add(root);
  let count = 1;
  for (const child of Object.values(root)) count += countContainers(child, visited);
  return count;
}

function countExactStructures(root) {
  const identities = new WeakMap();
  const structures = new Map();
  function identify(value) {
    if (!value || typeof value !== 'object') {
      return [typeof value, typeof value === 'bigint' ? String(value) :
        typeof value === 'number' ? Object.is(value, -0) ? '-0' : String(value) : value];
    }
    if (identities.has(value)) return ['object', identities.get(value)];
    const key = JSON.stringify([
      Array.isArray(value),
      Object.entries(value).map(([field, child]) => [field, identify(child)]),
    ]);
    if (!structures.has(key)) structures.set(key, structures.size);
    const identity = structures.get(key);
    identities.set(value, identity);
    return ['object', identity];
  }
  identify(root);
  return structures.size;
}

try {
  const started = performance.now();
  const parsed = parseJavaScript(source);
  console.log(`Parsed and inferred source in ${Math.round(performance.now() - started)} ms`);
  console.log(`Surface stores ${countContainers(parsed)} distinct containers`);
  const sizes = parsed.items.map(item => [item.name ?? item.k, countContainers(item)]);
  console.table(sizes.sort((left, right) => right[1] - left[1]).slice(0, 24));
  console.log(`Surface has ${countExactStructures(parsed)} distinct enumerable structures`);
  const program = checkProgram(parsed);
  console.log(`Checked ${program.declarations.size} authored declarations in ${snapshot}`);
} catch (error) {
  const { start, end } = error.span ?? {};
  if (Number.isSafeInteger(start) && Number.isSafeInteger(end) && start >= 0 && end <= source.length) {
    console.error(source.slice(Math.max(0, start - 350), Math.min(source.length, end + 350)));
    console.error(`Failing source at ${start}..${end}`, JSON.stringify(source.slice(start, end)));
  }
  throw error;
}
