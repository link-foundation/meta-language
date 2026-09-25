#!/usr/bin/env node
import { execFileSync } from 'node:child_process';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import {
  LinkNetwork,
  LinkType,
  analyzeProgram,
  constructProgram,
  decodeProgramTranslation,
  translateProgram,
} from '../src/index.js';
import { HIDDEN_TEXT_TERM } from '../src/programming-language-parser.js';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const corpus = JSON.parse(
  await readFile(path.join(root, 'parity/fixtures/four-language-conformance.json'), 'utf8'),
);
const grammarInventory = JSON.parse(
  await readFile(path.join(root, 'parity/language-grammar-inventory.json'), 'utf8'),
);
const rust = JSON.parse(execFileSync('cargo', [
  'run', '--quiet', '--manifest-path', path.join(root, 'rust/Cargo.toml'),
  '--example', 'issue_195_runtime_probe',
], { cwd: root, encoding: 'utf8', maxBuffer: 16 * 1024 * 1024 }));
const javascript = runtimeObservation();

const artifactsOption = process.argv.indexOf('--artifacts-dir');
if (artifactsOption !== -1) {
  const directory = path.resolve(root, process.argv[artifactsOption + 1]);
  await mkdir(directory, { recursive: true });
  await Promise.all([
    writeFile(path.join(directory, 'javascript.json'), stableJson(javascript)),
    writeFile(path.join(directory, 'rust.json'), stableJson(rust)),
  ]);
}

for (const section of ['positive', 'negative', 'inventory', 'semantics', 'transforms', 'translations']) {
  if (stableJson(javascript[section]) !== stableJson(rust[section])) {
    throw new Error(`JavaScript/Rust runtime parity mismatch in ${section}`);
  }
}
console.log('issue-195 runtime parity: shared fixture observations agree; full language and translation coverage remains subject to the acceptance gate');

function runtimeObservation() {
  return {
    schemaVersion: 1,
    positive: corpus.languages.map(({ name, source }) => networkObservation(name, source)),
    negative: corpus.negativeCases.map(({ language, source }) =>
      networkObservation(language, source)),
    inventory: grammarInventory.languages.map(({ name, source }) =>
      networkObservation(name, source)),
    semantics: corpus.semanticPrograms.map(programObservation),
    transforms: corpus.transformationPrograms.map(transformObservation),
    translations: translationObservations(),
  };
}

function networkObservation(language, source) {
  const network = LinkNetwork.parse(source, language);
  const links = network.links();
  const nodes = links
    .filter((link) => isStructuralNode(link) && !ignoredWrapper(link))
    .map(nodeSignature)
    .sort(compareUtf8);
  const directParents = new Map();
  for (const parent of links.filter((link) => link.metadata().linkType === LinkType.Syntax)) {
    for (const child of parent.references().map((reference) => network.link(reference))) {
      if (isStructuralNode(child)) directParents.set(child.id().asU64(), parent);
    }
  }
  const edges = links
    .filter((link) => isStructuralNode(link) && !ignoredWrapper(link))
    .flatMap((child) => {
      let parent = directParents.get(child.id().asU64());
      while (parent && ignoredWrapper(parent)) parent = directParents.get(parent.id().asU64());
      return parent ? [`${nodeSignature(parent)} -> ${nodeSignature(child)}`] : [];
    })
    .sort(compareUtf8);
  const fields = links
    .filter((link) => link.metadata().linkType === LinkType.Field)
    .flatMap((field) => {
      const [parent, child] = field.references().map((reference) => network.link(reference));
      return [`${field.metadata().term}: ${nodeSignature(parent)} -> ${nodeSignature(child)}`];
    })
    .sort(compareUtf8);
  const annotations = links.flatMap(annotationSignature).sort(compareUtf8);
  return {
    language: canonicalLanguage(language),
    source,
    reconstruction: network.reconstructText(),
    clean: network.verifyFullMatch().isClean(),
    nodes,
    edges,
    fields,
    annotations,
  };
}

// Region-scoped language identification and Unicode annotations. Both
// runtimes share the trigram identifier, so its term is compared exactly; the
// word segmenter engines differ by runtime, so only their presence is
// compared and their verdicts show up in the tokens.
function annotationSignature(link) {
  const metadata = link.metadata();
  if (!metadata.span || metadata.term === undefined) return [];
  const kind = { [LinkType.Language]: 'language', [LinkType.Semantic]: 'semantic' }[metadata.linkType];
  if (!kind) return [];
  const term = metadata.term.replace(/^segmentation:.*$/su, 'segmentation');
  const language = metadata.language ? canonicalLanguage(metadata.language) : '';
  return [`${kind} ${term} @ ${language}`];
}

// Rust sorts strings by UTF-8 bytes; JavaScript's default sort compares UTF-16
// code units, which orders supplementary-plane characters differently.
function compareUtf8(left, right) {
  return Buffer.compare(Buffer.from(left), Buffer.from(right));
}

function isStructuralNode(link) {
  return link && [LinkType.Syntax, LinkType.SourceToken].includes(link.metadata().linkType);
}

function nodeSignature(link) {
  const metadata = link.metadata();
  const span = metadata.span && {
    byteStart: metadata.span.byteRange.start,
    byteEnd: metadata.span.byteRange.end,
    startRow: metadata.span.start.row,
    startColumn: metadata.span.start.column,
    endRow: metadata.span.end.row,
    endColumn: metadata.span.end.column,
  };
  return canonicalJson({
    type: metadata.linkType === LinkType.Syntax ? 'syntax' : 'token',
    term: metadata.term ?? null,
    language: metadata.language ? canonicalLanguage(metadata.language) : null,
    named: metadata.linkType === LinkType.SourceToken ? false : metadata.named,
    span: span ?? null,
    flags: {
      isError: metadata.flags.isError,
      hasError: metadata.flags.hasError,
      isMissing: metadata.flags.isMissing,
      isExtra: metadata.flags.isExtra,
    },
  });
}

function ignoredWrapper(link) {
  const metadata = link.metadata();
  return metadata.linkType === LinkType.Syntax && ['whitespace', HIDDEN_TEXT_TERM].includes(metadata.term);
}

function programObservation(fixture) {
  const program = analyzeProgram(fixture.source, fixture.language, fixture.project);
  const fact = ({ kind, name, start, end, phase = null }) => ({
    kind,
    name,
    range: byteRange(fixture.source, start, end),
    phase,
  });
  return {
    language: fixture.language,
    source: fixture.source,
    scopes: program.scopes.map(({ parent, start, end, depth }) => ({
      parent: parent !== null,
      range: byteRange(fixture.source, start, end),
      depth,
    })),
    bindings: program.bindings.map(({ name, kind, declaration, references }) => ({
      name,
      kind,
      declaration: byteRange(fixture.source, declaration.start, declaration.end),
      references: references.map(({ start, end }) => byteRange(fixture.source, start, end)),
    })),
    unresolvedReferences: sorted(program.unresolvedReferences.map(fact)),
    modules: sorted(program.modules.map(fact)),
    types: sorted(program.types.filter(({ kind }) => kind !== 'syntax-type').map(fact)),
    extensions: sorted(program.extensions
      .filter((value) => semanticExtension(fixture.language, fixture.source, value)).map(fact)),
    proofs: sorted(program.proofs
      .filter((value) => semanticProof(fixture.language, fixture.source, value)).map(fact)),
    sourceMappingCoverage: sourceMappingCoverage(
      fixture.source,
      program.sourceMappings.map(({ byteStart, byteEnd }) => ({
        start: byteStart,
        end: byteEnd,
      })),
    ),
    diagnostics: sorted(program.diagnostics.map(({ kind, term, start, end }) => ({
      kind,
      term,
      range: byteRange(fixture.source, start, end),
    }))),
    constructs: sorted(program.constructs.map(({ kind, status, evidence, rationale = null }) => ({
      kind,
      status,
      evidencePresent: evidence.length > 0,
      rationale,
    }))),
  };
}

function sourceMappingCoverage(source, ranges) {
  const length = Buffer.byteLength(source);
  return {
    nonEmpty: ranges.length > 0,
    fullSpan: ranges.some(({ start, end }) => start === 0 && end === length),
    rangesValid: ranges.every(({ start, end }) => start <= end && end <= length),
  };
}

function semanticExtension(language, source, { kind, start, end }) {
  const markers = {
    JavaScript: ['String', 'directive'],
    Rust: ['macro_rules', '#'],
    Lean: ['macro', 'notation', 'syntax', 'postfix', 'prefix', 'infix', 'infixl', 'infixr', '@'],
    Rocq: ['Notation', 'Ltac', '#'],
  };
  return markers[language].includes(kind) &&
    (kind === 'directive' || source.slice(start, end) === kind);
}

function semanticProof(language, source, { kind, start, end }) {
  const markers = {
    Lean: ['theorem', 'lemma', 'by', 'rfl', 'simp', 'exact', 'apply'],
    Rocq: ['Theorem', 'Lemma', 'Proof', 'Qed', 'Defined', 'reflexivity', 'intros', 'exact', 'apply'],
  };
  return (markers[language] ?? []).includes(kind) && source.slice(start, end) === kind;
}

function sorted(values) {
  return values.sort((left, right) => {
    const leftKey = canonicalJson(left);
    const rightKey = canonicalJson(right);
    return leftKey < rightKey ? -1 : leftKey > rightKey ? 1 : 0;
  });
}

function transformObservation(fixture) {
  const program = constructProgram(fixture.source, fixture.language);
  const boundary = Buffer.byteLength(fixture.first);
  const first = { start: 0, end: boundary };
  const second = { start: boundary, end: Buffer.byteLength(fixture.source) };
  const replacement = fixture.first.replace(/first|FIRST/u, 'primary');
  return {
    language: fixture.language,
    queryCount: program.querySyntax('identifier').length,
    emit: program.emit(),
    replace: program.replace(first, replacement).emit(),
    insert: program.insert(second.end, fixture.inserted).emit(),
    delete: program.delete(second).emit(),
    clone: program.clone(first, second.end).emit(),
    move: program.move(second, 0).emit(),
  };
}

function translationObservations() {
  const languages = ['JavaScript', 'Rust', 'Lean', 'Rocq'];
  return languages.flatMap((sourceLanguage) => {
    const source = corpus.semanticPrograms.find(({ language }) => language === sourceLanguage).source;
    return languages.filter((targetLanguage) => targetLanguage !== sourceLanguage)
      .map((targetLanguage) => {
        const translated = translateProgram(source, sourceLanguage, targetLanguage);
        return {
          sourceLanguage,
          targetLanguage,
          code: translated.code,
          decodedSource: decodeProgramTranslation(translated.code, targetLanguage).source,
          contract: translated.contract,
        };
      });
  });
}

function byteRange(source, start, end) {
  return {
    start: Buffer.byteLength(source.slice(0, start)),
    end: Buffer.byteLength(source.slice(0, end)),
  };
}

function canonicalLanguage(language) {
  return language.toLowerCase() === 'coq' ? 'Rocq' : language;
}

function canonicalJson(value) {
  if (Array.isArray(value)) return `[${value.map(canonicalJson).join(',')}]`;
  if (value && typeof value === 'object') {
    return `{${Object.keys(value).sort().map((key) =>
      `${JSON.stringify(key)}:${canonicalJson(value[key])}`).join(',')}}`;
  }
  return JSON.stringify(value);
}

function stableJson(value) {
  return `${JSON.stringify(sortKeys(value), null, 2)}\n`;
}

function sortKeys(value) {
  if (Array.isArray(value)) return value.map(sortKeys);
  if (!value || typeof value !== 'object') return value;
  return Object.fromEntries(Object.keys(value).sort().map((key) => [key, sortKeys(value[key])]));
}
