import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { naturalLanguageValues } from '../../js/src/natural-language-values.js';

const cases = [
  ['Arabic', 'السعر ١٥٠٬٠٠٠٫٥٠ EGP', '١٥٠٬٠٠٠٫٥٠ EGP', 'value:money {"amount":150000.5,"currency":"EGP"}'],
  ['Hindi', 'मूल्य १२ ३०० INR', '१२ ३०० INR', 'value:money {"amount":12300,"currency":"INR"}'],
  ['Bengali', 'তারিখ ২০২৪-০২-২৯', '২০২৪-০২-২৯', 'value:date 2024-02-29'],
  ['English', '😀 𝟝-𝟟', '𝟝-𝟟', 'value:number-range {"min":5,"max":7}'],
];

for (const [language, source, spelling, expected] of cases) {
  const start = Buffer.byteLength(source.slice(0, source.indexOf(spelling)));
  const end = start + Buffer.byteLength(spelling);
  const values = naturalLanguageValues(source, language);
  assert.ok(values.some(({ start: first, end: last, term }) => first === start && last === end && term === expected),
    `${language}: missing ${expected} at UTF-8 span ${start}..${end}`);
}

const digitData = readFileSync(new URL('../../js/src/data/natural-language-values/decimal-digits.lino', import.meta.url), 'utf8');
for (const zero of digitData.trim().slice(1, -1).split(' ').slice(3)) {
  const spelling = Array.from({ length: 10 }, (_, value) => String.fromCodePoint(Number.parseInt(zero, 16) + value)).join('');
  assert.deepEqual(naturalLanguageValues(spelling, 'Spanish'), [
    { start: 0, end: Buffer.byteLength(spelling), term: 'value:number 123456789' },
  ], `Unicode decimal digit block beginning at U+${zero.toUpperCase()}`);
}
