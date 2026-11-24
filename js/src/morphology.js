import { readFileSync } from 'node:fs';
import { join } from 'node:path';
import { openRussianMorphology } from './morphology-dictionary.js';
import { englishMorphology } from './morphology-english.js';
import { russianNumberAgreementKind } from './morphology-policy.js';

const data = new URL('./data/morphology/', import.meta.url);
const rules = name => readFileSync(new URL(name, data), 'utf8').trim().split('\n').map(line => line.slice(1, -1).split(' '));
const profiles = new Map();
const languages = new Map([
  ['arabic', 'ar'], ['modern standard arabic', 'ar'], ['bengali', 'bn'], ['chinese', 'zh'], ['mandarin', 'zh'], ['mandarin chinese', 'zh'],
  ['english', 'en'], ['french', 'fr'], ['hindi', 'hi'], ['portuguese', 'pt'], ['russian', 'ru'], ['spanish', 'es'], ['urdu', 'ur'],
]);
let morphologyRules;

function profileFor(language) {
  const name = language.toLowerCase();
  const locale = languages.get(name) ?? name;
  if (profiles.has(locale)) return profiles.get(locale);
  morphologyRules ??= rules('rules.lino');
  const forms = new Map();
  const inflections = new Map();
  const parts = new Map();
  const features = new Map();
  const suffixes = [];
  const agreement = new Map();
  const profile = { locale, forms, inflections, parts, features, suffixes, agreement, foldedForms: new Map() };
  for (const [kind, owner, ...fields] of morphologyRules) {
    if (kind === 'part') parts.set(owner, fields);
    else if (kind === 'feature') features.set(owner, fields[0]);
    else if (locale === owner) {
      if (kind === 'suffix') suffixes.push(fields);
      else if (kind === 'agreement') agreement.set(`${fields[0]}:${fields[1]}`, fields[2]);
      else if (kind === 'form') {
        const [form, lemma, weight, upos, ...tags] = fields;
        indexForm(profile, form, { lemma, weight: Number(weight), upos, features: tags, source: 'dictionary' });
      }
    }
  }
  suffixes.sort((left, right) => right[0].length - left[0].length);
  if (locale === 'ru') {
    const directory = process.env.META_LANGUAGE_MORPHOLOGY_DATA;
    if (directory) profile.dictionary = openRussianMorphology(join(directory, 'ru'));
    else for (const [, form, lemma, weight, tag] of rules('russian.lino')) {
      indexForm(profile, form, tagReading(profile, { lemma, weight: Number(weight), tag, source: 'dictionary' }));
    }
  }
  profiles.set(locale, profile);
  return profile;
}

function tagReading(profile, { lemma, weight, tag, source }) {
  const [part, ...grammemes] = tag.split(/[, ]+/u);
  const [pos = 'X', ...properties] = profile.parts.get(part) ?? [];
  const features = [...new Set([...properties, ...grammemes.map(name => profile.features.get(name)).filter(Boolean)])];
  const properName = pos === 'NOUN' && features.includes('Proper=Yes');
  const upos = properName ? 'PROPN' : part === 'ADJF' && grammemes.includes('Apro') ? 'DET' : pos;
  if (part === 'NPRO') features.push('PronType=Prs');
  if (part === 'COMP') features.push('Degree=Cmp');
  return { lemma, weight, upos, features: features.filter(feature => feature !== 'Proper=Yes').sort(), source };
}

function indexForm(profile, form, reading) {
  if (!profile.forms.has(form)) profile.forms.set(form, []);
  profile.forms.get(form).push(reading);
  if (profile.locale === 'ru' && form.includes('ё')) {
    const folded = form.replaceAll('ё', 'е');
    if (!profile.foldedForms.has(folded)) profile.foldedForms.set(folded, []);
    profile.foldedForms.get(folded).push(reading);
  }
  if (!profile.inflections.has(reading.lemma)) profile.inflections.set(reading.lemma, []);
  profile.inflections.get(reading.lemma).push({ form, ...reading });
}

function predictions(profile, word) {
  if (/^\p{Nd}+$/u.test(word)) return [{ lemma: word, weight: 1, upos: 'NUM', features: ['NumType=Card'], source: 'shape' }];
  const result = [];
  let longest = 0;
  for (const [ending, replacement, upos, weight, ...features] of profile.suffixes) {
    if (ending.length < longest) break;
    if (word.length <= ending.length + 1 || !word.endsWith(ending)) continue;
    longest = ending.length;
    const lemma = word.slice(0, -longest) + (replacement === '-' ? '' : replacement);
    result.push({ lemma, upos, weight: Number(weight), features: features.sort(), source: 'suffix' });
  }
  return result.length ? result : [{ lemma: word, weight: 1, upos: 'X', features: [], source: 'identity' }];
}

/** Supplies all observed or predicted readings of one word in score order. */
export function analyzeMorphology(word, language = 'Russian') {
  const profile = profileFor(language);
  const spelling = word.normalize('NFC').toLowerCase();
  if (!spelling) return [];
  let dictionaries = profile.dictionary?.analyze(spelling).map(entry => tagReading(profile, entry))
    ?? [...profile.forms.get(spelling) ?? [], ...profile.foldedForms.get(spelling) ?? []];
  if (dictionaries.length === 0 && profile.locale === 'en') {
    dictionaries = englishMorphology(spelling).map(({ form, ...reading }) => reading);
  }
  const readings = dictionaries.length ? dictionaries : predictions(profile, spelling);
  const distinct = new Map();
  for (const reading of readings) {
    const key = `${reading.lemma}\t${reading.upos}\t${reading.features.join('|')}`;
    const match = distinct.get(key);
    if (match) match.weight += reading.weight;
    else distinct.set(key, { ...reading });
  }
  const unique = [...distinct.values()];
  const total = unique.reduce((sum, reading) => sum + reading.weight, 0);
  return unique.sort((left, right) => right.weight - left.weight)
    .map(({ weight, ...reading }) => ({ ...reading, features: [...reading.features], score: Math.round(weight * 1_000_000 / total) / 1_000_000 }));
}

/** Returns an attested form matching all requested UD features, if one exists. */
export function inflect(lemma, features, language = 'Russian') {
  const profile = profileFor(language);
  const spelling = lemma.normalize('NFC').toLowerCase();
  const requested = Object.entries(features).map(([name, value]) => `${name}=${value}`);
  const data = profile.dictionary?.analyze(spelling).filter(reading => reading.lemma === spelling && reading.source === 'dictionary')
    .flatMap(reading => profile.dictionary.inflections(reading)
      .map(({ form, tag }) => ({ form, ...tagReading(profile, { ...reading, tag }) })))
    ?? profile.inflections.get(spelling) ?? [];
  const matches = reading => !reading.features.includes('Abbr=Yes') && requested.every(feature => reading.features.includes(feature));
  const form = data.find(matches)?.form;
  return form ?? (profile.locale === 'en' ? englishMorphology(spelling, true).find(matches)?.form : undefined);
}

/** Inflects a lemma for an ordinary cardinal quantity. */
export function agreeWithNumber(lemma, quantity, language = 'Russian') {
  if (!Number.isFinite(quantity) || Math.abs(quantity) > Number.MAX_SAFE_INTEGER) return undefined;
  const { locale, agreement } = profileFor(language);
  const number = Math.abs(quantity);
  if (locale !== 'ru') return inflect(lemma, { Number: number === 1 ? 'Sing' : 'Plur' }, language);
  if (!Number.isInteger(number)) return inflect(lemma, { Case: 'Gen', Number: 'Sing' }, language);
  const kind = russianNumberAgreementKind(number);
  if (kind === 0) return inflect(lemma, { Case: 'Nom', Number: 'Sing' }, language);
  if (kind === 1) return inflect(lemma, { Case: 'Gen', Number: 'Sing' }, language);
  return agreement.get(`${lemma.normalize('NFC').toLowerCase()}:many`)
    ?? inflect(lemma, { Case: 'Gen', Number: 'Plur' }, language);
}
