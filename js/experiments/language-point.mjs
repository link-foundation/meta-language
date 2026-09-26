// Prints the Language point term the JS runtime records for each alias.
import { readFileSync } from 'node:fs';
import { LinkNetwork } from '../src/index.js';

const inventory = JSON.parse(readFileSync(new URL('../../parity/language-grammar-inventory.json', import.meta.url)));
for (const language of inventory.languages) {
  for (const alias of language.aliases) {
    const network = LinkNetwork.parse(language.source, alias);
    const point = network.links().find((link) => link.metadata().linkType === 'Language');
    console.log(`${language.name}\t${alias}\t${point?.metadata().term}\t${network.language ?? ''}`);
  }
}
