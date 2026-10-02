// Parses one inventory language repeatedly, idles so background TurboFan
// tier-up can run, and reports the resident memory before and after idling.
// Usage: node per-language-tierup.mjs <language name>
import { readFileSync } from 'node:fs';
import { LinkNetwork } from '../../js/src/index.js';

const inventory = JSON.parse(readFileSync(new URL('../../parity/language-grammar-inventory.json', import.meta.url), 'utf8'));
const language = inventory.languages.find(({ name }) => name === process.argv[2]);
const mb = () => Math.round(process.memoryUsage().rss / 1048576);
const before = mb();
for (let round = 0; round < 20; round += 1) LinkNetwork.parse(language.source, language.name);
const parsed = mb();
await new Promise((resolve) => setTimeout(resolve, 1500));
console.log(`${language.name}\t${before}\t${parsed}\t${mb()}`);
