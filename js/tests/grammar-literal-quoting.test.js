import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import test from 'node:test';

import {
  GrammarBuilder,
  GrammarImportError,
  emitBnf,
  emitEbnf,
  importBnf,
  importEbnf,
} from '../src/index.js';
import { renderGrammarRule } from './support/render-grammar-expression.js';

// Shared with rust/tests/unit/grammar_literal_quoting.rs.
const fixture = JSON.parse(
  readFileSync(new URL('../../parity/fixtures/grammar-literal-quoting.json', import.meta.url)),
);
const importers = { bnf: importBnf, ebnf: importEbnf };
const emitters = { bnf: emitBnf, ebnf: emitEbnf };

test('shared BNF/EBNF literal quoting imports match the Rust runtime', () => {
  for (const { id, format, source, rule, error } of fixture.imports) {
    if (error) {
      assert.throws(() => importers[format](source), GrammarImportError, id);
      continue;
    }
    assert.equal(renderGrammarRule(importers[format](source).rule('a')), rule, id);
  }
});

test('BNF/EBNF emitters quote terminals so the same importer re-reads them', () => {
  for (const { id, format, value, source, lossy, reimported } of fixture.emits) {
    const grammar = new GrammarBuilder('a').rule('a', GrammarBuilder.literal(value)).build();
    const emitted = emitters[format](grammar);
    assert.equal(emitted.source, source, id);
    assert.deepEqual(emitted.report.lossy, lossy, id);
    assert.equal(renderGrammarRule(importers[format](emitted.source).rule('a')), reimported, id);
  }
});
