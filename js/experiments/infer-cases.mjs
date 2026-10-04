import { translateProgram } from '../src/program-translation.js';
const cases = {
  conflict: "function id(x) { return x; }\nconsole.log(id(1));\nconsole.log(id('a'));",
  bigMix: "function f(x) { return x + 1n; }\nconsole.log(f(1));",
  data: "/** @typedef {{ $: 'leaf' } | { $: 'node', left: Tree, value: bigint, right: Tree }} Tree */\nfunction size(t) { switch (t.$) { case 'leaf': return 0n; case 'node': return size(t.left) + 1n + size(t.right); } }\nconsole.log(size({ $: 'node', left: { $: 'leaf' }, value: 3n, right: { $: 'leaf' } }));",
  ns: "const M = { double(x) { return x * 2; }, quad(x) { return M.double(M.double(x)); } };\nconsole.log(M.quad(1.25));",
  strPlus: "function label(n) { return n + '!'; }\nconsole.log(label(3));",
  unused: "function k(x) { return 1n; }\nconsole.log(k(2n));",
  guardNumber: "function g(n) { if (n < 0) throw new RangeError('neg'); return n; }\nconsole.log(g(3));",
};
for (const [name, src] of Object.entries(cases)) {
  for (const t of ['Rust', 'Lean']) {
    try {
      const r = translateProgram(src, 'JavaScript', t);
      console.log(name, t, r.contract.support, r.contract.support === 'portable-encoding' ? r.contract.obligations?.[0] ?? JSON.stringify(r.contract).slice(0, 300) : r.code.split('\n').filter((l) => /^(pub fn|def) (id|f|size|double|quad|label|k|g)\b/.test(l.trim())).join(' | '));
    } catch (e) { console.log(name, t, 'ERR', e.message); }
  }
}
