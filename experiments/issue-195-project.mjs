// Usage: node experiments/issue-195-project.mjs <Language> <project-dir> <entry> [--no-sources]
// Analyzes an entry file with every other project file as a source and
// prints the project-aware results.
import { readFileSync, readdirSync, statSync } from 'node:fs';
import path from 'node:path';
import { analyzeProgram } from '../js/src/index.js';

const [language, directory, entry, flag] = process.argv.slice(2);
const walk = (dir) => readdirSync(dir).flatMap((name) => {
  const full = path.join(dir, name);
  if (statSync(full).isDirectory()) return ['.lake', 'target', 'node_modules'].includes(name) ? [] : walk(full);
  return /\.(vo|vok|vos|glob|aux|olean|ilean)$|^\./u.test(name) || name === 'lake-manifest.json' || name === 'Cargo.lock' ? [] : [full];
});
const sources = flag === '--no-sources' ? [] : walk(directory).map((full) =>
  ({ path: path.relative(directory, full), source: readFileSync(full, 'utf8') }));
const source = readFileSync(path.join(directory, entry), 'utf8');
const program = analyzeProgram(source, language, { root: directory, entry, sources });
const text = (file, start, end) => (file === entry ? source : sources.find((item) => item.path === file)?.source ?? '').slice(start, end);
console.log('modules', program.projectModules);
for (const fact of program.projectFacts) console.log('fact', fact.kind, fact.name, fact.file, JSON.stringify(text(fact.file, fact.start, fact.end)).slice(0, 50));
for (const ref of program.projectReferences) {
  console.log('ref', ref.role, ref.name, JSON.stringify(source.slice(ref.start, ref.end)), ref.symbol, ref.targetKind, ref.traits.join(','), JSON.stringify(text(ref.file, ref.declaration.start, ref.declaration.end)).slice(0, 40));
}
for (const expansion of program.expansions) console.log('expansion', expansion, JSON.stringify(source.slice(expansion.start, expansion.end)));
console.log('diagnostics', program.diagnostics.map(({ kind, term }) => `${kind}:${term}`));
for (const construct of program.constructs) console.log('construct', construct.kind, construct.status, construct.evidence.filter((item) => item.file !== undefined).length);
