// Authenticate additive lexer declaration lines against the actual bounded
// ANTLR importer, mutation and rename operations. Every historical rule line,
// source/export/layout/sample and non-ANTLR fixture remains immutable.
import { isDeepStrictEqual } from 'node:util';
import { emitAntlr } from '../src/grammar-emitters/antlr.js';
import { importAntlr } from '../src/grammar-importers/antlr.js';
import { emitGrammarLossless, importGrammarLossless, renderGrammarLayoutLinks } from '../src/grammar-lossless.js';
import { renderGrammarLinks } from '../src/grammar-links.js';
import { renameGrammarRule } from '../src/grammar-rename.js';
import { mutateGrammarStartRule, renderGrammarLexicalOwnerComparison } from '../src/grammar-round-trip.js';

const fields = [['links'], ['reimportedLinks'], ['mutated', 'links'], ['renamed', 'links']];

function withoutLexerDeclarations(value) {
  if (typeof value !== 'string') throw new Error('an ANTLR derived Links field must be text');
  return value.split('\n').filter((line) => !line.startsWith('(lexer ')).join('\n');
}

/** Compute only the four derived Links views from one unchanged source case. */
export function deriveAntlrReverseFixtureLinks(fixture) {
  if (fixture.format !== 'antlr') throw new Error('the bounded fixture refresh accepts only ANTLR');
  const { grammar, layout } = importGrammarLossless(fixture.source, 'antlr');
  if (renderGrammarLayoutLinks(layout) !== fixture.layout || emitAntlr(grammar).source !== fixture.exported) {
    throw new Error('ANTLR lexical refresh must preserve original layout and export bytes');
  }
  const mutated = mutateGrammarStartRule(grammar);
  const renamed = renameGrammarRule(grammar, fixture.renamed.from, fixture.renamed.to).grammar;
  for (const [edited, expected] of [[mutated, fixture.mutated], [renamed, fixture.renamed]]) {
    const emitted = emitGrammarLossless(edited, layout);
    if (emitted.source !== expected.source || !isDeepStrictEqual(emitted.report.lossy, expected.lossy)) {
      throw new Error('ANTLR lexical refresh must preserve every original edited source and fidelity report');
    }
    if (renderGrammarLexicalOwnerComparison(edited) !== renderGrammarLexicalOwnerComparison(importAntlr(emitted.source))) {
      throw new Error('ANTLR lexical refresh must preserve the complete edited lexical context under a checked owner bijection');
    }
  }
  const result = { links: renderGrammarLinks(grammar), reimportedLinks: renderGrammarLinks(importAntlr(fixture.exported)),
    mutated: { links: renderGrammarLinks(mutated) }, renamed: { links: renderGrammarLinks(renamed) } };
  for (const path of fields) {
    const read = (value) => path.reduce((current, key) => current[key], value);
    if (withoutLexerDeclarations(read(result)) !== withoutLexerDeclarations(read(fixture))) {
      throw new Error('ANTLR lexical refresh changed a protected original rule or grammar line');
    }
  }
  return result;
}

/** Restore the frozen historical view only after exact current regeneration. */
export function normalizeMigrationAntlrLexerDeclarations(original, current) {
  const normalized = structuredClone(current);
  if (!Array.isArray(original.reverse) || !Array.isArray(normalized.reverse)
    || original.reverse.length !== normalized.reverse.length) throw new Error('ANTLR lexical refresh changed protected fixture collections');
  let selected = 0;
  for (let index = 0; index < original.reverse.length; index += 1) {
    const historical = original.reverse[index];
    if (historical.format !== 'antlr') continue;
    selected += 1;
    const actual = normalized.reverse[index];
    if (actual.id !== historical.id || actual.format !== 'antlr') throw new Error('ANTLR lexical refresh changed a protected fixture identity');
    const expected = deriveAntlrReverseFixtureLinks(historical);
    for (const path of fields) {
      const read = (value) => path.reduce((current, key) => current[key], value);
      if (read(actual) !== read(expected)) throw new Error('ANTLR lexical refresh differs from actual regenerated lexical declarations');
      const parent = path.slice(0, -1).reduce((value, key) => value[key], actual);
      parent[path.at(-1)] = read(historical);
    }
  }
  if (selected === 0 || !isDeepStrictEqual(normalized, original)) {
    throw new Error('ANTLR lexical refresh changed protected sources, samples, layouts, exports or original rule fields');
  }
  return normalized;
}

/** Apply the narrowly permitted generated addition without editing any input. */
export function refreshAntlrReverseFixtureLexers(original) {
  const result = structuredClone(original);
  for (const fixture of result.reverse ?? []) {
    if (fixture.format !== 'antlr') continue;
    const generated = deriveAntlrReverseFixtureLinks(fixture);
    fixture.links = generated.links;
    fixture.reimportedLinks = generated.reimportedLinks;
    fixture.mutated.links = generated.mutated.links;
    fixture.renamed.links = generated.renamed.links;
  }
  normalizeMigrationAntlrLexerDeclarations(original, result);
  return result;
}
