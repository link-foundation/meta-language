// Pin identifier properties as shared numeric ranges instead of category approximations.
import { readProjectData, writeProjectData } from './project-data.mjs';
import { selectUnicodeIdentifierAssertion } from './unicode-identifier-assertions.mjs';
const expectation = selectUnicodeIdentifierAssertion(process.versions.unicode);
if (process.versions.unicode !== expectation.expected) throw new Error('the active Unicode version differs from its selected identifier property expectation');
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
const file = new URL('../src/data/unicode-identifier-properties.lino', import.meta.url);
const result = { generatedBy: 'js/scripts/build-unicode-identifier-properties.mjs', node: process.versions.node, unicode: process.versions.unicode, properties };
if (process.argv.includes('--check')) {
  const pinned = readProjectData(file);
  if (pinned.unicode !== result.unicode || JSON.stringify(pinned.properties) !== JSON.stringify(properties)) {
    throw new Error('identifier properties differ from the pinned Unicode version and ranges');
  }
} else writeProjectData(file, result);
