import assert from 'node:assert/strict';
import { naturalLanguageValues } from '../../js/src/natural-language-values.js';

const cases = [
  ['Russian', 'Оклад от 150 до 200 тысяч рублей.', 'от 150 до 200 тысяч рублей', 'value:money {"min":150000,"max":200000,"currency":"RUB"}'],
  ['Russian', 'Сумма составила 2,5 миллиона рублей.', '2,5 миллиона рублей', 'value:money {"amount":2500000,"currency":"RUB"}'],
  ['English', 'The total is 1.25 million USD.', '1.25 million USD', 'value:money {"amount":1250000,"currency":"USD"}'],
  ['English', 'The estimate was $5–7 thousand.', '$5–7 thousand', 'value:money {"min":5000,"max":7000,"currency":"USD"}'],
];

for (const [language, source, spelling, term] of cases) {
  const start = Buffer.byteLength(source.slice(0, source.indexOf(spelling)));
  const end = start + Buffer.byteLength(spelling);
  assert.ok(naturalLanguageValues(source, language).some(value => value.start === start && value.end === end && value.term === term),
    `${language}: missing ${term} at UTF-8 span ${start}..${end}`);
}
