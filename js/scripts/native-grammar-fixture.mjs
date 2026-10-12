// Fixture generation shares one fresh independent oracle snapshot for the
// rows and recovery assertions of each source in this build only.
import { renderSyntaxTree } from '../src/grammar-runtime/syntax-tree.js';
import { nativeRows, oracleSnapshot } from './native-grammar-rows.mjs';

/** Builds the complete fixture using an already compiled native parser. */
export function buildNativeGrammarFixtureFromParser(entry, parser, snapshot = oracleSnapshot) {
  const snapshots = new Map();
  const oracle = (source) => {
    if (!snapshots.has(source)) snapshots.set(source, snapshot(source, entry.language));
    return snapshots.get(source);
  };
  const native = (source) => {
    const outcome = parser.parseTree(source);
    if (!outcome.ok) throw new Error(`${entry.grammar} rejects ${JSON.stringify(source)}`);
    return nativeRows(outcome.tree, source, entry);
  };
  const matches = entry.matches.map((source) => {
    const { rows, recovers } = oracle(source);
    if (recovers) throw new Error(`the ${entry.oracle} oracle recovers from ${JSON.stringify(source)}`);
    if (JSON.stringify(native(source)) !== JSON.stringify(rows)) {
      throw new Error(`${entry.grammar} and ${entry.oracle} disagree on ${JSON.stringify(source)}`);
    }
    return { source, rows };
  });
  const divergences = entry.divergences.map(({ source, reason }) => {
    if (!oracle(source).recovers) {
      throw new Error(`the ${entry.oracle} oracle accepts the divergence ${JSON.stringify(source)}`);
    }
    return { source, reason, rows: native(source) };
  });
  const rejections = entry.rejections.map((source) => {
    if (!oracle(source).recovers) {
      throw new Error(`the ${entry.oracle} oracle accepts the rejection ${JSON.stringify(source)}`);
    }
    if (parser.parseTree(source).ok) throw new Error(`${entry.grammar} accepts ${JSON.stringify(source)}`);
    const repaired = parser.parseTree(source, { errorRecovery: true });
    if (repaired.rejection?.reason !== 'recovered') throw new Error(`${entry.grammar} does not recover from ${JSON.stringify(source)}`);
    return { source, recovered: renderSyntaxTree(repaired.tree) };
  });
  const { id, language, grammar, sources, hidden, anonymous, extras, oracleKinds, corpus, rootIncludesLeadingTrivia } = entry;
  return {
    schemaVersion: 1, id, language, grammar, oracle: entry.oracle, sources, ...(corpus ? { corpus } : {}), hidden, anonymous, extras, oracleKinds, ...(rootIncludesLeadingTrivia ? { rootIncludesLeadingTrivia } : {}), matches, divergences, rejections,
  };
}
