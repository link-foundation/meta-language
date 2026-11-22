// Trigram language identification shared with Rust's
// `language_identification.rs`: franc's algorithm over the generated
// `language-trigrams.js` data, counted in Unicode scalar values so both
// runtimes reach the same verdict for the same text.
import { LANGUAGE_TRIGRAMS } from './language-trigrams.js';
import { minimumLanguageLead, trigramConfidence } from './language-identification-confidence.js';

const MAX_LENGTH = 2048;
const MAX_DIFFERENCE = 300;
const MINIMUM_LETTERS = 3;
const DEFAULT_CONFIDENCE = 0.06;

const SCRIPTS = LANGUAGE_TRIGRAMS.scripts;
const MODELS = new Map(
  LANGUAGE_TRIGRAMS.models.map(({ script, languages }) => [
    script,
    languages.map(({ code, trigrams }) => [
      code,
      new Map(trigrams.split('|').map((trigram, rank) => [trigram, rank])),
    ]),
  ]),
);
const LANGUAGES = new Map(Object.entries(LANGUAGE_TRIGRAMS.languages));
const CYRILLIC_LANGUAGES = new Map(Object.entries(LANGUAGE_TRIGRAMS.cyrillicLanguages));
const CYRILLIC_ALPHABETS = new Map(Object.entries(LANGUAGE_TRIGRAMS.cyrillicAlphabets)
  .map(([code, letters]) => [code, new Set(letters)]));

// ECMAScript `\s`, spelled out so Rust can match it exactly.
const WHITESPACE = new Set([
  0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x20, 0xa0, 0x1680, 0x2000, 0x2001, 0x2002, 0x2003, 0x2004,
  0x2005, 0x2006, 0x2007, 0x2008, 0x2009, 0x200a, 0x2028, 0x2029, 0x202f, 0x205f, 0x3000,
  0xfeff,
]);

/** Engine term recorded on identification annotations. */
export const LANGUAGE_IDENTIFIER_TERM = 'identifier:trigram';

/** Ranks language evidence globally and within each continuous script span. */
export function rankLanguageIdentification(text, { minimumConfidence = DEFAULT_CONFIDENCE } = {}) {
  if (typeof minimumConfidence !== 'number' || !(minimumConfidence >= 0 && minimumConfidence <= 1)) {
    throw new RangeError('minimumConfidence must be between zero and one');
  }

  const spans = [];
  const evidence = new Map();
  let letterCount = 0;
  let byteOffset = 0;
  let examinedCharacters = 0;
  let run;
  for (const character of text) {
    if (examinedCharacters >= MAX_LENGTH) break;
    examinedCharacters += 1;
    const start = byteOffset;
    const codePoint = character.codePointAt(0);
    byteOffset += codePoint <= 0x7f ? 1 : codePoint <= 0x7ff ? 2 : codePoint <= 0xffff ? 3 : 4;
    const script = SCRIPTS.find(({ ranges }) => inRanges(codePoint, ranges))?.name;
    if (script === undefined) {
      if (run && run.characters.at(-1) !== ' ') run.characters.push(' ');
      continue;
    }
    if (run?.script !== script) {
      if (run) record(run);
      run = { script, start, end: byteOffset, characters: [], letters: 0 };
    }
    run.characters.push(character);
    run.end = byteOffset;
    run.letters += 1;
  }
  if (run) record(run);

  const candidates = rankCandidates([...evidence].map(([language, score]) => ({
    language, confidence: score / letterCount,
  })));
  return { language: selectLanguage(candidates, minimumConfidence), candidates, spans };

  function record({ script, start, end, characters, letters }) {
    if (characters.at(-1) === ' ') characters.pop();
    const candidates = rankScript(script, characters, letters);
    for (const { language, confidence } of candidates) {
      evidence.set(language, (evidence.get(language) ?? 0) + confidence * letters);
    }
    letterCount += letters;
    spans.push({ script, start, end, language: selectLanguage(candidates, minimumConfidence), candidates });
  }
}

function rankCandidates(candidates) {
  return candidates.sort((left, right) => right.confidence - left.confidence);
}

function selectLanguage(candidates, minimumConfidence) {
  const [first, second] = candidates;
  if (!first || first.confidence < minimumConfidence) return null;
  if (second && !minimumLanguageLead(first.confidence, second.confidence)) return null;
  return first.language;
}

function rankScript(script, characters, letters) {
  if (letters < MINIMUM_LETTERS) return [];
  const models = MODELS.get(script);
  if (models === undefined) {
    const language = LANGUAGES.get(script);
    return language ? [{ language, confidence: 1 }] : [];
  }

  const trigrams = trigramCounts(characters);
  const maximumDistance = trigrams.length * MAX_DIFFERENCE;
  if (maximumDistance === 0) return [];
  const cyrillicLetters = script === 'Cyrillic'
    ? [...new Set(cleanCharacters(characters).filter((character) => character !== ' '))]
    : [];
  const candidates = [];
  for (const [code, model] of models) {
    const alphabet = CYRILLIC_ALPHABETS.get(code);
    if (alphabet && cyrillicLetters.some((character) => !alphabet.has(character))) continue;
    const distance = trigramDistance(trigrams, model);
    if (distance < maximumDistance) {
      candidates.push({ language: LANGUAGES.get(code) ?? CYRILLIC_LANGUAGES.get(code),
        confidence: trigramConfidence(distance, maximumDistance, letters) });
    }
  }
  return rankCandidates(candidates);
}

/**
 * Returns the canonical name of the supported natural language the text is
 * written in, or `undefined` when the text carries no usable evidence.
 */
export function identifyLanguage(text) {
  const characters = [...text].slice(0, MAX_LENGTH);
  if (characters.length === 0) return undefined;

  const script = topScript(characters);
  if (script === undefined) return undefined;
  const models = MODELS.get(script);
  if (models === undefined) return LANGUAGES.get(script);
  if (models.length === 0) return undefined;

  const trigrams = trigramCounts(characters);
  let best;
  for (const [code, model] of models) {
    if (!LANGUAGES.has(code)) continue;
    const distance = trigramDistance(trigrams, model);
    if (best === undefined || distance < best.distance) best = { code, distance };
  }
  // A distance of MAX_DIFFERENCE per character means no trigram matched.
  if (!best || best.distance >= characters.length * MAX_DIFFERENCE) return undefined;
  return LANGUAGES.get(best.code);
}

function topScript(characters) {
  let top;
  let topCount = 0;
  for (const { name, ranges } of SCRIPTS) {
    let count = 0;
    for (const character of characters) {
      if (inRanges(character.codePointAt(0), ranges)) count += 1;
    }
    if (count > topCount) {
      top = name;
      topCount = count;
    }
  }
  return top;
}

function inRanges(codePoint, ranges) {
  let low = 0;
  let high = ranges.length - 1;
  while (low <= high) {
    const middle = (low + high) >> 1;
    const [start, end] = ranges[middle];
    if (codePoint < start) high = middle - 1;
    else if (codePoint > end) low = middle + 1;
    else return true;
  }
  return false;
}

// franc's `clean`: ASCII punctuation and digits become spaces, whitespace
// runs collapse to one space, the result is trimmed and lowercased.
function cleanCharacters(characters) {
  const cleaned = [];
  for (const character of characters) {
    const codePoint = character.codePointAt(0);
    const space = (codePoint >= 0x21 && codePoint <= 0x40) || WHITESPACE.has(codePoint);
    if (space) {
      if (cleaned.length > 0 && cleaned.at(-1) !== ' ') cleaned.push(' ');
    } else {
      cleaned.push(character);
    }
  }
  if (cleaned.at(-1) === ' ') cleaned.pop();
  return [...cleaned.join('').toLowerCase()];
}

// Trigram counts in first-occurrence order, stably sorted by count.
function trigramCounts(characters) {
  const padded = [' ', ...cleanCharacters(characters), ' '];
  const counts = new Map();
  for (let index = 0; index + 3 <= padded.length; index += 1) {
    const trigram = padded.slice(index, index + 3).join('');
    counts.set(trigram, (counts.get(trigram) ?? 0) + 1);
  }
  return [...counts].sort((left, right) => left[1] - right[1]);
}

function trigramDistance(trigrams, model) {
  let distance = 0;
  for (const [trigram, count] of trigrams) {
    const rank = model.get(trigram);
    distance += rank === undefined ? MAX_DIFFERENCE : Math.abs(count - rank - 1);
  }
  return distance;
}
