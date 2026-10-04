// Parses every inventory language, then idles and reports resident memory
// every 250 ms, to see whether memory grows after the parsing work ends.
import { readFileSync } from 'node:fs';
import { LinkNetwork } from '../../js/src/index.js';

const inventory = JSON.parse(readFileSync(new URL('../../parity/language-grammar-inventory.json', import.meta.url), 'utf8'));
const mb = () => Math.round(process.memoryUsage().rss / 1048576);
for (const language of inventory.languages) {
  if (language.source) LinkNetwork.parse(language.source, language.name);
}
console.log('parsed', mb());
for (let tick = 0; tick < 16; tick += 1) {
  await new Promise((resolve) => setTimeout(resolve, 250));
  console.log('idle', tick, mb());
}
