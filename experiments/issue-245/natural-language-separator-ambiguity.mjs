import assert from 'node:assert/strict';
import { writeProjectData } from '../../js/scripts/project-data.mjs';
import { naturalLanguageValues } from '../../js/src/index.js';

const fixtures = [
  {
    source: '💵 1,234; 1.234; USD 1,234; 3,500 years; 1,234 months and 2.345 months; 1,234–2,345; USD 1,234–2,345k.',
    language: 'English',
    exactValues: [
      ['1,234', 'number', ['1234', '1.234']],
      ['1.234', 'number', ['1.234', '1234']],
      ['USD 1,234', 'money', [{ amount: 1234, currency: 'USD' }, { amount: 1.234, currency: 'USD' }]],
      ['3,500 years', 'duration', [{ months: 42000 }, { months: 42 }]],
      ['1,234 months and 2.345 months', 'duration', [{ months: 1236.345 }, { months: 3579 }, { months: 3.579 }, { months: 2346.234 }]],
      ['1,234–2,345', 'number-range', [{ min: 1234, max: 2345 }, { min: 2.345, max: 1234 }, { min: 1.234, max: 2345 }, { min: 1.234, max: 2.345 }]],
      ['USD 1,234–2,345k', 'money', [
        { min: 1234000, max: 2345000, currency: 'USD' }, { min: 2345, max: 1234000, currency: 'USD' },
        { min: 1234, max: 2345000, currency: 'USD' }, { min: 1234, max: 2345, currency: 'USD' },
      ]],
    ],
  },
  {
    source: '📌 1,234; 1.234; 1,234 руб.; менее 3,500 года; −١,٢٣٤; 1,234–1.234.',
    language: 'Russian',
    exactValues: [
      ['1,234', 'number', ['1.234', '1234']],
      ['1.234', 'number', ['1234', '1.234']],
      ['1,234 руб.', 'money', [{ amount: 1.234, currency: 'RUB' }, { amount: 1234, currency: 'RUB' }]],
      ['менее 3,500 года', 'duration', [{ lessThanMonths: 42 }, { lessThanMonths: 42000 }]],
      ['−١,٢٣٤', 'number', ['-1.234', '-1234']],
      ['1,234–1.234', 'number-range', [{ min: 1.234, max: 1234 }, { min: 1.234, max: 1.234 }, { min: 1234, max: 1234 }]],
    ],
  },
  {
    source: 'Importes ¥1,234; ١٬٢٣٤٫٥٦٧; ٣٫٥٠٠; 1 234,567; 1,234,567; 1,234.567; 0,123; 1234,567.',
    language: 'Spanish',
    exactValues: [
      ['¥1,234', 'money', [
        { amount: 1234, currency: 'JPY' }, { amount: 1234, currency: 'CNY' },
        { amount: 1.234, currency: 'JPY' }, { amount: 1.234, currency: 'CNY' },
      ]],
      ['١٬٢٣٤٫٥٦٧', 'number', ['1234.567']],
      ['٣٫٥٠٠', 'number', ['3.5']],
      ['1 234,567', 'number', ['1234.567']],
      ['1,234,567', 'number', ['1234567']],
      ['1,234.567', 'number', ['1234.567']],
      ['0,123', 'number', ['0.123']],
      ['1234,567', 'number', ['1234.567']],
    ],
  },
];

for (const { source, language, exactValues } of fixtures) {
  const bytes = Buffer.from(source);
  const values = naturalLanguageValues(source, language);
  for (const [spelling, kind, expected] of exactValues) {
    const start = bytes.indexOf(spelling);
    assert.ok(start >= 0, spelling);
    const end = start + Buffer.byteLength(spelling);
    const prefix = `value:${kind} `;
    const terms = values.filter(value => value.start === start && value.end === end && value.term.startsWith(prefix));
    const readings = terms.map(({ term }) => term.slice(prefix.length));
    assert.deepEqual(kind === 'number' ? readings : readings.map(JSON.parse), expected, spelling);
  }
}

if (process.argv.includes('--write')) {
  writeProjectData(new URL('../../parity/fixtures/natural-language-separator-ambiguity.lino', import.meta.url), { fixtures });
}
