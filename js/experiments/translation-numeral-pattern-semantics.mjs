// Runs the large-numeral pattern cases through the checked pipeline and
// compares the emitted JavaScript's `f` against a reference on many inputs.
import { runStages } from '../scripts/translation-stage-lib.mjs';
import cases from '../tests/fixtures/translation-cases/check.mjs';

const reference = {
  'lean-large-numeral-before-succ': (n) => (n === 1000n ? 7n : n === 0n ? 1n : n >= 2n ? n - 2n : 3n),
  'lean-large-numeral-after-succ': (n, m) => (n === 0n ? 1n : n >= 3n && m === 0n ? n - 3n : n === 10n ** 24n ? m : m + 1n),
  'lean-large-numeral-unreachable-zero': (n) => (n === 17n ? 2n : n === 0n ? 1n : n - 1n),
  'lean-large-numeral-redundant': (n) => (n === 0n ? 1n : n - 1n),
  'lean-offset-at-limit': (n) => (n >= 16n ? n - 16n : 0n),
  'rocq-large-numeral-mixed': (n) => (n >= 2n ? n - 2n : n === 0n ? 1n : 2n),
};
const inputs = [0n, 1n, 2n, 3n, 4n, 15n, 16n, 17n, 18n, 99n, 999n, 1000n, 1001n, 10n ** 24n, 10n ** 24n + 1n];
for (const [name, [extension, source]] of Object.entries(cases)) {
  if (!/numeral|offset/u.test(name)) continue;
  const stages = runStages(extension, source);
  const failed = stages.parse.error ?? stages.check?.error;
  if (failed) {
    console.log(name, 'rejected:', JSON.stringify(failed));
    continue;
  }
  const js = stages.emit.javascript;
  if (js.error) {
    console.log(name, 'emit failed', JSON.stringify(js.error));
    continue;
  }
  const text = js.value.text.replace(/^main\(\);?$/mu, '').replace(/\bmain\(\);/u, '');
  const f = new Function(`${text}\nreturn f;`)();
  let mismatches = 0;
  for (const n of inputs) {
    for (const m of f.length > 1 ? [0n, 5n] : [undefined]) {
      const expected = reference[name](n, m);
      const actual = f(n, m);
      if (actual !== expected) {
        mismatches += 1;
        console.log(' ', name, n, m, 'expected', expected, 'got', actual);
      }
    }
  }
  console.log(name, mismatches ? `${mismatches} mismatches` : 'matches the reference', `(${text.length} chars)`);
}
