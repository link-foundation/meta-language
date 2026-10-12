#!/usr/bin/env node
// Restore the graph sharing produced by nativeGrammarConceptReuse without
// reading, compiling or executing any grammar. Legacy JSON snapshots retained
// the values of language lists, but could not retain their object identities.
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { readProjectData, resolveProjectDataPath, serializeProjectData, writeProjectData } from './project-data.mjs';

const fixture = new URL('../../parity/fixtures/native-grammar-concept-reuse.json', import.meta.url);
const generatedBy = 'js/scripts/build-native-grammar-concept-reuse.mjs';
const fail = message => { throw new TypeError(`native concept reuse normalization: ${message}`); };

export function normalizeNativeConceptReuse(report) {
  if (!report || report.generatedBy !== generatedBy || !Array.isArray(report.grammars) || !Array.isArray(report.concepts)) fail('foreign report shape');
  const concepts = new Map();
  for (const entry of report.concepts) {
    if (typeof entry.concept !== 'string' || concepts.has(entry.concept) || !Array.isArray(entry.languages)
      || entry.languages.some(language => typeof language !== 'string')) fail('invalid canonical concept language list');
    concepts.set(entry.concept, entry.languages);
  }
  const grammars = new Set();
  const normalized = { ...report, grammars: report.grammars.map(grammar => {
    if (typeof grammar.grammar !== 'string' || grammars.has(grammar.grammar) || !Array.isArray(grammar.shared)) fail('invalid grammar identity or shared rules');
    grammars.add(grammar.grammar);
    return { ...grammar, shared: grammar.shared.map(rule => {
      const languages = concepts.get(rule.concept);
      if (!languages || !Array.isArray(rule.languages) || languages.length !== rule.languages.length
        || languages.some((language, index) => language !== rule.languages[index])) fail(`language list differs for ${grammar.grammar}:${rule.rule}`);
      return { ...rule, languages };
    }) };
  }) };
  if (JSON.stringify(normalized) !== JSON.stringify(report)) fail('normalization changed an ordered report value');
  return normalized;
}

export function normalizeNativeConceptReuseFile(file = fixture, { check = false } = {}) {
  const report = readProjectData(file);
  const normalized = normalizeNativeConceptReuse(report);
  const expected = serializeProjectData(normalized, file);
  const physical = resolveProjectDataPath(file);
  if (check) {
    if (readFileSync(physical, 'utf8') !== expected) fail('the fixture does not retain canonical language list identities');
  } else writeProjectData(file, normalized);
  return { grammars: report.grammars.length, concepts: report.concepts.length,
    sharedRules: report.grammars.reduce((count, grammar) => count + grammar.shared.length, 0) };
}

if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  const result = normalizeNativeConceptReuseFile(fixture, { check: process.argv.includes('--check') });
  console.log(`canonical concept language lists retained for ${result.grammars} grammars, ${result.concepts} concepts and ${result.sharedRules} shared rules`);
}
