import assert from 'node:assert/strict';
import { readProjectData, writeProjectData } from '../../js/scripts/project-data.mjs';
import { naturalLanguageValues } from '../../js/src/index.js';

const fixtures = [
  {
    source: 'zero thousand; one million zero thousand five; zero thousand years; thousand.',
    language: 'English',
    expected: [
      ['zero thousand', 'value:number 0'],
      ['one million zero thousand five', 'value:number 1000005'],
      ['zero thousand years', 'value:duration {"months":0}'],
      ['thousand', 'value:number 1000'],
    ],
    absent: [
      ['zero thousand', 'value:number 1000'],
      ['one million zero thousand five', 'value:number 1001005'],
      ['zero thousand years', 'value:duration {"months":12000}'],
    ],
  },
  {
    source: 'ноль тысяч; ноль миллионов; ноль тысяч лет; тысяча.',
    language: 'Russian',
    expected: [
      ['ноль тысяч', 'value:number 0'],
      ['ноль миллионов', 'value:number 0'],
      ['ноль тысяч лет', 'value:duration {"months":0}'],
      ['тысяча', 'value:number 1000'],
    ],
    absent: [
      ['ноль тысяч', 'value:number 1000'],
      ['ноль миллионов', 'value:number 1000000'],
      ['ноль тысяч лет', 'value:duration {"months":12000}'],
    ],
  },
];

if (process.argv.includes('--write')) {
  const file = new URL('../../parity/fixtures/natural-language-values.lino', import.meta.url);
  const data = readProjectData(file);
  const previous = new Set(data.fixtures.map(({ source }) => source));
  const additions = fixtures.filter(({ source }) => !previous.has(source));
  if (additions.length) {
    data.fixtures.push(...additions);
    writeProjectData(file, data);
  }
} else {
  for (const { source, language, expected, absent } of fixtures) {
    const bytes = Buffer.from(source);
    const values = naturalLanguageValues(source, language).map(({ start, end, term }) => (
      [bytes.subarray(start, end).toString(), term]
    ));
    for (const entry of expected) assert.ok(values.some(value => value[0] === entry[0] && value[1] === entry[1]), entry.join(' → '));
    for (const entry of absent) assert.ok(!values.some(value => value[0] === entry[0] && value[1] === entry[1]), entry.join(' → '));
  }
}
