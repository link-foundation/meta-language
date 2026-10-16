#!/usr/bin/env node
// The bulk grammar pipeline (I195-GRAMMAR-BULK-PIPELINE): for every language
// of parity/language-grammar-inventory.json with a grammar, it imports the
// pinned tree-sitter grammar.json natively and the pinned antlr/grammars-v4
// grammar of parity/grammars-v4-sources.json, compiles both, parses the
// inventory's sample source with each, compares the native rows with the
// oracle rows of parity/fixtures/default-cst-expected.json, merges the
// grammars that imported, and lists every feature still missing: unsupported
// constructs, undefined rules, compile errors and rejected samples.
//
//   node js/scripts/run-grammar-bulk-pipeline.mjs --out-dir DIR [--only A,B] [--jobs N] [--timeout SECONDS] [--formal-definitions FILE]
//
// It writes DIR/grammar-bulk-matrix.lino and DIR/grammar-bulk-matrix.md. Each
// language runs in its own process with a memory limit and a timeout, and
// reports after every stage, so one slow grammar keeps the stages it finished
// and never stops the others. Registry grammar.json files need `cargo fetch`
// in rust/; upstream grammars (vendored tree-sitter parsers and grammars-v4)
// are fetched at their pinned revisions into GRAMMAR_BULK_CACHE (default: a
// directory under the system temporary directory).
import { spawn } from 'node:child_process';
import { createHash } from 'node:crypto';
import { existsSync, mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { gunzipSync } from 'node:zlib';
import { Grammar } from '../src/grammar.js';
import { renderGrammarLinks, parseGrammarLinks } from '../src/grammar-links.js';
import { createGlobalConceptSpace, globalConceptSpaceToLinks } from '../src/global-concept-space.js';
import { readExternalGrammarAsset } from './external-grammar-cache.mjs';
import { readProjectData } from './project-data.mjs';
import { readGrammarLicense } from './grammar-files.mjs';
import { readGrammarReport, writeGrammarReport } from './grammar-report-files.mjs';
import { planGrammarWorkOrder } from './grammar-work-order.mjs';
import { describeGrammarRejection } from './grammar-rejection-description.mjs';
import { readNativeGrammarProjection } from './native-grammar-projection.mjs';

const root = resolve(dirname(fileURLToPath(import.meta.url)), '../..');
const script = fileURLToPath(import.meta.url);
export const GRAMMARS_V4_SOURCES = 'parity/grammars-v4-sources.json';
export const ADDITIONAL_GRAMMAR_SOURCES = 'parity/grammar-additional-sources.json';
const readJson = (file) => readProjectData(join(root, file));
const cacheDir = process.env.GRAMMAR_BULK_CACHE ?? join(tmpdir(), 'meta-language-grammar-bulk');

/** The inventory languages with a grammar, in inventory order. */
export function bulkLanguages() {
  return readJson('parity/language-grammar-inventory.json').languages.filter(({ grammars }) => grammars?.length > 0);
}

// The text of `url`, cached by its path under the cache directory.
async function fetchCached(url) {
  const file = join(cacheDir, url.replace(/^https:\/\//u, '').replace(/[^\w./-]/gu, '_'));
  if (existsSync(file)) return readFileSync(file, 'utf8');
  const response = await fetch(url);
  if (!response.ok) throw new Error(`${url}: HTTP ${response.status}`);
  const text = await response.text();
  mkdirSync(dirname(file), { recursive: true });
  writeFileSync(file, text);
  return text;
}

/**
 * The tree-sitter grammar.json text of grammar `id` and where it came from: a
 * source pinned in parity/grammars/sources.json (with its entry, `pinned`),
 * the crate rust/Cargo.lock pins, or the upstream revision of a vendored
 * parser.
 */
export async function treeSitterGrammarJson(id) {
  const { sourceText } = await import('./import-native-grammars.mjs');
  const pinned = readJson('parity/grammars/sources.json').sources.find(({ language }) => language === id);
  if (pinned) return { origin: `${pinned.repository}@${pinned.revision}`, text: sourceText(pinned), pinned };
  const { GRAMMAR_SOURCES, cargoLockVersions, grammarSource } = await import('./build-vendored-grammars.mjs');
  const source = GRAMMAR_SOURCES[id];
  if (!source) throw new Error(`no pinned tree-sitter grammar ${id}`);
  if (source.vendored) {
    const path = [source.dir, 'src/grammar.json'].filter((part) => part && part !== '.').join('/');
    const text = await fetchCached(`https://raw.githubusercontent.com/${source.upstream}/${source.revision}/${path}`);
    return { origin: `https://github.com/${source.upstream}@${source.revision} (without ${source.patch ?? 'a local patch'})`.replace(' (without a local patch)', ''), text };
  }
  const { crateDir, version } = await grammarSource(id, await cargoLockVersions());
  return { origin: `crates.io ${source.crate} ${version}`, text: readFileSync(join(crateDir, source.dir, 'src/grammar.json'), 'utf8') };
}

/**
 * The concatenated pinned grammars-v4 text of `language` and its entry point,
 * or null. ANTLR has no start rule: grammars-v4 names the rule its tests
 * parse from in the `desc.xml` beside the grammar. Without one, the first
 * parser rule is the start rule.
 */
export async function grammarsV4Text(language) {
  const { revision, languages } = readJson(GRAMMARS_V4_SOURCES);
  const files = languages[language];
  if (!files) return null;
  const url = (file) => `https://raw.githubusercontent.com/antlr/grammars-v4/${revision}/${file}`;
  const texts = await Promise.all(files.map((file) => fetchCached(url(file))));
  const desc = await fetchCached(url(`${dirname(files[0])}/desc.xml`)).catch(() => null);
  return { files, text: texts.join('\n'), entryPoint: desc === null ? null : descEntryPoint(desc) };
}

/** The first `<entry-point>` of a grammars-v4 `desc.xml`, or null. */
export function descEntryPoint(xml) {
  return /<entry-point>\s*([A-Za-z_][A-Za-z0-9_]*)\s*<\/entry-point>/u.exec(xml)?.[1] ?? null;
}

/** `grammar` started at `name`, or `grammar` itself when it has no such rule. */
export function startingAt(grammar, name) {
  if (name === null || name === grammar.startRule()?.name || !grammar.rule(name)) return grammar;
  return new Grammar(name, grammar.rules, grammar.sourceFormat, grammar.declarations);
}

/** Verified pinned bytes of each additional official or mature grammar. */
export function additionalGrammarTexts(language) {
  return (readJson(ADDITIONAL_GRAMMAR_SOURCES).languages[language] ?? []).map((source) => {
    if (!/^[0-9a-f]{40}$/u.test(source.revision) || !source.license || !source.repository || !source.files?.length) {
      throw new Error(`${language}: additional source ${source.id} needs pinned provenance and a license`);
    }
    const verified = source.files.map((file) => {
      const stored = file.role === 'license' ? readGrammarLicense(file.file) : readExternalGrammarAsset(file.file);
      const bytes = file.file.endsWith('.gz') ? gunzipSync(stored) : stored;
      const digest = sha256(bytes);
      if (digest !== file.sha256 || bytes.length !== file.bytes) throw new Error(`${file.file}: pinned source bytes differ`);
      return { ...file, text: bytes.toString('utf8') };
    });
    const texts = verified.filter(({ role }) => role === undefined || role === 'grammar').map(({ text }) => text);
    const lexerText = verified.filter(({ role }) => role === 'lexer').map(({ text }) => text).join('\n');
    if (texts.length === 0) throw new Error(`${source.id}: source record contains no grammar files`);
    return { ...source, text: texts.join('\n'), ...(lexerText ? { lexerText } : {}) };
  });
}

/** Import additional formats through the same native intermediate representation. */
export async function importAdditionalGrammar(source) {
  const importers = await import('../src/grammar-importers.js');
  const importerName = { antlr: 'importAntlr', bnf: 'importBnf', ebnf: 'importEbnf', abnf: 'importAbnf', pest: 'importPest', lark: 'importLark', gbnf: 'importGbnf' }[source.format];
  let grammar;
  if (importerName) grammar = importers[importerName](source.text);
  else if (['yacc', 'bison', 'menhir', 'lex', 'lexer'].includes(source.format)) {
    const importer = await import('../src/grammar-importers/yacc.js');
    const method = ['lex', 'lexer'].includes(source.format) ? 'importLex' : 'importYacc';
    const lexer = source.lexerText ? importer.importLex(source.lexerText) : null;
    const tokenRules = lexer ? [...lexer.rules.values()].filter(({ kind }) => kind === 'token') : undefined;
    grammar = importer[method](source.text, {
      ...(source.format === 'menhir' ? { dialect: 'menhir' } : {}),
      ...(tokenRules ? { tokenRules } : {}),
      ...(lexer ? { extras: lexer.declarations.extras, modes: lexer.declarations.modes } : {}),
      ...source.options,
    });
  } else throw new Error(`unsupported additional grammar format ${source.format}`);
  if (source.entryPoint && !grammar.rule(source.entryPoint)) throw new Error(`${source.id}: no entry point ${source.entryPoint}`);
  return startingAt(grammar, source.entryPoint ?? null);
}

const message = (error) => String(error?.message ?? error).replaceAll('\r', '\\r').replaceAll('\n', '\\n').slice(0, 300);
const elapsed = (start) => Math.round(performance.now() - start);
const sha256 = (text) => createHash('sha256').update(text).digest('hex');
const bulkSourceId = (language, id) => `${language.replace(/[^A-Za-z0-9_-]/gu, '_')}--${id}`;

/** Rebuild the aggregate global space with only caller-supplied premises. */
export function createPipelineGlobalConceptSpace(sources, options = {}) {
  return createGlobalConceptSpace(sources.map(({ grammarLinks, ...source }) => ({
    ...source, grammar: parseGrammarLinks(grammarLinks),
  })), options);
}

// A rejection as `syntax at 3:1, expected "\n" or "\r"`.
function rejectionText(rejection) {
  return describeGrammarRejection(rejection);
}

// Compiles `grammar` and parses `source`, filling `row`.
function compileAndParse(row, grammar, source, compileGrammar) {
  let start = performance.now();
  const compiled = compileGrammar(grammar);
  row.compiled = true;
  row.compileMs = elapsed(start);
  start = performance.now();
  const result = compiled.parseTree(source);
  row.parseMs = elapsed(start);
  row.sample = result.tree ? 'accepted' : 'rejected';
  if (!result.tree) row.rejection = rejectionText(result.rejection);
  return result;
}

// The native import of a source pinned in parity/grammars/sources.json, as
// js/scripts/import-native-grammars.mjs makes it, and the row projection of
// its native grammar fixture.
async function importPinned(pinned) {
  const { NAME_EXPANSIONS, importSource } = await import('./import-native-grammars.mjs');
  const naming = readJson(NAME_EXPANSIONS);
  const words = new Map(naming.words.map(({ word, replacement }) => [word, replacement]));
  const { imported, text } = importSource(pinned, words, naming.grammars[pinned.language]);
  // Fixture ids select their matching native grammar, independently of a
  // source record's native field, just as the former fixture lookup did.
  const { hidden, anonymous, extras, oracleKinds } = readNativeGrammarProjection(
    `native-${pinned.language}`, readJson('parity/language-grammar-inventory.json'),
  );
  return { imported, listing: text, projection: { hidden, anonymous, extras, oracleKinds } };
}

/** Runs every stage of one language, calling `report(row)` after each. */
export async function bulkLanguageRow(entry, report = () => {}, options = {}) {
  const { compileGrammar, importAntlr, mergeGrammars, parseGrammarLinks, sharedRuleDecisions } = await import('../src/index.js');
  const { importTreeSitterNative, renderTreeSitterNative } = await import('../src/grammar-importers/tree-sitter-native.js');
  const { nativeRows } = await import('./native-grammar-rows.mjs');
  const oracle = readJson('parity/fixtures/default-cst-expected.json').languages[entry.name]?.positive ?? null;
  const row = { language: entry.name, grammars: entry.grammars, treeSitter: { stage: 'source' }, grammarsV4: null, additionalSources: [], merge: null };
  const grammars = [];
  // Preserve the original first-source row and merge identity. Other catalog
  // occurrences are audited independently; block/inline or dialect components
  // are not interchangeable grammar roots to merge blindly.
  row.treeSitterComponents = entry.grammars.map((id, position) => ({
    id, position, includedInPrimaryMerge: position === 0,
    sampleSha256: sha256(entry.source), sampleBytes: Buffer.byteLength(entry.source),
    sampleRole: position === 0 ? 'catalog-language-sample' : 'catalog-document-probe-without-composition',
    outcome: position === 0 ? row.treeSitter : { stage: 'pending' },
  }));
  row.componentConceptSources = [];
  const componentDefinitions = new Map();
  const sourceOf = options.loadTreeSitterSource ?? treeSitterGrammarJson;
  for (const component of row.treeSitterComponents) {
    const tree = component.outcome;
    tree.stage = 'source';
    report(row);
    try {
      const { origin, text, pinned } = await sourceOf(component.id, component.position, entry);
      tree.origin = origin;
      tree.sha256 = sha256(text);
      const definitionKey = JSON.stringify([component.id, tree.origin, tree.sha256]);
      component.sourceOrigin = componentDefinitions.get(definitionKey)
        ?? bulkSourceId(entry.name, component.position === 0 ? 'tree-sitter' : `tree-sitter-component-${component.position}-${component.id}`);
      tree.stage = 'import';
      const json = JSON.parse(text);
      // A pinned source is imported as the shipped native grammar is, with its
      // reviewed names and native scanner, and projected as its fixture is.
      const { imported, listing, projection } = pinned ? await importPinned(pinned) : (() => {
        const result = importTreeSitterNative(json, { wordRule: 'word_characters' });
        const extras = (json.extras ?? []).filter(({ type }) => type === 'SYMBOL').map(({ name }) => name);
        return { imported: result, listing: renderTreeSitterNative(result), projection: { extras, anonymous: ['unnamed_token'] } };
      })();
      tree.rules = imported.rules.length;
      tree.approximations = imported.report.approximations.length;
      tree.unsupported = imported.report.unsupported.map((item) => (typeof item === 'string' ? item : JSON.stringify(item)));
      tree.stage = 'compile';
      report(row);
      const grammar = parseGrammarLinks(listing);
      if (component.position === 0) grammars.push({ id: 'tree-sitter', grammar });
      else {
        component.grammarLinks = renderGrammarLinks(grammar);
        if (!componentDefinitions.has(definitionKey)) row.componentConceptSources.push({
          id: component.sourceOrigin,
          language: entry.name, grammarLinks: component.grammarLinks,
        });
      }
      componentDefinitions.set(definitionKey, component.sourceOrigin);
      const result = compileAndParse(tree, grammar, entry.source, compileGrammar);
      tree.stage = 'done';
      if (component.position === 0 && result.tree && oracle) tree.rowsMatch = JSON.stringify(nativeRows(result.tree, entry.source, projection)) === JSON.stringify(oracle);
    } catch (error) {
      tree.error = message(error);
    }
    report(row);
  }

  try {
    const v4 = await (options.loadGrammarsV4 ?? grammarsV4Text)(entry.name);
    if (v4) {
      row.grammarsV4 = { files: v4.files, stage: 'import' };
      row.grammarsV4.sha256 = sha256(v4.text);
      report(row);
      const grammar = startingAt(importAntlr(v4.text), v4.entryPoint);
      row.grammarsV4.start = grammar.startRule()?.name;
      row.grammarsV4.rules = grammar.rules.size;
      row.grammarsV4.undefinedRules = grammar.undefinedNonterminals();
      row.grammarsV4.stage = 'compile';
      report(row);
      grammars.push({ id: 'grammars-v4', grammar });
      compileAndParse(row.grammarsV4, grammar, entry.source, compileGrammar);
      row.grammarsV4.stage = 'done';
    }
  } catch (error) {
    row.grammarsV4 ??= { stage: 'source' };
    row.grammarsV4.error = message(error);
  }
  report(row);

  try {
    for (const source of (options.loadAdditionalSources ?? additionalGrammarTexts)(entry.name)) {
      const stage = { id: source.id, format: source.format, origin: `${source.repository}@${source.revision}`, sha256: sha256(source.text), files: source.files, stage: 'import' };
      row.additionalSources.push(stage);
      report(row);
      try {
        const grammar = await importAdditionalGrammar(source);
        stage.rules = grammar.rules.size;
        stage.undefinedRules = grammar.undefinedNonterminals();
        stage.stage = 'compile';
        grammars.push({ id: source.id, grammar });
        report(row);
        compileAndParse(stage, grammar, entry.source, compileGrammar);
        stage.stage = 'done';
      } catch (error) { stage.error = message(error); }
      report(row);
    }
  } catch (error) {
    row.additionalSources.push({ id: 'source-register', stage: 'source', error: message(error) });
    report(row);
  }

  if (grammars.length > 0) {
    try {
      row.merge = { sources: grammars.map(({ id }) => id), stage: 'merge' };
      report(row);
      const start = performance.now();
      const formalDefinitions = options.formalDefinitions === undefined ? undefined : { ...options.formalDefinitions };
      if (formalDefinitions) for (const { id, grammar } of grammars) for (const rule of grammar.rules.values()) {
        const alias = `${bulkSourceId(entry.name, id)}:${rule.name}`;
        if (Object.hasOwn(formalDefinitions, alias)) formalDefinitions[`${id}:${rule.name}`] = formalDefinitions[alias];
      }
      const merged = mergeGrammars(
        grammars.map(({ id, grammar }, precedence) => ({ id, language: entry.name, precedence, grammar })),
        { reconcile: true, formalDefinitions },
      );
      const shared = merged.groups.flatMap(sharedRuleDecisions);
      row.merge = {
        ...row.merge,
        stage: 'done',
        status: merged.status,
        rules: merged.groups.reduce((sum, group) => sum + group.grammar.rules.size, 0),
        sharedRules: shared.length,
        // The shared rules by how they were matched: proven equivalent,
        // corresponding up to lexical roles, or corresponding by name.
        sharedBy: Object.fromEntries(['merged', 'structural', 'name'].map((basis) => [basis, shared.filter(({ kind, basis: how }) =>
          (basis === 'merged' ? kind === 'merged' : kind === 'reconciled' && how.startsWith(basis === 'name' ? 'name' : 'structural'))).length])),
        mergeMs: elapsed(start),
        fingerprints: merged.groups.map(({ key, fingerprint }) => ({ key, fingerprint })),
        decisions: merged.groups.flatMap(({ decisions }) => decisions),
        sourceProductions: merged.groups.flatMap(({ sourceProductions }) => sourceProductions),
      };
      row.nativeGrammar = renderGrammarLinks(merged.groups[0].grammar);
      row.conceptSources = grammars.map(({ id, grammar }) => ({
        id: bulkSourceId(entry.name, id),
        language: entry.name,
        grammarLinks: renderGrammarLinks(grammar),
      }));
      row.globalConcepts = {
        copiedOrigins: Object.keys(merged.globalConceptSpace.mappings).length,
        concepts: merged.globalConceptSpace.concepts.length,
        proofs: merged.globalConceptSpace.proofs.length,
        formalOrigins: Object.keys(merged.globalConceptSpace.formalDefinitions).length,
        failures: merged.globalConceptSpace.failures,
        ambiguities: merged.globalConceptSpace.ambiguities,
      };
      row.merge.stage = 'compile';
      report(row);
      compileAndParse(row.merge, merged.groups[0].grammar, entry.source, compileGrammar);
      row.merge.stage = 'done';
    } catch (error) {
      row.merge.error = message(error);
    }
  }
  report(row);
  return row;
}

/** Explain each changed shared correspondence against an earlier CI matrix. */
export function compareMergeDecisionReports(previous, current) {
  const earlier = new Map((previous?.languages ?? []).map((row) => [row.language, row]));
  const shared = (row) => new Map((row.merge?.decisions ?? [])
    .filter(({ kind }) => kind === 'merged' || kind === 'reconciled')
    .filter(({ members }) => new Set(members.map((alias) => alias.slice(0, alias.indexOf(':')))).size > 1)
    .map((decision) => [JSON.stringify([...decision.members].sort()), decision]));
  return current.languages.map((row) => {
    const prior = earlier.get(row.language);
    const old = shared(prior ?? {});
    const now = shared(row);
    const changed = [];
    for (const [key, decision] of old) {
      const next = now.get(key);
      if (!next) {
        const currentMembers = decision.members.map((alias) => ({
          alias,
          decisions: (row.merge?.decisions ?? []).filter((candidate) => ['merged', 'reconciled', 'kept-unique'].includes(candidate.kind) && candidate.members.includes(alias)),
        }));
        const meanings = new Set(currentMembers.flatMap(({ decisions }) => decisions.map(({ concept }) => concept).filter((concept) => concept !== undefined)));
        const reason = currentMembers.some(({ decisions }) => decisions.length === 0)
          ? 'a source rule is absent from the current import; compare its source hashes and import diagnostics'
          : meanings.size > 1 ? 'explicit semantic concepts now distinguish the source productions'
            : 'the source productions are now kept in distinct correspondence classes; the current definitions and decisions are retained';
        changed.push({ kind: 'removed-correspondence', members: decision.members, previous: decision, currentMembers, reason });
      }
      else if (decision.kind !== next.kind || decision.basis !== next.basis || decision.definition !== next.definition) {
        changed.push({ kind: 'changed-correspondence', members: decision.members, previous: decision, current: next, reason: 'the current proof basis or preserved production union changed' });
      }
    }
    for (const [key, decision] of now) if (!old.has(key)) changed.push({ kind: 'added-correspondence', members: decision.members, current: decision });
    return {
      language: row.language,
      baselineAvailable: Array.isArray(prior?.merge?.decisions),
      previousSharedRules: prior?.merge?.sharedRules ?? null,
      currentSharedRules: row.merge?.sharedRules ?? null,
      delta: prior?.merge?.sharedRules === undefined || row.merge?.sharedRules === undefined ? null : row.merge.sharedRules - prior.merge.sharedRules,
      sourceHashes: { previous: [prior?.treeSitter?.sha256 ?? null, prior?.grammarsV4?.sha256 ?? null], current: [row.treeSitter?.sha256 ?? null, row.grammarsV4?.sha256 ?? null] },
      changes: changed,
    };
  });
}

/** The features a row shows missing, as `{ stage, feature }`. */
export function missingFeatures(row) {
  const missing = [];
  const add = (stage, feature) => missing.push({ stage, feature });
  const { treeSitter: tree, grammarsV4: v4, merge } = row;
  for (const item of tree.unsupported ?? []) add('tree-sitter import', item);
  if (tree.error) add(`tree-sitter ${tree.stage}`, tree.error);
  if (tree.timedOut) add(`tree-sitter ${tree.stage}`, 'timed out');
  if (tree.sample === 'rejected') add('tree-sitter sample', tree.rejection);
  if (tree.sample === 'accepted' && tree.rowsMatch === false) add('tree-sitter rows', 'the native rows differ from the oracle rows');
  for (const component of row.treeSitterComponents ?? []) {
    if (component.position === 0) continue;
    const outcome = component.outcome;
    const stage = `tree-sitter component ${component.position}:${component.id}`;
    for (const item of outcome.unsupported ?? []) add(`${stage} import`, item);
    if (outcome.error) add(`${stage} ${outcome.stage}`, outcome.error);
    if (outcome.timedOut) add(`${stage} ${outcome.stage}`, 'timed out');
    if (outcome.stage === 'pending') add(`${stage} source`, 'component audit did not start');
    if (outcome.sample === 'rejected') add(`${stage} sample`, outcome.rejection);
  }
  if (v4) {
    if (v4.undefinedRules?.length) add('grammars-v4 import', `undefined rules: ${v4.undefinedRules.join(' ')}`);
    if (v4.error) add(`grammars-v4 ${v4.stage}`, v4.error);
    if (v4.timedOut) add(`grammars-v4 ${v4.stage}`, 'timed out');
    if (v4.sample === 'rejected') add('grammars-v4 sample', v4.rejection);
  }
  for (const source of row.additionalSources ?? []) {
    const stage = `additional ${source.id}`;
    if (source.undefinedRules?.length) add(`${stage} import`, `undefined rules: ${source.undefinedRules.join(' ')}`);
    if (source.error) add(`${stage} ${source.stage}`, source.error);
    if (source.timedOut) add(`${stage} ${source.stage}`, 'timed out');
    if (source.sample === 'rejected') add(`${stage} sample`, source.rejection);
  }
  if (merge?.error) add('merge', merge.error);
  if (merge?.sample === 'rejected' && !(merge.sources?.length === 1 && tree.sample === 'rejected' && merge.rejection === tree.rejection)) {
    add('merge sample', merge.rejection);
  }
  // A merge of two or more sources that shares no rule only concatenates them
  // (I195-MERGE-REAL-RECONCILIATION).
  if (merge?.stage === 'done' && merge.sources.length > 1 && merge.sharedRules === 0) {
    add('merge', `the merge only concatenates its sources, sharing no rule (${merge.sources.join(', ')}: ${merge.rules} rules)`);
  }
  if (merge?.timedOut) add('merge', 'timed out');
  return missing;
}

// Runs one language in a child process; resolves with its last reported row.
function runWorker(entry, { timeout, memory, formalDefinitionsFile }) {
  return new Promise((resolveRow) => {
    const child = spawn(process.execPath, [`--max-old-space-size=${memory}`, script, '--worker', entry.name,
      ...(formalDefinitionsFile ? ['--formal-definitions', formalDefinitionsFile] : [])], { stdio: ['ignore', 'pipe', 'pipe'] });
    let last = { language: entry.name, grammars: entry.grammars, treeSitter: { stage: 'start' }, grammarsV4: null, merge: null };
    let buffer = '';
    let stderr = '';
    let timedOut = false;
    child.stdout.on('data', (chunk) => {
      buffer += chunk;
      const lines = buffer.split('\n');
      buffer = lines.pop();
      for (const line of lines) if (line.startsWith('{')) last = JSON.parse(line);
    });
    child.stderr.on('data', (chunk) => { stderr = (stderr + chunk).slice(-2000); });
    const timer = setTimeout(() => { timedOut = true; child.kill('SIGKILL'); }, timeout * 1000);
    child.on('close', (code) => {
      clearTimeout(timer);
      // The stage still running when the process ended is the one that failed.
      const running = [last.merge, ...(last.additionalSources ?? []).slice().reverse(), last.grammarsV4,
        ...(last.treeSitterComponents ?? []).slice(1).reverse().map(({ outcome }) => outcome), last.treeSitter]
        .find((stage) => stage && stage.stage !== 'done' && stage.stage !== 'pending' && !stage.error);
      if (running && (timedOut || code !== 0)) {
        if (timedOut) running.timedOut = true;
        else running.error = message(stderr.trim().split('\n').filter(Boolean).pop() ?? `exit ${code}`);
      }
      resolveRow(last);
    });
  });
}

const yes = (value) => (value === undefined ? '' : value ? 'yes' : 'no');

function stageCells(stage) {
  if (!stage) return ['—', '', '', ''];
  const failed = stage.timedOut ? `timed out in ${stage.stage}` : stage.error ? `fails in ${stage.stage}` : '';
  return [String(stage.rules ?? ''), failed || yes(stage.compiled), stage.sample ?? '', stage.parseMs === undefined ? '' : String(stage.parseMs)];
}

/** The Markdown matrix of `matrix`. */
export function renderMatrix(matrix) {
  const lines = [
    '# Bulk grammar pipeline matrix',
    '',
    `Generated by \`js/scripts/run-grammar-bulk-pipeline.mjs\` for ${matrix.languages.length} inventory languages with a grammar; grammars-v4 at \`${matrix.grammarsV4Revision}\`.`,
    '',
    `Summary: ${matrix.summary.treeSitterImported} tree-sitter grammars import natively, ${matrix.summary.treeSitterCompiled} compile, ${matrix.summary.treeSitterAccepted} accept the sample and ${matrix.summary.rowsMatch} give the oracle rows; ${matrix.summary.grammarsV4Imported} of ${matrix.summary.grammarsV4Mapped} grammars-v4 grammars import, ${matrix.summary.grammarsV4Compiled} compile and ${matrix.summary.grammarsV4Accepted} accept the sample; ${matrix.summary.merged} languages merge, and ${matrix.summary.sharingRules} of the ${matrix.summary.mergedFromSeveral} merged from two or more sources share rules (${matrix.summary.sharedRules} shared rules in all).`,
    '',
    '| Language | tree-sitter rules | unsupported | compiles | sample | ms | oracle rows | grammars-v4 rules | compiles | sample | ms | merged rules | shared | missing features |',
    '| --- | ---: | ---: | --- | --- | ---: | --- | ---: | --- | --- | ---: | ---: | ---: | ---: |',
  ];
  for (const row of matrix.languages) {
    const tree = stageCells(row.treeSitter);
    const v4 = stageCells(row.grammarsV4);
    const merge = row.merge?.stage === 'done' ? [String(row.merge.rules), String(row.merge.sharedRules)] : [row.merge ? `fails in ${row.merge.stage}` : '', ''];
    lines.push(`| ${row.language} | ${tree[0]} | ${row.treeSitter.unsupported?.length ?? ''} | ${tree[1]} | ${tree[2]} | ${tree[3]} | ${yes(row.treeSitter.rowsMatch)} | ${v4.join(' | ')} | ${merge.join(' | ')} | ${row.missing.length} |`);
  }
  const components = matrix.languages.flatMap(row => (row.treeSitterComponents ?? []).map(component => ({ language: row.language, ...component })));
  if (components.length > 0) {
    lines.push('', '## Catalog grammar component audits', '',
      'Each catalog occurrence is audited separately. Additional component probes do not establish composed-language support.', '',
      '| Language | Position | Grammar | Source SHA-256 | Rules | Unsupported | Compiles | Sample |',
      '| --- | ---: | --- | --- | ---: | ---: | --- | --- |');
    for (const component of components) {
      const cells = stageCells(component.outcome);
      lines.push(`| ${component.language} | ${component.position} | ${component.id} | ${component.outcome.sha256 ?? ''} | ${cells[0]} | ${component.outcome.unsupported?.length ?? ''} | ${cells[1]} | ${cells[2]} |`);
    }
  }
  lines.push('', '## Missing features', '');
  for (const row of matrix.languages) {
    if (row.missing.length === 0) continue;
    lines.push(`### ${row.language}`, '');
    for (const { stage, feature } of row.missing) lines.push(`- ${stage}: ${feature.replace(/\|/gu, '\\|')}`);
    lines.push('');
  }
  const notes = Object.entries(matrix.grammarsV4Notes);
  if (notes.length > 0) {
    lines.push('## grammars-v4 notes', '');
    for (const [language, note] of notes) lines.push(`- ${language}: ${note}`);
    lines.push('');
  }
  return `${lines.join('\n')}\n`;
}

function summarize(rows, mapped) {
  const count = (predicate) => rows.filter(predicate).length;
  return {
    languages: rows.length,
    treeSitterImported: count(({ treeSitter }) => treeSitter.rules !== undefined),
    treeSitterCompiled: count(({ treeSitter }) => treeSitter.compiled),
    treeSitterAccepted: count(({ treeSitter }) => treeSitter.sample === 'accepted'),
    rowsMatch: count(({ treeSitter }) => treeSitter.rowsMatch === true),
    grammarsV4Mapped: mapped,
    grammarsV4Imported: count(({ grammarsV4 }) => grammarsV4?.rules !== undefined),
    grammarsV4Compiled: count(({ grammarsV4 }) => grammarsV4?.compiled),
    grammarsV4Accepted: count(({ grammarsV4 }) => grammarsV4?.sample === 'accepted'),
    merged: count(({ merge }) => merge?.stage === 'done'),
    mergedFromSeveral: count(({ merge }) => merge?.stage === 'done' && merge.sources.length > 1),
    sharingRules: count(({ merge }) => merge?.stage === 'done' && merge.sources.length > 1 && merge.sharedRules > 0),
    sharedRules: rows.reduce((sum, { merge }) => sum + (merge?.stage === 'done' ? merge.sharedRules : 0), 0),
    missingFeatures: rows.reduce((sum, row) => sum + row.missing.length, 0),
  };
}

/** Runs the pipeline over `only` (default: every language) and writes the matrix. */
export async function runBulkPipeline({ outDir, only = null, jobs = 2, timeout = 300, memory = 3072, previousMatrix = null, formalDefinitions }) {
  if (formalDefinitions !== undefined && (formalDefinitions === null || typeof formalDefinitions !== 'object' || Array.isArray(formalDefinitions))) {
    throw new TypeError('formal definitions must map qualified origins or concept identities to explicit Links archives');
  }
  const languages = bulkLanguages().filter(({ name }) => !only || only.includes(name));
  if (only) for (const name of only) if (!languages.some((entry) => entry.name === name)) throw new Error(`${name} is not an inventory language with a grammar`);
  const rows = new Array(languages.length);
  const executionOrder = planGrammarWorkOrder(languages);
  let formalDefinitionsFile;
  if (formalDefinitions !== undefined) {
    mkdirSync(outDir, { recursive: true });
    formalDefinitionsFile = join(outDir, 'formal-concept-definitions.input.lino');
    writeGrammarReport(formalDefinitionsFile, formalDefinitions);
  }
  let next = 0;
  const worker = async () => {
    while (next < executionOrder.length) {
      const index = executionOrder[next++];
      const row = await runWorker(languages[index], { timeout, memory, formalDefinitionsFile });
      row.missing = missingFeatures(row);
      rows[index] = row;
      process.stderr.write(`${row.language}: ${row.missing.length} missing features\n`);
    }
  };
  await Promise.all(Array.from({ length: Math.max(1, jobs) }, worker));
  const v4 = readJson(GRAMMARS_V4_SOURCES);
  const mapped = languages.filter(({ name }) => v4.languages[name]).length;
  const matrix = { grammarsV4Revision: v4.revision, summary: summarize(rows, mapped), grammarsV4Notes: v4.notes, languages: rows };
  mkdirSync(outDir, { recursive: true });
  const decisionChanges = compareMergeDecisionReports(previousMatrix, matrix);
  writeGrammarReport(join(outDir, 'grammar-merge-decision-changes.lino'), decisionChanges);
  const importedSources = rows.flatMap((row) => [...(row.conceptSources ?? []), ...(row.componentConceptSources ?? [])]);
  mkdirSync(join(outDir, 'native-grammars'), { recursive: true });
  mkdirSync(join(outDir, 'native-grammar-components'), { recursive: true });
  for (const row of rows) {
    if (row.nativeGrammar) writeFileSync(join(outDir, 'native-grammars', `${row.language.replace(/[^A-Za-z0-9_-]/gu, '_')}.lino`), row.nativeGrammar);
    delete row.nativeGrammar;
    delete row.conceptSources;
    delete row.componentConceptSources;
    for (const component of row.treeSitterComponents ?? []) if (component.grammarLinks) {
      const file = `${row.language}-${component.position}-${component.id}`.replace(/[^A-Za-z0-9_-]/gu, '_');
      writeFileSync(join(outDir, 'native-grammar-components', `${file}.lino`), component.grammarLinks);
      delete component.grammarLinks;
    }
  }
  writeGrammarReport(join(outDir, 'grammar-bulk-matrix.lino'), matrix);
  writeFileSync(join(outDir, 'grammar-bulk-matrix.md'), renderMatrix(matrix));
  // Retain every completed language and its diagnostics before the aggregate
  // proof construction. A failed aggregate must not erase the original work.
  process.stderr.write(`Building the global concept space from ${importedSources.length} imported sources\n`);
  const conceptSpace = createPipelineGlobalConceptSpace(importedSources, { formalDefinitions });
  writeGrammarReport(join(outDir, 'global-concept-space-data.lino'), conceptSpace);
  writeFileSync(join(outDir, 'global-concept-space.lino'), globalConceptSpaceToLinks(conceptSpace));
  matrix.globalConceptSpace = {
    copiedOrigins: Object.keys(conceptSpace.mappings).length,
    concepts: conceptSpace.concepts.length,
    proofs: conceptSpace.proofs.length,
    formalOrigins: Object.keys(conceptSpace.formalDefinitions).length,
    failures: conceptSpace.failures,
    ambiguities: conceptSpace.ambiguities,
  };
  writeGrammarReport(join(outDir, 'grammar-bulk-matrix.lino'), matrix);
  return matrix;
}

function option(args, name, fallback) {
  const index = args.indexOf(name);
  return index === -1 ? fallback : args[index + 1];
}

async function main(args) {
  const definitionsFile = option(args, '--formal-definitions', null);
  const formalDefinitions = definitionsFile === null ? undefined : readGrammarReport(resolve(definitionsFile));
  const worker = option(args, '--worker', null);
  if (worker) {
    const entry = bulkLanguages().find(({ name }) => name === worker);
    await bulkLanguageRow(entry, (row) => process.stdout.write(`${JSON.stringify(row)}\n`), { formalDefinitions });
    return;
  }
  const outDir = option(args, '--out-dir', null);
  if (!outDir) throw new Error('usage: run-grammar-bulk-pipeline.mjs --out-dir DIR [--only A,B] [--jobs N] [--timeout SECONDS] [--formal-definitions FILE]');
  const only = option(args, '--only', null)?.split(',');
  const previousFile = option(args, '--previous-matrix', null);
  const matrix = await runBulkPipeline({
    outDir: resolve(outDir),
    only,
    jobs: Number(option(args, '--jobs', 2)),
    timeout: Number(option(args, '--timeout', 300)),
    memory: Number(option(args, '--memory', 3072)),
    previousMatrix: previousFile === null ? null : readGrammarReport(resolve(previousFile)),
    formalDefinitions,
  });
  process.stdout.write(`${JSON.stringify(matrix.summary)}\n`);
}

if (process.argv[1] === script) {
  main(process.argv.slice(2)).catch((error) => {
    process.stderr.write(`${error.stack ?? error}\n`);
    process.exit(1);
  });
}
