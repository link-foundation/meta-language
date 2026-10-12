import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { readProjectData } from '../scripts/project-data.mjs';
import test from 'node:test';
import { generateNativeTextGrammars } from '../scripts/build-native-text-grammars.mjs';
import { compileGrammar, GrammarBuilder } from '../src/grammar.js';
import { parseGrammarLinks } from '../src/grammar-links.js';
import { LinkNetwork, LinkType } from '../src/index.js';
import { parseNaturalLanguageCst, parsePlainTextCst } from '../src/text-grammar.js';
import { recordIssue195Observations } from './support/issue-195-observations.js';

const fixturePath = 'parity/fixtures/native-text-grammars.json';
const fixture = readProjectData(new URL(`../../${fixturePath}`, import.meta.url));
const grammarText = await readFile(new URL('../src/data/native-text-grammar.lino', import.meta.url), 'utf8');
const metadata = readProjectData(new URL('../src/data/native-text-metadata.lino', import.meta.url));

test('native text adapters retain every independent built-in concrete tree expectation', async () => {
  const inventory = readProjectData(new URL('../../parity/language-grammar-inventory.json', import.meta.url));
  const expected = readProjectData(new URL('../../parity/fixtures/builtin-cst-expected.json', import.meta.url));
  for (const language of inventory.languages.filter(({ name, family }) => name === 'txt' || family === 'natural')) {
    for (const [part, field] of [['positive', 'source'], ['recovery', 'recoverySource']]) {
      const source = language[field];
      const parse = language.name === 'txt' ? parsePlainTextCst : parseNaturalLanguageCst;
      const rows = [];
      const visit = (tree, depth) => {
        rows.push([depth, null, tree.term, tree.named ? 1 : 0,
          Buffer.byteLength(source.slice(0, tree.start)), Buffer.byteLength(source.slice(0, tree.end)),
          `${tree.isError ? 'E' : ''}${tree.isMissing ? 'M' : ''}${tree.extra ? 'X' : ''}`]);
        tree.children.forEach((child) => visit(child, depth + 1));
      };
      visit(parse(source), 0);
      assert.deepEqual(rows, expected.languages[language.name][part], `${language.name} ${part}`);
    }
  }
});

test('native text grammars are reproducible and shipped identically to both executors', async () => {
  await generateNativeTextGrammars({ check: true });
  assert.equal(await readFile(new URL('../../rust/src/data/native-text-grammar.lino', import.meta.url), 'utf8'), grammarText);
  assert.deepEqual(readProjectData(new URL('../../rust/src/data/native-text-metadata.lino', import.meta.url)), metadata);
});

test('native plain text preserves inclusive lines, lexemes, Unicode and diagnostics', () => {
  for (const { source, regions, hasError } of fixture.plain) {
    const tree = parsePlainTextCst(source);
    assert.equal(tree.term, 'text_document');
    assert.deepEqual(tree.children.map(({ start, end }) => source.slice(start, end)), regions);
    assert.equal(Boolean(tree.hasError), hasError, JSON.stringify(source));
    assert.equal(tree.children.flatMap(({ children }) => children.map(({ start, end }) => source.slice(start, end))).join(''), source);
    assert.equal(LinkNetwork.parse(source, 'txt').reconstructText(), source);
  }
  const tree = parsePlainTextCst(fixture.lexical.source);
  assert.deepEqual(tree.children.flatMap(({ children }) => children.map(({ term, start, end }) => [term, fixture.lexical.source.slice(start, end)])), fixture.lexical.tokens);
});

test('native natural language rules execute with lossless spans, sentence lexemes and control diagnostics', () => {
  const native = compileGrammar(parseGrammarLinks(grammarText));
  for (const { source, language, regions, hasError = false } of fixture.natural) {
    const parsed = native.parseTree(source, { startRule: metadata.starts.natural });
    assert.equal(parsed.ok, true, language);
    assert.equal(parsed.tree.kind, 'natural_language_document');
    const bytes = Buffer.from(source);
    assert.deepEqual(parsed.tree.children.map(({ start, end }) => bytes.subarray(start, end).toString()), regions);
    const tree = parseNaturalLanguageCst(source);
    assert.deepEqual(tree.children.map(({ start, end }) => source.slice(start, end)), regions);
    assert.equal(tree.children.flatMap(({ children }) => children.map(({ start, end }) => source.slice(start, end))).join(''), source);
    assert.equal(Boolean(tree.hasError), hasError, language);
    const network = LinkNetwork.parse(source, language);
    assert.equal(network.reconstructText(), source);
    const syntax = network.links().filter((link) => link.metadata().linkType === LinkType.Syntax);
    assert.ok(syntax.some((link) => link.metadata().term === 'natural_language_document'));
    if (hasError) assert.ok(syntax.some((link) => link.metadata().flags.isError));
  }
  recordIssue195Observations({
    requirementId: 'I195-GRAMMAR-NATURAL-LANGUAGE-REPRESENTATION',
    suffix: 'behavior',
    fixtureId: 'native-text-grammars',
    testName: 'native natural language rules execute with lossless spans, sentence lexemes and control diagnostics',
    assertions: ['nativeNaturalLanguageGrammarExecuted', 'naturalLanguageUnicodeSpansLossless', 'naturalLanguageSentenceLexemesPreserved', 'naturalLanguageControlDiagnostics'],
    fixtureFile: fixturePath,
  });
});

test('editing a native sentence production changes its executable boundaries', () => {
  const grammar = parseGrammarLinks(grammarText);
  const source = 'First. Second!';
  const before = compileGrammar(grammar).parseTree(source, { startRule: metadata.starts.natural });
  assert.equal(before.tree.children.length, 2);
  const rule = grammar.rule('natural_language_sentence');
  grammar.rules.set(rule.name, { ...rule, expression: GrammarBuilder.repeat1(GrammarBuilder.any()) });
  const after = compileGrammar(grammar).parseTree(source, { startRule: metadata.starts.natural });
  assert.equal(after.ok, true);
  assert.equal(after.tree.children.length, 1);
  assert.equal(after.tree.children[0].end, Buffer.byteLength(source));
});
