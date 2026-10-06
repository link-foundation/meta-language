// Prints scopes, bindings and unresolved references of one program.
// Usage: node experiments/issue-195-bindings.mjs <Language> <file | -e source>
import { readFileSync } from 'node:fs';
import { analyzeProgram } from '../js/src/program-representation.js';

const [language, flag, value] = process.argv.slice(2);
const source = flag === '-e' ? value : readFileSync(flag, 'utf8');
const program = analyzeProgram(source, language, {});
const show = ({ start, end }) => JSON.stringify(source.slice(start, end));
console.log('scopes', program.scopes.map((scope) => `${scope.id}<${scope.parent ?? ''}> ${scope.kind ?? ''} ${scope.start}..${scope.end}`));
for (const binding of program.bindings) {
  console.log('binding', binding.id, binding.kind, binding.name, 'scope', binding.scope, 'refs', binding.references.map(show).length);
}
console.log('unresolved', program.unresolvedReferences.map(({ name, start }) => `${name}@${start}`).join(' '));
console.log('modules', program.modules.map(({ kind, name }) => `${kind}:${name}`).join(' '));
