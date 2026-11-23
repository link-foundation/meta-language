import assert from 'node:assert/strict';
import { readProjectData } from '../../js/scripts/project-data.mjs';
import { naturalLanguageValues } from '../../js/src/index.js';

const fixture = new URL('../../parity/fixtures/natural-language-named-calendar-dates.lino', import.meta.url);
const { fixtures } = readProjectData(fixture);

for (const { source, language, expected, absent = [] } of fixtures) {
  const bytes = Buffer.from(source);
  const values = naturalLanguageValues(source, language)
    .map(({ start, end, term }) => [bytes.subarray(start, end).toString(), term]);
  for (const [spelling, term] of expected) {
    assert.ok(values.some(value => value[0] === spelling && value[1] === term), `${spelling} → ${term}`);
  }
  for (const [spelling, term] of absent) {
    assert.ok(!values.some(value => value[0] === spelling && value[1] === term), `${spelling} → ${term}`);
  }
}

console.log(`${fixtures.length} named calendar date fixtures passed`);
