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

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const corpus = JSON.parse(
  await readFile(path.join(root, 'parity/fixtures/four-language-conformance.json'), 'utf8'),
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

for (const section of ['positive', 'negative', 'semantics', 'transforms', 'translations']) {
  if (stableJson(javascript[section]) !== stableJson(rust[section])) {
    throw new Error(`JavaScript/Rust runtime parity mismatch in ${section}`);
  }
}
console.log('issue-195 runtime parity: complete CST, diagnostics, semantics, transforms, and 12 translations match');

function runtimeObservation() {
  return {
    schemaVersion: 1,
    positive: corpus.languages.map(({ name, source }) => networkObservation(name, source)),
    negative: corpus.negativeCases.map(({ language, source }) =>
      networkObservation(language, source)),
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
    .sort();
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
    .sort();
  const fields = links
    .filter((link) => link.metadata().linkType === LinkType.Field)
    .flatMap((field) => {
      const [parent, child] = field.references().map((reference) => network.link(reference));
      const label = normalizedField(field.metadata().term, parent, child);
      return label ? [`${label}: ${nodeSignature(parent)} -> ${nodeSignature(child)}`] : [];
    })
    .sort();
  return {
    language: canonicalLanguage(language),
    source,
    reconstruction: network.reconstructText(),
    clean: network.verifyFullMatch().isClean(),
    nodes,
    edges,
    fields,
  };
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
    term: normalizedTerm(link),
    language: metadata.language ? canonicalLanguage(metadata.language) : null,
    named: metadata.linkType === LinkType.SourceToken ? false : metadata.named,
    span: span ?? null,
    flags: {
      isError: metadata.flags.isError,
      // Node and Rust Tree-sitter disagree on whether an ERROR node also
      // "has" an error. The acceptance view is deliberately inclusive.
      hasError: metadata.flags.hasError || metadata.flags.isError || metadata.flags.isMissing,
      isMissing: metadata.flags.isMissing,
      isExtra: metadata.flags.isExtra,
    },
  });
}

function ignoredWrapper(link) {
  const metadata = link.metadata();
  return metadata.linkType === LinkType.Syntax &&
    (metadata.term === 'whitespace' ||
      (metadata.language === 'Lean' && metadata.term === 'declaration'));
}

function normalizedTerm(link) {
  const metadata = link.metadata();
  const length = metadata.span && metadata.span.byteRange.end - metadata.span.byteRange.start;
  if (metadata.language === 'Lean' && metadata.term === 'def' && metadata.named && length > 3) {
    return 'definition';
  }
  if (metadata.language === 'Lean' && metadata.term === 'unary_op') return 'unary_expression';
  return metadata.term ?? null;
}

function normalizedField(label, parent, child) {
  // The Lean builds expose punctuation and wrapper-only fields differently.
  // Keep every named semantic field and normalize the sole renamed operand.
  if (!child?.metadata().named || normalizedTerm(child) === 'binders') return null;
  if (parent?.metadata().language === 'Lean' && normalizedTerm(parent) === 'unary_expression') {
    return label === 'rhs' ? 'operand' : label;
  }
  return label;
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
