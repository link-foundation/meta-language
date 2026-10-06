import { analyzeProgram } from '../js/src/index.js';

const source = 'function f() { if (true) { var x = 1; } return x; } globalThis.result = f();';
const program = analyzeProgram(source, 'JavaScript');
console.log(JSON.stringify({
  scopes: program.scopes,
  bindings: program.bindings,
  unresolvedReferences: program.unresolvedReferences,
  syntax: program.sourceMappings
    .filter(({ term }) => /function|arrow|method|statement_block/u.test(term))
    .map(({ term, start, end }) => ({ term, start, end })),
}, null, 2));
