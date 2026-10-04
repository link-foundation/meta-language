// Converts every pinned tree-sitter grammar.json in the cargo registry and
// reports its size, externals, keywords and what the conversion could not
// represent, one grammar at a time.
//   node experiments/tree-sitter-native-survey.mjs
import { readdirSync, readFileSync, existsSync, statSync } from 'node:fs';
import { homedir } from 'node:os';
import path from 'node:path';

import { compileGrammar } from '../src/grammar.js';
import { parseGrammarLinks } from '../src/grammar-links.js';
import { convertTreeSitterGrammar } from './tree-sitter-native-convert.mjs';

const registry = path.join(homedir(), '.cargo/registry/src');
const files = [];
for (const index of readdirSync(registry)) {
  for (const crate of readdirSync(path.join(registry, index))) {
    if (!crate.startsWith('tree-sitter-')) continue;
    const walk = (dir, depth) => {
      if (depth > 3) return;
      for (const entry of readdirSync(dir)) {
        const full = path.join(dir, entry);
        if (entry === 'grammar.json') files.push({ crate, file: full });
        else if (statSync(full).isDirectory() && !['node_modules', 'bindings', 'queries', 'test'].includes(entry)) walk(full, depth + 1);
      }
    };
    walk(path.join(registry, index, crate), 0);
  }
}
for (const { crate, file } of files) {
  const grammar = JSON.parse(readFileSync(file, 'utf8'));
  const row = [crate, path.relative(path.join(registry), file).split('/').slice(2, -1).join('/'), `${Math.round(statSync(file).size / 1024)}K`, `${Object.keys(grammar.rules).length}r`, `${(grammar.externals ?? []).length}x`];
  try {
    const { text, report, keywords } = convertTreeSitterGrammar(grammar);
    row.push(`${keywords.length}kw`, `unsupported=${report.unsupported.length}`);
    try { compileGrammar(parseGrammarLinks(text)); row.push('compiles'); } catch (error) { row.push(`COMPILE ${error.message.slice(0, 80)}`); }
    if (report.unsupported.length) row.push(report.unsupported.filter((u) => !u.startsWith('external')).slice(0, 3).join(' | '));
  } catch (error) { row.push(`CONVERT ${error.message.slice(0, 120)}`); }
  console.log(row.join(' '));
}
