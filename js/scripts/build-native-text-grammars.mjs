#!/usr/bin/env node
// Built-in text grammar behavior is generated from recorded shared rule data.
// Both executors load the same Links Notation. No generated file is edited.
import { createHash } from 'node:crypto';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { readProjectData, readProjectFileBytes, serializeProjectData } from './project-data.mjs';
import { Grammar, GrammarBuilder as expression } from '../src/grammar.js';
import { renderGrammarLinks } from '../src/grammar-links.js';

const root = resolve(dirname(fileURLToPath(import.meta.url)), '../..');
export const NATIVE_TEXT_SPECIFICATION = 'parity/grammars/native-text-specification.json';
export const NATIVE_TEXT_GRAMMAR_FILES = ['parity/grammars/native-text.lino', 'js/src/data/native-text-grammar.lino', 'rust/src/data/native-text-grammar.lino'];
export const NATIVE_TEXT_METADATA_FILES = ['js/src/data/native-text-metadata.lino', 'rust/src/data/native-text-metadata.lino'];

function characterClass(specification, negated = false) {
  return expression.charClass([
    ...(specification.categories ?? []).map((value) => ({ kind: 'category', value })),
    ...(specification.characters ?? []).map((value) => ({ kind: 'char', value: String.fromCodePoint(value) })),
    ...(specification.ranges ?? []).map(([start, end]) => ({ kind: 'range', start: String.fromCodePoint(start), end: String.fromCodePoint(end) })),
  ], negated);
}

/** A full native grammar: region boundaries, lexemes and validation are links. */
export function buildNativeTextGrammar(specification) {
  const whitespace = characterClass(specification.whitespace);
  const word = characterClass(specification.word);
  const control = characterClass(specification.rejectedControl);
  const punctuation = expression.charClass([...whitespace.items, ...word.items, ...control.items], true);
  const terminals = characterClass({ characters: specification.sentenceTerminals.map((value) => value.codePointAt(0)) });
  const closing = characterClass(specification.sentenceClosings);
  const end = expression.not(expression.any());
  const newline = expression.literal(specification.lineEnding);
  const immediateToken = (item) => ({ kind: 'immediateToken', item });
  const line = expression.seq(immediateToken(expression.repeat0(expression.seq(expression.not(newline), expression.any()))), expression.orderedChoice(newline, end));
  const sentence = expression.seq(
    expression.and(expression.any()),
    immediateToken(expression.repeat0(expression.seq(expression.not(terminals), expression.any()))),
    expression.orderedChoice(immediateToken(expression.seq(expression.repeat1(terminals), expression.repeat0(closing), expression.repeat0(whitespace))), end),
  );
  const opening = expression.literal(specification.parentheses.opening);
  const closingParenthesis = expression.literal(specification.parentheses.closing);
  const rules = new Map([
    ['text_document', { expression: expression.orderedChoice(expression.repeat1(expression.seq(expression.and(expression.any()), expression.ref('text_line'))), expression.ref('text_line')) }],
    ['text_line', { expression: line }],
    ['natural_language_document', { expression: expression.repeat0(expression.ref('natural_language_sentence')) }],
    ['natural_language_sentence', { expression: sentence }],
    ['lexical_document', { expression: expression.repeat0(expression.orderedChoice(...['whitespace', 'lexical_word', 'rejected_control', 'lexical_punctuation'].map((name) => expression.ref(name)))) }],
    ['whitespace', { expression: expression.repeat1(whitespace), kind: 'token' }],
    ['lexical_word', { expression: expression.repeat1(word), kind: 'token' }],
    ['rejected_control', { expression: expression.repeat1(control), kind: 'token' }],
    ['lexical_punctuation', { expression: expression.repeat1(punctuation), kind: 'token' }],
    ['balanced_parenthesis_sequence', { expression: expression.repeat0(expression.orderedChoice(
      expression.seq(opening, expression.ref('balanced_parenthesis_sequence'), closingParenthesis),
      immediateToken(expression.repeat1(expression.seq(expression.not(expression.orderedChoice(opening, closingParenthesis)), expression.any()))),
    )) }],
  ]);
  for (const [name, rule] of rules) {
    rule.concept = specification.concepts[name];
    rule.sourceNames = [{ source: 'native:native-text', name: specification.projection[name]?.term ?? name }];
  }
  return new Grammar(specification.starts.plain, rules, null, { matching: 'peg' });
}

export async function generateNativeTextGrammars({ check = false } = {}) {
  const specificationBytes = readProjectFileBytes(join(root, NATIVE_TEXT_SPECIFICATION));
  const specification = readProjectData(join(root, NATIVE_TEXT_SPECIFICATION));
  const grammar = renderGrammarLinks(buildNativeTextGrammar(specification));
  const metadata = serializeProjectData({ generatedBy: 'js/scripts/build-native-text-grammars.mjs',
    source: NATIVE_TEXT_SPECIFICATION, sourceSha256: createHash('sha256').update(specificationBytes).digest('hex'),
    grammarSha256: createHash('sha256').update(grammar).digest('hex'),
    starts: specification.starts, validation: specification.validation, projection: specification.projection }, join(root, NATIVE_TEXT_METADATA_FILES[0]));
  const outputs = [...NATIVE_TEXT_GRAMMAR_FILES.map((path) => [path, grammar]), ...NATIVE_TEXT_METADATA_FILES.map((path) => [path, metadata])];
  for (const [path, expected] of outputs) {
    if (check) {
      if (await readFile(join(root, path), 'utf8') !== expected) throw new Error(`native text grammar is stale: ${path}`);
    } else {
      await mkdir(dirname(join(root, path)), { recursive: true });
      await writeFile(join(root, path), expected);
    }
  }
  return outputs;
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  await generateNativeTextGrammars({ check: process.argv.includes('--check') });
}
