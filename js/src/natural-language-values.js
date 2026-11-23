import { readFileSync } from 'node:fs';
import { compileGrammar } from './grammar.js';
import { GrammarParseError } from './grammar-runtime.js';
import { parseGrammarLinks } from './grammar-links.js';
import { calendarMonthDays, preferredGroupingDelimiter } from './natural-language-value-policy.js';
import { sourceBoundaries } from './source-boundaries.js';

const parsers = new Map();
const data = new URL('./data/natural-language-values/', import.meta.url);
let vocabulary;
let currencyCodes;
let decimalDigits;

function loadDecimalDigits() {
  const source = readFileSync(new URL('decimal-digits.lino', data), 'utf8');
  const result = new Map();
  for (const zero of source.trim().slice(1, -1).split(' ').slice(3)) {
    const start = Number.parseInt(zero, 16);
    for (let digit = 0; digit < 10; digit += 1) result.set(String.fromCodePoint(start + digit), String(digit));
  }
  return result;
}

function valueProfile(language) {
  const name = language.toLowerCase();
  const locale = name === 'russian' || name === 'ru' ? 'ru' : name === 'english' || name === 'en' ? 'en' : 'common';
  if (!vocabulary) {
    const source = readFileSync(new URL('lexicon.lino', data), 'utf8');
    vocabulary = source.trim().split('\n').map(line => line.slice(1, -1).split(' '));
    const codes = readFileSync(new URL('currencies.lino', data), 'utf8');
    currencyCodes = new Set(codes.trim().slice(1, -1).split(' ').slice(2));
    decimalDigits = loadDecimalDigits();
  }
  let profile = parsers.get(locale);
  if (!profile) {
    const words = new Map();
    for (const [kind, owner, value, ...spellings] of vocabulary) {
      if (owner !== 'common' && owner !== locale) continue;
      if (!words.has(kind)) words.set(kind, new Map());
      for (const spelling of spellings) words.get(kind).set(spelling.toLowerCase(), value);
    }
    const grammar = parseGrammarLinks(readFileSync(new URL(`${locale}.lino`, data), 'utf8'));
    profile = { locale, words, parser: compileGrammar(grammar) };
    parsers.set(locale, profile);
  }
  return profile;
}

function exactDecimal(spelling) {
  const [whole, fraction = ''] = spelling.split('.');
  return { coefficient: BigInt(whole + fraction), scale: fraction.length };
}

function decimalText({ coefficient, scale }) {
  const sign = coefficient < 0n ? '-' : '';
  const digits = (coefficient < 0n ? -coefficient : coefficient).toString().padStart(scale + 1, '0');
  if (scale === 0) return sign + digits;
  const fraction = digits.slice(-scale).replace(/0+$/u, '');
  return sign + digits.slice(0, -scale) + (fraction ? `.${fraction}` : '');
}

function addDecimals(left, right) {
  const scale = Math.max(left.scale, right.scale);
  return { coefficient: left.coefficient * 10n ** BigInt(scale - left.scale)
    + right.coefficient * 10n ** BigInt(scale - right.scale), scale };
}

function scaleDecimal(decimal, factor) {
  return { coefficient: decimal.coefficient * BigInt(factor), scale: decimal.scale };
}

function normalizeDecimalDigits(text) {
  let normalized = '';
  for (const character of text) normalized += decimalDigits.get(character) ?? character;
  return normalized;
}

function numericSpelling(text, locale) {
  const signed = normalizeDecimalDigits(text).replaceAll(/[ \u00a0\u202f٬]/gu, '').replace('−', '-');
  if (signed.includes('٫')) return signed.replace('٫', '.');
  const comma = signed.lastIndexOf(',');
  const period = signed.lastIndexOf('.');
  if (comma >= 0 && period >= 0) {
    return comma > period ? signed.replaceAll('.', '').replace(',', '.') : signed.replaceAll(',', '');
  }
  const delimiter = comma >= 0 ? ',' : '.';
  const sections = signed.split(delimiter);
  if (sections.length === 1) return signed;
  if (sections.length > 2 || (preferredGroupingDelimiter(locale, delimiter) && /^[+-]?[1-9][0-9]{0,2}$/u.test(sections[0]) && sections[1].length === 3)) {
    return sections.join('');
  }
  return sections.join('.');
}

const numberWordRules = ['small_number_word', 'ones_number_word', 'tens_number_word', 'hundreds_number_word',
  'large_number_word_1', 'large_number_word_2', 'large_number_word_3'];
const captureNames = ['amount', 'numeral', 'scale', 'word_scale', ...numberWordRules,
  'first_amount', 'last_amount', 'currency', 'first_money', 'last_money', 'money_amount',
  'duration_amount', 'duration_unit', 'duration_comparison', 'first_date', 'last_date',
  'calendar_year', 'calendar_month', 'calendar_day', 'month_name', 'open_date'];

const inside = (outer, inner) => inner.start >= outer.start && inner.end <= outer.end;
const capturesInside = (match, range, name) => match.captures[name].filter(item => inside(range, item));
const wordValue = (profile, kind, value) => profile.words.get(kind)?.get(value?.toLowerCase());

function readScale(match, range, profile) {
  const abbreviated = capturesInside(match, range, 'scale')[0];
  const written = capturesInside(match, range, 'word_scale')[0];
  return wordValue(profile, 'scale', abbreviated?.text) ?? wordValue(profile, 'number', written?.text);
}

function readAmount(match, range, profile) {
  const numeral = capturesInside(match, range, 'numeral')[0];
  if (numeral) {
    return scaleDecimal(exactDecimal(numericSpelling(numeral.text, profile.locale)), readScale(match, range, profile) ?? 1);
  }
  const words = numberWordRules.flatMap(rule => capturesInside(match, range, rule)).sort((left, right) => left.start - right.start);
  let total = exactDecimal('0');
  let chunk;
  for (const { text } of words) {
    const number = wordValue(profile, 'number', text);
    const integer = Number(number);
    if (integer >= 1000) {
      total = addDecimals(total, scaleDecimal(chunk ?? exactDecimal('1'), number));
      chunk = undefined;
    } else if (integer === 100 && profile.locale === 'en') {
      chunk = scaleDecimal(chunk ?? exactDecimal('1'), number);
    } else chunk = addDecimals(chunk ?? exactDecimal('0'), exactDecimal(number));
  }
  return chunk === undefined ? total : addDecimals(total, chunk);
}

function readCurrency(match, profile) {
  let currency;
  const defaultSymbols = new Map([['$', ['USD', 'CAD', 'AUD', 'NZD', 'HKD', 'SGD']], ['¥', ['JPY', 'CNY']]]);
  const spellings = match.captures.currency;
  for (const { text } of spellings) {
    const candidate = wordValue(profile, 'currency', text) ?? (currencyCodes.has(text) ? text : undefined);
    if (!candidate) return undefined;
    if (defaultSymbols.has(text)) continue;
    if (currency && candidate !== currency) return undefined;
    currency = candidate;
  }
  for (const { text } of spellings) {
    const candidates = defaultSymbols.get(text);
    if (!candidates) continue;
    if (currency && !candidates.includes(currency)) return undefined;
    currency ??= candidates[0];
  }
  return currency;
}

function readDate(match, range, profile) {
  const part = name => capturesInside(match, range, name)[0]?.text;
  const calendarPart = name => {
    const value = part(name);
    return value && normalizeDecimalDigits(value);
  };
  if (part('open_date')) return 'open';
  const year = calendarPart('calendar_year');
  const month = calendarPart('calendar_month') ?? wordValue(profile, 'month', part('month_name'));
  const day = calendarPart('calendar_day');
  if (year === '0000' || (month && (Number(month) < 1 || Number(month) > 12))) return undefined;
  if (!month) return year;
  const paddedMonth = month.padStart(2, '0');
  if (!day) return `${year}-${paddedMonth}`;
  const yearNumber = Number(year ?? '2000');
  if (Number(day) < 1 || Number(day) > calendarMonthDays(yearNumber, Number(month))) return undefined;
  return `${year ?? '-'}-${paddedMonth}-${day.padStart(2, '0')}`;
}

function readDuration(match, profile) {
  let months = exactDecimal('0');
  for (const [index, amount] of match.captures.duration_amount.entries()) {
    const unit = wordValue(profile, 'unit', match.captures.duration_unit[index].text);
    months = addDecimals(months, scaleDecimal(readAmount(match, amount, profile), unit));
  }
  const comparison = match.captures.duration_comparison[0]?.text.toLowerCase();
  const property = !comparison ? 'months' : /^(less|under|меньше|менее)/u.test(comparison) ? 'lessThanMonths' : 'moreThanMonths';
  return `value:duration {"${property}":${decimalText(months)}}`;
}

function numberRange(match, profile, money) {
  const first = match.captures[money ? 'first_money' : 'first_amount'][0];
  const last = match.captures[money ? 'last_money' : 'last_amount'][0];
  let left = readAmount(match, first, profile);
  const right = readAmount(match, last, profile);
  const trailingScale = readScale(match, last, profile);
  if (trailingScale && capturesInside(match, first, 'numeral').length && readScale(match, first, profile) === undefined) {
    left = scaleDecimal(left, trailingScale);
  }
  const comparison = addDecimals(left, scaleDecimal(right, -1)).coefficient;
  const [minimum, maximum] = comparison <= 0n ? [left, right] : [right, left];
  const limits = `"min":${decimalText(minimum)},"max":${decimalText(maximum)}`;
  if (!money) return `value:number-range {${limits}}`;
  const currency = readCurrency(match, profile);
  if (currency) return `value:money {${limits},"currency":"${currency}"}`;
}

function termFor(rule, match, profile) {
  switch (rule) {
    case 'amount': return `value:number ${decimalText(readAmount(match, match, profile))}`;
    case 'number_range': return numberRange(match, profile, false);
    case 'money_range': return numberRange(match, profile, true);
    case 'money_amount': {
      const currency = readCurrency(match, profile);
      if (currency) return `value:money {"amount":${decimalText(readAmount(match, match, profile))},"currency":"${currency}"}`;
      break;
    }
    case 'duration': return readDuration(match, profile);
    case 'date_range': {
      const from = readDate(match, match.captures.first_date[0], profile);
      const to = readDate(match, match.captures.last_date[0], profile);
      if (from && to) return `value:date-range {"from":"${from}","to":"${to}"}`;
      break;
    }
    default: {
      const date = readDate(match, match, profile);
      if (date) return `value:date ${date}`;
    }
  }
}

function containsDecimalDigit(text) {
  for (const character of text) if (decimalDigits.has(character)) return true;
  return false;
}

function precededByDecimalDigit(text, offset) {
  // The previous code point can occupy two UTF-16 code units.
  let previous;
  for (const character of text.slice(Math.max(0, offset - 2), offset)) previous = character;
  return decimalDigits.has(previous);
}

/** Typed values with offsets in the original UTF-8 source. */
export function naturalLanguageValues(text, language) {
  const profile = valueProfile(language);
  if (!containsDecimalDigit(text) && !text.toLowerCase().split(/[^\p{L}\p{M}]+/u).some(word => profile.words.get('number')?.has(word))) return [];
  const boundaries = sourceBoundaries(text);
  const results = [];
  const seen = new Set();
  const rules = ['amount', 'money_range', 'number_range', 'date_range', 'money_amount', 'full_date', 'month_date', 'short_date'];
  const compounds = new Set();
  if (profile.locale !== 'common') rules.unshift('duration');
  for (const rule of rules) {
    let matches;
    try {
      matches = profile.parser.findAll(text, { rule, wordBoundaries: true, captures: captureNames });
    } catch (error) {
      if (!(error instanceof GrammarParseError)) throw error;
      continue;
    }
    for (const match of matches) {
      const range = `${match.start}:${match.end}`;
      if (rule === 'amount' && numberWordRules.some(name => match.captures[name].length)) compounds.add(range);
      if (rule === 'number_range' && compounds.has(range)) continue;
      if (rule === 'amount' && /^[-−]/u.test(match.text) && precededByDecimalDigit(text, match.start)) continue;
      const term = termFor(rule, match, profile);
      if (!term) continue;
      const start = boundaries.get(match.start).byte;
      const end = boundaries.get(match.end).byte;
      const key = `${start}:${end}:${term}`;
      if (seen.has(key)) continue;
      seen.add(key);
      results.push({ start, end, term });
    }
  }
  return results.sort((left, right) => left.start - right.start || right.end - left.end);
}
