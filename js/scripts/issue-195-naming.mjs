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
export const NAME_INVENTORIES = Object.freeze([
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
    extract: (text) => captures(text, /^pub\((?:crate|super)\) fn (scalar_loop_policy|scalar_loop_snapshot_binding|project_function|project_module)\(/gmu)
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
    files: ['js/src/translation/rust-borrowed-call-interface.js', 'js/src/translation/rust-primitive-owners.js', 'js/src/translation/javascript-scoped-binding-emission.js'],
    extract: (text) => captures(text, /^export const (RUST_BORROWED_CALL_INTERFACE_POLICY|RUST_PRIMITIVE_OWNER_POLICY|JAVASCRIPT_SCOPED_BINDING_EMISSION_POLICY)\b/gmu)
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
    extract: (text) => captures(text, /^\s*(?:export function|function|pub\((?:crate|super)\) fn|fn) (acceptRustPrimitiveOwnerImportName|readRustStandardPrimitivePosition|createRustPrimitiveOwnerProof|prepareRustPrimitiveOwnerProof|checkRustPrimitiveOwnerProof|accept_import_name|read_standard_primitive_position|create_primitive_owner_proof|prepare_primitive_owner_proof|validate_primitive_owner_records|validate_source_primitive_owners|policy|bodySites|body_sites|exact|spelling|owner)\(/gmu)
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
    extract: (text) => captures(text, /^(?:export function|pub\(crate\) fn|fn) (readJavaScriptScopedBindingEmission|validate_model|validate_program|validate_origin_context|function_role|main_role|value_type|flag_form)(?:<'[^>]+>)?\(/gmu)
      .map(name => name.replace(/([a-z])([A-Z])/gu, '$1-$2').replaceAll('_', '-').toLowerCase().replace('java-script', 'javascript')),
    recordOf: (name) => `translation.${({
      'validate-model': 'validate-javascript-scoped-binding-emission-model',
      'validate-program': 'read-javascript-scoped-binding-emission',
      'validate-origin-context': 'validate-javascript-scoped-binding-origin-context',
      'function-role': 'read-javascript-function-binding-role',
      'main-role': 'check-javascript-main-binding-role',
      'value-type': 'check-javascript-scoped-binding-value-type',
      'flag-form': 'read-javascript-scoped-binding-origin-flag',
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
    files: ['js/src/typed-program-checked-projection.js', 'rust/src/translation/ir.rs'],
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
    extract: (text) => captures(text, /^pub\(super\) fn (validate_checked_program)\(/gmu).map(name => name.replaceAll('_','-')),
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
    files: ['js/src/translation/rust.js', 'rust/src/translation/rust.rs'],
    extract: (text) => captures(text, /^(?:export function|pub(?:\(crate\))? fn) (parseRust|parse_rust|parse_rust_with_array_result_model|parse_rust_with_completion_model|parse_rust_with_source_models|parse_rust_with_source_signature_model|parse_rust_with_source_profiles|parse_rust_with_scalar_loop_model|parse_rust_with_primitive_owner_model|parse_rust_with_loop_profiles)\(/gmu)
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
    extract: (text) => captures(text, /^pub fn (create_typed_program_links_with_rust_array_result_model|create_typed_program_links_with_rust_completion_model|create_typed_program_links_with_rust_source_models|create_typed_program_links_with_rust_source_signature_model|create_typed_program_links_with_rust_source_profiles|create_typed_program_links_with_rust_primitive_owner_model)\(/gmu)
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
    extract: (text) => captures(text, /^pub\((?:crate|super)\) fn (read_typed_program_binding_identities)\(/gmu)
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
      return [schema.root, ...Object.values(schema.containers), ...Object.values(schema.scalars), ...Object.values(schema.fields), ...Object.values(schema.references)].map((name) => name.toLowerCase().replaceAll(' ', '-'));
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
    files: ['js/scripts/associative-stack-sources.mjs'],
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
