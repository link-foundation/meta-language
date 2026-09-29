import { readFile } from 'node:fs/promises';

/**
 * The record of every canonical concept and operation meta-language defines:
 * its stable identity, the readable English phrase the identity spells, its
 * role (a concept is a noun phrase, an operation a verb phrase), definition,
 * constraints, the names other sources give it, and the former names that
 * still decode to it. Generated from parity/naming/canonical-concepts.json by
 * js/scripts/build-concept-records.mjs; the Rust runtime embeds the same file.
 */
export const CONCEPT_RECORDS = deepFreeze(
  JSON.parse(await readFile(new URL('./data/concept-records.json', import.meta.url), 'utf8')).concepts,
);

/** The vocabulary of the alias links that keep a concept's former names. */
export const FORMER_CONCEPT_ID_VOCABULARY = 'meta-language';

/**
 * Former concept identities and the identities that replaced them, as in
 * Rust's `FORMER_CONCEPT_IDS`. Every pair is also a former name of its
 * concept record.
 */
export const FORMER_CONCEPT_IDS = Object.freeze([
  ['grammar.repetition', 'grammar.counted-repetition'],
  ['grammar.zero-or-more', 'grammar.zero-or-more-repetition'],
  ['grammar.one-or-more', 'grammar.one-or-more-repetition'],
  ['grammar.optional', 'grammar.optional-expression'],
  ['grammar.non-terminal', 'grammar.nonterminal'],
  ['grammar.char-class', 'grammar.character-class'],
  ['grammar.char-range', 'grammar.character-range'],
  ['grammar.any-char', 'grammar.any-character'],
  ['grammar.empty', 'grammar.empty-expression'],
  ['grammar.boolean', 'grammar.boolean-value'],
  ['sequence', 'sequential-composition'],
  ['strong', 'strong-emphasis'],
  ['blockquote', 'block-quote'],
].map((pair) => Object.freeze(pair)));

const FORMER_IDS = new Map(FORMER_CONCEPT_IDS);
const BY_NAME = new Map();
for (const record of CONCEPT_RECORDS) {
  for (const name of [record.id, ...record.formerNames]) BY_NAME.set(name, record);
}

/** The current identity of a concept: `id` itself, or the identity that replaced it. */
export function currentConceptId(id) {
  return FORMER_IDS.get(id) ?? id;
}

/** Every concept record, in register order. */
export function conceptRecords() {
  return CONCEPT_RECORDS;
}

/** The record whose identity or former name is `name`, or `undefined`. */
export function conceptRecord(name) {
  return BY_NAME.get(name);
}

/** The records a source (such as `ebnf` or `Rust`) names `name`, in register order. */
export function conceptRecordsForSourceName(source, name) {
  return CONCEPT_RECORDS.filter(({ sourceAliases }) =>
    sourceAliases.some((alias) => alias.source === source && alias.name === name));
}

/**
 * Assigns a concept record to `network`: the concept link under the record's
 * identity with its definition, its English phrase, an alias link for every
 * name a source gives it, and an alias link in the meta-language vocabulary
 * for every former name. `nameOrRecord` is a record, an identity or a former
 * name. Returns the concept link and the number of links added.
 */
export function insertConceptRecord(network, nameOrRecord) {
  const record = typeof nameOrRecord === 'string' ? conceptRecord(nameOrRecord) : nameOrRecord;
  if (!record) throw new Error(`no concept record is named ${nameOrRecord}`);
  const before = network.len();
  const concept = network.internConcept(record.id, record.definition);
  network._insertConceptSyntaxMapping(concept, record.id, 'en', record.phrase, false);
  for (const { source, name } of record.sourceAliases) {
    network._insertConceptAliasLink(concept, source, name);
  }
  for (const former of record.formerNames) {
    network._insertConceptAliasLink(concept, FORMER_CONCEPT_ID_VOCABULARY, former);
  }
  return { concept, links: network.len() - before };
}

function deepFreeze(value) {
  if (value && typeof value === 'object') {
    Object.values(value).forEach(deepFreeze);
    Object.freeze(value);
  }
  return value;
}
