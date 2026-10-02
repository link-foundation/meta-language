// Regenerates parity/fixtures/grammar-importers.json: appends the construct
// coverage cases and records, per case, the runtime-neutral rendering of every
// declared rule plus the same-format emitted text and fidelity notes. Both the
// JavaScript and Rust evidence tests compare against these recorded values.
// The malformed sources every importer must reject and the shared command
// expectations are carried over as is. The reverse section records, for one
// commented source per lossless format, the native links, the export of the
// grammar decoded from those links, the re-imported links, the layout links
// and the lossless emissions of the unchanged, mutated and renamed grammars.
import { readFileSync, writeFileSync } from 'node:fs';
import * as ml from '../js/src/index.js';
import { renderGrammarRule } from '../js/tests/support/render-grammar-expression.js';
import { candidates } from './issue-195-importer-construct-cases.mjs';
import { SOURCES } from './issue-195-reverse-conversion-sources.mjs';

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

// Samples written by hand, independent of every importer/emitter pair.
const REVERSE_ACCEPTS = ['Meta 42', 'a 7'];
const REVERSE_REJECTS = ['Meta', '42 Meta', 'Meta  42', 'Meta 4x'];
const reverse = Object.entries(SOURCES).map(([format, source]) => {
  const importGrammar = ml.grammarImporter(format);
  const report = ml.checkGrammarReverseConversion(source, {
    importGrammar, emitGrammar: ml.grammarEmitter(format), accepts: REVERSE_ACCEPTS, rejects: REVERSE_REJECTS,
  });
  if (report.status !== 'equivalent') throw new Error(`${format}: ${JSON.stringify(report.failures)}`);
  const { grammar, layout } = ml.importGrammarLossless(source, format);
  const decoded = ml.parseGrammarLinks(report.links);
  const rename = grammar.ruleNames().includes('word') ? 'word' : null;
  if (!rename) throw new Error(`${format}: no rule word to rename`);
  const lossless = (edited) => {
    const emitted = ml.emitGrammarLossless(edited, layout);
    if (ml.renderGrammarLinks(importGrammar(emitted.source)) !== ml.renderGrammarLinks(edited)) {
      throw new Error(`${format}: the lossless emission does not re-import to the edited grammar`);
    }
    return { links: ml.renderGrammarLinks(edited), source: emitted.source, lossy: emitted.report.lossy };
  };
  const unchanged = ml.emitGrammarLossless(decoded, layout);
  if (unchanged.source !== source) throw new Error(`${format}: the lossless emission is not exact`);
  return {
    id: `${format}:reverse`,
    format,
    source,
    accepts: REVERSE_ACCEPTS,
    rejects: REVERSE_REJECTS,
    links: report.links,
    exported: report.exported,
    reimportedLinks: report.reimportedLinks,
    layout: ml.renderGrammarLayoutLinks(layout),
    mutated: lossless(ml.mutateGrammarStartRule(decoded)),
    renamed: { from: rename, to: 'term', ...lossless(ml.renameGrammarRule(decoded, rename, 'term').grammar) },
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
writeFileSync(fixtureUrl, `${format({ schemaVersion: corpus.schemaVersion, cases, malformed: corpus.malformed ?? [], commands: corpus.commands ?? [], reverse })}\n`);
console.log(`wrote ${cases.length} cases and ${reverse.length} reverse conversions`);
