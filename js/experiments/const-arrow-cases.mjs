import { translateProgram } from '../src/program-translation.js';
const cases = [
  'const inc = x => x + 1; console.log(inc(41));',
  '/** @param {number} x @returns {number} */\nconst dbl = (x) => x * 2; console.log(dbl(21));',
  'const fact = function (n) { if (n < 0n) throw new RangeError("neg"); return n === 0n ? 1n : n * fact(n - 1n); }; console.log(String(fact(20n)));',
  'const add = (a, b) => { return a + b; }; export const hi = () => "hi"; console.log(add(1, 2), hi());',
  'console.log(1); const f = x => x;',
  'const f = function g(x) { return x; };',
  'const f = function* () {};',
  'const f = async x => x;',
  'const x = (1 + 2); console.log(x);',
];
for (const source of cases) {
  const r = translateProgram(source, 'JavaScript', 'Rust');
  console.log(JSON.stringify(source), '→', r.contract?.support, r.diagnostic ? `${r.diagnostic.message} at ${r.diagnostic.span?.start}..${r.diagnostic.span?.end}` : '');
}
