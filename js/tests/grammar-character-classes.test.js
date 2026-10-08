import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { test } from 'node:test';

import { compileGrammar, parseGrammarLinks } from '../src/index.js';
import { importTreeSitterNative, parseTreeSitterPattern, renderTreeSitterNative, UnsupportedTreeSitterPattern } from '../src/grammar-importers/tree-sitter-native.js';
import { importCharacterClassFixtures } from '../scripts/import-native-grammars.mjs';
import { recordIssue195Observations } from './support/issue-195-observations.js';

const fixtureFile = 'parity/fixtures/native-character-class-grammars.json';
const fixtureText = readFileSync(new URL(`../../${fixtureFile}`, import.meta.url), 'utf8');
const fixture = JSON.parse(fixtureText);

test('native pattern imports execute character-class intersection, difference and symmetric difference', () => {
  for (const [pattern, accepts, rejects] of [
    ['[a-z&&[^aeiou]]', ['b', 'z'], ['a', '1', '', 'bb']],
    ['[a-z--aeiou]', ['b', 'z'], ['a', '1', '', 'bb']],
    ['[a-g~~b-h]', ['a', 'h'], ['b', 'g', '', 'ah']],
  ]) {
    const imported = importTreeSitterNative({ name: 'character_classes', rules: { source_file: { type: 'PATTERN', value: pattern } } });
    assert.deepEqual(imported.report.unsupported, [], pattern);
    const parser = compileGrammar(parseGrammarLinks(renderTreeSitterNative(imported)));
    for (const input of accepts) assert.equal(parser.parseTree(input).ok, true, `${pattern} accepts ${input}`);
    for (const input of rejects) assert.equal(parser.parseTree(input).ok, false, `${pattern} rejects ${input}`);
  }
});

test('malformed hexadecimal and non-scalar escapes fail with an importer diagnostic', () => {
  for (const pattern of ['\\x{}', '\\x{', '\\x{GG}', '\\x{110000}', '\\u{D800}', '\\U00110000', '[a&&b']) {
    assert.throws(() => parseTreeSitterPattern(pattern), UnsupportedTreeSitterPattern, pattern);
  }
});

test('compiled character-class corpus preserves set membership and scalar consumption', () => {
  assert.equal(importCharacterClassFixtures()[0][1], fixtureText, 'generated Links data matches the importer');
  assert.equal(new Set(fixture.cases.map(({ id }) => id)).size, fixture.cases.length);
  for (const entry of fixture.cases) {
    const parser = compileGrammar(parseGrammarLinks(entry.grammar));
    assert.ok(!entry.grammar.includes('(regex '), `${entry.id} executes native Links operations`);
    for (let scalar = 0; scalar < 128; scalar += 1) {
      const input = String.fromCodePoint(scalar);
      const expected = entry.asciiExcludes === undefined ? entry.asciiAccepts.includes(input) : !entry.asciiExcludes.includes(input);
      assert.equal(parser.parseTree(input).ok, expected, `${entry.id}: ASCII ${scalar}`);
    }
    for (const input of entry.accepts) {
      const result = parser.parseTree(input);
      assert.equal(result.ok, true, `${entry.id} accepts ${JSON.stringify(input)}`);
      assert.deepEqual([result.tree.start, result.tree.end], [0, Buffer.byteLength(input)], `${entry.id} consumes the complete input`);
    }
    for (const input of entry.rejects) assert.equal(parser.parseTree(input).ok, false, `${entry.id} rejects ${JSON.stringify(input)}`);
  }
  recordIssue195Observations({
    requirementId: 'I195-GRAMMAR-CHARACTER-CLASS-SETS',
    suffix: 'behavior',
    fixtureId: 'planned:repository-directive:i195-grammar-character-class-sets',
    fixtureFile,
    assertions: ['compiledClassSetsUseLinks', 'nativeClassSetsMatchFixtures', 'nativeClassSetsConsumeCompleteInputs'],
    testName: 'compiled character-class corpus preserves set membership and scalar consumption',
  });
  recordIssue195Observations({
    requirementId: 'I195-GRAMMAR-HEXADECIMAL-SCALARS',
    suffix: 'behavior',
    fixtureId: 'planned:repository-directive:i195-grammar-hexadecimal-scalars',
    fixtureFile,
    assertions: ['nativeHexadecimalScalarsMatchFixtures', 'nativeScalarEscapesConsumeCompleteInputs'],
    testName: 'compiled character-class corpus preserves set membership and scalar consumption',
  });
});
