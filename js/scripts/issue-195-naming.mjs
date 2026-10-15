// The readable-naming check of docs/vision.md#readable-english-names. Every
// canonical name meta-language defines is a concept record in
// parity/naming/canonical-concepts.json (stable identity, readable phrase,
// role, definition, constraints and former names), and the check rejects:
//
// - a word that is neither an English word in WordNet 3.1 nor a registered
//   technical term (vocabulary);
// - a registered abbreviation, including the ones WordNet happens to list with
//   an unrelated meaning such as `char` or `ref` (abbreviation expansion);
// - a concept whose head is not a noun, or an operation or relation that does
//   not start with a verb (phrase role);
// - a phrase that names two different concepts, or an identity that is another
//   concept's former name (ambiguity);
// - a repeated identity, two names for one definition, two phrases WordNet
//   lists as synonyms word for word (a semantic duplicate, unless the records
//   state why the concepts differ), or a representation that names its concept
//   differently (duplicates);
// - a name found in the sources without a record, a record no source uses, a
//   former name still in use, or a former name a source still decodes that its
//   record does not list (full inventory coverage).

import { existsSync, readdirSync, readFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import { areSynonyms, baseForms, PARTS_OF_SPEECH, partsOfSpeech } from './english-vocabulary.mjs';
import { readProjectData, resolveProjectDataPath } from './project-data.mjs';
import { readStructuredDataLino } from '../src/structured-data-links.js';

export const NAMING_DIRECTORY = 'parity/naming';
export const CANONICAL_CONCEPTS = `${NAMING_DIRECTORY}/canonical-concepts.json`;
export const TECHNICAL_VOCABULARY = `${NAMING_DIRECTORY}/technical-vocabulary.json`;
export const ABBREVIATIONS = `${NAMING_DIRECTORY}/abbreviations.json`;
export const NAMING_FIXTURES = `${NAMING_DIRECTORY}/naming-fixtures.json`;

export const NAME_ROLES = Object.freeze(['concept', 'operation', 'relation']);
export const PROBLEM_KINDS = Object.freeze([
  'record',
  'vocabulary',
  'abbreviation',
  'phrase-role',
  'ambiguous',
  'duplicate',
  'inventory',
  'fixture',
]);

const readJson = (root, file) => readProjectData(path.join(root, file));

// Inventory extractors inspect the same records independently of their storage
// syntax. Executable grammar LiNo and host-language sources remain source text.
const inventoryText = (root, file) => {
  const actual = resolveProjectDataPath(path.join(root, file));
  if (!existsSync(actual)) return '';
  return file.endsWith('.json') ? JSON.stringify(readProjectData(actual)) : readFileSync(actual, 'utf8');
};

/** The committed concept records, technical vocabulary and abbreviation register. */
export function loadNamingRegisters(root) {
  const concepts = readJson(root, CANONICAL_CONCEPTS);
  const technical = readJson(root, TECHNICAL_VOCABULARY);
  const abbreviations = readJson(root, ABBREVIATIONS);
  return {
    records: concepts.concepts,
    exemptions: concepts.exemptions,
    vocabulary: {
      functionWords: new Set(technical.functionWords.words),
      technicalTerms: new Map(technical.terms.map((term) => [term.word, term])),
      abbreviations: new Map(abbreviations.abbreviations.map((entry) => [entry.abbreviation, entry])),
    },
  };
}

/** The last segment of an identity: `grammar.character-class` -> `character-class`. */
export const lastSegment = (id) => id.split(/\.|::/u).at(-1);

/** The readable phrase an identity spells: its last segment with spaces for hyphens. */
export const phraseOf = (id) => lastSegment(id).split('-').join(' ');

const wordsOf = (phrase) => phrase.split(' ').filter((word) => word !== '');

/** The words of a name as a source writes it: `grammar::value::absent-value` -> grammar value absent value. */
export const nameWords = (name) => name.split(/\.|::|:|-|_/u).filter((word) => word !== '');

/** The parts of speech of `word` in WordNet or the technical vocabulary. */
function wordParts(word, { wordnet, vocabulary }) {
  const technical = vocabulary.technicalTerms.get(word);
  return new Set([...partsOfSpeech(wordnet, word), ...(technical?.partsOfSpeech ?? [])]);
}

/** A word or compound (words joined by `_`, as WordNet writes them) that is a noun. */
function isNoun(words, context) {
  return wordParts(words.join('_'), context).has('noun');
}

/** The problems of the words of one name: vocabulary and abbreviations. */
function wordProblems(id, phrase, context) {
  const problems = [];
  for (const word of wordsOf(phrase)) {
    const abbreviation = context.vocabulary.abbreviations.get(word);
    if (abbreviation) {
      const expansions = abbreviation.expansions.map((expansion) => `"${expansion}"`).join(' or ');
      const ambiguity = abbreviation.expansions.length > 1 ? ' (an ambiguous abbreviation)' : '';
      problems.push({ kind: 'abbreviation', id, message: `${id}: "${word}" abbreviates ${expansions}${ambiguity}` });
      continue;
    }
    if (context.vocabulary.functionWords.has(word)) continue;
    if (wordParts(word, context).size === 0) {
      problems.push({
        kind: 'vocabulary',
        id,
        message: `${id}: "${word}" is not an English word in ${context.wordnet.source} or the technical vocabulary`,
      });
    }
  }
  return problems;
}

/** The vocabulary and abbreviation problems of a name as a source writes it (`sequence_2`, `grammar::value::character`). */
export const nameProblems = (name, context) => wordProblems(name, nameWords(name).join(' '), context);

/** The problem of the phrase role: concepts are noun phrases, operations and relations verb phrases. */
function roleProblem(id, phrase, role, context) {
  const words = wordsOf(phrase);
  if (words.length === 0) return { kind: 'phrase-role', id, message: `${id}: the phrase is empty` };
  if (role === 'concept') {
    const head = words.at(-1);
    const compound = words.slice(-2);
    const nounHead = isNoun([head], context) || (compound.length === 2 && isNoun(compound, context));
    if (context.vocabulary.functionWords.has(head) || !nounHead) {
      return { kind: 'phrase-role', id, message: `${id}: the concept "${phrase}" is not a noun phrase ("${head}" is not a noun)` };
    }
    return null;
  }
  const [verb] = words;
  const technicalVerb = context.vocabulary.technicalTerms.get(verb)?.partsOfSpeech.includes('verb') ?? false;
  if (!technicalVerb && !context.wordnet.lemmas.verb.has(verb)) {
    return { kind: 'phrase-role', id, message: `${id}: the ${role} "${phrase}" is not a verb phrase ("${verb}" is not a verb)` };
  }
  return null;
}

/**
 * A phrase with every word in its shortest WordNet noun base form, for
 * comparing names: `strings` is a lemma of its own but also the plural of
 * `string`.
 */
export function normalizedPhrase(phrase, { wordnet }) {
  return wordsOf(phrase.toLowerCase().split('-').join(' '))
    .map((word) => baseForms(wordnet, word, 'noun').reduce((shortest, form) => (form.length < shortest.length ? form : shortest), word))
    .join(' ');
}

const normalizedDefinition = (definition) => definition.toLowerCase().replace(/[^\p{L}\p{N}]+/gu, ' ').trim();

function recordProblems(record) {
  const problems = [];
  const { id } = record;
  const fail = (message) => problems.push({ kind: 'record', id, message: `${id}: ${message}` });
  if (typeof id !== 'string' || !/^[a-z]+(?:(?:\.|::|-)[a-z]+)*$/u.test(id)) fail('the identity is lower-case words');
  if (record.phrase !== phraseOf(String(id))) fail(`the phrase is "${phraseOf(String(id))}", the words of its identity`);
  if (!NAME_ROLES.includes(record.role)) fail(`the role is one of ${NAME_ROLES.join(', ')}`);
  if (typeof record.definition !== 'string' || !/^\p{Lu}.*\.$/u.test(record.definition)) fail('the definition is a sentence');
  const constraints = record.constraints;
  if (!Array.isArray(constraints) || constraints.length === 0 || constraints.some((item) => typeof item !== 'string' || item === '')) {
    fail('the constraints are a non-empty list of sentences');
  }
  if (!Array.isArray(record.formerNames)) fail('the former names are a list');
  const aliases = record.sourceAliases;
  if (!Array.isArray(aliases) || aliases.some((alias) => typeof alias?.source !== 'string' || typeof alias?.name !== 'string' || alias.source === '' || alias.name === '')) {
    fail('the source aliases are a list of source and name pairs');
  }
  const distinct = record.distinctFrom ?? [];
  if (!Array.isArray(distinct) || distinct.some((entry) => typeof entry?.id !== 'string' || !/^\p{Lu}.*\.$/u.test(entry.reason ?? ''))) {
    fail('each distinct-from entry names a concept and gives the reason as a sentence');
  }
  return problems;
}

// The parts of speech a word of a phrase is compared in: a concept's words are
// nouns and their modifiers, an operation or relation starts with its verb.
function comparedParts(role, index) {
  if (role === 'concept') return ['noun', 'adjective'];
  return index === 0 ? ['verb'] : PARTS_OF_SPEECH;
}

/**
 * Whether two phrases of one role say the same thing word for word: each word
 * equal or a WordNet synonym. Two inflections of one phrase are the same
 * phrase (an ambiguity), not synonyms.
 */
function synonymousPhrases(first, second, role, context) {
  const { wordnet } = context;
  const firstWords = wordsOf(first);
  const secondWords = wordsOf(second);
  if (firstWords.length !== secondWords.length || normalizedPhrase(first, context) === normalizedPhrase(second, context)) return false;
  return firstWords.every(
    (word, index) =>
      word === secondWords[index] ||
      comparedParts(role, index).some((part) => areSynonyms(wordnet, word, secondWords[index], part)),
  );
}

/**
 * The semantic duplicates: two concepts whose phrases WordNet lists as
 * synonyms, unless one of them states why they differ (`distinctFrom`).
 */
function synonymProblems(concepts, byId, context) {
  const problems = [];
  const stated = (record, other) => (record.distinctFrom ?? []).some((entry) => entry.id === other.id);
  for (const record of concepts) {
    for (const entry of record.distinctFrom ?? []) {
      const other = byId.get(entry.id);
      if (!other) {
        problems.push({ kind: 'record', id: record.id, message: `${record.id}: is distinct from the unrecorded concept ${entry.id}` });
      } else if (record.role !== other.role || !synonymousPhrases(record.phrase, other.phrase, record.role, context)) {
        problems.push({
          kind: 'record',
          id: record.id,
          message: `${record.id}: "${record.phrase}" and "${other.phrase}" are not synonyms, so no distinction is needed`,
        });
      }
    }
  }
  concepts.forEach((record, index) => {
    for (const other of concepts.slice(index + 1)) {
      if (record.role !== other.role || !synonymousPhrases(record.phrase, other.phrase, record.role, context)) continue;
      if (stated(record, other) || stated(other, record)) continue;
      problems.push({
        kind: 'duplicate',
        id: other.id,
        message: `${other.id}: "${other.phrase}" is a synonym of "${record.phrase}" (${record.id}) in ${context.wordnet.source}; one concept has one name`,
      });
    }
  });
  return problems;
}

/**
 * The problems of the concept records: each record's shape, words and role,
 * then the ambiguous and duplicate names across all of them.
 */
export function checkConceptRecords(records, context) {
  const problems = records.flatMap((record) => recordProblems(record));
  const byId = new Map();
  for (const record of records) {
    if (byId.has(record.id)) {
      problems.push({ kind: 'duplicate', id: record.id, message: `${record.id}: the identity is recorded twice` });
    }
    byId.set(record.id, record);
    if (typeof record.phrase !== 'string') continue;
    problems.push(...wordProblems(record.id, nameWords(String(record.id)).join(' '), context));
    const role = roleProblem(record.id, record.phrase, record.role, context);
    if (role) problems.push(role);
  }

  const concepts = records.filter((record) => record.represents === undefined && typeof record.phrase === 'string');
  const byPhrase = new Map();
  const byDefinition = new Map();
  for (const record of concepts) {
    const phrase = normalizedPhrase(record.phrase, context);
    const named = byPhrase.get(phrase);
    if (named && named.id !== record.id) {
      problems.push({
        kind: 'ambiguous',
        id: record.id,
        message: `${record.id}: "${record.phrase}" also names ${named.id}; each phrase names one concept`,
      });
    }
    byPhrase.set(phrase, record);
    if (typeof record.definition !== 'string') continue;
    const definition = normalizedDefinition(record.definition);
    const defined = byDefinition.get(definition);
    if (defined && defined.id !== record.id) {
      problems.push({
        kind: 'duplicate',
        id: record.id,
        message: `${record.id}: has the definition of ${defined.id}; one concept has one name`,
      });
    }
    byDefinition.set(definition, record);
  }

  for (const record of records.filter((candidate) => candidate.represents !== undefined)) {
    const concept = byId.get(record.represents);
    if (!concept) {
      problems.push({ kind: 'record', id: record.id, message: `${record.id}: represents the unrecorded concept ${record.represents}` });
    } else if (normalizedPhrase(record.phrase, context) !== normalizedPhrase(concept.phrase, context)) {
      problems.push({
        kind: 'duplicate',
        id: record.id,
        message: `${record.id}: names the concept ${concept.id} "${record.phrase}" instead of "${concept.phrase}"`,
      });
    }
  }

  problems.push(...synonymProblems(concepts, byId, context));

  const formerOwners = new Map();
  for (const record of records) {
    for (const former of Array.isArray(record.formerNames) ? record.formerNames : []) formerOwners.set(former, record);
  }
  for (const record of records) {
    const owner = formerOwners.get(record.id);
    if (owner && owner.id !== record.id) {
      problems.push({ kind: 'ambiguous', id: record.id, message: `${record.id}: the identity is a former name of ${owner.id}` });
    }
  }
  return problems;
}

// The block of `text` from the line containing `start` to the next `end`.
function block(text, start, end) {
  const from = text.indexOf(start);
  if (from < 0) return '';
  const to = text.indexOf(end, from);
  return text.slice(from, to < 0 ? undefined : to);
}

const captures = (text, pattern) => [...text.matchAll(pattern)].map(([, name]) => name);

const RUST_STRING = String.raw`"((?:[^"\\]|\\.)*)"`;
const rustString = (literal) => JSON.parse(`"${literal}"`);

/**
 * The definitions and syntax aliases a Rust concept table gives each identity:
 * `{ id: "…", definition: "…", syntax: &[("source", "name"), …] }`.
 */
function conceptDefinitions(text, aliasField) {
  const entry = new RegExp(String.raw`id: ${RUST_STRING},\s*definition:\s*${RUST_STRING},[\s\S]*?${aliasField}: &\[([\s\S]*?)\]`, 'gu');
  const alias = new RegExp(String.raw`\(\s*${RUST_STRING},\s*${RUST_STRING},?\s*\)`, 'gu');
  return [...text.matchAll(entry)].map(([, name, definition, aliases]) => ({
    name: rustString(name),
    definition: rustString(definition),
    sourceAliases: [...aliases.matchAll(alias)].map(([, source, alias]) => ({ source: rustString(source), name: rustString(alias) })),
  }));
}

// Rust text without its former-name tables and constants: they keep former
// names decodable, which is not a use (the former-name tables below read them).
const withoutFormerNames = (text) =>
  text.replace(/^(?:pub )?const FORMER_[A-Z_]+: &\[[^\n]*= &\[\n[\s\S]*?\n\];\n/gmu, '').replace(/^(?:pub )?const FORMER_[A-Z_]+: &str = "[^"]*";\n/gmu, '');

// `grammar::expression::choice` -> `grammar.choice`, `grammar::value::absent-value`
// -> `grammar.value.absent-value`: the runtime and expression tags name the
// grammar concepts themselves.
const grammarLinkRecord = (term) =>
  `grammar.${term.replace(/^grammar::/u, '').replace(/^runtime::/u, '').replace(/^expression::/u, '').split('::').join('.')}`;

const grammarConceptRecord = (name) => name.replace(/^grammar::concept::/u, 'grammar.');

// The native grammars of the catalog, one Links Notation file per language.
const NATIVE_GRAMMAR_DIRECTORY = 'parity/grammars/native';
const repositoryRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const NATIVE_GRAMMAR_FILES = readdirSync(path.join(repositoryRoot, NATIVE_GRAMMAR_DIRECTORY))
  .filter((file) => file.endsWith('.lino'))
  .sort()
  .map((file) => `${NATIVE_GRAMMAR_DIRECTORY}/${file}`);
NATIVE_GRAMMAR_FILES.push('parity/grammars/native-text.lino');

/** The source a native grammar file gives its rule names: `native:json` for parity/grammars/native/json.lino. */
export const nativeGrammarSource = (file) => `native:${path.basename(file, '.lino')}`;

// `(rule NAME KIND … (concept ID) …)`: each rule of a native grammar and its concept.
const nativeRuleConcepts = (text) => [...text.matchAll(/^\(rule ([a-z0-9_]+) [^\n]*?\(concept ([^()\s]+)\)/gmu)].map(([, rule, concept]) => ({ rule, concept }));

/**
 * Where meta-language defines canonical names. Each inventory reads the names
 * from its source files and gives the concept record each name belongs to; an
 * inventory of `words` has no records (generated names such as inferred rule
 * names) and only its words are checked.
 */
const BASE_NAME_INVENTORIES = Object.freeze([
  {
    inventory: 'closed empty primitive Rust option decisions',
    files: ['js/src/translation/rust-empty-option-rules.js', 'rust/src/translation/rust/empty_option_rules.rs'],
    extract: (text) => captures(text, /^(?:export function|pub(?: const)? fn) ([a-zA-Z_]+)\(/gmu)
      .map(name => name.replace(/([a-z])([A-Z])/gu, '$1-$2').replaceAll('_', '-').toLowerCase()),
    recordOf: (name) => `translation.${name}`,
  },
  {
    inventory: 'closed empty primitive Rust option source adapters',
    files: ['js/src/translation/rust-empty-options.js', 'rust/src/translation/rust/empty_options.rs', 'rust/src/translation/source_empty_options.rs'],
    extract: (text) => captures(text, /^\s*(?:export function|pub(?:\((?:crate|super)\))? fn) (readRustEmptyOptionResult|readRustEmptyOptionBody|createRustEmptyOptionProof|finishRustEmptyOptionSource|checkRustEmptyOptionProof|read_empty_option_result|empty_option_function|empty_option_attribute_guard|empty_option_item_guard|empty_option_main_guard|finish_empty_option_source|validate_empty_option_records|validate_source_empty_options|policy)\(/gmu)
      .map(name => name.replace(/([a-z])([A-Z])/gu, '$1-$2').replaceAll('_', '-').toLowerCase()),
    recordOf: (name) => `translation.${({
      'read-empty-option-result': 'read-rust-empty-option-result',
      'empty-option-function': 'read-rust-empty-option-function',
      'empty-option-attribute-guard': 'check-rust-empty-option-source-attributes',
      'empty-option-item-guard': 'check-rust-empty-option-source-item',
      'empty-option-main-guard': 'check-rust-empty-option-main',
      'finish-empty-option-source': 'finish-rust-empty-option-source',
      'validate-empty-option-records': 'validate-rust-empty-option-records',
      'validate-source-empty-options': 'check-rust-empty-option-proof',
      'policy': 'read-rust-empty-option-policy',
    })[name] ?? name}`,
  },
  {
    inventory: 'closed empty primitive Rust option fixture builders',
    files: ['js/scripts/generate-rust-empty-option-fixtures.mjs'],
    extract: (text) => captures(text, /^export function (rustEmptyOptionFixture)\(/gmu).map(() => 'rust-empty-option-fixture'),
    recordOf: () => 'repository.create-rust-empty-option-fixture',
  },
  {
    inventory: 'closed empty primitive Rust option model selections',
    files: ['js/src/translation/rust.js', 'rust/src/translation/rust.rs', 'js/scripts/generate-rust-empty-option-fixtures.mjs'],
    extract: (text) => captures(text, /\b(emptyOptionSourceModel|empty_option_source_model|RUST_EMPTY_OPTION_SOURCE_MODEL)\b/gu)
      .map(name => name.replace(/([a-z])([A-Z])/gu, '$1-$2').replaceAll('_', '-').toLowerCase()),
    recordOf: () => 'translation.rust-empty-option-source-model-option',
  },
  {
    inventory: 'typed program closed empty primitive Rust option model selection',
    files: ['js/src/typed-program-links.js'],
    extract: (text) => captures(text, /\b(rustEmptyOptionSourceModel)\b/gu).map(() => 'rust-empty-option-source-model'),
    recordOf: () => 'translation.typed-program-rust-empty-option-source-model-option',
  },
  {
    inventory: 'typed program closed empty primitive Rust option proof roles',
    files: ['js/src/typed-program-checked-projection.js', 'rust/src/translation/ir/declarations.rs', 'rust/src/translation/surface.rs'],
    extract: (text) => captures(text, /\b(sourceEmptyOption|source_empty_option)\b/gu).map(() => 'source-empty-option'),
    recordOf: () => 'translation.program-source-empty-option-proof-field',
  },
  {
    inventory: 'owned empty primitive Rust option source relation roles',
    files: ['js/src/translation/rust-empty-options.js', 'rust/src/translation/source_empty_options.rs'],
    extract: (text) => captures(text, /\b(construction|canonicalPath|externalOptionRoots|declarationNames)\b/gu)
      .map(name => name.replace(/([a-z])([A-Z])/gu, '$1-$2').toLowerCase()),
    recordOf: (name) => `translation.${({
      'construction': 'program-source-empty-option-constructor-proof-field',
      'canonical-path': 'program-source-canonical-owner-path-field',
      'external-option-roots': 'program-source-external-option-root-list-field',
      'declaration-names': 'program-source-declared-function-name-list-field',
    })[name]}`,
  },
  {
    inventory: 'native typed program field name reader',
    files: ['rust/src/typed_program_links.rs'],
    extract: (text) => captures(text, /^pub(?:\(super\))? fn (field_of)\(/gmu).map(() => 'field-of'),
    recordOf: () => 'translation.read-typed-program-field-name',
  },
  {
    inventory: 'whole linked program checked projection selection',
    files: ['js/src/typed-program-checked-projection.js'],
    extract: (text) => captures(text, /\b(wholeProgram)\b/gu).map(() => 'whole-program'),
    recordOf: () => 'translation.linked-program-whole-projection-option',
  },

  {
    inventory: 'shared numeric function admission operations',
    files: ['js/src/shared-numeric-function-rules.js', 'rust/src/shared_numeric_function_rules.rs'],
    extract: (text) => captures(text, /^(?:export function|pub(?: const)? fn) ([a-zA-Z_]+)\(/gmu)
      .map(name => name.replace(/([a-z])([A-Z])/gu, '$1-$2').replaceAll('_', '-').toLowerCase()),
    recordOf: (name) => `formal.${name === 'accept-shared-numeric-unary' || name === 'accept-shared-numeric-binary' ? `${name}-operation` : name}`,
  },
  {
    inventory: 'shared numeric function graph operations',
    files: ['js/src/shared-numeric-function-links.js', 'js/src/shared-numeric-function-model.js',
      'rust/src/shared_numeric_function_links.rs', 'rust/src/shared_numeric_function_model.rs'],
    extract: (text) => captures(text, /^(?:export function|pub(?:\(crate\))? fn) (encodeSharedNumericFunctionDefinition|readSharedNumericFunctionModel|normalizeSharedNumericFunctionLiterals|createSharedNumericFunctionMeaning|encode_shared_numeric_function_definition|read_shared_numeric_function_model|normalize_shared_numeric_function_literals|create_shared_numeric_function_meaning|validate_shared_numeric_origin_context)\(/gmu)
      .map(name => name.replace(/([a-z])([A-Z])/gu, '$1-$2').replaceAll('_', '-').toLowerCase()),
    recordOf: (name) => `formal.${name}`,
  },
  {
    inventory: 'shared numeric linked context flag operation',
    files: ['rust/src/shared_numeric_function_model.rs'],
    extract: (text) => captures(text, /^fn (context_flag_supported)\(/gmu)
      .map(name => name.replaceAll('_', '-')),
    recordOf: () => 'formal.validate-shared-numeric-linked-context-flag',
  },
  {
    inventory: 'shared numeric value bit pattern',
    files: ['js/src/shared-numeric-function-model.js', 'rust/src/shared_numeric_function_model.rs'],
    extract: (text) => [...text.matchAll(/\.with(?:Term\('([^']+)'\)|_term\("([^"]+)"\))/gu)].map(([, javascript, native]) => javascript ?? native)
      .filter(name => name === 'binary floating point bit pattern')
      .map(name => name.replaceAll(' ', '-')),
    recordOf: (name) => `formal.${name}`,
  },
  {
    inventory: 'shared numeric function model declaration',
    files: ['js/src/shared-numeric-function-model.js', 'rust/src/shared_numeric_function_model.rs'],
    extract: (text) => captures(text, /^(?:export|pub(?:\(crate\))?) const (SHARED_NUMERIC_FUNCTION_MODEL)\b/gmu)
      .map(name => name.replaceAll('_', '-').toLowerCase()),
    recordOf: () => 'translation.closed-binary-floating-point-function',
  },
  {
    inventory: 'shared numeric decision emitter formatting',
    files: ['js/scripts/generate-shared-numeric-function-rules.mjs'],
    extract: (text) => captures(text, /^function (formatDecisionExpression)\(/gmu)
      .map(name => name.replace(/([a-z])([A-Z])/gu, '$1-$2').toLowerCase()),
    recordOf: (name) => `repository.${name}`,
  },
  {
    inventory: 'reviewed generated policy source style operations',
    files: ['js/scripts/resource-description-reviewed-styles.mjs', 'js/scripts/product-rule-style-corrections.mjs'],
    extract: (text) => captures(text, /^export function ([a-zA-Z_]+)\(/gmu)
      .map(name => name.replace(/([a-z])([A-Z])/gu, '$1-$2').replaceAll('_', '-').toLowerCase()),
    recordOf: (name) => `repository.${name}`,
  },
  {
    inventory: 'formal function graph adapters',
    files: ['js/src/formal-function-links.js', 'rust/src/formal_function_links.rs'],
    extract: (text) => captures(text, /^(?:function|fn) (redirectSignature|redirect_signature|importSelection|import_selection)\(/gmu)
      .map(name => name.replace(/([a-z])([A-Z])/gu, '$1-$2').replaceAll('_', '-').toLowerCase()),
    recordOf: (name) => `formal.${name === 'redirect-signature' ? 'redirect-function-signature' : 'import-function-selection'}`,
  },
  {
    inventory: 'Rust source profile adapter operations',
    files: ['js/src/translation/rust-float-predicates.js', 'rust/src/translation/rust/float_predicates.rs',
      'js/src/translation/rust-scalar-parameters.js', 'rust/src/translation/rust/scalar_parameters.rs'],
    extract: (text) => captures(text, /^(?:export function|pub\(super\) fn) ([a-zA-Z_]+)\(/gmu)
      .map(name => name.replace(/([a-z])([A-Z])/gu, '$1-$2').replaceAll('_', '-').toLowerCase()),
    recordOf: (name) => `translation.${name === 'expression' ? 'create-rust-floating-point-predicate-expression' : name === 'operation-expression' ? 'create-rust-floating-point-operation-expression' : name === 'constant-expression' ? 'create-rust-floating-point-constant-expression' : name.replaceAll('float-predicate', 'floating-point-predicate').replaceAll('float-operation', 'floating-point-operation').replaceAll('float-constant', 'floating-point-constant')}`,
  },
  {
    inventory: 'Rust source signature admission operations',
    files: ['js/src/translation/rust-source-signature-rules.js', 'rust/src/translation/source_signature_rules.rs'],
    extract: (text) => captures(text, /^(?:export function|pub(?: const)? fn) ([a-zA-Z_]+)\(/gmu)
      .map(name => name.replace(/([a-z])([A-Z])/gu, '$1-$2').replaceAll('_', '-').toLowerCase()),
    recordOf: (name) => `translation.${name}`,
  },
  {
    inventory: 'Rust read only query admission operations',
    files: ['js/src/translation/rust-read-only-query-rules.js', 'rust/src/translation/rust/read_only_query_rules.rs'],
    extract: (text) => captures(text, /^(?:export function|pub(?: const)? fn) ([a-zA-Z_]+)\(/gmu)
      .map(name => name.replace(/([a-z])([A-Z])/gu, '$1-$2').replaceAll('_', '-').toLowerCase()),
    recordOf: (name) => `translation.${name}`,
  },
  {
    inventory: 'Rust read only query source adapters',
    files: ['js/src/translation/rust-read-only-queries.js', 'rust/src/translation/rust/expressions/read_only_queries.rs'],
    extract: (text) => captures(text, /^\s*(?:export function|pub\(super\) fn) ([a-zA-Z_]+)\(/gmu)
      .map(name => name.replace(/([a-z])([A-Z])/gu, '$1-$2').replaceAll('_', '-').toLowerCase()),
    recordOf: (name) => `translation.${name.startsWith('read-rust-') ? name : name.replace(/^read-/u, 'read-rust-')}`,
  },
  {
    inventory: 'selected scalar loop admission operations',
    files: ['js/src/translation/scalar-loop-rules.js', 'rust/src/translation/scalar_loop_rules.rs'],
    extract: (text) => captures(text, /^(?:export function|pub(?: const)? fn) ([a-zA-Z_]+)\(/gmu)
      .map(name => name.replace(/([a-z])([A-Z])/gu, '$1-$2').replaceAll('_', '-').toLowerCase()),
    recordOf: (name) => `translation.${name.replace('-math-operation', '-mathematical-operation')}`,
  },
  {
    inventory: 'selected scalar loop source adapters',
    files: ['js/src/translation/rust-scalar-loops.js',
      'rust/src/translation/rust/scalar_loops.rs',
      'rust/src/translation/source_loops/validation.rs'],
    extract: (text) => captures(text, /^\s*(?:export function|pub(?:\(super\))? fn) (checkRustScalarLoopProof|readRustScalarLoop|isJavaScriptScalarLoop|readJavaScriptScalarLoop|finishJavaScriptScalarLoops|read_scalar_loop|is_scalar_loop|finish_scalar_loops|validate_source_loop|validate_source_loop_at)\(/gmu)
      .map(name => name.replace(/([a-z])([A-Z])/gu, '$1-$2').replaceAll('_', '-').toLowerCase().replace('java-script', 'javascript')),
    recordOf: (name) => `translation.${({
      'check-rust-scalar-loop-proof': 'check-source-scalar-loop-proof',
      'validate-source-loop': 'check-source-scalar-loop-proof',
      'validate-source-loop-at': 'check-source-scalar-loop-proof',
      'read-scalar-loop': 'read-rust-scalar-loop',
    })[name] ?? name}`,
  },
  {
    inventory: 'selected JavaScript scalar loop source adapters',
    files: ['js/src/translation/javascript-scalar-loops.js', 'rust/src/translation/javascript/scalar_loops.rs'],
    extract: (text) => captures(text, /^\s*(?:export function|pub\(super\) fn) (isJavaScriptScalarLoop|readJavaScriptScalarLoop|finishJavaScriptScalarLoops|read_scalar_loop|is_scalar_loop|finish_scalar_loops)\(/gmu)
      .map(name => name.replace(/([a-z])([A-Z])/gu, '$1-$2').replaceAll('_', '-').toLowerCase().replace('java-script', 'javascript')),
    recordOf: (name) => `translation.${({
      'is-javascript-scalar-loop': 'detect-javascript-scalar-loop',
      'is-scalar-loop': 'detect-javascript-scalar-loop',
      'finish-scalar-loops': 'finish-javascript-scalar-loops',
      'read-scalar-loop': 'read-javascript-scalar-loop',
    })[name] ?? name}`,
  },
  {
    inventory: 'owned scalar loop proof entities',
    files: ['rust/src/translation/source_loops.rs'],
    extract: (text) => captures(text, /^pub struct (ScalarLoopState|ScalarLoopWrite|ScalarLoopTransition|ScalarLoopInterfaceType|ScalarLoopInterface|ScalarLoopContinuation|SourceLoop)\b/gmu)
      .map(name => name.replace(/([a-z])([A-Z])/gu, '$1-$2').toLowerCase()),
    recordOf: (name) => `translation.${({
      'scalar-loop-state': 'scalar-loop-state-slot',
      'scalar-loop-write': 'scalar-loop-state-assignment',
      'scalar-loop-transition': 'scalar-loop-state-transition',
      'source-loop': 'source-scalar-loop-proof',
      'scalar-loop-continuation': 'scalar-loop-declaration-continuation',
    })[name] ?? name}`,
  },
  {
    inventory: 'scalar loop policy and namespace adapters',
    files: ['rust/src/translation/source_loops.rs', 'rust/src/typed_program_links/checked/loop_modules.rs'],
    extract: (text) => captures(text, /^pub(?:\((?:crate|super)\))? fn (scalar_loop_policy|scalar_loop_snapshot_binding|project_function|project_module)\(/gmu)
      .map(name => name.replaceAll('_', '-')),
    recordOf: (name) => `translation.${({
      'scalar-loop-policy': 'read-scalar-loop-source-policy',
      'scalar-loop-snapshot-binding': 'accept-scalar-loop-snapshot-binding',
      'project-function': 'project-checked-function-surface',
      'project-module': 'project-scalar-loop-module',
    })[name]}`,
  },
  {
    inventory: 'Rust source header policy and rendering operations',
    files: ['js/src/translation/rust-source-header-emission-rules.js', 'rust/src/translation/source_header_emission_rules.rs'],
    extract: (text) => captures(text, /^(?:export function|pub(?: const)? fn) ([a-zA-Z_]+)\(/gmu)
      .map(name => name.replace(/([a-z])([A-Z])/gu, '$1-$2').replaceAll('_', '-').toLowerCase()),
    recordOf: (name) => `translation.${name}`,
  },
  {
    inventory: 'closed primitive borrowed call decisions',
    files: ['js/src/translation/rust-borrowed-call-interface-rules.js', 'rust/src/translation/borrowed_call_interface_rules.rs'],
    extract: (text) => captures(text, /^(?:export function|pub(?: const)? fn) ([a-zA-Z_]+)\(/gmu)
      .map(name => name.replace(/([a-z])([A-Z])/gu, '$1-$2').replaceAll('_', '-').toLowerCase()),
    recordOf: (name) => `translation.${name}`,
  },
  {
    inventory: 'closed primitive borrowed call adapters',
    files: ['js/src/translation/rust-borrowed-call-interface.js', 'rust/src/translation/borrowed_call_interface.rs'],
    extract: (text) => captures(text, /^(?:export function|pub(?:\(crate\))? fn) (readRustBorrowedCallInterface|renderRustBorrowedCall|read_rust_borrowed_call_interface|render_rust_borrowed_call|borrowed_call_policy)\(/gmu)
      .map(name => name.replace(/([a-z])([A-Z])/gu, '$1-$2').replaceAll('_', '-').toLowerCase()),
    recordOf: (name) => `translation.${name === 'borrowed-call-policy' ? 'read-rust-borrowed-call-policy' : name}`,
  },
  {
    inventory: 'selected interface policy entities',
    files: ['js/src/translation/rust-borrowed-call-interface.js', 'js/src/translation/rust-primitive-owners.js', 'js/src/translation/javascript-scoped-binding-emission.js', 'js/src/translation/rust-empty-options.js'],
    extract: (text) => captures(text, /^export const (RUST_BORROWED_CALL_INTERFACE_POLICY|RUST_PRIMITIVE_OWNER_POLICY|JAVASCRIPT_SCOPED_BINDING_EMISSION_POLICY|RUST_EMPTY_OPTION_POLICY)\b/gmu)
      .map(name => name.replaceAll('_', '-').toLowerCase()),
    recordOf: (name) => `translation.${name}`,
  },
  {
    inventory: 'owned borrowed interface entity',
    files: ['rust/src/translation/borrowed_call_interface.rs'],
    extract: (text) => captures(text, /^pub struct (BorrowedCallInterface)\b/gmu).map(() => 'borrowed-call-interface'),
    recordOf: () => 'translation.rust-borrowed-call-interface',
  },
  {
    inventory: 'qualified primitive owner decisions',
    files: ['js/src/translation/rust-primitive-owner-rules.js', 'rust/src/translation/rust/primitive_owner_rules.rs'],
    extract: (text) => captures(text, /^(?:export function|pub(?: const)? fn) ([a-zA-Z_]+)\(/gmu)
      .map(name => name.replace(/([a-z])([A-Z])/gu, '$1-$2').replaceAll('_', '-').toLowerCase()),
    recordOf: (name) => `translation.${name}`,
  },
  {
    inventory: 'qualified primitive owner adapters',
    files: ['js/src/translation/rust-primitive-owners.js', 'rust/src/translation/rust/primitive_owners.rs', 'rust/src/translation/source_primitive_owners.rs'],
    extract: (text) => captures(text, /^\s*(?:export function|function|pub(?:\((?:crate|super)\))? fn|fn) (acceptRustPrimitiveOwnerImportName|readRustStandardPrimitivePosition|createRustPrimitiveOwnerProof|prepareRustPrimitiveOwnerProof|checkRustPrimitiveOwnerProof|accept_import_name|read_standard_primitive_position|create_primitive_owner_proof|prepare_primitive_owner_proof|validate_primitive_owner_records|validate_source_primitive_owners|policy|bodySites|body_sites|exact|spelling|owner)\(/gmu)
      .map(name => name.replace(/([a-z])([A-Z])/gu, '$1-$2').replaceAll('_', '-').toLowerCase()),
    recordOf: (name) => `translation.${({
      'accept-import-name': 'accept-rust-primitive-owner-import-name',
      'read-standard-primitive-position': 'read-rust-standard-primitive-position',
      'create-primitive-owner-proof': 'create-rust-primitive-owner-proof',
      'prepare-primitive-owner-proof': 'prepare-rust-primitive-owner-proof',
      'validate-primitive-owner-records': 'validate-rust-primitive-owner-records',
      'validate-source-primitive-owners': 'check-rust-primitive-owner-proof',
      'policy': 'read-rust-primitive-owner-policy',
      'body-sites': 'read-rust-primitive-owner-body-sites',
      'exact': 'check-rust-primitive-owner-record-fields',
      'spelling': 'read-rust-primitive-owner-path-spelling',
      'owner': 'check-rust-primitive-owner-position',
    })[name] ?? name}`,
  },
  {
    inventory: 'owned JavaScript scoped binding decisions',
    files: ['js/src/translation/javascript-scoped-binding-emission-rules.js', 'rust/src/translation/scoped_binding_emission_rules.rs'],
    extract: (text) => captures(text, /^(?:export function|pub(?: const)? fn) ([a-zA-Z_]+)\(/gmu)
      .map(name => name.replace(/([a-z])([A-Z])/gu, '$1-$2').replaceAll('_', '-').toLowerCase().replace('java-script', 'javascript')),
    recordOf: (name) => `translation.${name}`,
  },
  {
    inventory: 'owned JavaScript scoped binding adapters',
    files: ['js/src/translation/javascript-scoped-binding-emission.js', 'rust/src/translation/scoped_binding_emission.rs'],
    extract: (text) => captures(text, /^(?:export function|pub(?:\(crate\))? fn|fn) (readJavaScriptScopedBindingEmission|validate_model|validate_program|validate_origin_context|function_role|main_role|value_type|flag_form|validate_raw_program)(?:<'[^>]+>)?\(/gmu)
      .map(name => name.replace(/([a-z])([A-Z])/gu, '$1-$2').replaceAll('_', '-').toLowerCase().replace('java-script', 'javascript')),
    recordOf: (name) => `translation.${({
      'validate-model': 'validate-javascript-scoped-binding-emission-model',
      'validate-program': 'read-javascript-scoped-binding-emission',
      'validate-origin-context': 'validate-javascript-scoped-binding-origin-context',
      'function-role': 'read-javascript-function-binding-role',
      'main-role': 'check-javascript-main-binding-role',
      'value-type': 'check-javascript-scoped-binding-value-type',
      'flag-form': 'read-javascript-scoped-binding-origin-flag',
      'validate-raw-program': 'validate-javascript-scoped-binding-raw-program',
    })[name] ?? name}`,
  },
  {
    inventory: 'direct JavaScript program emission',
    files: ['js/src/translation/emit-javascript.js', 'rust/src/translation/emit_javascript.rs'],
    extract: (text) => captures(text, /^(?:export function|pub fn) (emitJavaScript|emit_javascript|emit_javascript_with_options)\(/gmu)
      .map(name => name.replace(/([a-z])([A-Z])/gu, '$1-$2').replaceAll('_', '-').toLowerCase()),
    recordOf: () => 'translation.emit-javascript-program',
  },
  {
    inventory: 'JavaScript emission option entity',
    files: ['rust/src/translation/emit_javascript.rs'],
    extract: (text) => captures(text, /^pub struct (JavaScriptEmissionOptions)\b/gmu).map(() => 'javascript-emission-options'),
    recordOf: () => 'translation.javascript-emission-options',
  },
  {
    inventory: 'selected source and binding option roles',
    files: ['js/src/translation/emit-javascript.js', 'rust/src/translation/emit_javascript.rs', 'js/src/typed-program-links.js', 'rust/src/typed_program_links.rs', 'js/src/index.d.ts', 'js/src/translation/rust.js', 'rust/src/translation/rust.rs'],
    extract: (text) => [...text.matchAll(/\b(scopedBindingModel|scoped_binding_model|javaScriptScopedBindingEmissionModel|java_script_scoped_binding_emission_model|rustPrimitiveOwnerModel|primitiveOwnerModel|primitive_owner_model)\b/gu)]
      .map(([,name]) => name.replace(/([a-z])([A-Z])/gu, '$1-$2').replaceAll('_', '-').toLowerCase().replace('java-script', 'javascript')),
    recordOf: (name) => `translation.${({
      'scoped-binding-model': 'javascript-scoped-binding-model-option',
      'javascript-scoped-binding-emission-model': 'typed-program-javascript-scoped-binding-emission-model-option',
      'rust-primitive-owner-model': 'typed-program-rust-primitive-owner-model-option',
      'primitive-owner-model': 'rust-primitive-owner-model-option',
    })[name]}`,
  },
  {
    inventory: 'typed qualified owner facet role',
    files: ['js/src/typed-program-checked-projection.js', 'rust/src/translation/ir/declarations.rs'],
    extract: (text) => [...text.matchAll(/\b(sourcePrimitiveOwners|source_primitive_owners)\b/gu)].map(() => 'source-primitive-owners'),
    recordOf: () => 'translation.program-source-primitive-owner-proof-field',
  },
  {
    inventory: 'typed empty optional construct',
    files: ['rust/src/translation/ir.rs', 'rust/src/translation/surface.rs'],
    extract: (text) => captures(text, /^    (None) \{/gmu).map(() => 'none'),
    recordOf: () => 'translation.optional-empty-expression',
  },
  {
    inventory: 'typed native checked program admission',
    files: ['rust/src/typed_program_links.rs'],
    extract: (text) => captures(text, /^pub(?:\(super\))? fn (validate_checked_program)\(/gmu).map(name => name.replaceAll('_','-')),
    recordOf: () => 'translation.check-typed-linked-program',
  },
  {
    inventory: 'selected source and binding fixture builders',
    files: ['js/scripts/generate-rust-borrowed-call-interface-fixtures.mjs', 'js/scripts/generate-rust-primitive-owner-fixtures.mjs', 'js/scripts/generate-javascript-scoped-binding-emission-fixtures.mjs'],
    extract: (text) => captures(text, /^export function (rustBorrowedCallInterfaceFixture|rustPrimitiveOwnerFixture|javaScriptScopedBindingEmissionFixture)\(/gmu)
      .map(name => name.replace(/([a-z])([A-Z])/gu, '$1-$2').toLowerCase().replace('java-script','javascript')),
    recordOf: (name) => `repository.create-${name}`,
  },
  {
    inventory: 'reviewed Rust source header library styles',
    files: ['js/scripts/rust-source-header-library-style.mjs'],
    extract: (text) => captures(text, /^export function ([a-zA-Z_]+)\(/gmu).map(name => name.replace(/([a-z])([A-Z])/gu,'$1-$2').toLowerCase()),
    recordOf: (name) => `repository.${name}`,
  },
  {
    inventory: 'Rust source header emission adapters',
    files: ['js/src/translation/rust-source-header-emission.js', 'rust/src/translation/source_header_emission.rs'],
    extract: (text) => captures(text, /^(?:export function|pub(?:\(crate\))? fn|(?:const )?fn) (readRustSourceHeaderEmission|renderRustSourceHeaderEmission|readRustPrimitiveSourceHeaders|validateRustPrimitiveValueExpressions|read_rust_source_header_emission|render_rust_source_header_emission|render_rust_source_header_emission_with_reference|read_rust_primitive_source_headers|validate_rust_primitive_value_expressions|validate_rust_source_header_emission_model|value_type|policy|semantics_name)\(/gmu)
      .map(name => name.replace(/([a-z])([A-Z])/gu, '$1-$2').replaceAll('_', '-').toLowerCase()),
    recordOf: (name) => `translation.${({
      'value-type': 'check-rust-source-header-value-type',
      'policy': 'read-rust-source-header-emission-policy',
      'semantics-name': 'read-rust-source-header-semantic-qualifier',
      'render-rust-source-header-emission-with-reference': 'render-rust-source-header-emission',
    })[name] ?? name}`,
  },
  {
    inventory: 'selected source policy declarations',
    files: ['js/src/translation/rust-scalar-loops.js', 'js/src/translation/rust-source-header-emission.js'],
    extract: (text) => captures(text, /^export const (RUST_SCALAR_LOOP_POLICY|RUST_SOURCE_HEADER_EMISSION_POLICY)\b/gmu)
      .map(name => name.replaceAll('_', '-').toLowerCase()),
    recordOf: (name) => `translation.${name === 'rust-scalar-loop-policy' ? 'scalar-loop-source-policy' : 'rust-source-header-emission-policy'}`,
  },
  {
    inventory: 'selected Rust source fixture generators',
    files: ['js/scripts/generate-rust-read-only-query-fixtures.mjs', 'js/scripts/generate-rust-source-header-emission-fixtures.mjs'],
    extract: (text) => captures(text, /^export function (rustReadOnlyQueryFixture|rustSourceHeaderEmissionFixture)\(/gmu)
      .map(name => name.replace(/([a-z])([A-Z])/gu, '$1-$2').toLowerCase()),
    recordOf: (name) => `repository.create-${name}`,
  },
  {
    inventory: 'direct Rust program emission operations',
    files: ['js/src/translation/emit-rust.js', 'rust/src/translation/emit_rust.rs'],
    extract: (text) => captures(text, /^(?:export function|pub fn) (emitRust|emit_rust|emit_rust_with_options)\(/gmu)
      .map(name => name.replace(/([a-z])([A-Z])/gu, '$1-$2').replaceAll('_', '-').toLowerCase()),
    recordOf: () => 'translation.emit-rust-program',
  },
  {
    inventory: 'native source emission option entities',
    files: ['rust/src/translation/emit_rust.rs', 'rust/src/typed_program_links.rs', 'js/src/index.d.ts'],
    extract: (text) => captures(text, /^(?:pub struct|export interface) (RustEmissionOptions|TypedProgramLinksEmissionOptions)\b/gmu)
      .map(name => name.replace(/([a-z])([A-Z])/gu, '$1-$2').toLowerCase()),
    recordOf: (name) => `translation.${name}`,
  },
  {
    inventory: 'source header model option roles',
    files: ['js/src/translation/emit-rust.js', 'rust/src/translation/emit_rust.rs', 'js/src/typed-program-links.js', 'rust/src/typed_program_links.rs'],
    extract: (text) => [...text.matchAll(/(?:options\.(sourceHeaderModel|rustSourceHeaderEmissionModel)\b|^    pub (source_header_model|rust_source_header_emission_model):)/gmu)].map(([, javascript, native]) => javascript ?? native)
      .map(name => name.replace(/([a-z])([A-Z])/gu, '$1-$2').replaceAll('_', '-').toLowerCase()),
    recordOf: (name) => `translation.${name === 'source-header-model' ? 'rust-source-header-model-option' : 'typed-program-rust-source-header-emission-model-option'}`,
  },
  {
    inventory: 'Rust source signature adapter operations',
    files: ['js/src/translation/rust-source-signatures.js', 'rust/src/translation/source_signatures.rs',
      'rust/src/translation/rust/items/source_headers.rs'],
    extract: (text) => captures(text, /^\s*(?:export function|(?:pub(?:\([^)]*\))? )?fn) (readRustSourceTypeOwner|readRustSourceTypeHead|createRustSourceSignature|checkRustSourceSignature|validate_source_signature|source_type_head|source_type_owner|named_source_type_owner|capture_source_signature)\(/gmu)
      .map(name => name.replace(/([a-z])([A-Z])/gu, '$1-$2').replaceAll('_', '-').toLowerCase()),
    recordOf: (name) => `translation.${({
      'source-type-head': 'read-rust-source-type-head',
      'source-type-owner': 'read-rust-source-type-owner',
      'named-source-type-owner': 'read-rust-source-type-owner',
      'capture-source-signature': 'create-rust-source-signature',
      'validate-source-signature': 'check-rust-source-signature',
    })[name] ?? name}`,
  },
  {
    inventory: 'Rust source signature entities',
    files: ['rust/src/translation/source_signatures.rs'],
    extract: (text) => captures(text, /^pub struct (SourceOwner|SourceTypeSignature|SourceSignature)\b/gmu)
      .map(name => name.replace(/([a-z])([A-Z])/gu, '$1-$2').toLowerCase()),
    recordOf: (name) => `translation.${({
      'source-owner': 'source-type-owner',
      'source-signature': 'source-function-signature',
    })[name] ?? name}`,
  },
  {
    inventory: 'Rust floating point linked template operations',
    files: ['js/src/translation/rust-float-predicates.js', 'rust/src/translation/rust/float_predicates.rs'],
    extract: (text) => captures(text, /^(?:function|fn) (createRustFloatTemplateExpression|policy|instantiate|selected_expression)\(/gmu)
      .map(name => name.replace(/([a-z])([A-Z])/gu, '$1-$2').replaceAll('_', '-').toLowerCase()),
    recordOf: (name) => `translation.${({
      'create-rust-float-template-expression': 'create-rust-floating-point-template-expression',
      'instantiate': 'create-rust-floating-point-template-expression',
      'policy': 'read-rust-floating-point-expression-policy',
      'selected-expression': 'create-rust-selected-floating-point-expression',
    })[name]}`,
  },
  {
    inventory: 'Rust completion admission operations',
    files: ['js/src/translation/rust-completion-rules.js', 'rust/src/translation/rust/completion_rules.rs'],
    extract: (text) => captures(text, /^(?:export function|pub(?: const)? fn) ([a-zA-Z_]+)\(/gmu)
      .map(name => name.replace(/([a-z])([A-Z])/gu, '$1-$2').replaceAll('_', '-').toLowerCase()),
    recordOf: (name) => `translation.${name}`,
  },
  {
    inventory: 'Rust closed completion source operations',
    files: ['js/src/translation/rust-completions.js', 'rust/src/translation/rust/expressions/control/completions.rs'],
    extract: (text) => captures(text, /^\s*(?:export function|function|(?:pub\([^)]*\) )?fn) ([a-zA-Z_]+)\(/gmu)
      .map(name => name.replace(/([a-z])([A-Z])/gu, '$1-$2').replaceAll('_', '-').toLowerCase()),
    recordOf: (name) => `translation.${({
      'read-rust-function-completion': 'read-rust-function-completion',
      'closed-function-completion': 'read-rust-function-completion',
      'range': 'read-rust-completion-source-span',
      'block': 'read-rust-block-completion',
      'completion-block': 'read-rust-block-completion',
      'sequence': 'read-rust-sequence-completion',
      'completion-sequence': 'read-rust-sequence-completion',
      'conditional': 'read-rust-conditional-completion-expression',
      'conditional-completion': 'read-rust-conditional-completion-expression',
      'return-completion': 'read-rust-return-completion',
      'tail-completion': 'read-rust-tail-completion',
      'completion-continuation': 'read-rust-completion-continuation',
    })[name]}`,
  },
  {
    inventory: 'Rust completion panic message operations',
    files: ['js/src/translation/rust-completion-panic.js', 'rust/src/translation/rust/expressions/control/completion_panic.rs'],
    extract: (text) => captures(text, /^(?:export function|pub\(super\) fn) ([a-zA-Z_]+)\(/gmu)
      .map(name => name.replace(/([a-z])([A-Z])/gu, '$1-$2').replaceAll('_', '-').toLowerCase()),
    recordOf: (name) => `translation.${name}`,
  },
  {
    inventory: 'Rust source model parser entry points',
    files: ['js/src/translation/rust.js', 'rust/src/translation/rust/source_profiles.rs'],
    extract: (text) => captures(text, /^(?:export function|pub(?:\(crate\))? fn) (parseRust|parse_rust|parse_rust_with_array_result_model|parse_rust_with_completion_model|parse_rust_with_source_models|parse_rust_with_source_signature_model|parse_rust_with_source_profiles|parse_rust_with_scalar_loop_model|parse_rust_with_primitive_owner_model|parse_rust_with_loop_profiles|parse_rust_with_empty_option_source_model|parse_rust_with_type_namespace_model)\(/gmu)
      .map(name => name.replace(/([a-z])([A-Z])/gu, '$1-$2').replaceAll('_', '-').toLowerCase()),
    recordOf: () => 'translation.parse-rust-source-models',
  },
  {
    inventory: 'JavaScript source model parser entry points',
    files: ['js/src/translation/javascript.js', 'rust/src/translation/javascript.rs'],
    extract: (text) => captures(text, /^(?:export function|pub fn) (parseJavaScript|parse_javascript|parse_javascript_in|parse_javascript_with_scalar_loop_model)\(/gmu)
      .map(name => name.replace(/([a-z])([A-Z])/gu, '$1-$2').replaceAll('_', '-').toLowerCase()),
    recordOf: () => 'translation.parse-javascript-source-models',
  },
  {
    inventory: 'formal function selector boundary',
    files: ['js/src/index.d.ts', 'rust/src/formal_function_links.rs'],
    extract: (text) => captures(text, /^(?:export type|pub enum) (FormalFunctionSelector)/gmu).map(() => 'formal-function-selector'),
    recordOf: (name) => `formal.${name}`,
  },
  {
    inventory: 'formal function definition operations',
    files: ['js/src/formal-function-rules.js', 'js/src/formal-function-links.js', 'rust/src/formal_function_rules.rs', 'rust/src/formal_function_links.rs'],
    extract: (text) => captures(text, /^(?:export function|pub fn) ([a-zA-Z_]+)\(/gmu)
      .filter(name => /FormalFunction|formal_function/u.test(name))
      .map(name => name.replace(/([a-z])([A-Z])/gu, '$1-$2').replaceAll('_', '-').toLowerCase()),
    recordOf: (name) => `formal.${name}`,
  },
  {
    inventory: 'formal function definition entry',
    files: ['js/src/formal-function-links.js', 'rust/src/formal_function_links.rs'],
    extract: (text) => text.includes('selected_function_definition') ? ['selected-function-definition'] : [],
    recordOf: (name) => `formal.${name}`,
  },
  {
    inventory: 'Rust floating point predicate admission operations',
    files: ['js/src/translation/rust-float-predicate-rules.js', 'rust/src/translation/rust/float_predicate_rules.rs'],
    extract: (text) => captures(text, /^(?:export function|pub fn) ([a-zA-Z_]+)\(/gmu)
      .map(name => name.replace(/([a-z])([A-Z])/gu, '$1-$2').replaceAll('_', '-').toLowerCase()),
    recordOf: (name) => `translation.${name.replaceAll('float-predicate', 'floating-point-predicate').replaceAll('float-operation', 'floating-point-operation').replaceAll('float-constant', 'floating-point-constant')}`,
  },
  {
    inventory: 'Rust read only scalar parameter operations',
    files: ['js/src/translation/rust-scalar-parameter-rules.js', 'rust/src/translation/rust/scalar_parameter_rules.rs'],
    extract: (text) => captures(text, /^(?:export function|pub fn) ([a-zA-Z_]+)\(/gmu)
      .map(name => name.replace(/([a-z])([A-Z])/gu, '$1-$2').replaceAll('_', '-').toLowerCase()),
    recordOf: (name) => `translation.${name.replaceAll('unwritten-scalar', 'read-only-scalar')}`,
  },
  {
    inventory: 'resource triple syntax operations',
    files: ['js/src/resource-description-ntriples.js', 'js/src/resource-description-ntriples-rules.js',
      'js/src/resource-description-ntriples-characters.js', 'rust/src/resource_description_ntriples.rs',
      'rust/src/resource_description_ntriples/provenance.rs', 'rust/src/resource_description_ntriples_rules/part_0.rs'],
    extract: (text) => captures(text, /^(?:export function|pub fn) ([a-zA-Z_]+)\(/gmu)
      .filter(name => /NTriples|n_triples|ntriples|ScalarEscape|scalar_escape/u.test(name))
      .map(name => name.replace(/([a-z])([A-Z])/gu, '$1-$2').replaceAll('_', '-').toLowerCase().replaceAll('n-triples', 'ntriples')),
    recordOf: (name) => `knowledge.${({
      'resource-description-ntriples-name': 'accept-resource-description-triple-syntax-name',
      'resource-description-ntriples-escape': 'decode-resource-description-character-escape',
    })[name] ?? name.replaceAll('ntriples', 'triple-syntax')}`,
  },
  {
    inventory: 'resource triple syntax boundary types',
    files: ['js/src/resource-description-ntriples.js', 'rust/src/resource_description_ntriples.rs'],
    extract: (text) => captures(text, /^(?:export class|pub struct) (ResourceDescriptionNTriples[A-Za-z]+)/gmu)
      .map(name => name.replace(/([a-z])([A-Z])/gu, '$1-$2').toLowerCase()),
    recordOf: (name) => `knowledge.${name.replaceAll('ntriples', 'triple-syntax')}`,
  },
  {
    inventory: 'resource triple syntax graph roles',
    files: ['js/src/data/resource-description-ntriples-profile.json', 'rust/src/data/resource-description-ntriples-profile.json'],
    extract: (text) => Object.values(JSON.parse(text).terms).map(name => name.replaceAll(' ', '-')),
    recordOf: (name) => `knowledge.resource-triple-${name === 'blank-node-identity-correspondences' ? 'blank-node-identity-correspondence-collection' : name}`,
  },
  {
    inventory: 'binary link network operations',
    files: ['js/src/link-network-binary.js', 'js/src/link-network-binary-rules.js',
      'rust/src/link_network_binary.rs', ...readdirSync(path.join(repositoryRoot, 'rust/src/link_network_binary_rules'))
        .filter(file => /^part_\d+\.rs$/u.test(file))
        .map(file => `rust/src/link_network_binary_rules/${file}`)
        .filter(file => /^pub fn (?:link_network_binary|accept_link_network|read_link_network|render_link_network|transcode_link_network)/mu.test(readFileSync(path.join(repositoryRoot, file), 'utf8')))],
    extract: (text) => captures(text, /^(?:export function|pub fn) ((?:linkNetworkBinary|acceptLinkNetworkBinary|readLinkNetworkBinary|renderLinkNetworkBinary|transcodeLinkNetworkBinary|encodeLinkNetworkBinary|decodeLinkNetworkBinary)[A-Za-z]*|(?:link_network_binary|accept_link_network_binary|read_link_network_binary|render_link_network_binary|transcode_link_network_binary|encode_link_network_binary|decode_link_network_binary)[a-z_]*)\(/gmu)
      .map(name => name.replace(/([a-z])([A-Z])/gu, '$1-$2').replaceAll('_', '-').toLowerCase()),
    recordOf: (name) => `links.${({
      'link-network-binary-unit': 'read-link-network-binary-unit',
      'link-network-binary-integer-byte': 'read-link-network-binary-integer-byte',
      'transcode-link-network-binary': 'convert-link-network-binary-encoding',
    })[name] ?? name}`,
  },
  {
    inventory: 'binary link network boundary types',
    files: ['js/src/link-network-binary.js', 'rust/src/link_network_binary.rs'],
    extract: (text) => captures(text, /^(?:export class|pub struct) (BinaryLinks[A-Za-z]+)/gmu)
      .map(name => name.replace(/([a-z])([A-Z])/gu, '$1-$2').toLowerCase()),
    recordOf: (name) => `links.${name}`,
  },
  {
    inventory: 'binary network fixture generator',
    files: ['js/scripts/generate-link-network-binary-fixtures.mjs'],
    extract: (text) => captures(text, /^export function (createBinaryNetworkFixture)\(/gmu)
      .map(name => name.replace(/([a-z])([A-Z])/gu, '$1-$2').toLowerCase()),
    recordOf: () => 'repository.create-binary-network-fixture',
  },
  {
    inventory: 'concept view operations',
    files: ['js/src/concept-views.js', 'js/src/concept-view-rules.js',
      'rust/src/concept_ontology/views.rs', 'rust/src/concept_ontology/view_rules.rs'],
    extract: (text) => captures(text, /^(?:export function|pub fn) ((?:create|read|accept)[a-zA-Z_]+)\(/gmu)
      .map(name => name.replace(/([a-z])([A-Z])/gu, '$1-$2').replaceAll('_', '-').toLowerCase()),
    recordOf: (name) => `ontology.${name}`,
  },
  {
    inventory: 'concept view roles',
    files: ['js/src/concept-view-rules.js', 'rust/src/concept_ontology/view_rules.rs'],
    extract: (text) => [...new Set(captures(text, /role ===? ['"]([^'"]+)['"]/gu))].map(name => name.replaceAll(' ', '-')),
    recordOf: (name) => `ontology.${name}`,
  },
  {
    inventory: 'concept view relation declarations',
    files: ['js/src/concept-view-rules.js', 'rust/src/concept_ontology/view_rules.rs'],
    extract: (text) => [...new Set(captures(text, /kind ===? ['"]([^'"]+)['"]/gu))],
    recordOf: (name) => `ontology.declared-concept-${name}`,
  },
  {
    inventory: 'Rust source array admission operations',
    files: ['js/src/translation/rust-array-rules.js', 'rust/src/translation/rust/array_rules.rs'],
    extract: (text) => captures(text, /^(?:export function|pub fn) ([a-zA-Z_]+)\(/gmu)
      .map(name => name.replace(/([a-z])([A-Z])/gu, '$1-$2').replaceAll('_', '-').toLowerCase()),
    recordOf: (name) => `translation.${name}`,
  },
  {
    inventory: 'Rust string continuation operations',
    files: ['js/src/translation/rust-string-rules.js', 'rust/src/translation/frontend_rules/rust_strings.rs'],
    extract: (text) => captures(text, /^(?:export function|pub fn) ((?:accept|read)[a-zA-Z_]+)\(/gmu)
      .map(name => name.replace(/([a-z])([A-Z])/gu, '$1-$2').replaceAll('_', '-').toLowerCase()),
    recordOf: (name) => `translation.${name}`,
  },
  {
    inventory: 'resource description graph roles',
    files: ['js/src/data/resource-description-schema.json', 'rust/src/data/resource-description-schema.json'],
    extract: (text) => Object.values(JSON.parse(text).terms).map(name => name.toLowerCase().replaceAll(' ', '-')),
    recordOf: (name) => `knowledge.${name === 'literal-value' ? 'resource-literal-value' : name}`,
  },
  {
    inventory: 'discovered concept graph roles',
    files: ['js/src/discovered-concept-rules.js', 'rust/src/discovered_concept_rules.rs'],
    extract: (text) => [...new Set(captures(text, /term ===? ['"]([^'"]+)['"]/gu)
      .map(name => name.replaceAll(' ', '-')))],
    recordOf: (name) => `grammar.${name}`,
  },
  {
    inventory: 'resource description operations',
    files: ['js/src/resource-description-rules.js', 'js/src/resource-description-graphs.js',
      'rust/src/resource_description_graphs.rs', 'rust/src/resource_description_rules/part_0.rs',
      'rust/src/resource_description_rules/part_1.rs'],
    extract: (text) => captures(text, /^(?:export function|pub fn) ((?:resourceDescription|acceptResourceDescription|sameResourceDescription|importResourceDescription|decodeResourceDescription|mergeResourceDescription|verifyResourceDescription)[A-Za-z]+|(?:resource_description|accept_resource_description|same_resource_description|import_resource_description|decode_resource_description|merge_resource_description|verify_resource_description)_[a-z_]+)\(/gmu)
      .map((name) => name.replace(/([a-z])([A-Z])/gu, '$1-$2').replaceAll('_', '-').toLowerCase()),
    recordOf: (name) => `knowledge.${({
      'resource-description-unit': 'read-resource-description-unit',
      'resource-description-letter': 'recognize-resource-description-letter',
      'resource-description-digit': 'recognize-resource-description-digit',
      'resource-description-hexadecimal': 'recognize-resource-description-hexadecimal',
      'same-resource-description-term': 'compare-resource-description-term',
      'decode-resource-description-graph-with-limit': 'decode-resource-description-graph',
    })[name] ?? name}`,
  },
  {
    inventory: 'discovery integration operations',
    files: ['js/src/discovered-concept-links.js', 'rust/src/discovered_concept_links.rs',
      'js/src/discovered-concept-rules.js', 'rust/src/discovered_concept_rules.rs'],
    extract: (text) => captures(text, /^(?:export function|pub fn) ((?:create|read|accept)(?:DiscoveredConcept[A-Za-z]+|_discovered_concept_[a-z_]+))\(/gmu)
      .map((name) => name.replace(/([a-z])([A-Z])/gu, '$1-$2').replaceAll('_', '-').toLowerCase()),
    recordOf: (name) => `grammar.${name}`,
  },
  {
    inventory: 'Rust function lifetime fixture',
    files: ['js/scripts/generate-rust-function-lifetime-fixtures.mjs'],
    extract: (text) => captures(text, /^export function (rustFunctionLifetimeFixture)\(/gmu)
      .map((name) => name.replace(/([a-z])([A-Z])/gu, '$1-$2').toLowerCase()),
    recordOf: (name) => `translation.build-${name}`,
  },
  {
    inventory: 'formal relation identity decisions',
    files: ['js/src/formal-relation-rules.js', 'rust/src/formal_concept_definition/relation_rules.rs'],
    extract: (text) => captures(text, /^(?:export function|pub fn) ([a-zA-Z][a-zA-Z_]+)\(/gmu)
      .map((name) => name.replace(/([a-z])([A-Z])/gu, '$1-$2').replaceAll('_', '-').toLowerCase()),
    recordOf: (name) => `formal.${name.replace('formal-', '')}`,
  },
  {
    inventory: 'TypeScript typing projection entry points',
    files: ['js/src/index.js', 'rust/src/self_translation/typescript_portable_typing.rs'],
    extract: (text) => captures(text, /^(?:export function|pub fn) (projectTypeScriptTyping|project_type_script_typing)\(/gmu)
      .map((name) => name.replace(/([a-z])([A-Z])/gu, '$1-$2').replaceAll('_', '-').toLowerCase()),
    recordOf: () => 'translation.project-type-script-typing',
  },
  {
    inventory: 'global concept link records',
    files: ['js/src/global-concept-space.js'],
    extract: (text) => captures(text, /(?:head (?:===|!==)|tag !==) '([a-z-]+)'/gu),
    recordOf: (name) => ({
      'global-concept-space': 'grammar.global-concept-space',
      'global-concept': 'grammar.global-concept',
      'concept-origin': 'grammar.global-concept-origin',
      'grammar-origin': 'grammar.global-grammar-origin',
      'grammar-context': 'grammar.global-grammar-context',
      'concept-definition': 'grammar.global-concept-definition',
      'concept-proof': 'grammar.global-concept-proof',
      'unproved-equivalence': 'grammar.unproved-concept-equivalence',
      'ambiguous-origin': 'grammar.ambiguous-concept-origin',
      'concept-redirect': 'grammar.global-concept-identity-mapping',
      'source-alias': 'grammar.qualified-source-alias',
      correspondence: 'grammar.concept-correspondence',
      'formal-origin-definition': 'grammar.formal-origin-definition',
      links: 'grammar.formal-definition-network',
    })[name] ?? `grammar.${name}`,
  },
  {
    inventory: 'global concept proof method',
    files: ['js/src/global-concept-space.js'],
    extract: (text) => captures(text, /export const GLOBAL_CONCEPT_PROOF_METHOD = '([^']+)'/gu),
    recordOf: (name) => `grammar.${name}`,
  },
  {
    inventory: 'global concept interface operations',
    files: ['js/src/global-concept-space.js', 'rust/src/global_concept_space.rs', 'rust/src/global_concept_space/links.rs'],
    extract: (text) => captures(text, /^(?:export function|pub fn) ((?:createGlobalConceptSpace|globalConceptSpaceToLinks|readGlobalConceptSpaceLinks|translateGlobalConstruct|create_global_concept_space|global_concept_space_to_links|read_global_concept_space_links|translate_global_construct))\(/gmu)
      .map((name) => name.replace(/([a-z])([A-Z])/gu, '$1-$2').replaceAll('_', '-').toLowerCase()),
    recordOf: (name) => `operation.${name === 'global-concept-space-to-links' ? 'serialize-global-concept-space' : name}`,
  },
  {
    inventory: 'formal definition operations',
    files: ['js/src/formal-concept-definition.js', 'rust/src/formal_concept_definition.rs'],
    extract: (text) => captures(text, /^(?:export function|pub fn) (encodeFormalConceptDefinition|readFormalConceptDefinition|proveFormalConceptEquivalence|verifyFormalConceptEquivalence|encode_formal_concept_definition|read_formal_concept_definition|prove_formal_concept_equivalence|verify_formal_concept_equivalence)\(/gmu)
      .map((name) => name.replace(/([a-z])([A-Z])/gu, '$1-$2').replaceAll('_', '-').toLowerCase()),
    recordOf: (name) => `operation.${name}`,
  },
  {
    inventory: 'formal definition records',
    files: ['js/src/formal-concept-definition.js'],
    extract: (text) => [
      ...captures(text, /(?:const (?:ROOT|CERTIFICATE|PAIRS|PAIR) = |metadata\(|\.term !== )'([^']+)'/gu),
      ...(text.includes('`formal declaration ${typeof value}`') ? ['formal declaration string', 'formal declaration number', 'formal declaration boolean'] : []),
    ].map((name) => name.replaceAll(' ', '-')),
    recordOf: (name) => `grammar.${name === 'formal-declaration-boolean' ? 'formal-declaration-truth-value' : name}`,
  },
  {
    inventory: 'formal definition host inputs',
    files: ['rust/src/formal_concept_definition.rs'],
    extract: (text) => captures(text, /^pub (?:struct|enum) (FormalConceptArchive|FormalConceptDefinitionInput|FormalConceptDefinitionOptions)\b/gmu)
      .map((name) => name.replace(/([a-z])([A-Z])/gu, '$1-$2').toLowerCase()),
    recordOf: (name) => `grammar.${name}`,
  },
  {
    inventory: 'formal primitive models',
    files: ['js/src/data/formal-concept-models.json', 'rust/src/data/formal-concept-models.json'],
    extract: (text) => Object.keys(JSON.parse(text)).map((name) => name.replaceAll(' ', '-')),
    recordOf: (name) => `translation.${name}`,
  },
  {
    inventory: 'typed linked program operations',
    files: ['js/src/typed-program-links.js', 'js/src/typed-program-links-validation.js', 'js/src/typed-program-link-rules.js', 'js/src/typed-program-checked-projection.js', 'rust/src/translation/typed_program_link_rules.rs'],
    extract: (text) => captures(text, /^(?:export function|pub(?: const)? fn) ([a-zA-Z][a-zA-Z_]+)\(/gmu)
      .map((name) => name.replace(/([a-z])([A-Z])/gu, '$1-$2').replaceAll('_', '-').toLowerCase()),
    recordOf: (name) => `translation.${name === 'is-unsafe-program-field' ? 'detect-unsafe-program-field' : name}`,
  },
  {
    inventory: 'typed Rust source model factory operations',
    files: ['rust/src/typed_program_links.rs'],
    extract: (text) => captures(text, /^pub fn (create_typed_program_links_with_rust_array_result_model|create_typed_program_links_with_rust_completion_model|create_typed_program_links_with_rust_source_models|create_typed_program_links_with_rust_source_signature_model|create_typed_program_links_with_rust_source_profiles|create_typed_program_links_with_rust_primitive_owner_model|create_typed_program_links_with_rust_empty_option_source_model|create_typed_program_links_with_rust_type_namespace_model)\(/gmu)
      .map(name => name.replaceAll('_', '-')),
    recordOf: () => 'translation.create-typed-program-links',
  },
  {
    inventory: 'typed scalar loop source factory operations',
    files: ['rust/src/typed_program_links.rs'],
    extract: (text) => captures(text, /^pub fn (create_typed_program_links_with_rust_scalar_loop_model|create_typed_program_links_with_javascript_scalar_loop_model|create_typed_program_links_with_scalar_loop_profiles)\(/gmu)
      .map(name => name.replaceAll('_', '-')),
    recordOf: () => 'translation.create-typed-program-links',
  },
  {
    inventory: 'typed native program emission option operations',
    files: ['rust/src/typed_program_links.rs'],
    extract: (text) => captures(text, /^pub fn (emit_typed_program_links|emit_typed_program_links_with_options)\(/gmu)
      .map(name => name.replaceAll('_', '-')),
    recordOf: () => 'translation.emit-typed-program-links',
  },
  {
    inventory: 'typed program binding identities',
    files: ['rust/src/typed_program_links.rs'],
    extract: (text) => captures(text, /^pub(?:\((?:crate|super)\))? fn (read_typed_program_binding_identities)\(/gmu)
      .map((name) => name.replaceAll('_', '-')),
    recordOf: (name) => `translation.${name}`,
  },
  {
    inventory: 'shared rust array runtime',
    files: ['js/src/translation/emit-rust.js'],
    extract: (text) => captures(text, /^export function (readRustArrayPrelude)\(/gmu)
      .map((name) => name.replace(/([a-z])([A-Z])/gu, '$1-$2').toLowerCase()),
    recordOf: (name) => `translation.${name}`,
  },
  {
    inventory: 'typed program expression projections',
    files: ['js/src/typed-program-expression-projection.js', 'rust/src/typed_program_links/projection.rs'],
    extract: (text) => captures(text, /^(?:export function|pub\(super\) fn) (projectLinkedProgramExpression|project)\(/gmu)
      .map((name) => name.replace(/([a-z])([A-Z])/gu, '$1-$2').toLowerCase()),
    recordOf: () => 'translation.project-linked-program-expression',
  },
  {
    inventory: 'typed linked program records',
    files: ['js/src/data/typed-program-links-schema.json', 'rust/src/data/typed-program-links-schema.json'],
    extract: (text) => {
      const schema = JSON.parse(text);
      return [schema.root, ...Object.values(schema.containers), ...Object.values(schema.scalars), ...Object.values(schema.fields), ...Object.values(schema.references)].map((name) => name.toLowerCase().replaceAll('datatype', 'data type').replace(/\bconst\b/gu,'constant').replaceAll(' ', '-'));
    },
    recordOf: (name) => ({
      'string-value': 'translation.string-scalar-value',
      'number-value': 'translation.number-scalar-value',
      'boolean-value': 'translation.boolean-scalar-value',
      'integer-value': 'translation.big-integer-scalar-value',
      'null-value': 'translation.null-scalar-value',
      'undefined-value': 'translation.undefined-scalar-value',
      argument: 'translation.program-argument-field',
      arguments: 'translation.program-argument-list-field',
      constructor: 'translation.program-constructor-field',
      constructors: 'translation.program-constructor-list-field',
      parameters: 'translation.program-parameter-list-field',
      operation: 'translation.program-operation-field',
      expression: 'translation.program-expression-field',
      condition: 'translation.program-condition-field',
      'function-name': 'translation.program-function-name-field',
      'recursive-definition': 'translation.program-recursion-model',
      'unicode-code-unit-strings': 'translation.program-unicode-code-unit-string-model',
      'source-function-signature': 'translation.program-source-function-signature-field',
      'source-signature-parameters': 'translation.program-source-signature-parameter-list-field',
      'source-signature-result': 'translation.program-source-signature-result-field',
      'source-type-owner': 'translation.program-source-type-owner-field',
      'source-type-owner-arguments': 'translation.program-source-type-owner-argument-list-field',
      'source-type-wrappers': 'translation.program-source-type-wrapper-list-field',
      'source-type-lifetimes': 'translation.program-source-type-lifetime-list-field',
      'source-lifetime-bindings': 'translation.program-source-lifetime-binding-list-field',
      'reference-semantics': 'translation.program-reference-semantics-field',
      'rust-source-signature-model': 'translation.program-rust-source-signature-model-field',
      'source-loop-proof': 'translation.program-source-loop-proof-field',
      'source-empty-option-proof': 'translation.program-source-empty-option-proof-field',
      'empty-option-constructor-proof': 'translation.program-source-empty-option-constructor-proof-field',
      'canonical-owner-path': 'translation.program-source-canonical-owner-path-field',
      'external-option-roots': 'translation.program-source-external-option-root-list-field',
      'declared-function-names': 'translation.program-source-declared-function-name-list-field',
      'source-derivation-data-type-owner': 'translation.source-derivation-data-type-owner',
      'source-constant-function': 'translation.function-source-constant-qualification-proof-field',
      'keyword': 'translation.source-keyword-field',
      'keyword-span': 'translation.source-keyword-span-field',
      'function-keyword-span': 'translation.source-function-keyword-span-field',
      'header-span': 'translation.function-header-source-span-field',
      'body-span': 'translation.function-body-source-span-field',
      'source-name': 'translation.source-name-field',
      'rust-constant-function-model': 'translation.program-rust-constant-function-model-field',
      'source-derivation-data-type-owner': 'translation.source-derivation-data-type-owner',
      'source-derivation-data-type-owner': 'translation.source-derivation-data-type-owner',
      'source-derivation-data-type-owner': 'translation.source-derivation-data-type-owner',
    })[name] ?? `translation.${name}`,
  },
  {
    inventory: 'reference operation queries',
    files: ['js/src/translation/reference-model.js'],
    extract: (text) => captures(text, /^export function (referenceMethodOperation|referenceOperation|referenceOperations|referenceOperationNetwork)\(/gmu)
      .map((name) => name.replace(/([a-z])([A-Z])/gu, '$1-$2').toLowerCase()),
    recordOf: (name) => `translation.${name === 'reference-operations' ? 'read-reference-operation-list' : `read-${name}`}`,
  },
  {
    inventory: 'shared mutable binding decisions',
    files: ['js/src/translation/javascript-cells.js'],
    extract: (text) => captures(text, /^export function (mutableClosureCells|markSharedReferenceTypes)\(/gmu)
      .map((name) => name.replace(/([a-z])([A-Z])/gu, '$1-$2').toLowerCase()),
    recordOf: (name) => `translation.${name === 'mutable-closure-cells' ? 'find-mutable-closure-cells' : name}`,
  },
  {
    inventory: 'global concept correspondence verifier',
    files: ['js/src/global-concept-rules.js', 'rust/src/global_concept_rules.rs'],
    extract: (text) => captures(text, /^(?:export function|pub fn) (verifyGlobalConceptCorrespondence|verify_global_concept_correspondence)\(/gmu)
      .map((name) => name.replace(/([a-z])([A-Z])/gu, '$1-$2').replaceAll('_', '-').toLowerCase()),
    recordOf: (name) => `operation.${name}`,
  },
  {
    inventory: 'grammar concepts',
    files: ['rust/src/grammar/concepts.rs'],
    extract: (text) => captures(block(text, 'pub const GRAMMAR_CONCEPTS', '\n];'), /id: "([^"]+)"/gu),
    describe: (text) => conceptDefinitions(block(text, 'pub const GRAMMAR_CONCEPTS', '\n];'), 'syntax'),
  },
  {
    inventory: 'grammar expression concept identities',
    files: ['rust/src/grammar/concepts.rs'],
    extract: (text) => captures(block(text, 'fn grammar_expr_concept_id', '\n}'), /=> "([^"]+)"/gu),
  },
  {
    inventory: 'grammar constructs',
    files: ['rust/src/grammar/fidelity.rs'],
    extract: (text) => captures(block(text, 'pub const GRAMMAR_CONSTRUCTS', '\n];'), /"([^"]+)"/gu),
    recordOf: (name) => `grammar.${name}`,
  },
  {
    inventory: 'grammar link terms',
    files: ['rust/src/grammar/links.rs', 'rust/src/grammar/runtime/mod.rs', 'rust/src/grammar/surface/mod.rs'],
    extract: (text) => captures(withoutFormerNames(text), /"(grammar::[a-z-]+(?:::[a-z-]+)*)(?:::[^"]*)?"/gu),
    recordOf: grammarLinkRecord,
  },
  {
    inventory: 'grammar translation concepts',
    files: ['rust/src/grammar/translate.rs'],
    extract: (text) => captures(text, /concept: "([^"]+)"/gu),
    describe: (text) =>
      [...text.matchAll(/concept: "([^"]+)",\s*english: "([^"]+)",\s*russian: "([^"]+)"/gu)].map(([, name, english, russian]) => ({
        name,
        sourceAliases: [
          { source: 'English grammar surface', name: english },
          { source: 'Russian grammar surface', name: russian },
        ],
      })),
    recordOf: grammarConceptRecord,
  },
  {
    inventory: 'grammar surface words',
    files: ['rust/src/grammar/translate.rs'],
    extract: (text) => captures(text, /english: "([^"]+)"/gu),
    words: true,
  },
  {
    inventory: 'grammar inference naming concepts',
    files: ['rust/src/grammar/inference/advisor.rs'],
    extract: (text) => captures(block(text, 'const INFERENCE_NAMING_CONCEPTS', '\n];'), /id: "([^"]+)"/gu),
  },
  {
    inventory: 'inferred rule names',
    files: ['rust/src/grammar/inference/advisor.rs'],
    extract: (text) => {
      const naming = block(text, 'const INFERENCE_NAMING_CONCEPTS', '\n];') + block(text, 'fn structural_name', '\n}\n');
      return captures(naming, /(?:name: )?"([a-z_{}]+)"/gu)
        .map((name) => name.replace(/\{[a-z]*\}/gu, ''))
        .filter((name) => nameWords(name).length > 0);
    },
    words: true,
  },
  {
    inventory: 'required concept distinctions',
    files: ['rust/src/data/required-concept-distinctions.rs'],
    extract: (text) =>
      [...text.matchAll(/concepts: \[([^\]]*)\]/gu)].flatMap(([, pair]) => captures(pair, /"([^"]+)"/gu)),
  },
  {
    inventory: 'required concept distinctions',
    files: ['js/src/data/required-concept-distinctions.json'],
    extract: (text) => JSON.parse(text).flatMap(({ concepts }) => concepts),
  },
  {
    inventory: 'structural programming concepts',
    files: ['rust/src/concept_ontology.rs'],
    extract: (text) => captures(block(text, 'const STRUCTURAL_CONCEPTS', '\n];'), /id: "([^"]+)"/gu),
    describe: (text) => conceptDefinitions(block(text, 'const STRUCTURAL_CONCEPTS', '\n];'), 'syntax'),
  },
  {
    inventory: 'worked example concepts',
    files: ['rust/src/concept_ontology.rs'],
    extract: (text) => captures(text, /insert_typed_point\(\s*"([^"]+)",\s*LinkType::Concept/gu),
    describe: (text) =>
      [...text.matchAll(new RegExp(String.raw`insert_typed_point\(\s*"(statehood)",\s*LinkType::Concept,\s*Some\(${RUST_STRING}\)`, 'gu'))].map(
        ([, name, definition]) => ({
          name,
          definition: rustString(definition),
          sourceAliases: [...block(text, 'const STATEHOOD_PROPOSITION_SYNTAX', '\n];').matchAll(/\(\s*"([^"]+)",\s*"([^"]+)"\s*\)/gu)].map(
            ([, source, alias]) => ({ source, name: alias }),
          ),
        }),
      ),
  },
  {
    inventory: 'worked example concepts',
    files: ['js/src/concept-ontology.js'],
    extract: (text) => captures(text, /insertTypedPoint\(\s*LinkType\.Concept,\s*'([^']+)'/gu),
  },
  {
    inventory: 'external identifier vocabularies',
    files: ['rust/src/concept_ontology.rs'],
    extract: (text) => captures(withoutFormerNames(text), /const EXTERNAL_[A-Z_]+_PREFIX: &str = "([a-z-]+):";/gu),
    recordOf: (name) => `ontology.${name}`,
  },
  {
    inventory: 'external identifier vocabularies',
    files: ['js/src/network.js'],
    extract: (text) => captures(text, /const EXTERNAL_[A-Z_]+_PREFIX = '([a-z-]+):';/gu),
    recordOf: (name) => `ontology.${name}`,
  },
  {
    inventory: 'document formatting concepts',
    files: ['rust/src/document_formatting/mod.rs'],
    extract: (text) => captures(block(text, 'const DOCUMENT_FORMATTING_CONCEPTS', '\n];'), /id: "([^"]+)"/gu),
    describe: (text) => conceptDefinitions(block(text, 'const DOCUMENT_FORMATTING_CONCEPTS', '\n];'), 'templates'),
  },
  {
    inventory: 'cross-format concepts',
    files: ['rust/src/document_formatting/profile.rs'],
    extract: (text) => captures(block(text, 'pub const CROSS_FORMAT_CONCEPT_IDS', '\n];'), /"([^"]+)"/gu),
  },
  {
    inventory: 'semantic constructs',
    files: ['rust/src/program_representation.rs'],
    extract: (text) => captures(block(text, 'pub const SEMANTIC_CONSTRUCTS', '\n];'), /"([^"]+)"/gu),
    recordOf: (name) => `program.${name}`,
  },
  {
    inventory: 'semantic constructs',
    files: ['js/src/program-representation.js'],
    extract: (text) => captures(block(text, 'export const SEMANTIC_CONSTRUCTS', '\n]);'), /'([^']+)'/gu),
    recordOf: (name) => `program.${name}`,
  },
  {
    inventory: 'programming interface operations',
    files: ['rust/src/api_styles.rs'],
    extract: (text) => captures(block(text, 'pub const fn name(self)', '\n    }\n'), /=> "([^"]+)"/gu),
    recordOf: (name) => `operation.${name}`,
  },
  {
    inventory: 'programming interface operations',
    files: ['js/src/api-style-fixtures.js'],
    extract: (text) => captures(block(text, 'export const ApiOperation =', '\n});'), /: '([^']+)'/gu),
    recordOf: (name) => `operation.${name}`,
  },
  {
    inventory: 'native grammar rule names',
    files: NATIVE_GRAMMAR_FILES,
    extract: (text) => captures(text, /\((?:rule|alias) ([a-z0-9_]+) /gu),
    words: true,
  },
  {
    inventory: 'native grammar concept references',
    occurrences: true,
    files: NATIVE_GRAMMAR_FILES,
    extract: (text) => nativeRuleConcepts(text).map(({ concept }) => concept),
    describe: (text, file) =>
      nativeRuleConcepts(text).map(({ rule, concept }) => ({ name: concept, sourceAliases: [{ source: nativeGrammarSource(file), name: rule }] })),
  },
  {
    inventory: 'reviewed legacy native concept identities',
    files: ['parity/naming/grammar-name-expansions.json'],
    extract: (text) => [...new Set([...Object.keys(JSON.parse(text).concepts.legacyAliases ?? {}),
      ...Object.keys(JSON.parse(text).concepts.formerNames ?? {})])],
    describe: (text) => Object.entries(JSON.parse(text).concepts.legacyAliases ?? {})
      .map(([name, sourceAliases]) => ({ name, sourceAliases })),
  },
  {
    inventory: 'translation stages',
    files: ['parity/fixtures/translation-stages.json'],
    extract: (text) => [
      ...new Set(
        JSON.parse(text).programs.flatMap((program) =>
          Object.entries(program)
            .filter(([, value]) => value !== null && typeof value === 'object' && !Array.isArray(value))
            .map(([key]) => key),
        ),
      ),
    ],
    recordOf: (name) => `translation.${name}`,
  },
  {
    inventory: 'associative foundation audit operations',
    files: ['js/scripts/associative-stack-sources.mjs', 'js/scripts/meta-theory-foundation-sources.mjs'],
    extract: (text) => captures(text, /^export function ([a-zA-Z][a-zA-Z_]+)\(/gmu)
      .map((name) => name.replace(/([a-z])([A-Z])/gu, '$1-$2').toLowerCase()),
    recordOf: (name) => `foundation.${name}`,
  },
  {
    inventory: 'native token prefix source operations',
    files: ['js/scripts/native-token-prefix-context.mjs'],
    extract: (text) => captures(text, /^export function ([a-zA-Z][a-zA-Z_]+)\(/gmu)
      .map((name) => name.replace(/([a-z])([A-Z])/gu, '$1-$2').toLowerCase()),
    recordOf: (name) => `grammar.${name}`,
  },
  {
    inventory: 'frontend decision operations',
    files: ['js/src/translation/frontend-rules.js', 'rust/src/translation/frontend_rules.rs', 'rust/src/translation/frontend_rules/strings.rs'],
    extract: (text) => captures(text, /^(?:export function|pub(?: const)? fn) ([a-zA-Z][a-zA-Z_]+)\(/gmu)
      .filter((name) => !name.startsWith('ml_'))
      .map((name) => name.replace(/([a-z])([A-Z])/gu, '$1-$2').replaceAll('_', '-').toLowerCase()),
    recordOf: (name) => `translation.${name}`,
  },
  {
    inventory: 'frontend source operand comparison premises',
    files: ['js/src/translation/frontend-rules.js', 'rust/src/translation/frontend_rules/strings.rs'],
    extract: (text) => captures(text, /^(?:export function acceptPrecedenceOperandHead|pub const fn accept_precedence_operand_head)\(([^)]*)\)/gmu)
      .flatMap(parameters => [...parameters.matchAll(/\b(sourceRuleMatches|sourceReductionMatches|source_rule_matches|source_reduction_matches)\b/gu)].map(([, name]) => name))
      .map(name => name.replace(/([a-z])([A-Z])/gu, '$1-$2').replaceAll('_', '-').toLowerCase()),
    recordOf: (name) => `translation.precedence-operand-${name === 'source-rule-matches' ? 'source-rule-match' : 'source-reduction-match'}`,
  },
  {
    inventory: 'TypeScript erasure decisions',
    files: ['js/src/typescript-erasure-rules.js', 'rust/src/self_translation/typescript_erasure_rules.rs'],
    extract: (text) => captures(text, /^(?:export function|pub fn) ([a-zA-Z][a-zA-Z_]+)\(/gmu)
      .map((name) => name.replace(/([a-z])([A-Z])/gu, '$1-$2').replaceAll('_', '-').toLowerCase()),
    recordOf: (name) => `translation.${name}`,
  },
  {
    inventory: 'Rust source indirection decisions',
    files: ['js/src/translation/rust-indirection-rules.js', 'rust/src/translation/rust/indirection_rules.rs'],
    extract: (text) => captures(text, /^(?:export function|pub(?: const)? fn) ([a-zA-Z][a-zA-Z_]+)\(/gmu)
      .map((name) => name.replace(/([a-z])([A-Z])/gu, '$1-$2').replaceAll('_', '-').toLowerCase()),
    recordOf: (name) => `translation.${name}`,
  },
  {
    inventory: 'structured data links',
    files: ['js/src/structured-data-links.js', 'js/src/structured-data-encode.js', 'js/src/structured-data-graph.js', 'js/src/structured-data-model.js', 'js/src/structured-data-link-rules.js', 'js/src/structured-data-records.js', 'js/src/structured-data-records-write.js', 'scripts/lib/structured-data-bootstrap.mjs', 'rust/src/structured_data_links.rs', 'rust/src/structured_data_link_rules.rs', 'rust/src/structured_data_records.rs'],
    extract: (text) => captures(text, /^(?:export function|export class|pub fn|pub struct|pub enum) ([a-zA-Z][a-zA-Z_]+)\b/gmu)
      .map((name) => name.replace(/([a-z])([A-Z])/gu, '$1-$2').replaceAll('_', '-').toLowerCase()),
    recordOf: (name) => `structured-data.${({
      'structured-data-graph-cost': 'measure-structured-data-graph-cost',
      'limits-of': 'resolve-structured-data-limits',
      'well-formed-string': 'accept-well-formed-string',
      'read-bootstrap-structured-data-lino': 'read-bootstrap-structured-data-links-notation',
      'structured-data-record-to-lino': 'serialize-structured-data-record-as-links-notation',
      'structured-data-record-to-lino-with-namespace': 'serialize-structured-data-record-as-links-notation-with-namespace',
      'structured-data-record-to-lino-ordered': 'serialize-ordered-structured-data-record-as-links-notation',
      'structured-data-record-to-lino-ordered-with-namespace': 'serialize-ordered-structured-data-record-as-links-notation-with-namespace',
      'read-structured-data-records-ordered': 'read-ordered-structured-data-records',
      'read-structured-data-records-ordered-with-limits': 'read-ordered-structured-data-records-with-limits',
      'structured-data-to-lino': 'serialize-structured-data-as-links-notation',
      'structured-data-to-lino-ordered': 'serialize-ordered-structured-data-as-links-notation',
      'read-structured-data-lino': 'read-structured-data-links-notation',
      'read-structured-data-lino-ordered': 'read-structured-data-links-notation-ordered',
      'read-structured-data-lino-ordered-with-limits': 'read-structured-data-links-notation-ordered-with-limits',
      'read-structured-data-lino-with-limits': 'read-structured-data-links-notation-with-limits',
    })[name] ?? name}`,
  },
  {
    inventory: 'project data storage operations',
    files: ['js/src/project-data-files.js', 'js/scripts/convert-structured-data.mjs', 'js/scripts/project-data-storage.mjs', 'js/scripts/project-data-tooling-evidence.mjs', 'js/scripts/validate-structured-data-migration.mjs', 'js/scripts/generate-structured-data-migration-revisions.mjs', 'rust/src/project_data.rs'],
    extract: (text) => captures(text, /^(?:export (?:async )?function|pub fn|pub enum) ([a-zA-Z][a-zA-Z_]+)\b/gmu)
      .map((name) => name.replace(/([a-z])([A-Z])/gu, '$1-$2').replaceAll('_', '-').toLowerCase()),
    recordOf: (name) => `repository.${name}`,
  },
  {
    inventory: 'lazy shipped concept operations',
    files: ['js/src/lazy-shipped-data.js', 'js/src/concept-records.js'],
    extract: (text) => captures(text, /^(?:export )?function (createLazyFrozenArray|loadConceptRecords|ensureConceptIndexes)\(/gmu)
      .map((name) => name.replace(/([a-z])([A-Z])/gu, '$1-$2').toLowerCase()),
    recordOf: (name) => `operation.${name}`,
  },
  {
    inventory: 'grammar report storage operations',
    files: ['js/scripts/grammar-report-files.mjs', 'js/scripts/grammar-report-fragments.mjs'],
    extract: (text) => captures(text, /^export function (writeGrammarReport|writeFactoredGrammarReport|readFactoredGrammarReport)\(/gmu)
      .map((name) => name.replace(/([a-z])([A-Z])/gu, '$1-$2').toLowerCase()),
    recordOf: (name) => `repository.${name}`,
  },
  {
    inventory: 'native grammar fixture execution operations',
    files: ['js/scripts/native-grammar-fixture.mjs', 'js/scripts/generate-native-grammar-fixtures.mjs'],
    extract: (text) => captures(text, /^export function (buildNativeGrammarFixtureFromParser|refreshNativeDefaultProvenance)\(/gmu)
      .map((name) => name.replace(/([a-z])([A-Z])/gu, '$1-$2').toLowerCase()),
    recordOf: (name) => `grammar.${name}`,
  },
  {
    inventory: 'temporary external grammar asset operations',
    files: ['js/scripts/external-grammar-cache.mjs'],
    extract: (text) => captures(text, /^export (?:async )?function ([a-zA-Z][a-zA-Z]+)\(/gmu)
      .map((name) => name.replace(/([a-z])([A-Z])/gu, '$1-$2').toLowerCase()),
    recordOf: (name) => `grammar.${({ 'external-grammar-path': 'resolve-external-grammar-path',
      'external-grammar-directory': 'resolve-external-grammar-directory' })[name] ?? name}`,
  },
  {
    inventory: 'project and grammar asset readers',
    files: ['js/src/project-data-loader.js', 'js/src/grammar-asset-path.js', 'js/scripts/grammar-files.mjs'],
    extract: (text) => captures(text, /^export function (read[a-zA-Z]+)\(/gmu)
      .map((name) => name.replace(/([a-z])([A-Z])/gu, '$1-$2').toLowerCase()),
    recordOf: (name) => `repository.${name}`,
  },
  {
    inventory: 'linked reference operation programs',
    files: ['js/src/data/reference-operations.lino', 'rust/src/data/reference-operations.lino'],
    extract: (text) => captures(text, /^\(([a-z-]+):/gmu),
    describe: (text, file) => [...text.matchAll(/^\(([a-z-]+):.*?\(kind ([a-z-]+)\)(?: \(method ([a-z-]+)\))?/gmu)].map(([, name, kind, method]) => ({
      name,
      sourceAliases: [
        { source: file.startsWith('js/') ? 'JavaScript reference operations' : 'Rust reference operations', name },
        ...(method ? [{ source: `JavaScript ${kind} methods`, name: method }] : []),
      ],
    })),
    recordOf: (name) => `translation.${name}`,
  },
  {
    inventory: 'linked reference program instructions',
    files: ['js/src/data/reference-operations.lino', 'rust/src/data/reference-operations.lino'],
    extract: (text) => captures(text, /\((allocate-reference|read-reference|write-reference|compare-identities|return-value|invoke-array-method|invoke-map-method|invoke-set-method|read-selected-value|write-selected-value)\b/gu),
    recordOf: (name) => `translation.${({
      'allocate-reference': 'allocate-reference-cell',
      'read-reference': 'read-reference-cell',
      'write-reference': 'write-reference-cell',
      'compare-identities': 'compare-reference-identity',
      'return-value': 'return-reference-operation-value',
    })[name] ?? name}`,
  },
  {
    inventory: 'linked reference value records',
    files: ['js/src/translation/reference-model.js'],
    extract: (text) => [
      ...captures(text, /(?:this\.network\.point\(|term === )'([a-z-]+)'/gu),
      ...captures(text, /return '(array|map|set|record)'/gu).map((kind) => `${kind}-value`),
      ...[...text.matchAll(/\? '(null-prototype)' : '(object-prototype)'/gu)].flatMap(([, first, second]) => [first, second]),
      ...(text.includes('_newCell(kind)') ? ['reference-cell'] : []),
      ...captures(text, /^export class (ReferenceHeap|ReferenceValue)\b/gmu).map((name) => name.replace(/([a-z])([A-Z])/gu, '$1-$2').toLowerCase()),
    ].filter((name) => name !== 'reference-heap' || text.includes("'reference-heap'")),
    recordOf: (name) => `translation.${name === 'set-value' ? 'set-reference-value' : name === 'program' ? 'reference-operation-program' : ['undefined-value', 'null-value', 'boolean-value', 'number-value', 'big-integer-value', 'string-value'].includes(name) ? name.replace('-value', '-scalar-value') : name}`,
  },
  {
    inventory: 'reference representation selection',
    files: ['js/src/translation/reference-analysis.js'],
    extract: (text) => captures(text, /^export function (requiresReferenceSemantics|hasStructuralIdentity)\(/gmu)
      .map((name) => name.replace(/([a-z])([A-Z])/gu, '$1-$2').toLowerCase()),
    recordOf: (name) => `translation.${name === 'requires-reference-semantics' ? 'require-reference-semantics' : 'find-structural-identity'}`,
  },
  {
    inventory: 'primitive value decisions',
    files: ['js/src/translation/primitive-value-rules.js', 'rust/src/translation/primitive_value_rules.rs'],
    extract: (text) => captures(text, /^(?:export function|pub fn) ([a-zA-Z][a-zA-Z_]+)\(/gmu)
      .map((name) => name.replace(/([a-z])([A-Z])/gu, '$1-$2').replaceAll('_', '-').toLowerCase()),
    recordOf: (name) => `translation.${name}`,
  },
  {
    inventory: 'immutable product decisions',
    files: ['js/src/translation/product-emission-rules.js', 'js/src/translation/rust-product-rules.js', 'js/src/translation/product-source-rules.js', 'rust/src/translation/product_emission_rules.rs', 'rust/src/translation/rust/product_rules.rs', 'rust/src/translation/javascript/product_source_rules.rs'],
    extract: (text) => captures(text, /^(?:export function|function|pub(?: const)? fn) (joinProductItems|[a-zA-Z][a-zA-Z_]+)\(/gmu)
      .filter((name) => !name.startsWith('ml_'))
      .map((name) => name.replace(/([a-z])([A-Z])/gu, '$1-$2').replaceAll('_', '-').toLowerCase()),
    recordOf: (name) => `translation.${name === 'render-java-script-unit' ? 'render-javascript-unit-value' : name}`,
  },
  {
    inventory: 'JavaScript product source operations',
    files: ['js/src/translation/javascript-products.js'],
    extract: (text) => captures(text, /^export function (readJavaScriptProductIndex|requiresJavaScriptProductProof|validateJavaScriptProducts)\(/gmu)
      .map((name) => name.replace(/([a-z])([A-Z])/gu, '$1-$2').toLowerCase().replace('java-script', 'javascript')),
    recordOf: (name) => `translation.${({ 'requires-javascript-product-proof': 'require-javascript-product-proof',
      'validate-javascript-products': 'validate-javascript-product-source' })[name] ?? name}`,
  },
  {
    inventory: 'bounded repository input and CI operations',
    files: ['js/scripts/project-data-storage.mjs', 'js/scripts/generate-self-translation-report.mjs', 'js/scripts/test-groups.mjs', 'js/scripts/self-translation-report-worker-host.mjs'],
    extract: (text) => captures(text, /^(?:export )?(?:async )?function (readTemporaryGrammarInputs|readPortableExportBindings|readModuleHeaderBindings|partitionGroupFiles|measureSelfTranslationModule|createReportMeasurementWorker)\(/gmu)
      .map((name) => name.replace(/([a-z])([A-Z])/gu, '$1-$2').toLowerCase()),
    recordOf: (name) => `repository.${name}`,
  },
  {
    inventory: 'partitioned JavaScript suite evidence operations',
    files: ['js/scripts/issue-195-javascript-suite-partitions.mjs', 'js/scripts/issue-195-evidence-execution-records.mjs',
      'js/scripts/issue-195-evidence-command-logs.mjs'],
    extract: (text) => captures(text, /^export (?:async )?function ([a-zA-Z_]+)\(/gmu)
      .map(name => name.replace(/([a-z])([A-Z])/gu, '$1-$2').toLowerCase()),
    recordOf: (name) => `repository.${({
      'javascript-suite-test-groups': 'read-javascript-suite-test-groups',
      'javascript-suite-partition': 'create-javascript-suite-partition',
      'javascript-suite-selection': 'read-javascript-suite-selection',
      'javascript-suite-group-entries': 'read-javascript-suite-group-entries',
      'javascript-suite-command-arguments': 'read-javascript-suite-command-arguments',
      'is-java-script-suite-partition-name': 'detect-javascript-suite-partition-name',
      'merge-java-script-suite-partitions': 'merge-javascript-suite-partitions',
    })[name] ?? name}`,
  },
  {
    inventory: 'source constructor prototype definitions',
    files: ['js/src/translation/javascript.js', 'rust/src/translation/surface.rs'],
    extract: (text) => captures(text, /(?:let |^pub struct )(prototypeDefinition|ConstructorPrototypeDefinition)(?: = null;| \{)/gmu)
      .map((name) => name.replace(/([a-z])([A-Z])/gu, '$1-$2').toLowerCase()),
    recordOf: () => 'translation.constructor-prototype-definition',
  },
  {
    inventory: 'structured type decisions',
    files: ['js/src/translation/structured-type-rules.js', 'rust/src/translation/structured_type_rules.rs'],
    extract: (text) => captures(text, /^(?:export function|pub fn) (mergeOptionalAbsence|acceptTypeIdentity|acceptOptionalAbsence|renderPortableTypeIdentity|renderRustCompoundType|readRustOptionalPrelude|wrapPortableReferenceIdentity|renderRustReferenceType|merge_optional_absence|accept_type_identity|accept_optional_absence|render_portable_type_identity|render_rust_compound_type|read_rust_optional_prelude|wrap_portable_reference_identity|render_rust_reference_type)\(/gmu)
      .map((name) => name.replace(/([a-z])([A-Z])/gu, '$1-$2').replaceAll('_', '-').toLowerCase()),
    recordOf: (name) => `translation.${name}`,
  },
  {
    inventory: 'indentation scanner generator',
    files: ['js/scripts/scanner-families.mjs'],
    extract: (text) => captures(text, /^export function (indentationScanner|prefixedQuotedScanner)\(/gmu)
      .map((name) => name.replace(/([a-z])([A-Z])/gu, '$1-$2').toLowerCase()),
    recordOf: (name) => `grammar.generate-${name}`,
  },
  {
    inventory: 'declaration and root source contexts',
    files: ['js/scripts/native-csharp-declarations.mjs', 'js/scripts/native-csharp-roots.mjs'],
    extract: (text) => captures(text, /^export function (applyDeclarationSourceTransform|applyRootSourceTransform)\(/gmu)
      .map((name) => name.replace(/([a-z])([A-Z])/gu, '$1-$2').toLowerCase()),
    recordOf: (name) => `grammar.${name}`,
  },
  {
    inventory: 'recorded source context boundaries',
    files: ['js/scripts/native-source-contexts.mjs', 'js/scripts/native-source-boundaries.mjs', 'js/scripts/native-source-preferences.mjs', 'js/scripts/native-optional-call-context.mjs', 'js/scripts/native-argument-suffix-context.mjs', 'js/scripts/native-field-expression-context.mjs', 'js/scripts/native-syntactic-boundary-context.mjs', 'js/scripts/native-field-alternatives.mjs'],
    extract: (text) => captures(text, /^export function (transformFieldFollowingContext|transformTriviaContinuationBoundary|applySourceContextPreference|transformOptionalCallContext|transformArgumentSuffixContext|transformFieldExpressionContext|transformSyntacticBoundaryContext|transformOrderedFieldAlternatives)\(/gmu)
      .map((name) => name.replace(/([a-z])([A-Z])/gu, '$1-$2').toLowerCase()),
    recordOf: (name) => `grammar.${name}`,
  },
  {
    inventory: 'recorded compiler feedback',
    files: ['js/scripts/compiler-style-feedback.mjs'],
    extract: (text) => captures(text, /^export function (retainCompilerFeedback|applyRecordedCompilerFeedback|readCompilerFeedbackCorrections)\(/gmu)
      .map((name) => name.replace(/([a-z])([A-Z])/gu, '$1-$2').toLowerCase()),
    recordOf: (name) => `translation.${name}`,
  },
  {
    inventory: 'structured and counted scanner generators',
    files: ['js/scripts/structured-scanner-families.mjs', 'js/scripts/counted-interpolation-scanner.mjs', 'js/scripts/generate-native-scanner-profiles.mjs'],
    extract: (text) => captures(text, /^export function (hereDocumentScanner|interpolatedQuotedScanner|fencedBlockScanner|offsideLayoutScanner|countedInterpolationScanner|nativeScannerProfile)\(/gmu)
      .map((name) => name.replace(/([a-z])([A-Z])/gu, '$1-$2').toLowerCase()),
    recordOf: (name) => `grammar.generate-${name}`,
  },
  {
    inventory: 'template context scanner generator',
    files: ['js/scripts/template-context-scanner.mjs'],
    extract: (text) => captures(text, /^export function (templateContextScanner)\(/gmu)
      .map((name) => name.replace(/([a-z])([A-Z])/gu, '$1-$2').toLowerCase()),
    recordOf: (name) => `grammar.generate-${name}`,
  },
  {
    inventory: 'contextual bracket and raw literal scanner generators',
    files: ['js/scripts/scoped-layout-scanner.mjs', 'js/scripts/quoted-counted-scanner.mjs'],
    extract: (text) => captures(text, /^export function (scopedLayoutScanner|quotedCountedScanner)\(/gmu)
      .map((name) => name.replace(/([a-z])([A-Z])/gu, '$1-$2').toLowerCase()),
    recordOf: (name) => `grammar.generate-${name}`,
  },
  {
    inventory: 'native grammar embedding generator',
    files: ['js/scripts/build-language-catalog.mjs'],
    extract: (text) => captures(text, /^export function (nativeGrammarRustSource)\(/gmu)
      .map((name) => name.replace(/([a-z])([A-Z])/gu, '$1-$2').toLowerCase()),
    recordOf: () => 'grammar.generate-native-grammar-source',
  },
  {
    inventory: 'frontend decision result concepts',
    files: ['js/src/translation/frontend-rules.js', 'rust/src/translation/frontend_rules.rs'],
    extract: (text) => captures(text, /(?:\} |^pub enum )(UnicodeEscape)(?: \*\/| \{)/gmu).map(() => 'unicode-escape'),
    recordOf: () => 'translation.unicode-escape',
  },
]);

/**
 * The tables that keep former names decodable. Each reads `[former, current]`
 * pairs from its source file; the record of `current` lists `former` among its
 * former names.
 */
export const FORMER_NAME_TABLES = Object.freeze([
  {
    table: 'reviewed legacy native concept identities',
    file: 'parity/naming/grammar-name-expansions.json',
    extract: (text) => Object.entries(JSON.parse(text).concepts.formerNames ?? {})
      .flatMap(([current, formers]) => formers.map((former) => [former, current])),
  },
  {
    table: 'FORMER_CONCEPT_IDS',
    file: 'rust/src/concept_ontology.rs',
    extract: (text) =>
      [...block(text, 'pub const FORMER_CONCEPT_IDS', '\n];').matchAll(/\(\s*"([^"]+)",\s*"([^"]+)",?\s*\)/gu)].map(([, former, current]) => [former, current]),
  },
  {
    table: 'FORMER_GRAMMAR_CONSTRUCTS',
    file: 'rust/src/grammar/fidelity.rs',
    extract: (text) =>
      [...block(text, 'pub const FORMER_GRAMMAR_CONSTRUCTS', '\n];').matchAll(/\(\s*"([^"]+)",\s*"([^"]+)",?\s*\)/gu)].map(([, former, current]) => [former, current]),
    recordOf: (name) => `grammar.${name}`,
  },
  {
    table: 'FORMER_GRAMMAR_TAGS and FORMER_VALUE_PREFIXES',
    file: 'rust/src/grammar/links.rs',
    extract: (text) => {
      const constants = new Map(captures(text, /const ([A-Z_]+: &str = "[^"]+")/gu).map((entry) => entry.split(': &str = ').map((part) => part.replace(/"/gu, ''))));
      const tables = block(text, 'const FORMER_GRAMMAR_TAGS', '\n];') + block(text, 'const FORMER_VALUE_PREFIXES', '\n];');
      return [...tables.matchAll(/\(\s*"([^"]+)",\s*([A-Z_]+),?\s*\)/gu)].map(([, former, current]) => [
        former.replace(/::$/u, ''),
        (constants.get(current) ?? current).replace(/::$/u, ''),
      ]);
    },
    recordOf: grammarLinkRecord,
  },
  {
    // The published cross-format identities, position by position with the readable ones.
    table: 'CROSS_FORMAT_CONCEPTS',
    file: 'rust/src/document_formatting/profile.rs',
    extract: (text) => {
      const published = captures(block(text, 'pub const CROSS_FORMAT_CONCEPTS', '\n];'), /"([^"]+)"/gu);
      const current = captures(block(text, 'pub const CROSS_FORMAT_CONCEPT_IDS', '\n];'), /"([^"]+)"/gu);
      return published.map((former, index) => [former, current[index]]).filter(([former, readable]) => former !== readable);
    },
  },
  {
    table: 'FORMER_EXTERNAL_IDENTIFIER_VOCABULARY_PREFIX',
    file: 'rust/src/concept_ontology.rs',
    extract: (text) => {
      const former = text.match(/const FORMER_EXTERNAL_IDENTIFIER_VOCABULARY_PREFIX: &str = "([a-z-]+):";/u)?.[1];
      const current = text.match(/const EXTERNAL_IDENTIFIER_VOCABULARY_PREFIX: &str = "([a-z-]+):";/u)?.[1];
      return former && current ? [[former, current]] : [];
    },
    recordOf: (name) => `ontology.${name}`,
  },
]);

// Additive source bindings for the resolved type namespace and reviewed policy corrections.
const RESOLVED_SOURCE_NAME_BINDINGS = [
  {
    "inventory": "resolved Rust type namespace decisions",
    "files": {
      "js/src/translation/rust-type-namespace-rules.js": [
        {
          "name": "acceptRustTypeNamespaceModel",
          "id": "translation.accept-rust-type-namespace-model"
        },
        {
          "name": "acceptRustTypeNamespaceDeclaration",
          "id": "translation.accept-rust-type-namespace-declaration"
        },
        {
          "name": "acceptRustTypeNamespaceFields",
          "id": "translation.accept-rust-type-namespace-fields"
        },
        {
          "name": "acceptRustTypeNamespaceAnchor",
          "id": "translation.accept-rust-type-namespace-anchor"
        },
        {
          "name": "acceptRustTypeNamespaceBodyConstructor",
          "id": "translation.accept-rust-type-namespace-body-constructor"
        },
        {
          "name": "acceptRustTypeNamespaceBodyMethod",
          "id": "translation.accept-rust-type-namespace-body-method"
        },
        {
          "name": "acceptRustTypeNamespaceMethodDiscovery",
          "id": "translation.accept-rust-type-namespace-method-discovery"
        },
        {
          "name": "acceptRustTypeNamespaceUnitClone",
          "id": "translation.accept-rust-type-namespace-unit-clone"
        },
        {
          "name": "acceptRustTypeNamespaceContainer",
          "id": "translation.accept-rust-type-namespace-container"
        },
        {
          "name": "acceptRustTypeNamespaceDiscoveryType",
          "id": "translation.accept-rust-type-namespace-discovery-type"
        },
        {
          "name": "acceptRustTypeNamespaceRecordValue",
          "id": "translation.accept-rust-type-namespace-record-value"
        },
        {
          "name": "acceptRustTypeNamespacePosition",
          "id": "translation.accept-rust-type-namespace-position"
        }
      ],
      "rust/src/translation/type_namespace_rules.rs": [
        {
          "name": "accept_rust_type_namespace_model",
          "id": "translation.accept-rust-type-namespace-model"
        },
        {
          "name": "accept_rust_type_namespace_declaration",
          "id": "translation.accept-rust-type-namespace-declaration"
        },
        {
          "name": "accept_rust_type_namespace_fields",
          "id": "translation.accept-rust-type-namespace-fields"
        },
        {
          "name": "accept_rust_type_namespace_anchor",
          "id": "translation.accept-rust-type-namespace-anchor"
        },
        {
          "name": "accept_rust_type_namespace_body_constructor",
          "id": "translation.accept-rust-type-namespace-body-constructor"
        },
        {
          "name": "accept_rust_type_namespace_body_method",
          "id": "translation.accept-rust-type-namespace-body-method"
        },
        {
          "name": "accept_rust_type_namespace_method_discovery",
          "id": "translation.accept-rust-type-namespace-method-discovery"
        },
        {
          "name": "accept_rust_type_namespace_unit_clone",
          "id": "translation.accept-rust-type-namespace-unit-clone"
        },
        {
          "name": "accept_rust_type_namespace_container",
          "id": "translation.accept-rust-type-namespace-container"
        },
        {
          "name": "accept_rust_type_namespace_discovery_type",
          "id": "translation.accept-rust-type-namespace-discovery-type"
        },
        {
          "name": "accept_rust_type_namespace_record_value",
          "id": "translation.accept-rust-type-namespace-record-value"
        },
        {
          "name": "accept_rust_type_namespace_position",
          "id": "translation.accept-rust-type-namespace-position"
        }
      ]
    }
  },
  {
    "inventory": "resolved Rust type namespace source operations",
    "files": {
      "js/src/translation/rust-type-namespace.js": [
        {
          "name": "checkRustTypeNamespaceRecordShapes",
          "id": "translation.check-rust-type-namespace-record-shapes"
        },
        {
          "name": "checkRustTypeNamespaceContainers",
          "id": "translation.check-rust-type-namespace-containers"
        },
        {
          "name": "acceptRustTypeNamespaceSpellingSegment",
          "id": "translation.accept-rust-type-namespace-spelling-segment"
        },
        {
          "name": "createRustTypeNamespaceProof",
          "id": "translation.create-rust-type-namespace-proof"
        },
        {
          "name": "requireRustTypeNamespaceAnchor",
          "id": "translation.require-rust-type-namespace-anchor"
        },
        {
          "name": "validateRustTypeNamespace",
          "id": "translation.validate-rust-type-namespace"
        },
        {
          "name": "requireRustTypeNamespaceWithoutImports",
          "id": "translation.require-rust-type-namespace-without-imports"
        }
      ],
      "rust/src/translation/source_type_namespaces.rs": [
        {
          "name": "validate_namespace_value_shapes",
          "id": "translation.check-rust-type-namespace-record-shapes"
        },
        {
          "name": "validate_source_type_namespaces",
          "id": "translation.validate-rust-type-namespace"
        },
        {
          "name": "policy",
          "id": "translation.read-rust-type-namespace-policy"
        },
        {
          "name": "validate_type_namespace_records",
          "id": "translation.validate-rust-type-namespace-records"
        },
        {
          "name": "validate_type_namespace_declaration_record",
          "id": "translation.validate-rust-type-namespace-declaration-record"
        },
        {
          "name": "spelling",
          "id": "translation.read-rust-type-namespace-spelling"
        },
        {
          "name": "position",
          "id": "translation.check-rust-type-namespace-position"
        },
        {
          "name": "unrepresented",
          "id": "translation.check-rust-type-namespace-record-fields"
        }
      ],
      "js/src/typed-program-expression-projection.js": [
        {
          "name": "projectLinkedProgramDeclarationPath",
          "id": "translation.project-linked-program-declaration-path"
        }
      ],
      "rust/src/translation/rust/type_namespaces.rs": [
        {
          "name": "accept_namespace_segment",
          "id": "translation.accept-rust-type-namespace-spelling-segment"
        },
        {
          "name": "capture_type_namespace",
          "id": "translation.create-rust-type-namespace-proof"
        },
        {
          "name": "require_anchor",
          "id": "translation.require-rust-type-namespace-anchor"
        },
        {
          "name": "require_type_namespace_without_imports",
          "id": "translation.require-rust-type-namespace-without-imports"
        },
        {
          "name": "new",
          "id": "translation.create-rust-type-namespace-catalog"
        },
        {
          "name": "lookup",
          "id": "translation.look-up-rust-type-namespace-owner"
        },
        {
          "name": "require_type_namespace_without_constant",
          "id": "translation.require-rust-type-namespace-without-constant"
        },
        {
          "name": "require_type_namespace_body_constructor",
          "id": "translation.require-rust-type-namespace-body-constructor"
        },
        {
          "name": "require_type_namespace_body_method",
          "id": "translation.require-rust-type-namespace-body-method"
        },
        {
          "name": "read_type_namespace_discovery_type",
          "id": "translation.read-rust-type-namespace-discovery-type"
        },
        {
          "name": "require_type_namespace_without_externals",
          "id": "translation.require-rust-type-namespace-without-externals"
        },
        {
          "name": "require_unit_clone_attached_enum",
          "id": "translation.require-rust-unit-clone-attached-enumeration"
        },
        {
          "name": "require_unit_clone_attached_variant",
          "id": "translation.require-rust-unit-clone-attached-variant"
        },
        {
          "name": "has_clone",
          "id": "translation.check-rust-unit-clone-attribute"
        }
      ],
      "js/src/translation/rust-unit-clone.js": [
        {
          "name": "captureRustUnitCloneDerivation",
          "id": "translation.capture-rust-unit-clone-derivation"
        },
        {
          "name": "checkRustUnitCloneDerivation",
          "id": "translation.check-rust-unit-clone-derivation"
        },
        {
          "name": "checkRustUnitCloneDerivations",
          "id": "translation.check-rust-program-unit-clone-derivations"
        },
        {
          "name": "acceptRustUnitCloneReceiver",
          "id": "translation.accept-rust-unit-clone-receiver"
        }
      ],
      "rust/src/translation/source_unit_clone_derivations.rs": [
        {
          "name": "capture_unit_clone_derivation",
          "id": "translation.capture-rust-unit-clone-derivation"
        },
        {
          "name": "validate_unit_clone_declaration_record",
          "id": "translation.check-rust-unit-clone-derivation"
        },
        {
          "name": "validate_source_unit_clone_derivations",
          "id": "translation.check-rust-program-unit-clone-derivations"
        },
        {
          "name": "validate_unit_clone_records",
          "id": "translation.check-rust-program-unit-clone-derivations"
        },
        {
          "name": "validate_surface_unit_clone_derivations",
          "id": "translation.validate-rust-surface-unit-clone-derivations"
        }
      ],
      "rust/src/typed_program_links/checked/declarations.rs": [
        {
          "name": "project_data",
          "id": "translation.project-checked-data-declaration"
        }
      ]
    }
  },
  {
    "inventory": "resolved Rust type namespace catalog",
    "files": {
      "rust/src/translation/rust/type_namespaces.rs": [
        {
          "name": "TypeNamespaceCatalog",
          "id": "translation.rust-type-namespace-catalog"
        }
      ]
    }
  },
  {
    "inventory": "represented program host containers",
    "files": {
      "js/src/translation/owned-program-containers.js": [
        {
          "name": "checkOwnedProgramContainers",
          "id": "translation.check-owned-program-containers"
        }
      ]
    }
  },
  {
    "inventory": "resolved Rust type namespace fixture operations",
    "files": {
      "js/scripts/generate-rust-type-namespace-fixtures.mjs": [
        {
          "name": "rustTypeNamespaceRecordFixture",
          "id": "repository.create-rust-type-namespace-record-fixture"
        },
        {
          "name": "rustTypeNamespaceField",
          "id": "repository.read-rust-type-namespace-fixture-field"
        },
        {
          "name": "rustTypeNamespaceFixture",
          "id": "repository.create-rust-type-namespace-fixture"
        },
        {
          "name": "rustTypeNamespaceTraitFixture",
          "id": "repository.create-rust-type-namespace-trait-fixture"
        },
        {
          "name": "rustTypeNamespaceContainerFixture",
          "id": "repository.create-rust-type-namespace-container-fixture"
        }
      ]
    }
  },
  {
    "inventory": "resolved Rust type namespace model selections",
    "files": {
      "js/src/translation/rust.js": [
        {
          "name": "typeNamespaceModel",
          "id": "translation.rust-type-namespace-model-option"
        }
      ],
      "rust/src/translation/rust.rs": [
        {
          "name": "type_namespace_model",
          "id": "translation.rust-type-namespace-model-option"
        }
      ],
      "js/src/typed-program-links.js": [
        {
          "name": "rustTypeNamespaceModel",
          "id": "translation.typed-program-rust-type-namespace-model-option"
        }
      ],
      "js/src/index.d.ts": [
        {
          "name": "rustTypeNamespaceModel",
          "id": "translation.typed-program-rust-type-namespace-model-option"
        }
      ]
    }
  },
  {
    "inventory": "owned source namespace and derivation fields",
    "files": {
      "js/src/typed-program-checked-projection.js": [
        {
          "name": "sourceTypeNamespace",
          "id": "translation.source-type-namespace"
        }
      ],
      "rust/src/translation/ir/declarations.rs": [
        {
          "name": "source_type_namespace",
          "id": "translation.source-type-namespace"
        },
        {
          "name": "source_unit_clone_derivation",
          "id": "translation.source-unit-clone-derivation"
        }
      ],
      "rust/src/translation/surface.rs": [
        {
          "name": "source_type_namespace",
          "id": "translation.source-type-namespace"
        },
        {
          "name": "source_unit_clone_derivation",
          "id": "translation.source-unit-clone-derivation"
        }
      ],
      "js/src/translation/rust-unit-clone.js": [
        {
          "name": "sourceUnitCloneDerivation",
          "id": "translation.source-unit-clone-derivation"
        },
        {
          "name": "datatypeOwner",
          "id": "translation.source-derivation-data-type-owner"
        }
      ],
      "rust/src/translation/source_unit_clone_derivations.rs": [
        {
          "name": "datatypeOwner",
          "id": "translation.source-derivation-data-type-owner"
        }
      ]
    }
  },
  {
    "inventory": "resolved Rust type namespace record policy fields",
    "files": {
      "js/src/translation/data/rust-type-namespace.lino": [
        {
          "name": "recordBooleanFields",
          "id": "translation.source-type-namespace-boolean-field-list"
        },
        {
          "name": "sourceOffsetMaximum",
          "id": "translation.source-type-namespace-offset-maximum"
        }
      ],
      "rust/src/data/rust-type-namespace.lino": [
        {
          "name": "recordBooleanFields",
          "id": "translation.source-type-namespace-boolean-field-list"
        },
        {
          "name": "sourceOffsetMaximum",
          "id": "translation.source-type-namespace-offset-maximum"
        }
      ]
    }
  },
  {
    "inventory": "reviewed generated policy correction helpers",
    "files": {
      "js/scripts/generate-link-network-binary-rules.mjs": [
        {
          "name": "recordFollowupReviewedStyles",
          "id": "repository.record-followup-reviewed-binary-styles"
        }
      ],
      "js/scripts/generate-typed-program-link-rules.mjs": [
        {
          "name": "combineSelectorConditions",
          "id": "repository.combine-program-selector-conditions"
        }
      ],
      "js/scripts/product-rule-style-corrections.mjs": [
        {
          "name": "readProductSourceConditionStyleCorrections",
          "id": "repository.read-product-source-condition-style-corrections"
        },
        {
          "name": "bindConditionBlocks",
          "id": "repository.bind-product-source-condition-blocks"
        }
      ],
      "js/scripts/generate-rust-completion-rules.mjs": [
        {
          "name": "removeRedundantBooleanGrouping",
          "id": "repository.remove-redundant-completion-boolean-grouping"
        }
      ],
      "js/scripts/resource-description-reviewed-styles.mjs": [
        {
          "name": "hasConditionBinding",
          "id": "repository.check-resource-condition-binding"
        },
        {
          "name": "excludeBindings",
          "id": "repository.exclude-resource-condition-bindings"
        },
        {
          "name": "scalarEquality",
          "id": "repository.check-resource-scalar-equality"
        },
        {
          "name": "scalarTerms",
          "id": "repository.read-resource-scalar-terms"
        },
        {
          "name": "factor",
          "id": "repository.factor-resource-scalar-conditions"
        },
        {
          "name": "scalar",
          "id": "repository.check-resource-scalar-expression"
        },
        {
          "name": "simpleBoolean",
          "id": "repository.check-resource-simple-boolean-expression"
        }
      ],
      "js/tests/translation-resource-description-rule-styles.test.js": [
        {
          "name": "replay",
          "id": "repository.replay-reviewed-resource-policy"
        }
      ]
    }
  }
];
const normalizeResolvedSourceName = name => name.replace(/([a-z])([A-Z])/gu, '$1-$2').replaceAll('_', '-').toLowerCase().replaceAll('datatype', 'data-type').replaceAll('namespaces', 'namespace').replace(/enum$/u, 'enumeration').replace(/^unrepresented$/u, 'check-record-fields');
const RESOLVED_SOURCE_NAME_INVENTORIES = RESOLVED_SOURCE_NAME_BINDINGS.map(({inventory, files}) => ({
  inventory, files: Object.keys(files),
  extract: (text, file) => files[file].filter(({name}) => new RegExp('\\b' + name + '\\b', 'u').test(text)).map(({name}) => normalizeResolvedSourceName(name)),
  recordOf: name => Object.values(files).flat().find(binding => normalizeResolvedSourceName(binding.name) === name).id,
}));
// Additive source bindings for paired owned text, pure Unit calls and source span numbers.
const VALUE_SOURCE_NAME_BINDINGS = [
  {
    "inventory": "owned Rust local text decisions",
    "files": {
      "js/src/translation/rust-owned-local-state-rules.js": [
        {
          "name": "acceptRustOwnedLocalStateModel",
          "id": "translation.accept-rust-owned-local-state-model"
        },
        {
          "name": "acceptRustOwnedLocalValueType",
          "id": "translation.accept-rust-owned-local-value-type"
        },
        {
          "name": "acceptRustOwnedLocalExpression",
          "id": "translation.accept-rust-owned-local-expression"
        },
        {
          "name": "acceptRustOwnedLocalBinaryOperation",
          "id": "translation.accept-rust-owned-local-binary-operation"
        },
        {
          "name": "acceptRustOwnedLocalSourceOperation",
          "id": "translation.accept-rust-owned-local-source-operation"
        },
        {
          "name": "acceptRustOwnedLocalContainer",
          "id": "translation.accept-rust-owned-local-container"
        },
        {
          "name": "acceptRustOwnedLocalRecordValue",
          "id": "translation.accept-rust-owned-local-record-value"
        },
        {
          "name": "acceptRustOwnedLocalSourceSpan",
          "id": "translation.accept-rust-owned-local-source-span"
        },
        {
          "name": "acceptRustOwnedLocalTextUnits",
          "id": "translation.accept-rust-owned-local-text-units"
        }
      ],
      "rust/src/translation/rust/owned_local_state_rules.rs": [
        {
          "name": "accept_rust_owned_local_state_model",
          "id": "translation.accept-rust-owned-local-state-model"
        },
        {
          "name": "accept_rust_owned_local_value_type",
          "id": "translation.accept-rust-owned-local-value-type"
        },
        {
          "name": "accept_rust_owned_local_expression",
          "id": "translation.accept-rust-owned-local-expression"
        },
        {
          "name": "accept_rust_owned_local_binary_operation",
          "id": "translation.accept-rust-owned-local-binary-operation"
        },
        {
          "name": "accept_rust_owned_local_source_operation",
          "id": "translation.accept-rust-owned-local-source-operation"
        },
        {
          "name": "accept_rust_owned_local_container",
          "id": "translation.accept-rust-owned-local-container"
        },
        {
          "name": "accept_rust_owned_local_record_value",
          "id": "translation.accept-rust-owned-local-record-value"
        },
        {
          "name": "accept_rust_owned_local_source_span",
          "id": "translation.accept-rust-owned-local-source-span"
        },
        {
          "name": "accept_rust_owned_local_text_units",
          "id": "translation.accept-rust-owned-local-text-units"
        }
      ]
    }
  },
  {
    "inventory": "owned Rust local text source operations",
    "files": {
      "js/src/translation/rust-owned-local-state-rules.js": [
        {
          "name": "renderRustOwnedLocalTextInput",
          "id": "translation.render-rust-owned-local-text-input"
        },
        {
          "name": "acceptRustOwnedLocalTextInput",
          "id": "translation.accept-rust-owned-local-text-input"
        }
      ],
      "rust/src/translation/rust/owned_local_state_rules.rs": [
        {
          "name": "render_rust_owned_local_text_input",
          "id": "translation.render-rust-owned-local-text-input"
        }
      ],
      "js/src/translation/rust-owned-local-state.js": [
        {
          "name": "checkRustOwnedLocalStateContainers",
          "id": "translation.check-rust-owned-local-state-containers"
        },
        {
          "name": "checkRustOwnedLocalStateRecordShapes",
          "id": "translation.check-rust-owned-local-state-record-shapes"
        },
        {
          "name": "ownedLocalType",
          "id": "translation.check-rust-owned-local-value-type"
        },
        {
          "name": "ownedLocalPair",
          "id": "translation.check-rust-owned-local-pair-type"
        },
        {
          "name": "ownedLocalPosition",
          "id": "translation.check-rust-owned-local-source-position"
        },
        {
          "name": "prepareRustOwnedLocalStateProof",
          "id": "translation.prepare-rust-owned-local-state-proof"
        },
        {
          "name": "checkRustOwnedLocalStateProof",
          "id": "translation.check-rust-owned-local-state-proof"
        },
        {
          "name": "bodyRecords",
          "id": "translation.read-rust-owned-local-body-records"
        }
      ],
      "rust/src/translation/owned_local_state.rs": [
        {
          "name": "owned_local_type",
          "id": "translation.check-rust-owned-local-value-type"
        },
        {
          "name": "value_type",
          "id": "translation.check-rust-owned-local-value-type"
        },
        {
          "name": "owned_local_pair",
          "id": "translation.check-rust-owned-local-pair-type"
        },
        {
          "name": "pair_type",
          "id": "translation.check-rust-owned-local-pair-type"
        },
        {
          "name": "owned_local_position",
          "id": "translation.check-rust-owned-local-source-position"
        },
        {
          "name": "prepare_owned_local_state_proof",
          "id": "translation.prepare-rust-owned-local-state-proof"
        },
        {
          "name": "validate_owned_local_state",
          "id": "translation.check-rust-owned-local-state-proof"
        },
        {
          "name": "validate_owned_local_state_records",
          "id": "translation.check-rust-owned-local-state-proof"
        },
        {
          "name": "policy",
          "id": "translation.read-rust-owned-local-state-policy"
        },
        {
          "name": "same_order",
          "id": "translation.compare-rust-owned-local-record-order"
        },
        {
          "name": "read_source_offset",
          "id": "translation.read-rust-owned-local-source-offset"
        }
      ],
      "rust/src/translation/owned_local_state/records.rs": [
        {
          "name": "prepare",
          "id": "translation.prepare-rust-owned-local-state-proof"
        },
        {
          "name": "collect",
          "id": "translation.read-rust-owned-local-body-records"
        },
        {
          "name": "visit",
          "id": "translation.read-rust-owned-local-body-records"
        }
      ],
      "rust/src/translation/owned_local_state/validation.rs": [
        {
          "name": "validate",
          "id": "translation.check-rust-owned-local-state-proof"
        }
      ],
      "js/src/translation/rust-owned-local-state-reader.js": [
        {
          "name": "readRustOwnedLocalStateBody",
          "id": "translation.read-rust-owned-local-state-body"
        },
        {
          "name": "mergeAvailability",
          "id": "translation.merge-rust-owned-local-value-availability"
        }
      ],
      "rust/src/translation/rust/owned_local_state.rs": [
        {
          "name": "read_owned_local_state_body",
          "id": "translation.read-rust-owned-local-state-body"
        },
        {
          "name": "merge_availability",
          "id": "translation.merge-rust-owned-local-value-availability"
        },
        {
          "name": "require_owned_local_indirection",
          "id": "translation.require-rust-owned-local-indirection-proof"
        },
        {
          "name": "read_owned_local_copy",
          "id": "translation.read-rust-owned-local-copy"
        },
        {
          "name": "require_owned_local_program",
          "id": "translation.require-rust-owned-local-program"
        },
        {
          "name": "binding_expression",
          "id": "translation.create-rust-owned-local-binding-expression"
        }
      ]
    }
  },
  {
    "inventory": "owned Rust local text checked body operations",
    "files": {
      "rust/src/translation/owned_local_state/expression.rs": [
        {
          "name": "expression",
          "id": "translation.check-rust-owned-local-expression"
        },
        {
          "name": "read",
          "id": "translation.check-rust-owned-local-read"
        },
        {
          "name": "binding",
          "id": "translation.check-rust-owned-local-binding"
        },
        {
          "name": "tuple_index",
          "id": "translation.check-rust-owned-local-tuple-index"
        },
        {
          "name": "literal",
          "id": "translation.check-rust-owned-local-literal"
        }
      ],
      "rust/src/translation/rust/owned_local_state/syntax.rs": [
        {
          "name": "owned_local_raw_expression",
          "id": "translation.read-rust-owned-local-raw-expression"
        },
        {
          "name": "owned_local_pattern",
          "id": "translation.read-rust-owned-local-pattern"
        },
        {
          "name": "owned_local_pattern_name",
          "id": "translation.read-rust-owned-local-pattern-name"
        },
        {
          "name": "owned_local_pattern_at_depth",
          "id": "translation.read-rust-owned-local-pattern-at-depth"
        },
        {
          "name": "owned_local_syntax_block",
          "id": "translation.read-rust-owned-local-syntax-block"
        },
        {
          "name": "owned_local_statements",
          "id": "translation.read-rust-owned-local-statements"
        }
      ],
      "rust/src/translation/rust/owned_local_state/lowering.rs": [
        {
          "name": "sequence",
          "id": "translation.lower-rust-owned-local-sequence"
        }
      ]
    }
  },
  {
    "inventory": "owned Rust local text source expression lowering",
    "files": {
      "rust/src/translation/rust/owned_local_state/expressions.rs": [
        {
          "name": "reference",
          "id": "translation.read-rust-owned-local-value-reference"
        },
        {
          "name": "expression",
          "id": "translation.lower-rust-owned-local-expression"
        }
      ]
    }
  },
  {
    "inventory": "owned Rust local text record shape operations",
    "files": {
      "rust/src/translation/owned_local_state/shapes.rs": [
        {
          "name": "validate",
          "id": "translation.check-rust-owned-local-state-record-shapes"
        }
      ]
    }
  },
  {
    "inventory": "owned Rust local text linked order operations",
    "files": {
      "rust/src/typed_program_links/codec_owned_local_state.rs": [
        {
          "name": "validate_order",
          "id": "translation.validate-owned-local-state-field-order"
        },
        {
          "name": "prepare_order",
          "id": "translation.prepare-owned-local-state-field-order"
        },
        {
          "name": "prepare_decoded_order",
          "id": "translation.prepare-decoded-owned-local-state-order"
        },
        {
          "name": "validate_original_shapes",
          "id": "translation.validate-owned-local-state-original-shapes"
        },
        {
          "name": "compare_policy_order",
          "id": "translation.compare-owned-local-state-policy-order"
        },
        {
          "name": "compare_occurrence_span_order",
          "id": "translation.compare-owned-local-state-occurrence-order"
        }
      ]
    }
  },
  {
    "inventory": "owned Rust local text fixture and rule generators",
    "files": {
      "experiments/generate-rust-owned-local-state-fixture.mjs": [
        {
          "name": "rustOwnedLocalStateFixture",
          "id": "repository.create-rust-owned-local-state-fixture"
        }
      ],
      "experiments/generate-rust-owned-local-container-fixture.mjs": [
        {
          "name": "rustOwnedLocalContainerFixture",
          "id": "repository.create-rust-owned-local-container-fixture"
        },
        {
          "name": "readRustOwnedLocalContainerOwner",
          "id": "repository.read-rust-owned-local-container-fixture-owner"
        },
        {
          "name": "applyRustOwnedLocalContainerMutation",
          "id": "repository.apply-rust-owned-local-container-fixture-mutation"
        }
      ],
      "experiments/generate-rust-owned-local-source-operation-fixture.mjs": [
        {
          "name": "rustOwnedLocalSourceOperationFixture",
          "id": "repository.create-rust-owned-local-source-operation-fixture"
        }
      ],
      "experiments/rust-owned-local-authentic-source.mjs": [
        {
          "name": "withRustOwnedLocalAuthenticSource",
          "id": "repository.read-authentic-owned-local-rust-source"
        }
      ],
      "experiments/generate-rust-owned-local-state-rules.mjs": [
        {
          "name": "rustOwnedLocalStatePortableRules",
          "id": "repository.generate-rust-owned-local-portable-rules"
        },
        {
          "name": "rustOwnedLocalStateRules",
          "id": "repository.generate-rust-owned-local-state-rules"
        },
        {
          "name": "generateNativeOwnedLocalStateRules",
          "id": "repository.generate-native-owned-local-state-rules"
        }
      ]
    }
  },
  {
    "inventory": "owned Rust local text model selections",
    "files": {
      "js/src/translation/rust.js": [
        {
          "name": "localValueStateModel",
          "id": "translation.rust-owned-local-state-model-option"
        }
      ],
      "rust/src/translation/rust.rs": [
        {
          "name": "local_value_state_model",
          "id": "translation.rust-owned-local-state-model-option"
        },
        {
          "name": "local_value_state",
          "id": "translation.rust-owned-local-state-model-option"
        }
      ],
      "js/src/typed-program-links.js": [
        {
          "name": "rustOwnedLocalStateModel",
          "id": "translation.typed-program-rust-owned-local-state-model-option"
        }
      ],
      "js/src/index.d.ts": [
        {
          "name": "rustOwnedLocalStateModel",
          "id": "translation.typed-program-rust-owned-local-state-model-option"
        }
      ]
    }
  },
  {
    "inventory": "owned local text source proof roles",
    "files": {
      "js/src/typed-program-checked-projection.js": [
        {
          "name": "sourceLocalState",
          "id": "translation.function-source-owned-local-state-proof-field"
        }
      ],
      "rust/src/translation/ir/declarations.rs": [
        {
          "name": "source_local_state",
          "id": "translation.function-source-owned-local-state-proof-field"
        }
      ],
      "rust/src/translation/surface.rs": [
        {
          "name": "source_local_state",
          "id": "translation.function-source-owned-local-state-proof-field"
        }
      ]
    }
  },
  {
    "inventory": "closed Rust pure Unit source decisions",
    "files": {
      "js/src/translation/rust-pure-unit-call-rules.js": [
        {
          "name": "acceptRustPureUnitCallModel",
          "id": "translation.accept-rust-pure-unit-call-model"
        },
        {
          "name": "acceptRustPureUnitNameCharacter",
          "id": "translation.accept-rust-pure-unit-name-character"
        },
        {
          "name": "acceptRustPureUnitName",
          "id": "translation.accept-rust-pure-unit-name"
        },
        {
          "name": "acceptRustPureUnitHeaderForm",
          "id": "translation.accept-rust-pure-unit-header-form"
        },
        {
          "name": "acceptRustPureUnitBodyForm",
          "id": "translation.accept-rust-pure-unit-body-form"
        },
        {
          "name": "acceptRustPureUnitPathForm",
          "id": "translation.accept-rust-pure-unit-path-form"
        },
        {
          "name": "acceptRustPureUnitContext",
          "id": "translation.accept-rust-pure-unit-context"
        },
        {
          "name": "acceptRustPureUnitVisibility",
          "id": "translation.accept-rust-pure-unit-visibility"
        },
        {
          "name": "acceptRustPureUnitContainer",
          "id": "translation.accept-rust-pure-unit-container"
        },
        {
          "name": "acceptRustPureUnitSourceOffset",
          "id": "translation.accept-rust-pure-unit-source-offset"
        }
      ],
      "rust/src/translation/rust/pure_unit_call_rules.rs": [
        {
          "name": "accept_rust_pure_unit_call_model",
          "id": "translation.accept-rust-pure-unit-call-model"
        },
        {
          "name": "accept_rust_pure_unit_name_character",
          "id": "translation.accept-rust-pure-unit-name-character"
        },
        {
          "name": "accept_rust_pure_unit_name",
          "id": "translation.accept-rust-pure-unit-name"
        },
        {
          "name": "accept_rust_pure_unit_header_form",
          "id": "translation.accept-rust-pure-unit-header-form"
        },
        {
          "name": "accept_rust_pure_unit_body_form",
          "id": "translation.accept-rust-pure-unit-body-form"
        },
        {
          "name": "accept_rust_pure_unit_path_form",
          "id": "translation.accept-rust-pure-unit-path-form"
        },
        {
          "name": "accept_rust_pure_unit_context",
          "id": "translation.accept-rust-pure-unit-context"
        },
        {
          "name": "accept_rust_pure_unit_visibility",
          "id": "translation.accept-rust-pure-unit-visibility"
        },
        {
          "name": "accept_rust_pure_unit_container",
          "id": "translation.accept-rust-pure-unit-container"
        },
        {
          "name": "accept_rust_pure_unit_source_offset",
          "id": "translation.accept-rust-pure-unit-source-offset"
        }
      ]
    }
  },
  {
    "inventory": "closed Rust pure Unit source operations",
    "files": {
      "js/src/translation/rust-pure-unit-calls.js": [
        {
          "name": "readRustPureUnitItemContext",
          "id": "translation.read-rust-pure-unit-item-context"
        },
        {
          "name": "readRustPureUnitFunction",
          "id": "translation.read-rust-pure-unit-function"
        },
        {
          "name": "readRustPureUnitMain",
          "id": "translation.read-rust-pure-unit-main"
        },
        {
          "name": "finishRustPureUnitSource",
          "id": "translation.finish-rust-pure-unit-source"
        },
        {
          "name": "checkRustPureUnitCalls",
          "id": "translation.check-rust-pure-unit-calls"
        },
        {
          "name": "equalRustPureUnitSourceSpans",
          "id": "translation.equal-rust-pure-unit-source-spans"
        },
        {
          "name": "nameAccepted",
          "id": "translation.check-rust-pure-unit-source-name"
        },
        {
          "name": "readName",
          "id": "translation.read-rust-pure-unit-name"
        },
        {
          "name": "readUnit",
          "id": "translation.read-rust-pure-unit-token-pair"
        },
        {
          "name": "range",
          "id": "translation.create-rust-pure-unit-source-span"
        }
      ],
      "rust/src/translation/rust/pure_unit_calls.rs": [
        {
          "name": "pure_unit_item_context",
          "id": "translation.read-rust-pure-unit-item-context"
        },
        {
          "name": "pure_unit_function",
          "id": "translation.read-rust-pure-unit-function"
        },
        {
          "name": "pure_unit_main",
          "id": "translation.read-rust-pure-unit-main"
        },
        {
          "name": "finish_pure_unit_source",
          "id": "translation.finish-rust-pure-unit-source"
        },
        {
          "name": "pure_unit_attribute_guard",
          "id": "translation.require-rust-pure-unit-source-attributes"
        },
        {
          "name": "ordinary_name",
          "id": "translation.check-rust-pure-unit-source-name"
        },
        {
          "name": "pure_unit_name",
          "id": "translation.read-rust-pure-unit-name"
        },
        {
          "name": "pure_unit_token_pair",
          "id": "translation.read-rust-pure-unit-token-pair"
        },
        {
          "name": "actual_span",
          "id": "translation.create-rust-pure-unit-source-span"
        }
      ],
      "rust/src/translation/source_pure_unit_calls.rs": [
        {
          "name": "validate_source_pure_unit_calls",
          "id": "translation.check-rust-pure-unit-calls"
        },
        {
          "name": "validate_pure_unit_records",
          "id": "translation.check-rust-pure-unit-calls"
        },
        {
          "name": "validate_pure_unit_declaration_record",
          "id": "translation.validate-rust-pure-unit-declaration-record"
        },
        {
          "name": "policy",
          "id": "translation.read-rust-pure-unit-policy"
        },
        {
          "name": "source_offset",
          "id": "translation.read-rust-pure-unit-source-offset"
        }
      ]
    }
  },
  {
    "inventory": "closed Rust pure Unit model selections",
    "files": {
      "js/src/translation/rust.js": [
        {
          "name": "pureUnitCallModel",
          "id": "translation.rust-pure-unit-call-model-option"
        }
      ],
      "rust/src/translation/rust.rs": [
        {
          "name": "pure_unit_call_model",
          "id": "translation.rust-pure-unit-call-model-option"
        },
        {
          "name": "pure_unit_call",
          "id": "translation.rust-pure-unit-call-model-option"
        }
      ],
      "rust/src/translation/rust/source_profiles.rs": [
        {
          "name": "pure_unit_call",
          "id": "translation.rust-pure-unit-call-model-option"
        }
      ],
      "js/src/typed-program-links.js": [
        {
          "name": "rustPureUnitCallModel",
          "id": "translation.typed-program-rust-pure-unit-call-model-option"
        }
      ],
      "js/src/index.d.ts": [
        {
          "name": "rustPureUnitCallModel",
          "id": "translation.typed-program-rust-pure-unit-call-model-option"
        }
      ]
    }
  },
  {
    "inventory": "closed Rust pure Unit source proof roles",
    "files": {
      "js/src/translation/rust-pure-unit-calls.js": [
        {
          "name": "sourcePureUnitCalls",
          "id": "translation.program-source-pure-unit-call-model-field"
        },
        {
          "name": "sourcePureUnit",
          "id": "translation.function-source-pure-unit-proof-field"
        },
        {
          "name": "sourcePureUnitCall",
          "id": "translation.statement-source-pure-unit-call-proof-field"
        }
      ],
      "rust/src/translation/ir.rs": [
        {
          "name": "source_pure_unit_calls",
          "id": "translation.program-source-pure-unit-call-model-field"
        },
        {
          "name": "source_pure_unit_call",
          "id": "translation.statement-source-pure-unit-call-proof-field"
        }
      ],
      "rust/src/translation/surface.rs": [
        {
          "name": "source_pure_unit_calls",
          "id": "translation.program-source-pure-unit-call-model-field"
        },
        {
          "name": "source_pure_unit",
          "id": "translation.function-source-pure-unit-proof-field"
        },
        {
          "name": "source_pure_unit_call",
          "id": "translation.statement-source-pure-unit-call-proof-field"
        }
      ],
      "rust/src/translation/ir/declarations.rs": [
        {
          "name": "source_pure_unit",
          "id": "translation.function-source-pure-unit-proof-field"
        }
      ]
    }
  },
  {
    "inventory": "closed Rust pure Unit fixture generators",
    "files": {
      "js/scripts/generate-rust-pure-unit-call-fixtures.mjs": [
        {
          "name": "rustPureUnitCallFixture",
          "id": "repository.create-rust-pure-unit-call-fixture"
        },
        {
          "name": "applyPureUnitContainerMutation",
          "id": "repository.apply-pure-unit-container-fixture-mutation"
        }
      ],
      "js/scripts/generate-rust-public-pure-unit-call-fixtures.mjs": [
        {
          "name": "rustPublicPureUnitCallFixture",
          "id": "repository.create-rust-public-pure-unit-call-fixture"
        }
      ]
    }
  },
  {
    "inventory": "shared source span numerical operations",
    "files": {
      "js/src/source-span-rules.js": [
        {
          "name": "acceptSourceSpanOffset",
          "id": "translation.accept-source-span-offset"
        },
        {
          "name": "acceptSourceSpanInterval",
          "id": "translation.accept-source-span-interval"
        }
      ],
      "rust/src/typed_program_links/source_span_rules.rs": [
        {
          "name": "accept_source_span_offset",
          "id": "translation.accept-source-span-offset"
        },
        {
          "name": "accept_source_span_interval",
          "id": "translation.accept-source-span-interval"
        }
      ],
      "js/src/source-spans.js": [
        {
          "name": "acceptSourceSpanRecord",
          "id": "translation.accept-source-span-record"
        },
        {
          "name": "validateSourceSpans",
          "id": "translation.validate-source-spans"
        }
      ],
      "rust/src/typed_program_links/source_spans.rs": [
        {
          "name": "read_source_span_offset",
          "id": "translation.read-source-span-offset"
        },
        {
          "name": "normalize_source_spans",
          "id": "translation.normalize-source-spans"
        }
      ]
    }
  },
  {
    "inventory": "shared source span numerical policy",
    "files": {
      "js/src/source-spans.js": [
        {
          "name": "SOURCE_SPAN_POLICY",
          "id": "translation.source-span-offset-policy"
        }
      ]
    }
  },
  {
    "inventory": "shared source span fixture generators",
    "files": {
      "js/scripts/generate-source-span-fixtures.mjs": [
        {
          "name": "sourceSpanNumberArchive",
          "id": "repository.create-source-span-number-archive"
        },
        {
          "name": "sourceSpanNumberFixture",
          "id": "repository.create-source-span-number-fixture"
        }
      ]
    }
  },
  {
    "inventory": "native Rust source factory locations",
    "files": {
      "rust/src/translation/rust/source_profiles.rs": [
        {
          "name": "parse_rust",
          "id": "translation.parse-rust-source-models"
        },
        {
          "name": "parse_rust_with_array_result_model",
          "id": "translation.parse-rust-source-models"
        },
        {
          "name": "parse_rust_with_completion_model",
          "id": "translation.parse-rust-source-models"
        },
        {
          "name": "parse_rust_with_source_models",
          "id": "translation.parse-rust-source-models"
        },
        {
          "name": "parse_rust_with_source_signature_model",
          "id": "translation.parse-rust-source-models"
        },
        {
          "name": "parse_rust_with_source_profiles",
          "id": "translation.parse-rust-source-models"
        },
        {
          "name": "parse_rust_with_scalar_loop_model",
          "id": "translation.parse-rust-source-models"
        },
        {
          "name": "parse_rust_with_primitive_owner_model",
          "id": "translation.parse-rust-source-models"
        },
        {
          "name": "parse_rust_with_loop_profiles",
          "id": "translation.parse-rust-source-models"
        },
        {
          "name": "parse_rust_with_empty_option_source_model",
          "id": "translation.parse-rust-source-models"
        },
        {
          "name": "parse_rust_with_type_namespace_model",
          "id": "translation.parse-rust-source-models"
        },
        {
          "name": "parse_rust_with_pure_unit_call_model",
          "id": "translation.parse-rust-source-models"
        }
      ],
      "rust/src/translation/rust.rs": [
        {
          "name": "parse_rust_with_owned_local_state_model",
          "id": "translation.parse-rust-source-models"
        }
      ]
    }
  },
  {
    "inventory": "native Rust typed source factory locations",
    "files": {
      "rust/src/typed_program_links.rs": [
        {
          "name": "create_typed_program_links_with_rust_owned_local_state_model",
          "id": "translation.create-typed-program-links"
        },
        {
          "name": "create_typed_program_links_with_rust_pure_unit_call_model",
          "id": "translation.create-typed-program-links"
        }
      ]
    }
  },
  {
    "inventory": "extracted source import and external entities",
    "files": {
      "rust/src/translation/surface/externals.rs": [
        {
          "name": "SImport",
          "id": "translation.source-module-import"
        },
        {
          "name": "SImportName",
          "id": "translation.source-import-name"
        },
        {
          "name": "External",
          "id": "translation.source-external-item"
        },
        {
          "name": "ExternalParam",
          "id": "translation.source-external-parameter"
        }
      ]
    }
  },
  {
    "inventory": "extracted source external operations",
    "files": {
      "rust/src/translation/surface/externals.rs": [
        {
          "name": "name",
          "id": "translation.read-source-external-item-name"
        },
        {
          "name": "signature",
          "id": "translation.read-source-external-item-signature"
        },
        {
          "name": "renamed",
          "id": "translation.rename-source-external-item"
        }
      ]
    }
  }
];
const normalizeValueSourceName = name => name.replace(/([a-z])([A-Z])/gu, '$1-$2').replaceAll('_', '-').toLowerCase().replace(/^simport(?=-|$)/u,'source-import').replace(/^external-param$/u,'external-parameter');
const VALUE_SOURCE_NAME_INVENTORIES = VALUE_SOURCE_NAME_BINDINGS.map(({inventory,files})=>({
 inventory, files:Object.keys(files),
 extract:(text,file)=>files[file].filter(({name})=>new RegExp('\\b'+name+'\\b','u').test(text)).map(({name})=>normalizeValueSourceName(name)),
 recordOf:name=>Object.values(files).flat().find(binding=>normalizeValueSourceName(binding.name)===name).id,
}));
// Additive source bindings for source qualification, numeric aliases and text bytes.
const SOURCE_PROCESSING_NAME_BINDINGS = [
  {
    "inventory": "owned Rust constant predicate decisions",
    "files": {
      "js/src/translation/rust-const-function-rules.js": [
        {
          "name": "acceptRustConstFunctionModel",
          "id": "translation.accept-rust-constant-function-model"
        },
        {
          "name": "acceptRustConstFunctionType",
          "id": "translation.accept-rust-constant-function-type"
        },
        {
          "name": "acceptRustConstFunctionBinary",
          "id": "translation.accept-rust-constant-function-binary"
        },
        {
          "name": "acceptRustConstFunctionContainer",
          "id": "translation.accept-rust-constant-function-container"
        },
        {
          "name": "acceptRustConstFunctionOffset",
          "id": "translation.accept-rust-constant-function-offset"
        },
        {
          "name": "acceptRustConstFunctionInterval",
          "id": "translation.accept-rust-constant-function-interval"
        },
        {
          "name": "acceptRustConstFunctionNameCharacter",
          "id": "translation.accept-rust-constant-function-name-character"
        },
        {
          "name": "acceptRustConstFunctionName",
          "id": "translation.accept-rust-constant-function-name"
        },
        {
          "name": "acceptRustConstFunctionFiniteLiteral",
          "id": "translation.accept-rust-constant-function-finite-literal"
        }
      ],
      "rust/src/translation/rust/const_function_rules.rs": [
        {
          "name": "accept_rust_const_function_model",
          "id": "translation.accept-rust-constant-function-model"
        },
        {
          "name": "accept_rust_const_function_type",
          "id": "translation.accept-rust-constant-function-type"
        },
        {
          "name": "accept_rust_const_function_binary",
          "id": "translation.accept-rust-constant-function-binary"
        },
        {
          "name": "accept_rust_const_function_container",
          "id": "translation.accept-rust-constant-function-container"
        },
        {
          "name": "accept_rust_const_function_offset",
          "id": "translation.accept-rust-constant-function-offset"
        },
        {
          "name": "accept_rust_const_function_interval",
          "id": "translation.accept-rust-constant-function-interval"
        },
        {
          "name": "accept_rust_const_function_name_character",
          "id": "translation.accept-rust-constant-function-name-character"
        },
        {
          "name": "accept_rust_const_function_name",
          "id": "translation.accept-rust-constant-function-name"
        },
        {
          "name": "accept_rust_const_function_finite_literal",
          "id": "translation.accept-rust-constant-function-finite-literal"
        }
      ]
    }
  },
  {
    "inventory": "owned Rust constant predicate source operations",
    "files": {
      "js/src/translation/rust-const-functions.js": [
        {
          "name": "checkRustConstFunctionContainers",
          "id": "translation.check-rust-constant-function-containers"
        },
        {
          "name": "createRustConstFunctionProof",
          "id": "translation.create-rust-constant-function-proof"
        },
        {
          "name": "checkRustConstFunctionProof",
          "id": "translation.check-rust-constant-function-proof"
        },
        {
          "name": "renderRustConstFunctionHeader",
          "id": "translation.render-rust-constant-function-header"
        }
      ],
      "rust/src/translation/rust/const_functions.rs": [
        {
          "name": "create_const_function_proof",
          "id": "translation.create-rust-constant-function-proof"
        },
        {
          "name": "render_const_function_header",
          "id": "translation.render-rust-constant-function-header"
        },
        {
          "name": "const_function_visibility_guard",
          "id": "translation.require-rust-constant-function-visibility"
        },
        {
          "name": "const_function_item_guard",
          "id": "translation.require-rust-constant-function-item"
        },
        {
          "name": "require_const_function_program",
          "id": "translation.require-rust-constant-function-program"
        }
      ],
      "rust/src/translation/source_const_functions.rs": [
        {
          "name": "validate_const_function_records",
          "id": "translation.check-rust-constant-function-proof"
        },
        {
          "name": "validate_const_functions",
          "id": "translation.check-rust-constant-function-proof"
        },
        {
          "name": "policy",
          "id": "translation.read-rust-constant-function-policy"
        },
        {
          "name": "validate_interface",
          "id": "translation.validate-rust-constant-function-interface"
        },
        {
          "name": "validate_declaration",
          "id": "translation.validate-rust-constant-function-declaration"
        }
      ],
      "rust/src/translation/source_const_functions/expression.rs": [
        {
          "name": "validate",
          "id": "translation.validate-rust-constant-function-expression"
        }
      ],
      "rust/src/typed_program_links/codec_const_functions.rs": [
        {
          "name": "validate_order",
          "id": "translation.validate-constant-function-field-order"
        },
        {
          "name": "prepare_order",
          "id": "translation.prepare-constant-function-field-order"
        }
      ]
    }
  },
  {
    "inventory": "owned Rust constant predicate policy",
    "files": {
      "js/src/translation/rust-const-functions.js": [
        {
          "name": "RUST_CONST_FUNCTION_POLICY",
          "id": "translation.rust-constant-function-policy"
        }
      ]
    }
  },
  {
    "inventory": "owned Rust constant predicate model options",
    "files": {
      "js/src/translation/rust.js": [
        {
          "name": "constFunctionModel",
          "id": "translation.rust-constant-function-model-option"
        }
      ],
      "rust/src/translation/rust.rs": [
        {
          "name": "const_function_model",
          "id": "translation.rust-constant-function-model-option"
        },
        {
          "name": "const_function",
          "id": "translation.rust-constant-function-model-option"
        }
      ],
      "js/src/typed-program-links.js": [
        {
          "name": "rustConstFunctionModel",
          "id": "translation.typed-program-rust-constant-function-model-option"
        }
      ],
      "js/src/index.d.ts": [
        {
          "name": "rustConstFunctionModel",
          "id": "translation.typed-program-rust-constant-function-model-option"
        }
      ]
    }
  },
  {
    "inventory": "owned source qualification relation roles",
    "files": {
      "js/src/translation/rust-const-functions.js": [
        {
          "name": "sourceConstFunction",
          "id": "translation.function-source-constant-qualification-proof-field"
        },
        {
          "name": "keyword",
          "id": "translation.source-keyword-field"
        },
        {
          "name": "keywordSpan",
          "id": "translation.source-keyword-span-field"
        },
        {
          "name": "functionKeywordSpan",
          "id": "translation.source-function-keyword-span-field"
        },
        {
          "name": "headerSpan",
          "id": "translation.function-header-source-span-field"
        },
        {
          "name": "bodySpan",
          "id": "translation.function-body-source-span-field"
        },
        {
          "name": "sourceName",
          "id": "translation.source-name-field"
        }
      ],
      "rust/src/translation/ir/declarations.rs": [
        {
          "name": "source_const_function",
          "id": "translation.function-source-constant-qualification-proof-field"
        }
      ],
      "rust/src/translation/surface.rs": [
        {
          "name": "source_const_function",
          "id": "translation.function-source-constant-qualification-proof-field"
        }
      ],
      "js/src/typed-program-links.js": [
        {
          "name": "rustConstFunctionModel",
          "id": "translation.program-rust-constant-function-model-field"
        }
      ]
    }
  },
  {
    "inventory": "constant predicate fixture and rule authoring",
    "files": {
      "experiments/generate-rust-const-function-rules.mjs": [
        {
          "name": "rustConstFunctionPolicy",
          "id": "translation.rust-constant-function-policy"
        },
        {
          "name": "rustConstFunctionRules",
          "id": "repository.generate-rust-constant-function-rules"
        },
        {
          "name": "rustConstFunctionDecorators",
          "id": "repository.generate-rust-constant-function-decorators"
        },
        {
          "name": "generateNativeRules",
          "id": "repository.generate-native-constant-function-rules"
        }
      ],
      "experiments/generate-rust-const-function-fixture.mjs": [
        {
          "name": "pinnedRustConstPredicateDefinitions",
          "id": "repository.read-fixed-rust-constant-predicate-definitions"
        },
        {
          "name": "rustConstFunctionFixture",
          "id": "repository.create-rust-constant-function-fixture"
        }
      ]
    }
  },
  {
    "inventory": "typed integral metadata operations",
    "files": {
      "js/src/program-numeric-metadata-rules.js": [
        {
          "name": "acceptProgramIntegralMetadata",
          "id": "translation.accept-program-integral-metadata"
        },
        {
          "name": "acceptProgramOrderedPosition",
          "id": "translation.accept-program-ordered-position"
        }
      ],
      "rust/src/typed_program_links/program_numeric_metadata_rules.rs": [
        {
          "name": "accept_program_integral_metadata",
          "id": "translation.accept-program-integral-metadata"
        },
        {
          "name": "accept_program_ordered_position",
          "id": "translation.accept-program-ordered-position"
        }
      ],
      "js/src/translation/rust-owned-local-state.js": [
        {
          "name": "matchRustOwnedLocalOccurrencePath",
          "id": "translation.match-rust-owned-local-occurrence-path"
        }
      ],
      "rust/src/translation/owned_local_state.rs": [
        {
          "name": "read_owned_local_position",
          "id": "translation.read-owned-local-position"
        }
      ],
      "rust/src/typed_program_links/program_numeric_metadata.rs": [
        {
          "name": "normalize_program_integral_metadata_record",
          "id": "translation.normalize-program-integral-metadata-record"
        },
        {
          "name": "normalize_program_integral_metadata",
          "id": "translation.normalize-program-integral-metadata"
        }
      ],
      "js/src/translation/rust-source-signatures.js": [
        {
          "name": "acceptProgramIntegralMetadata",
          "id": "translation.accept-program-integral-metadata"
        }
      ]
    }
  },
  {
    "inventory": "integral metadata fixture authoring",
    "files": {
      "experiments/generate-program-numeric-metadata-fixture.mjs": [
        {
          "name": "numericMetadataArchive",
          "id": "repository.create-integral-metadata-archive"
        },
        {
          "name": "numericMetadataFixture",
          "id": "repository.create-integral-metadata-fixture"
        }
      ],
      "experiments/generate-owned-occurrence-position-fixture.mjs": [
        {
          "name": "ownedOccurrencePositionArchive",
          "id": "repository.create-owned-occurrence-position-archive"
        },
        {
          "name": "ownedOccurrencePositionFixture",
          "id": "repository.create-owned-occurrence-position-fixture"
        }
      ]
    }
  },
  {
    "inventory": "source text byte unit operations",
    "files": {
      "js/src/translation/source-text-byte-rules.js": [
        {
          "name": "readSourceTextScalar",
          "id": "translation.read-source-text-scalar"
        },
        {
          "name": "readSourceTextScalarByteWidth",
          "id": "translation.read-source-text-scalar-byte-width"
        },
        {
          "name": "readSourceTextByteLength",
          "id": "translation.read-source-text-unit-byte-length"
        },
        {
          "name": "acceptSourceTextScalarUnits",
          "id": "translation.accept-source-text-scalar-units"
        },
        {
          "name": "readSourceTextCodeUnitOffset",
          "id": "translation.read-source-text-code-unit-offset"
        },
        {
          "name": "acceptSourceTextByteSlice",
          "id": "translation.accept-source-text-byte-slice"
        },
        {
          "name": "readSourceTextByte",
          "id": "translation.read-source-text-unit-byte"
        },
        {
          "name": "acceptSourceTextUnitCount",
          "id": "translation.accept-source-text-unit-count"
        }
      ],
      "rust/src/translation/source_text_byte_rules.rs": [
        {
          "name": "read_source_text_scalar",
          "id": "translation.read-source-text-scalar"
        },
        {
          "name": "read_source_text_scalar_byte_width",
          "id": "translation.read-source-text-scalar-byte-width"
        },
        {
          "name": "read_source_text_byte_length",
          "id": "translation.read-source-text-unit-byte-length"
        },
        {
          "name": "accept_source_text_scalar_units",
          "id": "translation.accept-source-text-scalar-units"
        },
        {
          "name": "read_source_text_code_unit_offset",
          "id": "translation.read-source-text-code-unit-offset"
        },
        {
          "name": "accept_source_text_byte_slice",
          "id": "translation.accept-source-text-byte-slice"
        },
        {
          "name": "read_source_text_byte",
          "id": "translation.read-source-text-unit-byte"
        },
        {
          "name": "accept_source_text_unit_count",
          "id": "translation.accept-source-text-unit-count"
        },
        {
          "name": "source_text_integer",
          "id": "translation.check-source-text-integral-number"
        }
      ],
      "experiments/source-text-byte-rules.mjs": [
        {
          "name": "readSourceTextScalar",
          "id": "translation.read-source-text-scalar"
        },
        {
          "name": "readSourceTextScalarByteWidth",
          "id": "translation.read-source-text-scalar-byte-width"
        },
        {
          "name": "readSourceTextByteLength",
          "id": "translation.read-source-text-unit-byte-length"
        },
        {
          "name": "acceptSourceTextScalarUnits",
          "id": "translation.accept-source-text-scalar-units"
        },
        {
          "name": "readSourceTextCodeUnitOffset",
          "id": "translation.read-source-text-code-unit-offset"
        },
        {
          "name": "acceptSourceTextByteSlice",
          "id": "translation.accept-source-text-byte-slice"
        },
        {
          "name": "readSourceTextByte",
          "id": "translation.read-source-text-unit-byte"
        }
      ]
    }
  },
  {
    "inventory": "primitive source text byte operations",
    "files": {
      "js/src/translation/source-text-bytes.js": [
        {
          "name": "readSourceTextUnits",
          "id": "translation.read-source-text-units"
        },
        {
          "name": "readSourceTextByteLength",
          "id": "translation.read-source-text-byte-length"
        },
        {
          "name": "readSourceTextByte",
          "id": "translation.read-source-text-byte"
        },
        {
          "name": "sliceSourceTextBytes",
          "id": "translation.slice-source-text-bytes"
        }
      ],
      "rust/src/translation/source_text_bytes.rs": [
        {
          "name": "read_source_text_units",
          "id": "translation.read-source-text-units"
        },
        {
          "name": "read_source_text_byte_length",
          "id": "translation.read-source-text-byte-length"
        },
        {
          "name": "read_source_text_byte",
          "id": "translation.read-source-text-byte"
        },
        {
          "name": "slice_source_text_bytes",
          "id": "translation.slice-source-text-bytes"
        }
      ],
      "js/src/index.js": [
        {
          "name": "readSourceTextUnits",
          "id": "translation.read-source-text-units"
        },
        {
          "name": "readSourceTextByteLength",
          "id": "translation.read-source-text-byte-length"
        },
        {
          "name": "readSourceTextByte",
          "id": "translation.read-source-text-byte"
        },
        {
          "name": "sliceSourceTextBytes",
          "id": "translation.slice-source-text-bytes"
        }
      ],
      "js/src/index.d.ts": [
        {
          "name": "readSourceTextUnits",
          "id": "translation.read-source-text-units"
        },
        {
          "name": "readSourceTextByteLength",
          "id": "translation.read-source-text-byte-length"
        },
        {
          "name": "readSourceTextByte",
          "id": "translation.read-source-text-byte"
        },
        {
          "name": "sliceSourceTextBytes",
          "id": "translation.slice-source-text-bytes"
        }
      ],
      "rust/src/lib.rs": [
        {
          "name": "read_source_text_units",
          "id": "translation.read-source-text-units"
        },
        {
          "name": "read_source_text_byte_length",
          "id": "translation.read-source-text-byte-length"
        },
        {
          "name": "read_source_text_byte",
          "id": "translation.read-source-text-byte"
        },
        {
          "name": "slice_source_text_bytes",
          "id": "translation.slice-source-text-bytes"
        }
      ]
    }
  },
  {
    "inventory": "source text byte authoring operations",
    "files": {
      "experiments/generate-source-text-byte-rules.mjs": [
        {
          "name": "sourceTextByteRulesSource",
          "id": "repository.generate-source-text-byte-rules"
        }
      ],
      "experiments/complete-binary-formatter-probe.mjs": [
        {
          "name": "completeBinaryFormatterProbe",
          "id": "repository.probe-complete-binary-format-source"
        }
      ],
      "js/scripts/source-text-byte-library-style.mjs": [
        {
          "name": "readSourceTextRustBlock",
          "id": "repository.read-source-text-native-source-block"
        },
        {
          "name": "readSourceTextByteLibraryStyles",
          "id": "repository.read-source-text-byte-library-styles"
        },
        {
          "name": "syntax",
          "id": "repository.read-source-text-style-token-pairs"
        },
        {
          "name": "parentheses",
          "id": "repository.remove-source-text-style-parentheses"
        }
      ]
    }
  },
  {
    "inventory": "pinned associative provider audit operations",
    "files": {
      "js/scripts/associative-stack-sources.mjs": [
        {
          "name": "checkAssociativeStackProviderRoles",
          "id": "foundation.check-associative-stack-provider-roles"
        }
      ]
    }
  },
  {
    "inventory": "pinned associative audit source mapping",
    "files": {
      "js/scripts/issue-195-conversation-requirements.mjs": [
        {
          "name": "sourceTooling",
          "id": "repository.fixed-associative-audit-source-map"
        }
      ]
    }
  },
  {
    "inventory": "reviewed compiler diagnostic test receipt",
    "files": {
      "js/tests/translation-bulk-twenty-ci-feedback.test.js": [
        {
          "name": "receipt",
          "id": "repository.reviewed-compiler-diagnostic-receipt"
        }
      ]
    }
  },
  {
    "inventory": "constant predicate public consumer operations",
    "files": {
      "js/tests/translation-const-public-declaration-contracts.test.js": [
        {
          "name": "runConsumer",
          "id": "repository.run-constant-predicate-public-consumer"
        },
        {
          "name": "observe",
          "id": "repository.observe-constant-predicate-public-values"
        }
      ]
    }
  },
  {
    "inventory": "source text byte public consumer operations",
    "files": {
      "js/tests/translation-source-text-byte-public-declaration-contracts.test.js": [
        {
          "name": "runConsumer",
          "id": "repository.run-source-text-byte-public-consumer"
        }
      ]
    }
  },
  {
    "inventory": "extracted source function and documentation readers",
    "files": {
      "rust/src/translation/rust/items/functions.rs": [
        {
          "name": "function",
          "id": "translation.read-rust-source-function"
        },
        {
          "name": "function_with_lifetimes",
          "id": "translation.read-rust-source-function-with-lifetimes"
        }
      ],
      "rust/src/translation/javascript/declarations/jsdoc.rs": [
        {
          "name": "promised",
          "id": "translation.read-javascript-promised-type"
        },
        {
          "name": "jsdoc_type",
          "id": "translation.read-javascript-documentation-type"
        }
      ]
    }
  },
  {
    "inventory": "native constant source factory locations",
    "files": {
      "rust/src/translation/rust.rs": [
        {
          "name": "parse_rust_with_const_function_model",
          "id": "translation.parse-rust-source-models"
        }
      ],
      "rust/src/typed_program_links.rs": [
        {
          "name": "create_typed_program_links_with_rust_const_function_model",
          "id": "translation.create-typed-program-links"
        }
      ]
    }
  }
];
const normalizeSourceProcessingName=name=>name.replace(/([a-z])([A-Z])/gu,'$1-$2').replaceAll('_','-').toLowerCase().replace(/\bconst\b/gu,'constant').replace(/\bjsdoc\b/gu,'javascript-documentation').replace(/\bpinned\b/gu,'fixed').replace(/\bformatter\b/gu,'format');
const SOURCE_PROCESSING_NAME_INVENTORIES=SOURCE_PROCESSING_NAME_BINDINGS.map(({inventory,files})=>({
 inventory,files:Object.keys(files),
 extract:(text,file)=>files[file].filter(({name})=>new RegExp('\\b'+name+'\\b','u').test(text)).map(({name})=>normalizeSourceProcessingName(name)),
 recordOf:name=>Object.values(files).flat().find(entry=>normalizeSourceProcessingName(entry.name)===name).id,
}));
// Additive exact source bindings for the complete closed Rust runtime graph.
const RUNTIME_SHELL_NAME_BINDINGS = [
  {
    "inventory": "closed Rust runtime shell generated decisions",
    "files": {
      "js/src/translation/rust-runtime-shell-rules.js": [
        {
          "name": "acceptRustRuntimeShellModel",
          "id": "translation.accept-rust-runtime-shell-model"
        },
        {
          "name": "acceptRustRuntimeShellReservedName",
          "id": "translation.accept-rust-runtime-shell-reserved-name"
        },
        {
          "name": "acceptRustRuntimeShellStackShift",
          "id": "translation.accept-rust-runtime-shell-stack-shift"
        }
      ],
      "rust/src/translation/runtime_shell_rules.rs": [
        {
          "name": "accept_rust_runtime_shell_model",
          "id": "translation.accept-rust-runtime-shell-model"
        },
        {
          "name": "accept_rust_runtime_shell_reserved_name",
          "id": "translation.accept-rust-runtime-shell-reserved-name"
        },
        {
          "name": "accept_rust_runtime_shell_stack_shift",
          "id": "translation.accept-rust-runtime-shell-stack-shift"
        }
      ]
    }
  },
  {
    "inventory": "closed Rust runtime shell operations",
    "files": {
      "js/src/translation/rust-runtime-shell.js": [
        {
          "name": "checkRustRuntimeShellContainers",
          "id": "translation.check-rust-runtime-shell-containers"
        },
        {
          "name": "checkRustRuntimeShell",
          "id": "translation.check-rust-runtime-shell"
        },
        {
          "name": "captureRustRuntimeShell",
          "id": "translation.capture-rust-runtime-shell"
        },
        {
          "name": "readRustRuntimeShellSource",
          "id": "translation.parse-rust-runtime-shell-source"
        },
        {
          "name": "recordRustRuntimeShellAssumptions",
          "id": "translation.record-rust-runtime-shell-assumptions"
        },
        {
          "name": "emitRustRuntimeShell",
          "id": "translation.emit-rust-runtime-shell"
        },
        {
          "name": "refuseRustRuntimeShellComposition",
          "id": "translation.refuse-rust-runtime-shell-composition"
        },
        {
          "name": "data",
          "id": "translation.check-rust-runtime-shell-data"
        },
        {
          "name": "fields",
          "id": "translation.check-rust-runtime-shell-record-fields"
        },
        {
          "name": "interval",
          "id": "translation.check-rust-runtime-shell-source-interval"
        },
        {
          "name": "compare",
          "id": "translation.compare-rust-runtime-shell-data"
        },
        {
          "name": "assumptions",
          "id": "translation.create-rust-runtime-shell-operation-assumptions"
        },
        {
          "name": "tokenValue",
          "id": "translation.read-rust-runtime-shell-token-name"
        },
        {
          "name": "validateTokens",
          "id": "translation.check-rust-runtime-shell-tokens"
        },
        {
          "name": "compatibleCore",
          "id": "translation.check-rust-runtime-shell-core"
        },
        {
          "name": "renderTokens",
          "id": "translation.render-rust-runtime-shell-tokens"
        }
      ],
      "rust/src/translation/runtime_shell.rs": [
        {
          "name": "validate_records",
          "id": "translation.check-rust-runtime-shell"
        },
        {
          "name": "validate_program",
          "id": "translation.check-rust-runtime-shell"
        },
        {
          "name": "capture_program",
          "id": "translation.capture-rust-runtime-shell"
        },
        {
          "name": "parse_source",
          "id": "translation.parse-rust-runtime-shell-source"
        },
        {
          "name": "bounded",
          "id": "translation.check-rust-runtime-shell-data"
        },
        {
          "name": "fields",
          "id": "translation.check-rust-runtime-shell-record-fields"
        },
        {
          "name": "source_interval",
          "id": "translation.check-rust-runtime-shell-source-interval"
        },
        {
          "name": "assumptions",
          "id": "translation.create-rust-runtime-shell-operation-assumptions"
        },
        {
          "name": "tokens",
          "id": "translation.check-rust-runtime-shell-tokens"
        },
        {
          "name": "policy",
          "id": "translation.read-rust-runtime-shell-policy"
        },
        {
          "name": "fail",
          "id": "translation.create-rust-runtime-shell-proof-error"
        },
        {
          "name": "position",
          "id": "translation.read-rust-runtime-shell-position"
        },
        {
          "name": "role",
          "id": "translation.check-rust-runtime-shell-policy-fields"
        },
        {
          "name": "identifier",
          "id": "translation.accept-rust-runtime-shell-identifier"
        },
        {
          "name": "operation_path",
          "id": "translation.find-rust-runtime-shell-operation-path"
        },
        {
          "name": "normalize_core",
          "id": "translation.normalize-rust-runtime-shell-core-view"
        }
      ],
      "js/src/translation/rust-runtime-shell-parser.js": [
        {
          "name": "parseRustRuntimeShell",
          "id": "translation.parse-rust-runtime-shell-source"
        },
        {
          "name": "readRustRuntimeShellSyntax",
          "id": "translation.read-rust-runtime-shell-syntax"
        },
        {
          "name": "ordinary",
          "id": "translation.accept-rust-runtime-shell-name"
        },
        {
          "name": "take",
          "id": "translation.take-rust-runtime-shell-token"
        },
        {
          "name": "literal",
          "id": "translation.read-rust-runtime-shell-literal"
        },
        {
          "name": "bind",
          "id": "translation.bind-rust-runtime-shell-name"
        },
        {
          "name": "reference",
          "id": "translation.read-rust-runtime-shell-binding-reference"
        },
        {
          "name": "helperReference",
          "id": "translation.read-rust-runtime-shell-helper-reference"
        },
        {
          "name": "match",
          "id": "translation.match-rust-runtime-shell-pattern"
        },
        {
          "name": "nodes",
          "id": "translation.find-rust-runtime-shell-operation-nodes"
        },
        {
          "name": "single",
          "id": "translation.require-rust-runtime-shell-single-operation"
        },
        {
          "name": "range",
          "id": "translation.read-rust-runtime-shell-source-range"
        }
      ],
      "rust/src/translation/runtime_shell/parser.rs": [
        {
          "name": "read_syntax",
          "id": "translation.read-rust-runtime-shell-syntax"
        },
        {
          "name": "token_name",
          "id": "translation.read-rust-runtime-shell-token-name"
        },
        {
          "name": "ordinary",
          "id": "translation.accept-rust-runtime-shell-name"
        },
        {
          "name": "take",
          "id": "translation.take-rust-runtime-shell-token"
        },
        {
          "name": "match_pattern",
          "id": "translation.match-rust-runtime-shell-pattern"
        },
        {
          "name": "nodes",
          "id": "translation.find-rust-runtime-shell-operation-nodes"
        },
        {
          "name": "single",
          "id": "translation.require-rust-runtime-shell-single-operation"
        },
        {
          "name": "range",
          "id": "translation.read-rust-runtime-shell-source-range"
        },
        {
          "name": "source_fail",
          "id": "translation.create-rust-runtime-shell-source-error"
        },
        {
          "name": "token",
          "id": "translation.read-rust-runtime-shell-referenced-token"
        }
      ],
      "rust/src/translation/runtime_shell/emission.rs": [
        {
          "name": "record_assumptions",
          "id": "translation.record-rust-runtime-shell-assumptions"
        },
        {
          "name": "emit_rust",
          "id": "translation.emit-rust-runtime-shell"
        },
        {
          "name": "render",
          "id": "translation.render-rust-runtime-shell-tokens"
        }
      ],
      "rust/src/typed_program_links/codec_runtime_shell.rs": [
        {
          "name": "reference_path",
          "id": "translation.read-rust-runtime-shell-reference-path"
        },
        {
          "name": "operation",
          "id": "translation.project-rust-runtime-shell-operation-graph"
        },
        {
          "name": "normalize",
          "id": "translation.normalize-rust-runtime-shell-reference-graph"
        }
      ]
    }
  },
  {
    "inventory": "closed Rust runtime shell emission assumptions",
    "files": {
      "rust/src/translation/runtime_shell/emission.rs": [
        {
          "name": "assumptions",
          "id": "translation.create-rust-runtime-shell-emission-assumptions"
        }
      ]
    }
  },
  {
    "inventory": "closed Rust runtime shell recursive data visitor",
    "files": {
      "js/src/translation/rust-runtime-shell.js": [
        {
          "name": "visit",
          "id": "translation.walk-rust-runtime-shell-owned-data"
        }
      ]
    }
  },
  {
    "inventory": "closed Rust runtime shell recursive operation visitor",
    "files": {
      "rust/src/translation/runtime_shell.rs": [
        {
          "name": "visit",
          "id": "translation.walk-rust-runtime-shell-operation-paths"
        }
      ]
    }
  },
  {
    "inventory": "closed Rust runtime shell recursive rendering visitor",
    "files": {
      "rust/src/translation/runtime_shell/emission.rs": [
        {
          "name": "collect",
          "id": "translation.collect-rust-runtime-shell-render-token-positions"
        }
      ]
    }
  },
  {
    "inventory": "closed Rust runtime shell policy",
    "files": {
      "js/src/translation/rust-runtime-shell.js": [
        {
          "name": "RUST_RUNTIME_SHELL_POLICY",
          "id": "translation.rust-runtime-shell-policy"
        }
      ]
    }
  },
  {
    "inventory": "closed Rust runtime shell model options",
    "files": {
      "js/src/translation/rust.js": [
        {
          "name": "runtimeShellModel",
          "id": "translation.rust-runtime-shell-model-option"
        }
      ],
      "js/src/typed-program-links.js": [
        {
          "name": "rustRuntimeShellModel",
          "id": "translation.typed-program-rust-runtime-shell-model-option"
        }
      ],
      "js/src/index.d.ts": [
        {
          "name": "rustRuntimeShellModel",
          "id": "translation.typed-program-rust-runtime-shell-model-option"
        }
      ]
    }
  },
  {
    "inventory": "closed Rust runtime shell source facets",
    "files": {
      "js/src/translation/rust-runtime-shell.js": [
        {
          "name": "sourceRuntimeShell",
          "id": "translation.program-source-runtime-shell-field"
        },
        {
          "name": "sourceRuntimeWorker",
          "id": "translation.main-source-runtime-worker-field"
        }
      ],
      "rust/src/translation/ir.rs": [
        {
          "name": "source_runtime_shell",
          "id": "translation.program-source-runtime-shell-field"
        },
        {
          "name": "source_runtime_worker",
          "id": "translation.main-source-runtime-worker-field"
        }
      ],
      "rust/src/translation/surface.rs": [
        {
          "name": "source_runtime_shell",
          "id": "translation.program-source-runtime-shell-field"
        },
        {
          "name": "source_runtime_worker",
          "id": "translation.main-source-runtime-worker-field"
        }
      ],
      "rust/src/translation/runtime_shell.rs": [
        {
          "name": "sourceRuntimeShell",
          "id": "translation.program-source-runtime-shell-field"
        },
        {
          "name": "sourceRuntimeWorker",
          "id": "translation.main-source-runtime-worker-field"
        }
      ]
    }
  },
  {
    "inventory": "closed Rust runtime shell graph relation roles",
    "files": {
      "js/src/translation/rust-runtime-shell.js": [
        {
          "name": "observationModel",
          "id": "translation.runtime-observation-model-field"
        },
        {
          "name": "tokens",
          "id": "translation.runtime-source-token-list-field"
        },
        {
          "name": "operations",
          "id": "translation.runtime-source-operation-root-field"
        },
        {
          "name": "operationNode",
          "id": "translation.runtime-assumption-operation-reference-field"
        },
        {
          "name": "declaration",
          "id": "translation.runtime-token-declaration-reference-field"
        },
        {
          "name": "binding",
          "id": "translation.runtime-token-binding-reference-field"
        },
        {
          "name": "alias",
          "id": "translation.runtime-source-occurrence-alias-field"
        },
        {
          "name": "parts",
          "id": "translation.runtime-operation-part-list-field"
        }
      ],
      "rust/src/translation/runtime_shell.rs": [
        {
          "name": "observationModel",
          "id": "translation.runtime-observation-model-field"
        },
        {
          "name": "tokens",
          "id": "translation.runtime-source-token-list-field"
        },
        {
          "name": "operations",
          "id": "translation.runtime-source-operation-root-field"
        },
        {
          "name": "operationNode",
          "id": "translation.runtime-assumption-operation-reference-field"
        },
        {
          "name": "declaration",
          "id": "translation.runtime-token-declaration-reference-field"
        },
        {
          "name": "binding",
          "id": "translation.runtime-token-binding-reference-field"
        },
        {
          "name": "alias",
          "id": "translation.runtime-source-occurrence-alias-field"
        },
        {
          "name": "parts",
          "id": "translation.runtime-operation-part-list-field"
        }
      ],
      "rust/src/typed_program_links/codec_runtime_shell.rs": [
        {
          "name": "tokens",
          "id": "translation.runtime-source-token-list-field"
        },
        {
          "name": "operations",
          "id": "translation.runtime-source-operation-root-field"
        },
        {
          "name": "operationNode",
          "id": "translation.runtime-assumption-operation-reference-field"
        },
        {
          "name": "declaration",
          "id": "translation.runtime-token-declaration-reference-field"
        },
        {
          "name": "binding",
          "id": "translation.runtime-token-binding-reference-field"
        },
        {
          "name": "parts",
          "id": "translation.runtime-operation-part-list-field"
        }
      ]
    }
  },
  {
    "inventory": "closed Rust runtime shell private graph adapter roles",
    "files": {
      "rust/src/typed_program_links/codec_runtime_shell.rs": [
        {
          "name": "declarationPosition",
          "id": "translation.runtime-private-declaration-position-field"
        },
        {
          "name": "tokenPosition",
          "id": "translation.runtime-private-token-position-field"
        },
        {
          "name": "bindingPosition",
          "id": "translation.runtime-private-binding-position-field"
        },
        {
          "name": "operationPosition",
          "id": "translation.runtime-private-operation-position-field"
        }
      ],
      "rust/src/translation/runtime_shell.rs": [
        {
          "name": "declarationPosition",
          "id": "translation.runtime-private-declaration-position-field"
        },
        {
          "name": "bindingPosition",
          "id": "translation.runtime-private-binding-position-field"
        },
        {
          "name": "operationPosition",
          "id": "translation.runtime-private-operation-position-field"
        }
      ],
      "rust/src/translation/runtime_shell/parser.rs": [
        {
          "name": "declarationPosition",
          "id": "translation.runtime-private-declaration-position-field"
        },
        {
          "name": "tokenPosition",
          "id": "translation.runtime-private-token-position-field"
        },
        {
          "name": "bindingPosition",
          "id": "translation.runtime-private-binding-position-field"
        }
      ],
      "rust/src/translation/runtime_shell/emission.rs": [
        {
          "name": "tokenPosition",
          "id": "translation.runtime-private-token-position-field"
        }
      ]
    }
  },
  {
    "inventory": "closed Rust runtime shell parser entities",
    "files": {
      "rust/src/translation/runtime_shell/parser.rs": [
        {
          "name": "Helper",
          "id": "translation.rust-runtime-shell-owned-helper"
        },
        {
          "name": "Parsed",
          "id": "translation.rust-runtime-shell-parsed-graph"
        },
        {
          "name": "Cursor",
          "id": "translation.rust-runtime-shell-token-cursor"
        }
      ]
    }
  },
  {
    "inventory": "closed Rust runtime shell codec entities",
    "files": {
      "rust/src/typed_program_links/codec_runtime_shell.rs": [
        {
          "name": "Graph",
          "id": "translation.rust-runtime-shell-owned-graph-decoder"
        }
      ]
    }
  },
  {
    "inventory": "closed Rust runtime shell linked operation meanings",
    "files": {
      "js/src/translation/data/rust-runtime-shell.lino": [
        {
          "name": "inert runtime lint attribute",
          "id": "translation.runtime-inert-runtime-lint-attribute"
        },
        {
          "name": "owned public Unit helper",
          "id": "translation.runtime-owned-public-unit-helper"
        },
        {
          "name": "pure Unit helper value",
          "id": "translation.runtime-pure-unit-helper-value"
        },
        {
          "name": "owned Unit call binding",
          "id": "translation.runtime-owned-unit-call-binding"
        },
        {
          "name": "owned Unit invocation",
          "id": "translation.runtime-owned-unit-invocation"
        },
        {
          "name": "owned Unit worker definition",
          "id": "translation.runtime-owned-unit-worker-definition"
        },
        {
          "name": "ordered owned Unit worker calls",
          "id": "translation.runtime-ordered-owned-unit-worker-calls"
        },
        {
          "name": "complete standard runtime entry",
          "id": "translation.runtime-complete-standard-runtime-entry"
        },
        {
          "name": "unused Boolean argument scan",
          "id": "translation.runtime-unused-boolean-argument-scan"
        },
        {
          "name": "standard argument existence test",
          "id": "translation.runtime-standard-argument-existence-test"
        },
        {
          "name": "standard argument iteration",
          "id": "translation.runtime-standard-argument-iteration"
        },
        {
          "name": "standard argument owner",
          "id": "translation.runtime-standard-argument-owner"
        },
        {
          "name": "closed argument predicate",
          "id": "translation.runtime-closed-argument-predicate"
        },
        {
          "name": "standard argument flag comparison",
          "id": "translation.runtime-standard-argument-flag-comparison"
        },
        {
          "name": "owned thread handle binding",
          "id": "translation.runtime-owned-thread-handle-binding"
        },
        {
          "name": "standard thread builder construction",
          "id": "translation.runtime-standard-thread-builder-construction"
        },
        {
          "name": "standard thread builder owner",
          "id": "translation.runtime-standard-thread-builder-owner"
        },
        {
          "name": "bounded stack size request",
          "id": "translation.runtime-bounded-stack-size-request"
        },
        {
          "name": "unsigned stack size shift",
          "id": "translation.runtime-unsigned-stack-size-shift"
        },
        {
          "name": "standard Unit thread spawn",
          "id": "translation.runtime-standard-unit-thread-spawn"
        },
        {
          "name": "closed moved Unit worker",
          "id": "translation.runtime-closed-moved-unit-worker"
        },
        {
          "name": "owned worker invocation",
          "id": "translation.runtime-owned-worker-invocation"
        },
        {
          "name": "standard thread success requirement",
          "id": "translation.runtime-standard-thread-success-requirement"
        },
        {
          "name": "retained worker failure branch",
          "id": "translation.runtime-retained-worker-failure-branch"
        },
        {
          "name": "standard worker failure predicate",
          "id": "translation.runtime-standard-worker-failure-predicate"
        },
        {
          "name": "standard worker join",
          "id": "translation.runtime-standard-worker-join"
        },
        {
          "name": "standard worker failure exit",
          "id": "translation.runtime-standard-worker-failure-exit"
        },
        {
          "name": "standard exit owner",
          "id": "translation.runtime-standard-exit-owner"
        },
        {
          "name": "complete closed Rust runtime crate",
          "id": "translation.runtime-complete-closed-rust-runtime-crate"
        },
        {
          "name": "ordered owned helper declarations",
          "id": "translation.runtime-ordered-owned-helper-declarations"
        }
      ],
      "rust/src/data/rust-runtime-shell.lino": [
        {
          "name": "inert runtime lint attribute",
          "id": "translation.runtime-inert-runtime-lint-attribute"
        },
        {
          "name": "owned public Unit helper",
          "id": "translation.runtime-owned-public-unit-helper"
        },
        {
          "name": "pure Unit helper value",
          "id": "translation.runtime-pure-unit-helper-value"
        },
        {
          "name": "owned Unit call binding",
          "id": "translation.runtime-owned-unit-call-binding"
        },
        {
          "name": "owned Unit invocation",
          "id": "translation.runtime-owned-unit-invocation"
        },
        {
          "name": "owned Unit worker definition",
          "id": "translation.runtime-owned-unit-worker-definition"
        },
        {
          "name": "ordered owned Unit worker calls",
          "id": "translation.runtime-ordered-owned-unit-worker-calls"
        },
        {
          "name": "complete standard runtime entry",
          "id": "translation.runtime-complete-standard-runtime-entry"
        },
        {
          "name": "unused Boolean argument scan",
          "id": "translation.runtime-unused-boolean-argument-scan"
        },
        {
          "name": "standard argument existence test",
          "id": "translation.runtime-standard-argument-existence-test"
        },
        {
          "name": "standard argument iteration",
          "id": "translation.runtime-standard-argument-iteration"
        },
        {
          "name": "standard argument owner",
          "id": "translation.runtime-standard-argument-owner"
        },
        {
          "name": "closed argument predicate",
          "id": "translation.runtime-closed-argument-predicate"
        },
        {
          "name": "standard argument flag comparison",
          "id": "translation.runtime-standard-argument-flag-comparison"
        },
        {
          "name": "owned thread handle binding",
          "id": "translation.runtime-owned-thread-handle-binding"
        },
        {
          "name": "standard thread builder construction",
          "id": "translation.runtime-standard-thread-builder-construction"
        },
        {
          "name": "standard thread builder owner",
          "id": "translation.runtime-standard-thread-builder-owner"
        },
        {
          "name": "bounded stack size request",
          "id": "translation.runtime-bounded-stack-size-request"
        },
        {
          "name": "unsigned stack size shift",
          "id": "translation.runtime-unsigned-stack-size-shift"
        },
        {
          "name": "standard Unit thread spawn",
          "id": "translation.runtime-standard-unit-thread-spawn"
        },
        {
          "name": "closed moved Unit worker",
          "id": "translation.runtime-closed-moved-unit-worker"
        },
        {
          "name": "owned worker invocation",
          "id": "translation.runtime-owned-worker-invocation"
        },
        {
          "name": "standard thread success requirement",
          "id": "translation.runtime-standard-thread-success-requirement"
        },
        {
          "name": "retained worker failure branch",
          "id": "translation.runtime-retained-worker-failure-branch"
        },
        {
          "name": "standard worker failure predicate",
          "id": "translation.runtime-standard-worker-failure-predicate"
        },
        {
          "name": "standard worker join",
          "id": "translation.runtime-standard-worker-join"
        },
        {
          "name": "standard worker failure exit",
          "id": "translation.runtime-standard-worker-failure-exit"
        },
        {
          "name": "standard exit owner",
          "id": "translation.runtime-standard-exit-owner"
        },
        {
          "name": "complete closed Rust runtime crate",
          "id": "translation.runtime-complete-closed-rust-runtime-crate"
        },
        {
          "name": "ordered owned helper declarations",
          "id": "translation.runtime-ordered-owned-helper-declarations"
        }
      ]
    }
  },
  {
    "inventory": "closed Rust runtime shell fixture model",
    "files": {
      "js/scripts/generate-rust-runtime-shell-fixtures.mjs": [
        {
          "name": "RUST_RUNTIME_SHELL_MODEL",
          "id": "repository.rust-runtime-shell-fixture-model"
        }
      ]
    }
  },
  {
    "inventory": "closed Rust runtime shell fixture operations",
    "files": {
      "js/scripts/generate-rust-runtime-shell-fixtures.mjs": [
        {
          "name": "rustRuntimeShellFixture",
          "id": "repository.create-rust-runtime-shell-fixture"
        }
      ]
    }
  },
  {
    "inventory": "closed Rust runtime shell test operations",
    "files": {
      "rust/tests/unit/translation_rust_runtime_shell.rs": [
        {
          "name": "fixture",
          "id": "repository.read-rust-runtime-shell-fixture"
        },
        {
          "name": "example",
          "id": "repository.create-rust-runtime-shell-example"
        },
        {
          "name": "execute_javascript",
          "id": "repository.execute-rust-runtime-shell-javascript"
        },
        {
          "name": "observe_in_javascript",
          "id": "repository.observe-rust-runtime-shell-public-packet"
        },
        {
          "name": "reject",
          "id": "repository.require-rust-runtime-shell-refusal"
        },
        {
          "name": "archive",
          "id": "repository.save-rust-runtime-shell-test-archive"
        },
        {
          "name": "checked",
          "id": "repository.read-rust-runtime-shell-checked-program"
        },
        {
          "name": "calls",
          "id": "repository.read-rust-runtime-shell-test-calls"
        },
        {
          "name": "javascript_packet",
          "id": "repository.create-rust-runtime-shell-javascript-packet"
        },
        {
          "name": "javascript_refusal",
          "id": "repository.read-rust-runtime-shell-javascript-refusal"
        },
        {
          "name": "compile_actual_artifact",
          "id": "repository.compile-rust-runtime-shell-test-artifact"
        },
        {
          "name": "field",
          "id": "repository.read-rust-runtime-shell-test-field"
        },
        {
          "name": "replace",
          "id": "repository.replace-rust-runtime-shell-test-field"
        },
        {
          "name": "scalar",
          "id": "repository.create-rust-runtime-shell-test-scalar"
        },
        {
          "name": "clone_record",
          "id": "repository.clone-rust-runtime-shell-test-record"
        },
        {
          "name": "api",
          "id": "repository.read-rust-runtime-shell-test-programming-interface-path"
        }
      ],
      "js/tests/translation-rust-runtime-shell.test.js": [
        {
          "name": "example",
          "id": "repository.create-rust-runtime-shell-example"
        },
        {
          "name": "execute",
          "id": "repository.execute-rust-runtime-shell-javascript"
        },
        {
          "name": "observe",
          "id": "repository.observe-rust-runtime-shell-public-packet"
        },
        {
          "name": "operations",
          "id": "repository.find-rust-runtime-shell-test-operations"
        },
        {
          "name": "assertRefused",
          "id": "repository.require-rust-runtime-shell-refusal"
        }
      ]
    }
  },
  {
    "inventory": "closed Rust runtime shell public consumer operations",
    "files": {
      "js/tests/translation-runtime-shell-public-declaration-contracts.test.js": [
        {
          "name": "runConsumer",
          "id": "repository.run-rust-runtime-shell-public-consumer"
        },
        {
          "name": "executeWholeJavaScript",
          "id": "repository.execute-whole-runtime-consumer-javascript"
        },
        {
          "name": "readOwnedRuntimeOperations",
          "id": "repository.read-owned-runtime-consumer-operations"
        },
        {
          "name": "checkRuntimeEvidence",
          "id": "repository.check-runtime-consumer-evidence"
        }
      ]
    }
  },
  {
    "inventory": "closed Rust runtime shell factory locations",
    "files": {
      "rust/src/translation/rust/source_profiles.rs": [
        {
          "name": "parse_rust_with_runtime_shell_model",
          "id": "translation.parse-rust-source-models"
        }
      ],
      "rust/src/typed_program_links.rs": [
        {
          "name": "create_typed_program_links_with_rust_runtime_shell_model",
          "id": "translation.create-typed-program-links"
        }
      ],
      "rust/src/lib.rs": [
        {
          "name": "create_typed_program_links_with_rust_runtime_shell_model",
          "id": "translation.create-typed-program-links"
        }
      ]
    }
  }
];
const normalizeRuntimeShellName=name=>name.replace(/([a-z])([A-Z])/gu,'$1-$2').replaceAll('_','-').replaceAll(' ','-').toLowerCase().replace(/^api$/u,'programming-interface');
const RUNTIME_SHELL_NAME_INVENTORIES=RUNTIME_SHELL_NAME_BINDINGS.map(({inventory,files})=>({
 inventory,files:Object.keys(files),
 extract:(raw,file)=>{const text=file.endsWith('.lino')?decodeURIComponent(raw):raw;return files[file].filter(({name})=>new RegExp('\\b'+name+'\\b','u').test(text)).map(({name})=>normalizeRuntimeShellName(name));},
 recordOf:name=>Object.values(files).flat().find(entry=>normalizeRuntimeShellName(entry.name)===name).id,
}));
// Shared decimal suffix boundary decision, generated JavaScript first.
const RUST_NUMBER_BOUNDARY_NAME_INVENTORIES=[{
 inventory:'Rust decimal integer suffix boundary decisions',
 files:["js/src/translation/rust-number-rules.js","rust/src/translation/lexer/number_rules.rs"],
 extract:text=>captures(text,/^(?:export function|pub fn) (acceptRustNumberDigitBoundary|accept_rust_number_digit_boundary)\(/gmu).map(name=>name.replace(/([a-z])([A-Z])/gu,'$1-$2').replaceAll('_','-').toLowerCase()),
 recordOf:()=> 'translation.accept-rust-number-digit-boundary',
}];
// Exact source bindings for the shared ECMAScript line boundary rule.
const JAVASCRIPT_LINE_BOUNDARY_NAME_BINDINGS=[
  {
    "inventory": "ECMAScript line terminator decisions",
    "files": {
      "js/src/translation/javascript-line-terminator-rules.js": [
        {
          "name": "acceptJavaScriptLineTerminator",
          "id": "translation.accept-javascript-line-terminator"
        }
      ],
      "rust/src/translation/javascript_line_terminator_rules.rs": [
        {
          "name": "accept_java_script_line_terminator",
          "id": "translation.accept-javascript-line-terminator"
        }
      ]
    }
  },
  {
    "inventory": "ECMAScript line terminator host search",
    "files": {
      "js/src/translation/javascript-line-terminators.js": [
        {
          "name": "findJavaScriptLineTerminator",
          "id": "translation.find-javascript-line-terminator"
        }
      ]
    }
  },
  {
    "inventory": "ECMAScript line terminator policy",
    "files": {
      "js/scripts/generate-javascript-line-terminator-rules.mjs": [
        {
          "name": "policy",
          "id": "translation.javascript-line-terminator-policy"
        }
      ]
    }
  },
  {
    "inventory": "ECMAScript line terminator test vectors",
    "files": {
      "js/tests/translation-javascript-line-terminators.test.js": [
        {
          "name": "terminators",
          "id": "repository.javascript-line-terminator-test-vectors"
        }
      ],
      "rust/tests/unit/translation_javascript_line_terminators.rs": [
        {
          "name": "TERMINATORS",
          "id": "repository.javascript-line-terminator-test-vectors"
        }
      ]
    }
  },
  {
    "inventory": "JavaScript line boundary source observations",
    "files": {
      "js/tests/translation-javascript-line-terminators.test.js": [
        {
          "name": "execute",
          "id": "repository.execute-javascript-line-boundary-source"
        },
        {
          "name": "replay",
          "id": "repository.replay-javascript-line-boundary-program"
        }
      ]
    }
  },
  {
    "inventory": "native JavaScript line boundary source observations",
    "files": {
      "rust/tests/unit/translation_javascript_line_terminators.rs": [
        {
          "name": "observe",
          "id": "repository.observe-native-javascript-line-boundary-output"
        }
      ]
    }
  },
  {
    "inventory": "authentic native suite diagnostic provenance",
    "files": {
      "js/tests/translation-bulk-twenty-ci-feedback.test.js": [
        {
          "name": "suite",
          "id": "repository.reviewed-compiler-diagnostic-receipt"
        }
      ]
    }
  },
  {
    "inventory": "experimental generated Unit linked operation meanings",
    "files": {
      "experiments/generated-javascript-unit/data/policy.lino": [
        {
          "name": "owned strict JavaScript directive",
          "id": "experiment.owned-strict-javascript-directive"
        },
        {
          "name": "owned generated Unit helper",
          "id": "experiment.owned-generated-unit-helper"
        },
        {
          "name": "owned null Unit return",
          "id": "experiment.owned-null-unit-return"
        },
        {
          "name": "owned generated Unit binding",
          "id": "experiment.owned-generated-unit-binding"
        },
        {
          "name": "owned generated Unit invocation",
          "id": "experiment.owned-generated-unit-invocation"
        },
        {
          "name": "owned generated Unit entry",
          "id": "experiment.owned-generated-unit-entry"
        },
        {
          "name": "ordered generated Unit bindings",
          "id": "experiment.ordered-generated-unit-bindings"
        },
        {
          "name": "complete generated JavaScript Unit module",
          "id": "experiment.complete-generated-javascript-unit-module"
        },
        {
          "name": "ordered generated Unit helpers",
          "id": "experiment.ordered-generated-unit-helpers"
        },
        {
          "name": "owned generated entry invocation",
          "id": "experiment.owned-generated-entry-invocation"
        }
      ]
    }
  },
  {
    "inventory": "experimental generated Unit source graph relation roles",
    "files": {
      "experiments/generated-javascript-unit/model.mjs": [
        {
          "name": "sourceGeneratedJavaScriptUnit",
          "id": "experiment.generated-javascript-unit-source-graph-field"
        },
        {
          "name": "sourceGeneratedJavaScriptEntry",
          "id": "experiment.generated-javascript-unit-entry-context-field"
        },
        {
          "name": "entryTarget",
          "id": "experiment.generated-javascript-unit-entry-target-field"
        },
        {
          "name": "gaps",
          "id": "experiment.generated-javascript-unit-trivia-gap-list-field"
        },
        {
          "name": "raw",
          "id": "experiment.generated-javascript-unit-source-spelling-field"
        },
        {
          "name": "units",
          "id": "experiment.generated-javascript-unit-trivia-code-unit-list-field"
        },
        {
          "name": "valueEncoding",
          "id": "experiment.generated-javascript-unit-value-encoding-field"
        },
        {
          "name": "binding",
          "id": "experiment.generated-javascript-unit-token-effect-binding-field"
        },
        {
          "name": "declaration",
          "id": "experiment.generated-javascript-unit-token-declaration-field"
        },
        {
          "name": "observationModel",
          "id": "experiment.generated-javascript-unit-observation-model-field"
        }
      ]
    }
  },
  {
    "inventory": "experimental shared ECMAScript decision reuse",
    "files": {
      "experiments/generated-javascript-unit/parser.mjs": [
        {
          "name": "acceptJavaScriptLineTerminator",
          "id": "translation.accept-javascript-line-terminator"
        }
      ]
    }
  },
  {
    "inventory": "experimental shared ECMAScript policy reuse",
    "files": {
      "experiments/generated-javascript-unit/generate-rules.mjs": [
        {
          "name": "lineTerminatorPolicy",
          "id": "translation.javascript-line-terminator-policy"
        }
      ],
      "experiments/generated-javascript-unit/model.test.mjs": [
        {
          "name": "shared",
          "id": "translation.javascript-line-terminator-policy"
        }
      ]
    }
  }
];
const normalizeJavaScriptLineBoundaryName=name=>name.replace(/([a-z])([A-Z])/gu,'$1-$2').replaceAll('_','-').toLowerCase().replaceAll('java-script','javascript');
const JAVASCRIPT_LINE_BOUNDARY_NAME_INVENTORIES=JAVASCRIPT_LINE_BOUNDARY_NAME_BINDINGS.map(({inventory,files})=>({
 inventory,files:Object.keys(files),
 extract:(raw,file)=>{const text=file.endsWith('.lino')?decodeURIComponent(raw):raw;return files[file].filter(({name})=>new RegExp('\\b'+name+'\\b','u').test(text)).map(({name})=>normalizeJavaScriptLineBoundaryName(name));},
 recordOf:name=>Object.values(files).flat().find(entry=>normalizeJavaScriptLineBoundaryName(entry.name)===name).id,
}));
// Actual paired owned tuple initialization source and helper identities.
const OWNED_TUPLE_CALL_NAME_BINDINGS=[
  {
    "inventory": "owned tuple initialization shared decisions",
    "files": {
      "js/src/translation/rust-owned-tuple-call-rules.js": [
        {
          "name": "acceptRustOwnedTupleCallModel",
          "id": "translation.accept-rust-owned-tuple-call-model"
        },
        {
          "name": "acceptRustOwnedTupleCallInitializer",
          "id": "translation.accept-rust-owned-tuple-call-initializer"
        }
      ],
      "rust/src/translation/rust/owned_tuple_call_rules.rs": [
        {
          "name": "accept_rust_owned_tuple_call_model",
          "id": "translation.accept-rust-owned-tuple-call-model"
        },
        {
          "name": "accept_rust_owned_tuple_call_initializer",
          "id": "translation.accept-rust-owned-tuple-call-initializer"
        }
      ]
    }
  },
  {
    "inventory": "owned tuple initialization operations",
    "files": {
      "js/src/translation/rust-owned-tuple-calls.js": [
        {
          "name": "ownedTupleCallPairPosition",
          "id": "translation.check-owned-tuple-call-source-position"
        },
        {
          "name": "ownedTupleCallParameters",
          "id": "translation.check-owned-tuple-call-parameters"
        },
        {
          "name": "checkOwnedTupleCallTarget",
          "id": "translation.check-owned-tuple-call-target"
        }
      ],
      "rust/src/translation/owned_local_state.rs": [
        {
          "name": "owned_tuple_call_position",
          "id": "translation.check-owned-tuple-call-source-position"
        },
        {
          "name": "tuple_call_policy",
          "id": "translation.read-owned-tuple-call-policy"
        }
      ],
      "rust/src/translation/owned_local_state/tuple_calls.rs": [
        {
          "name": "parameters",
          "id": "translation.check-owned-tuple-call-parameters"
        },
        {
          "name": "target",
          "id": "translation.check-owned-tuple-call-target"
        },
        {
          "name": "declarations",
          "id": "translation.check-owned-tuple-call-declaration-map"
        }
      ],
      "rust/src/translation/owned_local_state/validation.rs": [
        {
          "name": "validate_entry",
          "id": "translation.validate-rust-owned-local-declaration"
        }
      ],
      "rust/src/translation/rust/owned_local_state/expressions.rs": [
        {
          "name": "initializer_expression",
          "id": "translation.lower-owned-tuple-call-initializer-expression"
        }
      ],
      "rust/src/typed_program_links/codec_owned_local_state.rs": [
        {
          "name": "interpretation",
          "id": "translation.read-owned-local-state-interpretation"
        }
      ]
    }
  },
  {
    "inventory": "program declaration ownership operations",
    "files": {
      "js/src/typed-program-declaration-owners.js": [
        {
          "name": "checkProgramDeclarationOwners",
          "id": "translation.check-program-declaration-owners"
        },
        {
          "name": "orderedItems",
          "id": "translation.check-program-declaration-item-order"
        },
        {
          "name": "containsOwnedLoop",
          "id": "translation.check-program-owned-loop-presence"
        }
      ]
    }
  },
  {
    "inventory": "owned tuple initialization policy",
    "files": {
      "js/src/translation/rust-owned-tuple-calls.js": [
        {
          "name": "RUST_OWNED_TUPLE_CALL_POLICY",
          "id": "translation.rust-owned-tuple-call-policy"
        }
      ],
      "experiments/generate-rust-owned-tuple-call-rules.mjs": [
        {
          "name": "ownedTupleCallPolicy",
          "id": "translation.rust-owned-tuple-call-policy"
        }
      ]
    }
  },
  {
    "inventory": "owned tuple initialization discovery context",
    "files": {
      "js/src/translation/rust.js": [
        {
          "name": "ownedTupleCallDiscovery",
          "id": "translation.owned-tuple-call-discovery-context"
        }
      ],
      "rust/src/translation/rust.rs": [
        {
          "name": "owned_tuple_call_discovery",
          "id": "translation.owned-tuple-call-discovery-context"
        }
      ]
    }
  },
  {
    "inventory": "owned tuple initialization source fact roles",
    "files": {
      "js/src/translation/rust-owned-local-state-reader.js": [
        {
          "name": "tupleInitialization",
          "id": "translation.owned-tuple-initialization-interpretation-field"
        },
        {
          "name": "target",
          "id": "translation.owned-tuple-initializer-target-name-field"
        },
        {
          "name": "fieldModes",
          "id": "translation.owned-tuple-initializer-field-mode-list-field"
        }
      ],
      "rust/src/translation/rust/owned_local_state.rs": [
        {
          "name": "tupleInitialization",
          "id": "translation.owned-tuple-initialization-interpretation-field"
        }
      ],
      "rust/src/translation/rust/owned_local_state/lowering.rs": [
        {
          "name": "target",
          "id": "translation.owned-tuple-initializer-target-name-field"
        },
        {
          "name": "fieldModes",
          "id": "translation.owned-tuple-initializer-field-mode-list-field"
        }
      ]
    }
  },
  {
    "inventory": "extracted owned local domain operations",
    "files": {
      "js/src/translation/rust-owned-local-domains.js": [
        {
          "name": "ownedLocalType",
          "id": "translation.check-rust-owned-local-value-type"
        },
        {
          "name": "ownedLocalPair",
          "id": "translation.check-rust-owned-local-pair-type"
        },
        {
          "name": "ownedLocalPosition",
          "id": "translation.check-rust-owned-local-source-position"
        },
        {
          "name": "acceptProgramIntegralMetadata",
          "id": "translation.accept-program-integral-metadata"
        },
        {
          "name": "acceptRustOwnedLocalValueType",
          "id": "translation.accept-rust-owned-local-value-type"
        }
      ]
    }
  },
  {
    "inventory": "owned local expression context implementations",
    "files": {
      "rust/src/translation/rust/owned_local_state/expressions.rs": [
        {
          "name": "expression_in_context",
          "id": "translation.lower-rust-owned-local-expression"
        }
      ],
      "js/src/translation/rust-owned-local-state-reader.js": [
        {
          "name": "expression",
          "id": "translation.lower-rust-owned-local-expression"
        }
      ]
    }
  },
  {
    "inventory": "owned tuple initialization rule authoring operations",
    "files": {
      "experiments/generate-rust-owned-tuple-call-rules.mjs": [
        {
          "name": "rustOwnedTupleCallRules",
          "id": "repository.generate-rust-owned-tuple-call-rules"
        },
        {
          "name": "generateNativeRules",
          "id": "repository.generate-native-owned-tuple-call-rules"
        }
      ]
    }
  },
  {
    "inventory": "owned tuple initialization fixture operations",
    "files": {
      "experiments/generate-owned-tuple-call-fixture.mjs": [
        {
          "name": "ownedTupleCallFixture",
          "id": "repository.create-owned-tuple-call-fixture"
        }
      ],
      "experiments/generate-owned-tuple-call-native-fixture.mjs": [
        {
          "name": "ownedTupleCallNativeFixture",
          "id": "repository.create-owned-tuple-call-native-fixture"
        }
      ],
      "experiments/owned-tuple-call.test.mjs": [
        {
          "name": "reload",
          "id": "repository.reload-owned-tuple-call-test-program"
        },
        {
          "name": "execute",
          "id": "repository.execute-owned-tuple-call-test-program"
        },
        {
          "name": "call",
          "id": "repository.find-owned-tuple-call-test-initializer"
        }
      ]
    }
  },
  {
    "inventory": "owned tuple initialization public consumer operations",
    "files": {
      "js/tests/translation-owned-tuple-call-public-declaration-contracts.test.js": [
        {
          "name": "runConsumer",
          "id": "repository.run-owned-tuple-call-public-consumer"
        },
        {
          "name": "findTupleInitializer",
          "id": "repository.find-public-owned-tuple-call-initializer"
        },
        {
          "name": "executeTupleCall",
          "id": "repository.execute-public-owned-tuple-call"
        },
        {
          "name": "checkTupleCallPacket",
          "id": "repository.check-public-owned-tuple-call-packet"
        }
      ]
    }
  },
  {
    "inventory": "owned tuple initialization native test operations",
    "files": {
      "rust/tests/unit/translation_rust_owned_tuple_calls.rs": [
        {
          "name": "fixture",
          "id": "repository.read-owned-tuple-call-test-fixture"
        },
        {
          "name": "archives",
          "id": "repository.read-owned-tuple-call-native-test-archives"
        },
        {
          "name": "model",
          "id": "repository.read-owned-tuple-call-test-model"
        },
        {
          "name": "archive",
          "id": "repository.reload-owned-tuple-call-native-test-archive"
        },
        {
          "name": "program",
          "id": "repository.read-owned-tuple-call-native-test-program"
        },
        {
          "name": "selected",
          "id": "repository.parse-owned-tuple-call-native-test-source"
        },
        {
          "name": "function_mut",
          "id": "repository.read-mutable-owned-tuple-test-function"
        },
        {
          "name": "initializer",
          "id": "repository.find-native-owned-tuple-test-initializer"
        },
        {
          "name": "retarget",
          "id": "repository.change-owned-tuple-test-target"
        },
        {
          "name": "observe_javascript",
          "id": "repository.observe-owned-tuple-test-javascript"
        },
        {
          "name": "expected",
          "id": "repository.read-owned-tuple-test-expected-output"
        },
        {
          "name": "javascript_archive",
          "id": "repository.emit-owned-tuple-test-archive-through-javascript"
        },
        {
          "name": "relation",
          "id": "repository.read-owned-tuple-test-relation"
        },
        {
          "name": "declaration",
          "id": "repository.read-owned-tuple-test-declaration"
        },
        {
          "name": "replace_field",
          "id": "repository.replace-owned-tuple-test-field"
        },
        {
          "name": "scalar_metadata",
          "id": "repository.create-owned-tuple-test-scalar-metadata"
        },
        {
          "name": "refuses",
          "id": "repository.require-owned-tuple-test-refusal"
        },
        {
          "name": "rust_observations",
          "id": "repository.execute-owned-tuple-test-rust-artifacts"
        }
      ]
    }
  },
  {
    "inventory": "retained Rust constant predicate source operations",
    "files": {
      "experiments/generate-rust-const-function-fixture.mjs": [
        {
          "name": "readRustConstPredicateSources",
          "id": "repository.read-rust-constant-predicate-sources"
        }
      ]
    }
  },
  {
    "inventory": "evidence workflow dependency test operations",
    "files": {
      "js/tests/issue-195-evidence-dag.test.js": [
        {
          "name": "read",
          "id": "repository.read-evidence-workflow-source"
        },
        {
          "name": "matrixEntries",
          "id": "repository.read-evidence-workflow-matrix-entries"
        },
        {
          "name": "needs",
          "id": "repository.read-evidence-workflow-job-dependencies"
        },
        {
          "name": "condition",
          "id": "repository.read-evidence-workflow-job-condition"
        },
        {
          "name": "assertEvidenceDagConservation",
          "id": "repository.check-evidence-workflow-dependency-conservation"
        }
      ]
    }
  }
];
const normalizeOwnedTupleCallName=name=>name.replace(/([a-z])([A-Z])/gu,'$1-$2').replaceAll('_','-').toLowerCase().replace(/\bmut\b/gu,'mutable').replace(/^retarget$/u,'change-target').replace(/\bconst\b/gu,'constant').replace(/\bdag\b/gu,'directed-acyclic-graph');
const OWNED_TUPLE_CALL_NAME_INVENTORIES=OWNED_TUPLE_CALL_NAME_BINDINGS.map(({inventory,files})=>({
 inventory,files:Object.keys(files),
 extract:(raw,file)=>{const text=file.endsWith('.lino')?decodeURIComponent(raw):raw;return files[file].filter(({name})=>new RegExp('\\b'+name+'\\b','u').test(text)).map(({name})=>normalizeOwnedTupleCallName(name));},
 recordOf:name=>Object.values(files).flat().find(entry=>normalizeOwnedTupleCallName(entry.name)===name).id,
}));
// Actual paired Rust lexical, transport tooling and isolated whole source experiment identities.
const RUST_LEXICAL_SOURCE_NAME_BINDINGS=[
  {
    "inventory": "paired Rust lexical decisions",
    "files": {
      "js/src/translation/rust-cooked-string-rules.js": [
        {
          "name": "acceptRustCookedStringEscapePrefix",
          "id": "translation.accept-rust-cooked-string-escape-prefix"
        },
        {
          "name": "readRustAsciiEscapeDigit",
          "id": "translation.read-rust-hexadecimal-escape-digit"
        },
        {
          "name": "readRustAsciiEscapeValue",
          "id": "translation.read-rust-cooked-byte-escape-value"
        }
      ],
      "rust/src/translation/lexer/cooked_strings.rs": [
        {
          "name": "accept_rust_cooked_string_escape_prefix",
          "id": "translation.accept-rust-cooked-string-escape-prefix"
        },
        {
          "name": "read_rust_ascii_escape_digit",
          "id": "translation.read-rust-hexadecimal-escape-digit"
        },
        {
          "name": "read_rust_ascii_escape_value",
          "id": "translation.read-rust-cooked-byte-escape-value"
        }
      ],
      "js/src/translation/rust-unicode-escape-rules.js": [
        {
          "name": "decodeRustUnicodeEscape",
          "id": "translation.decode-rust-unicode-escape"
        }
      ],
      "rust/src/translation/lexer/unicode_escapes.rs": [
        {
          "name": "decode_rust_unicode_escape",
          "id": "translation.decode-rust-unicode-escape"
        }
      ],
      "js/src/translation/rust-source-whitespace-rules.js": [
        {
          "name": "acceptRustSourceWhitespace",
          "id": "translation.accept-rust-source-whitespace"
        },
        {
          "name": "acceptRustSourceByteOrderMark",
          "id": "translation.accept-rust-source-byte-order-mark"
        },
        {
          "name": "acceptRustSourceNumberDigitBoundary",
          "id": "translation.accept-rust-source-number-digit-boundary"
        }
      ],
      "rust/src/translation/lexer/source_whitespace.rs": [
        {
          "name": "accept_rust_source_whitespace",
          "id": "translation.accept-rust-source-whitespace"
        },
        {
          "name": "accept_rust_source_byte_order_mark",
          "id": "translation.accept-rust-source-byte-order-mark"
        },
        {
          "name": "accept_rust_source_number_digit_boundary",
          "id": "translation.accept-rust-source-number-digit-boundary"
        }
      ]
    }
  },
  {
    "inventory": "reused Rust hexadecimal classifier",
    "files": {
      "js/src/translation/rust-unicode-escape-rules.js": [
        {
          "name": "readRustAsciiEscapeDigit",
          "id": "translation.read-rust-hexadecimal-escape-digit"
        }
      ],
      "rust/src/translation/lexer/unicode_escapes.rs": [
        {
          "name": "read_rust_ascii_escape_digit",
          "id": "translation.read-rust-hexadecimal-escape-digit"
        }
      ]
    }
  },
  {
    "inventory": "reused Rust numeric boundary",
    "files": {
      "js/src/translation/rust-source-whitespace-rules.js": [
        {
          "name": "acceptRustNumberDigitBoundary",
          "id": "translation.accept-rust-number-digit-boundary"
        }
      ],
      "rust/src/translation/lexer/source_whitespace.rs": [
        {
          "name": "accept_rust_number_digit_boundary",
          "id": "translation.accept-rust-number-digit-boundary"
        }
      ]
    }
  },
  {
    "inventory": "canonical Unicode escape result",
    "files": {
      "js/src/translation/rust-unicode-escape-rules.js": [
        {
          "name": "RustUnicodeEscape",
          "id": "translation.unicode-escape"
        }
      ],
      "rust/src/translation/lexer/unicode_escapes.rs": [
        {
          "name": "UnicodeEscape",
          "id": "translation.unicode-escape"
        }
      ],
      "js/scripts/generate-rust-unicode-escape-rules.mjs": [
        {
          "name": "RustUnicodeEscape",
          "id": "translation.unicode-escape"
        },
        {
          "name": "UnicodeEscape",
          "id": "translation.unicode-escape"
        }
      ]
    }
  },
  {
    "inventory": "Rust Unicode escape continuation",
    "files": {
      "rust/src/translation/lexer/unicode_escapes.rs": [
        {
          "name": "ml_decode_rust_unicode_escape_loop1",
          "id": "translation.continue-rust-unicode-escape"
        }
      ]
    }
  },
  {
    "inventory": "Rust Unicode escape continuation result",
    "files": {
      "rust/src/translation/lexer/unicode_escapes.rs": [
        {
          "name": "MlDecodeRustUnicodeEscapeLoop1Result",
          "id": "translation.rust-unicode-escape-continuation-result"
        }
      ]
    }
  },
  {
    "inventory": "Rust lexical policy cooked-string",
    "files": {
      "js/scripts/generate-rust-cooked-string-rules.mjs": [
        {
          "name": "policy",
          "id": "translation.rust-cooked-string-escape-policy"
        }
      ]
    }
  },
  {
    "inventory": "Rust lexical policy unicode-escape",
    "files": {
      "js/scripts/generate-rust-unicode-escape-rules.mjs": [
        {
          "name": "policy",
          "id": "translation.rust-unicode-scalar-escape-policy"
        }
      ]
    }
  },
  {
    "inventory": "Rust lexical policy source-whitespace",
    "files": {
      "js/scripts/generate-rust-source-whitespace-rules.mjs": [
        {
          "name": "policy",
          "id": "translation.rust-source-whitespace-policy"
        }
      ]
    }
  },
  {
    "inventory": "reused cooked string digit policy",
    "files": {
      "js/scripts/generate-rust-unicode-escape-rules.mjs": [
        {
          "name": "digitPolicy",
          "id": "translation.rust-cooked-string-escape-policy"
        }
      ]
    }
  },
  {
    "inventory": "reused native lexical library authoring readSourceTextByteLibraryStyles",
    "files": {
      "js/scripts/generate-rust-cooked-string-rules.mjs": [
        {
          "name": "readSourceTextByteLibraryStyles",
          "id": "repository.read-source-text-byte-library-styles"
        }
      ],
      "js/scripts/generate-rust-unicode-escape-rules.mjs": [
        {
          "name": "readSourceTextByteLibraryStyles",
          "id": "repository.read-source-text-byte-library-styles"
        }
      ],
      "js/scripts/generate-rust-source-whitespace-rules.mjs": [
        {
          "name": "readSourceTextByteLibraryStyles",
          "id": "repository.read-source-text-byte-library-styles"
        }
      ]
    }
  },
  {
    "inventory": "reused native lexical library authoring readSourceTextRustBlock",
    "files": {
      "js/scripts/generate-rust-cooked-string-rules.mjs": [
        {
          "name": "readSourceTextRustBlock",
          "id": "repository.read-source-text-native-source-block"
        }
      ],
      "js/scripts/generate-rust-unicode-escape-rules.mjs": [
        {
          "name": "readSourceTextRustBlock",
          "id": "repository.read-source-text-native-source-block"
        }
      ],
      "js/scripts/generate-rust-source-whitespace-rules.mjs": [
        {
          "name": "readSourceTextRustBlock",
          "id": "repository.read-source-text-native-source-block"
        }
      ]
    }
  },
  {
    "inventory": "Rust Unicode escape fixture authoring",
    "files": {
      "js/scripts/build-rust-unicode-escape-fixtures.mjs": [
        {
          "name": "rustUnicodeEscapeFixture",
          "id": "repository.create-rust-unicode-escape-fixture"
        }
      ]
    }
  },
  {
    "inventory": "Rust cooked string fixture authoring",
    "files": {
      "js/scripts/build-rust-cooked-string-escape-fixtures.mjs": [
        {
          "name": "fixture",
          "id": "repository.rust-cooked-string-escape-fixture"
        }
      ]
    }
  },
  {
    "inventory": "Rust lexical JavaScript test execute",
    "files": {
      "js/tests/translation-rust-unicode-escapes.test.js": [
        {
          "name": "execute",
          "id": "repository.execute-rust-unicode-test-artifact"
        }
      ]
    }
  },
  {
    "inventory": "Rust lexical JavaScript test replay",
    "files": {
      "js/tests/translation-rust-source-whitespace.test.js": [
        {
          "name": "replay",
          "id": "repository.replay-rust-source-whitespace-test"
        }
      ]
    }
  },
  {
    "inventory": "Rust lexical native test operations",
    "files": {
      "rust/tests/unit/translation_rust_lexical_rules.rs": [
        {
          "name": "cooked_fixture",
          "id": "repository.read-rust-cooked-string-test-fixture"
        },
        {
          "name": "unicode_fixture",
          "id": "repository.read-rust-unicode-test-fixture"
        },
        {
          "name": "source_for",
          "id": "repository.create-rust-lexical-test-source"
        },
        {
          "name": "original_offset",
          "id": "repository.read-rust-lexical-original-offset"
        },
        {
          "name": "archives",
          "id": "repository.reload-rust-lexical-test-archives"
        },
        {
          "name": "string_value",
          "id": "repository.read-rust-lexical-test-string-value"
        },
        {
          "name": "complete_program",
          "id": "repository.create-complete-rust-lexical-test-program"
        },
        {
          "name": "observe_javascript",
          "id": "repository.observe-rust-lexical-javascript-artifact"
        },
        {
          "name": "compile_and_observe",
          "id": "repository.execute-rust-lexical-native-test-artifact"
        }
      ]
    }
  },
  {
    "inventory": "translation feedback transport operations",
    "files": {
      "js/scripts/translation-test-feedback.mjs": [
        {
          "name": "captureTranslationTestFeedbackPartition",
          "id": "repository.capture-translation-test-feedback-partition"
        },
        {
          "name": "mergeTranslationTestFeedbackPartitions",
          "id": "repository.merge-translation-test-feedback-partitions"
        },
        {
          "name": "commitIdentity",
          "id": "repository.check-translation-feedback-commit-identity"
        },
        {
          "name": "partitionFiles",
          "id": "repository.read-translation-feedback-partition-files"
        },
        {
          "name": "filesWithin",
          "id": "repository.read-translation-feedback-payload-files"
        },
        {
          "name": "payloadPath",
          "id": "repository.check-translation-feedback-payload-path"
        },
        {
          "name": "targetDirectory",
          "id": "repository.check-translation-feedback-target-directory"
        },
        {
          "name": "fingerprint",
          "id": "repository.read-translation-feedback-payload-fingerprint"
        },
        {
          "name": "main",
          "id": "repository.run-translation-test-feedback-command"
        }
      ]
    }
  },
  {
    "inventory": "translation feedback transport constants",
    "files": {
      "js/scripts/translation-test-feedback.mjs": [
        {
          "name": "PARTS",
          "id": "repository.translation-feedback-partition-count"
        },
        {
          "name": "RECEIPT",
          "id": "repository.translation-feedback-receipt-file-name"
        },
        {
          "name": "KIND",
          "id": "repository.translation-feedback-receipt-kind"
        },
        {
          "name": "SOURCES",
          "id": "repository.translation-feedback-payload-source-list"
        }
      ]
    }
  },
  {
    "inventory": "translation feedback transport test operations",
    "files": {
      "js/tests/issue-195-evidence-translation-feedback.test.js": [
        {
          "name": "fixture",
          "id": "repository.create-translation-feedback-test-partitions"
        },
        {
          "name": "editReceipt",
          "id": "repository.change-translation-feedback-test-receipt"
        },
        {
          "name": "assertInitialTranslationShards",
          "id": "repository.check-initial-translation-test-shards"
        }
      ]
    }
  },
  {
    "inventory": "experimental owned Rust decisions",
    "files": {
      "experiments/generated-owned-rust/rules.mjs": [
        {
          "name": "acceptGeneratedOwnedRustModel",
          "id": "repository.accept-generated-owned-rust-model"
        },
        {
          "name": "acceptGeneratedOwnedRustClone",
          "id": "repository.accept-generated-owned-rust-clone"
        },
        {
          "name": "acceptGeneratedOwnedRustLiteralCodePoint",
          "id": "repository.accept-generated-owned-rust-literal-code-point"
        },
        {
          "name": "acceptGeneratedOwnedRustEqualityGrouping",
          "id": "repository.accept-generated-owned-rust-equality-grouping"
        },
        {
          "name": "acceptGeneratedOwnedRustOrdinaryComment",
          "id": "repository.accept-generated-owned-rust-ordinary-comment"
        }
      ],
      "experiments/generated-owned-rust/data/rules.rs": [
        {
          "name": "accept_generated_owned_rust_model",
          "id": "repository.accept-generated-owned-rust-model"
        },
        {
          "name": "accept_generated_owned_rust_clone",
          "id": "repository.accept-generated-owned-rust-clone"
        },
        {
          "name": "accept_generated_owned_rust_literal_code_point",
          "id": "repository.accept-generated-owned-rust-literal-code-point"
        },
        {
          "name": "accept_generated_owned_rust_equality_grouping",
          "id": "repository.accept-generated-owned-rust-equality-grouping"
        },
        {
          "name": "accept_generated_owned_rust_ordinary_comment",
          "id": "repository.accept-generated-owned-rust-ordinary-comment"
        }
      ]
    }
  },
  {
    "inventory": "shared experimental code unit decision",
    "files": {
      "experiments/generated-owned-rust/rules.mjs": [
        {
          "name": "acceptGeneratedOwnedRustCodeUnit",
          "id": "repository.accept-experimental-source-code-unit"
        }
      ],
      "experiments/generated-owned-rust/data/rules.rs": [
        {
          "name": "accept_generated_owned_rust_code_unit",
          "id": "repository.accept-experimental-source-code-unit"
        }
      ],
      "experiments/generated-javascript-unit/rules.mjs": [
        {
          "name": "acceptGeneratedJavaScriptUnitCodeUnit",
          "id": "repository.accept-experimental-source-code-unit"
        }
      ]
    }
  },
  {
    "inventory": "experimental owned Rust model operations",
    "files": {
      "experiments/generated-owned-rust/model.mjs": [
        {
          "name": "checkGeneratedOwnedRust",
          "id": "repository.check-generated-owned-rust"
        },
        {
          "name": "createGeneratedOwnedRust",
          "id": "repository.create-generated-owned-rust"
        },
        {
          "name": "createGeneratedOwnedRustLinks",
          "id": "repository.create-generated-owned-rust-links"
        },
        {
          "name": "decodeGeneratedOwnedRustLinks",
          "id": "repository.decode-generated-owned-rust-links"
        },
        {
          "name": "encodeGeneratedOwnedRustLinks",
          "id": "repository.encode-generated-owned-rust-links"
        },
        {
          "name": "emitGeneratedOwnedRust",
          "id": "repository.emit-generated-owned-rust"
        },
        {
          "name": "emitGeneratedOwnedRustLinks",
          "id": "repository.emit-generated-owned-rust-links"
        },
        {
          "name": "emitCanonicalGeneratedOwnedRust",
          "id": "repository.emit-canonical-generated-owned-rust"
        },
        {
          "name": "fields",
          "id": "repository.check-generated-owned-rust-record-fields"
        },
        {
          "name": "interval",
          "id": "repository.check-generated-owned-rust-source-interval"
        },
        {
          "name": "data",
          "id": "repository.check-generated-owned-rust-data-graph"
        },
        {
          "name": "compare",
          "id": "repository.compare-generated-owned-rust-proof"
        },
        {
          "name": "operationNodes",
          "id": "repository.read-generated-owned-rust-operation-nodes"
        },
        {
          "name": "surfaceMap",
          "id": "repository.map-generated-owned-rust-core-identities"
        },
        {
          "name": "checkCst",
          "id": "repository.check-generated-owned-rust-concrete-source-tree"
        },
        {
          "name": "tokenText",
          "id": "repository.render-generated-owned-rust-token"
        }
      ]
    }
  },
  {
    "inventory": "experimental owned Rust parser operations",
    "files": {
      "experiments/generated-owned-rust/parser.mjs": [
        {
          "name": "tokenName",
          "id": "repository.read-generated-owned-rust-token-name"
        },
        {
          "name": "readGeneratedOwnedRust",
          "id": "repository.read-generated-owned-rust"
        },
        {
          "name": "parseGeneratedOwnedRust",
          "id": "repository.parse-generated-owned-rust"
        },
        {
          "name": "binder",
          "id": "repository.read-generated-owned-rust-binder"
        },
        {
          "name": "readType",
          "id": "repository.read-generated-owned-rust-type"
        },
        {
          "name": "expression",
          "id": "repository.read-generated-owned-rust-expression"
        },
        {
          "name": "block",
          "id": "repository.read-generated-owned-rust-block"
        },
        {
          "name": "helper",
          "id": "repository.read-generated-owned-rust-helper"
        },
        {
          "name": "match",
          "id": "repository.match-generated-owned-rust-runtime-descriptor"
        }
      ]
    }
  },
  {
    "inventory": "experimental owned Rust codec operations",
    "files": {
      "experiments/generated-owned-rust/codec.mjs": [
        {
          "name": "encodeGeneratedOwnedRust",
          "id": "repository.encode-generated-owned-rust"
        },
        {
          "name": "decodeGeneratedOwnedRust",
          "id": "repository.decode-generated-owned-rust"
        },
        {
          "name": "bind",
          "id": "repository.bind-generated-owned-rust-core-links"
        },
        {
          "name": "read",
          "id": "repository.read-generated-owned-rust-proof-links"
        }
      ]
    }
  },
  {
    "inventory": "experimental owned Rust trivia operation",
    "files": {
      "experiments/generated-owned-rust/trivia.mjs": [
        {
          "name": "readGeneratedOwnedRustTrivia",
          "id": "repository.read-generated-owned-rust-trivia"
        }
      ]
    }
  },
  {
    "inventory": "experimental reuse of exact Rust whitespace",
    "files": {
      "experiments/generated-owned-rust/trivia.mjs": [
        {
          "name": "acceptRustSourceWhitespace",
          "id": "translation.accept-rust-source-whitespace"
        }
      ]
    }
  },
  {
    "inventory": "experimental reuse of initial source byte order mark",
    "files": {
      "experiments/generated-owned-rust/trivia.mjs": [
        {
          "name": "acceptRustSourceByteOrderMark",
          "id": "translation.accept-rust-source-byte-order-mark"
        }
      ]
    }
  },
  {
    "inventory": "experimental reuse of source interval offsets",
    "files": {
      "experiments/generated-owned-rust/model.mjs": [
        {
          "name": "acceptSourceSpanOffset",
          "id": "translation.accept-source-span-offset"
        }
      ]
    }
  },
  {
    "inventory": "experimental owned Rust policy",
    "files": {
      "experiments/generated-owned-rust/model.mjs": [
        {
          "name": "GENERATED_OWNED_RUST_POLICY",
          "id": "repository.generated-owned-rust-policy"
        }
      ],
      "experiments/generated-owned-rust/generate-rules.mjs": [
        {
          "name": "generatedOwnedRustPolicy",
          "id": "repository.generated-owned-rust-policy"
        }
      ]
    }
  },
  {
    "inventory": "experimental owned Rust authoring generate-rules.mjs",
    "files": {
      "experiments/generated-owned-rust/generate-rules.mjs": [
        {
          "name": "generatedOwnedRustRules",
          "id": "repository.generate-owned-rust-experimental-rules"
        },
        {
          "name": "nativeDecisions",
          "id": "repository.translate-owned-rust-experimental-decisions"
        }
      ]
    }
  },
  {
    "inventory": "experimental owned Rust authoring generate-fixtures.mjs",
    "files": {
      "experiments/generated-owned-rust/generate-fixtures.mjs": [
        {
          "name": "generatedOwnedRustFixture",
          "id": "repository.create-generated-owned-rust-fixture"
        }
      ]
    }
  },
  {
    "inventory": "experimental owned Rust authoring model.test.mjs",
    "files": {
      "experiments/generated-owned-rust/model.test.mjs": [
        {
          "name": "call",
          "id": "repository.find-generated-owned-rust-test-call"
        },
        {
          "name": "execute",
          "id": "repository.execute-generated-owned-rust-test-artifact"
        },
        {
          "name": "coherentTarget",
          "id": "repository.change-generated-owned-rust-test-target"
        }
      ]
    }
  },
  {
    "inventory": "experimental owned Rust source role owner",
    "files": {
      "experiments/generated-owned-rust/model.mjs": [
        {
          "name": "owner",
          "id": "repository.generated-owned-rust-clone-owner-path"
        }
      ]
    }
  },
  {
    "inventory": "experimental owned Rust source role receiver",
    "files": {
      "experiments/generated-owned-rust/model.mjs": [
        {
          "name": "receiver",
          "id": "repository.generated-owned-rust-clone-receiver"
        }
      ]
    }
  },
  {
    "inventory": "experimental owned Rust source role binding",
    "files": {
      "experiments/generated-owned-rust/model.mjs": [
        {
          "name": "binding",
          "id": "repository.generated-owned-rust-lexical-binding-target"
        }
      ]
    }
  },
  {
    "inventory": "experimental owned Rust source role declaration",
    "files": {
      "experiments/generated-owned-rust/model.mjs": [
        {
          "name": "declaration",
          "id": "repository.generated-owned-rust-declaration-target"
        }
      ]
    }
  },
  {
    "inventory": "experimental owned Rust source role literal",
    "files": {
      "experiments/generated-owned-rust/model.mjs": [
        {
          "name": "literal",
          "id": "repository.generated-owned-rust-literal-target"
        }
      ]
    }
  },
  {
    "inventory": "experimental owned Rust source role worker",
    "files": {
      "experiments/generated-owned-rust/model.mjs": [
        {
          "name": "worker",
          "id": "repository.generated-owned-rust-worker-target"
        }
      ]
    }
  },
  {
    "inventory": "experimental owned Rust source role valueAssumptions",
    "files": {
      "experiments/generated-owned-rust/model.mjs": [
        {
          "name": "valueAssumptions",
          "id": "repository.generated-owned-rust-value-assumption-list"
        }
      ]
    }
  },
  {
    "inventory": "experimental owned Rust source role gaps",
    "files": {
      "experiments/generated-owned-rust/model.mjs": [
        {
          "name": "gaps",
          "id": "repository.generated-owned-rust-trivia-gap-list"
        }
      ]
    }
  },
  {
    "inventory": "experimental owned Rust source role clones",
    "files": {
      "experiments/generated-owned-rust/model.mjs": [
        {
          "name": "clones",
          "id": "repository.generated-owned-rust-clone-proof-list"
        }
      ]
    }
  },
  {
    "inventory": "experimental owned Rust source role experimentalSourceModel",
    "files": {
      "experiments/generated-owned-rust/codec.mjs": [
        {
          "name": "experimentalSourceModel",
          "id": "repository.generated-owned-rust-provenance-model-field"
        }
      ]
    }
  },
  {
    "inventory": "experimental owned Rust source role sourceInterpretation",
    "files": {
      "experiments/generated-owned-rust/codec.mjs": [
        {
          "name": "sourceInterpretation",
          "id": "repository.generated-owned-rust-provenance-proof-field"
        }
      ]
    }
  },
  {
    "inventory": "experimental owned Rust source operations",
    "files": {
      "experiments/generated-owned-rust/data/policy.lino": [
        {
          "name": "owned generated value helper",
          "id": "repository.owned-generated-value-helper-source-operation"
        },
        {
          "name": "owned standard String clone",
          "id": "repository.owned-standard-string-clone-source-operation"
        },
        {
          "name": "owned standard String literal construction",
          "id": "repository.owned-standard-string-literal-construction-source-operation"
        },
        {
          "name": "owned lexical value",
          "id": "repository.owned-lexical-value-source-operation"
        },
        {
          "name": "owned ordered product",
          "id": "repository.owned-ordered-product-source-operation"
        },
        {
          "name": "owned product field",
          "id": "repository.owned-product-field-source-operation"
        },
        {
          "name": "owned immutable local",
          "id": "repository.owned-immutable-local-source-operation"
        },
        {
          "name": "owned scalar equality",
          "id": "repository.owned-scalar-equality-source-operation"
        },
        {
          "name": "owned conditional value",
          "id": "repository.owned-conditional-value-source-operation"
        },
        {
          "name": "owned declared product call",
          "id": "repository.owned-declared-product-call-source-operation"
        }
      ],
      "experiments/generated-owned-rust/parser.mjs": [
        {
          "name": "complete generated owned Rust module",
          "id": "repository.complete-generated-owned-rust-module-source-operation"
        }
      ]
    }
  },
  {
    "inventory": "reused actual runtime syntax operations",
    "files": {
      "experiments/generated-owned-rust/data/policy.lino": [
        {
          "name": "inert runtime lint attribute",
          "id": "translation.runtime-inert-runtime-lint-attribute"
        },
        {
          "name": "owned Unit worker definition",
          "id": "translation.runtime-owned-unit-worker-definition"
        },
        {
          "name": "complete standard runtime entry",
          "id": "translation.runtime-complete-standard-runtime-entry"
        },
        {
          "name": "unused Boolean argument scan",
          "id": "translation.runtime-unused-boolean-argument-scan"
        },
        {
          "name": "standard argument existence test",
          "id": "translation.runtime-standard-argument-existence-test"
        },
        {
          "name": "standard argument iteration",
          "id": "translation.runtime-standard-argument-iteration"
        },
        {
          "name": "standard argument owner",
          "id": "translation.runtime-standard-argument-owner"
        },
        {
          "name": "closed argument predicate",
          "id": "translation.runtime-closed-argument-predicate"
        },
        {
          "name": "standard argument flag comparison",
          "id": "translation.runtime-standard-argument-flag-comparison"
        },
        {
          "name": "owned thread handle binding",
          "id": "translation.runtime-owned-thread-handle-binding"
        },
        {
          "name": "standard thread builder construction",
          "id": "translation.runtime-standard-thread-builder-construction"
        },
        {
          "name": "standard thread builder owner",
          "id": "translation.runtime-standard-thread-builder-owner"
        },
        {
          "name": "bounded stack size request",
          "id": "translation.runtime-bounded-stack-size-request"
        },
        {
          "name": "unsigned stack size shift",
          "id": "translation.runtime-unsigned-stack-size-shift"
        },
        {
          "name": "standard Unit thread spawn",
          "id": "translation.runtime-standard-unit-thread-spawn"
        },
        {
          "name": "closed moved Unit worker",
          "id": "translation.runtime-closed-moved-unit-worker"
        },
        {
          "name": "owned worker invocation",
          "id": "translation.runtime-owned-worker-invocation"
        },
        {
          "name": "standard thread success requirement",
          "id": "translation.runtime-standard-thread-success-requirement"
        },
        {
          "name": "retained worker failure branch",
          "id": "translation.runtime-retained-worker-failure-branch"
        },
        {
          "name": "standard worker failure predicate",
          "id": "translation.runtime-standard-worker-failure-predicate"
        },
        {
          "name": "standard worker join",
          "id": "translation.runtime-standard-worker-join"
        },
        {
          "name": "standard worker failure exit",
          "id": "translation.runtime-standard-worker-failure-exit"
        },
        {
          "name": "standard exit owner",
          "id": "translation.runtime-standard-exit-owner"
        }
      ]
    }
  }
];
const normalizeRustLexicalSourceName=name=>name.replace(/([a-z0-9])([A-Z])/gu,'$1-$2').replaceAll('_','-').replaceAll(' ','-').toLowerCase().replace(/\bascii\b/gu,'american-standard-code-for-information-interchange').replace(/\bcst\b/gu,'concrete-source-tree').replace(/\bml\b/gu,'meta-language').replace(/loop1/gu,'loop-one').replace(/\bconst\b/gu,'constant');
const RUST_LEXICAL_SOURCE_NAME_INVENTORIES=RUST_LEXICAL_SOURCE_NAME_BINDINGS.map(({inventory,files})=>({
 inventory,files:Object.keys(files),
 extract:(raw,file)=>{const text=file.endsWith('.lino')?decodeURIComponent(raw):raw;return files[file].filter(({name})=>new RegExp('\\b'+name+'\\b','u').test(text)).map(({name})=>normalizeRustLexicalSourceName(name));},
 recordOf:name=>Object.values(files).flat().find(entry=>normalizeRustLexicalSourceName(entry.name)===name).id,
}));
// Actual shared raw string source and isolated owned literal edit identities.
const RUST_LITERAL_VALUE_NAME_BINDINGS=[
  {
    "inventory": "unchanged shared Rust string rendering",
    "files": {
      "js/src/translation/rust-string-literals.js": [
        {
          "name": "rustEscape",
          "id": "translation.escape-rust-string-content"
        },
        {
          "name": "rustString",
          "id": "translation.render-rust-string-literal"
        }
      ],
      "js/src/translation/emit-rust.js": [
        {
          "name": "rustEscape",
          "id": "translation.escape-rust-string-content"
        },
        {
          "name": "rustString",
          "id": "translation.render-rust-string-literal"
        }
      ],
      "experiments/generated-owned-rust/model.mjs": [
        {
          "name": "rustString",
          "id": "translation.render-rust-string-literal"
        }
      ]
    }
  },
  {
    "inventory": "Rust raw string decision",
    "files": {
      "js/src/translation/rust-raw-string-rules.js": [
        {
          "name": "decodeRustRawString",
          "id": "translation.decode-rust-raw-string"
        }
      ],
      "rust/src/translation/lexer/raw_strings.rs": [
        {
          "name": "decode_rust_raw_string",
          "id": "translation.decode-rust-raw-string"
        }
      ]
    }
  },
  {
    "inventory": "Rust raw string scan result",
    "files": {
      "js/src/translation/rust-raw-string-rules.js": [
        {
          "name": "RustRawString",
          "id": "translation.rust-raw-string-scan-result"
        }
      ],
      "rust/src/translation/lexer/raw_strings.rs": [
        {
          "name": "RustRawString",
          "id": "translation.rust-raw-string-scan-result"
        }
      ]
    }
  },
  {
    "inventory": "Rust raw string policy",
    "files": {
      "js/scripts/generate-rust-raw-string-rules.mjs": [
        {
          "name": "policy",
          "id": "translation.rust-raw-string-literal-policy"
        }
      ]
    }
  },
  {
    "inventory": "Rust raw string reused readRustSourceLineBreakEnd",
    "files": {
      "js/src/translation/rust-raw-string-rules.js": [
        {
          "name": "readRustSourceLineBreakEnd",
          "id": "translation.read-rust-source-line-break-end"
        }
      ],
      "rust/src/translation/lexer/raw_strings.rs": [
        {
          "name": "read_rust_source_line_break_end",
          "id": "translation.read-rust-source-line-break-end"
        }
      ]
    }
  },
  {
    "inventory": "Rust raw string reused readSourceTextScalar",
    "files": {
      "js/src/translation/rust-raw-string-rules.js": [
        {
          "name": "readSourceTextScalar",
          "id": "translation.read-source-text-scalar"
        }
      ],
      "rust/src/translation/lexer/raw_strings.rs": [
        {
          "name": "read_source_text_scalar",
          "id": "translation.read-source-text-scalar"
        }
      ]
    }
  },
  {
    "inventory": "experimental owned literal edit decisions",
    "files": {
      "experiments/generated-owned-rust/value-edit-rules.mjs": [
        {
          "name": "acceptGeneratedOwnedRustValueEditModel",
          "id": "repository.accept-generated-owned-rust-value-edit-model"
        },
        {
          "name": "acceptGeneratedOwnedRustLiteralEditOwner",
          "id": "repository.accept-generated-owned-rust-literal-edit-owner"
        },
        {
          "name": "acceptGeneratedOwnedRustSignOwner",
          "id": "repository.accept-generated-owned-rust-sign-owner"
        }
      ],
      "experiments/generated-owned-rust/data/value-edit-rules.rs": [
        {
          "name": "accept_generated_owned_rust_value_edit_model",
          "id": "repository.accept-generated-owned-rust-value-edit-model"
        },
        {
          "name": "accept_generated_owned_rust_literal_edit_owner",
          "id": "repository.accept-generated-owned-rust-literal-edit-owner"
        },
        {
          "name": "accept_generated_owned_rust_sign_owner",
          "id": "repository.accept-generated-owned-rust-sign-owner"
        }
      ]
    }
  },
  {
    "inventory": "experimental literal edit policy",
    "files": {
      "experiments/generated-owned-rust/generate-value-edit-rules.mjs": [
        {
          "name": "generatedOwnedRustValueEditPolicy",
          "id": "repository.generated-owned-rust-value-edit-policy"
        }
      ],
      "experiments/generated-owned-rust/model.mjs": [
        {
          "name": "GENERATED_OWNED_RUST_VALUE_EDIT_POLICY",
          "id": "repository.generated-owned-rust-value-edit-policy"
        }
      ]
    }
  },
  {
    "inventory": "experimental owned literal edit model.mjs",
    "files": {
      "experiments/generated-owned-rust/model.mjs": [
        {
          "name": "sourcePolicy",
          "id": "repository.select-generated-owned-rust-source-policy"
        },
        {
          "name": "literalEdits",
          "id": "repository.read-generated-owned-rust-literal-edit-owners"
        },
        {
          "name": "operationAssumptions",
          "id": "repository.read-generated-owned-rust-operation-assumptions"
        }
      ]
    }
  },
  {
    "inventory": "experimental owned literal edit parser.mjs",
    "files": {
      "experiments/generated-owned-rust/parser.mjs": [
        {
          "name": "readGeneratedOwnedRustCookedLiteral",
          "id": "repository.read-generated-owned-rust-cooked-literal"
        }
      ]
    }
  },
  {
    "inventory": "experimental owned literal edit generate-value-edit-rules.mjs",
    "files": {
      "experiments/generated-owned-rust/generate-value-edit-rules.mjs": [
        {
          "name": "generatedOwnedRustValueEditRules",
          "id": "repository.generate-owned-rust-value-edit-rules"
        },
        {
          "name": "translatedRules",
          "id": "repository.translate-owned-rust-value-edit-decisions"
        }
      ]
    }
  },
  {
    "inventory": "experimental owned literal edit generate-value-edit-fixtures.mjs",
    "files": {
      "experiments/generated-owned-rust/generate-value-edit-fixtures.mjs": [
        {
          "name": "generatedOwnedRustValueEditFixtures",
          "id": "repository.create-owned-rust-value-edit-fixtures"
        }
      ]
    }
  },
  {
    "inventory": "experimental owned literal edit value-edits.test.mjs",
    "files": {
      "experiments/generated-owned-rust/value-edits.test.mjs": [
        {
          "name": "execute",
          "id": "repository.execute-owned-rust-value-edit-test-artifact"
        },
        {
          "name": "verifyRust",
          "id": "repository.check-owned-rust-value-edit-source-reload"
        }
      ]
    }
  },
  {
    "inventory": "experimental owned literal edit role literalEdits",
    "files": {
      "experiments/generated-owned-rust/data/value-edit-policy.lino": [
        {
          "name": "literalEdits",
          "id": "repository.generated-owned-rust-literal-edit-list"
        }
      ]
    }
  },
  {
    "inventory": "experimental owned literal edit role operationNode",
    "files": {
      "experiments/generated-owned-rust/model.mjs": [
        {
          "name": "operationNode",
          "id": "repository.generated-owned-rust-literal-edit-operation-target"
        }
      ]
    }
  },
  {
    "inventory": "experimental owned literal edit role signToken",
    "files": {
      "experiments/generated-owned-rust/model.mjs": [
        {
          "name": "signToken",
          "id": "repository.generated-owned-rust-literal-sign-target"
        }
      ]
    }
  },
  {
    "inventory": "experimental editable literal identity",
    "files": {
      "experiments/generated-owned-rust/data/value-edit-policy.lino": [
        {
          "name": "literal",
          "id": "repository.generated-owned-rust-literal-target"
        }
      ]
    }
  },
  {
    "inventory": "experimental existing String literal operation",
    "files": {
      "experiments/generated-owned-rust/data/value-edit-policy.lino": [
        {
          "name": "owned standard String literal construction",
          "id": "repository.owned-standard-string-literal-construction-source-operation"
        }
      ]
    }
  },
  {
    "inventory": "experimental existing signed integer literal operation",
    "files": {
      "experiments/generated-owned-rust/parser.mjs": [
        {
          "name": "owned signed i32 literal",
          "id": "repository.owned-signed-thirty-two-bit-integer-literal-source-operation"
        }
      ],
      "experiments/generated-owned-rust/data/value-edit-policy.lino": [
        {
          "name": "owned signed i32 literal",
          "id": "repository.owned-signed-thirty-two-bit-integer-literal-source-operation"
        }
      ]
    }
  },
  {
    "inventory": "Rust raw string fixture authoring",
    "files": {
      "js/scripts/generate-rust-raw-string-rules.mjs": [
        {
          "name": "fixture",
          "id": "repository.rust-raw-string-literal-fixture"
        }
      ]
    }
  },
  {
    "inventory": "Rust raw string JavaScript test operations",
    "files": {
      "js/tests/translation-rust-raw-string-literals.test.js": [
        {
          "name": "units",
          "id": "repository.read-rust-raw-string-test-code-units"
        },
        {
          "name": "literalFor",
          "id": "repository.create-rust-raw-string-test-literal"
        },
        {
          "name": "sourceFor",
          "id": "repository.create-rust-raw-string-test-program"
        },
        {
          "name": "execute",
          "id": "repository.execute-rust-raw-string-test-javascript"
        },
        {
          "name": "reloads",
          "id": "repository.reload-rust-raw-string-test-networks"
        }
      ]
    }
  },
  {
    "inventory": "Rust raw string host adapter",
    "files": {
      "rust/src/translation/lexer/literals.rs": [
        {
          "name": "raw_string_token",
          "id": "translation.read-rust-raw-string-token"
        }
      ]
    }
  },
  {
    "inventory": "Rust raw string generated scan operations",
    "files": {
      "rust/src/translation/lexer/raw_strings.rs": [
        {
          "name": "ml_decode_rust_raw_string_loop5",
          "id": "translation.read-rust-raw-string-opening-hash-end"
        },
        {
          "name": "ml_decode_rust_raw_string_loop6",
          "id": "translation.continue-rust-raw-string-content"
        },
        {
          "name": "ml_decode_rust_raw_string_loop8",
          "id": "translation.read-rust-raw-string-closing-hashes"
        }
      ]
    }
  },
  {
    "inventory": "Rust raw string generated control MlDecodeRustRawStringLoop6Result",
    "files": {
      "rust/src/translation/lexer/raw_strings.rs": [
        {
          "name": "MlDecodeRustRawStringLoop6Result",
          "id": "translation.rust-raw-string-content-scan-result"
        }
      ]
    }
  },
  {
    "inventory": "Rust raw string generated control MlDecodeRustRawStringJoin7",
    "files": {
      "rust/src/translation/lexer/raw_strings.rs": [
        {
          "name": "MlDecodeRustRawStringJoin7",
          "id": "translation.rust-raw-string-closing-choice-result"
        }
      ]
    }
  },
  {
    "inventory": "Rust raw string generated control MlDecodeRustRawStringLoop8Result",
    "files": {
      "rust/src/translation/lexer/raw_strings.rs": [
        {
          "name": "MlDecodeRustRawStringLoop8Result",
          "id": "translation.rust-raw-string-closing-hash-state"
        }
      ]
    }
  },
  {
    "inventory": "shared actual runtime test fixture",
    "files": {
      "rust/tests/unit/translation_rust_raw_string_literals.rs": [
        {
          "name": "runtime_fixture",
          "id": "repository.read-rust-runtime-shell-fixture"
        }
      ]
    }
  },
  {
    "inventory": "shared actual runtime test record copy",
    "files": {
      "rust/tests/unit/translation_rust_raw_string_literals.rs": [
        {
          "name": "clone_record",
          "id": "repository.clone-rust-runtime-shell-test-record"
        }
      ]
    }
  },
  {
    "inventory": "Rust raw string native test operations",
    "files": {
      "rust/tests/unit/translation_rust_raw_string_literals.rs": [
        {
          "name": "fixture",
          "id": "repository.read-rust-raw-string-test-fixture"
        },
        {
          "name": "literal",
          "id": "repository.create-native-raw-string-delimited-test-literal"
        },
        {
          "name": "case_literal",
          "id": "repository.create-native-raw-string-fixture-literal"
        },
        {
          "name": "source_for",
          "id": "repository.create-native-raw-string-test-program"
        },
        {
          "name": "offset",
          "id": "repository.read-native-raw-string-original-offset"
        },
        {
          "name": "archives",
          "id": "repository.reload-native-raw-string-test-networks"
        },
        {
          "name": "value",
          "id": "repository.read-native-raw-string-test-value"
        },
        {
          "name": "execute_javascript",
          "id": "repository.execute-native-raw-string-javascript-artifact"
        },
        {
          "name": "execute_runtime_javascript",
          "id": "repository.execute-native-raw-string-runtime-javascript"
        },
        {
          "name": "execute_rust",
          "id": "repository.execute-native-raw-string-rust-artifact"
        },
        {
          "name": "field",
          "id": "repository.read-native-raw-string-test-field"
        },
        {
          "name": "scalar_field",
          "id": "repository.read-native-raw-string-test-scalar-field"
        },
        {
          "name": "operations",
          "id": "repository.read-native-raw-string-test-operations"
        },
        {
          "name": "runtime_calls",
          "id": "repository.read-native-raw-string-test-runtime-calls"
        },
        {
          "name": "check_runtime_owners",
          "id": "repository.check-native-raw-string-test-runtime-owners"
        },
        {
          "name": "forge_owner",
          "id": "repository.change-native-raw-string-test-owner"
        }
      ]
    }
  }
];
const normalizeRustLiteralValueName=name=>name.replace(/([a-z0-9])([A-Z])/gu,'$1-$2').replaceAll('_','-').replaceAll(' ','-').toLowerCase().replace(/\bascii\b/gu,'american-standard-code-for-information-interchange').replace(/\bml\b/gu,'meta-language').replace(/\bi32\b/gu,'signed-thirty-two-bit-integer').replace(/\bconst\b/gu,'constant').replace(/\bloop5\b/gu,'loop-five').replace(/\bloop6\b/gu,'loop-six').replace(/\bjoin7\b/gu,'join-seven').replace(/\bloop8\b/gu,'loop-eight');
const RUST_LITERAL_VALUE_NAME_INVENTORIES=RUST_LITERAL_VALUE_NAME_BINDINGS.map(({inventory,files})=>({
 inventory,files:Object.keys(files),
 extract:(raw,file)=>{const text=file.endsWith('.lino')?decodeURIComponent(raw):raw;return files[file].filter(({name})=>new RegExp('\\b'+name+'\\b','u').test(text)).map(({name})=>normalizeRustLiteralValueName(name));},
 recordOf:name=>Object.values(files).flat().find(entry=>normalizeRustLiteralValueName(entry.name)===name).id,
}));
// Actual isolated source text value operations and unchanged helper aliases.
const RUST_TEXT_PROJECTION_NAME_BINDINGS=[
  {
    "inventory": "shared experimental source proof aliases",
    "files": {
      "experiments/generated-owned-rust/model.mjs": [
        {
          "name": "checkGeneratedOwnedRustData",
          "id": "repository.check-generated-owned-rust-data-graph"
        },
        {
          "name": "checkGeneratedOwnedRustFields",
          "id": "repository.check-generated-owned-rust-record-fields"
        },
        {
          "name": "checkGeneratedOwnedRustInterval",
          "id": "repository.check-generated-owned-rust-source-interval"
        },
        {
          "name": "compareGeneratedOwnedRustData",
          "id": "repository.compare-generated-owned-rust-proof"
        },
        {
          "name": "mapGeneratedOwnedRustSurface",
          "id": "repository.map-generated-owned-rust-core-identities"
        },
        {
          "name": "readGeneratedOwnedRustOperations",
          "id": "repository.read-generated-owned-rust-operation-nodes"
        },
        {
          "name": "checkGeneratedOwnedRustCst",
          "id": "repository.check-generated-owned-rust-concrete-source-tree"
        }
      ],
      "experiments/rust-text-value-projection/model.mjs": [
        {
          "name": "checkGeneratedOwnedRustData",
          "id": "repository.check-generated-owned-rust-data-graph"
        },
        {
          "name": "checkGeneratedOwnedRustFields",
          "id": "repository.check-generated-owned-rust-record-fields"
        },
        {
          "name": "checkGeneratedOwnedRustInterval",
          "id": "repository.check-generated-owned-rust-source-interval"
        },
        {
          "name": "compareGeneratedOwnedRustData",
          "id": "repository.compare-generated-owned-rust-proof"
        },
        {
          "name": "checkGeneratedOwnedRustCst",
          "id": "repository.check-generated-owned-rust-concrete-source-tree"
        }
      ]
    }
  },
  {
    "inventory": "experimental text projection decisions",
    "files": {
      "experiments/rust-text-value-projection/rules.mjs": [
        {
          "name": "acceptRustTextValueProjectionModel",
          "id": "repository.accept-rust-text-value-projection-model"
        },
        {
          "name": "readRustTextValueProjectionDomain",
          "id": "repository.read-rust-text-value-projection-domain"
        },
        {
          "name": "acceptRustTextValueProjectionOperation",
          "id": "repository.accept-rust-text-value-projection-operation"
        }
      ],
      "experiments/rust-text-value-projection/data/rules.rs": [
        {
          "name": "accept_rust_text_value_projection_model",
          "id": "repository.accept-rust-text-value-projection-model"
        },
        {
          "name": "read_rust_text_value_projection_domain",
          "id": "repository.read-rust-text-value-projection-domain"
        },
        {
          "name": "accept_rust_text_value_projection_operation",
          "id": "repository.accept-rust-text-value-projection-operation"
        }
      ]
    }
  },
  {
    "inventory": "experimental text projection policy",
    "files": {
      "experiments/rust-text-value-projection/generate-rules.mjs": [
        {
          "name": "rustTextValueProjectionPolicy",
          "id": "repository.rust-text-value-projection-policy"
        }
      ],
      "experiments/rust-text-value-projection/model.mjs": [
        {
          "name": "RUST_TEXT_VALUE_PROJECTION_POLICY",
          "id": "repository.rust-text-value-projection-policy"
        }
      ]
    }
  },
  {
    "inventory": "experimental text projection model operations",
    "files": {
      "experiments/rust-text-value-projection/model.mjs": [
        {
          "name": "actualOperation",
          "id": "repository.find-text-value-projection-source-operation"
        },
        {
          "name": "facts",
          "id": "repository.read-text-value-projection-function-facts"
        },
        {
          "name": "assumption",
          "id": "repository.create-text-value-projection-assumption"
        },
        {
          "name": "conditions",
          "id": "repository.read-text-value-projection-conditions"
        },
        {
          "name": "checkRows",
          "id": "repository.check-text-value-projection-proof-rows"
        },
        {
          "name": "checkRustTextValueProjection",
          "id": "repository.check-rust-text-value-projection"
        },
        {
          "name": "createRustTextValueProjection",
          "id": "repository.create-rust-text-value-projection"
        },
        {
          "name": "encodeRustTextValueProjection",
          "id": "repository.encode-rust-text-value-projection"
        },
        {
          "name": "createRustTextValueProjectionLinks",
          "id": "repository.create-rust-text-value-projection-links"
        },
        {
          "name": "decodeRustTextValueProjection",
          "id": "repository.decode-rust-text-value-projection"
        },
        {
          "name": "emissionConditions",
          "id": "repository.read-text-value-projection-emission-conditions"
        },
        {
          "name": "replayToken",
          "id": "repository.render-text-value-projection-source-token"
        },
        {
          "name": "emitRustTextValueProjection",
          "id": "repository.emit-rust-text-value-projection"
        },
        {
          "name": "emitRustTextValueProjectionLinks",
          "id": "repository.emit-rust-text-value-projection-links"
        },
        {
          "name": "emitGenericRustTextValueProjection",
          "id": "repository.emit-generic-rust-text-value-projection"
        }
      ]
    }
  },
  {
    "inventory": "experimental text projection source operations",
    "files": {
      "experiments/rust-text-value-projection/source.mjs": [
        {
          "name": "tokenValue",
          "id": "repository.read-text-projection-token-value"
        },
        {
          "name": "textPositionDomain",
          "id": "repository.read-text-projection-source-position-domain"
        },
        {
          "name": "parseTextProjectionCore",
          "id": "repository.parse-text-projection-core"
        },
        {
          "name": "tokenizeTextProjection",
          "id": "repository.read-text-projection-source-tokens"
        },
        {
          "name": "readTextProjectionSource",
          "id": "repository.read-text-projection-source"
        }
      ]
    }
  },
  {
    "inventory": "experimental text projection authoring generate-rules.mjs",
    "files": {
      "experiments/rust-text-value-projection/generate-rules.mjs": [
        {
          "name": "rustTextValueProjectionRules",
          "id": "repository.generate-rust-text-value-projection-rules"
        },
        {
          "name": "nativeRules",
          "id": "repository.translate-rust-text-value-projection-rules"
        }
      ]
    }
  },
  {
    "inventory": "experimental text projection authoring generate-fixtures.mjs",
    "files": {
      "experiments/rust-text-value-projection/generate-fixtures.mjs": [
        {
          "name": "rustTextValueProjectionFixtures",
          "id": "repository.create-rust-text-value-projection-fixtures"
        }
      ]
    }
  },
  {
    "inventory": "experimental text projection authoring model.test.mjs",
    "files": {
      "experiments/rust-text-value-projection/model.test.mjs": [
        {
          "name": "field",
          "id": "repository.read-text-value-projection-test-field"
        },
        {
          "name": "replaceField",
          "id": "repository.replace-text-value-projection-test-field"
        },
        {
          "name": "rawProof",
          "id": "repository.read-text-value-projection-test-proof"
        },
        {
          "name": "rows",
          "id": "repository.read-text-value-projection-test-rows"
        },
        {
          "name": "execute",
          "id": "repository.execute-text-value-projection-test-artifact"
        },
        {
          "name": "reject",
          "id": "repository.require-text-value-projection-test-refusal"
        },
        {
          "name": "retarget",
          "id": "repository.change-text-value-projection-test-input"
        },
        {
          "name": "verifyWhole",
          "id": "repository.check-complete-text-value-projection-test-output"
        }
      ]
    }
  },
  {
    "inventory": "experimental text projection role functions",
    "files": {
      "experiments/rust-text-value-projection/model.mjs": [
        {
          "name": "functions",
          "id": "repository.text-value-projection-function-fact-list"
        }
      ]
    }
  },
  {
    "inventory": "experimental text projection role worker",
    "files": {
      "experiments/rust-text-value-projection/model.mjs": [
        {
          "name": "worker",
          "id": "repository.text-value-projection-empty-main-target"
        }
      ]
    }
  },
  {
    "inventory": "experimental text projection role inputAssumptions",
    "files": {
      "experiments/rust-text-value-projection/model.mjs": [
        {
          "name": "inputAssumptions",
          "id": "repository.text-value-projection-input-condition-list"
        }
      ]
    }
  },
  {
    "inventory": "experimental text projection role valueAssumptions",
    "files": {
      "experiments/rust-text-value-projection/model.mjs": [
        {
          "name": "valueAssumptions",
          "id": "repository.text-value-projection-allocation-condition-list"
        }
      ]
    }
  },
  {
    "inventory": "experimental text projection role declaration",
    "files": {
      "experiments/rust-text-value-projection/source.mjs": [
        {
          "name": "declaration",
          "id": "repository.text-value-projection-declaration-target"
        }
      ]
    }
  },
  {
    "inventory": "experimental text projection role signature",
    "files": {
      "experiments/rust-text-value-projection/source.mjs": [
        {
          "name": "signature",
          "id": "repository.text-value-projection-signature-target"
        }
      ]
    }
  },
  {
    "inventory": "experimental text projection role resultPosition",
    "files": {
      "experiments/rust-text-value-projection/source.mjs": [
        {
          "name": "resultPosition",
          "id": "repository.text-value-projection-result-position-target"
        }
      ]
    }
  },
  {
    "inventory": "experimental text projection role operationNode",
    "files": {
      "experiments/rust-text-value-projection/source.mjs": [
        {
          "name": "operationNode",
          "id": "repository.text-value-projection-operation-target"
        }
      ]
    }
  },
  {
    "inventory": "experimental text projection role coreValue",
    "files": {
      "experiments/rust-text-value-projection/source.mjs": [
        {
          "name": "coreValue",
          "id": "repository.text-value-projection-core-value-target"
        }
      ]
    }
  },
  {
    "inventory": "experimental text projection role input",
    "files": {
      "experiments/rust-text-value-projection/source.mjs": [
        {
          "name": "input",
          "id": "repository.text-value-projection-input-target"
        }
      ]
    }
  },
  {
    "inventory": "experimental text projection role position",
    "files": {
      "experiments/rust-text-value-projection/source.mjs": [
        {
          "name": "position",
          "id": "repository.text-value-projection-input-position-target"
        }
      ]
    }
  },
  {
    "inventory": "experimental text projection role literal",
    "files": {
      "experiments/rust-text-value-projection/source.mjs": [
        {
          "name": "literal",
          "id": "repository.text-value-projection-literal-target"
        }
      ]
    }
  },
  {
    "inventory": "experimental text projection source operation labels",
    "files": {
      "experiments/rust-text-value-projection/data/policy.lino": [
        {
          "name": "owned standard String move",
          "id": "repository.text-value-owned-standard-string-move-source-operation"
        },
        {
          "name": "standard String construction from borrowed text",
          "id": "repository.text-value-standard-string-construction-from-borrowed-text-source-operation"
        },
        {
          "name": "standard String clone",
          "id": "repository.text-value-standard-string-clone-source-operation"
        },
        {
          "name": "standard borrowed str owned construction",
          "id": "repository.text-value-standard-borrowed-string-slice-owned-construction-source-operation"
        },
        {
          "name": "standard borrowed str display construction",
          "id": "repository.text-value-standard-borrowed-string-slice-display-construction-source-operation"
        }
      ]
    }
  },
  {
    "inventory": "experimental text projection source structure",
    "files": {
      "experiments/rust-text-value-projection/source.mjs": [
        {
          "name": "text value body",
          "id": "repository.text-value-body-source-operation"
        },
        {
          "name": "text value function",
          "id": "repository.text-value-function-source-operation"
        },
        {
          "name": "complete standard Rust text value module",
          "id": "repository.complete-standard-rust-text-value-module-source-operation"
        }
      ]
    }
  },
  {
    "inventory": "independent ordinary empty Rust entry",
    "files": {
      "experiments/rust-text-value-projection/source.mjs": [
        {
          "name": "independent empty main",
          "id": "repository.independent-empty-main-source-operation"
        }
      ]
    }
  },
  {
    "inventory": "experimental text projection domain owned",
    "files": {
      "experiments/rust-text-value-projection/data/policy.lino": [
        {
          "name": "owned standard String",
          "id": "repository.owned-standard-string-text-domain"
        }
      ]
    }
  },
  {
    "inventory": "experimental text projection domain reference",
    "files": {
      "experiments/rust-text-value-projection/data/policy.lino": [
        {
          "name": "shared standard String",
          "id": "repository.shared-standard-string-text-domain"
        }
      ]
    }
  },
  {
    "inventory": "experimental text projection domain slice",
    "files": {
      "experiments/rust-text-value-projection/data/policy.lino": [
        {
          "name": "shared primitive str",
          "id": "repository.shared-primitive-string-slice-text-domain"
        }
      ]
    }
  },
  {
    "inventory": "experimental text projection domain literal",
    "files": {
      "experiments/rust-text-value-projection/data/policy.lino": [
        {
          "name": "static primitive str literal",
          "id": "repository.static-primitive-string-slice-text-domain"
        }
      ]
    }
  },
  {
    "inventory": "experimental reused source operation ownedLocalType",
    "files": {
      "experiments/rust-text-value-projection/source.mjs": [
        {
          "name": "ownedLocalType",
          "id": "translation.check-rust-owned-local-value-type"
        }
      ]
    }
  },
  {
    "inventory": "experimental reused source operation ownedLocalPosition",
    "files": {
      "experiments/rust-text-value-projection/source.mjs": [
        {
          "name": "ownedLocalPosition",
          "id": "translation.check-rust-owned-local-source-position"
        }
      ]
    }
  },
  {
    "inventory": "experimental reused source operation rustString",
    "files": {
      "experiments/rust-text-value-projection/model.mjs": [
        {
          "name": "rustString",
          "id": "translation.render-rust-string-literal"
        }
      ]
    }
  },
  {
    "inventory": "experimental reused source operation encodeGeneratedOwnedRust",
    "files": {
      "experiments/rust-text-value-projection/model.mjs": [
        {
          "name": "encodeGeneratedOwnedRust",
          "id": "repository.encode-generated-owned-rust"
        }
      ]
    }
  },
  {
    "inventory": "experimental reused source operation decodeGeneratedOwnedRust",
    "files": {
      "experiments/rust-text-value-projection/model.mjs": [
        {
          "name": "decodeGeneratedOwnedRust",
          "id": "repository.decode-generated-owned-rust"
        }
      ]
    }
  },
  {
    "inventory": "experimental reused primitive checkRustSourceSignature",
    "files": {
      "experiments/rust-text-value-projection/source.mjs": [
        {
          "name": "checkRustSourceSignature",
          "id": "translation.check-rust-source-signature"
        }
      ]
    }
  },
  {
    "inventory": "experimental reused primitive acceptRustOwnedLocalTextInput",
    "files": {
      "experiments/rust-text-value-projection/source.mjs": [
        {
          "name": "acceptRustOwnedLocalTextInput",
          "id": "translation.accept-rust-owned-local-text-input"
        }
      ],
      "experiments/rust-text-value-projection/model.mjs": [
        {
          "name": "acceptRustOwnedLocalTextInput",
          "id": "translation.accept-rust-owned-local-text-input"
        }
      ]
    }
  },
  {
    "inventory": "experimental reused primitive renderRustOwnedLocalTextInput",
    "files": {
      "experiments/rust-text-value-projection/model.mjs": [
        {
          "name": "renderRustOwnedLocalTextInput",
          "id": "translation.render-rust-owned-local-text-input"
        }
      ]
    }
  },
  {
    "inventory": "text projection native source oracle gate",
    "files": {
      "js/tests/ci-workflow-structure.test.js": [
        {
          "name": "checkRustTextValueNativeStep",
          "id": "repository.check-rust-text-value-native-oracle-step"
        }
      ]
    }
  },
  {
    "inventory": "text projection external native oracle operations",
    "files": {
      "experiments/rust-text-value-projection/native-observations.mjs": [
        {
          "name": "verifyRustTextValueProjectionModel",
          "id": "repository.verify-rust-text-value-oracle-model"
        },
        {
          "name": "rustTextNativeMode",
          "id": "repository.read-rust-text-native-oracle-mode"
        },
        {
          "name": "planRustTextValueProjectionNative",
          "id": "repository.plan-rust-text-value-native-observations"
        },
        {
          "name": "runRustTextValueProjectionNative",
          "id": "repository.run-rust-text-value-native-observations"
        },
        {
          "name": "sha256",
          "id": "repository.hash-rust-text-native-oracle-content"
        },
        {
          "name": "textBytes",
          "id": "repository.read-rust-text-native-oracle-bytes"
        },
        {
          "name": "freeze",
          "id": "repository.freeze-rust-text-native-observation-plan"
        },
        {
          "name": "archiveSource",
          "id": "repository.reload-rust-text-native-oracle-source"
        },
        {
          "name": "parameterMode",
          "id": "repository.read-rust-text-native-consumer-parameter-mode"
        },
        {
          "name": "publicTarget",
          "id": "repository.check-rust-text-native-consumer-target"
        },
        {
          "name": "consumerSource",
          "id": "repository.create-rust-text-native-consumer-source"
        },
        {
          "name": "preparedEntry",
          "id": "repository.create-rust-text-native-observation-entry"
        },
        {
          "name": "editedSources",
          "id": "repository.create-rust-text-native-oracle-edits"
        },
        {
          "name": "checkFiles",
          "id": "repository.check-rust-text-native-oracle-files"
        },
        {
          "name": "runProcess",
          "id": "repository.run-rust-text-native-oracle-process"
        },
        {
          "name": "parseObservations",
          "id": "repository.read-rust-text-native-observation-stream"
        }
      ]
    }
  },
  {
    "inventory": "text projection external native oracle data",
    "files": {
      "experiments/rust-text-value-projection/native-observations.mjs": [
        {
          "name": "RUST_TEXT_VALUE_PROJECTION_MODEL_COMMIT",
          "id": "repository.rust-text-native-oracle-model-revision"
        },
        {
          "name": "modelDigests",
          "id": "repository.rust-text-native-oracle-model-digest-map"
        },
        {
          "name": "checkedPlans",
          "id": "repository.rust-text-native-authenticated-plan-set"
        }
      ]
    }
  },
  {
    "inventory": "text projection oracle reused model operations",
    "files": {
      "experiments/rust-text-value-projection/native-observations.mjs": [
        {
          "name": "createRustTextValueProjectionLinks",
          "id": "repository.create-rust-text-value-projection-links"
        },
        {
          "name": "decodeRustTextValueProjection",
          "id": "repository.decode-rust-text-value-projection"
        },
        {
          "name": "emitRustTextValueProjection",
          "id": "repository.emit-rust-text-value-projection"
        }
      ]
    }
  },
  {
    "inventory": "text projection oracle reused scalar admission",
    "files": {
      "experiments/rust-text-value-projection/native-observations.mjs": [
        {
          "name": "acceptRustOwnedLocalTextInput",
          "id": "translation.accept-rust-owned-local-text-input"
        }
      ]
    }
  },
  {
    "inventory": "text projection injected native runner contracts",
    "files": {
      "experiments/rust-text-value-projection/native-observations.test.mjs": [
        {
          "name": "stream",
          "id": "repository.create-rust-text-native-test-observation-stream"
        },
        {
          "name": "fakeProcess",
          "id": "repository.create-rust-text-native-test-process"
        },
        {
          "name": "injected",
          "id": "repository.run-rust-text-native-injected-contract"
        },
        {
          "name": "fails",
          "id": "repository.require-rust-text-native-test-failure"
        }
      ]
    }
  }
];
const normalizeRustTextProjectionName=name=>name.replace(/([a-z0-9])([A-Z])/gu,'$1-$2').replaceAll('_','-').replaceAll(' ','-').toLowerCase().replace(/\bcst\b/gu,'concrete-source-tree').replace(/\bstr\b/gu,'string-slice').replace(/^retarget$/u,'change-target').replace(/^tokenize-/u,'read-tokens-').replace(/\bsha256\b/gu,'secure-hash-algorithm-two-hundred-fifty-six');
const RUST_TEXT_PROJECTION_NAME_INVENTORIES=RUST_TEXT_PROJECTION_NAME_BINDINGS.map(({inventory,files})=>({
 inventory,files:Object.keys(files),
 extract:(raw,file)=>{const text=file.endsWith('.lino')?decodeURIComponent(raw):raw;return files[file].filter(({name})=>new RegExp('\\b'+name+'\\b','u').test(text)).map(({name})=>normalizeRustTextProjectionName(name));},
 recordOf:name=>Object.values(files).flat().find(entry=>normalizeRustTextProjectionName(entry.name)===name).id,
}));
// Actual isolated raw binding and portable text certificate operations and unchanged helper aliases.
const RUST_RAW_AND_TEXT_CERTIFICATE_NAME_BINDINGS=[
  {
    "inventory": "experimental raw binding decisions",
    "files": {
      "experiments/rust-raw-binding-identifiers/rules.mjs": [
        {
          "name": "acceptRustRawBindingIdentifierModel",
          "id": "repository.accept-rust-raw-binding-identifier-model"
        },
        {
          "name": "acceptRustRawBindingIdentifierName",
          "id": "repository.accept-rust-raw-binding-identifier-name"
        },
        {
          "name": "acceptRustRawBindingIdentifierType",
          "id": "repository.accept-rust-raw-binding-identifier-type"
        },
        {
          "name": "acceptRustRawBindingIdentifierClosure",
          "id": "repository.accept-rust-raw-binding-identifier-closure"
        },
        {
          "name": "acceptRustRawBindingIdentifierInvocationModel",
          "id": "repository.accept-rust-raw-binding-identifier-invocation-model"
        },
        {
          "name": "acceptRustRawBindingIdentifierInput",
          "id": "repository.accept-rust-raw-binding-identifier-input"
        }
      ],
      "experiments/rust-raw-binding-identifiers/data/rules.rs": [
        {
          "name": "accept_rust_raw_binding_identifier_model",
          "id": "repository.accept-rust-raw-binding-identifier-model"
        },
        {
          "name": "accept_rust_raw_binding_identifier_name",
          "id": "repository.accept-rust-raw-binding-identifier-name"
        },
        {
          "name": "accept_rust_raw_binding_identifier_type",
          "id": "repository.accept-rust-raw-binding-identifier-type"
        },
        {
          "name": "accept_rust_raw_binding_identifier_closure",
          "id": "repository.accept-rust-raw-binding-identifier-closure"
        },
        {
          "name": "accept_rust_raw_binding_identifier_invocation_model",
          "id": "repository.accept-rust-raw-binding-identifier-invocation-model"
        },
        {
          "name": "accept_rust_raw_binding_identifier_input",
          "id": "repository.accept-rust-raw-binding-identifier-input"
        }
      ]
    }
  },
  {
    "inventory": "experimental raw binding policy",
    "files": {
      "experiments/rust-raw-binding-identifiers/model.mjs": [
        {
          "name": "RUST_RAW_BINDING_IDENTIFIER_POLICY",
          "id": "repository.rust-raw-binding-identifier-policy"
        }
      ],
      "experiments/rust-raw-binding-identifiers/generate-rules.mjs": [
        {
          "name": "rustRawBindingIdentifierPolicy",
          "id": "repository.rust-raw-binding-identifier-policy"
        }
      ]
    }
  },
  {
    "inventory": "experimental raw binding model operations",
    "files": {
      "experiments/rust-raw-binding-identifiers/model.mjs": [
        {
          "name": "sourceText",
          "id": "repository.reconstruct-raw-binding-original-source"
        },
        {
          "name": "primitive",
          "id": "repository.accept-raw-binding-primitive-type"
        },
        {
          "name": "parse",
          "id": "repository.parse-raw-binding-complete-core"
        },
        {
          "name": "occurrences",
          "id": "repository.read-raw-binding-expression-occurrences"
        },
        {
          "name": "walk",
          "id": "repository.read-raw-binding-expression-tree"
        },
        {
          "name": "check",
          "id": "repository.check-rust-raw-binding-identifiers"
        },
        {
          "name": "checkRustRawBindingIdentifiers",
          "id": "repository.check-rust-raw-binding-identifiers"
        },
        {
          "name": "createRustRawBindingIdentifiers",
          "id": "repository.create-rust-raw-binding-identifiers"
        },
        {
          "name": "createRustRawBindingIdentifierLinks",
          "id": "repository.create-rust-raw-binding-identifier-links"
        },
        {
          "name": "decodeRustRawBindingIdentifierLinks",
          "id": "repository.decode-rust-raw-binding-identifier-links"
        },
        {
          "name": "encodeRustRawBindingIdentifierLinks",
          "id": "repository.encode-rust-raw-binding-identifier-links"
        },
        {
          "name": "checkRustRawBindingIdentifierInvocation",
          "id": "repository.check-rust-raw-binding-identifier-invocation"
        },
        {
          "name": "emitRustRawBindingIdentifiers",
          "id": "repository.emit-rust-raw-binding-identifiers"
        },
        {
          "name": "reconstructRustRawBindingIdentifierSource",
          "id": "repository.reconstruct-rust-raw-binding-identifier-source"
        }
      ]
    }
  },
  {
    "inventory": "experimental raw binding syntax operations",
    "files": {
      "experiments/rust-raw-binding-identifiers/syntax.mjs": [
        {
          "name": "validName",
          "id": "repository.accept-raw-binding-name-spelling"
        },
        {
          "name": "groups",
          "id": "repository.group-raw-binding-source-tokens"
        },
        {
          "name": "readRawBindingIdentifierSyntax",
          "id": "repository.read-raw-binding-identifier-syntax"
        },
        {
          "name": "alias",
          "id": "repository.create-raw-binding-owner-alias"
        },
        {
          "name": "renameRawBindingIdentifierSurface",
          "id": "repository.rename-raw-binding-identifier-surface"
        },
        {
          "name": "expression",
          "id": "repository.rename-raw-binding-surface-expression"
        }
      ]
    }
  },
  {
    "inventory": "experimental raw binding authoring generate-rules.mjs",
    "files": {
      "experiments/rust-raw-binding-identifiers/generate-rules.mjs": [
        {
          "name": "rustRawBindingIdentifierRules",
          "id": "repository.generate-rust-raw-binding-identifier-rules"
        },
        {
          "name": "rustRawBindingIdentifierFixtures",
          "id": "repository.rust-raw-binding-identifier-fixtures"
        },
        {
          "name": "translatedRules",
          "id": "repository.translate-rust-raw-binding-identifier-rules"
        }
      ]
    }
  },
  {
    "inventory": "experimental raw binding authoring model.test.mjs",
    "files": {
      "experiments/rust-raw-binding-identifiers/model.test.mjs": [
        {
          "name": "execute",
          "id": "repository.execute-raw-binding-test-artifact"
        },
        {
          "name": "refusal",
          "id": "repository.require-raw-binding-test-refusal"
        },
        {
          "name": "graphIdentity",
          "id": "repository.check-raw-binding-test-graph-identities"
        }
      ]
    }
  },
  {
    "inventory": "experimental raw binding role checkedOwner",
    "files": {
      "experiments/rust-raw-binding-identifiers/model.mjs": [
        {
          "name": "checkedOwner",
          "id": "repository.raw-binding-checked-owner-target"
        }
      ]
    }
  },
  {
    "inventory": "experimental raw binding role checkedExpression",
    "files": {
      "experiments/rust-raw-binding-identifiers/model.mjs": [
        {
          "name": "checkedExpression",
          "id": "repository.raw-binding-checked-expression-target"
        }
      ]
    }
  },
  {
    "inventory": "experimental raw binding role parameterOwners",
    "files": {
      "experiments/rust-raw-binding-identifiers/model.mjs": [
        {
          "name": "parameterOwners",
          "id": "repository.raw-binding-invocation-parameter-array-target"
        }
      ]
    }
  },
  {
    "inventory": "experimental raw binding role inputAssumptions",
    "files": {
      "experiments/rust-raw-binding-identifiers/model.mjs": [
        {
          "name": "inputAssumptions",
          "id": "repository.raw-binding-invocation-premise-list"
        }
      ]
    }
  },
  {
    "inventory": "experimental raw binding role aliases",
    "files": {
      "experiments/rust-raw-binding-identifiers/model.mjs": [
        {
          "name": "aliases",
          "id": "repository.raw-binding-inverse-alias-list"
        }
      ]
    }
  },
  {
    "inventory": "experimental raw binding role occurrences",
    "files": {
      "experiments/rust-raw-binding-identifiers/data/policy.lino": [
        {
          "name": "occurrences",
          "id": "repository.raw-binding-expression-occurrence-list"
        }
      ]
    }
  },
  {
    "inventory": "raw binding actual shared helper checkGeneratedOwnedRustData",
    "files": {
      "experiments/rust-raw-binding-identifiers/model.mjs": [
        {
          "name": "checkGeneratedOwnedRustData",
          "id": "repository.check-generated-owned-rust-data-graph"
        },
        {
          "name": "data",
          "id": "repository.check-generated-owned-rust-data-graph"
        }
      ]
    }
  },
  {
    "inventory": "raw binding actual shared helper checkGeneratedOwnedRustFields",
    "files": {
      "experiments/rust-raw-binding-identifiers/model.mjs": [
        {
          "name": "checkGeneratedOwnedRustFields",
          "id": "repository.check-generated-owned-rust-record-fields"
        },
        {
          "name": "fields",
          "id": "repository.check-generated-owned-rust-record-fields"
        }
      ],
      "experiments/rust-raw-binding-identifiers/syntax.mjs": [
        {
          "name": "checkGeneratedOwnedRustFields",
          "id": "repository.check-generated-owned-rust-record-fields"
        },
        {
          "name": "fields",
          "id": "repository.check-generated-owned-rust-record-fields"
        }
      ]
    }
  },
  {
    "inventory": "raw binding actual shared helper checkGeneratedOwnedRustInterval",
    "files": {
      "experiments/rust-raw-binding-identifiers/model.mjs": [
        {
          "name": "checkGeneratedOwnedRustInterval",
          "id": "repository.check-generated-owned-rust-source-interval"
        },
        {
          "name": "interval",
          "id": "repository.check-generated-owned-rust-source-interval"
        }
      ]
    }
  },
  {
    "inventory": "raw binding actual shared helper checkGeneratedOwnedRustCst",
    "files": {
      "experiments/rust-raw-binding-identifiers/model.mjs": [
        {
          "name": "checkGeneratedOwnedRustCst",
          "id": "repository.check-generated-owned-rust-concrete-source-tree"
        },
        {
          "name": "cst",
          "id": "repository.check-generated-owned-rust-concrete-source-tree"
        }
      ]
    }
  },
  {
    "inventory": "raw binding actual shared helper mapGeneratedOwnedRustSurface",
    "files": {
      "experiments/rust-raw-binding-identifiers/model.mjs": [
        {
          "name": "mapGeneratedOwnedRustSurface",
          "id": "repository.map-generated-owned-rust-core-identities"
        },
        {
          "name": "surfaceMap",
          "id": "repository.map-generated-owned-rust-core-identities"
        }
      ]
    }
  },
  {
    "inventory": "raw binding actual shared helper encodeGeneratedOwnedRust",
    "files": {
      "experiments/rust-raw-binding-identifiers/model.mjs": [
        {
          "name": "encodeGeneratedOwnedRust",
          "id": "repository.encode-generated-owned-rust"
        },
        {
          "name": "encode",
          "id": "repository.encode-generated-owned-rust"
        }
      ]
    }
  },
  {
    "inventory": "raw binding actual shared helper decodeGeneratedOwnedRust",
    "files": {
      "experiments/rust-raw-binding-identifiers/model.mjs": [
        {
          "name": "decodeGeneratedOwnedRust",
          "id": "repository.decode-generated-owned-rust"
        },
        {
          "name": "decode",
          "id": "repository.decode-generated-owned-rust"
        }
      ]
    }
  },
  {
    "inventory": "raw binding actual shared helper acceptRustPureUnitNameCharacter",
    "files": {
      "experiments/rust-raw-binding-identifiers/syntax.mjs": [
        {
          "name": "acceptRustPureUnitNameCharacter",
          "id": "translation.accept-rust-pure-unit-name-character"
        }
      ]
    }
  },
  {
    "inventory": "raw binding actual shared helper checkProgramDeclarationOwners",
    "files": {
      "experiments/rust-raw-binding-identifiers/model.mjs": [
        {
          "name": "checkProgramDeclarationOwners",
          "id": "translation.check-program-declaration-owners"
        }
      ]
    }
  },
  {
    "inventory": "shared fixed width integer bounds",
    "files": {
      "js/src/translation/types.js": [
        {
          "name": "fixedBounds",
          "id": "translation.read-fixed-width-integer-bounds"
        }
      ],
      "experiments/rust-raw-binding-identifiers/model.mjs": [
        {
          "name": "fixedBounds",
          "id": "translation.read-fixed-width-integer-bounds"
        }
      ]
    }
  },
  {
    "inventory": "portable text source certificate policy",
    "files": {
      "experiments/rust-text-certificate/policy.mjs": [
        {
          "name": "textCertificatePolicy",
          "id": "repository.portable-rust-text-certificate-policy"
        }
      ],
      "experiments/rust-text-certificate/bridge.mjs": [
        {
          "name": "RUST_TEXT_CERTIFICATE_POLICY",
          "id": "repository.portable-rust-text-certificate-policy"
        }
      ]
    }
  },
  {
    "inventory": "portable text source certificate decisions",
    "files": {
      "experiments/rust-text-certificate/rules.mjs": [
        {
          "name": "readTextCertificateKnownSymbol",
          "id": "repository.read-text-certificate-known-symbol"
        },
        {
          "name": "acceptTextCertificateStorage",
          "id": "repository.accept-text-certificate-storage"
        },
        {
          "name": "findTextCertificateRow",
          "id": "repository.find-text-certificate-row"
        },
        {
          "name": "acceptTextCertificateNumbers",
          "id": "repository.accept-text-certificate-numbers"
        },
        {
          "name": "acceptTextCertificateIdentities",
          "id": "repository.accept-text-certificate-identities"
        },
        {
          "name": "acceptTextCertificateName",
          "id": "repository.accept-text-certificate-name"
        },
        {
          "name": "readTextCertificateDomain",
          "id": "repository.read-text-certificate-domain"
        },
        {
          "name": "readTextCertificateType",
          "id": "repository.read-text-certificate-type"
        },
        {
          "name": "readTextCertificatePrefix",
          "id": "repository.read-text-certificate-prefix"
        },
        {
          "name": "readTextCertificateSuffix",
          "id": "repository.read-text-certificate-suffix"
        },
        {
          "name": "readTextCertificateOperation",
          "id": "repository.read-text-certificate-operation"
        },
        {
          "name": "readTextCertificateDomainName",
          "id": "repository.read-text-certificate-domain-name"
        },
        {
          "name": "acceptTextCertificateAllocation",
          "id": "repository.accept-text-certificate-allocation"
        },
        {
          "name": "matchTextCertificateSyntax",
          "id": "repository.match-text-certificate-syntax"
        },
        {
          "name": "acceptTextCertificateToken",
          "id": "repository.accept-text-certificate-token"
        },
        {
          "name": "acceptTextCertificateOperation",
          "id": "repository.accept-text-certificate-operation"
        },
        {
          "name": "acceptTextCertificateTokenOperation",
          "id": "repository.accept-text-certificate-token-operation"
        },
        {
          "name": "acceptTextCertificateCondition",
          "id": "repository.accept-text-certificate-condition"
        },
        {
          "name": "checkRustTextCertificate",
          "id": "repository.check-rust-text-certificate"
        }
      ],
      "experiments/rust-text-certificate/data/translation.lino": [
        {
          "name": "read_text_certificate_known_symbol",
          "id": "repository.read-text-certificate-known-symbol"
        },
        {
          "name": "accept_text_certificate_storage",
          "id": "repository.accept-text-certificate-storage"
        },
        {
          "name": "find_text_certificate_row",
          "id": "repository.find-text-certificate-row"
        },
        {
          "name": "accept_text_certificate_numbers",
          "id": "repository.accept-text-certificate-numbers"
        },
        {
          "name": "accept_text_certificate_identities",
          "id": "repository.accept-text-certificate-identities"
        },
        {
          "name": "accept_text_certificate_name",
          "id": "repository.accept-text-certificate-name"
        },
        {
          "name": "read_text_certificate_domain",
          "id": "repository.read-text-certificate-domain"
        },
        {
          "name": "read_text_certificate_type",
          "id": "repository.read-text-certificate-type"
        },
        {
          "name": "read_text_certificate_prefix",
          "id": "repository.read-text-certificate-prefix"
        },
        {
          "name": "read_text_certificate_suffix",
          "id": "repository.read-text-certificate-suffix"
        },
        {
          "name": "read_text_certificate_operation",
          "id": "repository.read-text-certificate-operation"
        },
        {
          "name": "read_text_certificate_domain_name",
          "id": "repository.read-text-certificate-domain-name"
        },
        {
          "name": "accept_text_certificate_allocation",
          "id": "repository.accept-text-certificate-allocation"
        },
        {
          "name": "match_text_certificate_syntax",
          "id": "repository.match-text-certificate-syntax"
        },
        {
          "name": "accept_text_certificate_token",
          "id": "repository.accept-text-certificate-token"
        },
        {
          "name": "accept_text_certificate_operation",
          "id": "repository.accept-text-certificate-operation"
        },
        {
          "name": "accept_text_certificate_token_operation",
          "id": "repository.accept-text-certificate-token-operation"
        },
        {
          "name": "accept_text_certificate_condition",
          "id": "repository.accept-text-certificate-condition"
        },
        {
          "name": "check_rust_text_certificate",
          "id": "repository.check-rust-text-certificate"
        }
      ]
    }
  },
  {
    "inventory": "portable text source certificate bridge operations",
    "files": {
      "experiments/rust-text-certificate/bridge.mjs": [
        {
          "name": "projectRustTextCertificate",
          "id": "repository.project-rust-text-certificate"
        },
        {
          "name": "evaluateRustTextCertificate",
          "id": "repository.evaluate-rust-text-certificate"
        },
        {
          "name": "checkPortableRustTextCertificate",
          "id": "repository.check-portable-rust-text-certificate"
        },
        {
          "name": "createPortableRustTextCertificate",
          "id": "repository.create-portable-rust-text-certificate"
        },
        {
          "name": "encodePortableRustTextCertificate",
          "id": "repository.encode-portable-rust-text-certificate"
        },
        {
          "name": "decodePortableRustTextCertificate",
          "id": "repository.decode-portable-rust-text-certificate"
        }
      ]
    }
  },
  {
    "inventory": "portable text source certificate authoring generate.mjs",
    "files": {
      "experiments/rust-text-certificate/generate.mjs": [
        {
          "name": "textCertificateSource",
          "id": "repository.generate-text-certificate-source"
        },
        {
          "name": "bundledTextCertificateSource",
          "id": "repository.bundle-text-certificate-source"
        }
      ]
    }
  },
  {
    "inventory": "portable text source certificate authoring feasibility.mjs",
    "files": {
      "experiments/rust-text-certificate/feasibility.mjs": [
        {
          "name": "probeTextCertificate",
          "id": "repository.probe-text-certificate-translation"
        }
      ]
    }
  },
  {
    "inventory": "portable text source certificate table context",
    "files": {
      "experiments/rust-text-certificate/bridge.mjs": [
        {
          "name": "context",
          "id": "repository.text-certificate-context-table"
        }
      ]
    }
  },
  {
    "inventory": "portable text source certificate table kinds",
    "files": {
      "experiments/rust-text-certificate/bridge.mjs": [
        {
          "name": "kinds",
          "id": "repository.text-certificate-identity-kind-table"
        }
      ]
    }
  },
  {
    "inventory": "portable text source certificate table tokens",
    "files": {
      "experiments/rust-text-certificate/bridge.mjs": [
        {
          "name": "tokens",
          "id": "repository.text-certificate-token-table"
        }
      ]
    }
  },
  {
    "inventory": "portable text source certificate table declarations",
    "files": {
      "experiments/rust-text-certificate/bridge.mjs": [
        {
          "name": "declarations",
          "id": "repository.text-certificate-declaration-table"
        }
      ]
    }
  },
  {
    "inventory": "portable text source certificate table parameters",
    "files": {
      "experiments/rust-text-certificate/bridge.mjs": [
        {
          "name": "parameters",
          "id": "repository.text-certificate-parameter-table"
        }
      ]
    }
  },
  {
    "inventory": "portable text source certificate table positions",
    "files": {
      "experiments/rust-text-certificate/bridge.mjs": [
        {
          "name": "positions",
          "id": "repository.text-certificate-source-position-table"
        }
      ]
    }
  },
  {
    "inventory": "portable text source certificate table paths",
    "files": {
      "experiments/rust-text-certificate/bridge.mjs": [
        {
          "name": "paths",
          "id": "repository.text-certificate-owner-path-table"
        }
      ]
    }
  },
  {
    "inventory": "portable text source certificate table layers",
    "files": {
      "experiments/rust-text-certificate/bridge.mjs": [
        {
          "name": "layers",
          "id": "repository.text-certificate-reference-layer-table"
        }
      ]
    }
  },
  {
    "inventory": "portable text source certificate table cores",
    "files": {
      "experiments/rust-text-certificate/bridge.mjs": [
        {
          "name": "cores",
          "id": "repository.text-certificate-expression-table"
        }
      ]
    }
  },
  {
    "inventory": "portable text source certificate table functions",
    "files": {
      "experiments/rust-text-certificate/bridge.mjs": [
        {
          "name": "functions",
          "id": "repository.text-certificate-function-fact-table"
        }
      ]
    }
  },
  {
    "inventory": "portable text source certificate table operations",
    "files": {
      "experiments/rust-text-certificate/bridge.mjs": [
        {
          "name": "operations",
          "id": "repository.text-certificate-operation-table"
        }
      ]
    }
  },
  {
    "inventory": "portable text source certificate table parts",
    "files": {
      "experiments/rust-text-certificate/bridge.mjs": [
        {
          "name": "parts",
          "id": "repository.text-certificate-operation-part-table"
        }
      ]
    }
  },
  {
    "inventory": "portable text source certificate table inputs",
    "files": {
      "experiments/rust-text-certificate/bridge.mjs": [
        {
          "name": "inputs",
          "id": "repository.text-certificate-input-condition-table"
        }
      ]
    }
  },
  {
    "inventory": "portable text source certificate table allocations",
    "files": {
      "experiments/rust-text-certificate/bridge.mjs": [
        {
          "name": "allocations",
          "id": "repository.text-certificate-allocation-condition-table"
        }
      ]
    }
  },
  {
    "inventory": "portable text source certificate table names",
    "files": {
      "experiments/rust-text-certificate/bridge.mjs": [
        {
          "name": "names",
          "id": "repository.text-certificate-symbol-range-table"
        }
      ]
    }
  },
  {
    "inventory": "portable text source certificate table units",
    "files": {
      "experiments/rust-text-certificate/bridge.mjs": [
        {
          "name": "units",
          "id": "repository.text-certificate-symbol-unit-table"
        }
      ]
    }
  },
  {
    "inventory": "portable text source certificate table items",
    "files": {
      "experiments/rust-text-certificate/bridge.mjs": [
        {
          "name": "items",
          "id": "repository.text-certificate-declaration-order-table"
        }
      ]
    }
  },
  {
    "inventory": "portable text source certificate value nullIdentity",
    "files": {
      "experiments/rust-text-certificate/policy.mjs": [
        {
          "name": "nullIdentity",
          "id": "repository.text-certificate-null-identity"
        }
      ]
    }
  },
  {
    "inventory": "portable text source certificate value absentIdentity",
    "files": {
      "experiments/rust-text-certificate/policy.mjs": [
        {
          "name": "absentIdentity",
          "id": "repository.text-certificate-absent-identity"
        }
      ]
    }
  },
  {
    "inventory": "portable text source certificate value output",
    "files": {
      "experiments/rust-text-certificate/policy.mjs": [
        {
          "name": "output",
          "id": "repository.text-certificate-decision-output"
        }
      ]
    }
  },
  {
    "inventory": "portable text certificate shared provider acceptRustPureUnitNameCharacter",
    "files": {
      "experiments/rust-text-certificate/rules.mjs": [
        {
          "name": "acceptRustPureUnitNameCharacter",
          "id": "translation.accept-rust-pure-unit-name-character"
        }
      ],
      "experiments/rust-text-certificate/generate.mjs": [
        {
          "name": "acceptRustPureUnitNameCharacter",
          "id": "translation.accept-rust-pure-unit-name-character"
        }
      ],
      "experiments/rust-text-certificate/data/translation.lino": [
        {
          "name": "accept_rust_pure_unit_name_character",
          "id": "translation.accept-rust-pure-unit-name-character"
        }
      ]
    }
  },
  {
    "inventory": "portable text certificate shared provider readRustTextValueProjectionDomain",
    "files": {
      "experiments/rust-text-certificate/rules.mjs": [
        {
          "name": "readRustTextValueProjectionDomain",
          "id": "repository.read-rust-text-value-projection-domain"
        }
      ],
      "experiments/rust-text-certificate/generate.mjs": [
        {
          "name": "readRustTextValueProjectionDomain",
          "id": "repository.read-rust-text-value-projection-domain"
        }
      ],
      "experiments/rust-text-certificate/data/translation.lino": [
        {
          "name": "read_rust_text_value_projection_domain",
          "id": "repository.read-rust-text-value-projection-domain"
        }
      ]
    }
  },
  {
    "inventory": "portable text certificate shared provider acceptRustTextValueProjectionOperation",
    "files": {
      "experiments/rust-text-certificate/rules.mjs": [
        {
          "name": "acceptRustTextValueProjectionOperation",
          "id": "repository.accept-rust-text-value-projection-operation"
        }
      ],
      "experiments/rust-text-certificate/generate.mjs": [
        {
          "name": "acceptRustTextValueProjectionOperation",
          "id": "repository.accept-rust-text-value-projection-operation"
        }
      ],
      "experiments/rust-text-certificate/data/translation.lino": [
        {
          "name": "accept_rust_text_value_projection_operation",
          "id": "repository.accept-rust-text-value-projection-operation"
        }
      ]
    }
  },
  {
    "inventory": "portable text certificate shared provider acceptRustOwnedLocalTextUnits",
    "files": {
      "experiments/rust-text-certificate/rules.mjs": [
        {
          "name": "acceptRustOwnedLocalTextUnits",
          "id": "translation.accept-rust-owned-local-text-units"
        }
      ],
      "experiments/rust-text-certificate/data/translation.lino": [
        {
          "name": "accept_rust_owned_local_text_units",
          "id": "translation.accept-rust-owned-local-text-units"
        }
      ]
    }
  },
  {
    "inventory": "portable text certificate shared provider checkRustSourceSignature",
    "files": {
      "experiments/rust-text-certificate/bridge.mjs": [
        {
          "name": "checkRustSourceSignature",
          "id": "translation.check-rust-source-signature"
        }
      ]
    }
  },
  {
    "inventory": "portable text certificate shared provider checkGeneratedOwnedRustData",
    "files": {
      "experiments/rust-text-certificate/bridge.mjs": [
        {
          "name": "checkGeneratedOwnedRustData",
          "id": "repository.check-generated-owned-rust-data-graph"
        }
      ]
    }
  },
  {
    "inventory": "portable text certificate shared provider checkGeneratedOwnedRustFields",
    "files": {
      "experiments/rust-text-certificate/bridge.mjs": [
        {
          "name": "checkGeneratedOwnedRustFields",
          "id": "repository.check-generated-owned-rust-record-fields"
        },
        {
          "name": "fields",
          "id": "repository.check-generated-owned-rust-record-fields"
        }
      ]
    }
  },
  {
    "inventory": "portable text certificate shared provider checkGeneratedOwnedRustCst",
    "files": {
      "experiments/rust-text-certificate/bridge.mjs": [
        {
          "name": "checkGeneratedOwnedRustCst",
          "id": "repository.check-generated-owned-rust-concrete-source-tree"
        }
      ]
    }
  },
  {
    "inventory": "portable text certificate shared provider checkGeneratedOwnedRustInterval",
    "files": {
      "experiments/rust-text-certificate/bridge.mjs": [
        {
          "name": "checkGeneratedOwnedRustInterval",
          "id": "repository.check-generated-owned-rust-source-interval"
        }
      ]
    }
  },
  {
    "inventory": "portable text certificate shared provider createRustTextValueProjection",
    "files": {
      "experiments/rust-text-certificate/bridge.mjs": [
        {
          "name": "createRustTextValueProjection",
          "id": "repository.create-rust-text-value-projection"
        }
      ]
    }
  },
  {
    "inventory": "portable text certificate shared provider encodeGeneratedOwnedRust",
    "files": {
      "experiments/rust-text-certificate/bridge.mjs": [
        {
          "name": "encodeGeneratedOwnedRust",
          "id": "repository.encode-generated-owned-rust"
        }
      ]
    }
  },
  {
    "inventory": "portable text certificate shared provider decodeGeneratedOwnedRust",
    "files": {
      "experiments/rust-text-certificate/bridge.mjs": [
        {
          "name": "decodeGeneratedOwnedRust",
          "id": "repository.decode-generated-owned-rust"
        }
      ]
    }
  },
  {
    "inventory": "portable text certificate existing operation labels",
    "files": {
      "experiments/rust-text-certificate/data/policy.lino": [
        {
          "name": "owned standard String move",
          "id": "repository.text-value-owned-standard-string-move-source-operation"
        },
        {
          "name": "standard String construction from borrowed text",
          "id": "repository.text-value-standard-string-construction-from-borrowed-text-source-operation"
        },
        {
          "name": "standard String clone",
          "id": "repository.text-value-standard-string-clone-source-operation"
        },
        {
          "name": "standard borrowed str owned construction",
          "id": "repository.text-value-standard-borrowed-string-slice-owned-construction-source-operation"
        },
        {
          "name": "standard borrowed str display construction",
          "id": "repository.text-value-standard-borrowed-string-slice-display-construction-source-operation"
        }
      ]
    }
  },
  {
    "inventory": "portable text certificate complete translated providers",
    "files": {
      "experiments/rust-text-certificate/data/translation.lino": [
        {
          "name": "acceptRustPureUnitCallModel",
          "id": "translation.accept-rust-pure-unit-call-model"
        },
        {
          "name": "accept_rust_pure_unit_call_model",
          "id": "translation.accept-rust-pure-unit-call-model"
        },
        {
          "name": "acceptRustPureUnitNameCharacter",
          "id": "translation.accept-rust-pure-unit-name-character"
        },
        {
          "name": "accept_rust_pure_unit_name_character",
          "id": "translation.accept-rust-pure-unit-name-character"
        },
        {
          "name": "acceptRustPureUnitName",
          "id": "translation.accept-rust-pure-unit-name"
        },
        {
          "name": "accept_rust_pure_unit_name",
          "id": "translation.accept-rust-pure-unit-name"
        },
        {
          "name": "acceptRustPureUnitHeaderForm",
          "id": "translation.accept-rust-pure-unit-header-form"
        },
        {
          "name": "accept_rust_pure_unit_header_form",
          "id": "translation.accept-rust-pure-unit-header-form"
        },
        {
          "name": "acceptRustPureUnitBodyForm",
          "id": "translation.accept-rust-pure-unit-body-form"
        },
        {
          "name": "accept_rust_pure_unit_body_form",
          "id": "translation.accept-rust-pure-unit-body-form"
        },
        {
          "name": "acceptRustPureUnitPathForm",
          "id": "translation.accept-rust-pure-unit-path-form"
        },
        {
          "name": "accept_rust_pure_unit_path_form",
          "id": "translation.accept-rust-pure-unit-path-form"
        },
        {
          "name": "acceptRustPureUnitContext",
          "id": "translation.accept-rust-pure-unit-context"
        },
        {
          "name": "accept_rust_pure_unit_context",
          "id": "translation.accept-rust-pure-unit-context"
        },
        {
          "name": "acceptRustPureUnitVisibility",
          "id": "translation.accept-rust-pure-unit-visibility"
        },
        {
          "name": "accept_rust_pure_unit_visibility",
          "id": "translation.accept-rust-pure-unit-visibility"
        },
        {
          "name": "acceptRustPureUnitContainer",
          "id": "translation.accept-rust-pure-unit-container"
        },
        {
          "name": "accept_rust_pure_unit_container",
          "id": "translation.accept-rust-pure-unit-container"
        },
        {
          "name": "acceptRustPureUnitSourceOffset",
          "id": "translation.accept-rust-pure-unit-source-offset"
        },
        {
          "name": "accept_rust_pure_unit_source_offset",
          "id": "translation.accept-rust-pure-unit-source-offset"
        },
        {
          "name": "acceptRustTextValueProjectionModel",
          "id": "repository.accept-rust-text-value-projection-model"
        },
        {
          "name": "accept_rust_text_value_projection_model",
          "id": "repository.accept-rust-text-value-projection-model"
        },
        {
          "name": "readRustTextValueProjectionDomain",
          "id": "repository.read-rust-text-value-projection-domain"
        },
        {
          "name": "read_rust_text_value_projection_domain",
          "id": "repository.read-rust-text-value-projection-domain"
        },
        {
          "name": "acceptRustTextValueProjectionOperation",
          "id": "repository.accept-rust-text-value-projection-operation"
        },
        {
          "name": "accept_rust_text_value_projection_operation",
          "id": "repository.accept-rust-text-value-projection-operation"
        },
        {
          "name": "acceptRustOwnedLocalStateModel",
          "id": "translation.accept-rust-owned-local-state-model"
        },
        {
          "name": "accept_rust_owned_local_state_model",
          "id": "translation.accept-rust-owned-local-state-model"
        },
        {
          "name": "acceptRustOwnedLocalValueType",
          "id": "translation.accept-rust-owned-local-value-type"
        },
        {
          "name": "accept_rust_owned_local_value_type",
          "id": "translation.accept-rust-owned-local-value-type"
        },
        {
          "name": "acceptRustOwnedLocalExpression",
          "id": "translation.accept-rust-owned-local-expression"
        },
        {
          "name": "accept_rust_owned_local_expression",
          "id": "translation.accept-rust-owned-local-expression"
        },
        {
          "name": "acceptRustOwnedLocalBinaryOperation",
          "id": "translation.accept-rust-owned-local-binary-operation"
        },
        {
          "name": "accept_rust_owned_local_binary_operation",
          "id": "translation.accept-rust-owned-local-binary-operation"
        },
        {
          "name": "acceptRustOwnedLocalSourceOperation",
          "id": "translation.accept-rust-owned-local-source-operation"
        },
        {
          "name": "accept_rust_owned_local_source_operation",
          "id": "translation.accept-rust-owned-local-source-operation"
        },
        {
          "name": "acceptRustOwnedLocalContainer",
          "id": "translation.accept-rust-owned-local-container"
        },
        {
          "name": "accept_rust_owned_local_container",
          "id": "translation.accept-rust-owned-local-container"
        },
        {
          "name": "acceptRustOwnedLocalRecordValue",
          "id": "translation.accept-rust-owned-local-record-value"
        },
        {
          "name": "accept_rust_owned_local_record_value",
          "id": "translation.accept-rust-owned-local-record-value"
        },
        {
          "name": "acceptRustOwnedLocalSourceSpan",
          "id": "translation.accept-rust-owned-local-source-span"
        },
        {
          "name": "accept_rust_owned_local_source_span",
          "id": "translation.accept-rust-owned-local-source-span"
        },
        {
          "name": "acceptRustOwnedLocalTextUnits",
          "id": "translation.accept-rust-owned-local-text-units"
        },
        {
          "name": "accept_rust_owned_local_text_units",
          "id": "translation.accept-rust-owned-local-text-units"
        },
        {
          "name": "renderRustOwnedLocalTextInput",
          "id": "translation.render-rust-owned-local-text-input"
        },
        {
          "name": "render_rust_owned_local_text_input",
          "id": "translation.render-rust-owned-local-text-input"
        },
        {
          "name": "readTextCertificateKnownSymbol",
          "id": "repository.read-text-certificate-known-symbol"
        },
        {
          "name": "read_text_certificate_known_symbol",
          "id": "repository.read-text-certificate-known-symbol"
        },
        {
          "name": "acceptTextCertificateStorage",
          "id": "repository.accept-text-certificate-storage"
        },
        {
          "name": "accept_text_certificate_storage",
          "id": "repository.accept-text-certificate-storage"
        },
        {
          "name": "findTextCertificateRow",
          "id": "repository.find-text-certificate-row"
        },
        {
          "name": "find_text_certificate_row",
          "id": "repository.find-text-certificate-row"
        },
        {
          "name": "acceptTextCertificateNumbers",
          "id": "repository.accept-text-certificate-numbers"
        },
        {
          "name": "accept_text_certificate_numbers",
          "id": "repository.accept-text-certificate-numbers"
        },
        {
          "name": "acceptTextCertificateIdentities",
          "id": "repository.accept-text-certificate-identities"
        },
        {
          "name": "accept_text_certificate_identities",
          "id": "repository.accept-text-certificate-identities"
        },
        {
          "name": "acceptTextCertificateName",
          "id": "repository.accept-text-certificate-name"
        },
        {
          "name": "accept_text_certificate_name",
          "id": "repository.accept-text-certificate-name"
        },
        {
          "name": "readTextCertificateDomain",
          "id": "repository.read-text-certificate-domain"
        },
        {
          "name": "read_text_certificate_domain",
          "id": "repository.read-text-certificate-domain"
        },
        {
          "name": "readTextCertificateType",
          "id": "repository.read-text-certificate-type"
        },
        {
          "name": "read_text_certificate_type",
          "id": "repository.read-text-certificate-type"
        },
        {
          "name": "readTextCertificatePrefix",
          "id": "repository.read-text-certificate-prefix"
        },
        {
          "name": "read_text_certificate_prefix",
          "id": "repository.read-text-certificate-prefix"
        },
        {
          "name": "readTextCertificateSuffix",
          "id": "repository.read-text-certificate-suffix"
        },
        {
          "name": "read_text_certificate_suffix",
          "id": "repository.read-text-certificate-suffix"
        },
        {
          "name": "readTextCertificateOperation",
          "id": "repository.read-text-certificate-operation"
        },
        {
          "name": "read_text_certificate_operation",
          "id": "repository.read-text-certificate-operation"
        },
        {
          "name": "readTextCertificateDomainName",
          "id": "repository.read-text-certificate-domain-name"
        },
        {
          "name": "read_text_certificate_domain_name",
          "id": "repository.read-text-certificate-domain-name"
        },
        {
          "name": "acceptTextCertificateAllocation",
          "id": "repository.accept-text-certificate-allocation"
        },
        {
          "name": "accept_text_certificate_allocation",
          "id": "repository.accept-text-certificate-allocation"
        },
        {
          "name": "matchTextCertificateSyntax",
          "id": "repository.match-text-certificate-syntax"
        },
        {
          "name": "match_text_certificate_syntax",
          "id": "repository.match-text-certificate-syntax"
        },
        {
          "name": "acceptTextCertificateToken",
          "id": "repository.accept-text-certificate-token"
        },
        {
          "name": "accept_text_certificate_token",
          "id": "repository.accept-text-certificate-token"
        },
        {
          "name": "acceptTextCertificateOperation",
          "id": "repository.accept-text-certificate-operation"
        },
        {
          "name": "accept_text_certificate_operation",
          "id": "repository.accept-text-certificate-operation"
        },
        {
          "name": "acceptTextCertificateTokenOperation",
          "id": "repository.accept-text-certificate-token-operation"
        },
        {
          "name": "accept_text_certificate_token_operation",
          "id": "repository.accept-text-certificate-token-operation"
        },
        {
          "name": "acceptTextCertificateCondition",
          "id": "repository.accept-text-certificate-condition"
        },
        {
          "name": "accept_text_certificate_condition",
          "id": "repository.accept-text-certificate-condition"
        },
        {
          "name": "checkRustTextCertificate",
          "id": "repository.check-rust-text-certificate"
        },
        {
          "name": "check_rust_text_certificate",
          "id": "repository.check-rust-text-certificate"
        }
      ]
    }
  },
  {
    "inventory": "portable text certificate translated control results",
    "files": {
      "experiments/rust-text-certificate/data/translation.lino": [
        {
          "name": "ml_acceptRustOwnedLocalTextUnits_loop1_result",
          "id": "repository.rust-owned-local-text-scalar-sequence-control-result"
        },
        {
          "name": "MlAcceptRustOwnedLocalTextUnitsLoop1Result",
          "id": "repository.rust-owned-local-text-scalar-sequence-control-result"
        },
        {
          "name": "ml_acceptTextCertificateStorage_loop5_result",
          "id": "repository.text-certificate-symbol-dictionary-storage-control-result"
        },
        {
          "name": "MlAcceptTextCertificateStorageLoop5Result",
          "id": "repository.text-certificate-symbol-dictionary-storage-control-result"
        },
        {
          "name": "ml_acceptTextCertificateStorage_loop7_result",
          "id": "repository.text-certificate-prior-symbol-uniqueness-control-result"
        },
        {
          "name": "MlAcceptTextCertificateStorageLoop7Result",
          "id": "repository.text-certificate-prior-symbol-uniqueness-control-result"
        },
        {
          "name": "ml_acceptTextCertificateStorage_join6",
          "id": "repository.text-certificate-fixed-symbol-comparison-control-result"
        },
        {
          "name": "MlAcceptTextCertificateStorageJoin6",
          "id": "repository.text-certificate-fixed-symbol-comparison-control-result"
        },
        {
          "name": "ml_acceptTextCertificateStorage_loop9_result",
          "id": "repository.text-certificate-fixed-symbol-units-control-result"
        },
        {
          "name": "MlAcceptTextCertificateStorageLoop9Result",
          "id": "repository.text-certificate-fixed-symbol-units-control-result"
        },
        {
          "name": "ml_acceptTextCertificateStorage_loop4_result",
          "id": "repository.text-certificate-structural-identity-coverage-control-result"
        },
        {
          "name": "MlAcceptTextCertificateStorageLoop4Result",
          "id": "repository.text-certificate-structural-identity-coverage-control-result"
        },
        {
          "name": "ml_acceptTextCertificateStorage_join10",
          "id": "repository.text-certificate-role-occurrence-count-control-result"
        },
        {
          "name": "MlAcceptTextCertificateStorageJoin10",
          "id": "repository.text-certificate-role-occurrence-count-control-result"
        },
        {
          "name": "ml_acceptTextCertificateStorage_loop3_result",
          "id": "repository.text-certificate-source-position-storage-ranges-control-result"
        },
        {
          "name": "MlAcceptTextCertificateStorageLoop3Result",
          "id": "repository.text-certificate-source-position-storage-ranges-control-result"
        },
        {
          "name": "ml_acceptTextCertificateStorage_loop2_result",
          "id": "repository.text-certificate-operation-part-ranges-control-result"
        },
        {
          "name": "MlAcceptTextCertificateStorageLoop2Result",
          "id": "repository.text-certificate-operation-part-ranges-control-result"
        },
        {
          "name": "ml_findTextCertificateRow_loop18_result",
          "id": "repository.text-certificate-unique-row-search-control-result"
        },
        {
          "name": "MlFindTextCertificateRowLoop18Result",
          "id": "repository.text-certificate-unique-row-search-control-result"
        },
        {
          "name": "ml_acceptTextCertificateNumbers_loop19_result",
          "id": "repository.text-certificate-numeric-domain-scan-control-result"
        },
        {
          "name": "MlAcceptTextCertificateNumbersLoop19Result",
          "id": "repository.text-certificate-numeric-domain-scan-control-result"
        },
        {
          "name": "ml_acceptTextCertificateIdentities_loop20_result",
          "id": "repository.text-certificate-row-identity-scan-control-result"
        },
        {
          "name": "MlAcceptTextCertificateIdentitiesLoop20Result",
          "id": "repository.text-certificate-row-identity-scan-control-result"
        },
        {
          "name": "ml_acceptTextCertificateName_loop21_result",
          "id": "repository.text-certificate-name-character-scan-control-result"
        },
        {
          "name": "MlAcceptTextCertificateNameLoop21Result",
          "id": "repository.text-certificate-name-character-scan-control-result"
        },
        {
          "name": "ml_readTextCertificateDomain_loop22_result",
          "id": "repository.text-certificate-reference-layer-scan-control-result"
        },
        {
          "name": "MlReadTextCertificateDomainLoop22Result",
          "id": "repository.text-certificate-reference-layer-scan-control-result"
        },
        {
          "name": "ml_matchTextCertificateSyntax_loop23_result",
          "id": "repository.text-certificate-fixed-syntax-scan-control-result"
        },
        {
          "name": "MlMatchTextCertificateSyntaxLoop23Result",
          "id": "repository.text-certificate-fixed-syntax-scan-control-result"
        },
        {
          "name": "ml_acceptTextCertificateOperation_loop24_result",
          "id": "repository.text-certificate-operation-part-comparison-control-result"
        },
        {
          "name": "MlAcceptTextCertificateOperationLoop24Result",
          "id": "repository.text-certificate-operation-part-comparison-control-result"
        },
        {
          "name": "ml_acceptTextCertificateTokenOperation_loop25_result",
          "id": "repository.text-certificate-operation-token-comparison-control-result"
        },
        {
          "name": "MlAcceptTextCertificateTokenOperationLoop25Result",
          "id": "repository.text-certificate-operation-token-comparison-control-result"
        },
        {
          "name": "ml_checkRustTextCertificate_loop27_result",
          "id": "repository.text-certificate-complete-function-scan-control-result"
        },
        {
          "name": "MlCheckRustTextCertificateLoop27Result",
          "id": "repository.text-certificate-complete-function-scan-control-result"
        },
        {
          "name": "ml_checkRustTextCertificate_join31",
          "id": "repository.text-certificate-input-condition-selection-control-result"
        },
        {
          "name": "MlCheckRustTextCertificateJoin31",
          "id": "repository.text-certificate-input-condition-selection-control-result"
        },
        {
          "name": "ml_checkRustTextCertificate_join30",
          "id": "repository.text-certificate-body-domain-selection-control-result"
        },
        {
          "name": "MlCheckRustTextCertificateJoin30",
          "id": "repository.text-certificate-body-domain-selection-control-result"
        },
        {
          "name": "ml_checkRustTextCertificate_loop29_result",
          "id": "repository.text-certificate-function-parameter-scan-control-result"
        },
        {
          "name": "MlCheckRustTextCertificateLoop29Result",
          "id": "repository.text-certificate-function-parameter-scan-control-result"
        },
        {
          "name": "ml_checkRustTextCertificate_loop28_result",
          "id": "repository.text-certificate-prior-declaration-name-scan-control-result"
        },
        {
          "name": "MlCheckRustTextCertificateLoop28Result",
          "id": "repository.text-certificate-prior-declaration-name-scan-control-result"
        },
        {
          "name": "ml_checkRustTextCertificate_loop26_result",
          "id": "repository.text-certificate-symbol-unit-bound-scan-control-result"
        },
        {
          "name": "MlCheckRustTextCertificateLoop26Result",
          "id": "repository.text-certificate-symbol-unit-bound-scan-control-result"
        }
      ]
    }
  },
  {
    "inventory": "portable text certificate translated continuations",
    "files": {
      "experiments/rust-text-certificate/data/translation.lino": [
        {
          "name": "ml_acceptRustOwnedLocalTextUnits_loop1",
          "id": "repository.continue-rust-owned-local-text-scalar-sequence"
        },
        {
          "name": "ml_accept_rust_owned_local_text_units_loop1",
          "id": "repository.continue-rust-owned-local-text-scalar-sequence"
        },
        {
          "name": "ml_acceptTextCertificateStorage_loop8",
          "id": "repository.continue-text-certificate-symbol-unit-comparison"
        },
        {
          "name": "ml_accept_text_certificate_storage_loop8",
          "id": "repository.continue-text-certificate-symbol-unit-comparison"
        },
        {
          "name": "ml_acceptTextCertificateStorage_loop7",
          "id": "repository.continue-text-certificate-prior-symbol-uniqueness"
        },
        {
          "name": "ml_accept_text_certificate_storage_loop7",
          "id": "repository.continue-text-certificate-prior-symbol-uniqueness"
        },
        {
          "name": "ml_acceptTextCertificateStorage_loop9",
          "id": "repository.continue-text-certificate-fixed-symbol-units"
        },
        {
          "name": "ml_accept_text_certificate_storage_loop9",
          "id": "repository.continue-text-certificate-fixed-symbol-units"
        },
        {
          "name": "ml_acceptTextCertificateStorage_loop5",
          "id": "repository.continue-text-certificate-symbol-dictionary-storage"
        },
        {
          "name": "ml_accept_text_certificate_storage_loop5",
          "id": "repository.continue-text-certificate-symbol-dictionary-storage"
        },
        {
          "name": "ml_acceptTextCertificateStorage_loop11",
          "id": "repository.continue-text-certificate-token-identity-count"
        },
        {
          "name": "ml_accept_text_certificate_storage_loop11",
          "id": "repository.continue-text-certificate-token-identity-count"
        },
        {
          "name": "ml_acceptTextCertificateStorage_loop12",
          "id": "repository.continue-text-certificate-declaration-identity-count"
        },
        {
          "name": "ml_accept_text_certificate_storage_loop12",
          "id": "repository.continue-text-certificate-declaration-identity-count"
        },
        {
          "name": "ml_acceptTextCertificateStorage_loop13",
          "id": "repository.continue-text-certificate-parameter-identity-count"
        },
        {
          "name": "ml_accept_text_certificate_storage_loop13",
          "id": "repository.continue-text-certificate-parameter-identity-count"
        },
        {
          "name": "ml_acceptTextCertificateStorage_loop14",
          "id": "repository.continue-text-certificate-signature-identity-count"
        },
        {
          "name": "ml_accept_text_certificate_storage_loop14",
          "id": "repository.continue-text-certificate-signature-identity-count"
        },
        {
          "name": "ml_acceptTextCertificateStorage_loop15",
          "id": "repository.continue-text-certificate-source-position-identity-count"
        },
        {
          "name": "ml_accept_text_certificate_storage_loop15",
          "id": "repository.continue-text-certificate-source-position-identity-count"
        },
        {
          "name": "ml_acceptTextCertificateStorage_loop16",
          "id": "repository.continue-text-certificate-expression-identity-count"
        },
        {
          "name": "ml_accept_text_certificate_storage_loop16",
          "id": "repository.continue-text-certificate-expression-identity-count"
        },
        {
          "name": "ml_acceptTextCertificateStorage_loop17",
          "id": "repository.continue-text-certificate-operation-identity-count"
        },
        {
          "name": "ml_accept_text_certificate_storage_loop17",
          "id": "repository.continue-text-certificate-operation-identity-count"
        },
        {
          "name": "ml_acceptTextCertificateStorage_loop4",
          "id": "repository.continue-text-certificate-structural-identity-coverage"
        },
        {
          "name": "ml_accept_text_certificate_storage_loop4",
          "id": "repository.continue-text-certificate-structural-identity-coverage"
        },
        {
          "name": "ml_acceptTextCertificateStorage_loop3",
          "id": "repository.continue-text-certificate-source-position-storage-ranges"
        },
        {
          "name": "ml_accept_text_certificate_storage_loop3",
          "id": "repository.continue-text-certificate-source-position-storage-ranges"
        },
        {
          "name": "ml_acceptTextCertificateStorage_loop2",
          "id": "repository.continue-text-certificate-operation-part-ranges"
        },
        {
          "name": "ml_accept_text_certificate_storage_loop2",
          "id": "repository.continue-text-certificate-operation-part-ranges"
        },
        {
          "name": "ml_findTextCertificateRow_loop18",
          "id": "repository.continue-text-certificate-unique-row-search"
        },
        {
          "name": "ml_find_text_certificate_row_loop18",
          "id": "repository.continue-text-certificate-unique-row-search"
        },
        {
          "name": "ml_acceptTextCertificateNumbers_loop19",
          "id": "repository.continue-text-certificate-numeric-domain-scan"
        },
        {
          "name": "ml_accept_text_certificate_numbers_loop19",
          "id": "repository.continue-text-certificate-numeric-domain-scan"
        },
        {
          "name": "ml_acceptTextCertificateIdentities_loop20",
          "id": "repository.continue-text-certificate-row-identity-scan"
        },
        {
          "name": "ml_accept_text_certificate_identities_loop20",
          "id": "repository.continue-text-certificate-row-identity-scan"
        },
        {
          "name": "ml_acceptTextCertificateName_loop21",
          "id": "repository.continue-text-certificate-name-character-scan"
        },
        {
          "name": "ml_accept_text_certificate_name_loop21",
          "id": "repository.continue-text-certificate-name-character-scan"
        },
        {
          "name": "ml_readTextCertificateDomain_loop22",
          "id": "repository.continue-text-certificate-reference-layer-scan"
        },
        {
          "name": "ml_read_text_certificate_domain_loop22",
          "id": "repository.continue-text-certificate-reference-layer-scan"
        },
        {
          "name": "ml_matchTextCertificateSyntax_loop23",
          "id": "repository.continue-text-certificate-fixed-syntax-scan"
        },
        {
          "name": "ml_match_text_certificate_syntax_loop23",
          "id": "repository.continue-text-certificate-fixed-syntax-scan"
        },
        {
          "name": "ml_acceptTextCertificateOperation_loop24",
          "id": "repository.continue-text-certificate-operation-part-comparison"
        },
        {
          "name": "ml_accept_text_certificate_operation_loop24",
          "id": "repository.continue-text-certificate-operation-part-comparison"
        },
        {
          "name": "ml_acceptTextCertificateTokenOperation_loop25",
          "id": "repository.continue-text-certificate-operation-token-comparison"
        },
        {
          "name": "ml_accept_text_certificate_token_operation_loop25",
          "id": "repository.continue-text-certificate-operation-token-comparison"
        },
        {
          "name": "ml_checkRustTextCertificate_loop29",
          "id": "repository.continue-text-certificate-function-parameter-scan"
        },
        {
          "name": "ml_check_rust_text_certificate_loop29",
          "id": "repository.continue-text-certificate-function-parameter-scan"
        },
        {
          "name": "ml_checkRustTextCertificate_loop28",
          "id": "repository.continue-text-certificate-prior-declaration-name-scan"
        },
        {
          "name": "ml_check_rust_text_certificate_loop28",
          "id": "repository.continue-text-certificate-prior-declaration-name-scan"
        },
        {
          "name": "ml_checkRustTextCertificate_loop27",
          "id": "repository.continue-text-certificate-complete-function-scan"
        },
        {
          "name": "ml_check_rust_text_certificate_loop27",
          "id": "repository.continue-text-certificate-complete-function-scan"
        },
        {
          "name": "ml_checkRustTextCertificate_loop26",
          "id": "repository.continue-text-certificate-symbol-unit-bound-scan"
        },
        {
          "name": "ml_check_rust_text_certificate_loop26",
          "id": "repository.continue-text-certificate-symbol-unit-bound-scan"
        }
      ]
    }
  },
  {
    "inventory": "portable text source certificate controls",
    "files": {
      "experiments/rust-text-certificate/kernel.test.mjs": [
        {
          "name": "accepted",
          "id": "repository.require-text-certificate-test-acceptance"
        },
        {
          "name": "refused",
          "id": "repository.require-text-certificate-test-refusal"
        },
        {
          "name": "retarget",
          "id": "repository.change-text-certificate-test-input"
        },
        {
          "name": "runValue",
          "id": "repository.execute-text-certificate-test-artifact"
        },
        {
          "name": "field",
          "id": "repository.read-text-certificate-test-field"
        },
        {
          "name": "replaceField",
          "id": "repository.replace-text-certificate-test-field"
        },
        {
          "name": "rawProof",
          "id": "repository.read-text-certificate-test-proof"
        },
        {
          "name": "rows",
          "id": "repository.read-text-certificate-test-rows"
        }
      ]
    }
  },
  {
    "inventory": "shared original source oracle mechanics",
    "files": {
      "experiments/source-native-observations.mjs": [
        {
          "name": "sha256",
          "id": "repository.hash-rust-text-native-oracle-content"
        },
        {
          "name": "checkFiles",
          "id": "repository.check-rust-text-native-oracle-files"
        },
        {
          "name": "runProcess",
          "id": "repository.run-rust-text-native-oracle-process"
        }
      ]
    }
  },
  {
    "inventory": "shared original source oracle factory",
    "files": {
      "experiments/source-native-observations.mjs": [
        {
          "name": "createSourceNativeObservationRunner",
          "id": "repository.create-source-native-observation-runner"
        }
      ]
    }
  },
  {
    "inventory": "shared original source oracle execution",
    "files": {
      "experiments/source-native-observations.mjs": [
        {
          "name": "runSourceNativeObservations",
          "id": "repository.run-source-native-observations"
        }
      ]
    }
  },
  {
    "inventory": "raw binding original source oracle operations",
    "files": {
      "experiments/rust-raw-binding-identifiers/native-observations.mjs": [
        {
          "name": "verifyRustRawBindingIdentifierModel",
          "id": "repository.verify-raw-binding-native-oracle-model"
        },
        {
          "name": "rustRawBindingIdentifierNativeMode",
          "id": "repository.read-raw-binding-native-oracle-mode"
        },
        {
          "name": "archiveSource",
          "id": "repository.reload-raw-binding-native-oracle-source"
        },
        {
          "name": "archiveEdit",
          "id": "repository.reload-raw-binding-native-oracle-edit"
        },
        {
          "name": "primitive",
          "id": "repository.read-raw-binding-native-primitive-mode"
        },
        {
          "name": "argumentValue",
          "id": "repository.read-raw-binding-native-argument"
        },
        {
          "name": "canonicalValue",
          "id": "repository.render-raw-binding-native-observation-value"
        },
        {
          "name": "publicTarget",
          "id": "repository.check-raw-binding-native-consumer-target"
        },
        {
          "name": "consumerSource",
          "id": "repository.create-raw-binding-native-consumer-source"
        },
        {
          "name": "preparedEntry",
          "id": "repository.create-raw-binding-native-observation-entry"
        },
        {
          "name": "receipt",
          "id": "repository.create-raw-binding-native-source-receipt"
        },
        {
          "name": "editedSources",
          "id": "repository.create-raw-binding-native-oracle-edits"
        },
        {
          "name": "planRustRawBindingIdentifierNative",
          "id": "repository.plan-raw-binding-native-observations"
        },
        {
          "name": "parseObservations",
          "id": "repository.read-raw-binding-native-observation-stream"
        },
        {
          "name": "runRustRawBindingIdentifierNative",
          "id": "repository.run-raw-binding-native-observations"
        }
      ]
    }
  },
  {
    "inventory": "raw binding original source oracle data",
    "files": {
      "experiments/rust-raw-binding-identifiers/native-observations.mjs": [
        {
          "name": "RUST_RAW_BINDING_IDENTIFIER_MODEL_COMMIT",
          "id": "repository.raw-binding-native-oracle-model-revision"
        },
        {
          "name": "modelDigests",
          "id": "repository.raw-binding-native-oracle-model-digest-map"
        },
        {
          "name": "checkedPlans",
          "id": "repository.raw-binding-native-authenticated-plan-set"
        }
      ]
    }
  },
  {
    "inventory": "raw binding native oracle actual shared provider sha256",
    "files": {
      "experiments/rust-raw-binding-identifiers/native-observations.mjs": [
        {
          "name": "sha256",
          "id": "repository.hash-rust-text-native-oracle-content"
        }
      ]
    }
  },
  {
    "inventory": "raw binding native oracle actual shared provider textBytes",
    "files": {
      "experiments/rust-raw-binding-identifiers/native-observations.mjs": [
        {
          "name": "textBytes",
          "id": "repository.read-rust-text-native-oracle-bytes"
        }
      ]
    }
  },
  {
    "inventory": "raw binding native oracle actual shared provider freeze",
    "files": {
      "experiments/rust-raw-binding-identifiers/native-observations.mjs": [
        {
          "name": "freeze",
          "id": "repository.freeze-rust-text-native-observation-plan"
        }
      ]
    }
  },
  {
    "inventory": "raw binding native oracle actual shared provider fixedBounds",
    "files": {
      "experiments/rust-raw-binding-identifiers/native-observations.mjs": [
        {
          "name": "fixedBounds",
          "id": "translation.read-fixed-width-integer-bounds"
        }
      ]
    }
  },
  {
    "inventory": "raw binding native oracle actual shared provider createSourceNativeObservationRunner",
    "files": {
      "experiments/rust-raw-binding-identifiers/native-observations.mjs": [
        {
          "name": "createSourceNativeObservationRunner",
          "id": "repository.create-source-native-observation-runner"
        }
      ]
    }
  },
  {
    "inventory": "raw binding native oracle actual shared provider createRustRawBindingIdentifierLinks",
    "files": {
      "experiments/rust-raw-binding-identifiers/native-observations.mjs": [
        {
          "name": "createRustRawBindingIdentifierLinks",
          "id": "repository.create-rust-raw-binding-identifier-links"
        }
      ]
    }
  },
  {
    "inventory": "raw binding native oracle actual shared provider decodeRustRawBindingIdentifierLinks",
    "files": {
      "experiments/rust-raw-binding-identifiers/native-observations.mjs": [
        {
          "name": "decodeRustRawBindingIdentifierLinks",
          "id": "repository.decode-rust-raw-binding-identifier-links"
        }
      ]
    }
  },
  {
    "inventory": "raw binding native oracle actual shared provider encodeRustRawBindingIdentifierLinks",
    "files": {
      "experiments/rust-raw-binding-identifiers/native-observations.mjs": [
        {
          "name": "encodeRustRawBindingIdentifierLinks",
          "id": "repository.encode-rust-raw-binding-identifier-links"
        }
      ]
    }
  },
  {
    "inventory": "raw binding native oracle actual shared provider checkRustRawBindingIdentifierInvocation",
    "files": {
      "experiments/rust-raw-binding-identifiers/native-observations.mjs": [
        {
          "name": "checkRustRawBindingIdentifierInvocation",
          "id": "repository.check-rust-raw-binding-identifier-invocation"
        }
      ]
    }
  },
  {
    "inventory": "raw binding native oracle actual shared provider emitRustRawBindingIdentifiers",
    "files": {
      "experiments/rust-raw-binding-identifiers/native-observations.mjs": [
        {
          "name": "emitRustRawBindingIdentifiers",
          "id": "repository.emit-rust-raw-binding-identifiers"
        }
      ]
    }
  },
  {
    "inventory": "raw binding native oracle actual shared provider validName",
    "files": {
      "experiments/rust-raw-binding-identifiers/native-observations.mjs": [
        {
          "name": "validName",
          "id": "repository.accept-raw-binding-name-spelling"
        }
      ]
    }
  },
  {
    "inventory": "raw binding native oracle actual shared provider readRawBindingIdentifierSyntax",
    "files": {
      "experiments/rust-raw-binding-identifiers/native-observations.mjs": [
        {
          "name": "readRawBindingIdentifierSyntax",
          "id": "repository.read-raw-binding-identifier-syntax"
        }
      ]
    }
  },
  {
    "inventory": "raw binding injected original source oracle controls",
    "files": {
      "experiments/rust-raw-binding-identifiers/native-observations.test.mjs": [
        {
          "name": "stream",
          "id": "repository.create-raw-binding-native-test-observation-stream"
        },
        {
          "name": "fakeProcess",
          "id": "repository.create-raw-binding-native-test-process"
        },
        {
          "name": "injected",
          "id": "repository.run-raw-binding-native-injected-contract"
        },
        {
          "name": "fails",
          "id": "repository.require-raw-binding-native-test-failure"
        }
      ]
    }
  },
  {
    "inventory": "raw binding original source oracle CI gates",
    "files": {
      "js/tests/ci-workflow-structure.test.js": [
        {
          "name": "checkRustRawBindingNativeSteps",
          "id": "repository.check-raw-binding-native-oracle-steps"
        }
      ]
    }
  },
  {
    "inventory": "whole text certificate source oracle operations",
    "files": {
      "experiments/rust-text-certificate/native-observations.mjs": [
        {
          "name": "verifyRustTextCertificateModel",
          "id": "repository.verify-text-certificate-native-oracle-model"
        },
        {
          "name": "rustTextCertificateNativeMode",
          "id": "repository.read-text-certificate-native-oracle-mode"
        },
        {
          "name": "numberBits",
          "id": "repository.encode-text-certificate-native-number-bits"
        },
        {
          "name": "bitsNumber",
          "id": "repository.decode-text-certificate-native-number-bits"
        },
        {
          "name": "artifactReplay",
          "id": "repository.reload-text-certificate-native-artifact"
        },
        {
          "name": "nativeHeader",
          "id": "repository.check-text-certificate-native-header"
        },
        {
          "name": "kernelObservation",
          "id": "repository.create-text-certificate-native-javascript-oracle"
        },
        {
          "name": "originalTransports",
          "id": "repository.create-text-certificate-native-original-transports"
        },
        {
          "name": "corruptedTransports",
          "id": "repository.create-text-certificate-native-corruptions"
        },
        {
          "name": "consumerSource",
          "id": "repository.create-text-certificate-native-consumer-source"
        },
        {
          "name": "planRustTextCertificateNative",
          "id": "repository.plan-text-certificate-native-observations"
        },
        {
          "name": "parseObservations",
          "id": "repository.read-text-certificate-native-observation-stream"
        },
        {
          "name": "runRustTextCertificateNative",
          "id": "repository.run-text-certificate-native-observations"
        }
      ]
    }
  },
  {
    "inventory": "whole text certificate source oracle data RUST_TEXT_CERTIFICATE_MODEL_COMMIT",
    "files": {
      "experiments/rust-text-certificate/native-observations.mjs": [
        {
          "name": "RUST_TEXT_CERTIFICATE_MODEL_COMMIT",
          "id": "repository.text-certificate-native-oracle-model-revision"
        }
      ]
    }
  },
  {
    "inventory": "whole text certificate source oracle data modelDigests",
    "files": {
      "experiments/rust-text-certificate/native-observations.mjs": [
        {
          "name": "modelDigests",
          "id": "repository.text-certificate-native-oracle-model-digest-map"
        }
      ]
    }
  },
  {
    "inventory": "whole text certificate source oracle data checkedPlans",
    "files": {
      "experiments/rust-text-certificate/native-observations.mjs": [
        {
          "name": "checkedPlans",
          "id": "repository.text-certificate-native-authenticated-plan-set"
        }
      ]
    }
  },
  {
    "inventory": "whole text certificate source oracle data inputBits",
    "files": {
      "experiments/rust-text-certificate/native-observations.mjs": [
        {
          "name": "inputBits",
          "id": "repository.text-certificate-native-input-bit-vectors"
        }
      ]
    }
  },
  {
    "inventory": "whole text certificate source oracle data resultBits",
    "files": {
      "experiments/rust-text-certificate/native-observations.mjs": [
        {
          "name": "resultBits",
          "id": "repository.text-certificate-native-result-bit-vector"
        }
      ]
    }
  },
  {
    "inventory": "whole text certificate source oracle data tableOrder",
    "files": {
      "experiments/rust-text-certificate/native-observations.mjs": [
        {
          "name": "tableOrder",
          "id": "repository.text-certificate-native-argument-order"
        }
      ]
    }
  },
  {
    "inventory": "whole text certificate source oracle data outputRoles",
    "files": {
      "experiments/rust-text-certificate/native-observations.mjs": [
        {
          "name": "outputRoles",
          "id": "repository.text-certificate-native-output-roles"
        }
      ]
    }
  },
  {
    "inventory": "whole text certificate source oracle data inputTables",
    "files": {
      "experiments/rust-text-certificate/native-observations.mjs": [
        {
          "name": "inputTables",
          "id": "repository.text-certificate-native-input-table-receipts"
        }
      ]
    }
  },
  {
    "inventory": "whole text certificate source oracle actual provider sha256",
    "files": {
      "experiments/rust-text-certificate/native-observations.mjs": [
        {
          "name": "sha256",
          "id": "repository.hash-rust-text-native-oracle-content"
        }
      ]
    }
  },
  {
    "inventory": "whole text certificate source oracle actual provider textBytes",
    "files": {
      "experiments/rust-text-certificate/native-observations.mjs": [
        {
          "name": "textBytes",
          "id": "repository.read-rust-text-native-oracle-bytes"
        }
      ]
    }
  },
  {
    "inventory": "whole text certificate source oracle actual provider freeze",
    "files": {
      "experiments/rust-text-certificate/native-observations.mjs": [
        {
          "name": "freeze",
          "id": "repository.freeze-rust-text-native-observation-plan"
        }
      ]
    }
  },
  {
    "inventory": "whole text certificate source oracle actual provider createSourceNativeObservationRunner",
    "files": {
      "experiments/rust-text-certificate/native-observations.mjs": [
        {
          "name": "createSourceNativeObservationRunner",
          "id": "repository.create-source-native-observation-runner"
        }
      ]
    }
  },
  {
    "inventory": "whole text certificate source oracle actual provider bundledTextCertificateSource",
    "files": {
      "experiments/rust-text-certificate/native-observations.mjs": [
        {
          "name": "bundledTextCertificateSource",
          "id": "repository.bundle-text-certificate-source"
        }
      ]
    }
  },
  {
    "inventory": "whole text certificate source oracle actual provider checkRustTextCertificate",
    "files": {
      "experiments/rust-text-certificate/native-observations.mjs": [
        {
          "name": "checkRustTextCertificate",
          "id": "repository.check-rust-text-certificate"
        }
      ]
    }
  },
  {
    "inventory": "whole text certificate source oracle actual provider createPortableRustTextCertificate",
    "files": {
      "experiments/rust-text-certificate/native-observations.mjs": [
        {
          "name": "createPortableRustTextCertificate",
          "id": "repository.create-portable-rust-text-certificate"
        }
      ]
    }
  },
  {
    "inventory": "whole text certificate source oracle actual provider encodePortableRustTextCertificate",
    "files": {
      "experiments/rust-text-certificate/native-observations.mjs": [
        {
          "name": "encodePortableRustTextCertificate",
          "id": "repository.encode-portable-rust-text-certificate"
        }
      ]
    }
  },
  {
    "inventory": "whole text certificate source oracle actual provider decodePortableRustTextCertificate",
    "files": {
      "experiments/rust-text-certificate/native-observations.mjs": [
        {
          "name": "decodePortableRustTextCertificate",
          "id": "repository.decode-portable-rust-text-certificate"
        }
      ]
    }
  },
  {
    "inventory": "whole text certificate source oracle test digest",
    "files": {
      "experiments/rust-text-certificate/native-observations.test.mjs": [
        {
          "name": "digest",
          "id": "repository.hash-rust-text-native-oracle-content"
        }
      ]
    }
  },
  {
    "inventory": "whole text certificate injected source oracle controls",
    "files": {
      "experiments/rust-text-certificate/native-observations.test.mjs": [
        {
          "name": "fakeProcess",
          "id": "repository.create-text-certificate-native-test-process"
        },
        {
          "name": "injected",
          "id": "repository.run-text-certificate-native-injected-contract"
        },
        {
          "name": "fails",
          "id": "repository.require-text-certificate-native-test-failure"
        }
      ]
    }
  }
];
const normalizeRustRawAndTextCertificateName=name=>name.replace(/([a-z0-9])([A-Z])/gu,'$1-$2').replaceAll('_','-').replaceAll(' ','-').toLowerCase().replace(/\bcst\b/gu,'concrete-source-tree').replace(/\bstr\b/gu,'string-slice').replace(/^retarget$/u,'change-target').replace(/^tokenize-/u,'read-tokens-').replace(/\bsha256\b/gu,'secure-hash-algorithm-two-hundred-fifty-six').replace(/\bml\b/gu,'meta-language').replace(/\b(loop|join)([0-9]+)\b/gu,(whole,kind,digits)=>{const words={1:'one',2:'two',3:'three',4:'four',5:'five',6:'six',7:'seven',8:'eight',9:'nine',10:'ten',11:'eleven',12:'twelve',13:'thirteen',14:'fourteen',15:'fifteen',16:'sixteen',17:'seventeen',18:'eighteen',19:'nineteen',20:'twenty',21:'twenty-one',22:'twenty-two',23:'twenty-three',24:'twenty-four',25:'twenty-five',26:'twenty-six',27:'twenty-seven',28:'twenty-eight',29:'twenty-nine',30:'thirty',31:'thirty-one'};return words[digits]?kind+'-'+words[digits]:whole;});
const RUST_RAW_AND_TEXT_CERTIFICATE_SOURCE_TEXTS=new Map();
const RUST_RAW_AND_TEXT_CERTIFICATE_NAME_INVENTORIES=RUST_RAW_AND_TEXT_CERTIFICATE_NAME_BINDINGS.map(({inventory,files})=>({
 inventory,files:Object.keys(files),
 extract:(raw,file)=>{let text=raw;if(file.endsWith('.lino')){const saved=RUST_RAW_AND_TEXT_CERTIFICATE_SOURCE_TEXTS.get(file);if(!saved||saved.raw!==raw)RUST_RAW_AND_TEXT_CERTIFICATE_SOURCE_TEXTS.set(file,{raw,text:JSON.stringify(readStructuredDataLino(raw))});text=RUST_RAW_AND_TEXT_CERTIFICATE_SOURCE_TEXTS.get(file).text;}return files[file].filter(({name})=>new RegExp('\\b'+name+'\\b','u').test(text)).map(({name})=>normalizeRustRawAndTextCertificateName(name));},
 recordOf:name=>Object.values(files).flat().find(entry=>normalizeRustRawAndTextCertificateName(entry.name)===name).id,
}));
// Actual CI-only complete raw binding selector source oracle identities.
const RAW_BINDING_RULE_ORACLE_NAME_BINDINGS=[
  {
    "inventory": "conditional raw binding rule native oracle operations",
    "files": {
      "experiments/rust-raw-binding-certificate-adapter/native-observations.mjs": [
        {
          "name": "verifyRustRawBindingRuleModel",
          "id": "repository.verify-raw-binding-rule-native-oracle-model"
        },
        {
          "name": "rustRawBindingRuleNativeMode",
          "id": "repository.read-raw-binding-rule-native-oracle-mode"
        },
        {
          "name": "selectorInventory",
          "id": "repository.read-raw-binding-rule-native-selector-inventory"
        },
        {
          "name": "boundaryCalls",
          "id": "repository.create-raw-binding-rule-native-boundary-calls"
        },
        {
          "name": "typedArgument",
          "id": "repository.encode-raw-binding-rule-native-argument"
        },
        {
          "name": "consumerSource",
          "id": "repository.create-raw-binding-rule-native-consumer-source"
        },
        {
          "name": "planRustRawBindingRuleNative",
          "id": "repository.plan-raw-binding-rule-native-observations"
        },
        {
          "name": "parseObservations",
          "id": "repository.read-raw-binding-rule-native-observation-stream"
        },
        {
          "name": "runRustRawBindingRuleNative",
          "id": "repository.run-raw-binding-rule-native-observations"
        }
      ]
    }
  },
  {
    "inventory": "conditional raw binding rule native oracle role RUST_RAW_BINDING_RULE_MODEL_COMMIT",
    "files": {
      "experiments/rust-raw-binding-certificate-adapter/native-observations.mjs": [
        {
          "name": "RUST_RAW_BINDING_RULE_MODEL_COMMIT",
          "id": "repository.raw-binding-rule-native-oracle-model-revision"
        }
      ]
    }
  },
  {
    "inventory": "conditional raw binding rule native oracle role digests",
    "files": {
      "experiments/rust-raw-binding-certificate-adapter/native-observations.mjs": [
        {
          "name": "digests",
          "id": "repository.raw-binding-rule-native-oracle-model-digest-map"
        }
      ]
    }
  },
  {
    "inventory": "conditional raw binding rule native oracle role checkedPlans",
    "files": {
      "experiments/rust-raw-binding-certificate-adapter/native-observations.mjs": [
        {
          "name": "checkedPlans",
          "id": "repository.raw-binding-rule-native-authenticated-plan-set"
        }
      ]
    }
  },
  {
    "inventory": "conditional raw binding rule native oracle role observationScope",
    "files": {
      "experiments/rust-raw-binding-certificate-adapter/native-observations.mjs": [
        {
          "name": "observationScope",
          "id": "repository.raw-binding-rule-native-conditional-observation-scope"
        }
      ]
    }
  },
  {
    "inventory": "conditional raw binding rule native oracle role declarations",
    "files": {
      "experiments/rust-raw-binding-certificate-adapter/native-observations.mjs": [
        {
          "name": "declarations",
          "id": "repository.raw-binding-rule-native-declaration-inventory"
        }
      ]
    }
  },
  {
    "inventory": "conditional raw binding rule native oracle role invocationCount",
    "files": {
      "experiments/rust-raw-binding-certificate-adapter/native-observations.mjs": [
        {
          "name": "invocationCount",
          "id": "repository.raw-binding-rule-native-invocation-plan-count"
        }
      ]
    }
  },
  {
    "inventory": "conditional selector oracle actual shared provider sha256",
    "files": {
      "experiments/rust-raw-binding-certificate-adapter/native-observations.mjs": [
        {
          "name": "sha256",
          "id": "repository.hash-rust-text-native-oracle-content"
        }
      ]
    }
  },
  {
    "inventory": "conditional selector oracle actual shared provider textBytes",
    "files": {
      "experiments/rust-raw-binding-certificate-adapter/native-observations.mjs": [
        {
          "name": "textBytes",
          "id": "repository.read-rust-text-native-oracle-bytes"
        }
      ]
    }
  },
  {
    "inventory": "conditional selector oracle actual shared provider freeze",
    "files": {
      "experiments/rust-raw-binding-certificate-adapter/native-observations.mjs": [
        {
          "name": "freeze",
          "id": "repository.freeze-rust-text-native-observation-plan"
        }
      ]
    }
  },
  {
    "inventory": "conditional selector oracle actual shared provider numberBits",
    "files": {
      "experiments/rust-raw-binding-certificate-adapter/native-observations.mjs": [
        {
          "name": "numberBits",
          "id": "repository.encode-text-certificate-native-number-bits"
        }
      ]
    }
  },
  {
    "inventory": "conditional selector oracle actual shared provider createSourceNativeObservationRunner",
    "files": {
      "experiments/rust-raw-binding-certificate-adapter/native-observations.mjs": [
        {
          "name": "createSourceNativeObservationRunner",
          "id": "repository.create-source-native-observation-runner"
        }
      ]
    }
  },
  {
    "inventory": "conditional selector oracle actual pure provider acceptRustRawBindingIdentifierModel",
    "files": {
      "experiments/rust-raw-binding-certificate-adapter/native-observations.mjs": [
        {
          "name": "acceptRustRawBindingIdentifierModel",
          "id": "repository.accept-rust-raw-binding-identifier-model"
        }
      ]
    }
  },
  {
    "inventory": "conditional selector oracle actual pure provider acceptRustRawBindingIdentifierName",
    "files": {
      "experiments/rust-raw-binding-certificate-adapter/native-observations.mjs": [
        {
          "name": "acceptRustRawBindingIdentifierName",
          "id": "repository.accept-rust-raw-binding-identifier-name"
        }
      ]
    }
  },
  {
    "inventory": "conditional selector oracle actual pure provider acceptRustRawBindingIdentifierType",
    "files": {
      "experiments/rust-raw-binding-certificate-adapter/native-observations.mjs": [
        {
          "name": "acceptRustRawBindingIdentifierType",
          "id": "repository.accept-rust-raw-binding-identifier-type"
        }
      ]
    }
  },
  {
    "inventory": "conditional selector oracle actual pure provider acceptRustRawBindingIdentifierClosure",
    "files": {
      "experiments/rust-raw-binding-certificate-adapter/native-observations.mjs": [
        {
          "name": "acceptRustRawBindingIdentifierClosure",
          "id": "repository.accept-rust-raw-binding-identifier-closure"
        }
      ]
    }
  },
  {
    "inventory": "conditional selector oracle actual pure provider acceptRustRawBindingIdentifierInvocationModel",
    "files": {
      "experiments/rust-raw-binding-certificate-adapter/native-observations.mjs": [
        {
          "name": "acceptRustRawBindingIdentifierInvocationModel",
          "id": "repository.accept-rust-raw-binding-identifier-invocation-model"
        }
      ]
    }
  },
  {
    "inventory": "conditional selector oracle actual pure provider acceptRustRawBindingIdentifierInput",
    "files": {
      "experiments/rust-raw-binding-certificate-adapter/native-observations.mjs": [
        {
          "name": "acceptRustRawBindingIdentifierInput",
          "id": "repository.accept-rust-raw-binding-identifier-input"
        }
      ]
    }
  },
  {
    "inventory": "conditional raw binding rule injected oracle controls",
    "files": {
      "experiments/rust-raw-binding-certificate-adapter/native-observations.test.mjs": [
        {
          "name": "fakeProcess",
          "id": "repository.create-raw-binding-rule-native-test-process"
        },
        {
          "name": "injected",
          "id": "repository.run-raw-binding-rule-native-injected-contract"
        },
        {
          "name": "rows",
          "id": "repository.read-raw-binding-rule-native-test-selector-rows"
        }
      ]
    }
  },
  {
    "inventory": "conditional selector oracle exact aggregate failure assertion",
    "files": {
      "experiments/rust-raw-binding-certificate-adapter/native-observations.test.mjs": [
        {
          "name": "fails",
          "id": "repository.require-text-certificate-native-test-failure"
        }
      ]
    }
  }
];
const normalizeRawBindingRuleOracleName=name=>name.replace(/([a-z0-9])([A-Z])/gu,'$1-$2').replaceAll('_','-').replaceAll(' ','-').toLowerCase().replace(/\bsha256\b/gu,'secure-hash-algorithm-two-hundred-fifty-six');
const RAW_BINDING_RULE_ORACLE_NAME_INVENTORIES=RAW_BINDING_RULE_ORACLE_NAME_BINDINGS.map(({inventory,files})=>({
 inventory,files:Object.keys(files),
 extract:(raw,file)=>files[file].filter(({name})=>new RegExp('\\b'+name+'\\b','u').test(raw)).map(({name})=>normalizeRawBindingRuleOracleName(name)),
 recordOf:name=>Object.values(files).flat().find(entry=>normalizeRawBindingRuleOracleName(entry.name)===name).id,
}));
// Actual isolated owned source adapters, library packaging and unchanged helper aliases.
const OWNED_SOURCE_ADAPTER_NAME_BINDINGS=[
  {
    "inventory": "owned text certificate adapter operations",
    "files": {
      "experiments/rust-text-certificate-adapter/model.mjs": [
        {
          "name": "readOwnedRustTextCertificate",
          "id": "repository.read-owned-rust-text-certificate"
        },
        {
          "name": "checkOwnedRustTextCertificate",
          "id": "repository.check-owned-rust-text-certificate"
        },
        {
          "name": "captureOwnedRustTextCertificate",
          "id": "repository.capture-owned-rust-text-certificate"
        },
        {
          "name": "emitOwnedRustTextCertificate",
          "id": "repository.emit-owned-rust-text-certificate"
        }
      ]
    }
  },
  {
    "inventory": "owned text source proof facet",
    "files": {
      "experiments/rust-text-certificate-adapter/policy.mjs": [
        {
          "name": "RUST_TEXT_VALUE_FACET",
          "id": "repository.owned-text-certificate-program-facet"
        },
        {
          "name": "sourceTextValueProjection",
          "id": "repository.owned-text-certificate-program-facet"
        }
      ]
    }
  },
  {
    "inventory": "owned text certificate adapter policy",
    "files": {
      "experiments/rust-text-certificate-adapter/policy.mjs": [
        {
          "name": "rustTextCertificateAdapterPolicy",
          "id": "repository.owned-text-certificate-adapter-policy"
        }
      ]
    }
  },
  {
    "inventory": "owned text original link transport",
    "files": {
      "experiments/rust-text-certificate-adapter/policy.mjs": [
        {
          "name": "rustTextCertificateIdentityTransport",
          "id": "repository.owned-text-certificate-identity-transport"
        }
      ]
    }
  },
  {
    "inventory": "owned text raw link boundary operations",
    "files": {
      "experiments/rust-text-certificate-adapter/raw-identity.mjs": [
        {
          "name": "decodeRustTextCertificateFacet",
          "id": "repository.decode-rust-text-certificate-facet"
        },
        {
          "name": "scan",
          "id": "repository.find-owned-text-certificate-markers"
        },
        {
          "name": "fieldName",
          "id": "repository.read-owned-text-certificate-field-name"
        }
      ]
    }
  },
  {
    "inventory": "exact link identity keys",
    "files": {
      "experiments/rust-text-certificate-adapter/raw-identity.mjs": [
        {
          "name": "key",
          "id": "repository.read-exact-link-identity-key"
        }
      ]
    }
  },
  {
    "inventory": "checked module overlay operations overlay.mjs",
    "files": {
      "experiments/rust-text-certificate-adapter/overlay.mjs": [
        {
          "name": "relocateCheckedModuleImports",
          "id": "repository.relocate-checked-module-imports"
        },
        {
          "name": "relocate",
          "id": "repository.relocate-checked-module-specifier"
        },
        {
          "name": "createCheckedModuleOverlay",
          "id": "repository.create-checked-module-overlay"
        },
        {
          "name": "importModule",
          "id": "repository.import-checked-overlay-module"
        },
        {
          "name": "dispose",
          "id": "repository.remove-checked-module-overlay"
        }
      ]
    }
  },
  {
    "inventory": "checked module overlay operations runtime.mjs",
    "files": {
      "experiments/rust-text-certificate-adapter/runtime.mjs": [
        {
          "name": "createRustTextCertificatePublicOverlay",
          "id": "repository.create-rust-text-certificate-public-overlay"
        }
      ]
    }
  },
  {
    "inventory": "checked module overlay operations generate.mjs",
    "files": {
      "experiments/rust-text-certificate-adapter/generate.mjs": [
        {
          "name": "rustTextCertificateOverlayRecords",
          "id": "repository.generate-rust-text-certificate-overlay-records"
        }
      ]
    }
  },
  {
    "inventory": "owned raw binding certificate decisions",
    "files": {
      "experiments/rust-raw-binding-certificate-adapter/rules.mjs": [
        {
          "name": "acceptRustRawBindingCertificateSource",
          "id": "repository.accept-rust-raw-binding-certificate-source"
        },
        {
          "name": "acceptRustRawBindingCertificateContext",
          "id": "repository.accept-rust-raw-binding-certificate-context"
        },
        {
          "name": "acceptRustRawBindingCertificateRoots",
          "id": "repository.accept-rust-raw-binding-certificate-facet-presence"
        },
        {
          "name": "acceptRustRawBindingCertificateOwners",
          "id": "repository.accept-rust-raw-binding-certificate-owners"
        }
      ],
      "experiments/rust-raw-binding-certificate-adapter/data/rules.rs": [
        {
          "name": "accept_rust_raw_binding_certificate_source",
          "id": "repository.accept-rust-raw-binding-certificate-source"
        },
        {
          "name": "accept_rust_raw_binding_certificate_context",
          "id": "repository.accept-rust-raw-binding-certificate-context"
        },
        {
          "name": "accept_rust_raw_binding_certificate_roots",
          "id": "repository.accept-rust-raw-binding-certificate-facet-presence"
        },
        {
          "name": "accept_rust_raw_binding_certificate_owners",
          "id": "repository.accept-rust-raw-binding-certificate-owners"
        }
      ]
    }
  },
  {
    "inventory": "owned raw binding certificate policy",
    "files": {
      "experiments/rust-raw-binding-certificate-adapter/generate-rules.mjs": [
        {
          "name": "rawBindingCertificatePolicy",
          "id": "repository.owned-raw-binding-certificate-policy"
        }
      ],
      "experiments/rust-raw-binding-certificate-adapter/certificate.mjs": [
        {
          "name": "RAW_BINDING_CERTIFICATE_POLICY",
          "id": "repository.owned-raw-binding-certificate-policy"
        }
      ]
    }
  },
  {
    "inventory": "owned raw binding certificate bridge operations",
    "files": {
      "experiments/rust-raw-binding-certificate-adapter/certificate.mjs": [
        {
          "name": "readRustRawBindingCertificate",
          "id": "repository.read-rust-raw-binding-certificate"
        },
        {
          "name": "captureRustRawBindingCertificate",
          "id": "repository.capture-rust-raw-binding-certificate"
        },
        {
          "name": "checkRustRawBindingCertificateInvocation",
          "id": "repository.check-rust-raw-binding-certificate-invocation"
        },
        {
          "name": "assumptions",
          "id": "repository.read-raw-binding-certificate-emission-premises"
        },
        {
          "name": "decorateRustRawBindingCertificateEmission",
          "id": "repository.decorate-rust-raw-binding-certificate-emission"
        },
        {
          "name": "emitRustRawBindingCertificate",
          "id": "repository.emit-rust-raw-binding-certificate"
        },
        {
          "name": "plainRecord",
          "id": "repository.accept-raw-binding-certificate-data-record"
        }
      ]
    }
  },
  {
    "inventory": "owned raw binding certificate role sourceRawBindingIdentifiers",
    "files": {
      "experiments/rust-raw-binding-certificate-adapter/data/policy.lino": [
        {
          "name": "sourceRawBindingIdentifiers",
          "id": "repository.owned-raw-binding-certificate-program-facet"
        }
      ]
    }
  },
  {
    "inventory": "owned raw binding certificate role sourceRawBindingIdentifierContext",
    "files": {
      "experiments/rust-raw-binding-certificate-adapter/data/policy.lino": [
        {
          "name": "sourceRawBindingIdentifierContext",
          "id": "repository.owned-raw-binding-certificate-main-context"
        }
      ]
    }
  },
  {
    "inventory": "owned raw binding certificate role sourceExtent",
    "files": {
      "experiments/rust-raw-binding-certificate-adapter/data/policy.lino": [
        {
          "name": "sourceExtent",
          "id": "repository.owned-raw-binding-certificate-source-extent-target"
        }
      ]
    }
  },
  {
    "inventory": "owned raw binding certificate authoring generate-rules.mjs",
    "files": {
      "experiments/rust-raw-binding-certificate-adapter/generate-rules.mjs": [
        {
          "name": "rawBindingCertificateRules",
          "id": "repository.generate-raw-binding-certificate-rules"
        },
        {
          "name": "checkRawBindingCertificateRules",
          "id": "repository.check-raw-binding-certificate-rules"
        }
      ]
    }
  },
  {
    "inventory": "owned raw binding certificate authoring generate-overlay.mjs",
    "files": {
      "experiments/rust-raw-binding-certificate-adapter/generate-overlay.mjs": [
        {
          "name": "rawBindingCertificateOverlayRecords",
          "id": "repository.generate-raw-binding-certificate-overlay-records"
        }
      ]
    }
  },
  {
    "inventory": "owned raw binding certificate authoring runtime.mjs",
    "files": {
      "experiments/rust-raw-binding-certificate-adapter/runtime.mjs": [
        {
          "name": "createRustRawBindingCertificateOverlay",
          "id": "repository.create-rust-raw-binding-certificate-overlay"
        }
      ]
    }
  },
  {
    "inventory": "shared emitted source parts",
    "files": {
      "js/src/translation/emit-common.js": [
        {
          "name": "withParts",
          "id": "translation.attach-source-output-parts"
        }
      ],
      "experiments/rust-raw-binding-certificate-adapter/certificate.mjs": [
        {
          "name": "withParts",
          "id": "translation.attach-source-output-parts"
        }
      ]
    }
  },
  {
    "inventory": "owned source adapter shared provider checkOwnedProgramContainers",
    "files": {
      "experiments/rust-text-certificate-adapter/model.mjs": [
        {
          "name": "checkOwnedProgramContainers",
          "id": "translation.check-owned-program-containers"
        }
      ],
      "experiments/rust-raw-binding-certificate-adapter/certificate.mjs": [
        {
          "name": "checkOwnedProgramContainers",
          "id": "translation.check-owned-program-containers"
        }
      ]
    }
  },
  {
    "inventory": "owned source adapter shared provider acceptRustOwnedLocalContainer",
    "files": {
      "experiments/rust-text-certificate-adapter/model.mjs": [
        {
          "name": "acceptRustOwnedLocalContainer",
          "id": "translation.accept-rust-owned-local-container"
        }
      ],
      "experiments/rust-raw-binding-certificate-adapter/certificate.mjs": [
        {
          "name": "acceptRustOwnedLocalContainer",
          "id": "translation.accept-rust-owned-local-container"
        }
      ]
    }
  },
  {
    "inventory": "owned source adapter shared provider checkPortableRustTextCertificate",
    "files": {
      "experiments/rust-text-certificate-adapter/model.mjs": [
        {
          "name": "checkPortableRustTextCertificate",
          "id": "repository.check-portable-rust-text-certificate"
        }
      ],
      "experiments/rust-text-certificate-adapter/raw-identity.mjs": [
        {
          "name": "checkPortableRustTextCertificate",
          "id": "repository.check-portable-rust-text-certificate"
        }
      ]
    }
  },
  {
    "inventory": "owned source adapter shared provider createPortableRustTextCertificate",
    "files": {
      "experiments/rust-text-certificate-adapter/model.mjs": [
        {
          "name": "createPortableRustTextCertificate",
          "id": "repository.create-portable-rust-text-certificate"
        }
      ]
    }
  },
  {
    "inventory": "owned source adapter shared provider encodeGeneratedOwnedRust",
    "files": {
      "experiments/rust-text-certificate-adapter/model.mjs": [
        {
          "name": "encodeGeneratedOwnedRust",
          "id": "repository.encode-generated-owned-rust"
        }
      ]
    }
  },
  {
    "inventory": "owned source adapter shared provider decodeGeneratedOwnedRust",
    "files": {
      "experiments/rust-text-certificate-adapter/model.mjs": [
        {
          "name": "decodeGeneratedOwnedRust",
          "id": "repository.decode-generated-owned-rust"
        }
      ],
      "experiments/rust-text-certificate-adapter/raw-identity.mjs": [
        {
          "name": "decodeGeneratedOwnedRust",
          "id": "repository.decode-generated-owned-rust"
        }
      ]
    }
  },
  {
    "inventory": "owned source adapter shared provider emitRustTextValueProjection",
    "files": {
      "experiments/rust-text-certificate-adapter/model.mjs": [
        {
          "name": "emitRustTextValueProjection",
          "id": "repository.emit-rust-text-value-projection"
        }
      ]
    }
  },
  {
    "inventory": "owned source adapter shared provider acceptProgramSchemaRoot",
    "files": {
      "experiments/rust-text-certificate-adapter/raw-identity.mjs": [
        {
          "name": "acceptProgramSchemaRoot",
          "id": "translation.accept-program-schema-root"
        }
      ]
    }
  },
  {
    "inventory": "owned source adapter shared provider readProgramContainerKind",
    "files": {
      "experiments/rust-text-certificate-adapter/raw-identity.mjs": [
        {
          "name": "readProgramContainerKind",
          "id": "translation.read-program-container-kind"
        }
      ]
    }
  },
  {
    "inventory": "owned source adapter shared provider createCheckedModuleOverlay",
    "files": {
      "experiments/rust-text-certificate-adapter/runtime.mjs": [
        {
          "name": "createCheckedModuleOverlay",
          "id": "repository.create-checked-module-overlay"
        }
      ],
      "experiments/rust-raw-binding-certificate-adapter/runtime.mjs": [
        {
          "name": "createCheckedModuleOverlay",
          "id": "repository.create-checked-module-overlay"
        }
      ]
    }
  },
  {
    "inventory": "owned source adapter shared provider createRustRawBindingIdentifiers",
    "files": {
      "experiments/rust-raw-binding-certificate-adapter/certificate.mjs": [
        {
          "name": "createRustRawBindingIdentifiers",
          "id": "repository.create-rust-raw-binding-identifiers"
        }
      ]
    }
  },
  {
    "inventory": "owned source adapter shared provider checkRustRawBindingIdentifiers",
    "files": {
      "experiments/rust-raw-binding-certificate-adapter/certificate.mjs": [
        {
          "name": "checkRustRawBindingIdentifiers",
          "id": "repository.check-rust-raw-binding-identifiers"
        }
      ]
    }
  },
  {
    "inventory": "owned source adapter shared provider checkRustRawBindingIdentifierInvocation",
    "files": {
      "experiments/rust-raw-binding-certificate-adapter/certificate.mjs": [
        {
          "name": "checkRustRawBindingIdentifierInvocation",
          "id": "repository.check-rust-raw-binding-identifier-invocation"
        }
      ]
    }
  },
  {
    "inventory": "owned source adapter shared provider emitRustRawBindingIdentifiers",
    "files": {
      "experiments/rust-raw-binding-certificate-adapter/certificate.mjs": [
        {
          "name": "emitRustRawBindingIdentifiers",
          "id": "repository.emit-rust-raw-binding-identifiers"
        }
      ]
    }
  },
  {
    "inventory": "owned source adapter shared provider readRawBindingIdentifierSyntax",
    "files": {
      "experiments/rust-raw-binding-certificate-adapter/certificate.mjs": [
        {
          "name": "readRawBindingIdentifierSyntax",
          "id": "repository.read-raw-binding-identifier-syntax"
        }
      ]
    }
  },
  {
    "inventory": "owned source adapter shared provider checkGeneratedOwnedRustData",
    "files": {
      "experiments/rust-raw-binding-certificate-adapter/certificate.mjs": [
        {
          "name": "checkGeneratedOwnedRustData",
          "id": "repository.check-generated-owned-rust-data-graph"
        },
        {
          "name": "data",
          "id": "repository.check-generated-owned-rust-data-graph"
        }
      ]
    }
  },
  {
    "inventory": "owned source adapter shared provider checkGeneratedOwnedRustFields",
    "files": {
      "experiments/rust-raw-binding-certificate-adapter/certificate.mjs": [
        {
          "name": "checkGeneratedOwnedRustFields",
          "id": "repository.check-generated-owned-rust-record-fields"
        },
        {
          "name": "fields",
          "id": "repository.check-generated-owned-rust-record-fields"
        }
      ]
    }
  },
  {
    "inventory": "owned source adapter shared hash",
    "files": {
      "experiments/rust-text-certificate-adapter/overlay.mjs": [
        {
          "name": "hash",
          "id": "repository.hash-rust-text-native-oracle-content"
        }
      ],
      "experiments/rust-text-certificate-adapter/generate.mjs": [
        {
          "name": "hash",
          "id": "repository.hash-rust-text-native-oracle-content"
        }
      ],
      "experiments/rust-text-certificate-adapter/overlay.test.mjs": [
        {
          "name": "hash",
          "id": "repository.hash-rust-text-native-oracle-content"
        }
      ],
      "experiments/rust-raw-binding-certificate-adapter/generate-overlay.mjs": [
        {
          "name": "hash",
          "id": "repository.hash-rust-text-native-oracle-content"
        }
      ]
    }
  },
  {
    "inventory": "owned text unchanged table context",
    "files": {
      "experiments/rust-text-certificate-adapter/data/identity-transport.lino": [
        {
          "name": "context",
          "id": "repository.text-certificate-context-table"
        }
      ]
    }
  },
  {
    "inventory": "owned text unchanged table kinds",
    "files": {
      "experiments/rust-text-certificate-adapter/data/identity-transport.lino": [
        {
          "name": "kinds",
          "id": "repository.text-certificate-identity-kind-table"
        }
      ]
    }
  },
  {
    "inventory": "owned text unchanged table tokens",
    "files": {
      "experiments/rust-text-certificate-adapter/data/identity-transport.lino": [
        {
          "name": "tokens",
          "id": "repository.text-certificate-token-table"
        }
      ]
    }
  },
  {
    "inventory": "owned text unchanged table declarations",
    "files": {
      "experiments/rust-text-certificate-adapter/data/identity-transport.lino": [
        {
          "name": "declarations",
          "id": "repository.text-certificate-declaration-table"
        }
      ]
    }
  },
  {
    "inventory": "owned text unchanged table parameters",
    "files": {
      "experiments/rust-text-certificate-adapter/data/identity-transport.lino": [
        {
          "name": "parameters",
          "id": "repository.text-certificate-parameter-table"
        }
      ]
    }
  },
  {
    "inventory": "owned text unchanged table positions",
    "files": {
      "experiments/rust-text-certificate-adapter/data/identity-transport.lino": [
        {
          "name": "positions",
          "id": "repository.text-certificate-source-position-table"
        }
      ]
    }
  },
  {
    "inventory": "owned text unchanged table paths",
    "files": {
      "experiments/rust-text-certificate-adapter/data/identity-transport.lino": [
        {
          "name": "paths",
          "id": "repository.text-certificate-owner-path-table"
        }
      ]
    }
  },
  {
    "inventory": "owned text unchanged table layers",
    "files": {
      "experiments/rust-text-certificate-adapter/data/identity-transport.lino": [
        {
          "name": "layers",
          "id": "repository.text-certificate-reference-layer-table"
        }
      ]
    }
  },
  {
    "inventory": "owned text unchanged table cores",
    "files": {
      "experiments/rust-text-certificate-adapter/data/identity-transport.lino": [
        {
          "name": "cores",
          "id": "repository.text-certificate-expression-table"
        }
      ]
    }
  },
  {
    "inventory": "owned text unchanged table functions",
    "files": {
      "experiments/rust-text-certificate-adapter/data/identity-transport.lino": [
        {
          "name": "functions",
          "id": "repository.text-certificate-function-fact-table"
        }
      ]
    }
  },
  {
    "inventory": "owned text unchanged table operations",
    "files": {
      "experiments/rust-text-certificate-adapter/data/identity-transport.lino": [
        {
          "name": "operations",
          "id": "repository.text-certificate-operation-table"
        }
      ]
    }
  },
  {
    "inventory": "owned text unchanged table parts",
    "files": {
      "experiments/rust-text-certificate-adapter/data/identity-transport.lino": [
        {
          "name": "parts",
          "id": "repository.text-certificate-operation-part-table"
        }
      ]
    }
  },
  {
    "inventory": "owned text unchanged table inputs",
    "files": {
      "experiments/rust-text-certificate-adapter/data/identity-transport.lino": [
        {
          "name": "inputs",
          "id": "repository.text-certificate-input-condition-table"
        }
      ]
    }
  },
  {
    "inventory": "owned text unchanged table allocations",
    "files": {
      "experiments/rust-text-certificate-adapter/data/identity-transport.lino": [
        {
          "name": "allocations",
          "id": "repository.text-certificate-allocation-condition-table"
        }
      ]
    }
  },
  {
    "inventory": "owned text unchanged table names",
    "files": {
      "experiments/rust-text-certificate-adapter/data/identity-transport.lino": [
        {
          "name": "names",
          "id": "repository.text-certificate-symbol-range-table"
        }
      ]
    }
  },
  {
    "inventory": "owned text unchanged table units",
    "files": {
      "experiments/rust-text-certificate-adapter/data/identity-transport.lino": [
        {
          "name": "units",
          "id": "repository.text-certificate-symbol-unit-table"
        }
      ]
    }
  },
  {
    "inventory": "owned text unchanged table items",
    "files": {
      "experiments/rust-text-certificate-adapter/data/identity-transport.lino": [
        {
          "name": "items",
          "id": "repository.text-certificate-declaration-order-table"
        }
      ]
    }
  },
  {
    "inventory": "owned raw unchanged proof role checkedOwner",
    "files": {
      "experiments/rust-raw-binding-certificate-adapter/model.test.mjs": [
        {
          "name": "checkedOwner",
          "id": "repository.raw-binding-checked-owner-target"
        }
      ]
    }
  },
  {
    "inventory": "owned raw unchanged proof role checkedExpression",
    "files": {
      "experiments/rust-raw-binding-certificate-adapter/model.test.mjs": [
        {
          "name": "checkedExpression",
          "id": "repository.raw-binding-checked-expression-target"
        }
      ]
    }
  },
  {
    "inventory": "owned raw unchanged proof role parameterOwners",
    "files": {
      "experiments/rust-raw-binding-certificate-adapter/certificate.mjs": [
        {
          "name": "parameterOwners",
          "id": "repository.raw-binding-invocation-parameter-array-target"
        }
      ],
      "experiments/rust-raw-binding-certificate-adapter/model.test.mjs": [
        {
          "name": "parameterOwners",
          "id": "repository.raw-binding-invocation-parameter-array-target"
        }
      ]
    }
  },
  {
    "inventory": "complete Rust library packaging operations",
    "files": {
      "js/scripts/rust-library-packaging.mjs": [
        {
          "name": "syntax",
          "id": "repository.read-rust-library-source-structure"
        },
        {
          "name": "readRustLibraryNesting",
          "id": "repository.read-rust-library-nesting"
        },
        {
          "name": "readRustLibraryItems",
          "id": "repository.read-rust-library-items"
        },
        {
          "name": "edits",
          "id": "repository.apply-rust-library-source-edits"
        },
        {
          "name": "conditional",
          "id": "repository.read-rust-library-tail-conditional"
        },
        {
          "name": "flattenBranches",
          "id": "repository.flatten-rust-library-tail-alternatives"
        },
        {
          "name": "compactIndentation",
          "id": "repository.compact-rust-library-indentation"
        },
        {
          "name": "qualify",
          "id": "repository.qualify-rust-library-item-paths"
        },
        {
          "name": "readRustLibraryStyles",
          "id": "repository.read-rust-library-styles"
        },
        {
          "name": "packageRustLibrary",
          "id": "repository.package-rust-library"
        }
      ]
    }
  },
  {
    "inventory": "complete text certificate library builders",
    "files": {
      "experiments/rust-text-certificate-library/generate.mjs": [
        {
          "name": "createTextCertificateLibrary",
          "id": "repository.create-text-certificate-library"
        },
        {
          "name": "materializeTextCertificateLibrary",
          "id": "repository.materialize-text-certificate-library"
        }
      ]
    }
  },
  {
    "inventory": "complete text certificate library policy",
    "files": {
      "experiments/rust-text-certificate-library/data/policy.lino": [
        {
          "name": "model",
          "id": "repository.complete-text-certificate-library-policy"
        }
      ]
    }
  },
  {
    "inventory": "complete text certificate library role definitions",
    "files": {
      "experiments/rust-text-certificate-library/generate.mjs": [
        {
          "name": "definitions",
          "id": "repository.text-certificate-library-definition-ownership"
        }
      ]
    }
  },
  {
    "inventory": "complete text certificate library role originalHeaders",
    "files": {
      "experiments/rust-text-certificate-library/generate.mjs": [
        {
          "name": "originalHeaders",
          "id": "repository.text-certificate-library-original-headers"
        }
      ]
    }
  },
  {
    "inventory": "complete text certificate library role originalTrailingSource",
    "files": {
      "experiments/rust-text-certificate-library/generate.mjs": [
        {
          "name": "originalTrailingSource",
          "id": "repository.text-certificate-library-original-trailing-source"
        }
      ]
    }
  },
  {
    "inventory": "complete text certificate library role modules",
    "files": {
      "experiments/rust-text-certificate-library/generate.mjs": [
        {
          "name": "modules",
          "id": "repository.text-certificate-library-complete-modules"
        }
      ]
    }
  },
  {
    "inventory": "complete library shared source hash",
    "files": {
      "experiments/rust-text-certificate-library/generate.mjs": [
        {
          "name": "sha256",
          "id": "repository.hash-rust-text-native-oracle-content"
        }
      ]
    }
  },
  {
    "inventory": "library unchanged shared source name admission",
    "files": {
      "js/scripts/rust-library-packaging.mjs": [
        {
          "name": "acceptRustPureUnitName",
          "id": "translation.accept-rust-pure-unit-name"
        }
      ]
    }
  },
  {
    "inventory": "complete library shared tooling packageRustLibrary",
    "files": {
      "experiments/rust-text-certificate-library/generate.mjs": [
        {
          "name": "packageRustLibrary",
          "id": "repository.package-rust-library"
        }
      ]
    }
  },
  {
    "inventory": "complete library shared tooling readRustLibraryNesting",
    "files": {
      "experiments/rust-text-certificate-library/generate.mjs": [
        {
          "name": "readRustLibraryNesting",
          "id": "repository.read-rust-library-nesting"
        }
      ]
    }
  },
  {
    "inventory": "complete library original definition aliases",
    "files": {
      "experiments/rust-text-certificate-library/data/library.lino": [
        {
          "name": "acceptRustPureUnitCallModel",
          "id": "translation.accept-rust-pure-unit-call-model"
        },
        {
          "name": "accept_rust_pure_unit_call_model",
          "id": "translation.accept-rust-pure-unit-call-model"
        },
        {
          "name": "acceptRustPureUnitNameCharacter",
          "id": "translation.accept-rust-pure-unit-name-character"
        },
        {
          "name": "accept_rust_pure_unit_name_character",
          "id": "translation.accept-rust-pure-unit-name-character"
        },
        {
          "name": "acceptRustPureUnitName",
          "id": "translation.accept-rust-pure-unit-name"
        },
        {
          "name": "accept_rust_pure_unit_name",
          "id": "translation.accept-rust-pure-unit-name"
        },
        {
          "name": "acceptRustPureUnitHeaderForm",
          "id": "translation.accept-rust-pure-unit-header-form"
        },
        {
          "name": "accept_rust_pure_unit_header_form",
          "id": "translation.accept-rust-pure-unit-header-form"
        },
        {
          "name": "acceptRustPureUnitBodyForm",
          "id": "translation.accept-rust-pure-unit-body-form"
        },
        {
          "name": "accept_rust_pure_unit_body_form",
          "id": "translation.accept-rust-pure-unit-body-form"
        },
        {
          "name": "acceptRustPureUnitPathForm",
          "id": "translation.accept-rust-pure-unit-path-form"
        },
        {
          "name": "accept_rust_pure_unit_path_form",
          "id": "translation.accept-rust-pure-unit-path-form"
        },
        {
          "name": "acceptRustPureUnitContext",
          "id": "translation.accept-rust-pure-unit-context"
        },
        {
          "name": "accept_rust_pure_unit_context",
          "id": "translation.accept-rust-pure-unit-context"
        },
        {
          "name": "acceptRustPureUnitVisibility",
          "id": "translation.accept-rust-pure-unit-visibility"
        },
        {
          "name": "accept_rust_pure_unit_visibility",
          "id": "translation.accept-rust-pure-unit-visibility"
        },
        {
          "name": "acceptRustPureUnitContainer",
          "id": "translation.accept-rust-pure-unit-container"
        },
        {
          "name": "accept_rust_pure_unit_container",
          "id": "translation.accept-rust-pure-unit-container"
        },
        {
          "name": "acceptRustPureUnitSourceOffset",
          "id": "translation.accept-rust-pure-unit-source-offset"
        },
        {
          "name": "accept_rust_pure_unit_source_offset",
          "id": "translation.accept-rust-pure-unit-source-offset"
        },
        {
          "name": "acceptRustTextValueProjectionModel",
          "id": "repository.accept-rust-text-value-projection-model"
        },
        {
          "name": "accept_rust_text_value_projection_model",
          "id": "repository.accept-rust-text-value-projection-model"
        },
        {
          "name": "readRustTextValueProjectionDomain",
          "id": "repository.read-rust-text-value-projection-domain"
        },
        {
          "name": "read_rust_text_value_projection_domain",
          "id": "repository.read-rust-text-value-projection-domain"
        },
        {
          "name": "acceptRustTextValueProjectionOperation",
          "id": "repository.accept-rust-text-value-projection-operation"
        },
        {
          "name": "accept_rust_text_value_projection_operation",
          "id": "repository.accept-rust-text-value-projection-operation"
        },
        {
          "name": "acceptRustOwnedLocalStateModel",
          "id": "translation.accept-rust-owned-local-state-model"
        },
        {
          "name": "accept_rust_owned_local_state_model",
          "id": "translation.accept-rust-owned-local-state-model"
        },
        {
          "name": "acceptRustOwnedLocalValueType",
          "id": "translation.accept-rust-owned-local-value-type"
        },
        {
          "name": "accept_rust_owned_local_value_type",
          "id": "translation.accept-rust-owned-local-value-type"
        },
        {
          "name": "acceptRustOwnedLocalExpression",
          "id": "translation.accept-rust-owned-local-expression"
        },
        {
          "name": "accept_rust_owned_local_expression",
          "id": "translation.accept-rust-owned-local-expression"
        },
        {
          "name": "acceptRustOwnedLocalBinaryOperation",
          "id": "translation.accept-rust-owned-local-binary-operation"
        },
        {
          "name": "accept_rust_owned_local_binary_operation",
          "id": "translation.accept-rust-owned-local-binary-operation"
        },
        {
          "name": "acceptRustOwnedLocalSourceOperation",
          "id": "translation.accept-rust-owned-local-source-operation"
        },
        {
          "name": "accept_rust_owned_local_source_operation",
          "id": "translation.accept-rust-owned-local-source-operation"
        },
        {
          "name": "acceptRustOwnedLocalContainer",
          "id": "translation.accept-rust-owned-local-container"
        },
        {
          "name": "accept_rust_owned_local_container",
          "id": "translation.accept-rust-owned-local-container"
        },
        {
          "name": "acceptRustOwnedLocalRecordValue",
          "id": "translation.accept-rust-owned-local-record-value"
        },
        {
          "name": "accept_rust_owned_local_record_value",
          "id": "translation.accept-rust-owned-local-record-value"
        },
        {
          "name": "acceptRustOwnedLocalSourceSpan",
          "id": "translation.accept-rust-owned-local-source-span"
        },
        {
          "name": "accept_rust_owned_local_source_span",
          "id": "translation.accept-rust-owned-local-source-span"
        },
        {
          "name": "acceptRustOwnedLocalTextUnits",
          "id": "translation.accept-rust-owned-local-text-units"
        },
        {
          "name": "accept_rust_owned_local_text_units",
          "id": "translation.accept-rust-owned-local-text-units"
        },
        {
          "name": "renderRustOwnedLocalTextInput",
          "id": "translation.render-rust-owned-local-text-input"
        },
        {
          "name": "render_rust_owned_local_text_input",
          "id": "translation.render-rust-owned-local-text-input"
        },
        {
          "name": "readTextCertificateKnownSymbol",
          "id": "repository.read-text-certificate-known-symbol"
        },
        {
          "name": "read_text_certificate_known_symbol",
          "id": "repository.read-text-certificate-known-symbol"
        },
        {
          "name": "acceptTextCertificateStorage",
          "id": "repository.accept-text-certificate-storage"
        },
        {
          "name": "accept_text_certificate_storage",
          "id": "repository.accept-text-certificate-storage"
        },
        {
          "name": "findTextCertificateRow",
          "id": "repository.find-text-certificate-row"
        },
        {
          "name": "find_text_certificate_row",
          "id": "repository.find-text-certificate-row"
        },
        {
          "name": "acceptTextCertificateNumbers",
          "id": "repository.accept-text-certificate-numbers"
        },
        {
          "name": "accept_text_certificate_numbers",
          "id": "repository.accept-text-certificate-numbers"
        },
        {
          "name": "acceptTextCertificateIdentities",
          "id": "repository.accept-text-certificate-identities"
        },
        {
          "name": "accept_text_certificate_identities",
          "id": "repository.accept-text-certificate-identities"
        },
        {
          "name": "acceptTextCertificateName",
          "id": "repository.accept-text-certificate-name"
        },
        {
          "name": "accept_text_certificate_name",
          "id": "repository.accept-text-certificate-name"
        },
        {
          "name": "readTextCertificateDomain",
          "id": "repository.read-text-certificate-domain"
        },
        {
          "name": "read_text_certificate_domain",
          "id": "repository.read-text-certificate-domain"
        },
        {
          "name": "readTextCertificateType",
          "id": "repository.read-text-certificate-type"
        },
        {
          "name": "read_text_certificate_type",
          "id": "repository.read-text-certificate-type"
        },
        {
          "name": "readTextCertificatePrefix",
          "id": "repository.read-text-certificate-prefix"
        },
        {
          "name": "read_text_certificate_prefix",
          "id": "repository.read-text-certificate-prefix"
        },
        {
          "name": "readTextCertificateSuffix",
          "id": "repository.read-text-certificate-suffix"
        },
        {
          "name": "read_text_certificate_suffix",
          "id": "repository.read-text-certificate-suffix"
        },
        {
          "name": "readTextCertificateOperation",
          "id": "repository.read-text-certificate-operation"
        },
        {
          "name": "read_text_certificate_operation",
          "id": "repository.read-text-certificate-operation"
        },
        {
          "name": "readTextCertificateDomainName",
          "id": "repository.read-text-certificate-domain-name"
        },
        {
          "name": "read_text_certificate_domain_name",
          "id": "repository.read-text-certificate-domain-name"
        },
        {
          "name": "acceptTextCertificateAllocation",
          "id": "repository.accept-text-certificate-allocation"
        },
        {
          "name": "accept_text_certificate_allocation",
          "id": "repository.accept-text-certificate-allocation"
        },
        {
          "name": "matchTextCertificateSyntax",
          "id": "repository.match-text-certificate-syntax"
        },
        {
          "name": "match_text_certificate_syntax",
          "id": "repository.match-text-certificate-syntax"
        },
        {
          "name": "acceptTextCertificateToken",
          "id": "repository.accept-text-certificate-token"
        },
        {
          "name": "accept_text_certificate_token",
          "id": "repository.accept-text-certificate-token"
        },
        {
          "name": "acceptTextCertificateOperation",
          "id": "repository.accept-text-certificate-operation"
        },
        {
          "name": "accept_text_certificate_operation",
          "id": "repository.accept-text-certificate-operation"
        },
        {
          "name": "acceptTextCertificateTokenOperation",
          "id": "repository.accept-text-certificate-token-operation"
        },
        {
          "name": "accept_text_certificate_token_operation",
          "id": "repository.accept-text-certificate-token-operation"
        },
        {
          "name": "acceptTextCertificateCondition",
          "id": "repository.accept-text-certificate-condition"
        },
        {
          "name": "accept_text_certificate_condition",
          "id": "repository.accept-text-certificate-condition"
        },
        {
          "name": "checkRustTextCertificate",
          "id": "repository.check-rust-text-certificate"
        },
        {
          "name": "check_rust_text_certificate",
          "id": "repository.check-rust-text-certificate"
        },
        {
          "name": "ml_acceptRustOwnedLocalTextUnits_loop1_result",
          "id": "repository.rust-owned-local-text-scalar-sequence-control-result"
        },
        {
          "name": "MlAcceptRustOwnedLocalTextUnitsLoop1Result",
          "id": "repository.rust-owned-local-text-scalar-sequence-control-result"
        },
        {
          "name": "ml_acceptRustOwnedLocalTextUnits_loop1",
          "id": "repository.continue-rust-owned-local-text-scalar-sequence"
        },
        {
          "name": "ml_accept_rust_owned_local_text_units_loop1",
          "id": "repository.continue-rust-owned-local-text-scalar-sequence"
        },
        {
          "name": "ml_acceptTextCertificateStorage_loop5_result",
          "id": "repository.text-certificate-symbol-dictionary-storage-control-result"
        },
        {
          "name": "MlAcceptTextCertificateStorageLoop5Result",
          "id": "repository.text-certificate-symbol-dictionary-storage-control-result"
        },
        {
          "name": "ml_acceptTextCertificateStorage_loop7_result",
          "id": "repository.text-certificate-prior-symbol-uniqueness-control-result"
        },
        {
          "name": "MlAcceptTextCertificateStorageLoop7Result",
          "id": "repository.text-certificate-prior-symbol-uniqueness-control-result"
        },
        {
          "name": "ml_acceptTextCertificateStorage_loop8",
          "id": "repository.continue-text-certificate-symbol-unit-comparison"
        },
        {
          "name": "ml_accept_text_certificate_storage_loop8",
          "id": "repository.continue-text-certificate-symbol-unit-comparison"
        },
        {
          "name": "ml_acceptTextCertificateStorage_loop7",
          "id": "repository.continue-text-certificate-prior-symbol-uniqueness"
        },
        {
          "name": "ml_accept_text_certificate_storage_loop7",
          "id": "repository.continue-text-certificate-prior-symbol-uniqueness"
        },
        {
          "name": "ml_acceptTextCertificateStorage_join6",
          "id": "repository.text-certificate-fixed-symbol-comparison-control-result"
        },
        {
          "name": "MlAcceptTextCertificateStorageJoin6",
          "id": "repository.text-certificate-fixed-symbol-comparison-control-result"
        },
        {
          "name": "ml_acceptTextCertificateStorage_loop9_result",
          "id": "repository.text-certificate-fixed-symbol-units-control-result"
        },
        {
          "name": "MlAcceptTextCertificateStorageLoop9Result",
          "id": "repository.text-certificate-fixed-symbol-units-control-result"
        },
        {
          "name": "ml_acceptTextCertificateStorage_loop9",
          "id": "repository.continue-text-certificate-fixed-symbol-units"
        },
        {
          "name": "ml_accept_text_certificate_storage_loop9",
          "id": "repository.continue-text-certificate-fixed-symbol-units"
        },
        {
          "name": "ml_acceptTextCertificateStorage_loop5",
          "id": "repository.continue-text-certificate-symbol-dictionary-storage"
        },
        {
          "name": "ml_accept_text_certificate_storage_loop5",
          "id": "repository.continue-text-certificate-symbol-dictionary-storage"
        },
        {
          "name": "ml_acceptTextCertificateStorage_loop4_result",
          "id": "repository.text-certificate-structural-identity-coverage-control-result"
        },
        {
          "name": "MlAcceptTextCertificateStorageLoop4Result",
          "id": "repository.text-certificate-structural-identity-coverage-control-result"
        },
        {
          "name": "ml_acceptTextCertificateStorage_join10",
          "id": "repository.text-certificate-role-occurrence-count-control-result"
        },
        {
          "name": "MlAcceptTextCertificateStorageJoin10",
          "id": "repository.text-certificate-role-occurrence-count-control-result"
        },
        {
          "name": "ml_acceptTextCertificateStorage_loop11",
          "id": "repository.continue-text-certificate-token-identity-count"
        },
        {
          "name": "ml_accept_text_certificate_storage_loop11",
          "id": "repository.continue-text-certificate-token-identity-count"
        },
        {
          "name": "ml_acceptTextCertificateStorage_loop12",
          "id": "repository.continue-text-certificate-declaration-identity-count"
        },
        {
          "name": "ml_accept_text_certificate_storage_loop12",
          "id": "repository.continue-text-certificate-declaration-identity-count"
        },
        {
          "name": "ml_acceptTextCertificateStorage_loop13",
          "id": "repository.continue-text-certificate-parameter-identity-count"
        },
        {
          "name": "ml_accept_text_certificate_storage_loop13",
          "id": "repository.continue-text-certificate-parameter-identity-count"
        },
        {
          "name": "ml_acceptTextCertificateStorage_loop14",
          "id": "repository.continue-text-certificate-signature-identity-count"
        },
        {
          "name": "ml_accept_text_certificate_storage_loop14",
          "id": "repository.continue-text-certificate-signature-identity-count"
        },
        {
          "name": "ml_acceptTextCertificateStorage_loop15",
          "id": "repository.continue-text-certificate-source-position-identity-count"
        },
        {
          "name": "ml_accept_text_certificate_storage_loop15",
          "id": "repository.continue-text-certificate-source-position-identity-count"
        },
        {
          "name": "ml_acceptTextCertificateStorage_loop16",
          "id": "repository.continue-text-certificate-expression-identity-count"
        },
        {
          "name": "ml_accept_text_certificate_storage_loop16",
          "id": "repository.continue-text-certificate-expression-identity-count"
        },
        {
          "name": "ml_acceptTextCertificateStorage_loop17",
          "id": "repository.continue-text-certificate-operation-identity-count"
        },
        {
          "name": "ml_accept_text_certificate_storage_loop17",
          "id": "repository.continue-text-certificate-operation-identity-count"
        },
        {
          "name": "ml_acceptTextCertificateStorage_loop4",
          "id": "repository.continue-text-certificate-structural-identity-coverage"
        },
        {
          "name": "ml_accept_text_certificate_storage_loop4",
          "id": "repository.continue-text-certificate-structural-identity-coverage"
        },
        {
          "name": "ml_acceptTextCertificateStorage_loop3_result",
          "id": "repository.text-certificate-source-position-storage-ranges-control-result"
        },
        {
          "name": "MlAcceptTextCertificateStorageLoop3Result",
          "id": "repository.text-certificate-source-position-storage-ranges-control-result"
        },
        {
          "name": "ml_acceptTextCertificateStorage_loop3",
          "id": "repository.continue-text-certificate-source-position-storage-ranges"
        },
        {
          "name": "ml_accept_text_certificate_storage_loop3",
          "id": "repository.continue-text-certificate-source-position-storage-ranges"
        },
        {
          "name": "ml_acceptTextCertificateStorage_loop2_result",
          "id": "repository.text-certificate-operation-part-ranges-control-result"
        },
        {
          "name": "MlAcceptTextCertificateStorageLoop2Result",
          "id": "repository.text-certificate-operation-part-ranges-control-result"
        },
        {
          "name": "ml_acceptTextCertificateStorage_loop2",
          "id": "repository.continue-text-certificate-operation-part-ranges"
        },
        {
          "name": "ml_accept_text_certificate_storage_loop2",
          "id": "repository.continue-text-certificate-operation-part-ranges"
        },
        {
          "name": "ml_findTextCertificateRow_loop18_result",
          "id": "repository.text-certificate-unique-row-search-control-result"
        },
        {
          "name": "MlFindTextCertificateRowLoop18Result",
          "id": "repository.text-certificate-unique-row-search-control-result"
        },
        {
          "name": "ml_findTextCertificateRow_loop18",
          "id": "repository.continue-text-certificate-unique-row-search"
        },
        {
          "name": "ml_find_text_certificate_row_loop18",
          "id": "repository.continue-text-certificate-unique-row-search"
        },
        {
          "name": "ml_acceptTextCertificateNumbers_loop19_result",
          "id": "repository.text-certificate-numeric-domain-scan-control-result"
        },
        {
          "name": "MlAcceptTextCertificateNumbersLoop19Result",
          "id": "repository.text-certificate-numeric-domain-scan-control-result"
        },
        {
          "name": "ml_acceptTextCertificateNumbers_loop19",
          "id": "repository.continue-text-certificate-numeric-domain-scan"
        },
        {
          "name": "ml_accept_text_certificate_numbers_loop19",
          "id": "repository.continue-text-certificate-numeric-domain-scan"
        },
        {
          "name": "ml_acceptTextCertificateIdentities_loop20_result",
          "id": "repository.text-certificate-row-identity-scan-control-result"
        },
        {
          "name": "MlAcceptTextCertificateIdentitiesLoop20Result",
          "id": "repository.text-certificate-row-identity-scan-control-result"
        },
        {
          "name": "ml_acceptTextCertificateIdentities_loop20",
          "id": "repository.continue-text-certificate-row-identity-scan"
        },
        {
          "name": "ml_accept_text_certificate_identities_loop20",
          "id": "repository.continue-text-certificate-row-identity-scan"
        },
        {
          "name": "ml_acceptTextCertificateName_loop21_result",
          "id": "repository.text-certificate-name-character-scan-control-result"
        },
        {
          "name": "MlAcceptTextCertificateNameLoop21Result",
          "id": "repository.text-certificate-name-character-scan-control-result"
        },
        {
          "name": "ml_acceptTextCertificateName_loop21",
          "id": "repository.continue-text-certificate-name-character-scan"
        },
        {
          "name": "ml_accept_text_certificate_name_loop21",
          "id": "repository.continue-text-certificate-name-character-scan"
        },
        {
          "name": "ml_readTextCertificateDomain_loop22_result",
          "id": "repository.text-certificate-reference-layer-scan-control-result"
        },
        {
          "name": "MlReadTextCertificateDomainLoop22Result",
          "id": "repository.text-certificate-reference-layer-scan-control-result"
        },
        {
          "name": "ml_readTextCertificateDomain_loop22",
          "id": "repository.continue-text-certificate-reference-layer-scan"
        },
        {
          "name": "ml_read_text_certificate_domain_loop22",
          "id": "repository.continue-text-certificate-reference-layer-scan"
        },
        {
          "name": "ml_matchTextCertificateSyntax_loop23_result",
          "id": "repository.text-certificate-fixed-syntax-scan-control-result"
        },
        {
          "name": "MlMatchTextCertificateSyntaxLoop23Result",
          "id": "repository.text-certificate-fixed-syntax-scan-control-result"
        },
        {
          "name": "ml_matchTextCertificateSyntax_loop23",
          "id": "repository.continue-text-certificate-fixed-syntax-scan"
        },
        {
          "name": "ml_match_text_certificate_syntax_loop23",
          "id": "repository.continue-text-certificate-fixed-syntax-scan"
        },
        {
          "name": "ml_acceptTextCertificateOperation_loop24_result",
          "id": "repository.text-certificate-operation-part-comparison-control-result"
        },
        {
          "name": "MlAcceptTextCertificateOperationLoop24Result",
          "id": "repository.text-certificate-operation-part-comparison-control-result"
        },
        {
          "name": "ml_acceptTextCertificateOperation_loop24",
          "id": "repository.continue-text-certificate-operation-part-comparison"
        },
        {
          "name": "ml_accept_text_certificate_operation_loop24",
          "id": "repository.continue-text-certificate-operation-part-comparison"
        },
        {
          "name": "ml_acceptTextCertificateTokenOperation_loop25_result",
          "id": "repository.text-certificate-operation-token-comparison-control-result"
        },
        {
          "name": "MlAcceptTextCertificateTokenOperationLoop25Result",
          "id": "repository.text-certificate-operation-token-comparison-control-result"
        },
        {
          "name": "ml_acceptTextCertificateTokenOperation_loop25",
          "id": "repository.continue-text-certificate-operation-token-comparison"
        },
        {
          "name": "ml_accept_text_certificate_token_operation_loop25",
          "id": "repository.continue-text-certificate-operation-token-comparison"
        },
        {
          "name": "ml_checkRustTextCertificate_loop27_result",
          "id": "repository.text-certificate-complete-function-scan-control-result"
        },
        {
          "name": "MlCheckRustTextCertificateLoop27Result",
          "id": "repository.text-certificate-complete-function-scan-control-result"
        },
        {
          "name": "ml_checkRustTextCertificate_join31",
          "id": "repository.text-certificate-input-condition-selection-control-result"
        },
        {
          "name": "MlCheckRustTextCertificateJoin31",
          "id": "repository.text-certificate-input-condition-selection-control-result"
        },
        {
          "name": "ml_checkRustTextCertificate_join30",
          "id": "repository.text-certificate-body-domain-selection-control-result"
        },
        {
          "name": "MlCheckRustTextCertificateJoin30",
          "id": "repository.text-certificate-body-domain-selection-control-result"
        },
        {
          "name": "ml_checkRustTextCertificate_loop29_result",
          "id": "repository.text-certificate-function-parameter-scan-control-result"
        },
        {
          "name": "MlCheckRustTextCertificateLoop29Result",
          "id": "repository.text-certificate-function-parameter-scan-control-result"
        },
        {
          "name": "ml_checkRustTextCertificate_loop29",
          "id": "repository.continue-text-certificate-function-parameter-scan"
        },
        {
          "name": "ml_check_rust_text_certificate_loop29",
          "id": "repository.continue-text-certificate-function-parameter-scan"
        },
        {
          "name": "ml_checkRustTextCertificate_loop28_result",
          "id": "repository.text-certificate-prior-declaration-name-scan-control-result"
        },
        {
          "name": "MlCheckRustTextCertificateLoop28Result",
          "id": "repository.text-certificate-prior-declaration-name-scan-control-result"
        },
        {
          "name": "ml_checkRustTextCertificate_loop28",
          "id": "repository.continue-text-certificate-prior-declaration-name-scan"
        },
        {
          "name": "ml_check_rust_text_certificate_loop28",
          "id": "repository.continue-text-certificate-prior-declaration-name-scan"
        },
        {
          "name": "ml_checkRustTextCertificate_loop27",
          "id": "repository.continue-text-certificate-complete-function-scan"
        },
        {
          "name": "ml_check_rust_text_certificate_loop27",
          "id": "repository.continue-text-certificate-complete-function-scan"
        },
        {
          "name": "ml_checkRustTextCertificate_loop26_result",
          "id": "repository.text-certificate-symbol-unit-bound-scan-control-result"
        },
        {
          "name": "MlCheckRustTextCertificateLoop26Result",
          "id": "repository.text-certificate-symbol-unit-bound-scan-control-result"
        },
        {
          "name": "ml_checkRustTextCertificate_loop26",
          "id": "repository.continue-text-certificate-symbol-unit-bound-scan"
        },
        {
          "name": "ml_check_rust_text_certificate_loop26",
          "id": "repository.continue-text-certificate-symbol-unit-bound-scan"
        }
      ]
    }
  },
  {
    "inventory": "owned text certificate model controls",
    "files": {
      "experiments/rust-text-certificate-adapter/model.test.mjs": [
        {
          "name": "execute",
          "id": "repository.execute-owned-text-certificate-test-artifact"
        },
        {
          "name": "refuse",
          "id": "repository.require-owned-text-certificate-test-refusal"
        },
        {
          "name": "retarget",
          "id": "repository.change-owned-text-certificate-test-input"
        }
      ]
    }
  },
  {
    "inventory": "owned text shared saved field reader",
    "files": {
      "experiments/rust-text-certificate-adapter/model.test.mjs": [
        {
          "name": "field",
          "id": "repository.read-text-certificate-test-field"
        }
      ],
      "experiments/rust-text-certificate-adapter/raw-identity.test.mjs": [
        {
          "name": "field",
          "id": "repository.read-text-certificate-test-field"
        }
      ]
    }
  },
  {
    "inventory": "owned text shared saved field replacement",
    "files": {
      "experiments/rust-text-certificate-adapter/model.test.mjs": [
        {
          "name": "replaceField",
          "id": "repository.replace-text-certificate-test-field"
        }
      ],
      "experiments/rust-text-certificate-adapter/raw-identity.test.mjs": [
        {
          "name": "replaceField",
          "id": "repository.replace-text-certificate-test-field"
        }
      ]
    }
  },
  {
    "inventory": "owned text proof control entry",
    "files": {
      "experiments/rust-text-certificate-adapter/model.test.mjs": [
        {
          "name": "proofId",
          "id": "repository.read-owned-text-certificate-test-proof"
        }
      ],
      "experiments/rust-text-certificate-adapter/raw-identity.test.mjs": [
        {
          "name": "proofId",
          "id": "repository.read-owned-text-certificate-test-proof"
        }
      ]
    }
  },
  {
    "inventory": "owned text raw identity controls",
    "files": {
      "experiments/rust-text-certificate-adapter/raw-identity.test.mjs": [
        {
          "name": "rows",
          "id": "repository.read-owned-text-certificate-test-rows"
        },
        {
          "name": "attach",
          "id": "repository.attach-owned-text-certificate-test-proof"
        },
        {
          "name": "cloneRecord",
          "id": "repository.clone-owned-text-certificate-test-record"
        },
        {
          "name": "scalar",
          "id": "repository.create-owned-text-certificate-test-scalar"
        }
      ]
    }
  },
  {
    "inventory": "owned raw certificate controls",
    "files": {
      "experiments/rust-raw-binding-certificate-adapter/model.test.mjs": [
        {
          "name": "field",
          "id": "repository.read-owned-raw-certificate-test-field"
        },
        {
          "name": "replaceChild",
          "id": "repository.replace-owned-raw-certificate-test-target"
        },
        {
          "name": "cloneRecord",
          "id": "repository.clone-owned-raw-certificate-test-record"
        },
        {
          "name": "execute",
          "id": "repository.execute-owned-raw-certificate-test-artifact"
        },
        {
          "name": "refused",
          "id": "repository.require-owned-raw-certificate-test-refusal"
        }
      ]
    }
  },
  {
    "inventory": "complete Rust library structural controls",
    "files": {
      "js/tests/translation-rust-library-packaging.test.js": [
        {
          "name": "tokens",
          "id": "repository.read-rust-library-test-tokens"
        },
        {
          "name": "styled",
          "id": "repository.render-rust-library-test-styles"
        }
      ]
    }
  }
];
const normalizeOwnedSourceAdapterName=name=>name.replace(/([a-z0-9])([A-Z])/gu,'$1-$2').replaceAll('_','-').replaceAll(' ','-').toLowerCase().replace(/\bcst\b/gu,'concrete-source-tree').replace(/\bstr\b/gu,'string-slice').replace(/^retarget$/u,'change-target').replace(/^tokenize-/u,'read-tokens-').replace(/\bsha256\b/gu,'secure-hash-algorithm-two-hundred-fifty-six').replace(/\bml\b/gu,'meta-language').replace(/\bid\b/gu,'identity').replace(/\b(loop|join)([0-9]+)\b/gu,(whole,kind,digits)=>{const words={1:'one',2:'two',3:'three',4:'four',5:'five',6:'six',7:'seven',8:'eight',9:'nine',10:'ten',11:'eleven',12:'twelve',13:'thirteen',14:'fourteen',15:'fifteen',16:'sixteen',17:'seventeen',18:'eighteen',19:'nineteen',20:'twenty',21:'twenty-one',22:'twenty-two',23:'twenty-three',24:'twenty-four',25:'twenty-five',26:'twenty-six',27:'twenty-seven',28:'twenty-eight',29:'twenty-nine',30:'thirty',31:'thirty-one'};return words[digits]?kind+'-'+words[digits]:whole;});
const OWNED_SOURCE_ADAPTER_SOURCE_TEXTS=new Map();
const OWNED_SOURCE_ADAPTER_NAME_INVENTORIES=OWNED_SOURCE_ADAPTER_NAME_BINDINGS.map(({inventory,files})=>({
 inventory,files:Object.keys(files),
 extract:(raw,file)=>{let text=raw;if(file.endsWith('.lino')){const saved=OWNED_SOURCE_ADAPTER_SOURCE_TEXTS.get(file);if(!saved||saved.raw!==raw)OWNED_SOURCE_ADAPTER_SOURCE_TEXTS.set(file,{raw,text:JSON.stringify(readStructuredDataLino(raw))});text=OWNED_SOURCE_ADAPTER_SOURCE_TEXTS.get(file).text;}return files[file].filter(({name})=>new RegExp('\\b'+name+'\\b','u').test(text)).map(({name})=>normalizeOwnedSourceAdapterName(name));},
 recordOf:name=>Object.values(files).flat().find(entry=>normalizeOwnedSourceAdapterName(entry.name)===name).id,
}));
// Actual CI-only source bundle and library oracle identities and safe identifier guards.
const FINAL_SOURCE_ORACLE_NAME_BINDINGS=[
  {
    "inventory": "authenticated native source bundle mechanics",
    "files": {
      "experiments/source-native-observations.mjs": [
        {
          "name": "prepareBundle",
          "id": "repository.prepare-native-source-observation-bundle"
        },
        {
          "name": "regularPath",
          "id": "repository.check-native-source-bundle-file-path"
        },
        {
          "name": "runCheckedProcess",
          "id": "repository.run-checked-native-source-observation-process"
        }
      ]
    }
  },
  {
    "inventory": "authenticated native source bundle data",
    "files": {
      "experiments/source-native-observations.mjs": [
        {
          "name": "additionalSources",
          "id": "repository.native-source-observation-bundle"
        }
      ]
    }
  },
  {
    "inventory": "native source bundle descriptors",
    "files": {
      "experiments/source-native-observations.test.mjs": [
        {
          "name": "descriptor",
          "id": "repository.create-native-source-bundle-descriptor"
        }
      ],
      "experiments/rust-text-certificate-library/native-observations.mjs": [
        {
          "name": "sourceReceipt",
          "id": "repository.create-native-source-bundle-descriptor"
        }
      ]
    }
  },
  {
    "inventory": "native source bundle injected controls",
    "files": {
      "experiments/source-native-observations.test.mjs": [
        {
          "name": "makePlan",
          "id": "repository.create-native-source-bundle-test-plan"
        },
        {
          "name": "harness",
          "id": "repository.create-native-source-bundle-test-harness"
        },
        {
          "name": "failure",
          "id": "repository.require-native-source-bundle-test-failure"
        }
      ]
    }
  },
  {
    "inventory": "complete certificate library source oracle operations",
    "files": {
      "experiments/rust-text-certificate-library/native-observations.mjs": [
        {
          "name": "verifyRustTextCertificateLibraryModel",
          "id": "repository.verify-text-certificate-library-native-model"
        },
        {
          "name": "rustTextCertificateLibraryNativeMode",
          "id": "repository.read-text-certificate-library-native-mode"
        },
        {
          "name": "publicHeader",
          "id": "repository.check-text-certificate-library-public-header"
        },
        {
          "name": "planRustTextCertificateLibraryNative",
          "id": "repository.plan-text-certificate-library-native-observations"
        },
        {
          "name": "runRustTextCertificateLibraryNative",
          "id": "repository.run-text-certificate-library-native-observations"
        }
      ]
    }
  },
  {
    "inventory": "complete certificate library source oracle role RUST_TEXT_CERTIFICATE_LIBRARY_MODEL_COMMIT",
    "files": {
      "experiments/rust-text-certificate-library/native-observations.mjs": [
        {
          "name": "RUST_TEXT_CERTIFICATE_LIBRARY_MODEL_COMMIT",
          "id": "repository.text-certificate-library-native-model-revision"
        }
      ]
    }
  },
  {
    "inventory": "complete certificate library source oracle role sourceDigests",
    "files": {
      "experiments/rust-text-certificate-library/native-observations.mjs": [
        {
          "name": "sourceDigests",
          "id": "repository.text-certificate-library-native-source-digests"
        }
      ]
    }
  },
  {
    "inventory": "complete certificate library source oracle role checkedPlans",
    "files": {
      "experiments/rust-text-certificate-library/native-observations.mjs": [
        {
          "name": "checkedPlans",
          "id": "repository.text-certificate-library-native-plan-set"
        }
      ]
    }
  },
  {
    "inventory": "complete certificate library source oracle role sourceFiles",
    "files": {
      "experiments/rust-text-certificate-library/native-observations.mjs": [
        {
          "name": "sourceFiles",
          "id": "repository.text-certificate-library-compilation-source-receipts"
        }
      ]
    }
  },
  {
    "inventory": "complete certificate library source oracle role originalArtifactReceipt",
    "files": {
      "experiments/rust-text-certificate-library/native-observations.mjs": [
        {
          "name": "originalArtifactReceipt",
          "id": "repository.text-certificate-library-original-artifact-receipt"
        }
      ]
    }
  },
  {
    "inventory": "complete certificate library source oracle role generationBoundary",
    "files": {
      "experiments/rust-text-certificate-library/native-observations.mjs": [
        {
          "name": "generationBoundary",
          "id": "repository.text-certificate-library-generation-boundary"
        }
      ]
    }
  },
  {
    "inventory": "complete certificate library injected controls",
    "files": {
      "experiments/rust-text-certificate-library/native-observations.test.mjs": [
        {
          "name": "injected",
          "id": "repository.run-text-certificate-library-injected-contract"
        },
        {
          "name": "failure",
          "id": "repository.require-text-certificate-library-native-test-failure"
        }
      ]
    }
  },
  {
    "inventory": "source bundle and library exact provider source hash",
    "files": {
      "experiments/source-native-observations.test.mjs": [
        {
          "name": "sha256",
          "id": "repository.hash-rust-text-native-oracle-content"
        }
      ],
      "experiments/rust-text-certificate-library/native-observations.mjs": [
        {
          "name": "sha256",
          "id": "repository.hash-rust-text-native-oracle-content"
        }
      ],
      "experiments/rust-text-certificate-library/native-observations.test.mjs": [
        {
          "name": "sha256",
          "id": "repository.hash-rust-text-native-oracle-content"
        }
      ]
    }
  },
  {
    "inventory": "source bundle and library exact provider scalar source bytes",
    "files": {
      "experiments/source-native-observations.test.mjs": [
        {
          "name": "textBytes",
          "id": "repository.read-rust-text-native-oracle-bytes"
        }
      ],
      "experiments/rust-text-certificate-library/native-observations.mjs": [
        {
          "name": "textBytes",
          "id": "repository.read-rust-text-native-oracle-bytes"
        }
      ]
    }
  },
  {
    "inventory": "source bundle and library exact provider immutable plans",
    "files": {
      "experiments/rust-text-certificate-library/native-observations.mjs": [
        {
          "name": "freeze",
          "id": "repository.freeze-rust-text-native-observation-plan"
        }
      ]
    }
  },
  {
    "inventory": "source bundle and library exact provider shared configured runner",
    "files": {
      "experiments/source-native-observations.test.mjs": [
        {
          "name": "createSourceNativeObservationRunner",
          "id": "repository.create-source-native-observation-runner"
        }
      ],
      "experiments/rust-text-certificate-library/native-observations.mjs": [
        {
          "name": "createSourceNativeObservationRunner",
          "id": "repository.create-source-native-observation-runner"
        }
      ]
    }
  },
  {
    "inventory": "source bundle and library exact provider original complete oracle",
    "files": {
      "experiments/rust-text-certificate-library/native-observations.mjs": [
        {
          "name": "planRustTextCertificateNative",
          "id": "repository.plan-text-certificate-native-observations"
        }
      ],
      "experiments/rust-text-certificate-library/native-observations.test.mjs": [
        {
          "name": "planRustTextCertificateNative",
          "id": "repository.plan-text-certificate-native-observations"
        }
      ]
    }
  },
  {
    "inventory": "source bundle and library exact provider complete library builder",
    "files": {
      "experiments/rust-text-certificate-library/native-observations.mjs": [
        {
          "name": "createTextCertificateLibrary",
          "id": "repository.create-text-certificate-library"
        }
      ]
    }
  },
  {
    "inventory": "source bundle and library exact provider unchanged forty case bit result reader",
    "files": {
      "experiments/rust-text-certificate-library/native-observations.mjs": [
        {
          "name": "parseObservations",
          "id": "repository.read-text-certificate-native-observation-stream"
        }
      ]
    }
  },
  {
    "inventory": "source bundle and library exact provider original definition ownership",
    "files": {
      "experiments/rust-text-certificate-library/native-observations.mjs": [
        {
          "name": "definitionOwners",
          "id": "repository.text-certificate-library-definition-ownership"
        }
      ]
    }
  },
  {
    "inventory": "source bundle and library exact provider unchanged argument order",
    "files": {
      "experiments/rust-text-certificate-library/native-observations.mjs": [
        {
          "name": "tableOrder",
          "id": "repository.text-certificate-native-argument-order"
        }
      ]
    }
  },
  {
    "inventory": "source bundle and library exact provider unchanged output roles",
    "files": {
      "experiments/rust-text-certificate-library/native-observations.mjs": [
        {
          "name": "outputRoles",
          "id": "repository.text-certificate-native-output-roles"
        }
      ]
    }
  },
  {
    "inventory": "source bundle and library exact provider unchanged input table receipts",
    "files": {
      "experiments/rust-text-certificate-library/native-observations.mjs": [
        {
          "name": "inputTables",
          "id": "repository.text-certificate-native-input-table-receipts"
        }
      ]
    }
  },
  {
    "inventory": "source bundle and library exact provider unchanged result bit vector",
    "files": {
      "experiments/rust-text-certificate-library/native-observations.mjs": [
        {
          "name": "resultBits",
          "id": "repository.text-certificate-native-result-bit-vector"
        }
      ]
    }
  },
  {
    "inventory": "source bundle and library exact provider fake bit output processes",
    "files": {
      "experiments/rust-text-certificate-library/native-observations.test.mjs": [
        {
          "name": "fakeProcess",
          "id": "repository.create-text-certificate-native-test-process"
        }
      ]
    }
  },
  {
    "inventory": "existing safe profile identity validation",
    "files": {
      "js/src/primitives.js": [
        {
          "name": "readLinkIdValue",
          "id": "repository.read-safe-link-identity-value"
        }
      ]
    }
  },
  {
    "inventory": "existing safe profile identity token parsing",
    "files": {
      "js/src/lino-serialization.js": [
        {
          "name": "parseLinkId",
          "id": "repository.parse-safe-link-identity-token"
        }
      ]
    }
  },
  {
    "inventory": "safe profile original token controls",
    "files": {
      "js/tests/link-id-boundaries.test.js": [
        {
          "name": "canonical",
          "id": "repository.render-link-identity-boundary-test-statement"
        }
      ]
    }
  }
];
const normalizeFinalSourceOracleName=name=>name.replace(/([a-z0-9])([A-Z])/gu,'$1-$2').replaceAll('_','-').replaceAll(' ','-').toLowerCase().replace(/\bsha256\b/gu,'secure-hash-algorithm-two-hundred-fifty-six').replace(/\bid\b/gu,'identity');
const FINAL_SOURCE_ORACLE_NAME_INVENTORIES=FINAL_SOURCE_ORACLE_NAME_BINDINGS.map(({inventory,files})=>({
 inventory,files:Object.keys(files),
 extract:(raw,file)=>files[file].filter(({name})=>new RegExp('\\b'+name+'\\b','u').test(raw)).map(({name})=>normalizeFinalSourceOracleName(name)),
 recordOf:name=>Object.values(files).flat().find(entry=>normalizeFinalSourceOracleName(entry.name)===name).id,
}));
// Actual isolated identity, role projection and finite collector meanings.
const ORIGINAL_LINK_AND_COLLECTOR_NAME_BINDINGS=[
  {
    "inventory": "exact original link identity decisions",
    "files": {
      "experiments/original-link-identity/rules.mjs": [
        {
          "name": "acceptOriginalLinkIdentityWord",
          "id": "repository.accept-original-link-identity-word"
        },
        {
          "name": "checkOriginalLinkIdentityIndex",
          "id": "repository.check-original-link-identity-index"
        },
        {
          "name": "findOriginalLinkIdentityOrdinal",
          "id": "repository.find-original-link-identity-ordinal"
        },
        {
          "name": "readOriginalLinkIdentityOrdinals",
          "id": "repository.read-original-link-identity-ordinals"
        }
      ],
      "experiments/original-link-identity/data/native-definitions.lino": [
        {
          "name": "accept_original_link_identity_word",
          "id": "repository.accept-original-link-identity-word"
        },
        {
          "name": "check_original_link_identity_index",
          "id": "repository.check-original-link-identity-index"
        },
        {
          "name": "find_original_link_identity_ordinal",
          "id": "repository.find-original-link-identity-ordinal"
        },
        {
          "name": "read_original_link_identity_ordinals",
          "id": "repository.read-original-link-identity-ordinals"
        }
      ]
    }
  },
  {
    "inventory": "original link identity policy and host operations",
    "files": {
      "experiments/original-link-identity/policy.mjs": [
        {
          "name": "originalLinkIdentityPolicy",
          "id": "repository.original-link-identity-policy"
        },
        {
          "name": "originalLinkIdentityRulesSource",
          "id": "repository.render-original-link-identity-rules"
        },
        {
          "name": "exactOriginalLinkIdentity",
          "id": "repository.read-original-link-identity-integer"
        },
        {
          "name": "originalLinkIdentityWords",
          "id": "repository.project-original-link-identity-words"
        },
        {
          "name": "originalLinkIdentityIndex",
          "id": "repository.create-original-link-identity-index"
        },
        {
          "name": "historicalPublicLinkId",
          "id": "repository.historical-public-link-identity-source"
        }
      ]
    }
  },
  {
    "inventory": "original link identity exact transport roles",
    "files": {
      "experiments/original-link-identity/policy.mjs": [
        {
          "name": "anchors",
          "id": "repository.original-link-identity-anchor-vector"
        },
        {
          "name": "nullHandle",
          "id": "repository.original-link-identity-missing-ordinal"
        },
        {
          "name": "absentHandle",
          "id": "repository.original-link-identity-absent-handle"
        }
      ]
    }
  },
  {
    "inventory": "original link identity linked source generation",
    "files": {
      "experiments/original-link-identity/generate.mjs": [
        {
          "name": "originalLinkIdentityNativeDefinitions",
          "id": "repository.build-original-link-identity-native-definitions"
        },
        {
          "name": "generateOriginalLinkIdentity",
          "id": "repository.generate-original-link-identity-data"
        }
      ]
    }
  },
  {
    "inventory": "original link certificate role decisions",
    "files": {
      "experiments/original-link-role-projection/rules.mjs": [
        {
          "name": "readOriginalLinkRoleField",
          "id": "repository.read-original-link-role-field"
        },
        {
          "name": "readOriginalLinkRoleReferences",
          "id": "repository.read-original-link-role-references"
        },
        {
          "name": "readOriginalLinkRoleOperation",
          "id": "repository.accept-original-link-role-operation"
        },
        {
          "name": "firstOriginalLinkRoleOccurrence",
          "id": "repository.accept-first-original-link-role-occurrence"
        },
        {
          "name": "readOriginalLinkRoleKind",
          "id": "repository.read-original-link-concrete-role-kind"
        },
        {
          "name": "registerOriginalLinkRoles",
          "id": "repository.register-original-link-certificate-roles"
        },
        {
          "name": "projectOriginalLinkRoleRegistry",
          "id": "repository.project-original-link-role-registry"
        }
      ],
      "experiments/original-link-role-projection/data/native-definitions.lino": [
        {
          "name": "read_original_link_role_field",
          "id": "repository.read-original-link-role-field"
        },
        {
          "name": "read_original_link_role_references",
          "id": "repository.read-original-link-role-references"
        },
        {
          "name": "read_original_link_role_operation",
          "id": "repository.accept-original-link-role-operation"
        },
        {
          "name": "first_original_link_role_occurrence",
          "id": "repository.accept-first-original-link-role-occurrence"
        },
        {
          "name": "read_original_link_role_kind",
          "id": "repository.read-original-link-concrete-role-kind"
        },
        {
          "name": "register_original_link_roles",
          "id": "repository.register-original-link-certificate-roles"
        },
        {
          "name": "project_original_link_role_registry",
          "id": "repository.project-original-link-role-registry"
        }
      ]
    }
  },
  {
    "inventory": "original link certificate role source operations",
    "files": {
      "experiments/original-link-role-projection/policy.mjs": [
        {
          "name": "originalLinkRolePolicy",
          "id": "repository.original-link-role-policy"
        },
        {
          "name": "originalLinkRoleRulesSource",
          "id": "repository.render-original-link-role-rules"
        }
      ],
      "experiments/original-link-role-projection/transport.mjs": [
        {
          "name": "ORIGINAL_LINK_ROLE_POLICY",
          "id": "repository.original-link-role-policy"
        },
        {
          "name": "readOriginalLinkRoleTransport",
          "id": "repository.read-original-link-role-transport"
        },
        {
          "name": "projectOriginalLinkCertificateRoles",
          "id": "repository.project-original-link-certificate-roles"
        }
      ],
      "experiments/original-link-role-projection/generate.mjs": [
        {
          "name": "generateOriginalLinkRoleProjection",
          "id": "repository.generate-original-link-role-data"
        }
      ]
    }
  },
  {
    "inventory": "original link role host transport operations",
    "files": {
      "experiments/original-link-role-projection/transport.mjs": [
        {
          "name": "ordinal",
          "id": "repository.read-original-link-role-membership-ordinals"
        },
        {
          "name": "fail",
          "id": "repository.raise-original-link-role-error"
        }
      ]
    }
  },
  {
    "inventory": "original link role exact reversible field names",
    "files": {
      "experiments/original-link-role-projection/transport.mjs": [
        {
          "name": "fieldName",
          "id": "repository.read-owned-text-certificate-field-name"
        }
      ]
    }
  },
  {
    "inventory": "original link role existing kinds table",
    "files": {
      "experiments/original-link-role-projection/rules.mjs": [
        {
          "name": "kinds",
          "id": "repository.text-certificate-identity-kind-table"
        }
      ],
      "experiments/original-link-role-projection/transport.mjs": [
        {
          "name": "kinds",
          "id": "repository.text-certificate-identity-kind-table"
        }
      ]
    }
  },
  {
    "inventory": "original link role selected occurrence data",
    "files": {
      "experiments/original-link-role-projection/rules.mjs": [
        {
          "name": "members",
          "id": "repository.original-link-role-membership-occurrences"
        },
        {
          "name": "roles",
          "id": "repository.original-link-certificate-role-occurrences"
        },
        {
          "name": "handles",
          "id": "repository.original-link-role-compact-handles"
        }
      ]
    }
  },
  {
    "inventory": "experiments/original-link-identity/translated control wrappers",
    "files": {
      "experiments/original-link-identity/data/native-definitions.lino": [
        {
          "name": "MlCheckOriginalLinkIdentityIndexLoop1Result",
          "id": "repository.original-link-identity-index-scan-control-result"
        },
        {
          "name": "MlReadOriginalLinkIdentityOrdinalsLoop4Result",
          "id": "repository.original-link-identity-missing-result-scan-control-result"
        },
        {
          "name": "MlReadOriginalLinkIdentityOrdinalsLoop3Result",
          "id": "repository.original-link-identity-query-word-scan-control-result"
        }
      ]
    }
  },
  {
    "inventory": "experiments/original-link-identity/translated control constructors",
    "files": {
      "experiments/original-link-identity/data/native-definitions.lino": [
        {
          "name": "MlCheckOriginalLinkIdentityIndexLoop1Done",
          "id": "repository.original-link-identity-index-scan-completion-constructor"
        },
        {
          "name": "MlCheckOriginalLinkIdentityIndexLoop1Return",
          "id": "repository.original-link-identity-index-scan-return-constructor"
        },
        {
          "name": "MlReadOriginalLinkIdentityOrdinalsLoop4Done",
          "id": "repository.original-link-identity-missing-result-scan-completion-constructor"
        },
        {
          "name": "MlReadOriginalLinkIdentityOrdinalsLoop4Return",
          "id": "repository.original-link-identity-missing-result-scan-return-constructor"
        },
        {
          "name": "MlReadOriginalLinkIdentityOrdinalsLoop3Done",
          "id": "repository.original-link-identity-query-word-scan-completion-constructor"
        },
        {
          "name": "MlReadOriginalLinkIdentityOrdinalsLoop3Return",
          "id": "repository.original-link-identity-query-word-scan-return-constructor"
        }
      ]
    }
  },
  {
    "inventory": "experiments/original-link-identity/translated control continuations",
    "files": {
      "experiments/original-link-identity/data/native-definitions.lino": [
        {
          "name": "ml_check_original_link_identity_index_loop1",
          "id": "repository.continue-original-link-identity-index-scan"
        },
        {
          "name": "ml_find_original_link_identity_ordinal_loop2",
          "id": "repository.continue-original-link-identity-binary-search"
        },
        {
          "name": "ml_read_original_link_identity_ordinals_loop4",
          "id": "repository.continue-original-link-identity-missing-result-scan"
        },
        {
          "name": "ml_read_original_link_identity_ordinals_loop3",
          "id": "repository.continue-original-link-identity-query-word-scan"
        }
      ]
    }
  },
  {
    "inventory": "experiments/original-link-role-projection/translated control wrappers",
    "files": {
      "experiments/original-link-role-projection/data/native-definitions.lino": [
        {
          "name": "ml_readOriginalLinkRoleField_loop1_result",
          "id": "repository.original-link-role-unique-field-scan-control-result"
        },
        {
          "name": "MlReadOriginalLinkRoleFieldLoop1Result",
          "id": "repository.original-link-role-unique-field-scan-control-result"
        },
        {
          "name": "ml_firstOriginalLinkRoleOccurrence_loop2_result",
          "id": "repository.original-link-role-first-occurrence-scan-control-result"
        },
        {
          "name": "MlFirstOriginalLinkRoleOccurrenceLoop2Result",
          "id": "repository.original-link-role-first-occurrence-scan-control-result"
        },
        {
          "name": "ml_registerOriginalLinkRoles_loop5_result",
          "id": "repository.original-link-role-unresolved-kind-scan-control-result"
        },
        {
          "name": "MlRegisterOriginalLinkRolesLoop5Result",
          "id": "repository.original-link-role-unresolved-kind-scan-control-result"
        },
        {
          "name": "ml_registerOriginalLinkRoles_loop4_result",
          "id": "repository.original-link-role-occurrence-validation-control-result"
        },
        {
          "name": "MlRegisterOriginalLinkRolesLoop4Result",
          "id": "repository.original-link-role-occurrence-validation-control-result"
        },
        {
          "name": "ml_registerOriginalLinkRoles_loop6_result",
          "id": "repository.original-link-role-contradiction-scan-control-result"
        },
        {
          "name": "MlRegisterOriginalLinkRolesLoop6Result",
          "id": "repository.original-link-role-contradiction-scan-control-result"
        },
        {
          "name": "ml_projectOriginalLinkRoleRegistry_loop7_result",
          "id": "repository.original-link-role-ordered-operation-traversal-control-result"
        },
        {
          "name": "MlProjectOriginalLinkRoleRegistryLoop7Result",
          "id": "repository.original-link-role-ordered-operation-traversal-control-result"
        }
      ]
    }
  },
  {
    "inventory": "experiments/original-link-role-projection/translated control constructors",
    "files": {
      "experiments/original-link-role-projection/data/native-definitions.lino": [
        {
          "name": "MlReadOriginalLinkRoleFieldLoop1Done",
          "id": "repository.original-link-role-unique-field-scan-completion-constructor"
        },
        {
          "name": "MlReadOriginalLinkRoleFieldLoop1Return",
          "id": "repository.original-link-role-unique-field-scan-return-constructor"
        },
        {
          "name": "MlFirstOriginalLinkRoleOccurrenceLoop2Done",
          "id": "repository.original-link-role-first-occurrence-scan-completion-constructor"
        },
        {
          "name": "MlFirstOriginalLinkRoleOccurrenceLoop2Return",
          "id": "repository.original-link-role-first-occurrence-scan-return-constructor"
        },
        {
          "name": "MlRegisterOriginalLinkRolesLoop5Done",
          "id": "repository.original-link-role-unresolved-kind-scan-completion-constructor"
        },
        {
          "name": "MlRegisterOriginalLinkRolesLoop5Return",
          "id": "repository.original-link-role-unresolved-kind-scan-return-constructor"
        },
        {
          "name": "MlRegisterOriginalLinkRolesLoop4Done",
          "id": "repository.original-link-role-occurrence-validation-completion-constructor"
        },
        {
          "name": "MlRegisterOriginalLinkRolesLoop4Return",
          "id": "repository.original-link-role-occurrence-validation-return-constructor"
        },
        {
          "name": "MlRegisterOriginalLinkRolesLoop6Done",
          "id": "repository.original-link-role-contradiction-scan-completion-constructor"
        },
        {
          "name": "MlRegisterOriginalLinkRolesLoop6Return",
          "id": "repository.original-link-role-contradiction-scan-return-constructor"
        },
        {
          "name": "MlProjectOriginalLinkRoleRegistryLoop7Done",
          "id": "repository.original-link-role-ordered-operation-traversal-completion-constructor"
        },
        {
          "name": "MlProjectOriginalLinkRoleRegistryLoop7Return",
          "id": "repository.original-link-role-ordered-operation-traversal-return-constructor"
        }
      ]
    }
  },
  {
    "inventory": "experiments/original-link-role-projection/translated control continuations",
    "files": {
      "experiments/original-link-role-projection/data/native-definitions.lino": [
        {
          "name": "ml_readOriginalLinkRoleField_loop1",
          "id": "repository.continue-original-link-role-unique-field-scan"
        },
        {
          "name": "ml_read_original_link_role_field_loop1",
          "id": "repository.continue-original-link-role-unique-field-scan"
        },
        {
          "name": "ml_firstOriginalLinkRoleOccurrence_loop2",
          "id": "repository.continue-original-link-role-first-occurrence-scan"
        },
        {
          "name": "ml_first_original_link_role_occurrence_loop2",
          "id": "repository.continue-original-link-role-first-occurrence-scan"
        },
        {
          "name": "ml_readOriginalLinkRoleKind_loop3",
          "id": "repository.continue-original-link-role-concrete-kind-scan"
        },
        {
          "name": "ml_read_original_link_role_kind_loop3",
          "id": "repository.continue-original-link-role-concrete-kind-scan"
        },
        {
          "name": "ml_registerOriginalLinkRoles_loop5",
          "id": "repository.continue-original-link-role-unresolved-kind-scan"
        },
        {
          "name": "ml_register_original_link_roles_loop5",
          "id": "repository.continue-original-link-role-unresolved-kind-scan"
        },
        {
          "name": "ml_registerOriginalLinkRoles_loop6",
          "id": "repository.continue-original-link-role-contradiction-scan"
        },
        {
          "name": "ml_register_original_link_roles_loop6",
          "id": "repository.continue-original-link-role-contradiction-scan"
        },
        {
          "name": "ml_registerOriginalLinkRoles_loop4",
          "id": "repository.continue-original-link-role-occurrence-validation"
        },
        {
          "name": "ml_register_original_link_roles_loop4",
          "id": "repository.continue-original-link-role-occurrence-validation"
        },
        {
          "name": "ml_projectOriginalLinkRoleRegistry_loop7",
          "id": "repository.continue-original-link-role-ordered-operation-traversal"
        },
        {
          "name": "ml_project_original_link_role_registry_loop7",
          "id": "repository.continue-original-link-role-ordered-operation-traversal"
        }
      ]
    }
  },
  {
    "inventory": "exact identity complete source oracle operations",
    "files": {
      "experiments/original-link-identity/native-observations.mjs": [
        {
          "name": "verifyOriginalLinkIdentityNativeModel",
          "id": "repository.verify-original-link-identity-native-model"
        },
        {
          "name": "originalLinkIdentityNativeMode",
          "id": "repository.read-original-link-identity-native-mode"
        },
        {
          "name": "nativeInterface",
          "id": "repository.read-original-link-identity-native-interface"
        },
        {
          "name": "cases",
          "id": "repository.build-original-link-identity-native-cases"
        },
        {
          "name": "consumerSource",
          "id": "repository.render-original-link-identity-native-consumer"
        },
        {
          "name": "planOriginalLinkIdentityNative",
          "id": "repository.plan-original-link-identity-native-observations"
        },
        {
          "name": "parseObservations",
          "id": "repository.read-original-link-identity-native-observation-stream"
        },
        {
          "name": "runOriginalLinkIdentityNative",
          "id": "repository.run-original-link-identity-native-observations"
        }
      ]
    }
  },
  {
    "inventory": "exact identity complete source oracle receipt roles",
    "files": {
      "experiments/original-link-identity/native-observations.mjs": [
        {
          "name": "ORIGINAL_LINK_IDENTITY_MODEL_COMMIT",
          "id": "repository.original-link-identity-native-model-revision"
        },
        {
          "name": "maximumObservedIndexRows",
          "id": "repository.original-link-identity-native-index-row-bound"
        },
        {
          "name": "maximumObservedQueryRows",
          "id": "repository.original-link-identity-native-query-row-bound"
        },
        {
          "name": "inputVectors",
          "id": "repository.original-link-identity-native-input-vector-receipts"
        },
        {
          "name": "resultBits",
          "id": "repository.original-link-identity-native-result-bit-vectors"
        }
      ]
    }
  },
  {
    "inventory": "original identity existing exact oracle provider source digest",
    "files": {
      "experiments/original-link-identity/native-observations.mjs": [
        {
          "name": "sha256",
          "id": "repository.hash-rust-text-native-oracle-content"
        }
      ],
      "experiments/original-link-identity/native-observations.test.mjs": [
        {
          "name": "digest",
          "id": "repository.hash-rust-text-native-oracle-content"
        }
      ]
    }
  },
  {
    "inventory": "original identity existing exact oracle provider source scalar bytes",
    "files": {
      "experiments/original-link-identity/native-observations.mjs": [
        {
          "name": "textBytes",
          "id": "repository.read-rust-text-native-oracle-bytes"
        }
      ]
    }
  },
  {
    "inventory": "original identity existing exact oracle provider immutable plan",
    "files": {
      "experiments/original-link-identity/native-observations.mjs": [
        {
          "name": "freeze",
          "id": "repository.freeze-rust-text-native-observation-plan"
        }
      ]
    }
  },
  {
    "inventory": "original identity existing exact oracle provider number bit encoder",
    "files": {
      "experiments/original-link-identity/native-observations.mjs": [
        {
          "name": "numberBits",
          "id": "repository.encode-text-certificate-native-number-bits"
        }
      ]
    }
  },
  {
    "inventory": "original identity existing exact oracle provider number bit decoder",
    "files": {
      "experiments/original-link-identity/native-observations.mjs": [
        {
          "name": "bitsNumber",
          "id": "repository.decode-text-certificate-native-number-bits"
        }
      ]
    }
  },
  {
    "inventory": "original identity existing exact oracle provider complete shared source runner",
    "files": {
      "experiments/original-link-identity/native-observations.mjs": [
        {
          "name": "createSourceNativeObservationRunner",
          "id": "repository.create-source-native-observation-runner"
        }
      ]
    }
  },
  {
    "inventory": "original identity existing exact oracle provider controlled complete artifact process",
    "files": {
      "experiments/original-link-identity/native-observations.test.mjs": [
        {
          "name": "fakeProcess",
          "id": "repository.create-text-certificate-native-test-process"
        }
      ]
    }
  },
  {
    "inventory": "original identity existing exact oracle provider exact aggregate failure contract",
    "files": {
      "experiments/original-link-identity/native-observations.test.mjs": [
        {
          "name": "fails",
          "id": "repository.require-text-certificate-native-test-failure"
        }
      ]
    }
  },
  {
    "inventory": "original identity injected process controls",
    "files": {
      "experiments/original-link-identity/native-observations.test.mjs": [
        {
          "name": "injected",
          "id": "repository.run-original-link-identity-injected-contract"
        }
      ]
    }
  },
  {
    "inventory": "original link identity independent controls",
    "files": {
      "experiments/original-link-identity/identity.test.mjs": [
        {
          "name": "words",
          "id": "repository.project-original-link-identity-test-word-vectors"
        },
        {
          "name": "run",
          "id": "repository.run-original-link-identity-test-query"
        },
        {
          "name": "oracle",
          "id": "repository.read-original-link-identity-linear-test-ordinals"
        },
        {
          "name": "reloads",
          "id": "repository.reload-original-link-identity-test-packet"
        }
      ]
    }
  },
  {
    "inventory": "original link role exact original test keys",
    "files": {
      "experiments/original-link-role-projection/projection.test.mjs": [
        {
          "name": "key",
          "id": "repository.read-exact-link-identity-key"
        }
      ]
    }
  },
  {
    "inventory": "original link identity actual saved field reader",
    "files": {
      "experiments/original-link-identity/identity.test.mjs": [
        {
          "name": "field",
          "id": "repository.read-text-certificate-test-field"
        }
      ]
    }
  },
  {
    "inventory": "original link role unique saved field reader",
    "files": {
      "experiments/original-link-role-projection/projection.test.mjs": [
        {
          "name": "field",
          "id": "repository.read-unique-original-link-role-test-field"
        }
      ]
    }
  },
  {
    "inventory": "original graph reversible test field labels",
    "files": {
      "experiments/original-link-role-projection/projection.test.mjs": [
        {
          "name": "label",
          "id": "repository.render-certificate-test-field-label"
        }
      ],
      "experiments/original-link-identity/identity.test.mjs": [
        {
          "name": "label",
          "id": "repository.render-certificate-test-field-label"
        }
      ]
    }
  },
  {
    "inventory": "original link role saved field replacement",
    "files": {
      "experiments/original-link-role-projection/projection.test.mjs": [
        {
          "name": "replace",
          "id": "repository.replace-text-certificate-test-field"
        }
      ]
    }
  },
  {
    "inventory": "original link role attached proof controls",
    "files": {
      "experiments/original-link-role-projection/projection.test.mjs": [
        {
          "name": "attach",
          "id": "repository.attach-owned-text-certificate-test-proof"
        }
      ]
    }
  },
  {
    "inventory": "original link role independent correspondence controls",
    "files": {
      "experiments/original-link-role-projection/projection.test.mjs": [
        {
          "name": "refs",
          "id": "repository.read-original-link-role-test-references"
        },
        {
          "name": "body",
          "id": "repository.read-original-link-role-test-body"
        },
        {
          "name": "create",
          "id": "repository.create-original-link-role-test-packet"
        },
        {
          "name": "proof",
          "id": "repository.read-original-link-role-test-proof"
        },
        {
          "name": "rows",
          "id": "repository.read-original-link-role-test-proof-rows"
        },
        {
          "name": "unpack",
          "id": "repository.read-original-link-role-test-registration"
        },
        {
          "name": "remapExistingTables",
          "id": "repository.change-original-link-role-test-table-identities"
        }
      ]
    }
  },
  {
    "inventory": "native workflow input selection controls",
    "files": {
      "js/tests/ci-workflow-structure.test.js": [
        {
          "name": "selectNativeTestInputs",
          "id": "repository.select-native-workflow-test-inputs"
        },
        {
          "name": "nativeInputCondition",
          "id": "repository.read-native-workflow-input-condition"
        },
        {
          "name": "evaluateNativeInputCondition",
          "id": "repository.evaluate-native-workflow-input-condition"
        },
        {
          "name": "nativeInputJobChain",
          "id": "repository.evaluate-native-workflow-input-job-chain"
        }
      ]
    }
  },
  {
    "inventory": "fresh Number array collector admission decisions",
    "files": {
      "experiments/fresh-number-array-collectors/rules.mjs": [
        {
          "name": "acceptFreshNumberArrayCollectorPattern",
          "id": "repository.accept-fresh-number-array-collector-pattern"
        },
        {
          "name": "acceptFreshNumberArrayCollectorBinding",
          "id": "repository.accept-fresh-number-array-collector-binding"
        }
      ],
      "experiments/fresh-number-array-collectors/data/rule-translation.lino": [
        {
          "name": "accept_fresh_number_array_collector_pattern",
          "id": "repository.accept-fresh-number-array-collector-pattern"
        },
        {
          "name": "accept_fresh_number_array_collector_binding",
          "id": "repository.accept-fresh-number-array-collector-binding"
        }
      ]
    }
  },
  {
    "inventory": "fresh Number array collector source operations",
    "files": {
      "experiments/fresh-number-array-collectors/policy.mjs": [
        {
          "name": "freshNumberArrayCollectorPolicy",
          "id": "repository.fresh-number-array-collector-policy"
        }
      ],
      "experiments/fresh-number-array-collectors/model.mjs": [
        {
          "name": "FRESH_NUMBER_ARRAY_COLLECTOR_POLICY",
          "id": "repository.fresh-number-array-collector-policy"
        },
        {
          "name": "checkContainers",
          "id": "repository.check-fresh-number-array-collector-containers"
        },
        {
          "name": "sourceFacts",
          "id": "repository.read-fresh-number-array-collector-source-facts"
        },
        {
          "name": "compile",
          "id": "repository.check-fresh-number-array-collector-lowering"
        },
        {
          "name": "certificate",
          "id": "repository.create-fresh-number-array-collector-certificate"
        },
        {
          "name": "originalSource",
          "id": "repository.reconstruct-fresh-number-array-collector-source"
        },
        {
          "name": "createFreshNumberArrayCollector",
          "id": "repository.create-fresh-number-array-collector"
        },
        {
          "name": "checkFreshNumberArrayCollector",
          "id": "repository.check-fresh-number-array-collector"
        },
        {
          "name": "encodeFreshNumberArrayCollector",
          "id": "repository.encode-fresh-number-array-collector"
        },
        {
          "name": "decodeFreshNumberArrayCollector",
          "id": "repository.decode-fresh-number-array-collector"
        },
        {
          "name": "fail",
          "id": "repository.raise-fresh-number-array-collector-error"
        },
        {
          "name": "equal",
          "id": "repository.check-fresh-number-array-collector-equality"
        },
        {
          "name": "codeOf",
          "id": "repository.read-fresh-number-array-collector-token-code"
        }
      ],
      "experiments/fresh-number-array-collectors/generate.mjs": [
        {
          "name": "freshNumberArrayCollectorRuleSource",
          "id": "repository.render-fresh-number-array-collector-rules"
        },
        {
          "name": "freshNumberArrayCollectorDecorators",
          "id": "repository.build-fresh-number-array-collector-decorators"
        }
      ],
      "experiments/fresh-number-array-collectors/generate-fixtures.mjs": [
        {
          "name": "freshNumberArrayCollectorFixtures",
          "id": "repository.build-fresh-number-array-collector-fixtures"
        }
      ]
    }
  },
  {
    "inventory": "fresh Number array collector actual premise ownership",
    "files": {
      "experiments/fresh-number-array-collectors/model.mjs": [
        {
          "name": "premiseOwner",
          "id": "repository.fresh-number-array-collector-premise-owner"
        }
      ]
    }
  },
  {
    "inventory": "fresh Number array collector lexical source data",
    "files": {
      "experiments/fresh-number-array-collectors/policy.mjs": [
        {
          "name": "sourceGrammarBasis",
          "id": "repository.fresh-number-array-collector-source-grammar-basis"
        },
        {
          "name": "unconditionallyReservedBindings",
          "id": "repository.fresh-number-array-collector-reserved-bindings"
        }
      ]
    }
  },
  {
    "inventory": "experiments/fresh-number-array-collectors/translated control wrappers",
    "files": {
      "experiments/fresh-number-array-collectors/data/rule-translation.lino": [
        {
          "name": "ml_acceptFreshNumberArrayCollectorPattern_loop1_result",
          "id": "repository.fresh-number-array-collector-binding-scan-control-result"
        },
        {
          "name": "MlAcceptFreshNumberArrayCollectorPatternLoop1Result",
          "id": "repository.fresh-number-array-collector-binding-scan-control-result"
        }
      ]
    }
  },
  {
    "inventory": "experiments/fresh-number-array-collectors/translated control constructors",
    "files": {
      "experiments/fresh-number-array-collectors/data/rule-translation.lino": [
        {
          "name": "MlAcceptFreshNumberArrayCollectorPatternLoop1Done",
          "id": "repository.fresh-number-array-collector-binding-scan-completion-constructor"
        },
        {
          "name": "MlAcceptFreshNumberArrayCollectorPatternLoop1Return",
          "id": "repository.fresh-number-array-collector-binding-scan-return-constructor"
        }
      ]
    }
  },
  {
    "inventory": "experiments/fresh-number-array-collectors/translated control continuations",
    "files": {
      "experiments/fresh-number-array-collectors/data/rule-translation.lino": [
        {
          "name": "ml_acceptFreshNumberArrayCollectorPattern_loop1",
          "id": "repository.continue-fresh-number-array-collector-binding-scan"
        },
        {
          "name": "ml_accept_fresh_number_array_collector_pattern_loop1",
          "id": "repository.continue-fresh-number-array-collector-binding-scan"
        }
      ]
    }
  },
  {
    "inventory": "fresh Number array collector actual declaration mapping owners",
    "files": {
      "experiments/fresh-number-array-collectors/model.mjs": [
        {
          "name": "checkProgramDeclarationOwners",
          "id": "translation.check-program-declaration-owners"
        }
      ]
    }
  },
  {
    "inventory": "fresh Number array collector unchanged proof encoder",
    "files": {
      "experiments/fresh-number-array-collectors/model.mjs": [
        {
          "name": "encodeGeneratedOwnedRust",
          "id": "repository.encode-generated-owned-rust"
        }
      ]
    }
  },
  {
    "inventory": "fresh Number array collector unchanged proof decoder",
    "files": {
      "experiments/fresh-number-array-collectors/model.mjs": [
        {
          "name": "decodeGeneratedOwnedRust",
          "id": "repository.decode-generated-owned-rust"
        }
      ]
    }
  },
  {
    "inventory": "fresh Number array collector independent source controls",
    "files": {
      "experiments/fresh-number-array-collectors/model.test.mjs": [
        {
          "name": "create",
          "id": "repository.create-fresh-number-array-collector-test-program"
        },
        {
          "name": "execute",
          "id": "repository.execute-fresh-number-array-collector-test-javascript"
        }
      ]
    }
  }
];
const normalizeOriginalLinkAndCollectorName=name=>name.replace(/([a-z0-9])([A-Z])/gu,'$1-$2').replaceAll('_','-').replaceAll(' ','-').toLowerCase().replace(/^remap-/u,'change-mapping-of-').replace(/\bsha256\b/gu,'secure-hash-algorithm-two-hundred-fifty-six').replace(/\bid\b/gu,'identity').replace(/\bml\b/gu,'meta-language').replace(/\b(loop|join)([1-7])\b/gu,(whole,kind,digit)=>kind+'-'+({1:'one',2:'two',3:'three',4:'four',5:'five',6:'six',7:'seven'})[digit]);
const ORIGINAL_LINK_AND_COLLECTOR_SOURCE_TEXTS=new Map();
const ORIGINAL_LINK_AND_COLLECTOR_NAME_INVENTORIES=ORIGINAL_LINK_AND_COLLECTOR_NAME_BINDINGS.map(({inventory,files})=>({
 inventory,files:Object.keys(files),
 extract:(raw,file)=>{let text=raw;if(file.endsWith('.lino')){const saved=ORIGINAL_LINK_AND_COLLECTOR_SOURCE_TEXTS.get(file);if(!saved||saved.raw!==raw)ORIGINAL_LINK_AND_COLLECTOR_SOURCE_TEXTS.set(file,{raw,text:JSON.stringify(readStructuredDataLino(raw))});text=ORIGINAL_LINK_AND_COLLECTOR_SOURCE_TEXTS.get(file).text;}return files[file].filter(({name})=>new RegExp('\\b'+name+'\\b','u').test(text)).map(({name})=>normalizeOriginalLinkAndCollectorName(name));},
 recordOf:name=>Object.values(files).flat().find(entry=>normalizeOriginalLinkAndCollectorName(entry.name)===name).id,
}));
// Exact finite global function admission and explicit collector Script context.
const FRESH_COLLECTOR_GLOBAL_CONTEXT_NAME_BINDINGS=[
  {
    "inventory": "fresh collector global function name decision",
    "files": {
      "experiments/fresh-number-array-collectors/rules.mjs": [
        {
          "name": "acceptFreshNumberArrayCollectorFunctionName",
          "id": "repository.accept-fresh-number-array-collector-function-name"
        }
      ],
      "experiments/fresh-number-array-collectors/model.mjs": [
        {
          "name": "acceptFreshNumberArrayCollectorFunctionName",
          "id": "repository.accept-fresh-number-array-collector-function-name"
        }
      ],
      "experiments/fresh-number-array-collectors/generate.mjs": [
        {
          "name": "acceptFreshNumberArrayCollectorFunctionName",
          "id": "repository.accept-fresh-number-array-collector-function-name"
        }
      ],
      "experiments/fresh-number-array-collectors/data/rule-translation.lino": [
        {
          "name": "accept_fresh_number_array_collector_function_name",
          "id": "repository.accept-fresh-number-array-collector-function-name"
        }
      ]
    }
  },
  {
    "inventory": "fresh collector explicit Script source context",
    "files": {
      "experiments/fresh-number-array-collectors/policy.mjs": [
        {
          "name": "forbiddenGlobalFunctionNames",
          "id": "repository.fresh-number-array-collector-global-function-exclusions"
        },
        {
          "name": "globalFunctionDeclarationBasis",
          "id": "repository.fresh-number-array-collector-global-function-authorities"
        },
        {
          "name": "sourceContext",
          "id": "repository.fresh-number-array-collector-source-context"
        },
        {
          "name": "sourceGoal",
          "id": "repository.fresh-number-array-collector-source-goal"
        },
        {
          "name": "initialGlobalRealm",
          "id": "repository.fresh-number-array-collector-initial-global-realm"
        },
        {
          "name": "initialGlobalValueProperties",
          "id": "repository.fresh-number-array-collector-global-value-properties"
        },
        {
          "name": "initialGlobalExtensibility",
          "id": "repository.fresh-number-array-collector-extensible-global-object"
        },
        {
          "name": "ambientDeclarationChanges",
          "id": "repository.fresh-number-array-collector-ambient-declaration-changes"
        }
      ],
      "experiments/fresh-number-array-collectors/data/policy.lino": [
        {
          "name": "forbiddenGlobalFunctionNames",
          "id": "repository.fresh-number-array-collector-global-function-exclusions"
        },
        {
          "name": "globalFunctionDeclarationBasis",
          "id": "repository.fresh-number-array-collector-global-function-authorities"
        },
        {
          "name": "sourceContext",
          "id": "repository.fresh-number-array-collector-source-context"
        },
        {
          "name": "sourceGoal",
          "id": "repository.fresh-number-array-collector-source-goal"
        },
        {
          "name": "initialGlobalRealm",
          "id": "repository.fresh-number-array-collector-initial-global-realm"
        },
        {
          "name": "initialGlobalValueProperties",
          "id": "repository.fresh-number-array-collector-global-value-properties"
        },
        {
          "name": "initialGlobalExtensibility",
          "id": "repository.fresh-number-array-collector-extensible-global-object"
        },
        {
          "name": "ambientDeclarationChanges",
          "id": "repository.fresh-number-array-collector-ambient-declaration-changes"
        }
      ],
      "experiments/fresh-number-array-collectors/model.mjs": [
        {
          "name": "sourceContext",
          "id": "repository.fresh-number-array-collector-source-context"
        }
      ]
    }
  }
];
const normalizeFreshCollectorGlobalContextName=name=>name.replace(/([a-z0-9])([A-Z])/gu,'$1-$2').replaceAll('_','-').toLowerCase().replace(/\bextensibility\b/gu,'capacity-for-extension');
const FRESH_COLLECTOR_GLOBAL_CONTEXT_SOURCE_TEXTS=new Map();
const FRESH_COLLECTOR_GLOBAL_CONTEXT_NAME_INVENTORIES=FRESH_COLLECTOR_GLOBAL_CONTEXT_NAME_BINDINGS.map(({inventory,files})=>({
 inventory,files:Object.keys(files),
 extract:(raw,file)=>{let text=raw;if(file.endsWith('.lino')){const saved=FRESH_COLLECTOR_GLOBAL_CONTEXT_SOURCE_TEXTS.get(file);if(!saved||saved.raw!==raw)FRESH_COLLECTOR_GLOBAL_CONTEXT_SOURCE_TEXTS.set(file,{raw,text:JSON.stringify(readStructuredDataLino(raw))});text=FRESH_COLLECTOR_GLOBAL_CONTEXT_SOURCE_TEXTS.get(file).text;}return files[file].filter(({name})=>new RegExp('\\b'+name+'\\b','u').test(text)).map(({name})=>normalizeFreshCollectorGlobalContextName(name));},
 recordOf:name=>Object.values(files).flat().find(entry=>normalizeFreshCollectorGlobalContextName(entry.name)===name).id,
}));
// Exact isolated String value, fixed table and complete source oracle meanings.
const ORIGINAL_LINK_VALUE_TABLE_NAME_BINDINGS=[
  {
    "inventory": "ordered original String value decisions",
    "files": {
      "experiments/original-link-symbol-projection/rules.mjs": [
        {
          "name": "acceptOriginalLinkSymbolInteger",
          "id": "repository.accept-original-link-symbol-integer"
        },
        {
          "name": "checkOriginalLinkSymbolTransport",
          "id": "repository.check-original-link-symbol-transport"
        },
        {
          "name": "equalOriginalLinkSymbolUnits",
          "id": "repository.compare-original-link-symbol-units"
        },
        {
          "name": "projectOriginalLinkSymbolValues",
          "id": "repository.project-original-link-symbol-values"
        }
      ],
      "experiments/original-link-symbol-projection/data/native-definitions.lino": [
        {
          "name": "accept_original_link_symbol_integer",
          "id": "repository.accept-original-link-symbol-integer"
        },
        {
          "name": "check_original_link_symbol_transport",
          "id": "repository.check-original-link-symbol-transport"
        },
        {
          "name": "equal_original_link_symbol_units",
          "id": "repository.compare-original-link-symbol-units"
        },
        {
          "name": "project_original_link_symbol_values",
          "id": "repository.project-original-link-symbol-values"
        }
      ]
    }
  },
  {
    "inventory": "ordered original String projection source authoring",
    "files": {
      "experiments/original-link-symbol-projection/policy.mjs": [
        {
          "name": "originalLinkSymbolPolicy",
          "id": "repository.original-link-symbol-policy"
        },
        {
          "name": "originalLinkSymbolRulesSource",
          "id": "repository.render-original-link-symbol-rules"
        }
      ],
      "experiments/original-link-symbol-projection/generate.mjs": [
        {
          "name": "generateOriginalLinkSymbolProjection",
          "id": "repository.generate-original-link-symbol-data"
        }
      ]
    }
  },
  {
    "inventory": "ordered original String projection data roles",
    "files": {
      "experiments/original-link-symbol-projection/rules.mjs": [
        {
          "name": "symbols",
          "id": "repository.original-link-string-value-symbols"
        },
        {
          "name": "names",
          "id": "repository.original-link-string-value-name-ranges"
        },
        {
          "name": "units",
          "id": "repository.original-link-string-value-code-units"
        }
      ]
    }
  },
  {
    "inventory": "experiments/original-link-symbol-projection/translated control wrappers",
    "files": {
      "experiments/original-link-symbol-projection/data/native-definitions.lino": [
        {
          "name": "ml_checkOriginalLinkSymbolTransport_loop2_result",
          "id": "repository.original-link-symbol-code-unit-scan-control-result"
        },
        {
          "name": "MlCheckOriginalLinkSymbolTransportLoop2Result",
          "id": "repository.original-link-symbol-code-unit-scan-control-result"
        },
        {
          "name": "ml_checkOriginalLinkSymbolTransport_loop1_result",
          "id": "repository.original-link-symbol-offset-scan-control-result"
        },
        {
          "name": "MlCheckOriginalLinkSymbolTransportLoop1Result",
          "id": "repository.original-link-symbol-offset-scan-control-result"
        },
        {
          "name": "ml_equalOriginalLinkSymbolUnits_loop3_result",
          "id": "repository.original-link-symbol-span-comparison-control-result"
        },
        {
          "name": "MlEqualOriginalLinkSymbolUnitsLoop3Result",
          "id": "repository.original-link-symbol-span-comparison-control-result"
        },
        {
          "name": "ml_projectOriginalLinkSymbolValues_loop6_result",
          "id": "repository.original-link-symbol-source-construction-control-result"
        },
        {
          "name": "MlProjectOriginalLinkSymbolValuesLoop6Result",
          "id": "repository.original-link-symbol-source-construction-control-result"
        },
        {
          "name": "ml_projectOriginalLinkSymbolValues_join8",
          "id": "repository.original-link-symbol-source-branch-state-control-result"
        },
        {
          "name": "MlProjectOriginalLinkSymbolValuesJoin8",
          "id": "repository.original-link-symbol-source-branch-state-control-result"
        },
        {
          "name": "ml_projectOriginalLinkSymbolValues_loop5_result",
          "id": "repository.original-link-symbol-fixed-construction-control-result"
        },
        {
          "name": "MlProjectOriginalLinkSymbolValuesLoop5Result",
          "id": "repository.original-link-symbol-fixed-construction-control-result"
        },
        {
          "name": "ml_projectOriginalLinkSymbolValues_loop4_result",
          "id": "repository.original-link-symbol-fixed-duplicate-scan-control-result"
        },
        {
          "name": "MlProjectOriginalLinkSymbolValuesLoop4Result",
          "id": "repository.original-link-symbol-fixed-duplicate-scan-control-result"
        },
        {
          "name": "ml_projectOriginalLinkSymbolValues_loop11_result",
          "id": "repository.original-link-symbol-prior-fixed-comparison-control-result"
        },
        {
          "name": "MlProjectOriginalLinkSymbolValuesLoop11Result",
          "id": "repository.original-link-symbol-prior-fixed-comparison-control-result"
        }
      ]
    }
  },
  {
    "inventory": "experiments/original-link-symbol-projection/translated state constructors",
    "files": {
      "experiments/original-link-symbol-projection/data/native-definitions.lino": [
        {
          "name": "MlCheckOriginalLinkSymbolTransportLoop2Done",
          "id": "repository.original-link-symbol-code-unit-scan-completion-constructor"
        },
        {
          "name": "MlCheckOriginalLinkSymbolTransportLoop2Return",
          "id": "repository.original-link-symbol-code-unit-scan-return-constructor"
        },
        {
          "name": "MlCheckOriginalLinkSymbolTransportLoop1Done",
          "id": "repository.original-link-symbol-offset-scan-completion-constructor"
        },
        {
          "name": "MlCheckOriginalLinkSymbolTransportLoop1Return",
          "id": "repository.original-link-symbol-offset-scan-return-constructor"
        },
        {
          "name": "MlEqualOriginalLinkSymbolUnitsLoop3Done",
          "id": "repository.original-link-symbol-span-comparison-completion-constructor"
        },
        {
          "name": "MlEqualOriginalLinkSymbolUnitsLoop3Return",
          "id": "repository.original-link-symbol-span-comparison-return-constructor"
        },
        {
          "name": "MlProjectOriginalLinkSymbolValuesLoop6Done",
          "id": "repository.original-link-symbol-source-construction-completion-constructor"
        },
        {
          "name": "MlProjectOriginalLinkSymbolValuesJoin8Next",
          "id": "repository.original-link-symbol-source-branch-state-next-state-constructor"
        },
        {
          "name": "MlProjectOriginalLinkSymbolValuesLoop5Done",
          "id": "repository.original-link-symbol-fixed-construction-completion-constructor"
        },
        {
          "name": "MlProjectOriginalLinkSymbolValuesLoop4Done",
          "id": "repository.original-link-symbol-fixed-duplicate-scan-completion-constructor"
        },
        {
          "name": "MlProjectOriginalLinkSymbolValuesLoop4Return",
          "id": "repository.original-link-symbol-fixed-duplicate-scan-return-constructor"
        },
        {
          "name": "MlProjectOriginalLinkSymbolValuesLoop11Done",
          "id": "repository.original-link-symbol-prior-fixed-comparison-completion-constructor"
        },
        {
          "name": "MlProjectOriginalLinkSymbolValuesLoop11Return",
          "id": "repository.original-link-symbol-prior-fixed-comparison-return-constructor"
        }
      ]
    }
  },
  {
    "inventory": "experiments/original-link-symbol-projection/translated continuations",
    "files": {
      "experiments/original-link-symbol-projection/data/native-definitions.lino": [
        {
          "name": "ml_checkOriginalLinkSymbolTransport_loop2",
          "id": "repository.continue-original-link-symbol-code-unit-scan"
        },
        {
          "name": "ml_check_original_link_symbol_transport_loop2",
          "id": "repository.continue-original-link-symbol-code-unit-scan"
        },
        {
          "name": "ml_checkOriginalLinkSymbolTransport_loop1",
          "id": "repository.continue-original-link-symbol-offset-scan"
        },
        {
          "name": "ml_check_original_link_symbol_transport_loop1",
          "id": "repository.continue-original-link-symbol-offset-scan"
        },
        {
          "name": "ml_equalOriginalLinkSymbolUnits_loop3",
          "id": "repository.continue-original-link-symbol-span-comparison"
        },
        {
          "name": "ml_equal_original_link_symbol_units_loop3",
          "id": "repository.continue-original-link-symbol-span-comparison"
        },
        {
          "name": "ml_projectOriginalLinkSymbolValues_loop7",
          "id": "repository.continue-original-link-symbol-existing-name-search"
        },
        {
          "name": "ml_project_original_link_symbol_values_loop7",
          "id": "repository.continue-original-link-symbol-existing-name-search"
        },
        {
          "name": "ml_projectOriginalLinkSymbolValues_loop9",
          "id": "repository.continue-original-link-symbol-dynamic-unit-copy"
        },
        {
          "name": "ml_project_original_link_symbol_values_loop9",
          "id": "repository.continue-original-link-symbol-dynamic-unit-copy"
        },
        {
          "name": "ml_projectOriginalLinkSymbolValues_loop6",
          "id": "repository.continue-original-link-symbol-source-construction"
        },
        {
          "name": "ml_project_original_link_symbol_values_loop6",
          "id": "repository.continue-original-link-symbol-source-construction"
        },
        {
          "name": "ml_projectOriginalLinkSymbolValues_loop10",
          "id": "repository.continue-original-link-symbol-fixed-unit-copy"
        },
        {
          "name": "ml_project_original_link_symbol_values_loop10",
          "id": "repository.continue-original-link-symbol-fixed-unit-copy"
        },
        {
          "name": "ml_projectOriginalLinkSymbolValues_loop5",
          "id": "repository.continue-original-link-symbol-fixed-construction"
        },
        {
          "name": "ml_project_original_link_symbol_values_loop5",
          "id": "repository.continue-original-link-symbol-fixed-construction"
        },
        {
          "name": "ml_projectOriginalLinkSymbolValues_loop11",
          "id": "repository.continue-original-link-symbol-prior-fixed-comparison"
        },
        {
          "name": "ml_project_original_link_symbol_values_loop11",
          "id": "repository.continue-original-link-symbol-prior-fixed-comparison"
        },
        {
          "name": "ml_projectOriginalLinkSymbolValues_loop4",
          "id": "repository.continue-original-link-symbol-fixed-duplicate-scan"
        },
        {
          "name": "ml_project_original_link_symbol_values_loop4",
          "id": "repository.continue-original-link-symbol-fixed-duplicate-scan"
        }
      ]
    }
  },
  {
    "inventory": "original link fixed symbol table decisions",
    "files": {
      "experiments/original-link-table-projection/rules.mjs": [
        {
          "name": "readOriginalLinkTableIdentity",
          "id": "repository.read-original-link-table-identity"
        },
        {
          "name": "readOriginalLinkTableSymbol",
          "id": "repository.read-original-link-table-symbol"
        },
        {
          "name": "readOriginalLinkTableNumber",
          "id": "repository.read-original-link-table-number"
        },
        {
          "name": "readOriginalLinkTableOperationCode",
          "id": "repository.read-original-link-table-operation-code"
        },
        {
          "name": "readOriginalLinkTableRole",
          "id": "repository.read-original-link-table-role"
        },
        {
          "name": "readOriginalLinkTablePrefix",
          "id": "repository.read-original-link-table-prefix"
        },
        {
          "name": "readOriginalLinkTableLayers",
          "id": "repository.read-original-link-table-layers"
        },
        {
          "name": "projectOriginalLinkCertificateTables",
          "id": "repository.project-original-link-certificate-tables"
        }
      ],
      "experiments/original-link-table-projection/data/native-definitions.lino": [
        {
          "name": "read_original_link_table_identity",
          "id": "repository.read-original-link-table-identity"
        },
        {
          "name": "read_original_link_table_symbol",
          "id": "repository.read-original-link-table-symbol"
        },
        {
          "name": "read_original_link_table_number",
          "id": "repository.read-original-link-table-number"
        },
        {
          "name": "read_original_link_table_operation_code",
          "id": "repository.read-original-link-table-operation-code"
        },
        {
          "name": "read_original_link_table_role",
          "id": "repository.read-original-link-table-role"
        },
        {
          "name": "read_original_link_table_prefix",
          "id": "repository.read-original-link-table-prefix"
        },
        {
          "name": "read_original_link_table_layers",
          "id": "repository.read-original-link-table-layers"
        },
        {
          "name": "project_original_link_certificate_tables",
          "id": "repository.project-original-link-certificate-tables"
        }
      ]
    }
  },
  {
    "inventory": "original link fixed symbol source operations",
    "files": {
      "experiments/original-link-table-projection/policy.mjs": [
        {
          "name": "originalLinkTablePolicy",
          "id": "repository.original-link-table-policy"
        },
        {
          "name": "originalLinkTableRulesSource",
          "id": "repository.render-original-link-table-rules"
        }
      ],
      "experiments/original-link-table-projection/transport.mjs": [
        {
          "name": "ORIGINAL_LINK_TABLE_POLICY",
          "id": "repository.original-link-table-policy"
        },
        {
          "name": "readOriginalLinkTableTransport",
          "id": "repository.read-original-link-table-transport"
        },
        {
          "name": "projectOriginalLinkFixedSymbolTables",
          "id": "repository.project-original-link-fixed-symbol-tables"
        }
      ],
      "experiments/original-link-table-projection/generate.mjs": [
        {
          "name": "generateOriginalLinkTableProjection",
          "id": "repository.generate-original-link-table-data"
        }
      ]
    }
  },
  {
    "inventory": "original link table exact scalar transport roles",
    "files": {
      "experiments/original-link-table-projection/rules.mjs": [
        {
          "name": "scalarKinds",
          "id": "repository.original-link-table-scalar-tags"
        },
        {
          "name": "scalarNumbers",
          "id": "repository.original-link-table-number-payloads"
        },
        {
          "name": "scalarSymbols",
          "id": "repository.original-link-table-fixed-symbol-codes"
        }
      ],
      "experiments/original-link-table-projection/policy.mjs": [
        {
          "name": "operationMarker",
          "id": "repository.original-link-table-operation-marker"
        }
      ]
    }
  },
  {
    "inventory": "original link table linked instruction authoring",
    "files": {
      "experiments/original-link-table-projection/policy.mjs": [
        {
          "name": "cell",
          "id": "repository.create-original-link-table-column-instruction"
        },
        {
          "name": "f",
          "id": "repository.read-original-link-table-field-code"
        },
        {
          "name": "fld",
          "id": "repository.render-original-link-table-field-path"
        },
        {
          "name": "refs",
          "id": "repository.render-original-link-table-reference-read"
        },
        {
          "name": "identity",
          "id": "repository.render-original-link-table-identity-read"
        },
        {
          "name": "symbol",
          "id": "repository.render-original-link-table-symbol-read"
        },
        {
          "name": "number",
          "id": "repository.render-original-link-table-number-read"
        },
        {
          "name": "expression",
          "id": "repository.render-original-link-table-cell-expression"
        }
      ]
    }
  },
  {
    "inventory": "original link table linked column data",
    "files": {
      "experiments/original-link-table-projection/policy.mjs": [
        {
          "name": "tables",
          "id": "repository.original-link-table-column-instructions"
        }
      ],
      "experiments/original-link-table-projection/data/policy.lino": [
        {
          "name": "tables",
          "id": "repository.original-link-table-column-instructions"
        }
      ]
    }
  },
  {
    "inventory": "table exact shared role declaration readOriginalLinkRoleField",
    "files": {
      "experiments/original-link-table-projection/data/native-definitions.lino": [
        {
          "name": "readOriginalLinkRoleField",
          "id": "repository.read-original-link-role-field"
        },
        {
          "name": "read_original_link_role_field",
          "id": "repository.read-original-link-role-field"
        }
      ]
    }
  },
  {
    "inventory": "table exact shared role source readOriginalLinkRoleField",
    "files": {
      "experiments/original-link-table-projection/rules.mjs": [
        {
          "name": "readOriginalLinkRoleField",
          "id": "repository.read-original-link-role-field"
        }
      ]
    }
  },
  {
    "inventory": "table exact shared role declaration readOriginalLinkRoleReferences",
    "files": {
      "experiments/original-link-table-projection/data/native-definitions.lino": [
        {
          "name": "readOriginalLinkRoleReferences",
          "id": "repository.read-original-link-role-references"
        },
        {
          "name": "read_original_link_role_references",
          "id": "repository.read-original-link-role-references"
        }
      ]
    }
  },
  {
    "inventory": "table exact shared role source readOriginalLinkRoleReferences",
    "files": {
      "experiments/original-link-table-projection/rules.mjs": [
        {
          "name": "readOriginalLinkRoleReferences",
          "id": "repository.read-original-link-role-references"
        }
      ]
    }
  },
  {
    "inventory": "table exact shared role declaration readOriginalLinkRoleOperation",
    "files": {
      "experiments/original-link-table-projection/data/native-definitions.lino": [
        {
          "name": "readOriginalLinkRoleOperation",
          "id": "repository.accept-original-link-role-operation"
        },
        {
          "name": "read_original_link_role_operation",
          "id": "repository.accept-original-link-role-operation"
        }
      ]
    }
  },
  {
    "inventory": "table exact shared role source readOriginalLinkRoleOperation",
    "files": {
      "experiments/original-link-table-projection/rules.mjs": [
        {
          "name": "readOriginalLinkRoleOperation",
          "id": "repository.accept-original-link-role-operation"
        }
      ]
    }
  },
  {
    "inventory": "table exact shared role declaration firstOriginalLinkRoleOccurrence",
    "files": {
      "experiments/original-link-table-projection/data/native-definitions.lino": [
        {
          "name": "firstOriginalLinkRoleOccurrence",
          "id": "repository.accept-first-original-link-role-occurrence"
        },
        {
          "name": "first_original_link_role_occurrence",
          "id": "repository.accept-first-original-link-role-occurrence"
        }
      ]
    }
  },
  {
    "inventory": "table exact shared role source firstOriginalLinkRoleOccurrence",
    "files": {
      "experiments/original-link-table-projection/rules.mjs": [
        {
          "name": "firstOriginalLinkRoleOccurrence",
          "id": "repository.accept-first-original-link-role-occurrence"
        }
      ]
    }
  },
  {
    "inventory": "table exact shared role declaration readOriginalLinkRoleKind",
    "files": {
      "experiments/original-link-table-projection/data/native-definitions.lino": [
        {
          "name": "readOriginalLinkRoleKind",
          "id": "repository.read-original-link-concrete-role-kind"
        },
        {
          "name": "read_original_link_role_kind",
          "id": "repository.read-original-link-concrete-role-kind"
        }
      ]
    }
  },
  {
    "inventory": "table exact shared role source readOriginalLinkRoleKind",
    "files": {
      "experiments/original-link-table-projection/rules.mjs": [
        {
          "name": "readOriginalLinkRoleKind",
          "id": "repository.read-original-link-concrete-role-kind"
        }
      ]
    }
  },
  {
    "inventory": "table exact shared role declaration registerOriginalLinkRoles",
    "files": {
      "experiments/original-link-table-projection/data/native-definitions.lino": [
        {
          "name": "registerOriginalLinkRoles",
          "id": "repository.register-original-link-certificate-roles"
        },
        {
          "name": "register_original_link_roles",
          "id": "repository.register-original-link-certificate-roles"
        }
      ]
    }
  },
  {
    "inventory": "table exact shared role source registerOriginalLinkRoles",
    "files": {
      "experiments/original-link-table-projection/rules.mjs": [
        {
          "name": "registerOriginalLinkRoles",
          "id": "repository.register-original-link-certificate-roles"
        }
      ]
    }
  },
  {
    "inventory": "table exact shared role declaration projectOriginalLinkRoleRegistry",
    "files": {
      "experiments/original-link-table-projection/data/native-definitions.lino": [
        {
          "name": "projectOriginalLinkRoleRegistry",
          "id": "repository.project-original-link-role-registry"
        },
        {
          "name": "project_original_link_role_registry",
          "id": "repository.project-original-link-role-registry"
        }
      ]
    }
  },
  {
    "inventory": "table exact shared role source projectOriginalLinkRoleRegistry",
    "files": {
      "experiments/original-link-table-projection/rules.mjs": [
        {
          "name": "projectOriginalLinkRoleRegistry",
          "id": "repository.project-original-link-role-registry"
        }
      ]
    }
  },
  {
    "inventory": "table exact shared role declaration ml_readOriginalLinkRoleField_loop1_result",
    "files": {
      "experiments/original-link-table-projection/data/native-definitions.lino": [
        {
          "name": "ml_readOriginalLinkRoleField_loop1_result",
          "id": "repository.original-link-role-unique-field-scan-control-result"
        },
        {
          "name": "MlReadOriginalLinkRoleFieldLoop1Result",
          "id": "repository.original-link-role-unique-field-scan-control-result"
        }
      ]
    }
  },
  {
    "inventory": "table exact shared role state constructors",
    "files": {
      "experiments/original-link-table-projection/data/native-definitions.lino": [
        {
          "name": "MlReadOriginalLinkRoleFieldLoop1Done",
          "id": "repository.original-link-role-unique-field-scan-completion-constructor"
        },
        {
          "name": "MlReadOriginalLinkRoleFieldLoop1Return",
          "id": "repository.original-link-role-unique-field-scan-return-constructor"
        },
        {
          "name": "MlFirstOriginalLinkRoleOccurrenceLoop2Done",
          "id": "repository.original-link-role-first-occurrence-scan-completion-constructor"
        },
        {
          "name": "MlFirstOriginalLinkRoleOccurrenceLoop2Return",
          "id": "repository.original-link-role-first-occurrence-scan-return-constructor"
        },
        {
          "name": "MlRegisterOriginalLinkRolesLoop5Done",
          "id": "repository.original-link-role-unresolved-kind-scan-completion-constructor"
        },
        {
          "name": "MlRegisterOriginalLinkRolesLoop5Return",
          "id": "repository.original-link-role-unresolved-kind-scan-return-constructor"
        },
        {
          "name": "MlRegisterOriginalLinkRolesLoop4Done",
          "id": "repository.original-link-role-occurrence-validation-completion-constructor"
        },
        {
          "name": "MlRegisterOriginalLinkRolesLoop4Return",
          "id": "repository.original-link-role-occurrence-validation-return-constructor"
        },
        {
          "name": "MlRegisterOriginalLinkRolesLoop6Done",
          "id": "repository.original-link-role-contradiction-scan-completion-constructor"
        },
        {
          "name": "MlRegisterOriginalLinkRolesLoop6Return",
          "id": "repository.original-link-role-contradiction-scan-return-constructor"
        },
        {
          "name": "MlProjectOriginalLinkRoleRegistryLoop7Done",
          "id": "repository.original-link-role-ordered-operation-traversal-completion-constructor"
        },
        {
          "name": "MlProjectOriginalLinkRoleRegistryLoop7Return",
          "id": "repository.original-link-role-ordered-operation-traversal-return-constructor"
        }
      ]
    }
  },
  {
    "inventory": "table exact shared role declaration ml_readOriginalLinkRoleField_loop1",
    "files": {
      "experiments/original-link-table-projection/data/native-definitions.lino": [
        {
          "name": "ml_readOriginalLinkRoleField_loop1",
          "id": "repository.continue-original-link-role-unique-field-scan"
        },
        {
          "name": "ml_read_original_link_role_field_loop1",
          "id": "repository.continue-original-link-role-unique-field-scan"
        }
      ]
    }
  },
  {
    "inventory": "table exact shared role declaration ml_firstOriginalLinkRoleOccurrence_loop2_result",
    "files": {
      "experiments/original-link-table-projection/data/native-definitions.lino": [
        {
          "name": "ml_firstOriginalLinkRoleOccurrence_loop2_result",
          "id": "repository.original-link-role-first-occurrence-scan-control-result"
        },
        {
          "name": "MlFirstOriginalLinkRoleOccurrenceLoop2Result",
          "id": "repository.original-link-role-first-occurrence-scan-control-result"
        }
      ]
    }
  },
  {
    "inventory": "table exact shared role declaration ml_firstOriginalLinkRoleOccurrence_loop2",
    "files": {
      "experiments/original-link-table-projection/data/native-definitions.lino": [
        {
          "name": "ml_firstOriginalLinkRoleOccurrence_loop2",
          "id": "repository.continue-original-link-role-first-occurrence-scan"
        },
        {
          "name": "ml_first_original_link_role_occurrence_loop2",
          "id": "repository.continue-original-link-role-first-occurrence-scan"
        }
      ]
    }
  },
  {
    "inventory": "table exact shared role declaration ml_readOriginalLinkRoleKind_loop3",
    "files": {
      "experiments/original-link-table-projection/data/native-definitions.lino": [
        {
          "name": "ml_readOriginalLinkRoleKind_loop3",
          "id": "repository.continue-original-link-role-concrete-kind-scan"
        },
        {
          "name": "ml_read_original_link_role_kind_loop3",
          "id": "repository.continue-original-link-role-concrete-kind-scan"
        }
      ]
    }
  },
  {
    "inventory": "table exact shared role declaration ml_registerOriginalLinkRoles_loop5_result",
    "files": {
      "experiments/original-link-table-projection/data/native-definitions.lino": [
        {
          "name": "ml_registerOriginalLinkRoles_loop5_result",
          "id": "repository.original-link-role-unresolved-kind-scan-control-result"
        },
        {
          "name": "MlRegisterOriginalLinkRolesLoop5Result",
          "id": "repository.original-link-role-unresolved-kind-scan-control-result"
        }
      ]
    }
  },
  {
    "inventory": "table exact shared role declaration ml_registerOriginalLinkRoles_loop5",
    "files": {
      "experiments/original-link-table-projection/data/native-definitions.lino": [
        {
          "name": "ml_registerOriginalLinkRoles_loop5",
          "id": "repository.continue-original-link-role-unresolved-kind-scan"
        },
        {
          "name": "ml_register_original_link_roles_loop5",
          "id": "repository.continue-original-link-role-unresolved-kind-scan"
        }
      ]
    }
  },
  {
    "inventory": "table exact shared role declaration ml_registerOriginalLinkRoles_loop4_result",
    "files": {
      "experiments/original-link-table-projection/data/native-definitions.lino": [
        {
          "name": "ml_registerOriginalLinkRoles_loop4_result",
          "id": "repository.original-link-role-occurrence-validation-control-result"
        },
        {
          "name": "MlRegisterOriginalLinkRolesLoop4Result",
          "id": "repository.original-link-role-occurrence-validation-control-result"
        }
      ]
    }
  },
  {
    "inventory": "table exact shared role declaration ml_registerOriginalLinkRoles_loop6_result",
    "files": {
      "experiments/original-link-table-projection/data/native-definitions.lino": [
        {
          "name": "ml_registerOriginalLinkRoles_loop6_result",
          "id": "repository.original-link-role-contradiction-scan-control-result"
        },
        {
          "name": "MlRegisterOriginalLinkRolesLoop6Result",
          "id": "repository.original-link-role-contradiction-scan-control-result"
        }
      ]
    }
  },
  {
    "inventory": "table exact shared role declaration ml_registerOriginalLinkRoles_loop6",
    "files": {
      "experiments/original-link-table-projection/data/native-definitions.lino": [
        {
          "name": "ml_registerOriginalLinkRoles_loop6",
          "id": "repository.continue-original-link-role-contradiction-scan"
        },
        {
          "name": "ml_register_original_link_roles_loop6",
          "id": "repository.continue-original-link-role-contradiction-scan"
        }
      ]
    }
  },
  {
    "inventory": "table exact shared role declaration ml_registerOriginalLinkRoles_loop4",
    "files": {
      "experiments/original-link-table-projection/data/native-definitions.lino": [
        {
          "name": "ml_registerOriginalLinkRoles_loop4",
          "id": "repository.continue-original-link-role-occurrence-validation"
        },
        {
          "name": "ml_register_original_link_roles_loop4",
          "id": "repository.continue-original-link-role-occurrence-validation"
        }
      ]
    }
  },
  {
    "inventory": "table exact shared role declaration ml_projectOriginalLinkRoleRegistry_loop7_result",
    "files": {
      "experiments/original-link-table-projection/data/native-definitions.lino": [
        {
          "name": "ml_projectOriginalLinkRoleRegistry_loop7_result",
          "id": "repository.original-link-role-ordered-operation-traversal-control-result"
        },
        {
          "name": "MlProjectOriginalLinkRoleRegistryLoop7Result",
          "id": "repository.original-link-role-ordered-operation-traversal-control-result"
        }
      ]
    }
  },
  {
    "inventory": "table exact shared role declaration ml_projectOriginalLinkRoleRegistry_loop7",
    "files": {
      "experiments/original-link-table-projection/data/native-definitions.lino": [
        {
          "name": "ml_projectOriginalLinkRoleRegistry_loop7",
          "id": "repository.continue-original-link-role-ordered-operation-traversal"
        },
        {
          "name": "ml_project_original_link_role_registry_loop7",
          "id": "repository.continue-original-link-role-ordered-operation-traversal"
        }
      ]
    }
  },
  {
    "inventory": "experiments/original-link-table-projection/translated continuations",
    "files": {
      "experiments/original-link-table-projection/data/native-definitions.lino": [
        {
          "name": "ml_readOriginalLinkTablePrefix_loop8",
          "id": "repository.continue-original-link-table-prefix-scan"
        },
        {
          "name": "ml_read_original_link_table_prefix_loop8",
          "id": "repository.continue-original-link-table-prefix-scan"
        }
      ]
    }
  },
  {
    "inventory": "table exact reversible field names",
    "files": {
      "experiments/original-link-table-projection/transport.mjs": [
        {
          "name": "fieldName",
          "id": "repository.read-owned-text-certificate-field-name"
        }
      ]
    }
  },
  {
    "inventory": "table exact original role host",
    "files": {
      "experiments/original-link-table-projection/transport.mjs": [
        {
          "name": "projectOriginalLinkCertificateRoles",
          "id": "repository.project-original-link-certificate-roles"
        }
      ]
    }
  },
  {
    "inventory": "table exact shared role source renderer",
    "files": {
      "experiments/original-link-table-projection/policy.mjs": [
        {
          "name": "originalLinkRoleRulesSource",
          "id": "repository.render-original-link-role-rules"
        }
      ]
    }
  },
  {
    "inventory": "table exact shared role policy",
    "files": {
      "experiments/original-link-table-projection/policy.mjs": [
        {
          "name": "originalLinkRolePolicy",
          "id": "repository.original-link-role-policy"
        }
      ]
    }
  },
  {
    "inventory": "existing complete certificate table role context",
    "files": {
      "experiments/original-link-table-projection/rules.mjs": [
        {
          "name": "table_context",
          "id": "repository.text-certificate-context-table"
        }
      ]
    }
  },
  {
    "inventory": "existing complete certificate table role kinds",
    "files": {
      "experiments/original-link-table-projection/rules.mjs": [
        {
          "name": "kinds",
          "id": "repository.text-certificate-identity-kind-table"
        }
      ]
    }
  },
  {
    "inventory": "existing complete certificate table role positions",
    "files": {
      "experiments/original-link-table-projection/rules.mjs": [
        {
          "name": "table_positions",
          "id": "repository.text-certificate-source-position-table"
        }
      ]
    }
  },
  {
    "inventory": "existing complete certificate table role paths",
    "files": {
      "experiments/original-link-table-projection/rules.mjs": [
        {
          "name": "table_paths",
          "id": "repository.text-certificate-owner-path-table"
        }
      ]
    }
  },
  {
    "inventory": "existing complete certificate table role layers",
    "files": {
      "experiments/original-link-table-projection/rules.mjs": [
        {
          "name": "table_layers",
          "id": "repository.text-certificate-reference-layer-table"
        }
      ]
    }
  },
  {
    "inventory": "existing complete certificate table role functions",
    "files": {
      "experiments/original-link-table-projection/rules.mjs": [
        {
          "name": "table_functions",
          "id": "repository.text-certificate-function-fact-table"
        }
      ]
    }
  },
  {
    "inventory": "existing complete certificate table role operations",
    "files": {
      "experiments/original-link-table-projection/rules.mjs": [
        {
          "name": "table_operations",
          "id": "repository.text-certificate-operation-table"
        }
      ]
    }
  },
  {
    "inventory": "existing complete certificate table role parts",
    "files": {
      "experiments/original-link-table-projection/rules.mjs": [
        {
          "name": "table_parts",
          "id": "repository.text-certificate-operation-part-table"
        }
      ]
    }
  },
  {
    "inventory": "existing complete certificate table role inputs",
    "files": {
      "experiments/original-link-table-projection/rules.mjs": [
        {
          "name": "table_inputs",
          "id": "repository.text-certificate-input-condition-table"
        }
      ]
    }
  },
  {
    "inventory": "existing complete certificate table role allocations",
    "files": {
      "experiments/original-link-table-projection/rules.mjs": [
        {
          "name": "table_allocations",
          "id": "repository.text-certificate-allocation-condition-table"
        }
      ]
    }
  },
  {
    "inventory": "existing complete certificate table role items",
    "files": {
      "experiments/original-link-table-projection/rules.mjs": [
        {
          "name": "table_items",
          "id": "repository.text-certificate-declaration-order-table"
        }
      ]
    }
  },
  {
    "inventory": "original String independent reference operations",
    "files": {
      "experiments/original-link-symbol-projection/projection.test.mjs": [
        {
          "name": "transport",
          "id": "repository.collect-original-string-test-units"
        },
        {
          "name": "reference",
          "id": "repository.project-original-string-reference-values"
        }
      ],
      "experiments/original-link-symbol-projection/native-observations.mjs": [
        {
          "name": "transport",
          "id": "repository.collect-original-string-test-units"
        },
        {
          "name": "reference",
          "id": "repository.project-original-string-reference-values"
        }
      ]
    }
  },
  {
    "inventory": "original String independent projection controls",
    "files": {
      "experiments/original-link-symbol-projection/projection.test.mjs": [
        {
          "name": "argumentsFor",
          "id": "repository.create-original-string-test-arguments"
        },
        {
          "name": "unpack",
          "id": "repository.read-original-string-test-output"
        },
        {
          "name": "assertProjection",
          "id": "repository.check-original-string-test-projection"
        },
        {
          "name": "stringsFrom",
          "id": "repository.read-original-certificate-test-strings"
        }
      ]
    }
  },
  {
    "inventory": "table existing exact original test provider key",
    "files": {
      "experiments/original-link-table-projection/projection.test.mjs": [
        {
          "name": "key",
          "id": "repository.read-exact-link-identity-key"
        }
      ]
    }
  },
  {
    "inventory": "table existing exact original test provider label",
    "files": {
      "experiments/original-link-table-projection/projection.test.mjs": [
        {
          "name": "label",
          "id": "repository.render-certificate-test-field-label"
        }
      ]
    }
  },
  {
    "inventory": "table existing exact original test provider field",
    "files": {
      "experiments/original-link-table-projection/projection.test.mjs": [
        {
          "name": "field",
          "id": "repository.read-unique-original-link-role-test-field"
        }
      ]
    }
  },
  {
    "inventory": "table existing exact original test provider refs",
    "files": {
      "experiments/original-link-table-projection/projection.test.mjs": [
        {
          "name": "refs",
          "id": "repository.read-original-link-role-test-references"
        }
      ]
    }
  },
  {
    "inventory": "table existing exact original test provider body",
    "files": {
      "experiments/original-link-table-projection/projection.test.mjs": [
        {
          "name": "body",
          "id": "repository.read-original-link-role-test-body"
        }
      ]
    }
  },
  {
    "inventory": "table existing exact original test provider proof",
    "files": {
      "experiments/original-link-table-projection/projection.test.mjs": [
        {
          "name": "proof",
          "id": "repository.read-original-link-role-test-proof"
        }
      ]
    }
  },
  {
    "inventory": "table existing exact original test provider rows",
    "files": {
      "experiments/original-link-table-projection/projection.test.mjs": [
        {
          "name": "rows",
          "id": "repository.read-original-link-role-test-proof-rows"
        }
      ]
    }
  },
  {
    "inventory": "table existing exact original test provider replace",
    "files": {
      "experiments/original-link-table-projection/projection.test.mjs": [
        {
          "name": "replace",
          "id": "repository.replace-text-certificate-test-field"
        }
      ]
    }
  },
  {
    "inventory": "table existing exact original test provider attach",
    "files": {
      "experiments/original-link-table-projection/projection.test.mjs": [
        {
          "name": "attach",
          "id": "repository.attach-owned-text-certificate-test-proof"
        }
      ]
    }
  },
  {
    "inventory": "table existing exact original test provider create",
    "files": {
      "experiments/original-link-table-projection/projection.test.mjs": [
        {
          "name": "create",
          "id": "repository.create-original-link-role-test-packet"
        }
      ]
    }
  },
  {
    "inventory": "original link table independent controls",
    "files": {
      "experiments/original-link-table-projection/projection.test.mjs": [
        {
          "name": "copy",
          "id": "repository.copy-original-link-table-test-arguments"
        },
        {
          "name": "expectedTables",
          "id": "repository.project-original-link-table-reference-correspondence"
        },
        {
          "name": "compare",
          "id": "repository.check-original-link-table-reference-output"
        },
        {
          "name": "tableRows",
          "id": "repository.read-original-link-table-test-output"
        },
        {
          "name": "ordinal",
          "id": "repository.read-original-link-table-test-member"
        },
        {
          "name": "scalar",
          "id": "repository.read-original-link-table-test-scalar-position"
        }
      ]
    }
  },
  {
    "inventory": "complete String projection source oracle",
    "files": {
      "experiments/original-link-symbol-projection/native-observations.mjs": [
        {
          "name": "verifyOriginalLinkSymbolNativeModel",
          "id": "repository.verify-original-link-symbol-native-model"
        },
        {
          "name": "originalLinkSymbolNativeMode",
          "id": "repository.read-original-link-symbol-native-mode"
        },
        {
          "name": "nativeInterface",
          "id": "repository.read-original-link-symbol-native-interface"
        },
        {
          "name": "cases",
          "id": "repository.build-original-link-symbol-native-cases"
        },
        {
          "name": "consumerSource",
          "id": "repository.render-original-link-symbol-native-consumer"
        },
        {
          "name": "planOriginalLinkSymbolNative",
          "id": "repository.plan-original-link-symbol-native-observations"
        },
        {
          "name": "parseObservations",
          "id": "repository.read-original-link-symbol-native-observation-stream"
        },
        {
          "name": "runOriginalLinkSymbolNative",
          "id": "repository.run-original-link-symbol-native-observations"
        }
      ]
    }
  },
  {
    "inventory": "complete String projection source oracle roles",
    "files": {
      "experiments/original-link-symbol-projection/native-observations.mjs": [
        {
          "name": "ORIGINAL_LINK_SYMBOL_MODEL_COMMIT",
          "id": "repository.original-link-symbol-native-model-revision"
        },
        {
          "name": "maximumObservedInputCells",
          "id": "repository.original-link-symbol-native-input-bound"
        },
        {
          "name": "maximumObservedOutputCells",
          "id": "repository.original-link-symbol-native-output-bound"
        },
        {
          "name": "inputVectors",
          "id": "repository.original-link-symbol-native-input-receipts"
        },
        {
          "name": "resultBits",
          "id": "repository.original-link-symbol-native-result-bits"
        }
      ]
    }
  },
  {
    "inventory": "String projection exact oracle provider source digest",
    "files": {
      "experiments/original-link-symbol-projection/native-observations.mjs": [
        {
          "name": "sha256",
          "id": "repository.hash-rust-text-native-oracle-content"
        }
      ],
      "experiments/original-link-symbol-projection/native-observations.test.mjs": [
        {
          "name": "digest",
          "id": "repository.hash-rust-text-native-oracle-content"
        }
      ]
    }
  },
  {
    "inventory": "String projection exact oracle provider source bytes",
    "files": {
      "experiments/original-link-symbol-projection/native-observations.mjs": [
        {
          "name": "textBytes",
          "id": "repository.read-rust-text-native-oracle-bytes"
        }
      ]
    }
  },
  {
    "inventory": "String projection exact oracle provider immutable source plan",
    "files": {
      "experiments/original-link-symbol-projection/native-observations.mjs": [
        {
          "name": "freeze",
          "id": "repository.freeze-rust-text-native-observation-plan"
        }
      ]
    }
  },
  {
    "inventory": "String projection exact oracle provider input bit words",
    "files": {
      "experiments/original-link-symbol-projection/native-observations.mjs": [
        {
          "name": "numberBits",
          "id": "repository.encode-text-certificate-native-number-bits"
        }
      ]
    }
  },
  {
    "inventory": "String projection exact oracle provider output bit words",
    "files": {
      "experiments/original-link-symbol-projection/native-observations.mjs": [
        {
          "name": "bitsNumber",
          "id": "repository.decode-text-certificate-native-number-bits"
        }
      ]
    }
  },
  {
    "inventory": "String projection exact oracle provider shared source runner",
    "files": {
      "experiments/original-link-symbol-projection/native-observations.mjs": [
        {
          "name": "createSourceNativeObservationRunner",
          "id": "repository.create-source-native-observation-runner"
        }
      ]
    }
  },
  {
    "inventory": "String projection exact oracle provider fake complete source processes",
    "files": {
      "experiments/original-link-symbol-projection/native-observations.test.mjs": [
        {
          "name": "fakeProcess",
          "id": "repository.create-text-certificate-native-test-process"
        }
      ]
    }
  },
  {
    "inventory": "String projection exact oracle provider exact aggregated failures",
    "files": {
      "experiments/original-link-symbol-projection/native-observations.test.mjs": [
        {
          "name": "fails",
          "id": "repository.require-text-certificate-native-test-failure"
        }
      ]
    }
  },
  {
    "inventory": "String projection injected oracle contracts",
    "files": {
      "experiments/original-link-symbol-projection/native-observations.test.mjs": [
        {
          "name": "injected",
          "id": "repository.run-original-link-symbol-injected-contract"
        }
      ]
    }
  },
  {
    "inventory": "String projection native output controls",
    "files": {
      "experiments/original-link-symbol-projection/native-observations.test.mjs": [
        {
          "name": "outputParts",
          "id": "repository.read-original-link-symbol-test-output"
        }
      ]
    }
  },
  {
    "inventory": "original link projection workflow conservation",
    "files": {
      "js/tests/ci-workflow-structure.test.js": [
        {
          "name": "checkOriginalLinkProjectionPaths",
          "id": "repository.check-original-link-projection-workflow-paths"
        },
        {
          "name": "checkOriginalLinkSymbolNativeSteps",
          "id": "repository.check-original-link-symbol-native-steps"
        },
        {
          "name": "ORIGINAL_LINK_PROJECTION_PACKAGE_STEPS",
          "id": "repository.original-link-projection-package-steps"
        },
        {
          "name": "ORIGINAL_LINK_SYMBOL_ORACLE_PACKAGE_STEPS",
          "id": "repository.original-link-symbol-oracle-package-steps"
        },
        {
          "name": "ORIGINAL_LINK_SYMBOL_NATIVE_STEPS",
          "id": "repository.original-link-symbol-native-steps"
        }
      ]
    }
  }
];
const normalizeOriginalLinkValueTableName=name=>name.replace(/([a-z0-9])([A-Z])/gu,'$1-$2').replaceAll('_','-').replaceAll(' ','-').toLowerCase().replace(/^assert-/u,'check-').replace(/^fld$/u,'field-path').replace(/^refs$/u,'references').replace(/^f$/u,'field-code').replace(/\bsha256\b/gu,'secure-hash-algorithm-two-hundred-fifty-six').replace(/\bid\b/gu,'identity').replace(/\bml\b/gu,'meta-language').replace(/\b(loop|join)([0-9]+)\b/gu,(whole,kind,digits)=>{const words={1:'one',2:'two',3:'three',4:'four',5:'five',6:'six',7:'seven',8:'eight',9:'nine',10:'ten',11:'eleven'};return words[digits]?kind+'-'+words[digits]:whole;});
const ORIGINAL_LINK_VALUE_TABLE_SOURCE_TEXTS=new Map();
const ORIGINAL_LINK_VALUE_TABLE_NAME_INVENTORIES=ORIGINAL_LINK_VALUE_TABLE_NAME_BINDINGS.map(({inventory,files})=>({
 inventory,files:Object.keys(files),
 extract:(raw,file)=>{let text=raw;if(file.endsWith('.lino')){const saved=ORIGINAL_LINK_VALUE_TABLE_SOURCE_TEXTS.get(file);if(!saved||saved.raw!==raw)ORIGINAL_LINK_VALUE_TABLE_SOURCE_TEXTS.set(file,{raw,text:JSON.stringify(readStructuredDataLino(raw))});text=ORIGINAL_LINK_VALUE_TABLE_SOURCE_TEXTS.get(file).text;}return files[file].filter(({name})=>new RegExp('\\b'+name+'\\b','u').test(text)).map(({name})=>normalizeOriginalLinkValueTableName(name));},
 recordOf:name=>Object.values(files).flat().find(entry=>normalizeOriginalLinkValueTableName(entry.name)===name).id,
}));
// Exact physical package scheduling helpers preserve all logical gates.
const GENERATED_PACKAGE_SCHEDULING_NAME_BINDINGS=[
  {
    "name": "GENERATED_PACKAGE_SLICES",
    "id": "repository.generated-package-physical-slices"
  },
  {
    "name": "GENERATED_PACKAGE_SLICE_STEPS",
    "id": "repository.generated-package-slice-step-owners"
  },
  {
    "name": "GENERATED_PACKAGE_SHARED_SETUP",
    "id": "repository.generated-package-shared-setup-step"
  },
  {
    "name": "packageCheckCells",
    "id": "repository.read-package-check-physical-cells"
  },
  {
    "name": "normalizeGeneratedPackageSlice",
    "id": "repository.read-generated-package-original-step-text"
  },
  {
    "name": "checkGeneratedPackageSlices",
    "id": "repository.check-generated-package-physical-slices"
  }
];
const normalizeGeneratedPackageSchedulingName=name=>name.replace(/([a-z0-9])([A-Z])/gu,'$1-$2').replaceAll('_','-').toLowerCase();
const GENERATED_PACKAGE_SCHEDULING_NAME_INVENTORIES=[{
 inventory:'physical generated package scheduling',files:["js/tests/ci-workflow-structure.test.js"],
 extract:raw=>GENERATED_PACKAGE_SCHEDULING_NAME_BINDINGS.filter(({name})=>new RegExp('\\b'+name+'\\b','u').test(raw)).map(({name})=>normalizeGeneratedPackageSchedulingName(name)),
 recordOf:name=>GENERATED_PACKAGE_SCHEDULING_NAME_BINDINGS.find(entry=>normalizeGeneratedPackageSchedulingName(entry.name)===name).id,
}];
// Exact complete original tables and isolated record metadata buffer meanings.
const COMPLETE_TABLE_BINARY_CANDIDATE_NAME_BINDINGS=[
  {
    "inventory": "complete original table decisions",
    "files": {
      "experiments/original-link-complete-table-projection/rules.mjs": [
        {
          "name": "readOriginalLinkCompleteTokenRole",
          "id": "repository.read-complete-original-link-token-role"
        },
        {
          "name": "readOriginalLinkCompleteTokenTarget",
          "id": "repository.read-complete-original-link-token-target"
        },
        {
          "name": "readOriginalLinkCompleteTokenValue",
          "id": "repository.read-complete-original-link-token-value"
        },
        {
          "name": "readOriginalLinkCompleteValue",
          "id": "repository.read-complete-original-link-expression-value"
        },
        {
          "name": "readOriginalLinkCompleteSymbol",
          "id": "repository.read-complete-original-link-value-symbol"
        },
        {
          "name": "readOriginalLinkCompleteScalarSymbol",
          "id": "repository.read-complete-original-link-scalar-symbol"
        },
        {
          "name": "readOriginalLinkCompleteStringPrefix",
          "id": "repository.read-complete-original-link-string-prefix"
        },
        {
          "name": "readOriginalLinkCompleteParameterPrefix",
          "id": "repository.read-complete-original-link-parameter-prefix"
        },
        {
          "name": "readOriginalLinkCompletePositionQueries",
          "id": "repository.read-complete-original-link-position-queries"
        },
        {
          "name": "readOriginalLinkCompleteStringQueries",
          "id": "repository.read-complete-original-link-string-queries"
        },
        {
          "name": "projectOriginalLinkCompleteCertificateTables",
          "id": "repository.project-complete-original-link-certificate-tables"
        }
      ],
      "experiments/original-link-complete-table-projection/data/native-definitions.lino": [
        {
          "name": "read_original_link_complete_token_role",
          "id": "repository.read-complete-original-link-token-role"
        },
        {
          "name": "read_original_link_complete_token_target",
          "id": "repository.read-complete-original-link-token-target"
        },
        {
          "name": "read_original_link_complete_token_value",
          "id": "repository.read-complete-original-link-token-value"
        },
        {
          "name": "read_original_link_complete_value",
          "id": "repository.read-complete-original-link-expression-value"
        },
        {
          "name": "read_original_link_complete_symbol",
          "id": "repository.read-complete-original-link-value-symbol"
        },
        {
          "name": "read_original_link_complete_scalar_symbol",
          "id": "repository.read-complete-original-link-scalar-symbol"
        },
        {
          "name": "read_original_link_complete_string_prefix",
          "id": "repository.read-complete-original-link-string-prefix"
        },
        {
          "name": "read_original_link_complete_parameter_prefix",
          "id": "repository.read-complete-original-link-parameter-prefix"
        },
        {
          "name": "read_original_link_complete_position_queries",
          "id": "repository.read-complete-original-link-position-queries"
        },
        {
          "name": "read_original_link_complete_string_queries",
          "id": "repository.read-complete-original-link-string-queries"
        },
        {
          "name": "project_original_link_complete_certificate_tables",
          "id": "repository.project-complete-original-link-certificate-tables"
        }
      ]
    }
  },
  {
    "inventory": "complete original table source operations",
    "files": {
      "experiments/original-link-complete-table-projection/policy.mjs": [
        {
          "name": "originalLinkCompleteTablePolicy",
          "id": "repository.complete-original-link-table-policy"
        },
        {
          "name": "originalLinkCompleteTableRulesSource",
          "id": "repository.render-complete-original-link-table-rules"
        }
      ],
      "experiments/original-link-complete-table-projection/transport.mjs": [
        {
          "name": "ORIGINAL_LINK_COMPLETE_TABLE_POLICY",
          "id": "repository.complete-original-link-table-policy"
        },
        {
          "name": "readOriginalLinkCompleteTableTransport",
          "id": "repository.read-complete-original-link-table-transport"
        },
        {
          "name": "projectOriginalLinkCompleteTables",
          "id": "repository.project-complete-original-link-tables"
        }
      ],
      "experiments/original-link-complete-table-projection/generate.mjs": [
        {
          "name": "generateOriginalLinkCompleteTableProjection",
          "id": "repository.generate-complete-original-link-table-data"
        }
      ]
    }
  },
  {
    "inventory": "complete table reversible field labels",
    "files": {
      "experiments/original-link-complete-table-projection/transport.mjs": [
        {
          "name": "fieldName",
          "id": "repository.read-owned-text-certificate-field-name"
        }
      ]
    }
  },
  {
    "inventory": "complete table original primitive String transport",
    "files": {
      "experiments/original-link-complete-table-projection/transport.mjs": [
        {
          "name": "primitiveStrings",
          "id": "repository.collect-original-string-test-units"
        }
      ]
    }
  },
  {
    "inventory": "complete original table recipe authoring",
    "files": {
      "experiments/original-link-complete-table-projection/policy.mjs": [
        {
          "name": "cell",
          "id": "repository.create-complete-original-link-column-instruction"
        },
        {
          "name": "f",
          "id": "repository.read-complete-original-link-field-code"
        },
        {
          "name": "fld",
          "id": "repository.render-complete-original-link-field-path"
        },
        {
          "name": "refs",
          "id": "repository.render-complete-original-link-reference-read"
        },
        {
          "name": "identity",
          "id": "repository.render-complete-original-link-identity-read"
        },
        {
          "name": "symbol",
          "id": "repository.render-complete-original-link-symbol-read"
        },
        {
          "name": "number",
          "id": "repository.render-complete-original-link-number-read"
        },
        {
          "name": "rendered",
          "id": "repository.render-complete-original-link-expression-read"
        },
        {
          "name": "tokenRole",
          "id": "repository.render-complete-original-link-token-role"
        },
        {
          "name": "target",
          "id": "repository.render-complete-original-link-token-target"
        },
        {
          "name": "tokenValue",
          "id": "repository.render-complete-original-link-token-value"
        },
        {
          "name": "queryColumns",
          "id": "repository.read-complete-original-link-query-columns"
        },
        {
          "name": "queryCell",
          "id": "repository.render-complete-original-link-query-cell"
        },
        {
          "name": "queryArray",
          "id": "repository.render-complete-original-link-query-array"
        },
        {
          "name": "flush",
          "id": "repository.append-complete-original-link-query-segment"
        },
        {
          "name": "cells",
          "id": "repository.render-complete-original-link-table-cell"
        }
      ]
    }
  },
  {
    "inventory": "complete original table generated result wrappers",
    "files": {
      "experiments/original-link-complete-table-projection/data/native-definitions.lino": [
        {
          "name": "ml_readOriginalLinkCompleteScalarSymbol_loop20_result",
          "id": "repository.complete-original-link-fixed-scalar-scan-control-result"
        },
        {
          "name": "MlReadOriginalLinkCompleteScalarSymbolLoop20Result",
          "id": "repository.complete-original-link-fixed-scalar-scan-control-result"
        }
      ]
    }
  },
  {
    "inventory": "complete original table generated constructors",
    "files": {
      "experiments/original-link-complete-table-projection/data/native-definitions.lino": [
        {
          "name": "MlReadOriginalLinkCompleteScalarSymbolLoop20Done",
          "id": "repository.complete-original-link-fixed-scalar-scan-completion-constructor"
        },
        {
          "name": "MlReadOriginalLinkCompleteScalarSymbolLoop20Return",
          "id": "repository.complete-original-link-fixed-scalar-scan-return-constructor"
        }
      ]
    }
  },
  {
    "inventory": "complete original table generated continuations",
    "files": {
      "experiments/original-link-complete-table-projection/data/native-definitions.lino": [
        {
          "name": "ml_readOriginalLinkCompleteScalarSymbol_loop20",
          "id": "repository.continue-complete-original-link-fixed-scalar-scan"
        },
        {
          "name": "ml_read_original_link_complete_scalar_symbol_loop20",
          "id": "repository.continue-complete-original-link-fixed-scalar-scan"
        },
        {
          "name": "ml_readOriginalLinkCompleteStringPrefix_loop21",
          "id": "repository.continue-complete-original-link-string-prefix-scan"
        },
        {
          "name": "ml_read_original_link_complete_string_prefix_loop21",
          "id": "repository.continue-complete-original-link-string-prefix-scan"
        },
        {
          "name": "ml_readOriginalLinkCompleteParameterPrefix_loop22",
          "id": "repository.continue-complete-original-link-parameter-prefix-scan"
        },
        {
          "name": "ml_read_original_link_complete_parameter_prefix_loop22",
          "id": "repository.continue-complete-original-link-parameter-prefix-scan"
        }
      ]
    }
  },
  {
    "inventory": "complete table checked inherited declarations",
    "files": {
      "experiments/original-link-complete-table-projection/data/native-definitions.lino": [
        {
          "name": "readOriginalLinkRoleField",
          "id": "repository.read-original-link-role-field"
        },
        {
          "name": "read_original_link_role_field",
          "id": "repository.read-original-link-role-field"
        },
        {
          "name": "readOriginalLinkRoleReferences",
          "id": "repository.read-original-link-role-references"
        },
        {
          "name": "read_original_link_role_references",
          "id": "repository.read-original-link-role-references"
        },
        {
          "name": "readOriginalLinkRoleOperation",
          "id": "repository.accept-original-link-role-operation"
        },
        {
          "name": "read_original_link_role_operation",
          "id": "repository.accept-original-link-role-operation"
        },
        {
          "name": "firstOriginalLinkRoleOccurrence",
          "id": "repository.accept-first-original-link-role-occurrence"
        },
        {
          "name": "first_original_link_role_occurrence",
          "id": "repository.accept-first-original-link-role-occurrence"
        },
        {
          "name": "readOriginalLinkRoleKind",
          "id": "repository.read-original-link-concrete-role-kind"
        },
        {
          "name": "read_original_link_role_kind",
          "id": "repository.read-original-link-concrete-role-kind"
        },
        {
          "name": "registerOriginalLinkRoles",
          "id": "repository.register-original-link-certificate-roles"
        },
        {
          "name": "register_original_link_roles",
          "id": "repository.register-original-link-certificate-roles"
        },
        {
          "name": "projectOriginalLinkRoleRegistry",
          "id": "repository.project-original-link-role-registry"
        },
        {
          "name": "project_original_link_role_registry",
          "id": "repository.project-original-link-role-registry"
        },
        {
          "name": "readOriginalLinkTableIdentity",
          "id": "repository.read-original-link-table-identity"
        },
        {
          "name": "read_original_link_table_identity",
          "id": "repository.read-original-link-table-identity"
        },
        {
          "name": "readOriginalLinkTableSymbol",
          "id": "repository.read-original-link-table-symbol"
        },
        {
          "name": "read_original_link_table_symbol",
          "id": "repository.read-original-link-table-symbol"
        },
        {
          "name": "readOriginalLinkTableNumber",
          "id": "repository.read-original-link-table-number"
        },
        {
          "name": "read_original_link_table_number",
          "id": "repository.read-original-link-table-number"
        },
        {
          "name": "readOriginalLinkTableOperationCode",
          "id": "repository.read-original-link-table-operation-code"
        },
        {
          "name": "read_original_link_table_operation_code",
          "id": "repository.read-original-link-table-operation-code"
        },
        {
          "name": "readOriginalLinkTableRole",
          "id": "repository.read-original-link-table-role"
        },
        {
          "name": "read_original_link_table_role",
          "id": "repository.read-original-link-table-role"
        },
        {
          "name": "readOriginalLinkTablePrefix",
          "id": "repository.read-original-link-table-prefix"
        },
        {
          "name": "read_original_link_table_prefix",
          "id": "repository.read-original-link-table-prefix"
        },
        {
          "name": "readOriginalLinkTableLayers",
          "id": "repository.read-original-link-table-layers"
        },
        {
          "name": "read_original_link_table_layers",
          "id": "repository.read-original-link-table-layers"
        },
        {
          "name": "projectOriginalLinkCertificateTables",
          "id": "repository.project-original-link-certificate-tables"
        },
        {
          "name": "project_original_link_certificate_tables",
          "id": "repository.project-original-link-certificate-tables"
        },
        {
          "name": "ml_readOriginalLinkRoleField_loop1_result",
          "id": "repository.original-link-role-unique-field-scan-control-result"
        },
        {
          "name": "MlReadOriginalLinkRoleFieldLoop1Result",
          "id": "repository.original-link-role-unique-field-scan-control-result"
        },
        {
          "name": "ml_readOriginalLinkRoleField_loop1",
          "id": "repository.continue-original-link-role-unique-field-scan"
        },
        {
          "name": "ml_read_original_link_role_field_loop1",
          "id": "repository.continue-original-link-role-unique-field-scan"
        },
        {
          "name": "ml_firstOriginalLinkRoleOccurrence_loop2_result",
          "id": "repository.original-link-role-first-occurrence-scan-control-result"
        },
        {
          "name": "MlFirstOriginalLinkRoleOccurrenceLoop2Result",
          "id": "repository.original-link-role-first-occurrence-scan-control-result"
        },
        {
          "name": "ml_firstOriginalLinkRoleOccurrence_loop2",
          "id": "repository.continue-original-link-role-first-occurrence-scan"
        },
        {
          "name": "ml_first_original_link_role_occurrence_loop2",
          "id": "repository.continue-original-link-role-first-occurrence-scan"
        },
        {
          "name": "ml_readOriginalLinkRoleKind_loop3",
          "id": "repository.continue-original-link-role-concrete-kind-scan"
        },
        {
          "name": "ml_read_original_link_role_kind_loop3",
          "id": "repository.continue-original-link-role-concrete-kind-scan"
        },
        {
          "name": "ml_registerOriginalLinkRoles_loop5_result",
          "id": "repository.original-link-role-unresolved-kind-scan-control-result"
        },
        {
          "name": "MlRegisterOriginalLinkRolesLoop5Result",
          "id": "repository.original-link-role-unresolved-kind-scan-control-result"
        },
        {
          "name": "ml_registerOriginalLinkRoles_loop5",
          "id": "repository.continue-original-link-role-unresolved-kind-scan"
        },
        {
          "name": "ml_register_original_link_roles_loop5",
          "id": "repository.continue-original-link-role-unresolved-kind-scan"
        },
        {
          "name": "ml_registerOriginalLinkRoles_loop4_result",
          "id": "repository.original-link-role-occurrence-validation-control-result"
        },
        {
          "name": "MlRegisterOriginalLinkRolesLoop4Result",
          "id": "repository.original-link-role-occurrence-validation-control-result"
        },
        {
          "name": "ml_registerOriginalLinkRoles_loop6_result",
          "id": "repository.original-link-role-contradiction-scan-control-result"
        },
        {
          "name": "MlRegisterOriginalLinkRolesLoop6Result",
          "id": "repository.original-link-role-contradiction-scan-control-result"
        },
        {
          "name": "ml_registerOriginalLinkRoles_loop6",
          "id": "repository.continue-original-link-role-contradiction-scan"
        },
        {
          "name": "ml_register_original_link_roles_loop6",
          "id": "repository.continue-original-link-role-contradiction-scan"
        },
        {
          "name": "ml_registerOriginalLinkRoles_loop4",
          "id": "repository.continue-original-link-role-occurrence-validation"
        },
        {
          "name": "ml_register_original_link_roles_loop4",
          "id": "repository.continue-original-link-role-occurrence-validation"
        },
        {
          "name": "ml_projectOriginalLinkRoleRegistry_loop7_result",
          "id": "repository.original-link-role-ordered-operation-traversal-control-result"
        },
        {
          "name": "MlProjectOriginalLinkRoleRegistryLoop7Result",
          "id": "repository.original-link-role-ordered-operation-traversal-control-result"
        },
        {
          "name": "ml_projectOriginalLinkRoleRegistry_loop7",
          "id": "repository.continue-original-link-role-ordered-operation-traversal"
        },
        {
          "name": "ml_project_original_link_role_registry_loop7",
          "id": "repository.continue-original-link-role-ordered-operation-traversal"
        },
        {
          "name": "ml_readOriginalLinkTablePrefix_loop8",
          "id": "repository.continue-original-link-table-prefix-scan"
        },
        {
          "name": "ml_read_original_link_table_prefix_loop8",
          "id": "repository.continue-original-link-table-prefix-scan"
        },
        {
          "name": "acceptOriginalLinkSymbolInteger",
          "id": "repository.accept-original-link-symbol-integer"
        },
        {
          "name": "accept_original_link_symbol_integer",
          "id": "repository.accept-original-link-symbol-integer"
        },
        {
          "name": "checkOriginalLinkSymbolTransport",
          "id": "repository.check-original-link-symbol-transport"
        },
        {
          "name": "check_original_link_symbol_transport",
          "id": "repository.check-original-link-symbol-transport"
        },
        {
          "name": "equalOriginalLinkSymbolUnits",
          "id": "repository.compare-original-link-symbol-units"
        },
        {
          "name": "equal_original_link_symbol_units",
          "id": "repository.compare-original-link-symbol-units"
        },
        {
          "name": "projectOriginalLinkSymbolValues",
          "id": "repository.project-original-link-symbol-values"
        },
        {
          "name": "project_original_link_symbol_values",
          "id": "repository.project-original-link-symbol-values"
        },
        {
          "name": "ml_checkOriginalLinkSymbolTransport_loop10_result",
          "id": "repository.original-link-symbol-code-unit-scan-control-result"
        },
        {
          "name": "MlCheckOriginalLinkSymbolTransportLoop10Result",
          "id": "repository.original-link-symbol-code-unit-scan-control-result"
        },
        {
          "name": "ml_checkOriginalLinkSymbolTransport_loop10",
          "id": "repository.continue-original-link-symbol-code-unit-scan"
        },
        {
          "name": "ml_check_original_link_symbol_transport_loop10",
          "id": "repository.continue-original-link-symbol-code-unit-scan"
        },
        {
          "name": "ml_checkOriginalLinkSymbolTransport_loop9_result",
          "id": "repository.original-link-symbol-offset-scan-control-result"
        },
        {
          "name": "MlCheckOriginalLinkSymbolTransportLoop9Result",
          "id": "repository.original-link-symbol-offset-scan-control-result"
        },
        {
          "name": "ml_checkOriginalLinkSymbolTransport_loop9",
          "id": "repository.continue-original-link-symbol-offset-scan"
        },
        {
          "name": "ml_check_original_link_symbol_transport_loop9",
          "id": "repository.continue-original-link-symbol-offset-scan"
        },
        {
          "name": "ml_equalOriginalLinkSymbolUnits_loop11_result",
          "id": "repository.original-link-symbol-span-comparison-control-result"
        },
        {
          "name": "MlEqualOriginalLinkSymbolUnitsLoop11Result",
          "id": "repository.original-link-symbol-span-comparison-control-result"
        },
        {
          "name": "ml_equalOriginalLinkSymbolUnits_loop11",
          "id": "repository.continue-original-link-symbol-span-comparison"
        },
        {
          "name": "ml_equal_original_link_symbol_units_loop11",
          "id": "repository.continue-original-link-symbol-span-comparison"
        },
        {
          "name": "ml_projectOriginalLinkSymbolValues_loop14_result",
          "id": "repository.original-link-symbol-source-construction-control-result"
        },
        {
          "name": "MlProjectOriginalLinkSymbolValuesLoop14Result",
          "id": "repository.original-link-symbol-source-construction-control-result"
        },
        {
          "name": "ml_projectOriginalLinkSymbolValues_loop15",
          "id": "repository.continue-original-link-symbol-existing-name-search"
        },
        {
          "name": "ml_project_original_link_symbol_values_loop15",
          "id": "repository.continue-original-link-symbol-existing-name-search"
        },
        {
          "name": "ml_projectOriginalLinkSymbolValues_join16",
          "id": "repository.original-link-symbol-source-branch-state-control-result"
        },
        {
          "name": "MlProjectOriginalLinkSymbolValuesJoin16",
          "id": "repository.original-link-symbol-source-branch-state-control-result"
        },
        {
          "name": "ml_projectOriginalLinkSymbolValues_loop17",
          "id": "repository.continue-original-link-symbol-dynamic-unit-copy"
        },
        {
          "name": "ml_project_original_link_symbol_values_loop17",
          "id": "repository.continue-original-link-symbol-dynamic-unit-copy"
        },
        {
          "name": "ml_projectOriginalLinkSymbolValues_loop14",
          "id": "repository.continue-original-link-symbol-source-construction"
        },
        {
          "name": "ml_project_original_link_symbol_values_loop14",
          "id": "repository.continue-original-link-symbol-source-construction"
        },
        {
          "name": "ml_projectOriginalLinkSymbolValues_loop13_result",
          "id": "repository.original-link-symbol-fixed-construction-control-result"
        },
        {
          "name": "MlProjectOriginalLinkSymbolValuesLoop13Result",
          "id": "repository.original-link-symbol-fixed-construction-control-result"
        },
        {
          "name": "ml_projectOriginalLinkSymbolValues_loop18",
          "id": "repository.continue-original-link-symbol-fixed-unit-copy"
        },
        {
          "name": "ml_project_original_link_symbol_values_loop18",
          "id": "repository.continue-original-link-symbol-fixed-unit-copy"
        },
        {
          "name": "ml_projectOriginalLinkSymbolValues_loop13",
          "id": "repository.continue-original-link-symbol-fixed-construction"
        },
        {
          "name": "ml_project_original_link_symbol_values_loop13",
          "id": "repository.continue-original-link-symbol-fixed-construction"
        },
        {
          "name": "ml_projectOriginalLinkSymbolValues_loop12_result",
          "id": "repository.original-link-symbol-fixed-duplicate-scan-control-result"
        },
        {
          "name": "MlProjectOriginalLinkSymbolValuesLoop12Result",
          "id": "repository.original-link-symbol-fixed-duplicate-scan-control-result"
        },
        {
          "name": "ml_projectOriginalLinkSymbolValues_loop19_result",
          "id": "repository.original-link-symbol-prior-fixed-comparison-control-result"
        },
        {
          "name": "MlProjectOriginalLinkSymbolValuesLoop19Result",
          "id": "repository.original-link-symbol-prior-fixed-comparison-control-result"
        },
        {
          "name": "ml_projectOriginalLinkSymbolValues_loop19",
          "id": "repository.continue-original-link-symbol-prior-fixed-comparison"
        },
        {
          "name": "ml_project_original_link_symbol_values_loop19",
          "id": "repository.continue-original-link-symbol-prior-fixed-comparison"
        },
        {
          "name": "ml_projectOriginalLinkSymbolValues_loop12",
          "id": "repository.continue-original-link-symbol-fixed-duplicate-scan"
        },
        {
          "name": "ml_project_original_link_symbol_values_loop12",
          "id": "repository.continue-original-link-symbol-fixed-duplicate-scan"
        }
      ]
    }
  },
  {
    "inventory": "complete table exact inherited source",
    "files": {
      "experiments/original-link-complete-table-projection/rules.mjs": [
        {
          "name": "readOriginalLinkRoleField",
          "id": "repository.read-original-link-role-field"
        },
        {
          "name": "readOriginalLinkRoleReferences",
          "id": "repository.read-original-link-role-references"
        },
        {
          "name": "readOriginalLinkRoleOperation",
          "id": "repository.accept-original-link-role-operation"
        },
        {
          "name": "firstOriginalLinkRoleOccurrence",
          "id": "repository.accept-first-original-link-role-occurrence"
        },
        {
          "name": "readOriginalLinkRoleKind",
          "id": "repository.read-original-link-concrete-role-kind"
        },
        {
          "name": "registerOriginalLinkRoles",
          "id": "repository.register-original-link-certificate-roles"
        },
        {
          "name": "projectOriginalLinkRoleRegistry",
          "id": "repository.project-original-link-role-registry"
        },
        {
          "name": "readOriginalLinkTableIdentity",
          "id": "repository.read-original-link-table-identity"
        },
        {
          "name": "readOriginalLinkTableSymbol",
          "id": "repository.read-original-link-table-symbol"
        },
        {
          "name": "readOriginalLinkTableNumber",
          "id": "repository.read-original-link-table-number"
        },
        {
          "name": "readOriginalLinkTableOperationCode",
          "id": "repository.read-original-link-table-operation-code"
        },
        {
          "name": "readOriginalLinkTableRole",
          "id": "repository.read-original-link-table-role"
        },
        {
          "name": "readOriginalLinkTablePrefix",
          "id": "repository.read-original-link-table-prefix"
        },
        {
          "name": "readOriginalLinkTableLayers",
          "id": "repository.read-original-link-table-layers"
        },
        {
          "name": "projectOriginalLinkCertificateTables",
          "id": "repository.project-original-link-certificate-tables"
        },
        {
          "name": "acceptOriginalLinkSymbolInteger",
          "id": "repository.accept-original-link-symbol-integer"
        },
        {
          "name": "checkOriginalLinkSymbolTransport",
          "id": "repository.check-original-link-symbol-transport"
        },
        {
          "name": "equalOriginalLinkSymbolUnits",
          "id": "repository.compare-original-link-symbol-units"
        },
        {
          "name": "projectOriginalLinkSymbolValues",
          "id": "repository.project-original-link-symbol-values"
        }
      ]
    }
  },
  {
    "inventory": "complete table checked inherited constructors",
    "files": {
      "experiments/original-link-complete-table-projection/data/native-definitions.lino": [
        {
          "name": "MlReadOriginalLinkRoleFieldLoop1Done",
          "id": "repository.original-link-role-unique-field-scan-completion-constructor"
        },
        {
          "name": "MlReadOriginalLinkRoleFieldLoop1Return",
          "id": "repository.original-link-role-unique-field-scan-return-constructor"
        },
        {
          "name": "MlFirstOriginalLinkRoleOccurrenceLoop2Done",
          "id": "repository.original-link-role-first-occurrence-scan-completion-constructor"
        },
        {
          "name": "MlFirstOriginalLinkRoleOccurrenceLoop2Return",
          "id": "repository.original-link-role-first-occurrence-scan-return-constructor"
        },
        {
          "name": "MlRegisterOriginalLinkRolesLoop5Done",
          "id": "repository.original-link-role-unresolved-kind-scan-completion-constructor"
        },
        {
          "name": "MlRegisterOriginalLinkRolesLoop5Return",
          "id": "repository.original-link-role-unresolved-kind-scan-return-constructor"
        },
        {
          "name": "MlRegisterOriginalLinkRolesLoop4Done",
          "id": "repository.original-link-role-occurrence-validation-completion-constructor"
        },
        {
          "name": "MlRegisterOriginalLinkRolesLoop4Return",
          "id": "repository.original-link-role-occurrence-validation-return-constructor"
        },
        {
          "name": "MlRegisterOriginalLinkRolesLoop6Done",
          "id": "repository.original-link-role-contradiction-scan-completion-constructor"
        },
        {
          "name": "MlRegisterOriginalLinkRolesLoop6Return",
          "id": "repository.original-link-role-contradiction-scan-return-constructor"
        },
        {
          "name": "MlProjectOriginalLinkRoleRegistryLoop7Done",
          "id": "repository.original-link-role-ordered-operation-traversal-completion-constructor"
        },
        {
          "name": "MlProjectOriginalLinkRoleRegistryLoop7Return",
          "id": "repository.original-link-role-ordered-operation-traversal-return-constructor"
        },
        {
          "name": "MlCheckOriginalLinkSymbolTransportLoop10Done",
          "id": "repository.original-link-symbol-code-unit-scan-completion-constructor"
        },
        {
          "name": "MlCheckOriginalLinkSymbolTransportLoop10Return",
          "id": "repository.original-link-symbol-code-unit-scan-return-constructor"
        },
        {
          "name": "MlCheckOriginalLinkSymbolTransportLoop9Done",
          "id": "repository.original-link-symbol-offset-scan-completion-constructor"
        },
        {
          "name": "MlCheckOriginalLinkSymbolTransportLoop9Return",
          "id": "repository.original-link-symbol-offset-scan-return-constructor"
        },
        {
          "name": "MlEqualOriginalLinkSymbolUnitsLoop11Done",
          "id": "repository.original-link-symbol-span-comparison-completion-constructor"
        },
        {
          "name": "MlEqualOriginalLinkSymbolUnitsLoop11Return",
          "id": "repository.original-link-symbol-span-comparison-return-constructor"
        },
        {
          "name": "MlProjectOriginalLinkSymbolValuesLoop14Done",
          "id": "repository.original-link-symbol-source-construction-completion-constructor"
        },
        {
          "name": "MlProjectOriginalLinkSymbolValuesJoin16Next",
          "id": "repository.original-link-symbol-source-branch-state-next-state-constructor"
        },
        {
          "name": "MlProjectOriginalLinkSymbolValuesLoop13Done",
          "id": "repository.original-link-symbol-fixed-construction-completion-constructor"
        },
        {
          "name": "MlProjectOriginalLinkSymbolValuesLoop12Done",
          "id": "repository.original-link-symbol-fixed-duplicate-scan-completion-constructor"
        },
        {
          "name": "MlProjectOriginalLinkSymbolValuesLoop12Return",
          "id": "repository.original-link-symbol-fixed-duplicate-scan-return-constructor"
        },
        {
          "name": "MlProjectOriginalLinkSymbolValuesLoop19Done",
          "id": "repository.original-link-symbol-prior-fixed-comparison-completion-constructor"
        },
        {
          "name": "MlProjectOriginalLinkSymbolValuesLoop19Return",
          "id": "repository.original-link-symbol-prior-fixed-comparison-return-constructor"
        }
      ]
    }
  },
  {
    "inventory": "complete certificate existing table formats",
    "files": {
      "experiments/original-link-complete-table-projection/data/policy.lino": [
        {
          "name": "context",
          "id": "repository.text-certificate-context-table"
        },
        {
          "name": "kinds",
          "id": "repository.text-certificate-identity-kind-table"
        },
        {
          "name": "tokens",
          "id": "repository.text-certificate-token-table"
        },
        {
          "name": "declarations",
          "id": "repository.text-certificate-declaration-table"
        },
        {
          "name": "parameters",
          "id": "repository.text-certificate-parameter-table"
        },
        {
          "name": "positions",
          "id": "repository.text-certificate-source-position-table"
        },
        {
          "name": "paths",
          "id": "repository.text-certificate-owner-path-table"
        },
        {
          "name": "layers",
          "id": "repository.text-certificate-reference-layer-table"
        },
        {
          "name": "cores",
          "id": "repository.text-certificate-expression-table"
        },
        {
          "name": "functions",
          "id": "repository.text-certificate-function-fact-table"
        },
        {
          "name": "operations",
          "id": "repository.text-certificate-operation-table"
        },
        {
          "name": "parts",
          "id": "repository.text-certificate-operation-part-table"
        },
        {
          "name": "inputs",
          "id": "repository.text-certificate-input-condition-table"
        },
        {
          "name": "allocations",
          "id": "repository.text-certificate-allocation-condition-table"
        },
        {
          "name": "names",
          "id": "repository.text-certificate-symbol-range-table"
        },
        {
          "name": "units",
          "id": "repository.text-certificate-symbol-unit-table"
        },
        {
          "name": "items",
          "id": "repository.text-certificate-declaration-order-table"
        }
      ]
    }
  },
  {
    "inventory": "complete table exact saved control providers",
    "files": {
      "experiments/original-link-complete-table-projection/projection.test.mjs": [
        {
          "name": "key",
          "id": "repository.read-exact-link-identity-key"
        },
        {
          "name": "label",
          "id": "repository.render-certificate-test-field-label"
        },
        {
          "name": "field",
          "id": "repository.read-unique-original-link-role-test-field"
        },
        {
          "name": "refs",
          "id": "repository.read-original-link-role-test-references"
        },
        {
          "name": "body",
          "id": "repository.read-original-link-role-test-body"
        },
        {
          "name": "proof",
          "id": "repository.read-original-link-role-test-proof"
        },
        {
          "name": "rows",
          "id": "repository.read-original-link-role-test-proof-rows"
        },
        {
          "name": "replace",
          "id": "repository.replace-text-certificate-test-field"
        },
        {
          "name": "attach",
          "id": "repository.attach-owned-text-certificate-test-proof"
        },
        {
          "name": "create",
          "id": "repository.create-original-link-role-test-packet"
        },
        {
          "name": "copy",
          "id": "repository.copy-original-link-table-test-arguments"
        }
      ]
    }
  },
  {
    "inventory": "complete original table independent controls",
    "files": {
      "experiments/original-link-complete-table-projection/projection.test.mjs": [
        {
          "name": "expectedTables",
          "id": "repository.read-complete-original-link-reference-tables"
        },
        {
          "name": "compare",
          "id": "repository.check-complete-original-link-table-output"
        },
        {
          "name": "tableRows",
          "id": "repository.read-complete-original-link-test-rows"
        },
        {
          "name": "ordinal",
          "id": "repository.read-complete-original-link-test-member"
        },
        {
          "name": "scalar",
          "id": "repository.read-complete-original-link-test-scalar"
        },
        {
          "name": "execute",
          "id": "repository.execute-complete-original-link-test-javascript"
        },
        {
          "name": "expectedQueries",
          "id": "repository.read-complete-original-link-reference-queries"
        }
      ]
    }
  },
  {
    "inventory": "binary candidate exact primitive providers",
    "files": {
      "experiments/binary-metadata-buffer/candidate.mjs": [
        {
          "name": "linkNetworkBinaryUnit",
          "id": "links.read-link-network-binary-unit"
        },
        {
          "name": "acceptLinkNetworkBinaryInteger",
          "id": "links.accept-link-network-binary-integer"
        },
        {
          "name": "readLinkNetworkBinaryInteger",
          "id": "links.read-link-network-binary-integer"
        },
        {
          "name": "linkNetworkBinaryIntegerByte",
          "id": "links.read-link-network-binary-integer-byte"
        },
        {
          "name": "acceptLinkNetworkBinaryUnicode",
          "id": "links.accept-link-network-binary-unicode"
        },
        {
          "name": "renderLinkNetworkBinaryHexadecimalDigit",
          "id": "links.render-link-network-binary-hexadecimal-digit"
        },
        {
          "name": "renderLinkNetworkBinaryField",
          "id": "links.render-link-network-binary-field"
        }
      ],
      "experiments/binary-metadata-buffer/prior-rules.mjs": [
        {
          "name": "linkNetworkBinaryUnit",
          "id": "links.read-link-network-binary-unit"
        },
        {
          "name": "acceptLinkNetworkBinaryInteger",
          "id": "links.accept-link-network-binary-integer"
        },
        {
          "name": "readLinkNetworkBinaryInteger",
          "id": "links.read-link-network-binary-integer"
        },
        {
          "name": "linkNetworkBinaryIntegerByte",
          "id": "links.read-link-network-binary-integer-byte"
        },
        {
          "name": "acceptLinkNetworkBinaryUnicode",
          "id": "links.accept-link-network-binary-unicode"
        },
        {
          "name": "renderLinkNetworkBinaryHexadecimalDigit",
          "id": "links.render-link-network-binary-hexadecimal-digit"
        },
        {
          "name": "renderLinkNetworkBinaryField",
          "id": "links.render-link-network-binary-field"
        }
      ],
      "experiments/binary-metadata-buffer/native/part_0.rs": [
        {
          "name": "link_network_binary_unit",
          "id": "links.read-link-network-binary-unit"
        },
        {
          "name": "accept_link_network_binary_integer",
          "id": "links.accept-link-network-binary-integer"
        },
        {
          "name": "read_link_network_binary_integer",
          "id": "links.read-link-network-binary-integer"
        },
        {
          "name": "link_network_binary_integer_byte",
          "id": "links.read-link-network-binary-integer-byte"
        },
        {
          "name": "accept_link_network_binary_unicode",
          "id": "links.accept-link-network-binary-unicode"
        },
        {
          "name": "render_link_network_binary_hexadecimal_digit",
          "id": "links.render-link-network-binary-hexadecimal-digit"
        },
        {
          "name": "render_link_network_binary_field",
          "id": "links.render-link-network-binary-field"
        }
      ]
    }
  },
  {
    "inventory": "binary candidate exact historical source",
    "files": {
      "experiments/binary-metadata-buffer/prior-rules.mjs": [
        {
          "name": "transcodeLinkNetworkBinary",
          "id": "links.convert-link-network-binary-encoding"
        }
      ]
    }
  },
  {
    "inventory": "binary candidate record accumulator",
    "files": {
      "experiments/binary-metadata-buffer/candidate.mjs": [
        {
          "name": "transcodeLinkNetworkBinary",
          "id": "repository.convert-record-buffered-link-network-binary"
        }
      ],
      "experiments/binary-metadata-buffer/native/part_1.rs": [
        {
          "name": "transcode_link_network_binary",
          "id": "repository.convert-record-buffered-link-network-binary"
        }
      ]
    }
  },
  {
    "inventory": "binary candidate complete support modules",
    "files": {
      "experiments/binary-metadata-buffer/native/part_0.rs": [
        {
          "name": "ml_number",
          "id": "repository.binary-candidate-number-provider"
        },
        {
          "name": "ml_math",
          "id": "repository.binary-candidate-mathematical-provider"
        },
        {
          "name": "ml_array",
          "id": "repository.binary-candidate-array-provider"
        },
        {
          "name": "ml_string",
          "id": "repository.binary-candidate-text-provider"
        }
      ]
    }
  },
  {
    "inventory": "binary candidate generated result wrappers",
    "files": {
      "experiments/binary-metadata-buffer/native/part_1.rs": [
        {
          "name": "MlReadLinkNetworkBinaryIntegerLoop1Result",
          "id": "repository.binary-framed-integer-scan-control-result"
        },
        {
          "name": "MlAcceptLinkNetworkBinaryUnicodeLoop3Result",
          "id": "repository.binary-scalar-unicode-scan-control-result"
        },
        {
          "name": "MlTranscodeLinkNetworkBinaryJoin8",
          "id": "repository.record-buffered-binary-record-stage-control-result"
        },
        {
          "name": "MlTranscodeLinkNetworkBinaryLoop9Result",
          "id": "repository.record-buffered-binary-state-section-scan-control-result"
        },
        {
          "name": "MlTranscodeLinkNetworkBinaryLoop11Result",
          "id": "repository.record-buffered-binary-state-entry-scan-control-result"
        },
        {
          "name": "MlTranscodeLinkNetworkBinaryJoin13",
          "id": "repository.record-buffered-binary-cache-stage-control-result"
        },
        {
          "name": "MlTranscodeLinkNetworkBinaryLoop12Result",
          "id": "repository.record-buffered-binary-state-string-scan-control-result"
        },
        {
          "name": "MlTranscodeLinkNetworkBinaryLoop16Result",
          "id": "repository.record-buffered-binary-state-text-copy-control-result"
        }
      ],
      "experiments/binary-metadata-buffer/native/part_2.rs": [
        {
          "name": "MlTranscodeLinkNetworkBinaryLoop7Result",
          "id": "repository.record-buffered-binary-record-scan-control-result"
        },
        {
          "name": "MlTranscodeLinkNetworkBinaryJoin23",
          "id": "repository.record-buffered-binary-optional-span-stage-control-result"
        },
        {
          "name": "MlTranscodeLinkNetworkBinaryLoop25Result",
          "id": "repository.record-buffered-binary-span-field-scan-control-result"
        },
        {
          "name": "MlTranscodeLinkNetworkBinaryLoop22Result",
          "id": "repository.record-buffered-binary-metadata-text-scan-control-result"
        },
        {
          "name": "MlTranscodeLinkNetworkBinaryLoop28Result",
          "id": "repository.record-buffered-binary-metadata-text-copy-control-result"
        },
        {
          "name": "MlTranscodeLinkNetworkBinaryLoop21Result",
          "id": "repository.record-buffered-binary-metadata-header-scan-control-result"
        },
        {
          "name": "MlTranscodeLinkNetworkBinaryJoin20",
          "id": "repository.record-buffered-binary-metadata-boundary-stage-control-result"
        },
        {
          "name": "MlTranscodeLinkNetworkBinaryLoop19Result",
          "id": "repository.record-buffered-binary-reference-scan-control-result"
        }
      ],
      "experiments/binary-metadata-buffer/native/part_3.rs": [
        {
          "name": "MlTranscodeLinkNetworkBinaryJoin5",
          "id": "repository.record-buffered-binary-next-identity-stage-control-result"
        },
        {
          "name": "MlTranscodeLinkNetworkBinaryJoin4",
          "id": "repository.record-buffered-binary-version-stage-control-result"
        },
        {
          "name": "MlTranscodeLinkNetworkBinaryLoop31Result",
          "id": "repository.record-buffered-binary-magic-header-scan-control-result"
        }
      ]
    }
  },
  {
    "inventory": "binary candidate generated constructors",
    "files": {
      "experiments/binary-metadata-buffer/native/part_1.rs": [
        {
          "name": "MlReadLinkNetworkBinaryIntegerLoop1Done",
          "id": "repository.binary-framed-integer-scan-completion-constructor"
        },
        {
          "name": "MlReadLinkNetworkBinaryIntegerLoop1Return",
          "id": "repository.binary-framed-integer-scan-return-constructor"
        },
        {
          "name": "MlAcceptLinkNetworkBinaryUnicodeLoop3Done",
          "id": "repository.binary-scalar-unicode-scan-completion-constructor"
        },
        {
          "name": "MlAcceptLinkNetworkBinaryUnicodeLoop3Return",
          "id": "repository.binary-scalar-unicode-scan-return-constructor"
        },
        {
          "name": "MlTranscodeLinkNetworkBinaryJoin8Next",
          "id": "repository.record-buffered-binary-record-stage-next-state-constructor"
        },
        {
          "name": "MlTranscodeLinkNetworkBinaryJoin8Return",
          "id": "repository.record-buffered-binary-record-stage-return-constructor"
        },
        {
          "name": "MlTranscodeLinkNetworkBinaryLoop9Done",
          "id": "repository.record-buffered-binary-state-section-scan-completion-constructor"
        },
        {
          "name": "MlTranscodeLinkNetworkBinaryLoop9Return",
          "id": "repository.record-buffered-binary-state-section-scan-return-constructor"
        },
        {
          "name": "MlTranscodeLinkNetworkBinaryLoop11Done",
          "id": "repository.record-buffered-binary-state-entry-scan-completion-constructor"
        },
        {
          "name": "MlTranscodeLinkNetworkBinaryLoop11Return",
          "id": "repository.record-buffered-binary-state-entry-scan-return-constructor"
        },
        {
          "name": "MlTranscodeLinkNetworkBinaryJoin13Next",
          "id": "repository.record-buffered-binary-cache-stage-next-state-constructor"
        },
        {
          "name": "MlTranscodeLinkNetworkBinaryJoin13Return",
          "id": "repository.record-buffered-binary-cache-stage-return-constructor"
        },
        {
          "name": "MlTranscodeLinkNetworkBinaryLoop12Done",
          "id": "repository.record-buffered-binary-state-string-scan-completion-constructor"
        },
        {
          "name": "MlTranscodeLinkNetworkBinaryLoop12Return",
          "id": "repository.record-buffered-binary-state-string-scan-return-constructor"
        },
        {
          "name": "MlTranscodeLinkNetworkBinaryLoop16Done",
          "id": "repository.record-buffered-binary-state-text-copy-completion-constructor"
        },
        {
          "name": "MlTranscodeLinkNetworkBinaryLoop16Return",
          "id": "repository.record-buffered-binary-state-text-copy-return-constructor"
        }
      ],
      "experiments/binary-metadata-buffer/native/part_2.rs": [
        {
          "name": "MlTranscodeLinkNetworkBinaryLoop7Done",
          "id": "repository.record-buffered-binary-record-scan-completion-constructor"
        },
        {
          "name": "MlTranscodeLinkNetworkBinaryLoop7Return",
          "id": "repository.record-buffered-binary-record-scan-return-constructor"
        },
        {
          "name": "MlTranscodeLinkNetworkBinaryJoin23Next",
          "id": "repository.record-buffered-binary-optional-span-stage-next-state-constructor"
        },
        {
          "name": "MlTranscodeLinkNetworkBinaryJoin23Return",
          "id": "repository.record-buffered-binary-optional-span-stage-return-constructor"
        },
        {
          "name": "MlTranscodeLinkNetworkBinaryLoop25Done",
          "id": "repository.record-buffered-binary-span-field-scan-completion-constructor"
        },
        {
          "name": "MlTranscodeLinkNetworkBinaryLoop25Return",
          "id": "repository.record-buffered-binary-span-field-scan-return-constructor"
        },
        {
          "name": "MlTranscodeLinkNetworkBinaryLoop22Done",
          "id": "repository.record-buffered-binary-metadata-text-scan-completion-constructor"
        },
        {
          "name": "MlTranscodeLinkNetworkBinaryLoop22Return",
          "id": "repository.record-buffered-binary-metadata-text-scan-return-constructor"
        },
        {
          "name": "MlTranscodeLinkNetworkBinaryLoop28Done",
          "id": "repository.record-buffered-binary-metadata-text-copy-completion-constructor"
        },
        {
          "name": "MlTranscodeLinkNetworkBinaryLoop28Return",
          "id": "repository.record-buffered-binary-metadata-text-copy-return-constructor"
        },
        {
          "name": "MlTranscodeLinkNetworkBinaryLoop21Done",
          "id": "repository.record-buffered-binary-metadata-header-scan-completion-constructor"
        },
        {
          "name": "MlTranscodeLinkNetworkBinaryLoop21Return",
          "id": "repository.record-buffered-binary-metadata-header-scan-return-constructor"
        },
        {
          "name": "MlTranscodeLinkNetworkBinaryJoin20Next",
          "id": "repository.record-buffered-binary-metadata-boundary-stage-next-state-constructor"
        },
        {
          "name": "MlTranscodeLinkNetworkBinaryJoin20Return",
          "id": "repository.record-buffered-binary-metadata-boundary-stage-return-constructor"
        },
        {
          "name": "MlTranscodeLinkNetworkBinaryLoop19Done",
          "id": "repository.record-buffered-binary-reference-scan-completion-constructor"
        },
        {
          "name": "MlTranscodeLinkNetworkBinaryLoop19Return",
          "id": "repository.record-buffered-binary-reference-scan-return-constructor"
        }
      ],
      "experiments/binary-metadata-buffer/native/part_3.rs": [
        {
          "name": "MlTranscodeLinkNetworkBinaryJoin5Next",
          "id": "repository.record-buffered-binary-next-identity-stage-next-state-constructor"
        },
        {
          "name": "MlTranscodeLinkNetworkBinaryJoin5Return",
          "id": "repository.record-buffered-binary-next-identity-stage-return-constructor"
        },
        {
          "name": "MlTranscodeLinkNetworkBinaryJoin4Next",
          "id": "repository.record-buffered-binary-version-stage-next-state-constructor"
        },
        {
          "name": "MlTranscodeLinkNetworkBinaryJoin4Return",
          "id": "repository.record-buffered-binary-version-stage-return-constructor"
        },
        {
          "name": "MlTranscodeLinkNetworkBinaryLoop31Done",
          "id": "repository.record-buffered-binary-magic-header-scan-completion-constructor"
        },
        {
          "name": "MlTranscodeLinkNetworkBinaryLoop31Return",
          "id": "repository.record-buffered-binary-magic-header-scan-return-constructor"
        }
      ]
    }
  },
  {
    "inventory": "binary candidate generated continuations",
    "files": {
      "experiments/binary-metadata-buffer/native/part_1.rs": [
        {
          "name": "ml_read_link_network_binary_integer_loop1",
          "id": "repository.continue-binary-framed-integer-scan"
        },
        {
          "name": "ml_link_network_binary_integer_byte_loop2",
          "id": "repository.continue-binary-integer-byte-scan"
        },
        {
          "name": "ml_accept_link_network_binary_unicode_loop3",
          "id": "repository.continue-binary-scalar-unicode-scan"
        },
        {
          "name": "ml_transcode_link_network_binary_loop6",
          "id": "repository.continue-record-buffered-binary-count-write"
        },
        {
          "name": "ml_transcode_link_network_binary_loop10",
          "id": "repository.continue-record-buffered-binary-state-count-write"
        },
        {
          "name": "ml_transcode_link_network_binary_loop14",
          "id": "repository.continue-record-buffered-binary-cache-identity-write"
        },
        {
          "name": "ml_transcode_link_network_binary_loop15",
          "id": "repository.continue-record-buffered-binary-state-text-length-write"
        },
        {
          "name": "ml_transcode_link_network_binary_loop16",
          "id": "repository.continue-record-buffered-binary-state-text-copy"
        },
        {
          "name": "ml_transcode_link_network_binary_loop12",
          "id": "repository.continue-record-buffered-binary-state-string-scan"
        },
        {
          "name": "ml_transcode_link_network_binary_loop11",
          "id": "repository.continue-record-buffered-binary-state-entry-scan"
        }
      ],
      "experiments/binary-metadata-buffer/native/part_2.rs": [
        {
          "name": "ml_transcode_link_network_binary_loop9",
          "id": "repository.continue-record-buffered-binary-state-section-scan"
        },
        {
          "name": "ml_transcode_link_network_binary_loop17",
          "id": "repository.continue-record-buffered-binary-record-identity-write"
        },
        {
          "name": "ml_transcode_link_network_binary_loop18",
          "id": "repository.continue-record-buffered-binary-reference-count-write"
        },
        {
          "name": "ml_transcode_link_network_binary_loop24",
          "id": "repository.continue-record-buffered-binary-metadata-size-write"
        },
        {
          "name": "ml_transcode_link_network_binary_loop26",
          "id": "repository.continue-record-buffered-binary-span-scalar-write"
        },
        {
          "name": "ml_transcode_link_network_binary_loop25",
          "id": "repository.continue-record-buffered-binary-span-field-scan"
        },
        {
          "name": "ml_transcode_link_network_binary_loop27",
          "id": "repository.continue-record-buffered-binary-metadata-text-length-write"
        },
        {
          "name": "ml_transcode_link_network_binary_loop28",
          "id": "repository.continue-record-buffered-binary-metadata-text-copy"
        },
        {
          "name": "ml_transcode_link_network_binary_loop22",
          "id": "repository.continue-record-buffered-binary-metadata-text-scan"
        },
        {
          "name": "ml_transcode_link_network_binary_loop21",
          "id": "repository.continue-record-buffered-binary-metadata-header-scan"
        },
        {
          "name": "ml_transcode_link_network_binary_loop29",
          "id": "repository.continue-record-buffered-binary-target-identity-write"
        },
        {
          "name": "ml_transcode_link_network_binary_loop19",
          "id": "repository.continue-record-buffered-binary-reference-scan"
        }
      ],
      "experiments/binary-metadata-buffer/native/part_3.rs": [
        {
          "name": "ml_transcode_link_network_binary_loop7",
          "id": "repository.continue-record-buffered-binary-record-scan"
        },
        {
          "name": "ml_transcode_link_network_binary_loop30",
          "id": "repository.continue-record-buffered-binary-next-identity-write"
        },
        {
          "name": "ml_transcode_link_network_binary_loop31",
          "id": "repository.continue-record-buffered-binary-magic-header-scan"
        },
        {
          "name": "ml_transcode_link_network_binary_loop32",
          "id": "repository.continue-record-buffered-binary-magic-header-write"
        }
      ]
    }
  },
  {
    "inventory": "binary candidate source and style controls",
    "files": {
      "js/scripts/binary-metadata-candidate-styles.mjs": [
        {
          "name": "syntax",
          "id": "repository.read-binary-candidate-rust-syntax"
        },
        {
          "name": "generated",
          "id": "repository.accept-binary-candidate-generated-spelling"
        },
        {
          "name": "alphaMapping",
          "id": "repository.prove-binary-candidate-style-correspondence"
        },
        {
          "name": "rename",
          "id": "repository.rename-binary-candidate-style-identifiers"
        },
        {
          "name": "borrowHeader",
          "id": "repository.adapt-binary-candidate-header-borrow"
        },
        {
          "name": "freshSpelling",
          "id": "repository.derive-binary-candidate-structural-style"
        },
        {
          "name": "declaration",
          "id": "repository.read-binary-candidate-declaration-symbol"
        },
        {
          "name": "readBinaryMetadataCandidateStyles",
          "id": "repository.read-binary-candidate-library-styles"
        }
      ],
      "js/scripts/generate-link-network-binary-rules.mjs": [
        {
          "name": "styleBinaryItems",
          "id": "repository.style-binary-candidate-complete-declarations"
        }
      ],
      "js/scripts/source-text-byte-library-style.mjs": [
        {
          "name": "simplifySourceTextRustParentheses",
          "id": "repository.simplify-source-text-rust-parentheses"
        }
      ],
      "experiments/binary-metadata-buffer/model.test.mjs": [
        {
          "name": "compare",
          "id": "repository.compare-binary-metadata-candidate-framing"
        },
        {
          "name": "replace",
          "id": "repository.replace-binary-candidate-test-cell"
        }
      ],
      "experiments/binary-metadata-buffer/styles.test.mjs": [
        {
          "name": "derive",
          "id": "repository.derive-binary-candidate-test-style"
        },
        {
          "name": "apply",
          "id": "repository.apply-binary-candidate-test-style"
        }
      ]
    }
  },
  {
    "inventory": "binary candidate scalar framing test readers",
    "files": {
      "experiments/binary-metadata-buffer/model.test.mjs": [
        {
          "name": "bytes",
          "id": "repository.read-binary-candidate-test-bytes"
        },
        {
          "name": "fields",
          "id": "repository.read-binary-candidate-test-fields"
        }
      ],
      "experiments/binary-metadata-buffer/native-observations.mjs": [
        {
          "name": "bytes",
          "id": "repository.read-binary-candidate-test-bytes"
        },
        {
          "name": "fields",
          "id": "repository.read-binary-candidate-test-fields"
        }
      ]
    }
  },
  {
    "inventory": "binary candidate generation receipt roles",
    "files": {
      "experiments/binary-metadata-buffer/native-generation.lino": [
        {
          "name": "generationPolicy",
          "id": "repository.binary-candidate-native-generation-policy"
        },
        {
          "name": "candidateSourceSha256",
          "id": "repository.binary-candidate-source-digest"
        },
        {
          "name": "recordedCandidateContexts",
          "id": "repository.binary-candidate-style-context-count"
        }
      ]
    }
  },
  {
    "inventory": "binary metadata candidate native source oracle",
    "files": {
      "experiments/binary-metadata-buffer/native-observations.mjs": [
        {
          "name": "BINARY_METADATA_BUFFER_MODEL_COMMIT",
          "id": "repository.binary-candidate-native-model-revision"
        },
        {
          "name": "verifyBinaryMetadataBufferNativeModel",
          "id": "repository.verify-binary-candidate-native-model"
        },
        {
          "name": "binaryMetadataBufferNativeMode",
          "id": "repository.select-binary-candidate-native-observation"
        },
        {
          "name": "planBinaryMetadataBufferNative",
          "id": "repository.plan-binary-candidate-native-observation"
        },
        {
          "name": "runBinaryMetadataBufferNative",
          "id": "repository.run-binary-candidate-native-observation"
        },
        {
          "name": "inventory",
          "id": "repository.read-binary-candidate-native-inventory"
        },
        {
          "name": "nativeInterface",
          "id": "repository.read-binary-candidate-native-interface"
        },
        {
          "name": "archiveReceipt",
          "id": "repository.archive-binary-candidate-native-source"
        },
        {
          "name": "cases",
          "id": "repository.read-binary-candidate-native-cases"
        },
        {
          "name": "consumerSource",
          "id": "repository.render-binary-candidate-native-consumer"
        },
        {
          "name": "emittedJavaScript",
          "id": "repository.execute-binary-candidate-generated-javascript"
        },
        {
          "name": "parseObservations",
          "id": "repository.read-binary-candidate-native-observations"
        }
      ]
    }
  },
  {
    "inventory": "binary metadata candidate exact native providers",
    "files": {
      "experiments/binary-metadata-buffer/native-observations.mjs": [
        {
          "name": "sha256",
          "id": "repository.hash-rust-text-native-oracle-content"
        },
        {
          "name": "textBytes",
          "id": "repository.read-rust-text-native-oracle-bytes"
        },
        {
          "name": "freeze",
          "id": "repository.freeze-rust-text-native-observation-plan"
        },
        {
          "name": "numberBits",
          "id": "repository.encode-text-certificate-native-number-bits"
        },
        {
          "name": "bitsNumber",
          "id": "repository.decode-text-certificate-native-number-bits"
        }
      ]
    }
  },
  {
    "inventory": "binary metadata candidate shared complete source runner",
    "files": {
      "experiments/binary-metadata-buffer/native-observations.mjs": [
        {
          "name": "createSourceNativeObservationRunner",
          "id": "repository.create-source-native-observation-runner"
        }
      ]
    }
  },
  {
    "inventory": "binary metadata candidate exact test digests",
    "files": {
      "experiments/binary-metadata-buffer/native-observations.test.mjs": [
        {
          "name": "digest",
          "id": "repository.hash-rust-text-native-oracle-content"
        }
      ]
    }
  },
  {
    "inventory": "binary metadata candidate complete fake process contracts",
    "files": {
      "experiments/binary-metadata-buffer/native-observations.test.mjs": [
        {
          "name": "fakeProcess",
          "id": "repository.create-text-certificate-native-test-process"
        }
      ]
    }
  },
  {
    "inventory": "binary metadata candidate exact aggregate failure contracts",
    "files": {
      "experiments/binary-metadata-buffer/native-observations.test.mjs": [
        {
          "name": "fails",
          "id": "repository.require-text-certificate-native-test-failure"
        }
      ]
    }
  },
  {
    "inventory": "binary metadata candidate injected observation contracts",
    "files": {
      "experiments/binary-metadata-buffer/native-observations.test.mjs": [
        {
          "name": "injected",
          "id": "repository.run-binary-candidate-injected-contract"
        }
      ]
    }
  },
  {
    "inventory": "complete original table native source oracle",
    "files": {
      "experiments/original-link-complete-table-projection/native-observations.mjs": [
        {
          "name": "ORIGINAL_LINK_COMPLETE_TABLE_MODEL_COMMIT",
          "id": "repository.complete-original-table-native-model-revision"
        },
        {
          "name": "verifyOriginalLinkCompleteTableNativeModel",
          "id": "repository.verify-complete-original-table-native-model"
        },
        {
          "name": "originalLinkCompleteTableNativeMode",
          "id": "repository.select-complete-original-table-native-observation"
        },
        {
          "name": "planOriginalLinkCompleteTableNative",
          "id": "repository.plan-complete-original-table-native-observation"
        },
        {
          "name": "runOriginalLinkCompleteTableNative",
          "id": "repository.run-complete-original-table-native-observation"
        },
        {
          "name": "argumentBits",
          "id": "repository.encode-complete-table-native-arguments"
        },
        {
          "name": "fieldRelation",
          "id": "repository.read-complete-table-native-field"
        },
        {
          "name": "selectedProof",
          "id": "repository.read-complete-table-native-source-proof"
        },
        {
          "name": "proofRows",
          "id": "repository.read-complete-table-native-proof-rows"
        },
        {
          "name": "attachFixture",
          "id": "repository.attach-complete-table-native-fixture-proof"
        },
        {
          "name": "fixturePacket",
          "id": "repository.create-complete-table-native-fixture-packet"
        },
        {
          "name": "archivePacket",
          "id": "repository.archive-complete-table-native-packet"
        },
        {
          "name": "actualOrdinal",
          "id": "repository.read-complete-table-native-original-ordinal"
        },
        {
          "name": "scalarOrdinal",
          "id": "repository.read-complete-table-native-scalar-ordinal"
        },
        {
          "name": "nativeInterface",
          "id": "repository.read-complete-table-native-interface"
        },
        {
          "name": "expectedTables",
          "id": "repository.read-complete-table-native-reference-tables"
        },
        {
          "name": "certificateObservation",
          "id": "repository.observe-complete-table-reference-certificate"
        },
        {
          "name": "actualExecution",
          "id": "repository.execute-complete-table-reference-javascript"
        },
        {
          "name": "cases",
          "id": "repository.read-complete-table-native-cases"
        },
        {
          "name": "protocolStream",
          "id": "repository.render-complete-table-native-bit-protocol"
        },
        {
          "name": "consumerSource",
          "id": "repository.render-complete-table-native-consumer"
        },
        {
          "name": "parseObservations",
          "id": "repository.read-complete-table-native-observations"
        }
      ]
    }
  },
  {
    "inventory": "complete original table exact native providers",
    "files": {
      "experiments/original-link-complete-table-projection/native-observations.mjs": [
        {
          "name": "sha256",
          "id": "repository.hash-rust-text-native-oracle-content"
        },
        {
          "name": "textBytes",
          "id": "repository.read-rust-text-native-oracle-bytes"
        },
        {
          "name": "freeze",
          "id": "repository.freeze-rust-text-native-observation-plan"
        },
        {
          "name": "numberBits",
          "id": "repository.encode-text-certificate-native-number-bits"
        },
        {
          "name": "bitsNumber",
          "id": "repository.decode-text-certificate-native-number-bits"
        }
      ]
    }
  },
  {
    "inventory": "complete original table shared complete source runner",
    "files": {
      "experiments/original-link-complete-table-projection/native-observations.mjs": [
        {
          "name": "createSourceNativeObservationRunner",
          "id": "repository.create-source-native-observation-runner"
        }
      ]
    }
  },
  {
    "inventory": "complete original table exact test digests",
    "files": {
      "experiments/original-link-complete-table-projection/native-observations.test.mjs": [
        {
          "name": "digest",
          "id": "repository.hash-rust-text-native-oracle-content"
        }
      ]
    }
  },
  {
    "inventory": "complete original table complete fake process contracts",
    "files": {
      "experiments/original-link-complete-table-projection/native-observations.test.mjs": [
        {
          "name": "fakeProcess",
          "id": "repository.create-text-certificate-native-test-process"
        }
      ]
    }
  },
  {
    "inventory": "complete original table exact aggregate failure contracts",
    "files": {
      "experiments/original-link-complete-table-projection/native-observations.test.mjs": [
        {
          "name": "fails",
          "id": "repository.require-text-certificate-native-test-failure"
        }
      ]
    }
  },
  {
    "inventory": "complete original table injected observation contracts",
    "files": {
      "experiments/original-link-complete-table-projection/native-observations.test.mjs": [
        {
          "name": "injected",
          "id": "repository.run-complete-original-table-injected-contract"
        }
      ]
    }
  },
  {
    "inventory": "complete table native shared actual owner controls",
    "files": {
      "experiments/original-link-complete-table-projection/native-observations.mjs": [
        {
          "name": "copyArguments",
          "id": "repository.copy-original-link-table-test-arguments"
        },
        {
          "name": "key",
          "id": "repository.read-exact-link-identity-key"
        },
        {
          "name": "fieldLabel",
          "id": "repository.render-certificate-test-field-label"
        },
        {
          "name": "references",
          "id": "repository.read-original-link-role-test-references"
        },
        {
          "name": "selectedBody",
          "id": "repository.read-original-link-role-test-body"
        },
        {
          "name": "replaceField",
          "id": "repository.replace-text-certificate-test-field"
        }
      ]
    }
  },
  {
    "inventory": "complete table injected protocol controls",
    "files": {
      "experiments/original-link-complete-table-projection/native-observations.test.mjs": [
        {
          "name": "bitStream",
          "id": "repository.render-complete-table-test-bit-protocol"
        },
        {
          "name": "finite",
          "id": "repository.decode-complete-table-test-number"
        },
        {
          "name": "altered",
          "id": "repository.alter-complete-table-test-protocol"
        }
      ]
    }
  },
  {
    "inventory": "complete table scoped native oracle controls",
    "files": {
      "experiments/original-link-complete-table-projection/native-observations.mjs": [
        {
          "name": "bind",
          "id": "repository.bind-complete-table-reference-owner"
        },
        {
          "name": "fld",
          "id": "repository.read-complete-table-reference-field"
        },
        {
          "name": "children",
          "id": "repository.read-complete-table-reference-child"
        },
        {
          "name": "walk",
          "id": "repository.walk-complete-table-reference-operation"
        },
        {
          "name": "identity",
          "id": "repository.map-complete-table-reference-identity"
        },
        {
          "name": "rewrite",
          "id": "repository.rewrite-complete-table-reference-identity-columns"
        },
        {
          "name": "add",
          "id": "repository.append-complete-table-native-observation"
        },
        {
          "name": "graphCase",
          "id": "repository.observe-complete-table-native-graph-case"
        },
        {
          "name": "mutation",
          "id": "repository.mutate-complete-table-native-transport"
        },
        {
          "name": "pool",
          "id": "repository.pool-complete-table-native-bit-vector"
        }
      ]
    }
  },
  {
    "inventory": "binary candidate scoped native oracle controls",
    "files": {
      "experiments/binary-metadata-buffer/native-observations.mjs": [
        {
          "name": "add",
          "id": "repository.append-binary-candidate-native-observation"
        }
      ]
    }
  },
  {
    "inventory": "native oracle exact transport and host distinctions",
    "files": {
      "experiments/original-link-complete-table-projection/native-observations.mjs": [
        {
          "name": "inputArguments",
          "id": "repository.complete-table-native-input-arguments"
        },
        {
          "name": "resultRows",
          "id": "repository.complete-table-native-result-rows"
        },
        {
          "name": "rawRefusals",
          "id": "repository.complete-table-native-host-refusals"
        },
        {
          "name": "certificate",
          "id": "repository.complete-table-reference-certificate-observation"
        },
        {
          "name": "provenance",
          "id": "repository.complete-table-native-archive-provenance"
        },
        {
          "name": "maximumProtocolBytes",
          "id": "repository.complete-table-native-protocol-bound"
        },
        {
          "name": "maximumConsumerBytes",
          "id": "repository.complete-table-native-consumer-bound"
        }
      ],
      "experiments/binary-metadata-buffer/native-observations.mjs": [
        {
          "name": "resultHex",
          "id": "repository.binary-candidate-native-result-bytes"
        },
        {
          "name": "recordLimitBits",
          "id": "repository.binary-candidate-native-record-limit"
        },
        {
          "name": "byteLimitBits",
          "id": "repository.binary-candidate-native-byte-limit"
        }
      ]
    }
  },
  {
    "inventory": "owned native generated decision coverage",
    "files": {
      "rust/tests/unit/translation_rust_owned_local_state.rs": [
        {
          "name": "actual_generated_owned_decisions_match_linked_fixtures_and_javascript",
          "id": "repository.check-native-owned-decision-fixture-parity"
        },
        {
          "name": "actual_native_text_renderer_preserves_javascript_bytes_and_utf16_input_decisions",
          "id": "repository.check-native-owned-text-output-parity"
        }
      ]
    }
  },
  {
    "inventory": "complete table and binary workflow conservation",
    "files": {
      "js/tests/ci-workflow-structure.test.js": [
        {
          "name": "checkBinaryMetadataNativeSteps",
          "id": "repository.check-binary-metadata-native-workflow-steps"
        },
        {
          "name": "checkCompleteTableNativeSteps",
          "id": "repository.check-complete-table-native-workflow-steps"
        },
        {
          "name": "ADDITIONAL_SOURCE_PACKAGE_CELLS",
          "id": "repository.additional-source-package-cells"
        },
        {
          "name": "COMPLETE_TABLE_PACKAGE_STEPS",
          "id": "repository.complete-table-package-steps"
        },
        {
          "name": "BINARY_METADATA_PACKAGE_STEPS",
          "id": "repository.binary-metadata-package-steps"
        },
        {
          "name": "BINARY_METADATA_NATIVE_STEPS",
          "id": "repository.binary-metadata-native-steps"
        },
        {
          "name": "COMPLETE_TABLE_NATIVE_STEPS",
          "id": "repository.complete-table-native-steps"
        }
      ]
    }
  }
];
const normalizeCompleteTableBinaryCandidateName=name=>name.replace(/([a-z0-9])([A-Z])/gu,'$1-$2').replaceAll('_','-').replaceAll(' ','-').toLowerCase().replace(/^fld$/u,'field-path').replace(/^refs$/u,'references').replace(/^children$/u,'child-references').replace(/^emitted-/u,'emit-').replace(/\brenderer\b/gu,'rendering').replace(/\butf16\b/gu,'unicode-transformation-format-sixteen-bit').replace(/^f$/u,'field-code').replace(/\bsha256\b/gu,'secure-hash-algorithm-two-hundred-fifty-six').replace(/\bid\b/gu,'identity').replace(/\bml\b/gu,'meta-language').replace(/\b(loop|join)([0-9]+)\b/gu,(whole,kind,digits)=>{const words={1:'one',2:'two',3:'three',4:'four',5:'five',6:'six',7:'seven',8:'eight',9:'nine',10:'ten',11:'eleven',12:'twelve',13:'thirteen',14:'fourteen',15:'fifteen',16:'sixteen',17:'seventeen',18:'eighteen',19:'nineteen',20:'twenty',21:'twenty-one',22:'twenty-two',23:'twenty-three',24:'twenty-four',25:'twenty-five',26:'twenty-six',27:'twenty-seven',28:'twenty-eight',29:'twenty-nine',30:'thirty',31:'thirty-one',32:'thirty-two'};return words[digits]?kind+'-'+words[digits]:whole;});
const COMPLETE_TABLE_BINARY_CANDIDATE_SOURCE_TEXTS=new Map();
const COMPLETE_TABLE_BINARY_CANDIDATE_NAME_INVENTORIES=COMPLETE_TABLE_BINARY_CANDIDATE_NAME_BINDINGS.map(({inventory,files})=>({
 inventory,files:Object.keys(files),
 extract:(raw,file)=>{let text=raw;if(file.endsWith('.lino')){const saved=COMPLETE_TABLE_BINARY_CANDIDATE_SOURCE_TEXTS.get(file);if(!saved||saved.raw!==raw)COMPLETE_TABLE_BINARY_CANDIDATE_SOURCE_TEXTS.set(file,{raw,text:JSON.stringify(readStructuredDataLino(raw))});text=COMPLETE_TABLE_BINARY_CANDIDATE_SOURCE_TEXTS.get(file).text;}return files[file].filter(({name})=>new RegExp('\\b'+name+'\\b','u').test(text)).map(({name})=>normalizeCompleteTableBinaryCandidateName(name));},
 recordOf:name=>Object.values(files).flat().find(entry=>normalizeCompleteTableBinaryCandidateName(entry.name)===name).id,
}));
// Original raw graph, finite cancellation evidence and exact source oracle meanings.
const ORIGINAL_RAW_CANCELLATION_NAME_BINDINGS=[
  {
    "inventory": "raw exact JavaScript identity source",
    "files": {
      "experiments/original-link-complete-table-projection/raw-rules.mjs": [
        {
          "name": "acceptOriginalLinkIdentityWord",
          "id": "repository.accept-original-link-identity-word"
        },
        {
          "name": "checkOriginalLinkIdentityIndex",
          "id": "repository.check-original-link-identity-index"
        },
        {
          "name": "findOriginalLinkIdentityOrdinal",
          "id": "repository.find-original-link-identity-ordinal"
        },
        {
          "name": "readOriginalLinkIdentityOrdinals",
          "id": "repository.read-original-link-identity-ordinals"
        }
      ],
      "experiments/original-link-complete-table-projection/data/raw-native-definitions.lino": [
        {
          "name": "acceptOriginalLinkIdentityWord",
          "id": "repository.accept-original-link-identity-word"
        },
        {
          "name": "checkOriginalLinkIdentityIndex",
          "id": "repository.check-original-link-identity-index"
        },
        {
          "name": "findOriginalLinkIdentityOrdinal",
          "id": "repository.find-original-link-identity-ordinal"
        },
        {
          "name": "readOriginalLinkIdentityOrdinals",
          "id": "repository.read-original-link-identity-ordinals"
        }
      ]
    }
  },
  {
    "inventory": "raw exact native scalar provider",
    "files": {
      "experiments/original-link-complete-table-projection/data/raw-native-definitions.lino": [
        {
          "name": "accept_original_link_identity_word",
          "id": "repository.accept-original-link-identity-word"
        }
      ]
    }
  },
  {
    "inventory": "raw exact native Boolean scan wrapper",
    "files": {
      "experiments/original-link-complete-table-projection/data/raw-native-definitions.lino": [
        {
          "name": "MlCheckOriginalLinkIdentityIndexLoop1Result",
          "id": "repository.original-link-identity-index-scan-control-result"
        },
        {
          "name": "MlCheckOriginalLinkIdentityIndexLoop1Done",
          "id": "repository.original-link-identity-index-scan-completion-constructor"
        },
        {
          "name": "MlCheckOriginalLinkIdentityIndexLoop1Return",
          "id": "repository.original-link-identity-index-scan-return-constructor"
        }
      ]
    }
  },
  {
    "inventory": "raw exact native Boolean source wrapper",
    "files": {
      "experiments/original-link-complete-table-projection/data/raw-native-definitions.lino": [
        {
          "name": "ml_checkOriginalLinkIdentityIndex_loop1_result",
          "id": "repository.original-link-identity-index-scan-control-result"
        }
      ]
    }
  },
  {
    "inventory": "original raw graph pure operations",
    "files": {
      "experiments/original-link-complete-table-projection/raw-rules.mjs": [
        {
          "name": "readOriginalRawPolicy",
          "id": "repository.read-original-raw-graph-policy"
        },
        {
          "name": "acceptOriginalRawInteger",
          "id": "repository.accept-original-raw-graph-integer"
        },
        {
          "name": "hasOriginalRawMember",
          "id": "repository.contain-original-raw-graph-member"
        },
        {
          "name": "readOriginalRawMemberPosition",
          "id": "repository.read-original-raw-graph-member-position"
        },
        {
          "name": "equalOriginalRawUnits",
          "id": "repository.equal-original-raw-graph-unit-ranges"
        },
        {
          "name": "readOriginalRawTextCode",
          "id": "repository.read-original-raw-graph-text-code"
        },
        {
          "name": "readOriginalRawFieldCode",
          "id": "repository.read-original-raw-graph-field-code"
        },
        {
          "name": "readOriginalRawReferenceRole",
          "id": "repository.read-original-raw-graph-reference-role"
        },
        {
          "name": "readOriginalRawReferences",
          "id": "repository.read-original-raw-graph-references"
        },
        {
          "name": "readOriginalRawField",
          "id": "repository.read-original-raw-graph-field"
        },
        {
          "name": "equalOriginalRawStrings",
          "id": "repository.equal-original-raw-graph-strings"
        },
        {
          "name": "acceptOriginalRawNumberText",
          "id": "repository.accept-original-raw-graph-number-text"
        },
        {
          "name": "readOriginalRawRoleField",
          "id": "repository.read-original-raw-graph-role-field"
        },
        {
          "name": "readOriginalRawRoleList",
          "id": "repository.read-original-raw-graph-role-list"
        },
        {
          "name": "equalOriginalRawOptionalOwner",
          "id": "repository.equal-original-raw-graph-optional-owner"
        },
        {
          "name": "checkOriginalLinkRawRoles",
          "id": "repository.check-original-raw-graph-owned-roles"
        },
        {
          "name": "checkOriginalLinkRawShape",
          "id": "repository.check-original-raw-graph-shape"
        }
      ],
      "experiments/original-link-complete-table-projection/data/raw-native-definitions.lino": [
        {
          "name": "readOriginalRawPolicy",
          "id": "repository.read-original-raw-graph-policy"
        },
        {
          "name": "read_original_raw_policy",
          "id": "repository.read-original-raw-graph-policy"
        },
        {
          "name": "acceptOriginalRawInteger",
          "id": "repository.accept-original-raw-graph-integer"
        },
        {
          "name": "accept_original_raw_integer",
          "id": "repository.accept-original-raw-graph-integer"
        },
        {
          "name": "hasOriginalRawMember",
          "id": "repository.contain-original-raw-graph-member"
        },
        {
          "name": "has_original_raw_member",
          "id": "repository.contain-original-raw-graph-member"
        },
        {
          "name": "readOriginalRawMemberPosition",
          "id": "repository.read-original-raw-graph-member-position"
        },
        {
          "name": "read_original_raw_member_position",
          "id": "repository.read-original-raw-graph-member-position"
        },
        {
          "name": "equalOriginalRawUnits",
          "id": "repository.equal-original-raw-graph-unit-ranges"
        },
        {
          "name": "equal_original_raw_units",
          "id": "repository.equal-original-raw-graph-unit-ranges"
        },
        {
          "name": "readOriginalRawTextCode",
          "id": "repository.read-original-raw-graph-text-code"
        },
        {
          "name": "read_original_raw_text_code",
          "id": "repository.read-original-raw-graph-text-code"
        },
        {
          "name": "readOriginalRawFieldCode",
          "id": "repository.read-original-raw-graph-field-code"
        },
        {
          "name": "read_original_raw_field_code",
          "id": "repository.read-original-raw-graph-field-code"
        },
        {
          "name": "readOriginalRawReferenceRole",
          "id": "repository.read-original-raw-graph-reference-role"
        },
        {
          "name": "read_original_raw_reference_role",
          "id": "repository.read-original-raw-graph-reference-role"
        },
        {
          "name": "readOriginalRawReferences",
          "id": "repository.read-original-raw-graph-references"
        },
        {
          "name": "read_original_raw_references",
          "id": "repository.read-original-raw-graph-references"
        },
        {
          "name": "readOriginalRawField",
          "id": "repository.read-original-raw-graph-field"
        },
        {
          "name": "read_original_raw_field",
          "id": "repository.read-original-raw-graph-field"
        },
        {
          "name": "equalOriginalRawStrings",
          "id": "repository.equal-original-raw-graph-strings"
        },
        {
          "name": "equal_original_raw_strings",
          "id": "repository.equal-original-raw-graph-strings"
        },
        {
          "name": "acceptOriginalRawNumberText",
          "id": "repository.accept-original-raw-graph-number-text"
        },
        {
          "name": "accept_original_raw_number_text",
          "id": "repository.accept-original-raw-graph-number-text"
        },
        {
          "name": "readOriginalRawRoleField",
          "id": "repository.read-original-raw-graph-role-field"
        },
        {
          "name": "read_original_raw_role_field",
          "id": "repository.read-original-raw-graph-role-field"
        },
        {
          "name": "readOriginalRawRoleList",
          "id": "repository.read-original-raw-graph-role-list"
        },
        {
          "name": "read_original_raw_role_list",
          "id": "repository.read-original-raw-graph-role-list"
        },
        {
          "name": "equalOriginalRawOptionalOwner",
          "id": "repository.equal-original-raw-graph-optional-owner"
        },
        {
          "name": "equal_original_raw_optional_owner",
          "id": "repository.equal-original-raw-graph-optional-owner"
        },
        {
          "name": "checkOriginalLinkRawRoles",
          "id": "repository.check-original-raw-graph-owned-roles"
        },
        {
          "name": "check_original_link_raw_roles",
          "id": "repository.check-original-raw-graph-owned-roles"
        },
        {
          "name": "checkOriginalLinkRawShape",
          "id": "repository.check-original-raw-graph-shape"
        },
        {
          "name": "check_original_link_raw_shape",
          "id": "repository.check-original-raw-graph-shape"
        }
      ]
    }
  },
  {
    "inventory": "original raw graph source and policy operations",
    "files": {
      "experiments/original-link-complete-table-projection/raw-policy.mjs": [
        {
          "name": "originalLinkRawPolicy",
          "id": "repository.original-raw-graph-policy"
        },
        {
          "name": "originalLinkRawInstructions",
          "id": "repository.create-original-raw-graph-instructions"
        },
        {
          "name": "renderOriginalLinkRawInstructions",
          "id": "repository.render-original-raw-graph-instructions"
        },
        {
          "name": "record",
          "id": "repository.create-original-raw-graph-record-recipe"
        },
        {
          "name": "list",
          "id": "repository.create-original-raw-graph-list-recipe"
        },
        {
          "name": "pack",
          "id": "repository.pack-original-raw-graph-policy-text"
        }
      ],
      "experiments/original-link-complete-table-projection/raw-transport.mjs": [
        {
          "name": "kind",
          "id": "repository.read-original-raw-graph-host-kind"
        },
        {
          "name": "readOriginalLinkRawTransport",
          "id": "repository.read-original-raw-graph-transport"
        },
        {
          "name": "checkOriginalLinkRawSelectedGraph",
          "id": "repository.check-original-raw-graph-selected-source"
        }
      ],
      "experiments/original-link-complete-table-projection/generate-raw.mjs": [
        {
          "name": "generateOriginalLinkRawJavaScript",
          "id": "repository.generate-original-raw-graph-javascript"
        },
        {
          "name": "generateOriginalLinkRawNative",
          "id": "repository.generate-original-raw-graph-native-artifact"
        },
        {
          "name": "recordOriginalLinkRawTranslationAttempt",
          "id": "repository.record-original-raw-graph-translation-attempt"
        }
      ]
    }
  },
  {
    "inventory": "raw owned native identity realizations",
    "files": {
      "experiments/original-link-complete-table-projection/data/raw-native-definitions.lino": [
        {
          "name": "check_original_link_identity_index",
          "id": "repository.check-owned-original-link-identity-index"
        },
        {
          "name": "find_original_link_identity_ordinal",
          "id": "repository.find-owned-original-link-identity-ordinal"
        },
        {
          "name": "read_original_link_identity_ordinals",
          "id": "repository.read-owned-original-link-identity-ordinals"
        }
      ]
    }
  },
  {
    "inventory": "original raw generated continuations",
    "files": {
      "experiments/original-link-complete-table-projection/data/raw-native-definitions.lino": [
        {
          "name": "ml_checkOriginalLinkIdentityIndex_loop1",
          "id": "repository.continue-owned-original-identity-index-scan"
        },
        {
          "name": "ml_check_original_link_identity_index_loop1",
          "id": "repository.continue-owned-original-identity-index-scan"
        },
        {
          "name": "ml_findOriginalLinkIdentityOrdinal_loop2",
          "id": "repository.continue-owned-original-identity-ordinal-search"
        },
        {
          "name": "ml_find_original_link_identity_ordinal_loop2",
          "id": "repository.continue-owned-original-identity-ordinal-search"
        },
        {
          "name": "ml_readOriginalLinkIdentityOrdinals_loop4",
          "id": "repository.continue-owned-original-identity-result-scan"
        },
        {
          "name": "ml_read_original_link_identity_ordinals_loop4",
          "id": "repository.continue-owned-original-identity-result-scan"
        },
        {
          "name": "ml_readOriginalLinkIdentityOrdinals_loop3",
          "id": "repository.continue-owned-original-identity-query-scan"
        },
        {
          "name": "ml_read_original_link_identity_ordinals_loop3",
          "id": "repository.continue-owned-original-identity-query-scan"
        },
        {
          "name": "ml_equalOriginalRawUnits_loop5",
          "id": "repository.continue-original-raw-unit-range-scan"
        },
        {
          "name": "ml_equal_original_raw_units_loop5",
          "id": "repository.continue-original-raw-unit-range-scan"
        },
        {
          "name": "ml_readOriginalRawTextCode_loop6",
          "id": "repository.continue-original-raw-fixed-text-scan"
        },
        {
          "name": "ml_read_original_raw_text_code_loop6",
          "id": "repository.continue-original-raw-fixed-text-scan"
        },
        {
          "name": "ml_readOriginalRawFieldCode_loop8",
          "id": "repository.continue-original-raw-field-normalization"
        },
        {
          "name": "ml_read_original_raw_field_code_loop8",
          "id": "repository.continue-original-raw-field-normalization"
        },
        {
          "name": "ml_readOriginalRawFieldCode_loop9",
          "id": "repository.continue-original-raw-field-dictionary-scan"
        },
        {
          "name": "ml_read_original_raw_field_code_loop9",
          "id": "repository.continue-original-raw-field-dictionary-scan"
        },
        {
          "name": "ml_readOriginalRawFieldCode_loop7",
          "id": "repository.continue-original-raw-field-alias-scan"
        },
        {
          "name": "ml_read_original_raw_field_code_loop7",
          "id": "repository.continue-original-raw-field-alias-scan"
        },
        {
          "name": "ml_readOriginalRawReferenceRole_loop10",
          "id": "repository.continue-original-raw-reference-label-scan"
        },
        {
          "name": "ml_read_original_raw_reference_role_loop10",
          "id": "repository.continue-original-raw-reference-label-scan"
        },
        {
          "name": "ml_acceptOriginalRawNumberText_loop14",
          "id": "repository.continue-original-raw-exponent-digit-scan"
        },
        {
          "name": "ml_accept_original_raw_number_text_loop14",
          "id": "repository.continue-original-raw-exponent-digit-scan"
        },
        {
          "name": "ml_acceptOriginalRawNumberText_loop15",
          "id": "repository.continue-original-raw-fraction-digit-scan"
        },
        {
          "name": "ml_accept_original_raw_number_text_loop15",
          "id": "repository.continue-original-raw-fraction-digit-scan"
        },
        {
          "name": "ml_acceptOriginalRawNumberText_loop11",
          "id": "repository.continue-original-raw-leading-digit-scan"
        },
        {
          "name": "ml_accept_original_raw_number_text_loop11",
          "id": "repository.continue-original-raw-leading-digit-scan"
        },
        {
          "name": "ml_checkOriginalLinkRawRoles_loop31",
          "id": "repository.continue-original-raw-condition-member-scan"
        },
        {
          "name": "ml_check_original_link_raw_roles_loop31",
          "id": "repository.continue-original-raw-condition-member-scan"
        },
        {
          "name": "ml_checkOriginalLinkRawRoles_loop30",
          "id": "repository.continue-original-raw-condition-owner-scan"
        },
        {
          "name": "ml_check_original_link_raw_roles_loop30",
          "id": "repository.continue-original-raw-condition-owner-scan"
        },
        {
          "name": "ml_checkOriginalLinkRawRoles_loop29",
          "id": "repository.continue-original-raw-condition-field-scan"
        },
        {
          "name": "ml_check_original_link_raw_roles_loop29",
          "id": "repository.continue-original-raw-condition-field-scan"
        },
        {
          "name": "ml_checkOriginalLinkRawRoles_loop28",
          "id": "repository.continue-original-raw-token-owner-scan"
        },
        {
          "name": "ml_check_original_link_raw_roles_loop28",
          "id": "repository.continue-original-raw-token-owner-scan"
        },
        {
          "name": "ml_checkOriginalLinkRawRoles_loop27",
          "id": "repository.continue-original-raw-function-proof-owner-scan"
        },
        {
          "name": "ml_check_original_link_raw_roles_loop27",
          "id": "repository.continue-original-raw-function-proof-owner-scan"
        },
        {
          "name": "ml_checkOriginalLinkRawRoles_loop32",
          "id": "repository.continue-original-raw-operation-part-scan"
        },
        {
          "name": "ml_check_original_link_raw_roles_loop32",
          "id": "repository.continue-original-raw-operation-part-scan"
        },
        {
          "name": "ml_checkOriginalLinkRawRoles_loop26",
          "id": "repository.continue-original-raw-operation-owner-scan"
        },
        {
          "name": "ml_check_original_link_raw_roles_loop26",
          "id": "repository.continue-original-raw-operation-owner-scan"
        },
        {
          "name": "ml_checkOriginalLinkRawRoles_loop34",
          "id": "repository.continue-original-raw-position-owner-scan"
        },
        {
          "name": "ml_check_original_link_raw_roles_loop34",
          "id": "repository.continue-original-raw-position-owner-scan"
        },
        {
          "name": "ml_checkOriginalLinkRawRoles_loop35",
          "id": "repository.continue-original-raw-parameter-name-scan"
        },
        {
          "name": "ml_check_original_link_raw_roles_loop35",
          "id": "repository.continue-original-raw-parameter-name-scan"
        },
        {
          "name": "ml_checkOriginalLinkRawRoles_loop33",
          "id": "repository.continue-original-raw-parameter-owner-scan"
        },
        {
          "name": "ml_check_original_link_raw_roles_loop33",
          "id": "repository.continue-original-raw-parameter-owner-scan"
        },
        {
          "name": "ml_checkOriginalLinkRawRoles_loop25",
          "id": "repository.continue-original-raw-declaration-owner-scan"
        },
        {
          "name": "ml_check_original_link_raw_roles_loop25",
          "id": "repository.continue-original-raw-declaration-owner-scan"
        },
        {
          "name": "ml_checkOriginalLinkRawRoles_loop40",
          "id": "repository.continue-original-raw-number-token-field-scan"
        },
        {
          "name": "ml_check_original_link_raw_roles_loop40",
          "id": "repository.continue-original-raw-number-token-field-scan"
        },
        {
          "name": "ml_checkOriginalLinkRawRoles_loop37",
          "id": "repository.continue-original-raw-required-field-scan"
        },
        {
          "name": "ml_check_original_link_raw_roles_loop37",
          "id": "repository.continue-original-raw-required-field-scan"
        },
        {
          "name": "ml_checkOriginalLinkRawRoles_loop41",
          "id": "repository.continue-original-raw-field-role-instruction-scan"
        },
        {
          "name": "ml_check_original_link_raw_roles_loop41",
          "id": "repository.continue-original-raw-field-role-instruction-scan"
        },
        {
          "name": "ml_checkOriginalLinkRawRoles_loop36",
          "id": "repository.continue-original-raw-represented-field-scan"
        },
        {
          "name": "ml_check_original_link_raw_roles_loop36",
          "id": "repository.continue-original-raw-represented-field-scan"
        },
        {
          "name": "ml_checkOriginalLinkRawRoles_loop24",
          "id": "repository.continue-original-raw-role-obligation-scan"
        },
        {
          "name": "ml_check_original_link_raw_roles_loop24",
          "id": "repository.continue-original-raw-role-obligation-scan"
        },
        {
          "name": "ml_checkOriginalLinkRawShape_loop53",
          "id": "repository.continue-original-raw-mapping-key-scan"
        },
        {
          "name": "ml_check_original_link_raw_shape_loop53",
          "id": "repository.continue-original-raw-mapping-key-scan"
        },
        {
          "name": "ml_checkOriginalLinkRawShape_loop52",
          "id": "repository.continue-original-raw-mapping-pair-scan"
        },
        {
          "name": "ml_check_original_link_raw_shape_loop52",
          "id": "repository.continue-original-raw-mapping-pair-scan"
        },
        {
          "name": "ml_checkOriginalLinkRawShape_loop50",
          "id": "repository.continue-original-raw-owned-structure-scan"
        },
        {
          "name": "ml_check_original_link_raw_shape_loop50",
          "id": "repository.continue-original-raw-owned-structure-scan"
        },
        {
          "name": "ml_checkOriginalLinkRawShape_loop49",
          "id": "repository.continue-original-raw-field-claim-scan"
        },
        {
          "name": "ml_check_original_link_raw_shape_loop49",
          "id": "repository.continue-original-raw-field-claim-scan"
        },
        {
          "name": "ml_checkOriginalLinkRawShape_loop54",
          "id": "repository.continue-original-raw-facet-field-scan"
        },
        {
          "name": "ml_check_original_link_raw_shape_loop54",
          "id": "repository.continue-original-raw-facet-field-scan"
        },
        {
          "name": "ml_checkOriginalLinkRawShape_loop48",
          "id": "repository.continue-original-raw-selected-facet-scan"
        },
        {
          "name": "ml_check_original_link_raw_shape_loop48",
          "id": "repository.continue-original-raw-selected-facet-scan"
        },
        {
          "name": "ml_checkOriginalLinkRawShape_loop55",
          "id": "repository.continue-original-raw-ignored-metadata-cell-scan"
        },
        {
          "name": "ml_check_original_link_raw_shape_loop55",
          "id": "repository.continue-original-raw-ignored-metadata-cell-scan"
        },
        {
          "name": "ml_checkOriginalLinkRawShape_loop47",
          "id": "repository.continue-original-raw-ignored-metadata-row-scan"
        },
        {
          "name": "ml_check_original_link_raw_shape_loop47",
          "id": "repository.continue-original-raw-ignored-metadata-row-scan"
        },
        {
          "name": "ml_checkOriginalLinkRawShape_loop46",
          "id": "repository.continue-original-raw-metadata-slot-scan"
        },
        {
          "name": "ml_check_original_link_raw_shape_loop46",
          "id": "repository.continue-original-raw-metadata-slot-scan"
        },
        {
          "name": "ml_checkOriginalLinkRawShape_loop45",
          "id": "repository.continue-original-raw-reference-word-scan"
        },
        {
          "name": "ml_check_original_link_raw_shape_loop45",
          "id": "repository.continue-original-raw-reference-word-scan"
        },
        {
          "name": "ml_checkOriginalLinkRawShape_loop57",
          "id": "repository.continue-original-raw-primitive-number-cell-scan"
        },
        {
          "name": "ml_check_original_link_raw_shape_loop57",
          "id": "repository.continue-original-raw-primitive-number-cell-scan"
        },
        {
          "name": "ml_checkOriginalLinkRawShape_loop44",
          "id": "repository.continue-original-raw-transport-column-scan"
        },
        {
          "name": "ml_check_original_link_raw_shape_loop44",
          "id": "repository.continue-original-raw-transport-column-scan"
        }
      ]
    }
  },
  {
    "inventory": "original raw generated result wrappers",
    "files": {
      "experiments/original-link-complete-table-projection/data/raw-native-definitions.lino": [
        {
          "name": "ml_readOriginalLinkIdentityOrdinals_loop4_result",
          "id": "repository.owned-original-identity-result-scan-control-result"
        },
        {
          "name": "MlReadOriginalLinkIdentityOrdinalsLoop4Result",
          "id": "repository.owned-original-identity-result-scan-control-result"
        },
        {
          "name": "ml_readOriginalLinkIdentityOrdinals_loop3_result",
          "id": "repository.owned-original-identity-query-scan-control-result"
        },
        {
          "name": "MlReadOriginalLinkIdentityOrdinalsLoop3Result",
          "id": "repository.owned-original-identity-query-scan-control-result"
        },
        {
          "name": "ml_equalOriginalRawUnits_loop5_result",
          "id": "repository.original-raw-unit-range-scan-control-result"
        },
        {
          "name": "MlEqualOriginalRawUnitsLoop5Result",
          "id": "repository.original-raw-unit-range-scan-control-result"
        },
        {
          "name": "ml_readOriginalRawTextCode_loop6_result",
          "id": "repository.original-raw-fixed-text-scan-control-result"
        },
        {
          "name": "MlReadOriginalRawTextCodeLoop6Result",
          "id": "repository.original-raw-fixed-text-scan-control-result"
        },
        {
          "name": "ml_readOriginalRawFieldCode_loop9_result",
          "id": "repository.original-raw-field-dictionary-scan-control-result"
        },
        {
          "name": "MlReadOriginalRawFieldCodeLoop9Result",
          "id": "repository.original-raw-field-dictionary-scan-control-result"
        },
        {
          "name": "ml_readOriginalRawFieldCode_loop7_result",
          "id": "repository.original-raw-field-alias-scan-control-result"
        },
        {
          "name": "MlReadOriginalRawFieldCodeLoop7Result",
          "id": "repository.original-raw-field-alias-scan-control-result"
        },
        {
          "name": "ml_readOriginalRawReferenceRole_loop10_result",
          "id": "repository.original-raw-reference-label-scan-control-result"
        },
        {
          "name": "MlReadOriginalRawReferenceRoleLoop10Result",
          "id": "repository.original-raw-reference-label-scan-control-result"
        },
        {
          "name": "ml_acceptOriginalRawNumberText_join13",
          "id": "repository.original-raw-exponent-stage-control-result"
        },
        {
          "name": "MlAcceptOriginalRawNumberTextJoin13",
          "id": "repository.original-raw-exponent-stage-control-result"
        },
        {
          "name": "ml_acceptOriginalRawNumberText_loop14_result",
          "id": "repository.original-raw-exponent-digit-scan-control-result"
        },
        {
          "name": "MlAcceptOriginalRawNumberTextLoop14Result",
          "id": "repository.original-raw-exponent-digit-scan-control-result"
        },
        {
          "name": "ml_acceptOriginalRawNumberText_join12",
          "id": "repository.original-raw-fraction-stage-control-result"
        },
        {
          "name": "MlAcceptOriginalRawNumberTextJoin12",
          "id": "repository.original-raw-fraction-stage-control-result"
        },
        {
          "name": "ml_acceptOriginalRawNumberText_loop15_result",
          "id": "repository.original-raw-fraction-digit-scan-control-result"
        },
        {
          "name": "MlAcceptOriginalRawNumberTextLoop15Result",
          "id": "repository.original-raw-fraction-digit-scan-control-result"
        },
        {
          "name": "ml_acceptOriginalRawNumberText_loop11_result",
          "id": "repository.original-raw-leading-digit-scan-control-result"
        },
        {
          "name": "MlAcceptOriginalRawNumberTextLoop11Result",
          "id": "repository.original-raw-leading-digit-scan-control-result"
        },
        {
          "name": "ml_checkOriginalLinkRawRoles_loop29_result",
          "id": "repository.original-raw-condition-field-scan-control-result"
        },
        {
          "name": "MlCheckOriginalLinkRawRolesLoop29Result",
          "id": "repository.original-raw-condition-field-scan-control-result"
        },
        {
          "name": "ml_checkOriginalLinkRawRoles_loop30_result",
          "id": "repository.original-raw-condition-owner-scan-control-result"
        },
        {
          "name": "MlCheckOriginalLinkRawRolesLoop30Result",
          "id": "repository.original-raw-condition-owner-scan-control-result"
        },
        {
          "name": "ml_checkOriginalLinkRawRoles_loop31_result",
          "id": "repository.original-raw-condition-member-scan-control-result"
        },
        {
          "name": "MlCheckOriginalLinkRawRolesLoop31Result",
          "id": "repository.original-raw-condition-member-scan-control-result"
        },
        {
          "name": "ml_checkOriginalLinkRawRoles_loop28_result",
          "id": "repository.original-raw-token-owner-scan-control-result"
        },
        {
          "name": "MlCheckOriginalLinkRawRolesLoop28Result",
          "id": "repository.original-raw-token-owner-scan-control-result"
        },
        {
          "name": "ml_checkOriginalLinkRawRoles_loop27_result",
          "id": "repository.original-raw-function-proof-owner-scan-control-result"
        },
        {
          "name": "MlCheckOriginalLinkRawRolesLoop27Result",
          "id": "repository.original-raw-function-proof-owner-scan-control-result"
        },
        {
          "name": "ml_checkOriginalLinkRawRoles_loop26_result",
          "id": "repository.original-raw-operation-owner-scan-control-result"
        },
        {
          "name": "MlCheckOriginalLinkRawRolesLoop26Result",
          "id": "repository.original-raw-operation-owner-scan-control-result"
        },
        {
          "name": "ml_checkOriginalLinkRawRoles_loop32_result",
          "id": "repository.original-raw-operation-part-scan-control-result"
        },
        {
          "name": "MlCheckOriginalLinkRawRolesLoop32Result",
          "id": "repository.original-raw-operation-part-scan-control-result"
        },
        {
          "name": "ml_checkOriginalLinkRawRoles_loop25_result",
          "id": "repository.original-raw-declaration-owner-scan-control-result"
        },
        {
          "name": "MlCheckOriginalLinkRawRolesLoop25Result",
          "id": "repository.original-raw-declaration-owner-scan-control-result"
        },
        {
          "name": "ml_checkOriginalLinkRawRoles_loop34_result",
          "id": "repository.original-raw-position-owner-scan-control-result"
        },
        {
          "name": "MlCheckOriginalLinkRawRolesLoop34Result",
          "id": "repository.original-raw-position-owner-scan-control-result"
        },
        {
          "name": "ml_checkOriginalLinkRawRoles_loop33_result",
          "id": "repository.original-raw-parameter-owner-scan-control-result"
        },
        {
          "name": "MlCheckOriginalLinkRawRolesLoop33Result",
          "id": "repository.original-raw-parameter-owner-scan-control-result"
        },
        {
          "name": "ml_checkOriginalLinkRawRoles_loop35_result",
          "id": "repository.original-raw-parameter-name-scan-control-result"
        },
        {
          "name": "MlCheckOriginalLinkRawRolesLoop35Result",
          "id": "repository.original-raw-parameter-name-scan-control-result"
        },
        {
          "name": "ml_checkOriginalLinkRawRoles_loop24_result",
          "id": "repository.original-raw-role-obligation-scan-control-result"
        },
        {
          "name": "MlCheckOriginalLinkRawRolesLoop24Result",
          "id": "repository.original-raw-role-obligation-scan-control-result"
        },
        {
          "name": "ml_checkOriginalLinkRawRoles_join39",
          "id": "repository.original-raw-token-form-stage-control-result"
        },
        {
          "name": "MlCheckOriginalLinkRawRolesJoin39",
          "id": "repository.original-raw-token-form-stage-control-result"
        },
        {
          "name": "ml_checkOriginalLinkRawRoles_loop40_result",
          "id": "repository.original-raw-number-token-field-scan-control-result"
        },
        {
          "name": "MlCheckOriginalLinkRawRolesLoop40Result",
          "id": "repository.original-raw-number-token-field-scan-control-result"
        },
        {
          "name": "ml_checkOriginalLinkRawRoles_join38",
          "id": "repository.original-raw-core-form-stage-control-result"
        },
        {
          "name": "MlCheckOriginalLinkRawRolesJoin38",
          "id": "repository.original-raw-core-form-stage-control-result"
        },
        {
          "name": "ml_checkOriginalLinkRawRoles_loop37_result",
          "id": "repository.original-raw-required-field-scan-control-result"
        },
        {
          "name": "MlCheckOriginalLinkRawRolesLoop37Result",
          "id": "repository.original-raw-required-field-scan-control-result"
        },
        {
          "name": "ml_checkOriginalLinkRawRoles_loop36_result",
          "id": "repository.original-raw-represented-field-scan-control-result"
        },
        {
          "name": "MlCheckOriginalLinkRawRolesLoop36Result",
          "id": "repository.original-raw-represented-field-scan-control-result"
        },
        {
          "name": "ml_checkOriginalLinkRawShape_loop50_result",
          "id": "repository.original-raw-owned-structure-scan-control-result"
        },
        {
          "name": "MlCheckOriginalLinkRawShapeLoop50Result",
          "id": "repository.original-raw-owned-structure-scan-control-result"
        },
        {
          "name": "ml_checkOriginalLinkRawShape_join51",
          "id": "repository.original-raw-container-form-stage-control-result"
        },
        {
          "name": "MlCheckOriginalLinkRawShapeJoin51",
          "id": "repository.original-raw-container-form-stage-control-result"
        },
        {
          "name": "ml_checkOriginalLinkRawShape_loop52_result",
          "id": "repository.original-raw-mapping-pair-scan-control-result"
        },
        {
          "name": "MlCheckOriginalLinkRawShapeLoop52Result",
          "id": "repository.original-raw-mapping-pair-scan-control-result"
        },
        {
          "name": "ml_checkOriginalLinkRawShape_loop53_result",
          "id": "repository.original-raw-mapping-key-scan-control-result"
        },
        {
          "name": "MlCheckOriginalLinkRawShapeLoop53Result",
          "id": "repository.original-raw-mapping-key-scan-control-result"
        },
        {
          "name": "ml_checkOriginalLinkRawShape_loop49_result",
          "id": "repository.original-raw-field-claim-scan-control-result"
        },
        {
          "name": "MlCheckOriginalLinkRawShapeLoop49Result",
          "id": "repository.original-raw-field-claim-scan-control-result"
        },
        {
          "name": "ml_checkOriginalLinkRawShape_loop48_result",
          "id": "repository.original-raw-selected-facet-scan-control-result"
        },
        {
          "name": "MlCheckOriginalLinkRawShapeLoop48Result",
          "id": "repository.original-raw-selected-facet-scan-control-result"
        },
        {
          "name": "ml_checkOriginalLinkRawShape_loop54_result",
          "id": "repository.original-raw-facet-field-scan-control-result"
        },
        {
          "name": "MlCheckOriginalLinkRawShapeLoop54Result",
          "id": "repository.original-raw-facet-field-scan-control-result"
        },
        {
          "name": "ml_checkOriginalLinkRawShape_loop47_result",
          "id": "repository.original-raw-ignored-metadata-row-scan-control-result"
        },
        {
          "name": "MlCheckOriginalLinkRawShapeLoop47Result",
          "id": "repository.original-raw-ignored-metadata-row-scan-control-result"
        },
        {
          "name": "ml_checkOriginalLinkRawShape_loop55_result",
          "id": "repository.original-raw-ignored-metadata-cell-scan-control-result"
        },
        {
          "name": "MlCheckOriginalLinkRawShapeLoop55Result",
          "id": "repository.original-raw-ignored-metadata-cell-scan-control-result"
        },
        {
          "name": "ml_checkOriginalLinkRawShape_loop46_result",
          "id": "repository.original-raw-metadata-slot-scan-control-result"
        },
        {
          "name": "MlCheckOriginalLinkRawShapeLoop46Result",
          "id": "repository.original-raw-metadata-slot-scan-control-result"
        },
        {
          "name": "ml_checkOriginalLinkRawShape_join56",
          "id": "repository.original-raw-metadata-string-stage-control-result"
        },
        {
          "name": "MlCheckOriginalLinkRawShapeJoin56",
          "id": "repository.original-raw-metadata-string-stage-control-result"
        },
        {
          "name": "ml_checkOriginalLinkRawShape_loop45_result",
          "id": "repository.original-raw-reference-word-scan-control-result"
        },
        {
          "name": "MlCheckOriginalLinkRawShapeLoop45Result",
          "id": "repository.original-raw-reference-word-scan-control-result"
        },
        {
          "name": "ml_checkOriginalLinkRawShape_loop44_result",
          "id": "repository.original-raw-transport-column-scan-control-result"
        },
        {
          "name": "MlCheckOriginalLinkRawShapeLoop44Result",
          "id": "repository.original-raw-transport-column-scan-control-result"
        },
        {
          "name": "ml_checkOriginalLinkRawShape_loop57_result",
          "id": "repository.original-raw-primitive-number-cell-scan-control-result"
        },
        {
          "name": "MlCheckOriginalLinkRawShapeLoop57Result",
          "id": "repository.original-raw-primitive-number-cell-scan-control-result"
        }
      ]
    }
  },
  {
    "inventory": "original raw generated constructors",
    "files": {
      "experiments/original-link-complete-table-projection/data/raw-native-definitions.lino": [
        {
          "name": "MlReadOriginalLinkIdentityOrdinalsLoop4Done",
          "id": "repository.owned-original-identity-result-scan-completion-constructor"
        },
        {
          "name": "MlReadOriginalLinkIdentityOrdinalsLoop4Return",
          "id": "repository.owned-original-identity-result-scan-return-constructor"
        },
        {
          "name": "MlReadOriginalLinkIdentityOrdinalsLoop3Done",
          "id": "repository.owned-original-identity-query-scan-completion-constructor"
        },
        {
          "name": "MlReadOriginalLinkIdentityOrdinalsLoop3Return",
          "id": "repository.owned-original-identity-query-scan-return-constructor"
        },
        {
          "name": "MlEqualOriginalRawUnitsLoop5Done",
          "id": "repository.original-raw-unit-range-scan-completion-constructor"
        },
        {
          "name": "MlEqualOriginalRawUnitsLoop5Return",
          "id": "repository.original-raw-unit-range-scan-return-constructor"
        },
        {
          "name": "MlReadOriginalRawTextCodeLoop6Done",
          "id": "repository.original-raw-fixed-text-scan-completion-constructor"
        },
        {
          "name": "MlReadOriginalRawTextCodeLoop6Return",
          "id": "repository.original-raw-fixed-text-scan-return-constructor"
        },
        {
          "name": "MlReadOriginalRawFieldCodeLoop9Done",
          "id": "repository.original-raw-field-dictionary-scan-completion-constructor"
        },
        {
          "name": "MlReadOriginalRawFieldCodeLoop9Return",
          "id": "repository.original-raw-field-dictionary-scan-return-constructor"
        },
        {
          "name": "MlReadOriginalRawFieldCodeLoop7Done",
          "id": "repository.original-raw-field-alias-scan-completion-constructor"
        },
        {
          "name": "MlReadOriginalRawFieldCodeLoop7Return",
          "id": "repository.original-raw-field-alias-scan-return-constructor"
        },
        {
          "name": "MlReadOriginalRawReferenceRoleLoop10Done",
          "id": "repository.original-raw-reference-label-scan-completion-constructor"
        },
        {
          "name": "MlReadOriginalRawReferenceRoleLoop10Return",
          "id": "repository.original-raw-reference-label-scan-return-constructor"
        },
        {
          "name": "MlAcceptOriginalRawNumberTextJoin13Next",
          "id": "repository.original-raw-exponent-stage-next-state-constructor"
        },
        {
          "name": "MlAcceptOriginalRawNumberTextJoin13Return",
          "id": "repository.original-raw-exponent-stage-return-constructor"
        },
        {
          "name": "MlAcceptOriginalRawNumberTextLoop14Done",
          "id": "repository.original-raw-exponent-digit-scan-completion-constructor"
        },
        {
          "name": "MlAcceptOriginalRawNumberTextJoin12Next",
          "id": "repository.original-raw-fraction-stage-next-state-constructor"
        },
        {
          "name": "MlAcceptOriginalRawNumberTextJoin12Return",
          "id": "repository.original-raw-fraction-stage-return-constructor"
        },
        {
          "name": "MlAcceptOriginalRawNumberTextLoop15Done",
          "id": "repository.original-raw-fraction-digit-scan-completion-constructor"
        },
        {
          "name": "MlAcceptOriginalRawNumberTextLoop11Done",
          "id": "repository.original-raw-leading-digit-scan-completion-constructor"
        },
        {
          "name": "MlCheckOriginalLinkRawRolesLoop29Done",
          "id": "repository.original-raw-condition-field-scan-completion-constructor"
        },
        {
          "name": "MlCheckOriginalLinkRawRolesLoop29Return",
          "id": "repository.original-raw-condition-field-scan-return-constructor"
        },
        {
          "name": "MlCheckOriginalLinkRawRolesLoop30Done",
          "id": "repository.original-raw-condition-owner-scan-completion-constructor"
        },
        {
          "name": "MlCheckOriginalLinkRawRolesLoop30Return",
          "id": "repository.original-raw-condition-owner-scan-return-constructor"
        },
        {
          "name": "MlCheckOriginalLinkRawRolesLoop31Done",
          "id": "repository.original-raw-condition-member-scan-completion-constructor"
        },
        {
          "name": "MlCheckOriginalLinkRawRolesLoop31Return",
          "id": "repository.original-raw-condition-member-scan-return-constructor"
        },
        {
          "name": "MlCheckOriginalLinkRawRolesLoop28Done",
          "id": "repository.original-raw-token-owner-scan-completion-constructor"
        },
        {
          "name": "MlCheckOriginalLinkRawRolesLoop28Return",
          "id": "repository.original-raw-token-owner-scan-return-constructor"
        },
        {
          "name": "MlCheckOriginalLinkRawRolesLoop27Done",
          "id": "repository.original-raw-function-proof-owner-scan-completion-constructor"
        },
        {
          "name": "MlCheckOriginalLinkRawRolesLoop27Return",
          "id": "repository.original-raw-function-proof-owner-scan-return-constructor"
        },
        {
          "name": "MlCheckOriginalLinkRawRolesLoop26Done",
          "id": "repository.original-raw-operation-owner-scan-completion-constructor"
        },
        {
          "name": "MlCheckOriginalLinkRawRolesLoop26Return",
          "id": "repository.original-raw-operation-owner-scan-return-constructor"
        },
        {
          "name": "MlCheckOriginalLinkRawRolesLoop32Done",
          "id": "repository.original-raw-operation-part-scan-completion-constructor"
        },
        {
          "name": "MlCheckOriginalLinkRawRolesLoop25Done",
          "id": "repository.original-raw-declaration-owner-scan-completion-constructor"
        },
        {
          "name": "MlCheckOriginalLinkRawRolesLoop25Return",
          "id": "repository.original-raw-declaration-owner-scan-return-constructor"
        },
        {
          "name": "MlCheckOriginalLinkRawRolesLoop34Done",
          "id": "repository.original-raw-position-owner-scan-completion-constructor"
        },
        {
          "name": "MlCheckOriginalLinkRawRolesLoop34Return",
          "id": "repository.original-raw-position-owner-scan-return-constructor"
        },
        {
          "name": "MlCheckOriginalLinkRawRolesLoop33Done",
          "id": "repository.original-raw-parameter-owner-scan-completion-constructor"
        },
        {
          "name": "MlCheckOriginalLinkRawRolesLoop33Return",
          "id": "repository.original-raw-parameter-owner-scan-return-constructor"
        },
        {
          "name": "MlCheckOriginalLinkRawRolesLoop35Done",
          "id": "repository.original-raw-parameter-name-scan-completion-constructor"
        },
        {
          "name": "MlCheckOriginalLinkRawRolesLoop35Return",
          "id": "repository.original-raw-parameter-name-scan-return-constructor"
        },
        {
          "name": "MlCheckOriginalLinkRawRolesLoop24Done",
          "id": "repository.original-raw-role-obligation-scan-completion-constructor"
        },
        {
          "name": "MlCheckOriginalLinkRawRolesLoop24Return",
          "id": "repository.original-raw-role-obligation-scan-return-constructor"
        },
        {
          "name": "MlCheckOriginalLinkRawRolesJoin39Next",
          "id": "repository.original-raw-token-form-stage-next-state-constructor"
        },
        {
          "name": "MlCheckOriginalLinkRawRolesJoin39Return",
          "id": "repository.original-raw-token-form-stage-return-constructor"
        },
        {
          "name": "MlCheckOriginalLinkRawRolesLoop40Done",
          "id": "repository.original-raw-number-token-field-scan-completion-constructor"
        },
        {
          "name": "MlCheckOriginalLinkRawRolesLoop40Return",
          "id": "repository.original-raw-number-token-field-scan-return-constructor"
        },
        {
          "name": "MlCheckOriginalLinkRawRolesJoin38Next",
          "id": "repository.original-raw-core-form-stage-next-state-constructor"
        },
        {
          "name": "MlCheckOriginalLinkRawRolesJoin38Return",
          "id": "repository.original-raw-core-form-stage-return-constructor"
        },
        {
          "name": "MlCheckOriginalLinkRawRolesLoop37Done",
          "id": "repository.original-raw-required-field-scan-completion-constructor"
        },
        {
          "name": "MlCheckOriginalLinkRawRolesLoop37Return",
          "id": "repository.original-raw-required-field-scan-return-constructor"
        },
        {
          "name": "MlCheckOriginalLinkRawRolesLoop36Done",
          "id": "repository.original-raw-represented-field-scan-completion-constructor"
        },
        {
          "name": "MlCheckOriginalLinkRawRolesLoop36Return",
          "id": "repository.original-raw-represented-field-scan-return-constructor"
        },
        {
          "name": "MlCheckOriginalLinkRawShapeLoop50Done",
          "id": "repository.original-raw-owned-structure-scan-completion-constructor"
        },
        {
          "name": "MlCheckOriginalLinkRawShapeLoop50Return",
          "id": "repository.original-raw-owned-structure-scan-return-constructor"
        },
        {
          "name": "MlCheckOriginalLinkRawShapeJoin51Next",
          "id": "repository.original-raw-container-form-stage-next-state-constructor"
        },
        {
          "name": "MlCheckOriginalLinkRawShapeJoin51Return",
          "id": "repository.original-raw-container-form-stage-return-constructor"
        },
        {
          "name": "MlCheckOriginalLinkRawShapeLoop52Done",
          "id": "repository.original-raw-mapping-pair-scan-completion-constructor"
        },
        {
          "name": "MlCheckOriginalLinkRawShapeLoop52Return",
          "id": "repository.original-raw-mapping-pair-scan-return-constructor"
        },
        {
          "name": "MlCheckOriginalLinkRawShapeLoop53Done",
          "id": "repository.original-raw-mapping-key-scan-completion-constructor"
        },
        {
          "name": "MlCheckOriginalLinkRawShapeLoop53Return",
          "id": "repository.original-raw-mapping-key-scan-return-constructor"
        },
        {
          "name": "MlCheckOriginalLinkRawShapeLoop49Done",
          "id": "repository.original-raw-field-claim-scan-completion-constructor"
        },
        {
          "name": "MlCheckOriginalLinkRawShapeLoop49Return",
          "id": "repository.original-raw-field-claim-scan-return-constructor"
        },
        {
          "name": "MlCheckOriginalLinkRawShapeLoop48Done",
          "id": "repository.original-raw-selected-facet-scan-completion-constructor"
        },
        {
          "name": "MlCheckOriginalLinkRawShapeLoop48Return",
          "id": "repository.original-raw-selected-facet-scan-return-constructor"
        },
        {
          "name": "MlCheckOriginalLinkRawShapeLoop54Done",
          "id": "repository.original-raw-facet-field-scan-completion-constructor"
        },
        {
          "name": "MlCheckOriginalLinkRawShapeLoop54Return",
          "id": "repository.original-raw-facet-field-scan-return-constructor"
        },
        {
          "name": "MlCheckOriginalLinkRawShapeLoop47Done",
          "id": "repository.original-raw-ignored-metadata-row-scan-completion-constructor"
        },
        {
          "name": "MlCheckOriginalLinkRawShapeLoop47Return",
          "id": "repository.original-raw-ignored-metadata-row-scan-return-constructor"
        },
        {
          "name": "MlCheckOriginalLinkRawShapeLoop55Done",
          "id": "repository.original-raw-ignored-metadata-cell-scan-completion-constructor"
        },
        {
          "name": "MlCheckOriginalLinkRawShapeLoop55Return",
          "id": "repository.original-raw-ignored-metadata-cell-scan-return-constructor"
        },
        {
          "name": "MlCheckOriginalLinkRawShapeLoop46Done",
          "id": "repository.original-raw-metadata-slot-scan-completion-constructor"
        },
        {
          "name": "MlCheckOriginalLinkRawShapeLoop46Return",
          "id": "repository.original-raw-metadata-slot-scan-return-constructor"
        },
        {
          "name": "MlCheckOriginalLinkRawShapeJoin56Next",
          "id": "repository.original-raw-metadata-string-stage-next-state-constructor"
        },
        {
          "name": "MlCheckOriginalLinkRawShapeJoin56Return",
          "id": "repository.original-raw-metadata-string-stage-return-constructor"
        },
        {
          "name": "MlCheckOriginalLinkRawShapeLoop45Done",
          "id": "repository.original-raw-reference-word-scan-completion-constructor"
        },
        {
          "name": "MlCheckOriginalLinkRawShapeLoop45Return",
          "id": "repository.original-raw-reference-word-scan-return-constructor"
        },
        {
          "name": "MlCheckOriginalLinkRawShapeLoop44Done",
          "id": "repository.original-raw-transport-column-scan-completion-constructor"
        },
        {
          "name": "MlCheckOriginalLinkRawShapeLoop44Return",
          "id": "repository.original-raw-transport-column-scan-return-constructor"
        },
        {
          "name": "MlCheckOriginalLinkRawShapeLoop57Done",
          "id": "repository.original-raw-primitive-number-cell-scan-completion-constructor"
        },
        {
          "name": "MlCheckOriginalLinkRawShapeLoop57Return",
          "id": "repository.original-raw-primitive-number-cell-scan-return-constructor"
        }
      ]
    }
  },
  {
    "inventory": "raw exact original identity and field controls",
    "files": {
      "experiments/original-link-complete-table-projection/raw-fixtures.mjs": [
        {
          "name": "label",
          "id": "repository.render-certificate-test-field-label"
        },
        {
          "name": "key",
          "id": "repository.read-exact-link-identity-key"
        },
        {
          "name": "replace",
          "id": "repository.replace-text-certificate-test-field"
        }
      ],
      "experiments/original-link-complete-table-projection/raw-corpus.mjs": [
        {
          "name": "label",
          "id": "repository.render-certificate-test-field-label"
        },
        {
          "name": "key",
          "id": "repository.read-exact-link-identity-key"
        }
      ],
      "experiments/original-link-complete-table-projection/raw-projection.test.mjs": [
        {
          "name": "label",
          "id": "repository.render-certificate-test-field-label"
        },
        {
          "name": "replace",
          "id": "repository.replace-text-certificate-test-field"
        },
        {
          "name": "children",
          "id": "repository.read-original-link-role-test-references"
        }
      ]
    }
  },
  {
    "inventory": "raw selected controls raw-fixtures.mjs",
    "files": {
      "experiments/original-link-complete-table-projection/raw-fixtures.mjs": [
        {
          "name": "field",
          "id": "repository.read-original-raw-fixture-field"
        },
        {
          "name": "body",
          "id": "repository.read-original-raw-fixture-body"
        },
        {
          "name": "marker",
          "id": "repository.read-original-raw-fixture-marker"
        },
        {
          "name": "proof",
          "id": "repository.read-original-raw-fixture-proof"
        },
        {
          "name": "scalar",
          "id": "repository.create-original-raw-fixture-scalar"
        },
        {
          "name": "functionFacts",
          "id": "repository.read-original-raw-fixture-function-facts"
        },
        {
          "name": "declaration",
          "id": "repository.find-original-raw-fixture-declaration"
        },
        {
          "name": "parameter",
          "id": "repository.read-original-raw-fixture-parameter"
        },
        {
          "name": "readEndOfFileRawString",
          "id": "repository.read-original-raw-end-of-file-string"
        },
        {
          "name": "replaceStringDefinition",
          "id": "repository.replace-original-raw-string-definition"
        },
        {
          "name": "createRawTextCertificateFixture",
          "id": "repository.create-original-raw-source-fixture"
        },
        {
          "name": "example",
          "id": "repository.create-original-raw-fixture-contract"
        },
        {
          "name": "originalLinkRawAdversarialFixtures",
          "id": "repository.read-original-raw-challenge-fixtures"
        }
      ]
    }
  },
  {
    "inventory": "raw selected controls raw-projection.test.mjs",
    "files": {
      "experiments/original-link-complete-table-projection/raw-projection.test.mjs": [
        {
          "name": "archive",
          "id": "repository.archive-original-raw-test-packet"
        },
        {
          "name": "field",
          "id": "repository.read-original-raw-test-field"
        },
        {
          "name": "programBody",
          "id": "repository.read-original-raw-test-body"
        },
        {
          "name": "selectedProof",
          "id": "repository.read-original-raw-test-proof"
        },
        {
          "name": "facts",
          "id": "repository.read-original-raw-test-function-facts"
        },
        {
          "name": "checkWhole",
          "id": "repository.check-original-raw-test-interpretation"
        }
      ]
    }
  },
  {
    "inventory": "raw test exact source digest",
    "files": {
      "experiments/original-link-complete-table-projection/raw-projection.test.mjs": [
        {
          "name": "hash",
          "id": "repository.hash-rust-text-native-oracle-content"
        }
      ]
    }
  },
  {
    "inventory": "original raw lazy certificate corpus",
    "files": {
      "experiments/original-link-complete-table-projection/raw-corpus.mjs": [
        {
          "name": "createOriginalLinkRawEditedFixture",
          "id": "repository.create-original-raw-graph-edited-fixture"
        },
        {
          "name": "createOriginalLinkRawOwnerForgery",
          "id": "repository.create-original-raw-graph-owner-forgery"
        },
        {
          "name": "originalLinkRawCertificateCorpus",
          "id": "repository.read-original-raw-graph-certificate-corpus"
        },
        {
          "name": "originalLinkRawNumericMetadataCorpus",
          "id": "repository.read-original-raw-graph-number-metadata-corpus"
        }
      ]
    }
  },
  {
    "inventory": "original raw lazy corpus controls",
    "files": {
      "experiments/original-link-complete-table-projection/raw-corpus.mjs": [
        {
          "name": "attachEditedPacket",
          "id": "repository.attach-original-raw-edited-proof"
        },
        {
          "name": "sourceFixtures",
          "id": "repository.read-original-raw-source-fixtures"
        },
        {
          "name": "field",
          "id": "repository.read-original-raw-corpus-field"
        }
      ]
    }
  },
  {
    "inventory": "original raw linked policy roles",
    "files": {
      "experiments/original-link-complete-table-projection/raw-policy.mjs": [
        {
          "name": "schema",
          "id": "repository.original-raw-graph-schema"
        },
        {
          "name": "roleNames",
          "id": "repository.original-raw-graph-role-names"
        },
        {
          "name": "roleRecipes",
          "id": "repository.original-raw-graph-role-recipes"
        },
        {
          "name": "transport",
          "id": "repository.original-raw-graph-transport"
        },
        {
          "name": "output",
          "id": "repository.original-raw-graph-output"
        },
        {
          "name": "refusalStages",
          "id": "repository.original-raw-graph-refusal-stages"
        },
        {
          "name": "observedFrontendBoundaries",
          "id": "repository.original-raw-graph-observed-front-end-boundaries"
        }
      ],
      "experiments/original-link-complete-table-projection/data/raw-policy.lino": [
        {
          "name": "schema",
          "id": "repository.original-raw-graph-schema"
        },
        {
          "name": "roleNames",
          "id": "repository.original-raw-graph-role-names"
        },
        {
          "name": "roleRecipes",
          "id": "repository.original-raw-graph-role-recipes"
        },
        {
          "name": "transport",
          "id": "repository.original-raw-graph-transport"
        },
        {
          "name": "output",
          "id": "repository.original-raw-graph-output"
        },
        {
          "name": "refusalStages",
          "id": "repository.original-raw-graph-refusal-stages"
        },
        {
          "name": "observedFrontendBoundaries",
          "id": "repository.original-raw-graph-observed-front-end-boundaries"
        }
      ]
    }
  },
  {
    "inventory": "historical cancellation evidence operations",
    "files": {
      "js/scripts/ci-cancellation-investigation.mjs": [
        {
          "name": "instant",
          "id": "repository.read-cancellation-investigation-instant"
        },
        {
          "name": "workflowSource",
          "id": "repository.decode-cancellation-investigation-workflow-source"
        },
        {
          "name": "unknown",
          "id": "repository.create-unknown-cancellation-observation"
        },
        {
          "name": "classifyWorkflowCancellation",
          "id": "repository.classify-historical-workflow-cancellation"
        },
        {
          "name": "checkWorkflowCancellationEvidence",
          "id": "repository.check-historical-cancellation-evidence"
        },
        {
          "name": "captureWorkflowCancellationEvidence",
          "id": "repository.capture-historical-cancellation-evidence"
        }
      ]
    }
  },
  {
    "inventory": "historical cancellation independent controls",
    "files": {
      "js/tests/ci-cancellation-investigation.test.js": [
        {
          "name": "file",
          "id": "repository.resolve-cancellation-investigation-project-file"
        },
        {
          "name": "facts",
          "id": "repository.copy-cancellation-investigation-facts"
        },
        {
          "name": "sourceEdit",
          "id": "repository.edit-cancellation-investigation-test-source"
        },
        {
          "name": "refused",
          "id": "repository.check-unknown-cancellation-investigation-result"
        }
      ]
    }
  },
  {
    "inventory": "historical cancellation evidence pins",
    "files": {
      "js/scripts/ci-cancellation-investigation.mjs": [
        {
          "name": "CANCELLATION_INVESTIGATION_FIXTURE",
          "id": "repository.historical-cancellation-fixture"
        },
        {
          "name": "CANCELLATION_INVESTIGATION_HEAD",
          "id": "repository.historical-cancellation-head"
        },
        {
          "name": "pinnedResponseHashes",
          "id": "repository.historical-cancellation-fixed-response-hashes"
        },
        {
          "name": "maximumResponseBytes",
          "id": "repository.historical-cancellation-maximum-response-bytes"
        },
        {
          "name": "maximumFixtureBytes",
          "id": "repository.historical-cancellation-maximum-fixture-bytes"
        }
      ]
    }
  },
  {
    "inventory": "raw and cancellation exact secure digest",
    "files": {
      "js/scripts/ci-cancellation-investigation.mjs": [
        {
          "name": "sha256",
          "id": "repository.hash-rust-text-native-oracle-content"
        }
      ],
      "js/tests/ci-cancellation-investigation.test.js": [
        {
          "name": "sha256",
          "id": "repository.hash-rust-text-native-oracle-content"
        }
      ],
      "experiments/original-link-complete-table-projection/generate-raw.mjs": [
        {
          "name": "sha256",
          "id": "repository.hash-rust-text-native-oracle-content"
        }
      ]
    }
  },
  {
    "inventory": "original raw complete source oracle",
    "files": {
      "experiments/original-link-complete-table-projection/raw-native-observations.mjs": [
        {
          "name": "ORIGINAL_LINK_RAW_MODEL_COMMIT",
          "id": "repository.original-raw-native-model-revision"
        },
        {
          "name": "requireOriginalLinkRawPlanning",
          "id": "repository.require-original-raw-native-planning-context"
        },
        {
          "name": "originalLinkRawNativeMode",
          "id": "repository.select-original-raw-native-observation"
        },
        {
          "name": "verifyOriginalLinkRawNativeModel",
          "id": "repository.verify-original-raw-native-model"
        },
        {
          "name": "planOriginalLinkRawNative",
          "id": "repository.plan-original-raw-native-observation"
        },
        {
          "name": "runOriginalLinkRawNative",
          "id": "repository.run-original-raw-native-observation"
        },
        {
          "name": "textBytes",
          "id": "repository.read-original-raw-native-source-bytes"
        },
        {
          "name": "argumentBits",
          "id": "repository.encode-original-raw-native-arguments"
        },
        {
          "name": "createCarrier",
          "id": "repository.create-original-raw-native-bit-carrier"
        },
        {
          "name": "vectorBits",
          "id": "repository.read-original-raw-native-carrier-vector"
        },
        {
          "name": "protocol",
          "id": "repository.render-original-raw-native-bit-protocol"
        },
        {
          "name": "archiveCompleteRustSource",
          "id": "repository.archive-original-raw-native-source"
        },
        {
          "name": "nativeInterface",
          "id": "repository.read-original-raw-native-interface"
        },
        {
          "name": "consumerSource",
          "id": "repository.render-original-raw-native-consumer"
        },
        {
          "name": "parseObservations",
          "id": "repository.read-original-raw-native-observations"
        },
        {
          "name": "vector",
          "id": "repository.encode-original-raw-native-carrier-vector"
        }
      ]
    }
  },
  {
    "inventory": "original raw exact oracle providers",
    "files": {
      "experiments/original-link-complete-table-projection/raw-native-observations.mjs": [
        {
          "name": "sha256",
          "id": "repository.hash-rust-text-native-oracle-content"
        },
        {
          "name": "freeze",
          "id": "repository.freeze-rust-text-native-observation-plan"
        },
        {
          "name": "numberBits",
          "id": "repository.encode-text-certificate-native-number-bits"
        },
        {
          "name": "createSourceNativeObservationRunner",
          "id": "repository.create-source-native-observation-runner"
        }
      ]
    }
  },
  {
    "inventory": "original raw injected native contracts",
    "files": {
      "experiments/original-link-complete-table-projection/raw-native-observations.test.mjs": [
        {
          "name": "fakeProcess",
          "id": "repository.create-original-raw-native-test-process"
        },
        {
          "name": "run",
          "id": "repository.run-original-raw-native-injected-contract"
        },
        {
          "name": "fails",
          "id": "repository.check-original-raw-native-test-failure"
        }
      ]
    }
  },
  {
    "inventory": "original raw native transport distinctions",
    "files": {
      "experiments/original-link-complete-table-projection/raw-native-observations.mjs": [
        {
          "name": "fixtureInventory",
          "id": "repository.original-raw-native-fixture-inventory"
        },
        {
          "name": "inventoryCounts",
          "id": "repository.original-raw-native-premise-counts"
        },
        {
          "name": "independentJavaScriptControls",
          "id": "repository.original-raw-independent-javascript-controls"
        },
        {
          "name": "premises",
          "id": "repository.original-raw-native-premises"
        },
        {
          "name": "serializationRefusals",
          "id": "repository.original-raw-native-serialization-refusals"
        },
        {
          "name": "hostChecks",
          "id": "repository.original-raw-native-host-checks"
        },
        {
          "name": "observations",
          "id": "repository.original-raw-native-observation-rows"
        },
        {
          "name": "carrierSha256",
          "id": "repository.original-raw-native-carrier-digest"
        },
        {
          "name": "sizes",
          "id": "repository.original-raw-native-source-bounds"
        }
      ]
    }
  },
  {
    "inventory": "original raw workflow conservation",
    "files": {
      "js/tests/ci-workflow-structure.test.js": [
        {
          "name": "checkOriginalLinkRawNativeJob",
          "id": "repository.check-original-raw-native-workflow-job"
        },
        {
          "name": "RAW_SOURCE_PACKAGE_CELLS",
          "id": "repository.original-raw-source-package-cells"
        },
        {
          "name": "RAW_SOURCE_PACKAGE_STEPS",
          "id": "repository.original-raw-source-package-steps"
        },
        {
          "name": "ORIGINAL_LINK_RAW_NATIVE_JOB",
          "id": "repository.original-raw-native-workflow-job"
        }
      ]
    }
  }
];
const normalizeOriginalRawCancellationName=name=>name.replace(/([a-z0-9])([A-Z])/gu,'$1-$2').replaceAll('_','-').replaceAll(' ','-').toLowerCase().replace(/^has-/u,'contain-').replace(/^children$/u,'child-references').replace(/\badversarial\b/gu,'challenge').replace(/\bfrontend\b/gu,'front-end').replace(/^pinned-/u,'fixed-').replace(/\bsha256\b/gu,'secure-hash-algorithm-two-hundred-fifty-six').replace(/\bml\b/gu,'meta-language').replace(/\b(loop|join)([0-9]+)\b/gu,(whole,kind,digits)=>{const small=['zero','one','two','three','four','five','six','seven','eight','nine','ten','eleven','twelve','thirteen','fourteen','fifteen','sixteen','seventeen','eighteen','nineteen'],tens={2:'twenty',3:'thirty',4:'forty',5:'fifty'},number=Number(digits);if(number<20)return kind+'-'+small[number];return number<=57?kind+'-'+tens[Math.floor(number/10)]+(number%10?'-'+small[number%10]:''):whole;});
const ORIGINAL_RAW_CANCELLATION_SOURCE_TEXTS=new Map();
const ORIGINAL_RAW_CANCELLATION_NAME_INVENTORIES=ORIGINAL_RAW_CANCELLATION_NAME_BINDINGS.map(({inventory,files})=>({
 inventory,files:Object.keys(files),
 extract:(raw,file)=>{let text=raw;if(file.endsWith('.lino')){const saved=ORIGINAL_RAW_CANCELLATION_SOURCE_TEXTS.get(file);if(!saved||saved.raw!==raw)ORIGINAL_RAW_CANCELLATION_SOURCE_TEXTS.set(file,{raw,text:JSON.stringify(readStructuredDataLino(raw))});text=ORIGINAL_RAW_CANCELLATION_SOURCE_TEXTS.get(file).text;}return files[file].filter(({name})=>new RegExp('\\b'+name+'\\b','u').test(text)).map(({name})=>normalizeOriginalRawCancellationName(name));},
 recordOf:name=>Object.values(files).flat().find(entry=>normalizeOriginalRawCancellationName(entry.name)===name).id,
}));
// Native repair source, representation and exact host boundary meanings.
const NATIVE_REPAIR_NAME_BINDINGS=[
  {
    "inventory": "selected ordered project fixture members",
    "files": {
      "rust/tests/unit/project_data.rs": [
        {
          "name": "project_data_member",
          "id": "repository.project-selected-project-data-member"
        }
      ]
    }
  },
  {
    "inventory": "selected ordered project fixture controls",
    "files": {
      "rust/tests/unit/project_data.rs": [
        {
          "name": "selected_owned_cases_retain_the_complete_archive_and_all_exact_utf16_inputs",
          "id": "repository.check-selected-owned-fixture-archive-preservation"
        },
        {
          "name": "member_selection_keeps_exact_keys_null_and_host_domain_refusals_distinct",
          "id": "repository.check-project-member-key-and-host-domains"
        }
      ]
    }
  },
  {
    "inventory": "native recovery and fixture repair hosts",
    "files": {
      "rust/tests/unit/issue_195_native_grammar_rows.rs": [
        {
          "name": "rebuilt_recovery",
          "id": "repository.reconstruct-recovered-grammar-source"
        }
      ],
      "rust/tests/unit/parse_scaling.rs": [
        {
          "name": "report_scaling_progress",
          "id": "repository.report-parser-scaling-progress"
        }
      ],
      "rust/tests/unit/lino_grammar.rs": [
        {
          "name": "read_lino_grammar_cases",
          "id": "repository.read-links-notation-grammar-fixtures"
        }
      ]
    }
  },
  {
    "inventory": "live native evidence log transport",
    "files": {
      "js/scripts/issue-195-evidence-command-logs.mjs": [
        {
          "name": "createEvidenceCommandLogForwarder",
          "id": "repository.create-evidence-command-log-forwarder"
        }
      ]
    }
  },
  {
    "inventory": "live native evidence line implementation",
    "files": {
      "js/scripts/issue-195-evidence-command-logs.mjs": [
        {
          "name": "append",
          "id": "repository.append-bounded-evidence-line"
        },
        {
          "name": "flush",
          "id": "repository.flush-redacted-evidence-line"
        },
        {
          "name": "consume",
          "id": "repository.consume-live-evidence-text"
        },
        {
          "name": "write",
          "id": "repository.write-live-evidence-output"
        },
        {
          "name": "end",
          "id": "repository.end-live-evidence-output"
        }
      ]
    }
  },
  {
    "inventory": "native workflow deadline conservation",
    "files": {
      "js/tests/ci-workflow-structure.test.js": [
        {
          "name": "checkRustWorkflowTimeouts",
          "id": "repository.check-rust-workflow-time-limits"
        }
      ]
    }
  },
  {
    "inventory": "default native network initialization",
    "files": {
      "rust/src/link_network.rs": [
        {
          "name": "default",
          "id": "repository.create-default-link-network"
        }
      ]
    }
  },
  {
    "inventory": "surface array item presence representation",
    "files": {
      "rust/src/translation/surface/array_item.rs": [
        {
          "name": "SArrayItem",
          "id": "repository.surface-array-item"
        },
        {
          "name": "Fields",
          "id": "repository.surface-array-item-wire-fields"
        },
        {
          "name": "spread_present",
          "id": "repository.surface-array-spread-field-presence"
        }
      ]
    }
  },
  {
    "inventory": "surface array item presence operations",
    "files": {
      "rust/src/translation/surface/array_item.rs": [
        {
          "name": "new",
          "id": "repository.create-explicit-surface-array-item"
        },
        {
          "name": "eq",
          "id": "repository.compare-surface-array-item-values"
        },
        {
          "name": "serialize",
          "id": "repository.encode-present-surface-array-item"
        },
        {
          "name": "read_present_spread",
          "id": "repository.read-present-surface-array-spread"
        },
        {
          "name": "deserialize",
          "id": "repository.decode-present-surface-array-item"
        }
      ]
    }
  },
  {
    "inventory": "surface array item presence controls",
    "files": {
      "rust/tests/unit/translation_surface_array_items.rs": [
        {
          "name": "shared_math_arguments",
          "id": "repository.read-shared-mathematical-array-arguments"
        },
        {
          "name": "omitted_and_explicit_spread_flags_keep_exact_wire_presence_and_boolean_values",
          "id": "repository.check-surface-array-spread-presence-and-values"
        },
        {
          "name": "public_boolean_edits_cannot_hide_a_true_spread_or_erase_an_explicit_false",
          "id": "repository.check-public-surface-spread-value-edits"
        },
        {
          "name": "present_null_nonboolean_duplicate_and_missing_values_keep_host_refusals",
          "id": "repository.check-surface-array-item-host-refusals"
        },
        {
          "name": "actual_shared_float_surface_arguments_round_trip_without_inventing_spread_fields",
          "id": "repository.check-shared-surface-argument-presence"
        }
      ]
    }
  },
  {
    "inventory": "nullable surface binding annotation representation",
    "files": {
      "rust/src/translation/surface/binding_type_annotation.rs": [
        {
          "name": "SurfaceBindingTypeAnnotation",
          "id": "repository.present-surface-binding-annotation-value"
        }
      ]
    }
  },
  {
    "inventory": "nullable surface binding annotation operations",
    "files": {
      "rust/src/translation/surface/binding_type_annotation.rs": [
        {
          "name": "null",
          "id": "repository.create-null-surface-binding-annotation"
        },
        {
          "name": "as_ref",
          "id": "repository.borrow-surface-binding-annotation-type"
        },
        {
          "name": "into_option",
          "id": "repository.consume-surface-binding-annotation-type"
        },
        {
          "name": "from",
          "id": "repository.create-typed-surface-binding-annotation"
        },
        {
          "name": "read_present_binding_type_annotation",
          "id": "repository.read-present-surface-binding-annotation"
        }
      ]
    }
  },
  {
    "inventory": "nullable surface binding annotation controls",
    "files": {
      "rust/tests/unit/translation_surface_binding_type_annotations.rs": [
        {
          "name": "omitted_null_and_concrete_binding_annotations_retain_exact_wire_shapes",
          "id": "repository.check-surface-binding-annotation-wire-forms"
        },
        {
          "name": "present_invalid_binding_annotations_and_duplicate_fields_still_refuse",
          "id": "repository.check-surface-binding-annotation-refusals"
        },
        {
          "name": "annotation_presence_preserves_actual_checker_semantics_and_type_refusals",
          "id": "repository.check-surface-binding-annotation-semantics"
        },
        {
          "name": "unchanged_scalar_reverse_surfaces_keep_every_annotation_after_host_round_trip",
          "id": "repository.check-reverse-scalar-surface-annotations"
        }
      ]
    }
  },
  {
    "inventory": "self translation representation authoring",
    "files": {
      "js/scripts/generate-self-translation-representation.mjs": [
        {
          "name": "originalSelector",
          "id": "repository.read-original-self-translation-selector"
        },
        {
          "name": "selfTranslationRepresentationPolicy",
          "id": "repository.create-self-translation-representation-policy"
        },
        {
          "name": "rules",
          "id": "repository.render-self-translation-representation-rules"
        },
        {
          "name": "generateSelfTranslationRepresentation",
          "id": "repository.generate-self-translation-representation"
        },
        {
          "name": "generateSelfTranslationRepresentationNative",
          "id": "repository.generate-native-self-translation-representation"
        }
      ]
    }
  },
  {
    "inventory": "self translation representation digest",
    "files": {
      "js/scripts/generate-self-translation-representation.mjs": [
        {
          "name": "digest",
          "id": "repository.hash-rust-text-native-oracle-content"
        }
      ],
      "js/tests/translation-self-representation.test.js": [
        {
          "name": "digest",
          "id": "repository.hash-rust-text-native-oracle-content"
        }
      ]
    }
  },
  {
    "inventory": "shared self translation representation rules",
    "files": {
      "js/src/self-translation-representation-rules.js": [
        {
          "name": "acceptSelfTranslationUtf16Member",
          "id": "repository.accept-self-translation-text-member"
        },
        {
          "name": "requireSelfTranslationUtf16Representation",
          "id": "repository.require-self-translation-text-representation"
        },
        {
          "name": "requireSelfTranslationUtf16Text",
          "id": "repository.require-self-translation-text-type"
        }
      ],
      "rust/src/self_translation/representation_rules.rs": [
        {
          "name": "accept_self_translation_utf16_member",
          "id": "repository.accept-self-translation-text-member"
        },
        {
          "name": "require_self_translation_utf16_representation",
          "id": "repository.require-self-translation-text-representation"
        },
        {
          "name": "require_self_translation_utf16_text",
          "id": "repository.require-self-translation-text-type"
        }
      ]
    }
  },
  {
    "inventory": "self translation representation context",
    "files": {
      "rust/src/self_translation/representation.rs": [
        {
          "name": "Context",
          "id": "repository.self-translation-representation-context"
        }
      ]
    }
  },
  {
    "inventory": "self translation representation native hosts",
    "files": {
      "rust/src/self_translation/representation.rs": [
        {
          "name": "policy",
          "id": "repository.read-native-self-translation-representation-policy"
        },
        {
          "name": "string_prelude",
          "id": "repository.read-native-self-translation-string-prelude"
        },
        {
          "name": "select",
          "id": "repository.select-native-self-translation-representation"
        },
        {
          "name": "contains_text",
          "id": "repository.contain-checked-self-translation-text-type"
        },
        {
          "name": "validate",
          "id": "repository.validate-native-self-translation-representation"
        }
      ]
    }
  },
  {
    "inventory": "self translation representation selection fields",
    "files": {
      "rust/src/self_translation.rs": [
        {
          "name": "utf16_strings",
          "id": "repository.self-translation-text-representation-selection"
        }
      ],
      "rust/src/self_translation/representation.rs": [
        {
          "name": "utf16_strings",
          "id": "repository.self-translation-text-representation-selection"
        }
      ],
      "rust/src/translation/emit_rust.rs": [
        {
          "name": "utf16_strings",
          "id": "repository.self-translation-text-representation-selection"
        }
      ],
      "js/src/self-translation.js": [
        {
          "name": "utf16Strings",
          "id": "repository.self-translation-text-representation-selection"
        }
      ]
    }
  },
  {
    "inventory": "self translation retained exact prelude",
    "files": {
      "js/src/translation/string-runtime.js": [
        {
          "name": "STRING_PRELUDE",
          "id": "repository.self-translation-string-prelude"
        }
      ],
      "js/src/data/self-translation-representation.lino": [
        {
          "name": "prelude",
          "id": "repository.self-translation-string-prelude"
        }
      ],
      "rust/src/data/self-translation-representation.lino": [
        {
          "name": "prelude",
          "id": "repository.self-translation-string-prelude"
        }
      ],
      "rust/src/translation/emit_rust.rs": [
        {
          "name": "Prelude::String",
          "id": "repository.self-translation-string-prelude-constructor"
        }
      ]
    }
  },
  {
    "inventory": "native selected representation emission",
    "files": {
      "rust/src/translation/emit_rust.rs": [
        {
          "name": "emit_rust_with_representation",
          "id": "repository.emit-rust-with-selected-representation"
        },
        {
          "name": "emit_rust_context",
          "id": "repository.emit-rust-in-representation-context"
        }
      ]
    }
  },
  {
    "inventory": "native selected representation item emission",
    "files": {
      "rust/src/self_translation/emission.rs": [
        {
          "name": "emit_item",
          "id": "repository.emit-self-translation-item"
        }
      ]
    }
  },
  {
    "inventory": "native selected representation controls",
    "files": {
      "rust/tests/unit/self_translation_scope.rs": [
        {
          "name": "selected_utf16_prelude_retains_scalar_source_and_carries_unproved_text_values",
          "id": "repository.check-selected-text-prelude-and-source-carrying"
        }
      ]
    }
  },
  {
    "inventory": "shared self translation policy roles",
    "files": {
      "js/src/data/self-translation-representation.lino": [
        {
          "name": "originalSelector",
          "id": "repository.self-translation-original-selector"
        },
        {
          "name": "memberNames",
          "id": "repository.self-translation-selected-member-names"
        },
        {
          "name": "nativeBoundary",
          "id": "repository.self-translation-native-text-boundary"
        },
        {
          "name": "preludeSha256",
          "id": "repository.self-translation-prelude-digest"
        }
      ],
      "rust/src/data/self-translation-representation.lino": [
        {
          "name": "originalSelector",
          "id": "repository.self-translation-original-selector"
        },
        {
          "name": "memberNames",
          "id": "repository.self-translation-selected-member-names"
        },
        {
          "name": "nativeBoundary",
          "id": "repository.self-translation-native-text-boundary"
        },
        {
          "name": "preludeSha256",
          "id": "repository.self-translation-prelude-digest"
        }
      ]
    }
  },
  {
    "inventory": "shared Rust cast source boundary",
    "files": {
      "js/src/translation/rust-cast-syntax-rules.js": [
        {
          "name": "acceptRustCastFollowingToken",
          "id": "repository.accept-rust-cast-following-token"
        },
        {
          "name": "readRustCastSyntaxMessage",
          "id": "repository.read-rust-cast-boundary-message"
        }
      ],
      "rust/src/translation/rust/cast_syntax_rules.rs": [
        {
          "name": "accept_rust_cast_following_token",
          "id": "repository.accept-rust-cast-following-token"
        },
        {
          "name": "read_rust_cast_syntax_message",
          "id": "repository.read-rust-cast-boundary-message"
        }
      ]
    }
  },
  {
    "inventory": "native Rust cast source boundary",
    "files": {
      "rust/src/translation/rust/expressions.rs": [
        {
          "name": "require_cast_following_token",
          "id": "repository.require-rust-cast-following-token"
        }
      ]
    }
  },
  {
    "inventory": "Rust cast source controls",
    "files": {
      "js/tests/translation-rust-cast-syntax.test.js": [
        {
          "name": "source",
          "id": "repository.create-rust-cast-comparison-source"
        }
      ]
    }
  },
  {
    "inventory": "Rust cast source digest",
    "files": {
      "js/tests/translation-rust-cast-syntax.test.js": [
        {
          "name": "sha256",
          "id": "repository.hash-rust-text-native-oracle-content"
        }
      ]
    }
  },
  {
    "inventory": "native Rust cast source controls",
    "files": {
      "rust/tests/unit/translation_rust_frontend.rs": [
        {
          "name": "cast_target_grouping_delimits_the_type_before_less_than_comparison",
          "id": "repository.check-rust-cast-type-comparison-boundary"
        }
      ]
    }
  },
  {
    "inventory": "shared proposition transport plans",
    "files": {
      "js/src/typed-program-link-rules.js": [
        {
          "name": "readProgramPropositionOperandFields",
          "id": "translation.read-program-proposition-operand-fields"
        },
        {
          "name": "readProgramPropositionBindingForm",
          "id": "translation.read-program-proposition-binding-form"
        }
      ],
      "rust/src/translation/typed_program_link_rules.rs": [
        {
          "name": "read_program_proposition_operand_fields",
          "id": "translation.read-program-proposition-operand-fields"
        },
        {
          "name": "read_program_proposition_binding_form",
          "id": "translation.read-program-proposition-binding-form"
        }
      ]
    }
  },
  {
    "inventory": "checked proposition surface projection",
    "files": {
      "js/src/typed-program-checked-projection.js": [
        {
          "name": "projectProposition",
          "id": "repository.project-checked-assertion-proposition"
        }
      ],
      "rust/src/typed_program_links/checked.rs": [
        {
          "name": "project_assertion_proposition",
          "id": "repository.project-checked-assertion-proposition"
        }
      ]
    }
  },
  {
    "inventory": "native proposition encoding normalization",
    "files": {
      "rust/src/typed_program_links/codec_encoding.rs": [
        {
          "name": "normalize_proposition",
          "id": "repository.normalize-linked-proposition-operands"
        }
      ]
    }
  },
  {
    "inventory": "shared proposition schema roles",
    "files": {
      "js/src/data/typed-program-links-schema.lino": [
        {
          "name": "propositionOperands",
          "id": "repository.program-proposition-operand-plan"
        },
        {
          "name": "propositionBindingForms",
          "id": "repository.program-proposition-binding-plan"
        }
      ],
      "rust/src/data/typed-program-links-schema.lino": [
        {
          "name": "propositionOperands",
          "id": "repository.program-proposition-operand-plan"
        },
        {
          "name": "propositionBindingForms",
          "id": "repository.program-proposition-binding-plan"
        }
      ]
    }
  },
  {
    "inventory": "proposition source generation values",
    "files": {
      "js/scripts/generate-typed-program-link-rules.mjs": [
        {
          "name": "propositionSource",
          "id": "repository.program-proposition-generated-source"
        },
        {
          "name": "propositionDefinitions",
          "id": "repository.program-proposition-generated-declarations"
        }
      ]
    }
  },
  {
    "inventory": "native proposition ownership controls",
    "files": {
      "rust/tests/unit/typed_program_links.rs": [
        {
          "name": "assertion_propositions_preserve_owned_expression_and_quantified_binding_links",
          "id": "repository.check-assertion-proposition-owned-bindings"
        }
      ]
    }
  },
  {
    "inventory": "persistent output comparison controls",
    "files": {
      "rust/src/grammar/feature_runtime/walk/tests.rs": [
        {
          "name": "token",
          "id": "repository.create-lazy-comparison-token"
        },
        {
          "name": "assert_linked",
          "id": "repository.check-persistent-child-list-retention"
        },
        {
          "name": "equal_outputs_preserve_different_lazy_partitions_and_shared_aliases",
          "id": "repository.check-lazy-output-partitions-and-aliases"
        },
        {
          "name": "different_counts_and_order_refuse_without_flattening",
          "id": "repository.check-lazy-output-count-and-order"
        },
        {
          "name": "nested_output_metadata_remains_distinct_without_materializing_children",
          "id": "repository.check-lazy-output-nested-metadata"
        }
      ]
    }
  },
  {
    "inventory": "focused Rust source diagnostic controls",
    "files": {
      "js/tests/translation-rust-frontend-diagnostics.test.js": [
        {
          "name": "surface",
          "id": "repository.read-rust-diagnostic-surface"
        },
        {
          "name": "source",
          "id": "repository.create-rust-visibility-control-source"
        }
      ]
    }
  },
  {
    "inventory": "focused native Rust source controls",
    "files": {
      "rust/tests/unit/translation_rust_frontend.rs": [
        {
          "name": "the_original_vector_rejection_source_has_its_exact_supported_array_surface",
          "id": "repository.check-original-vector-source-array-surface"
        }
      ],
      "rust/tests/unit/translation_rust_lifetimes.rs": [
        {
          "name": "ordinary_lifetime_modules_preserve_actual_declaration_and_call_ownership",
          "id": "repository.check-ordinary-lifetime-module-ownership"
        }
      ]
    }
  },
  {
    "inventory": "raw additive schema closure source operations",
    "files": {
      "experiments/original-link-complete-table-projection/raw-schema-source-closure.mjs": [
        {
          "name": "readOriginalLinkRawHistoricalSources",
          "id": "repository.read-original-raw-historical-source-bindings"
        },
        {
          "name": "compareOriginalSchema",
          "id": "repository.compare-original-raw-schema-identity"
        },
        {
          "name": "verifyOriginalLinkRawSchemaSourceClosure",
          "id": "repository.verify-original-raw-schema-source-closure"
        }
      ]
    }
  },
  {
    "inventory": "raw additive schema closure generation",
    "files": {
      "experiments/original-link-complete-table-projection/generate-raw-schema-source-closure.mjs": [
        {
          "name": "generateOriginalLinkRawSchemaSourceClosure",
          "id": "repository.generate-original-raw-schema-source-closure"
        },
        {
          "name": "gitSource",
          "id": "repository.read-committed-schema-source"
        }
      ]
    }
  },
  {
    "inventory": "raw additive schema closure digests",
    "files": {
      "experiments/original-link-complete-table-projection/raw-schema-source-closure.mjs": [
        {
          "name": "sha256",
          "id": "repository.hash-rust-text-native-oracle-content"
        }
      ],
      "experiments/original-link-complete-table-projection/generate-raw-schema-source-closure.mjs": [
        {
          "name": "sha256",
          "id": "repository.hash-rust-text-native-oracle-content"
        }
      ]
    }
  },
  {
    "inventory": "raw additive schema closure constants",
    "files": {
      "experiments/original-link-complete-table-projection/raw-schema-source-closure.mjs": [
        {
          "name": "ORIGINAL_LINK_RAW_SCHEMA_PRODUCER",
          "id": "repository.original-raw-schema-producer-identity"
        },
        {
          "name": "ORIGINAL_LINK_RAW_SCHEMA_FILE",
          "id": "repository.original-raw-schema-source-path"
        },
        {
          "name": "ORIGINAL_LINK_RAW_SCHEMA_BASELINE",
          "id": "repository.original-raw-schema-baseline-digest"
        },
        {
          "name": "ORIGINAL_LINK_RAW_SCHEMA_PROPERTIES",
          "id": "repository.original-raw-consumed-schema-properties"
        },
        {
          "name": "ORIGINAL_LINK_RAW_SCHEMA_ADDITIONS",
          "id": "repository.original-raw-unused-schema-additions"
        }
      ]
    }
  },
  {
    "inventory": "raw additive schema closure controls",
    "files": {
      "experiments/original-link-complete-table-projection/raw-native-observations.test.mjs": [
        {
          "name": "schemaClosureFixture",
          "id": "repository.create-raw-schema-closure-control"
        }
      ]
    }
  },
  {
    "inventory": "selected owned source refusal capture",
    "files": {
      "experiments/generate-rust-owned-local-source-operation-fixture.mjs": [
        {
          "name": "readSourceRefusal",
          "id": "repository.read-owned-source-operation-refusal"
        }
      ]
    }
  },
  {
    "inventory": "selected owned source refusal corpus",
    "files": {
      "experiments/generate-rust-owned-local-source-operation-fixture.mjs": [
        {
          "name": "borrowedMethodRefusals",
          "id": "repository.borrowed-owned-source-method-refusal-corpus"
        }
      ]
    }
  },
  {
    "inventory": "selected owned source refusal digests",
    "files": {
      "js/tests/translation-rust-owned-local-source-operations.test.js": [
        {
          "name": "digest",
          "id": "repository.hash-rust-text-native-oracle-content"
        }
      ]
    }
  },
  {
    "inventory": "selected owned source refusal controls",
    "files": {
      "js/tests/translation-rust-owned-local-source-operations.test.js": [
        {
          "name": "all six original source-operation cases retain their values and actual ordered graph identities",
          "id": "repository.check-original-owned-source-refusal-identities"
        },
        {
          "name": "the generated nonempty method corpus records actual complete sources and exact original diagnostics",
          "id": "repository.check-complete-owned-method-refusal-corpus"
        },
        {
          "name": "actual public source import preserves each selected method receiver refusal and its span",
          "id": "repository.check-public-owned-method-receiver-refusals"
        },
        {
          "name": "actual owned String clones and literal constructors retain the canonical copy interpretation",
          "id": "repository.check-owned-string-copy-interpretations"
        },
        {
          "name": "the unselected actual string slice method keeps its prior value projection",
          "id": "repository.check-unselected-string-slice-projection"
        }
      ]
    }
  },
  {
    "inventory": "native adapter repair workflow source gates",
    "files": {
      "js/tests/ci-workflow-structure.test.js": [
        {
          "name": "NATIVE_ADAPTER_REPAIR_PACKAGE_STEPS",
          "id": "repository.native-adapter-repair-package-steps"
        },
        {
          "name": "repairNames",
          "id": "repository.native-adapter-repair-gate-names"
        }
      ]
    }
  }
];
const normalizeNativeRepairName=name=>name.replace(/([A-Z])([A-Z][a-z])/gu,'$1-$2').replace(/([a-z0-9])([A-Z])/gu,'$1-$2').replaceAll('_','-').replaceAll(' ','-').toLowerCase().replace(/^s-array-item$/u,'surface-array-item').replace(/^eq$/u,'equal').replace(/^as-ref$/u,'as-reference').replace(/^serialize$/u,'encode').replace(/^deserialize$/u,'decode').replace(/\butf16\b/gu,'unicode-transformation-format-sixteen-bit').replace(/\bsha256\b/gu,'secure-hash-algorithm-two-hundred-fifty-six').replace(/\bnonboolean\b/gu,'non-boolean').replace(/\btimeouts\b/gu,'time-limits').replace(/\brebuilt\b/gu,'rebuild').replace(/\bomitted\b/gu,'absent').replace(/\bcannot\b/gu,'can-not').replace(/\bless-than\b/gu,'less-token').replace(/\bquantified\b/gu,'quantifier').replace(/\bchildren\b/gu,'child-references').replace(/\btheir\b/gu,'the').replace(/\bnonempty\b/gu,'non-empty');
const NATIVE_REPAIR_SOURCE_TEXTS=new Map();
const NATIVE_REPAIR_NAME_INVENTORIES=NATIVE_REPAIR_NAME_BINDINGS.map(({inventory,files})=>({
 inventory,files:Object.keys(files),
 extract:(raw,file)=>{let text=raw;if(file.endsWith('.lino')){const saved=NATIVE_REPAIR_SOURCE_TEXTS.get(file);if(!saved||saved.raw!==raw)NATIVE_REPAIR_SOURCE_TEXTS.set(file,{raw,text:JSON.stringify(readStructuredDataLino(raw))});text=NATIVE_REPAIR_SOURCE_TEXTS.get(file).text;}return files[file].filter(({name})=>text.includes(name)).map(({name})=>normalizeNativeRepairName(name));},
 recordOf:name=>Object.values(files).flat().find(entry=>normalizeNativeRepairName(entry.name)===name).id,
}));
export const NAME_INVENTORIES = Object.freeze([...BASE_NAME_INVENTORIES, ...RESOLVED_SOURCE_NAME_INVENTORIES, ...VALUE_SOURCE_NAME_INVENTORIES, ...SOURCE_PROCESSING_NAME_INVENTORIES, ...RUNTIME_SHELL_NAME_INVENTORIES, ...RUST_NUMBER_BOUNDARY_NAME_INVENTORIES, ...JAVASCRIPT_LINE_BOUNDARY_NAME_INVENTORIES, ...OWNED_TUPLE_CALL_NAME_INVENTORIES, ...RUST_LEXICAL_SOURCE_NAME_INVENTORIES, ...RUST_LITERAL_VALUE_NAME_INVENTORIES, ...RUST_TEXT_PROJECTION_NAME_INVENTORIES, ...RUST_RAW_AND_TEXT_CERTIFICATE_NAME_INVENTORIES, ...RAW_BINDING_RULE_ORACLE_NAME_INVENTORIES, ...OWNED_SOURCE_ADAPTER_NAME_INVENTORIES, ...FINAL_SOURCE_ORACLE_NAME_INVENTORIES, ...ORIGINAL_LINK_AND_COLLECTOR_NAME_INVENTORIES, ...FRESH_COLLECTOR_GLOBAL_CONTEXT_NAME_INVENTORIES, ...ORIGINAL_LINK_VALUE_TABLE_NAME_INVENTORIES, ...GENERATED_PACKAGE_SCHEDULING_NAME_INVENTORIES, ...COMPLETE_TABLE_BINARY_CANDIDATE_NAME_INVENTORIES, ...ORIGINAL_RAW_CANCELLATION_NAME_INVENTORIES, ...NATIVE_REPAIR_NAME_INVENTORIES]);

/** Every canonical name the sources define, with its inventory, file and record. */
export function extractNameInventory(root, inventories = NAME_INVENTORIES) {
  const names = [];
  for (const { inventory, files, extract, recordOf = (name) => name, words = false, occurrences = false } of inventories) {
    for (const file of files) {
      const text = inventoryText(root, file);
      for (const name of occurrences ? extract(text, file) : new Set(extract(text, file))) names.push({ inventory, file, name, record: words ? null : recordOf(name) });
    }
  }
  return names;
}

/** The definition and syntax aliases each source concept table gives its identities. */
export function extractSourceDescriptions(root, inventories = NAME_INVENTORIES) {
  const descriptions = [];
  for (const { inventory, files, describe, recordOf = (name) => name } of inventories) {
    if (!describe) continue;
    for (const file of files) {
      const text = inventoryText(root, file);
      for (const entry of describe(text, file)) descriptions.push({ inventory, file, ...entry, record: recordOf(entry.name) });
    }
  }
  return descriptions;
}

/**
 * The description problems: a record whose definition differs from the one
 * its source table gives, or that misses a syntax alias of its source.
 */
export function checkSourceDescriptions(records, descriptions) {
  const problems = [];
  const byId = new Map(records.map((record) => [record.id, record]));
  for (const { inventory, file, definition, sourceAliases, record } of descriptions) {
    const concept = byId.get(record);
    if (!concept) continue;
    if (definition !== undefined && concept.definition !== definition) {
      problems.push({ kind: 'record', id: record, message: `${record}: the definition differs from ${file} (${inventory}): "${definition}"` });
    }
    const recorded = new Set((concept.sourceAliases ?? []).map((alias) => `${alias.source}\u0000${alias.name}`));
    for (const alias of sourceAliases) {
      if (!recorded.has(`${alias.source}\u0000${alias.name}`)) {
        problems.push({ kind: 'record', id: record, message: `${record}: misses the ${alias.source} alias "${alias.name}" of ${file}` });
      }
    }
  }
  return problems;
}

/** Every former name a table keeps decodable, with the record of its current name. */
export function extractFormerNames(root, tables = FORMER_NAME_TABLES) {
  const formers = [];
  for (const { table, file, extract, recordOf = (name) => name } of tables) {
    const text = inventoryText(root, file);
    for (const [former, current] of extract(text)) formers.push({ table, file, former, current, record: recordOf(current) });
  }
  return formers;
}

/**
 * The inventory problems: an inventory or former-name table that reads no
 * names, a source name whose words are not English, a source name with no
 * record and no exemption, a former name still in use, a record no source
 * uses, and a former name a table decodes that its record does not list.
 */
export function checkInventoryCoverage(records, exemptions, names, context, formers = [], tables = FORMER_NAME_TABLES, inventories = NAME_INVENTORIES) {
  const problems = [];
  const byId = new Map(records.map((record) => [record.id, record]));
  const formerNames = new Map(records.flatMap((record) => (record.formerNames ?? []).map((former) => [former, record.id])));
  const exempt = (name) => exemptions.some(({ pattern }) => new RegExp(pattern, 'u').test(name));
  for (const { inventory, files } of inventories) {
    for (const file of files) {
      if (!names.some((entry) => entry.inventory === inventory && entry.file === file)) {
        problems.push({ kind: 'inventory', id: inventory, message: `${inventory}: ${file} defines no names` });
      }
    }
  }
  for (const { table, file } of tables) {
    if (!formers.some((entry) => entry.table === table)) {
      problems.push({ kind: 'inventory', id: table, message: `${table}: ${file} keeps no former names` });
    }
  }
  const used = new Set();
  const checkedWords = new Set();
  for (const { inventory, file, name, record } of names) {
    if (exempt(name)) continue;
    if (!checkedWords.has(name)) {
      checkedWords.add(name);
      for (const problem of nameProblems(name, context)) {
        problems.push({ ...problem, message: `${file} (${inventory}) ${problem.message}` });
      }
    }
    if (record === null) continue;
    used.add(record);
    if (byId.has(record)) continue;
    if (formerNames.has(name) || formerNames.has(record)) {
      const owner = formerNames.get(name) ?? formerNames.get(record);
      problems.push({ kind: 'inventory', id: record, message: `${file} (${inventory}) still uses ${name}, a former name of ${owner}` });
    } else {
      problems.push({ kind: 'inventory', id: record, message: `${file} (${inventory}) defines ${name} without a concept record` });
    }
  }
  for (const record of records) {
    if (!used.has(record.id)) {
      problems.push({ kind: 'inventory', id: record.id, message: `${record.id}: no inventory defines the recorded name` });
    }
  }
  for (const { table, file, former, current, record } of formers) {
    const owner = byId.get(record);
    if (!owner) {
      problems.push({ kind: 'inventory', id: record, message: `${file} (${table}) maps ${former} to ${current}, which has no concept record` });
    } else if (!(owner.formerNames ?? []).includes(former)) {
      problems.push({ kind: 'inventory', id: record, message: `${file} (${table}) decodes ${former} as ${current}, but ${record} does not list it as a former name` });
    }
  }
  return problems;
}

/**
 * The fixture problems: a fixture whose records or names the check does not
 * judge as it expects (`expected` lists the problem kinds, empty to accept).
 */
export function checkNamingFixtures(fixtures, context) {
  const problems = [];
  for (const fixture of fixtures) {
    const found = [
      ...checkConceptRecords(fixture.records ?? [], context),
      ...(fixture.names ?? []).flatMap((name) => nameProblems(name, context)),
    ];
    const kinds = [...new Set(found.map((problem) => problem.kind))].sort();
    const expected = [...new Set(fixture.expected)].sort();
    if (kinds.join(',') !== expected.join(',')) {
      problems.push({
        kind: 'fixture',
        id: fixture.name,
        message: `fixture "${fixture.name}": expected ${expected.join(', ') || 'no problem'}, found ${kinds.join(', ') || 'no problem'}${found.length ? ` (${found.map((problem) => problem.message).join('; ')})` : ''}`,
      });
    }
  }
  return problems;
}

/** The problems of the repository's canonical names, and the inventory read. */
export function checkRepositoryNames(root, wordnet, { records: overrideRecords, names: overrideNames } = {}) {
  const registers = loadNamingRegisters(root);
  const records = overrideRecords ?? registers.records;
  const { exemptions, vocabulary } = registers;
  const context = { wordnet, vocabulary };
  const names = overrideNames ?? extractNameInventory(root);
  const formers = extractFormerNames(root);
  const descriptions = extractSourceDescriptions(root);
  const problems = [
    ...checkConceptRecords(records, context),
    ...checkSourceDescriptions(records, descriptions),
    ...checkInventoryCoverage(records, exemptions, names, context, formers),
  ];
  return { problems, records, names, formers, descriptions, exemptions };
}
