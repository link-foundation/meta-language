// Prints every canonical name the naming inventories read from the sources.
import { extractNameInventory } from '../js/scripts/issue-195-naming.mjs';
const names = extractNameInventory(new URL('..', import.meta.url).pathname);
for (const { inventory, file, name, record } of names) console.log(`${inventory}\t${file}\t${name}\t${record}`);
console.error(`${names.length} names`);
