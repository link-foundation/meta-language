import { originalJsonSchema } from '../grammar-importers/json-schema.js';
import { emitReport } from './common.js';

const maximumAlternatives = 1024;
const maximumSourceLength = 65_536;
const maximumExpansion = 4096;

function combine(prefixes, suffixes) {
  if (prefixes.size * suffixes.size > maximumAlternatives) return null;
  const joined = new Set();
  for (const prefix of prefixes) {
    for (const suffix of suffixes) {
      if (prefix.length + suffix.length > maximumSourceLength) return null;
      joined.add(prefix + suffix);
    }
  }
  return joined;
}

function finiteLanguage(grammar) {
  const active = new Set();
  const cache = new Map();
  let remaining = maximumExpansion;
  function resolve(name) {
    if (cache.has(name)) return cache.get(name);
    const rule = grammar.rule(name);
    if (!rule || active.has(name) || rule.modes?.length || rule.parameters?.length || rule.action) return null;
    active.add(name);
    const language = expand(rule.expression);
    active.delete(name);
    cache.set(name, language);
    return language;
  }
  function expand(expression) {
    remaining -= 1;
    if (remaining < 0 || active.size > 128) return null;
    switch (expression.kind) {
      case 'empty':
        return new Set(['']);
      case 'literal':
        return expression.value.length <= maximumSourceLength ? new Set([expression.value]) : null;
      case 'ref':
        return resolve(expression.name);
      case 'seq': {
        let parts = new Set(['']);
        for (const item of expression.items) {
          const suffixes = expand(item);
          parts = suffixes && combine(parts, suffixes);
          if (!parts) return null;
        }
        return parts;
      }
      case 'choice': {
        if (expression.ordered) return null;
        const choices = new Set();
        for (const item of expression.items) {
          const alternatives = expand(item);
          if (!alternatives) return null;
          for (const alternative of alternatives) choices.add(alternative);
          if (choices.size > maximumAlternatives) return null;
        }
        return choices;
      }
      case 'repeat':
      case 'optional': {
        const min = expression.kind === 'optional' ? 0 : expression.min;
        const max = expression.kind === 'optional' ? 1 : expression.max;
        if (!Number.isSafeInteger(max) || max > 64) return null;
        const parts = expand(expression.item);
        if (!parts) return null;
        let strings = new Set(['']);
        const choices = new Set();
        for (let count = 0; count <= max; count += 1) {
          if (count >= min) for (const item of strings) choices.add(item);
          if (choices.size > maximumAlternatives) return null;
          if (count < max) strings = combine(strings, parts);
          if (!strings) return null;
        }
        return choices;
      }
      case 'alias':
      case 'capture':
      case 'token':
      case 'immediateToken':
        return expand(expression.item);
      default:
        return null;
    }
  }
  return resolve(grammar.startRule()?.name);
}

/** Emit the original schema or infer semantic constants from finite JSON productions. */
export function emitJsonSchema(grammar) {
  const report = emitReport();
  const original = originalJsonSchema(grammar);
  if (original !== undefined) return { schema: original, report };
  const hasDeclarations = Object.entries(grammar.declarations).some(([name, value]) => (
    name !== 'matching' && (Array.isArray(value) ? value.length > 0 : value !== undefined)
  ));
  const language = hasDeclarations ? null : finiteLanguage(grammar);
  const values = new Map();
  if (language) {
    for (const text of language) {
      try {
        const value = JSON.parse(text);
        values.set(JSON.stringify(value), value);
      } catch {
        report.lossy.push('The grammar includes productions outside the JSON document language.');
        return { schema: null, report };
      }
    }
    report.lossy.push('The schema represents parsed JSON values. Grammar-specific spellings and trivia are omitted.');
    if (values.size === 1) return { schema: { const: values.values().next().value }, report };
    if (values.size > 1) return { schema: { enum: [...values.values()] }, report };
  }
  report.lossy.push('A JSON Schema could not be inferred from the grammar productions.');
  return { schema: null, report };
}
