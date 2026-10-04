// Usage: node experiments/issue-195-project-fixture.mjs [--write]
// Builds the `projectPrograms` section of the four-language fixture from the
// sample projects in experiments/issue-195-projects/<language>, printing the
// observed project-aware results for review. With --write, stores them as the
// expected results in parity/fixtures/four-language-conformance.json.
import { readFileSync, readdirSync, statSync, writeFileSync } from 'node:fs';
import path from 'node:path';
import { analyzeProgram } from '../js/src/index.js';

const repository = new URL('../', import.meta.url);
const fixturePath = new URL('parity/fixtures/four-language-conformance.json', repository);
const projects = [
  { language: 'JavaScript', directory: 'javascript', entry: 'main.js', dependencies: ['node:assert/strict'] },
  { language: 'Rust', directory: 'rust', entry: 'src/main.rs', dependencies: [] },
  { language: 'Lean', directory: 'lean', entry: 'Main.lean', dependencies: [] },
  { language: 'Rocq', directory: 'rocq', entry: 'Main.v', dependencies: [] },
];
// Each broken context replaces project files of a valid project.
const brokenContexts = {
  JavaScript: [{
    description: 'util.js no longer exports sumTo',
    replace: { 'util.js': (text) => text.replace('export function sumTo', 'function sumTo') },
  }],
  Rust: [{
    description: 'util::double is private',
    replace: { 'src/util.rs': (text) => text.replace('pub const fn double', 'const fn double') },
  }],
  Lean: [{
    description: 'lakefile.toml declares no Util library',
    replace: { 'lakefile.toml': (text) => text.replace('[[lean_lib]]\nname = "Util"\n\n', '') },
  }],
  Rocq: [{
    description: '_CoqProject maps no load path to Util',
    replace: { _CoqProject: (text) => text.replace('-Q theories Util\n', '') },
  }],
};

const walk = (directory, prefix = '') => readdirSync(directory).sort().flatMap((name) => {
  const full = path.join(directory, name);
  return statSync(full).isDirectory() ? walk(full, `${prefix}${name}/`) : [`${prefix}${name}`];
});

const analyze = (project, sources) => {
  const entry = sources.find(({ path: file }) => file === project.entry).source;
  return analyzeProgram(entry, project.language, {
    root: '/workspace', entry: project.entry, sources, files: sources.map(({ path: file }) => file), dependencies: project.dependencies,
  });
};

const programs = projects.map((project) => {
  const directory = new URL(`experiments/issue-195-projects/${project.directory}/`, repository);
  const sources = walk(directory.pathname).map((file) => ({ path: file, source: readFileSync(new URL(file, directory), 'utf8') }));
  const text = (file, start, end) => sources.find(({ path: item }) => item === file).source.slice(start, end);
  const entrySource = sources.find(({ path: file }) => file === project.entry).source;
  const program = analyze(project, sources);
  const bare = analyzeProgram(entrySource, project.language, { root: '/workspace', entry: project.entry, sources: [], dependencies: project.dependencies });
  if (program.diagnostics.length > 0) throw new Error(`${project.language}: ${JSON.stringify(program.diagnostics)}`);
  return {
    language: project.language,
    root: '/workspace',
    entry: project.entry,
    dependencies: project.dependencies,
    sources,
    modules: program.projectModules.map(({ request, module, start, end }) => ({ request, module, text: entrySource.slice(start, end) })),
    missingContext: bare.diagnostics.map(({ kind, term, start, end }) => ({ kind, term, text: entrySource.slice(start, end) })),
    brokenContexts: brokenContexts[project.language].map(({ description, replace }) => {
      const replaced = sources.map((item) => (replace[item.path] ? { path: item.path, source: replace[item.path](item.source) } : item));
      const broken = analyze(project, replaced);
      return {
        description,
        sources: replaced.filter((item) => replace[item.path]),
        diagnostics: broken.diagnostics.map(({ kind, term, start, end }) => ({ kind, term, text: entrySource.slice(start, end) })),
      };
    }),
    constructs: Object.fromEntries(program.constructs.map(({ kind, evidence }) => [kind, evidence
      .filter((item) => item.file !== undefined)
      .map(({ kind: evidenceKind, name, file, start, end }) => ({ kind: evidenceKind, name, file, text: text(file, start, end) }))])),
    expansions: program.expansions.map(({ name, kind, start, end, expansion, target }) => ({ name, kind, text: entrySource.slice(start, end), expansion, target })),
  };
});

console.log(JSON.stringify(programs.map(({ sources, ...rest }) => rest), null, 1));
if (process.argv.includes('--write')) {
  // The section is kept last, so the rest of the hand-formatted file is untouched.
  const text = readFileSync(fixturePath, 'utf8');
  const marker = ',\n  "projectPrograms": ';
  const head = text.includes(marker) ? text.slice(0, text.indexOf(marker)) : text.slice(0, text.lastIndexOf('\n}'));
  const section = JSON.stringify(programs, null, 2).split('\n').join('\n  ');
  writeFileSync(fixturePath, `${head}${marker}${section}\n}\n`);
}
