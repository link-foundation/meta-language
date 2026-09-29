// The English vocabulary the naming check reads: the lemmas of WordNet 3.1
// (the `wordnet-db` package, an independent lexical database from Princeton
// University), by part of speech, with WordNet's own detachment rules for
// inflected forms. Canonical names are checked against it rather than against
// a letters-only pattern or a hand-picked word list.

import { readFileSync } from 'node:fs';
import { createRequire } from 'node:module';
import path from 'node:path';

const require = createRequire(import.meta.url);

export const PARTS_OF_SPEECH = Object.freeze(['noun', 'verb', 'adjective', 'adverb']);
const INDEX_FILES = Object.freeze({ noun: 'index.noun', verb: 'index.verb', adjective: 'index.adj', adverb: 'index.adv' });

// WordNet's morphological detachment rules (morphy(7WN)), by part of speech.
const DETACHMENT_RULES = Object.freeze({
  noun: [['s', ''], ['ses', 's'], ['xes', 'x'], ['zes', 'z'], ['ches', 'ch'], ['shes', 'sh'], ['men', 'man'], ['ies', 'y']],
  verb: [['s', ''], ['ies', 'y'], ['es', 'e'], ['es', ''], ['ed', 'e'], ['ed', ''], ['ing', 'e'], ['ing', '']],
  adjective: [['er', ''], ['est', ''], ['er', 'e'], ['est', 'e']],
  adverb: [],
});

/**
 * The lemmas of one WordNet index file (lower case, `_` joining the words of a
 * compound) with the synsets each lemma belongs to, its senses in WordNet order.
 * An index line is `lemma pos synset_cnt p_cnt [ptr_symbol...] sense_cnt
 * tagsense_cnt synset_offset...` (wndb(5WN)).
 */
function readIndex(file) {
  const synsets = new Map();
  for (const line of readFileSync(file, 'utf8').split('\n')) {
    if (line === '' || line.startsWith(' ')) continue;
    const fields = line.trim().split(/\s+/u);
    const synsetCount = Number(fields[2]);
    synsets.set(fields[0], fields.slice(-synsetCount));
  }
  return synsets;
}

/** The WordNet release the vocabulary reads, its lemmas and their synsets by part of speech. */
export function loadWordNet(directory = require('wordnet-db').path) {
  const { version } = require('wordnet-db');
  const synsets = Object.fromEntries(
    PARTS_OF_SPEECH.map((part) => [part, readIndex(path.join(directory, INDEX_FILES[part]))]),
  );
  const lemmas = Object.fromEntries(PARTS_OF_SPEECH.map((part) => [part, new Set(synsets[part].keys())]));
  return { source: `WordNet ${version} (wordnet-db)`, lemmas, synsets };
}

/** The base forms of `word` that WordNet lists as `part`, the word itself first. */
export function baseForms(wordnet, word, part) {
  const lemma = word.toLowerCase();
  const index = wordnet.lemmas[part];
  const forms = index.has(lemma) ? [lemma] : [];
  for (const [suffix, ending] of DETACHMENT_RULES[part]) {
    if (lemma.length > suffix.length && lemma.endsWith(suffix)) {
      const candidate = lemma.slice(0, -suffix.length) + ending;
      if (index.has(candidate) && !forms.includes(candidate)) forms.push(candidate);
    }
  }
  return forms;
}

/** The parts of speech WordNet gives `word` (a single word or `_`-joined compound). */
export function partsOfSpeech(wordnet, word) {
  return PARTS_OF_SPEECH.filter((part) => baseForms(wordnet, word, part).length > 0);
}

/** The synsets of `word` as `part`, over all of its base forms. */
export function synsetsOf(wordnet, word, part) {
  return new Set(baseForms(wordnet, word, part).flatMap((form) => wordnet.synsets[part].get(form)));
}

/** Whether two different words are synonyms: WordNet lists both in one synset of `part`. */
export function areSynonyms(wordnet, first, second, part) {
  if (first === second) return false;
  const firstSynsets = synsetsOf(wordnet, first, part);
  return [...synsetsOf(wordnet, second, part)].some((synset) => firstSynsets.has(synset));
}
