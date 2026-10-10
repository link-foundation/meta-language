import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import test from 'node:test';
import { scannerFamilies } from '../scripts/scanner-families.mjs';
import { compileGrammar, parseGrammarLinks, renderSyntaxTree } from '../src/index.js';
import { readScannerContinuationAction } from '../src/translation/frontend-rules.js';

const fixture = JSON.parse(readFileSync(new URL('../../parity/fixtures/scanner-indentation.json', import.meta.url), 'utf8'));
const parser = compileGrammar(parseGrammarLinks(fixture.listing));
function blockStructure(tree) {
  return tree.children.filter((child) => child.kind === 'statement').map((statement) => {
    const name = statement.children.find((child) => child.kind === 'name').text;
    const block = statement.children.find((child) => child.kind === 'block');
    return block ? [name, blockStructure(block)] : name;
  });
}
function sourceText(tree) {
  return tree.type === 'token' ? tree.text : tree.children.map(sourceText).join('');
}

test('indentation scanner data preserves independent block structures and every source byte', () => {
  assert.equal(scannerFamilies(fixture.scanners), fixture.generated);
  for (const { input, structure, tree } of fixture.accept) {
    const result = parser.parseTree(input);
    assert.equal(result.ok, true, input);
    assert.deepEqual(blockStructure(result.tree), structure, input);
    assert.equal(sourceText(result.tree), input);
    assert.equal(renderSyntaxTree(result.tree), tree);
  }
  for (const input of fixture.reject) assert.equal(parser.parseTree(input).ok, false, input);
});

test('virtual tokens keep the nearest actual lexed continuation', () => {
  for (const widthless of [false, true]) {
    for (const startsAfter of [false, true]) {
      for (const endsAfter of [false, true]) {
        assert.equal(readScannerContinuationAction(widthless, startsAfter, endsAfter), widthless ? 0 : startsAfter && endsAfter ? 1 : -1);
      }
    }
  }
});

test('indentation counts wrap at the descriptor bound and reject malformed parameters', () => {
  const descriptor = { ...fixture.scanners[0], countModulo: 16 };
  const generated = scannerFamilies([descriptor]);
  const bounded = compileGrammar(parseGrammarLinks(fixture.listing.replace(fixture.generated, generated)));
  assert.equal(bounded.parseTree(`a:\n${' '.repeat(15)}b\n`).ok, true);
  assert.equal(bounded.parseTree(`a:\n${' '.repeat(16)}b\n`).ok, false);
  for (const change of [{ indentToken: 'newline' }, { tabWidth: 0 }, { tabWidth: 1.5 }, { countModulo: 8 }, { countModulo: Infinity }, { commentPrefix: '' }, { resetCharacters: ['ab'] }, { quoteCharacters: [] }, { continuation: '' }, { name: ') fail (' }]) {
    assert.throws(() => scannerFamilies([{ ...descriptor, ...change }]), TypeError);
  }
});
