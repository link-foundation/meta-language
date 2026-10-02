// Regenerates parity/fixtures/grammar-importers.json: appends the construct
// coverage cases and records, per case, the runtime-neutral rendering of every
// declared rule plus the same-format emitted text and fidelity notes. Both the
// JavaScript and Rust evidence tests compare against these recorded values.
// The malformed sources every importer must reject are carried over as is.
import { readFileSync, writeFileSync } from 'node:fs';
import * as ml from '../js/src/index.js';
import { renderGrammarRule } from '../js/tests/support/render-grammar-expression.js';
import { candidates } from './issue-195-importer-construct-cases.mjs';

const fixtureUrl = new URL('../parity/fixtures/grammar-importers.json', import.meta.url);
const corpus = JSON.parse(readFileSync(fixtureUrl, 'utf8'));
const pairs = {
  abnf: [ml.importAbnf, ml.emitAbnf],
  bnf: [ml.importBnf, ml.emitBnf],
  ebnf: [ml.importEbnf, ml.emitEbnf],
  pest: [ml.importPest, ml.emitPest],
  'tree-sitter-json': [ml.importTreeSitterJson, ml.emitTreeSitterJson],
};
const base = corpus.cases.filter((fixture) => !fixture.id?.endsWith(':constructs'));
const cases = [
  ...base.map((fixture) => ({ id: `${fixture.format}:message`, ...fixture })),
  ...candidates.map((fixture) => ({ id: `${fixture.format}:constructs`, ...fixture })),
].map(({ id, format, source, start, rules, accepts, rejects }) => {
  const [importer, emitter] = pairs[format];
  const grammar = importer(source);
  const emitted = emitter(grammar);
  return {
    id, format, source, start, rules,
    expressions: Object.fromEntries(rules.map((name) => [name, renderGrammarRule(grammar.rule(name))])),
    emitted: { source: emitted.source, lossy: emitted.report.lossy },
    accepts, rejects,
  };
});

// Two-space JSON with short string arrays kept on one line, as before.
function format(value, indent = '') {
  const next = `${indent}  `;
  if (Array.isArray(value)) {
    if (value.every((item) => typeof item === 'string')) {
      return `[${value.map((item) => JSON.stringify(item)).join(', ')}]`;
    }
    return `[\n${value.map((item) => next + format(item, next)).join(',\n')}\n${indent}]`;
  }
  if (value && typeof value === 'object') {
    const entries = Object.entries(value);
    if (entries.length === 0) return '{}';
    return `{\n${entries.map(([key, item]) => `${next}${JSON.stringify(key)}: ${format(item, next)}`).join(',\n')}\n${indent}}`;
  }
  return JSON.stringify(value);
}
writeFileSync(fixtureUrl, `${format({ schemaVersion: corpus.schemaVersion, cases, malformed: corpus.malformed ?? [] })}\n`);
console.log(`wrote ${cases.length} cases`);
