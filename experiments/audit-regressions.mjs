// Runs the audit's JavaScript → Rust regression table through the public translator.
import { translateProgram } from '../js/src/program-translation.js';

const cases = [
  'console.log(6n * 7n);\n',
  'console.log(42);',
  'console.log(42);\n',
  'console.log(6 * 7);',
  'function answer() { return 42; } console.log(answer());',
  'const inc = x => x + 1; console.log(inc(41));',
  'async function answer() { return 42; } console.log(await answer());',
  'for (let i = 0; i < 3; i++) console.log(i);',
  'function hello(name) { console.log("hi " + name); } hello("a");',
  'const xs = [1, 2, 3]; console.log(xs.length);',
  'let s = 0; for (const x of [1, 2, 3]) s += x; console.log(s);',
  'console.log("a", 1);',
  'console.log(Math.max(1, 2));',
  'class A { constructor(x) { this.x = x; } } console.log(new A(1).x);',
  'const o = { a: 1 }; console.log(o.a);',
  'console.log(`x=${1 + 2}`);',
];
for (const source of cases) {
  for (const target of ['Rust']) {
    const result = translateProgram(source, 'JavaScript', target);
    console.log(JSON.stringify(source), '→', target, result.contract.support, result.diagnostic ? result.diagnostic.message ?? JSON.stringify(result.diagnostic).slice(0, 200) : '');
  }
}
