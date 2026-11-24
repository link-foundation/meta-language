import assert from 'node:assert/strict';
import { writeProjectData } from '../../js/scripts/project-data.mjs';
import { naturalLanguageValues } from '../../js/src/index.js';

const dollarCurrencies = 'USD ARS AUD BBD BMD BND BSD BZD CAD CLP COP CUP DOP FJD GYD HKD JMD KYD LRD MXN NAD NZD SBD SGD SRD TTD TWD UYU XCD'.split(' ');
const fixtures = [
  {
    source: '💰 Budget $5k–7k.',
    language: 'English',
    exactMoney: [['$5k–7k', { min: 5000, max: 7000 }, dollarCurrencies]],
  },
  {
    source: 'Суммы ¥1200; ¥1300 CNY; JP¥1400.',
    language: 'Russian',
    exactMoney: [
      ['¥1200', { amount: 1200 }, ['JPY', 'CNY']],
      ['¥1300 CNY', { amount: 1300 }, ['CNY']],
      ['JP¥1400', { amount: 1400 }, ['JPY']],
    ],
  },
  {
    source: 'Importes $1200 MXN; C$8; c$9; C$5–$7; £12; ₩21; €14.',
    language: 'Spanish',
    exactMoney: [
      ['$1200 MXN', { amount: 1200 }, ['MXN']],
      ['C$8', { amount: 8 }, ['CAD', 'NIO']],
      ['c$9', { amount: 9 }, ['CAD', 'NIO']],
      ['C$5–$7', { min: 5, max: 7 }, ['CAD']],
      ['£12', { amount: 12 }, ['GBP', 'FKP', 'GIP', 'SHP', 'SSP', 'SYP']],
      ['₩21', { amount: 21 }, ['KRW', 'KPW']],
      ['€14', { amount: 14 }, ['EUR']],
    ],
  },
  {
    source: 'Mixed $5–¥7; CAD 8 USD; ¥9 AUD.',
    language: 'English',
    exactMoney: [
      ['$5–¥7', {}, []],
      ['CAD 8 USD', {}, []],
      ['¥9 AUD', {}, []],
    ],
  },
  {
    source: 'Prices MX$12; NT$13; E£14; L£15; EC$16; GH₵17; T$18; ₼19; ⃀20; ؋21.',
    language: 'English',
    exactMoney: [
      ['MX$12', { amount: 12 }, ['MXN']],
      ['NT$13', { amount: 13 }, ['TWD']],
      ['E£14', { amount: 14 }, ['EGP']],
      ['L£15', { amount: 15 }, ['LBP']],
      ['EC$16', { amount: 16 }, ['XCD']],
      ['GH₵17', { amount: 17 }, ['GHS']],
      ['T$18', { amount: 18 }, ['TOP']],
      ['₼19', { amount: 19 }, ['AZN']],
      ['⃀20', { amount: 20 }, ['KGS']],
      ['؋21', { amount: 21 }, ['AFN']],
    ],
  },
];

if (process.argv.includes('--write')) {
  writeProjectData(new URL('../../parity/fixtures/natural-language-currency-ambiguity.lino', import.meta.url), { fixtures });
}

for (const { source, language, exactMoney } of fixtures) {
  const bytes = Buffer.from(source);
  const values = naturalLanguageValues(source, language);
  for (const [spelling, fields, currencies] of exactMoney) {
    const start = bytes.indexOf(spelling);
    assert.ok(start >= 0, spelling);
    const end = start + Buffer.byteLength(spelling);
    const terms = values.filter(value => value.start === start && value.end === end && value.term.startsWith('value:money '));
    assert.deepEqual(terms.map(({ term }) => JSON.parse(term.slice('value:money '.length))),
      currencies.map(currency => ({ ...fields, currency })), spelling);
  }
}
