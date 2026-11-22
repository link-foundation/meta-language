import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { readProjectData, writeProjectData } from '../../js/scripts/project-data.mjs';
import { readProfiledNamingSourceBindingDocument } from '../../js/scripts/naming-source-binding-codec-profile.mjs';
import { readBootstrapStructuredDataLino } from '../../scripts/lib/structured-data-bootstrap.mjs';

const root = new URL('../../', import.meta.url);
const source = 'js/tests/issue-195-evidence-translation-partition-conservation.test.js';
const previousDigest = '762312280e9425897b080d3cfeeba9b5262eeae81cdcf65e792cfc54a55d80d3';
const previousQualifier = 'with current issue one hundred ninety five evidence translation partition conservation test source';
const qualifier = 'with the updated report schedule control source';
const current = readFileSync(new URL(source, root), 'utf8');
const currentDigest = createHash('sha256').update(current).digest('hex');
const [mode] = process.argv.slice(2);
assert.ok(['--bindings', '--concepts'].includes(mode));

const bindings = readProfiledNamingSourceBindingDocument();
const predecessors = bindings.filter(row => row.source === source && row.sourceSha256 === previousDigest);
assert.equal(predecessors.length, 5);
const previous = predecessors[0].sourceContext;
const firstDifference = [...previous].findIndex((character, index) => current[index] !== character);
assert.ok(firstDifference >= 0);
let commonSuffixLength = 0;
while (previous.at(-1 - commonSuffixLength) === current.at(-1 - commonSuffixLength)) commonSuffixLength++;
const previousEditEnd = previous.length - commonSuffixLength;
const shift = current.length - previous.length;
const successors = predecessors.map(row => {
  assert.equal(row.sourceContext, previous);
  const { start, end } = row.sourceRange;
  assert.equal(previous.slice(start, end), row.sourceText);
  assert.ok(start <= firstDifference || start >= previousEditEnd);
  assert.ok(end <= firstDifference || end >= previousEditEnd);
  const sourceRange = { start: start >= previousEditEnd ? start + shift : start, end: end >= previousEditEnd ? end + shift : end };
  const sourceText = current.slice(sourceRange.start, sourceRange.end);
  assert.equal(sourceText === row.sourceText, end <= firstDifference || start >= previousEditEnd);
  const inventoryName = row.inventoryName.replace(previousQualifier, qualifier);
  assert.notEqual(inventoryName, row.inventoryName);
  return {
    ...row,
    id: `repository.${inventoryName.replaceAll(' ', '-')}`,
    name: row.name.replace(previousDigest, currentDigest),
    inventoryName,
    sourceText,
    sourceContext: current,
    sourceRange,
    sourceSha256: currentDigest,
  };
});

if (mode === '--bindings') {
  assert.ok(successors.every(row => bindings.every(previousRow => previousRow.id !== row.id)));
  writeProjectData(new URL('parity/naming/source-bindings.lino', root), [...bindings, ...successors]);
} else {
  assert.ok(successors.every(row => bindings.some(candidate => candidate.id === row.id && candidate.name === row.name)));
  const file = new URL('parity/naming/canonical-concepts.lino', root);
  const registry = readBootstrapStructuredDataLino(readFileSync(file, 'utf8'));
  for (const [index, successor] of successors.entries()) {
    const previous = registry.concepts.find(record => record.id === predecessors[index].id);
    assert.ok(previous && !registry.concepts.some(record => record.id === successor.id));
    registry.concepts.push({
      ...previous,
      id: successor.id,
      phrase: successor.inventoryName,
      definition: previous.definition.replace(previousDigest, currentDigest),
      sourceAliases: [{ source, name: successor.name }],
    });
  }
  writeProjectData(file, registry);
}
