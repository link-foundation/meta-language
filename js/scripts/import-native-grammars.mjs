#!/usr/bin/env node
// The automatic merge pipeline of the native grammars
// (I195-MERGE-AUTOMATIC-PIPELINE): imports each pinned upstream grammar of
// parity/grammars/sources.json as an executable native grammar, names its
// rules in readable English through parity/naming/grammar-name-expansions.json,
// gives every rule a concept record in parity/naming/canonical-concepts.json
// (shared by name across languages), and writes the merge report of each
// language. A hand edit is a reviewed decision in the expansions file, never
// an edit of the generated grammar.
//
//   node js/scripts/import-native-grammars.mjs          # write the grammars, concepts and reports
//   node js/scripts/import-native-grammars.mjs --check  # fail on drift
//
// After writing, run build-language-catalog.mjs and build-concept-records.mjs
// to refresh the shipped copies.
import { createHash } from 'node:crypto';
import { readFileSync, writeFileSync } from 'node:fs';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { gunzipSync } from 'node:zlib';

import { importTreeSitterNative, renderTreeSitterNative } from '../src/grammar-importers/tree-sitter-native.js';

const root = resolve(dirname(fileURLToPath(import.meta.url)), '../..');
export const GRAMMAR_SOURCES = 'parity/grammars/sources.json';
export const NAME_EXPANSIONS = 'parity/naming/grammar-name-expansions.json';
export const CONCEPT_REGISTER = 'parity/naming/canonical-concepts.json';
export const NATIVE_DIRECTORY = 'parity/grammars/native';
export const MERGE_REPORT_DIRECTORY = 'parity/grammars/merge-reports';
const WORD_RULE = 'word_characters';
const SHARED = 'Native grammars share this concept only where the construct means the same in each language.';
export const GENERATED = 'js/scripts/import-native-grammars.mjs generates this record from the rule names of the imported grammars.';
const ONLY = /^Only the native .+ grammar defines this construct\.$/u;
const CATALOG = 'js/src/data/language-catalog.json';

const readJson = (file) => JSON.parse(readFileSync(join(root, file), 'utf8'));

/** The bytes of a pinned source (or corpus), checked against its recorded hash. */
export function sourceText(entry) {
  const bytes = gunzipSync(readFileSync(join(root, entry.file)));
  const sha256 = createHash('sha256').update(bytes).digest('hex');
  if (sha256 !== entry.sha256) throw new Error(`${entry.file}: sha256 ${sha256}, the registry pins ${entry.sha256}`);
  return bytes.toString('utf8');
}

/** The native scanner file of a source, checked against its recorded hash, or `''`. */
export function scannerText(entry) {
  if (!entry.scanner) return '';
  const bytes = readFileSync(join(root, entry.scanner.file));
  const sha256 = createHash('sha256').update(bytes).digest('hex');
  if (sha256 !== entry.scanner.sha256) throw new Error(`${entry.scanner.file}: sha256 ${sha256}, the registry pins ${entry.scanner.sha256}`);
  return bytes.toString('utf8');
}

/** The pinned source of a native grammar id (`native-c`). */
export function grammarSourceOf(native) {
  const entry = readJson(GRAMMAR_SOURCES).sources.find((source) => source.native === native);
  if (!entry) throw new Error(`${GRAMMAR_SOURCES} pins no source of ${native}`);
  return entry;
}

/**
 * The cases of one tree-sitter corpus file, `{ title, source }` in file
 * order. A case is a `===` line, its title, its attribute lines, a closing
 * `===` line, the source, a `---` line and the expected tree. A
 * `:language(NAME)` attribute adds `language: NAME` (tree-sitter-typescript's
 * corpus runs a case on the typescript or the tsx grammar only); the other
 * attributes are kept as `attributes`. Like `tree-sitter test`, the divider is
 * the longest line of dashes before the next case, the last of equally long
 * ones, so a source may itself hold shorter `---` lines (Cargo script
 * frontmatter, YAML documents); of the line breaks that end the source, one
 * stays.
 */
export function corpusFileCases(corpus) {
  const lines = corpus.split('\n');
  // The line after the closing `===` of a header at `index`, or -1.
  const headerEnd = (index) => {
    if (!/^={3,}/u.test(lines[index] ?? '') || index + 1 >= lines.length) return -1;
    let close = index + 2;
    while (/^:/u.test(lines[close] ?? '')) close += 1;
    return /^={3,}/u.test(lines[close] ?? '') ? close + 1 : -1;
  };
  const cases = [];
  for (let index = 0; index < lines.length; index += 1) {
    const start = headerEnd(index);
    if (start < 0) continue;
    const attributes = lines.slice(index + 2, start - 1).map((line) => line.slice(1).trim());
    let next = start;
    while (next < lines.length && headerEnd(next) < 0) next += 1;
    let divider = -1;
    for (let line = start; line < next; line += 1) {
      if (/^-{3,}\s*$/u.test(lines[line]) && (divider < 0 || lines[line].length >= lines[divider].length)) divider = line;
    }
    const end = divider < 0 ? next : divider;
    const item = { title: lines[index + 1].trim(), source: lines.slice(start, end).join('\n').replace(/\n+$/u, '\n') };
    const language = attributes.map((attribute) => attribute.match(/^language\((\S+)\)$/u)?.[1]).find(Boolean);
    const others = attributes.filter((attribute) => !/^language\(/u.test(attribute));
    cases.push({ ...item, ...(language ? { language } : {}), ...(others.length > 0 ? { attributes: others } : {}) });
    index = next - 1;
  }
  return cases;
}

/**
 * The cases of the pinned upstream test corpus of a source, `{ file, title,
 * source }` in corpus order. Where the corpus serves several grammars
 * (`entry.corpus.language`), a case with a `:language` attribute counts for
 * that grammar only.
 */
export function corpusCases(entry) {
  const { files } = JSON.parse(sourceText(entry.corpus));
  const wanted = entry.corpus.language;
  return Object.entries(files).flatMap(([file, corpus]) => corpusFileCases(corpus)
    .filter((item) => item.language === undefined || item.language === wanted)
    .map(({ language: _language, ...item }) => ({ file, ...item })));
}

/**
 * The readable native name of an upstream rule name: a reviewed name
 * decision, or each word replaced by its expansion. A leading underscore,
 * tree-sitter's mark of a hidden rule, goes: the rule is silent instead, and
 * its source name keeps the upstream spelling.
 */
export function nativeName(name, expansions, decisions = {}) {
  // Own decisions only: an upstream rule may be named `constructor`.
  if (Object.hasOwn(decisions.names ?? {}, name)) return decisions.names[name];
  const words = name.replace(/^_+/u, '').replace(/([a-z0-9])([A-Z])/gu, '$1_$2').toLowerCase().split('_');
  return words.flatMap((word) => (expansions.get(word) ?? word).split('_')).join('_');
}

/** The phrase a native rule name spells: `preprocessor_else` -> `preprocessor else`. */
export const rulePhrase = (name) => name.split('_').join(' ');

const article = (phrase) => (/^[aeiou]/u.test(phrase) ? 'an' : 'a');

/** The concept identity of a native rule: a reviewed decision or `grammar.<phrase>`. */
export function ruleConcept(rule, decisions = {}) {
  const source = rule.sourceName ?? rule.name;
  const decided = Object.hasOwn(decisions.concepts ?? {}, source) ? decisions.concepts[source] : undefined;
  return decided ?? `grammar.${rulePhrase(rule.name).split(' ').join('-')}`;
}

/** Imports one source: the native grammar text, its rules with their concepts, and its report. */
export function importSource(entry, expansions, decisions = {}) {
  const grammar = JSON.parse(sourceText(entry));
  const imported = importTreeSitterNative(grammar, {
    nameOf: (name) => nativeName(name, expansions, decisions),
    wordRule: WORD_RULE,
    scanners: scannerText(entry),
    immediate: entry.scanner?.immediate ?? [],
  });
  const rules = imported.rules.map((rule) => ({ ...rule, concept: ruleConcept(rule, decisions) }));
  // A decided conflict keeps both parses where the source's precedences
  // reduce one rule before a token another rule of the group shifts (Lean's
  // `#check @foo`, see `reductionFacts` in the executor).
  const decided = (decisions.conflicts ?? []).map(({ rules: group }) => group);
  for (const group of decided) {
    for (const name of group) if (!rules.some((rule) => rule.name === name)) throw new Error(`the decided conflict ${group.join(' ')} names no rule ${name} of ${entry.language}`);
  }
  const text = renderTreeSitterNative(
    { ...imported, conflicts: [...imported.conflicts, ...decided], rules },
    {
      annotate: (rule) => [
        `(concept ${rule.concept})`,
        ...(rule.sourceName !== null && rule.sourceName !== rule.name ? [`(source-names (tree-sitter ${rule.sourceName}))`] : []),
      ],
    },
  );
  return { entry, grammar, imported, rules, text, decisions };
}

const nativeSource = (entry) => `native:${entry.language}`;

// The aliases other sources give a record first, then the native grammars' in order.
const sortAliases = (aliases) => [
  ...aliases.filter(({ source }) => !source.startsWith('native:')),
  ...aliases.filter(({ source }) => source.startsWith('native:'))
    .sort((a, b) => a.source.localeCompare(b.source) || a.name.localeCompare(b.name)),
];

/** The catalog name of a source's language (`C` for `c`). */
function languageName(entry) {
  const language = readJson(CATALOG).languages.find(({ aliases }) => aliases.includes(entry.language));
  return language?.name ?? entry.language;
}

/**
 * The concept register with the records of the imported grammars merged in:
 * every rule's alias joins the record of its concept, a concept with no record
 * gets a generated one, and generated records left without an alias go. A
 * record an imported grammar names states first how native grammars use it:
 * shared where two or more name it, else only by its one language.
 */
export function mergeConcepts(register, imports, decisions = {}) {
  const sources = new Set(imports.map(({ entry }) => nativeSource(entry)));
  const concepts = register.concepts
    .map((record) => ({ ...record, sourceAliases: record.sourceAliases.filter(({ source }) => !sources.has(source)) }))
    .filter((record) => !(record.constraints.includes(GENERATED) && record.sourceAliases.length === 0));
  const byId = new Map(concepts.map((record) => [record.id, record]));
  const former = new Map(concepts.flatMap((record) => record.formerNames.map((name) => [name, record.id])));
  for (const { entry, rules } of imports) {
    for (const rule of rules) {
      // A former name stays retired: a rule that would revive it needs a reviewed decision.
      if (former.has(rule.concept)) {
        throw new Error(`the ${entry.language} rule ${rule.sourceName ?? rule.name} names ${rule.concept}, a former name of ${former.get(rule.concept)}; decide its concept in ${NAME_EXPANSIONS}`);
      }
      const alias = { source: nativeSource(entry), name: rule.name };
      let record = byId.get(rule.concept);
      if (!record) {
        const phrase = rule.concept.split('.').at(-1).split('-').join(' ');
        const upstream = rule.sourceName ?? rule.name;
        record = {
          id: rule.concept,
          phrase,
          role: 'concept',
          definition: decisions.definitions?.[rule.concept]
            ?? `A rule for ${article(phrase)} ${phrase}, the construct the upstream grammar names ${upstream}.`,
          constraints: [SHARED, GENERATED],
          sourceAliases: [],
          formerNames: [],
        };
        const distinct = decisions.distinctFrom?.[rule.concept];
        if (distinct) record.distinctFrom = distinct;
        concepts.push(record);
        byId.set(record.id, record);
      }
      record.sourceAliases = sortAliases([...record.sourceAliases, alias]);
    }
  }
  const names = new Map(imports.map(({ entry }) => [nativeSource(entry), languageName(entry)]));
  for (const record of concepts) {
    const native = new Set(record.sourceAliases.map(({ source }) => source).filter((source) => source.startsWith('native:')));
    if (![...native].some((source) => sources.has(source))) continue;
    const usage = native.size > 1 ? SHARED : `Only the native ${names.get([...native][0])} grammar defines this construct.`;
    record.constraints = [usage, ...record.constraints.filter((text) => text !== SHARED && !ONLY.test(text))];
  }
  return { ...register, concepts };
}

/** The merge report of one imported language. */
export function mergeReport(result, register, words) {
  const { entry, imported, rules, grammar } = result;
  const generated = new Set(register.concepts.filter((record) => record.constraints.includes(GENERATED)).map(({ id }) => id));
  const used = new Set();
  for (const name of Object.keys(grammar.rules)) {
    for (const word of name.replace(/^_+/u, '').toLowerCase().split('_')) if (words.has(word)) used.add(word);
  }
  return {
    generatedBy: 'js/scripts/import-native-grammars.mjs',
    language: entry.language,
    native: entry.native,
    sources: [{
      id: entry.id, format: entry.format, package: entry.package, repository: entry.repository, revision: entry.revision,
      path: entry.path, sha256: entry.sha256, license: entry.license, ...(entry.patch ? { patch: entry.patch } : {}),
    }],
    ...(entry.scanner ? {
      scanner: {
        file: entry.scanner.file, sha256: entry.scanner.sha256, ports: entry.scanner.ports,
        scanners: imported.scanners.map((line) => line.match(/^\(scanner (\S+) \(tokens ([^)]*)\)/u)).map(([, name, tokens]) => ({ name, tokens: tokens.split(' ') })),
        immediate: entry.scanner.immediate,
      },
    } : {}),
    rules: rules.length,
    keywords: imported.keywords,
    renamed: rules.filter((rule) => rule.sourceName !== null && rule.sourceName !== rule.name)
      .map((rule) => ({ source: rule.sourceName, native: rule.name })),
    expandedWords: [...used].sort(),
    concepts: {
      shared: rules.filter((rule) => !generated.has(rule.concept)).map((rule) => ({ rule: rule.name, concept: rule.concept })),
      generated: rules.filter((rule) => generated.has(rule.concept)).length,
    },
    ...(result.decisions?.conflicts?.length ? { decidedConflicts: result.decisions.conflicts } : {}),
    approximations: imported.report.approximations,
    unsupported: imported.report.unsupported,
  };
}

const json = (value) => `${JSON.stringify(value, null, 2)}\n`;

/** Every file the pipeline produces, `[path, text]`. */
export function importNativeGrammars() {
  const registry = readJson(GRAMMAR_SOURCES);
  const naming = readJson(NAME_EXPANSIONS);
  const words = new Map(naming.words.map(({ word, replacement }) => [word, replacement]));
  const imports = registry.sources.map((entry) => importSource(entry, words, naming.grammars[entry.language]));
  const register = mergeConcepts(readJson(CONCEPT_REGISTER), imports, naming.concepts);
  return [
    ...imports.map((result) => [`${NATIVE_DIRECTORY}/${result.entry.language}.lino`, result.text]),
    ...imports.map((result) => [`${MERGE_REPORT_DIRECTORY}/${result.entry.language}.json`, json(mergeReport(result, register, words))]),
    [CONCEPT_REGISTER, json(register)],
  ];
}

function main() {
  const files = importNativeGrammars();
  if (process.argv.includes('--check')) {
    const stale = files.filter(([file, text]) => {
      try {
        return readFileSync(join(root, file), 'utf8') !== text;
      } catch {
        return true;
      }
    }).map(([file]) => file);
    if (stale.length > 0) {
      console.error(`imported native grammars are stale: ${stale.join(', ')}`);
      console.error('run: node js/scripts/import-native-grammars.mjs');
      process.exit(1);
    }
    console.log(`imported native grammars match their pinned sources (${files.length} files)`);
    return;
  }
  for (const [file, text] of files) writeFileSync(join(root, file), text);
  console.log(`wrote ${files.length} files`);
}

if (process.argv[1] === fileURLToPath(import.meta.url)) main();
