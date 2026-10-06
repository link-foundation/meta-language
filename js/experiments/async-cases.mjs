// Async/await cases: each is translated to every target, or rejected with a diagnostic.
import { translateProgram } from '../src/program-translation.js';

const cases = {
  awaited: 'async function answer() { return 42; }\nconsole.log(await answer());\n',
  nested: 'async function inc(n) { return n + 1; }\nasync function two(n) { return await inc(await inc(n)); }\nconsole.log(await two(40));\n',
  returned: 'async function inc(n) { return n + 1; }\nasync function again(n) { return inc(n); }\nconsole.log(await again(41));\n',
  arrow: 'const inc = async (n) => n + 1;\nconsole.log(await inc(41));\n',
  promiseDoc: '/** @param {number} n @returns {Promise<number>} */\nasync function inc(n) { return n + 1; }\nconsole.log(await inc(41));\n',
  unawaited: 'async function answer() { return 42; }\nconsole.log(answer());\n',
  syncCaller: 'async function answer() { return 42; }\nfunction f() { return answer(); }\nconsole.log(f());\n',
  awaitInSync: 'function f(n) { return await n; }\nconsole.log(f(1));\n',
  asyncParam: 'const f = (async) => async + 1;\nconsole.log(f(41));\n',
  awaitValue: 'console.log(await 42);\n',
};
for (const [name, source] of Object.entries(cases)) {
  for (const target of ['Rust', 'Lean', 'Rocq']) {
    try {
      const result = translateProgram(source, 'JavaScript', target);
      console.log(name, target, result.contract?.support ?? 'ok');
      if (process.env.SHOW) console.log(result.code);
    } catch (error) {
      console.log(name, target, 'ERROR', error.message);
      break;
    }
  }
}
