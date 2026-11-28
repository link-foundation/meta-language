// Trigram language identification shared with Rust's
// `language_identification.rs`: franc's algorithm over the generated
// `language-trigrams.js` data, counted in Unicode scalar values so both
// runtimes reach the same verdict for the same text.
import { LANGUAGE_TRIGRAMS } from './language-trigrams.js';

const MAX_LENGTH = 2048;
const MAX_DIFFERENCE = 300;
const DEFAULT_THRESHOLD = 0.1;
const MINIMUM_CONFIDENCE_GAP = 0.02;
const LETTER = /\p{L}/u;
const MARK = /\p{M}/u;

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
const CYRILLIC_CATALOG_ALPHABET = new Set(LANGUAGE_TRIGRAMS.cyrillicCatalogAlphabet);

// ECMAScript `\s`, spelled out so Rust can match it exactly.
const WHITESPACE = new Set([
  0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x20, 0xa0, 0x1680, 0x2000, 0x2001, 0x2002, 0x2003, 0x2004,
  0x2005, 0x2006, 0x2007, 0x2008, 0x2009, 0x200a, 0x2028, 0x2029, 0x202f, 0x205f, 0x3000,
  0xfeff,
]);

/** Engine term recorded on identification annotations. */
export const LANGUAGE_IDENTIFIER_TERM = 'identifier:trigram';

/**
 * Ranks catalog languages for the text and for each script run. A verdict below
 * the inclusive threshold is `null`, and its candidates remain visible.
 * The sample covers the first 2,048 Unicode scalar values.
 */
export function identifyLanguage(text, { threshold = DEFAULT_THRESHOLD } = {}) {
  if (!Number.isFinite(threshold) || threshold < 0 || threshold > 1) {
    throw new RangeError('Language identification threshold must be between 0 and 1');
  }

  const totals = new Map();
  let letterCount = 0;
  const spans = scriptRuns(text).map((run) => {
    const candidates = rankRun(run.script, text.slice(run.index, run.limit), run.letterCount);
    letterCount += run.letterCount;
    for (const { language, confidence } of candidates) {
      totals.set(language, (totals.get(language) ?? 0) + confidence * run.letterCount);
    }
    return { start: run.start, end: run.end, language: selectLanguage(candidates, threshold), candidates };
  });
  if (spans.length === 1) {
    const { language, candidates } = spans[0];
    return { language, candidates, spans };
  }
  const candidates = [...totals].map(([language, total]) => ({ language, confidence: total / letterCount }));
  candidates.sort((left, right) => right.confidence - left.confidence);
  return { language: selectLanguage(candidates, threshold), candidates, spans };
}

function selectLanguage(candidates, threshold) {
  const [best, second] = candidates;
  if (!best || best.confidence < threshold) return null;
  if (second && best.confidence - second.confidence < MINIMUM_CONFIDENCE_GAP) return null;
  return best.language;
}

function scriptRuns(text) {
  const runs = [];
  let run;
  let index = 0;
  let byteOffset = 0;
  let attached = false;
  let count = 0;
  for (const character of text) {
    if (count === MAX_LENGTH) break;
    count += 1;
    const codePoint = character.codePointAt(0);
    const byteLength = codePoint < 0x80 ? 1 : codePoint < 0x800 ? 2 : codePoint < 0x10000 ? 3 : 4;
    const nextIndex = index + character.length;
    const nextOffset = byteOffset + byteLength;
    if (LETTER.test(character)) {
      const script = SCRIPTS.find(({ ranges }) => inRanges(codePoint, ranges))?.name;
      if (!run || run.script !== script) {
        run = { script, start: byteOffset, end: nextOffset, index, limit: nextIndex, letterCount: 0 };
        runs.push(run);
      }
      run.letterCount += 1;
      attached = true;
    } else {
      attached = attached && MARK.test(character);
    }
    if (attached) {
      run.end = nextOffset;
      run.limit = nextIndex;
    }
    index = nextIndex;
    byteOffset = nextOffset;
  }
  return runs;
}

function rankRun(script, text, letterCount) {
  const models = MODELS.get(script);
  if (models === undefined) {
    const language = LANGUAGES.get(script);
    return language ? [{ language, confidence: 1 }] : [];
  }
  if (letterCount < 3 || models.length === 0) return [];

  const characters = [...text.normalize('NFC')];
  if (script === 'Cyrillic' && characters.some((character) =>
    LETTER.test(character) && !CYRILLIC_CATALOG_ALPHABET.has(character.toLowerCase())
  )) return [];

  const trigrams = trigramCounts(characters);
  const maximum = trigrams.length * MAX_DIFFERENCE;
  const ranked = models.map(([code, model]) => ({
    language: LANGUAGES.get(code),
    distance: trigramDistance(trigrams, model),
  }));
  ranked.sort((left, right) => left.distance - right.distance);
  // Competing Cyrillic models can reject a Russian verdict, including ties.
  if (ranked.some(({ language, distance }) => !language && distance === ranked[0].distance)) return [];
  return ranked
    .filter(({ language, distance }) => language && distance < maximum)
    .map(({ language, distance }) => ({ language, confidence: (maximum - distance) / maximum }));
}

/** Returns the catalog verdict used by existing parse annotations. */
export function identifyCatalogLanguage(text) {
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
