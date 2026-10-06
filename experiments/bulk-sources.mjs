// Experiment: where each inventory grammar's tree-sitter grammar.json is.
import { existsSync, readdirSync } from 'node:fs';
import { join } from 'node:path';
import { GRAMMAR_SOURCES, cargoLockVersions, grammarSource } from '../js/scripts/build-vendored-grammars.mjs';
const versions = await cargoLockVersions();
for (const id of Object.keys(GRAMMAR_SOURCES)) {
  try {
    const s = await grammarSource(id, versions);
    const dir = join(s.crateDir, s.dir ?? '.', 'src');
    console.log(id, existsSync(join(dir, 'grammar.json')) ? 'grammar.json' : `MISSING ${readdirSync(dir).join(',')}`);
  } catch (e) { console.log(id, 'ERR', e.message); }
}
