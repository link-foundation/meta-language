// Bound the historical inference loop without altering the checked source.
// From the repository root, run:
// node --max-old-space-size=352 experiments/optional-join-fixed-point/probe.mjs <inference-revision>
import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { registerHooks } from 'node:module';

const [revision] = process.argv.slice(2);
assert.match(revision ?? '', /^[0-9a-f]{7,40}$/u);
const path = 'js/src/translation/javascript-infer.js';
const url = new URL(`../../${path}`, import.meta.url).href;
const original = execFileSync('git', ['show', `${revision}:${path}`], { encoding: 'utf8' });
const entry = '    const before = this.unificationChanges;\n    const constraints = readOptionalJoinConstraints';
assert.equal(original.split(entry).length, 2, 'one actual optional join step');
const instrumented = original.replace(entry, `    globalThis.visitOptionalJoin();\n${entry}`);
let visits = 0;
globalThis.visitOptionalJoin = () => {
  if (++visits > 8192) throw new Error('source inference exceeded 8192 join visits');
};
const hook = registerHooks({
  load(module, context, nextLoad) {
    if (module !== url) return nextLoad(module, context);
    return { source: instrumented, format: 'module', shortCircuit: true };
  },
});
const { parseJavaScript } = await import('../../js/src/translation/javascript.js');
hook.deregister();

const source = `
function select(row, take) { return take ? row?.count : null; }
console.log(select({ count: 4 }, true));
console.log(select(undefined, true));
console.log(select({ count: 4 }, false));`;
try {
  const program = parseJavaScript(source);
  console.log('Inferred result', program.items.find(item => item.name === 'select').ret);
  console.log('Join visits', visits);
} finally {
  delete globalThis.visitOptionalJoin;
}
