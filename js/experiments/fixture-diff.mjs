import { readProjectData } from '../scripts/project-data.mjs';
import { readFileSync } from 'node:fs';
import { NATIVE_GRAMMARS, buildNativeGrammarFixture, fixturePath } from '../scripts/generate-native-grammar-fixtures.mjs';
const root = new URL('../../', import.meta.url).pathname;
for (const id of process.argv.slice(2)) {
  const entry = NATIVE_GRAMMARS.find((e) => e.id === id);
  const now = buildNativeGrammarFixture(entry);
  const old = readProjectData(root + fixturePath(entry));
  for (const key of ['matches', 'divergences', 'rejections']) {
    const a = old[key] ?? [], b = now[key] ?? [];
    if (a.length !== b.length) console.log(id, key, 'length', a.length, b.length);
    for (let i = 0; i < Math.max(a.length, b.length); i++) if (JSON.stringify(a[i]) !== JSON.stringify(b[i])) console.log(id, key, i, '\nOLD', JSON.stringify(a[i]), '\nNEW', JSON.stringify(b[i]));
  }
}
