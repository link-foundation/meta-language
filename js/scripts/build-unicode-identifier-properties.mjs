// Pin identifier properties as shared numeric ranges instead of category approximations.
import { readFileSync, writeFileSync } from 'node:fs';
if (process.versions.unicode !== '17.0') throw new Error('regenerate identifier properties with the pinned Unicode 17.0 toolchain');
const properties = {};
for (const property of ['ID_Start', 'ID_Continue', 'XID_Start', 'XID_Continue']) {
  const matches = new RegExp(`^\\p{${property}}$`, 'u');
  const ranges = [];
  let first = null;
  for (let point = 0; point <= 0x110000; point += 1) {
    const included = point < 0x110000 && matches.test(String.fromCodePoint(point));
    if (included && first === null) first = point;
    else if (!included && first !== null) { ranges.push([first, point - 1]); first = null; }
  }
  properties[property] = ranges;
}
const file = new URL('../src/data/unicode-identifier-properties.json', import.meta.url);
const result = { generatedBy: 'js/scripts/build-unicode-identifier-properties.mjs', node: process.versions.node, unicode: process.versions.unicode, properties };
if (process.argv.includes('--check')) {
  const pinned = JSON.parse(readFileSync(file, 'utf8'));
  if (pinned.unicode !== result.unicode || JSON.stringify(pinned.properties) !== JSON.stringify(properties)) {
    throw new Error('identifier properties differ from the pinned Unicode version and ranges');
  }
} else writeFileSync(file, `${JSON.stringify(result, null, 2)}\n`);
