// One-off: copies the validated draft sources into the grammar inventory as
// each language's positive `source`, its `recoverySource` and the vendored
// grammar ids that parse it.
//   node experiments/apply-default-cst-drafts.mjs
import { readFile, writeFile } from 'node:fs/promises';

import { DRAFTS } from './default-cst-source-drafts.mjs';
import { formatInventoryJson } from '../scripts/inventory-json.mjs';

const url = new URL('../../parity/language-grammar-inventory.json', import.meta.url);
const inventory = JSON.parse(await readFile(url, 'utf8'));
inventory.languages = inventory.languages.map((language) => {
  const draft = DRAFTS[language.name];
  if (!draft) return language;
  const { name, aliases, family, javascript, rust } = language;
  const grammars = draft.grammar === 'markdown' ? ['markdown', 'markdown_inline'] : [draft.grammar];
  return { name, aliases, family, grammars, source: draft.source, recoverySource: draft.recovery, javascript, rust };
});
await writeFile(url, formatInventoryJson(inventory));
