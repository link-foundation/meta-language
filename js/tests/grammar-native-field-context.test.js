import assert from 'node:assert/strict';
import { test } from 'node:test';
import { compileGrammar, parseGrammarLinks } from '../src/index.js';
import { importTreeSitterNative, renderTreeSitterNative } from '../src/grammar-importers/tree-sitter-native.js';
import { transformNativeSource } from '../scripts/native-grammar-transforms.mjs';

const source = {
  name: 'separated_call', extras: [{ type: 'PATTERN', value: '[ \\t]' }],
  rules: {
    source: { type: 'SYMBOL', name: 'call' },
    call: { type: 'SEQ', members: [
      { type: 'FIELD', name: 'target', content: { type: 'SYMBOL', name: 'identifier' } },
      { type: 'STRING', value: '+' },
      { type: 'FIELD', name: 'argument', content: { type: 'SYMBOL', name: 'identifier' } },
    ] },
    identifier: { type: 'PATTERN', value: '[a-z]+' },
  },
};
const context = { family: 'field-following-context', rule: 'call', field: 'target', pattern: '[ \\t]', count: 1 };
const text = node => node.type === 'node' ? node.children.map(text).join('') : node.text;

test('raw following field lookahead preserves separators and rejects adjacent arguments', () => {
  const transformed = transformNativeSource(source, [context]);
  assert.deepEqual(source.rules.call.members[0].content, { type: 'SYMBOL', name: 'identifier' });
  const parser = compileGrammar(parseGrammarLinks(renderTreeSitterNative(importTreeSitterNative(transformed))));
  for (const input of ['f +x', 'f\t+x', 'target  +argument']) {
    const result = parser.parseTree(input);
    assert.equal(result.ok, true, input);
    assert.equal(text(result.tree), input);
    const target = result.tree.children[0].children.find(child => child.field === 'target');
    assert.equal(target.end, input.indexOf(input.includes('\t') ? '\t' : ' '));
  }
  assert.equal(parser.parseTree('f+x').ok, false);
});

test('recorded following field contexts reject missing or drifting selectors', () => {
  assert.throws(() => transformNativeSource(source, [{ ...context, count: 2 }]), /recorded occurrences/u);
  assert.throws(() => transformNativeSource(source, [{ ...context, field: 'missing' }]), /recorded occurrences/u);
  assert.throws(() => transformNativeSource(source, [{ ...context, pattern: '' }]), /following field contexts/u);
});
