import { translateProgram } from '../src/program-translation.js';
const cases = process.argv.slice(2);
for (const source of cases) {
  for (const target of ['Rust', 'Lean', 'Rocq']) {
    const t = translateProgram(source, 'JavaScript', target);
    console.log(target, t.diagnostic?.message ?? t.contract.support);
    if (!t.diagnostic && process.env.CODE) console.log(t.code.split('\n').filter((l) => /main|Math|ml_|let|Definition main|def main/u.test(l)).slice(-8).join('\n'));
  }
}
