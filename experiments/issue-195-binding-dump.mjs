// Prints bindings, references, and unresolved names for one program:
// node experiments/issue-195-binding-dump.mjs <Language> '<source>'
import { analyzeProgram } from '../js/src/index.js';
const [language, source] = process.argv.slice(2);
const program = analyzeProgram(source, language);
console.log('clean', program.network.verifyFullMatch().isClean());
for (const b of program.bindings) {
  console.log(`${b.kind} ${b.name}@${b.declaration.start} ${b.scope} refs=[${b.references.map((r) => r.start).join(',')}]`);
}
console.log('unresolved', program.unresolvedReferences.map((r) => `${r.name}@${r.start}`).join(' '));
