// Lists the corpus cases whose native fixture rows differ from a fresh build.
// Usage: node experiments/native-fixture-diff.mjs <id> [<id>...]
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { buildNativeGrammarFixture, fixturePath, NATIVE_GRAMMARS } from '../scripts/generate-native-grammar-fixtures.mjs';

const root = resolve(import.meta.dirname, '../..');
for (const id of process.argv.slice(2)) {
  const entry = NATIVE_GRAMMARS.find((candidate) => candidate.id === id);
  const old = JSON.parse(readFileSync(resolve(root, fixturePath(entry)), 'utf8'));
  const fresh = buildNativeGrammarFixture(entry);
  const key = (list) => new Map((list ?? []).map((match) => [match.source, JSON.stringify(match.rows ?? match)]));
  for (const field of ['matches', 'divergences', 'rejections']) {
    const a = key(old[field]);
    const b = key(fresh[field]);
    for (const [source, rows] of a) if (b.get(source) !== rows) console.log(id, field, 'changed', JSON.stringify(source).slice(0, 200));
    for (const source of b.keys()) if (!a.has(source)) console.log(id, field, 'added', JSON.stringify(source).slice(0, 200));
  }
}
