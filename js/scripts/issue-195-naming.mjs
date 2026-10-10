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

const readJson = (root, file) => JSON.parse(readFileSync(path.join(root, file), 'utf8'));

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
    inventory: 'frontend decision operations',
    files: ['js/src/translation/frontend-rules.js', 'rust/src/translation/frontend_rules.rs'],
    extract: (text) => captures(text, /^(?:export function|pub(?: const)? fn) ([a-zA-Z][a-zA-Z_]+)\(/gmu)
      .filter((name) => !name.startsWith('ml_'))
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
      const text = existsSync(path.join(root, file)) ? readFileSync(path.join(root, file), 'utf8') : '';
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
      const text = existsSync(path.join(root, file)) ? readFileSync(path.join(root, file), 'utf8') : '';
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
    const text = existsSync(path.join(root, file)) ? readFileSync(path.join(root, file), 'utf8') : '';
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
