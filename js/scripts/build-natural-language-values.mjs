import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync, writeFileSync } from 'node:fs';
import { GrammarBuilder } from '../src/grammar.js';
import { renderGrammarLinks } from '../src/grammar-links.js';

const data = new URL('../src/data/natural-language-values/', import.meta.url);
const nativeData = new URL('../../rust/src/data/natural-language-values/', import.meta.url);
const check = process.argv.includes('--check');
const argumentsToRead = process.argv.slice(2).filter(argument => argument !== '--check');
const unicodeOption = argumentsToRead.indexOf('--unicode-data');
let unicodeSource;
if (unicodeOption >= 0) {
  [, unicodeSource] = argumentsToRead.splice(unicodeOption, 2);
  assert.ok(unicodeSource, '--unicode-data requires the UnicodeData.txt path');
}
const [isoSource] = argumentsToRead;
const lexicon = readFileSync(new URL('lexicon.lino', data), 'utf8');
const rows = lexicon.trim().split('\n').map(line => line.slice(1, -1).split(' '));
let currencies = readFileSync(new URL('currencies.lino', data), 'utf8');
if (isoSource) {
  const xml = readFileSync(isoSource);
  assert.equal(createHash('sha256').update(xml).digest('hex'), '33139b438657d1cee116ba737807ea71d19d6de4b90f799a09c56f0cc6a1b0ff');
  const codes = [...new Set(Array.from(xml.toString().matchAll(/<Ccy>([A-Z]{3})<\/Ccy>/gu), match => match[1]))].sort();
  currencies = `(iso4217 2026-09-17 ${codes.join(' ')})\n`;
}

// https://www.unicode.org/Public/17.0.0/ucd/UnicodeData.txt
function collectDecimalDigits(path) {
  const unicodeData = readFileSync(path);
  const digest = createHash('sha256').update(unicodeData).digest('hex');
  assert.equal(digest, '2e1efc1dcb59c575eedf5ccae60f95229f706ee6d031835247d843c11d96470c');
  const zeros = [];
  let decimalValue = 0;
  for (const record of unicodeData.toString().split('\n')) {
    const [code, , category, , , , value] = record.split(';');
    if (category !== 'Nd') continue;
    const point = Number.parseInt(code, 16);
    assert.equal(Number(value), decimalValue);
    if (decimalValue === 0) zeros.push(point);
    assert.equal(point, zeros.at(-1) + decimalValue);
    decimalValue = (decimalValue + 1) % 10;
  }
  assert.equal(decimalValue, 0);
  return `(unicode-decimal-digits 17.0.0 ${digest} ${zeros.map(point => point.toString(16)).join(' ')})\n`;
}

const digitData = unicodeSource ? collectDecimalDigits(unicodeSource) : readFileSync(new URL('decimal-digits.lino', data), 'utf8');
const unicodeLicense = readFileSync(new URL('UNICODE-LICENSE.txt', data), 'utf8');
const digitBlocks = digitData.trim().slice(1, -1).split(' ').slice(3).map(zero => Number.parseInt(zero, 16));
const { seq, choice, literal, literalInsensitive, charClass, charRange, ref, optional, repeat } = GrammarBuilder;
const digit = ref('decimal_digit');
const characters = symbols => charClass(Array.from(symbols, value => ({ kind: 'char', value })));
const spaces = characters(' \u00a0\u202f\t');
const gap = repeat(spaces, 0, 8);
const separator = repeat(spaces, 1, 8);
const words = spellings => choice(...[...new Set(spellings)].sort((left, right) => right.length - left.length).map(literalInsensitive));
const fixedDigits = count => repeat(digit, count, count);
const decimal = characters('.,٫');
const dash = characters('-–—−');
const letter = charRange('A', 'Z');
const threeDigits = fixedDigits(3);
const fraction = repeat(digit, 1, 8);
const group = character => seq(repeat(digit, 1, 3), repeat(seq(character, threeDigits), 1, 5));
const groupedNumeral = seq(group(characters(' \u00a0\u202f')), optional(seq(decimal, fraction)));
const punctuatedNumeral = choice(
  seq(group(literal(',')), optional(seq(literal('.'), fraction))),
  seq(group(literal('.')), optional(seq(literal(','), fraction))),
  seq(group(literal('٬')), optional(seq(literal('٫'), fraction))),
);
const plainNumeral = seq(repeat(digit, 1, 18), optional(seq(decimal, fraction)));
const range = (left, right, between) => seq(left, between, right);

function profile(locale) {
  const relevant = rows.filter(([, language]) => language === 'common' || language === locale);
  const vocabulary = (category, select = () => true) => words(relevant
    .filter(([kind, , value]) => kind === category && select(Number(value)))
    .flatMap(([, , , ...spellings]) => spellings));
  const builder = new GrammarBuilder('amount');
  const local = locale !== 'common';
  const localizedSeparator = locale === 'ru' ? ['до', 'по'] : locale === 'en' ? ['to', 'through', 'until'] : [];
  const rangeSeparator = choice(seq(gap, dash, gap), ...localizedSeparator.map(word => seq(separator, literalInsensitive(word), separator)));
  const prefix = locale === 'ru' ? ['от', 'с', 'со'] : locale === 'en' ? ['from'] : [];
  const rangePrefix = local ? optional(seq(words(prefix), separator)) : GrammarBuilder.empty();
  const openDate = locale === 'ru' ? choice(seq(literalInsensitive('настоящее'), separator, literalInsensitive('время')),
    seq(literalInsensitive('н.'), gap, literalInsensitive('в.')), literalInsensitive('сейчас'))
    : words(['present', 'now', 'today']);

  builder.rule('decimal_digit', charClass(digitBlocks.map(zero => ({
    kind: 'range', start: String.fromCodePoint(zero), end: String.fromCodePoint(zero + 9),
  }))), 'token');
  builder.rule('numeral', seq(optional(characters('+-−')), choice(groupedNumeral, punctuatedNumeral, plainNumeral)), 'token');
  builder.rule('scale', vocabulary('scale'), 'token');
  const abbreviatedScale = seq(gap, ref('scale'));
  const numeralScale = local ? choice(abbreviatedScale, seq(separator, ref('word_scale'))) : abbreviatedScale;
  builder.rule('amount', choice(seq(ref('numeral'), optional(numeralScale)),
    ...(local ? [ref('word_amount')] : [])));
  if (local) {
    builder.rule('word_scale', vocabulary('number', value => value >= 1000), 'token');
    builder.rule('small_number_word', vocabulary('number', value => value < 20), 'token');
    builder.rule('ones_number_word', vocabulary('number', value => value > 0 && value < 10 && Number.isInteger(value)), 'token');
    builder.rule('tens_number_word', vocabulary('number', value => value >= 20 && value < 100), 'token');
    builder.rule('hundreds_number_word', vocabulary('number', value => value >= 100 && value < 1000), 'token');
    const small = choice(ref('small_number_word'), seq(ref('tens_number_word'),
      optional(seq(locale === 'en' ? choice(separator, literal('-')) : separator, ref('ones_number_word')))));
    const conjunction = locale === 'en' ? optional(seq(literalInsensitive('and'), separator)) : GrammarBuilder.empty();
    const hundreds = seq(locale === 'en' ? optional(seq(ref('ones_number_word'), separator)) : GrammarBuilder.empty(),
      ref('hundreds_number_word'), optional(seq(separator, conjunction, small)));
    const triplet = choice(hundreds, small);
    const tiers = [1_000_000_000, 1_000_000, 1000].map((magnitude, index) => {
      const name = `large_number_word_${index + 1}`;
      builder.rule(name, vocabulary('number', value => value === magnitude), 'token');
      return seq(optional(seq(triplet, separator)), ref(name));
    });
    const wordAmount = choice(...tiers.map((tier, index) => seq(tier,
      ...tiers.slice(index + 1).map(next => optional(seq(separator, next))), optional(seq(separator, conjunction, triplet)))), triplet);
    builder.rule('word_amount', wordAmount);
    builder.rule('duration_unit', vocabulary('unit'), 'token');
    builder.rule('duration_amount', ref('amount'));
    const comparison = locale === 'ru' ? words(['более', 'свыше', 'меньше', 'менее'])
      : choice(seq(words(['more', 'less']), separator, literalInsensitive('than')), literalInsensitive('over'), literalInsensitive('under'));
    builder.rule('duration_comparison', comparison, 'token');
    const part = seq(ref('duration_amount'), separator, ref('duration_unit'));
    const join = locale === 'en' ? literalInsensitive('and') : literalInsensitive('и');
    builder.rule('duration', seq(optional(seq(ref('duration_comparison'), separator)), part,
      optional(seq(separator, optional(seq(join, separator)), part))));
    builder.rule('month_name', vocabulary('month'), 'token');
  }
  builder.rule('first_amount', ref('amount'));
  builder.rule('last_amount', ref('amount'));
  builder.rule('number_range', seq(rangePrefix, range(ref('first_amount'), ref('last_amount'), rangeSeparator)));
  builder.rule('currency', choice(vocabulary('currency'), repeat(letter, 3, 3)), 'token');
  builder.rule('money_amount', choice(seq(ref('currency'), gap, ref('amount'), optional(seq(gap, ref('currency')))),
    seq(ref('amount'), gap, ref('currency'))));
  builder.rule('first_money', choice(ref('money_amount'), ref('amount')));
  builder.rule('last_money', choice(ref('money_amount'), ref('amount')));
  builder.rule('money_range', seq(rangePrefix, range(ref('first_money'), ref('last_money'), rangeSeparator)));
  builder.rule('calendar_year', fixedDigits(4), 'token');
  builder.rule('calendar_month', repeat(digit, 1, 2), 'token');
  builder.rule('calendar_day', repeat(digit, 1, 2), 'token');
  builder.rule('full_date', choice(
    seq(ref('calendar_year'), literal('-'), ref('calendar_month'), literal('-'), ref('calendar_day')),
    seq(ref('calendar_day'), literal('.'), ref('calendar_month'), literal('.'), ref('calendar_year')),
  ));
  builder.rule('month_date', choice(
    seq(ref('calendar_month'), literal('.'), ref('calendar_year')),
    seq(ref('calendar_year'), literal('-'), ref('calendar_month')),
    ...(local ? [seq(ref('month_name'), separator, ref('calendar_year'))] : []),
  ));
  builder.rule('short_date', locale === 'en'
    ? seq(ref('calendar_month'), dash, ref('calendar_day'))
    : seq(ref('calendar_day'), dash, ref('calendar_month')));
  builder.rule('date_endpoint', choice(ref('full_date'), ref('month_date'), ref('calendar_year')));
  builder.rule('first_date', ref('date_endpoint'));
  builder.rule('last_date', choice(ref('date_endpoint'), ref('open_date')));
  builder.rule('open_date', openDate, 'token');
  builder.rule('date_range', seq(rangePrefix, range(ref('first_date'), ref('last_date'), rangeSeparator)));
  return renderGrammarLinks(builder.build());
}

for (const [name, value] of [
  ['lexicon.lino', lexicon], ['currencies.lino', currencies],
  ['decimal-digits.lino', digitData], ['UNICODE-LICENSE.txt', unicodeLicense],
  ...['en', 'ru', 'common'].map(locale => [`${locale}.lino`, profile(locale)]),
]) {
  for (const directory of [data, nativeData]) {
    const file = new URL(name, directory);
    if (check) assert.equal(readFileSync(file, 'utf8'), value, file.pathname);
    else writeFileSync(file, value);
  }
}
