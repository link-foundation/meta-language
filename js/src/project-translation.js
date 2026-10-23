// One complete supplied source inventory, shared dependency contexts and an
// independent semantic pass. This drafts products; it executes no product.
import { createHash } from 'node:crypto';
import { isDeepStrictEqual } from 'node:util';
import { selfTranslate, selfTranslationLanguage, selfTranslationSignatures } from './self-translation.js';
import { encodeStructuredDataLinks } from './structured-data-links.js';

const fail = message => { throw new TypeError(`project translation: ${message}`); };
const digest = source => createHash('sha256').update(source, 'utf8').digest('hex');
const compare = (first, second) => first < second ? -1 : first > second ? 1 : 0;

function sourceRecords(inputs, options) {
  if (!Array.isArray(inputs) || inputs.length === 0 || inputs.length > (options.moduleLimit ?? 10000)) fail('expected a bounded nonempty complete module inventory');
  const seen = new Set();
  let bytes = 0;
  const records = inputs.map(input => {
    if (!input || typeof input.identifier !== 'string' || !input.identifier.length
      || typeof input.source !== 'string' || !input.source.isWellFormed()
      || seen.has(input.identifier)) fail('invalid or duplicate source module');
    seen.add(input.identifier);
    const language = selfTranslationLanguage(input.language);
    if (!language) fail(`unknown source language for ${input.identifier}`);
    bytes += Buffer.byteLength(input.source);
    if (bytes > (options.byteLimit ?? 128 * 1024 * 1024)) fail('source byte resource limit exceeded');
    const imports = Object.entries(input.imports ?? {}).map(([specifier, identifier]) => {
      if (typeof identifier !== 'string' || !identifier.length) fail('invalid imported source identity');
      return { specifier, identifier };
    });
    return { identifier: input.identifier, language, source: input.source,
      sourceSha256: digest(input.source), sourceBytes: Buffer.byteLength(input.source), imports,
      options: input.options ?? {} };
  }).sort((first, second) => compare(first.identifier, second.identifier));
  for (const record of records) for (const dependency of record.imports) {
    if (!seen.has(dependency.identifier)) fail(`missing source ${dependency.identifier} imported by ${record.identifier}`);
  }
  return records;
}

function diagnostic(error) {
  return { name: error.name, message: error.message,
    ...(typeof error.kind === 'string' ? { kind: error.kind } : {}),
    ...(typeof error.reason === 'string' ? { reason: error.reason } : {}),
    ...(error.span ? { span: error.span } : {}) };
}

function context(record, signatures, options) {
  const imports = { ...(options.externalImports?.[record.identifier] ?? {}) };
  for (const dependency of record.imports) {
    if (Object.hasOwn(imports, dependency.specifier)) fail('an internal source dependency cannot be replaced by an external signature');
    imports[dependency.specifier] = signatures.get(dependency.identifier);
  }
  return { ...options.translationOptions, ...record.options, imports,
    semanticTranslation: true, retainDiagnostics: true };
}

/** Draft every supplied module, retaining all failures and dependency passes.
 * No native compiler, product execution, oracle or source restoration runs.
 * Import signatures come only from ordinary source translation, never from a
 * fabricated cross-module type or a declaration that failed translation.
 */
export function draftProjectTranslation(inputs, options = {}) {
  const targetLanguage = selfTranslationLanguage(options.targetLanguage);
  if (!targetLanguage) fail('an explicit supported target language is required');
  for (const name of ['moduleLimit', 'byteLimit', 'refinementLimit']) {
    if (options[name] !== undefined && (!Number.isSafeInteger(options[name]) || options[name] < 1)) fail('invalid drafting resource limit');
  }
  const records = sourceRecords(inputs, options);
  let signatures = new Map(records.map(record => [record.identifier, []]));
  const dependents = new Map(records.map(record => [record.identifier, new Set()]));
  for (const record of records) for (const dependency of record.imports) dependents.get(dependency.identifier).add(record.identifier);
  let pending = new Set(records.map(record => record.identifier));
  const passes = [];
  const limit = options.refinementLimit ?? records.length + 1;
  let stable = false;
  for (let pass = 0; pass < limit; pass += 1) {
    const next = new Map(signatures), refusals = [];
    for (const record of records) {
      if (!pending.has(record.identifier)) continue;
      try {
        next.set(record.identifier, selfTranslationSignatures(record.source, record.language, context(record, signatures, options)));
      } catch (error) {
        next.set(record.identifier, []);
        refusals.push({ identifier: record.identifier, diagnostic: diagnostic(error) });
      }
    }
    const changed = records.filter(record => !isDeepStrictEqual(signatures.get(record.identifier), next.get(record.identifier)))
      .map(record => record.identifier);
    passes.push({ pass, changed, refusals });
    signatures = next;
    if (changed.length === 0) { stable = true; break; }
    pending = new Set(changed.flatMap(identifier => [...dependents.get(identifier)]));
    if (pending.size === 0) { stable = true; break; }
  }
  const modules = records.map(record => {
    const base = { identifier: record.identifier, sourceLanguage: record.language, targetLanguage,
      sourceSha256: record.sourceSha256, sourceBytes: record.sourceBytes,
      imports: record.imports, signatures: signatures.get(record.identifier) };
    try {
      const translation = selfTranslate(record.source, record.language, targetLanguage, context(record, signatures, options));
      const sameFamily = record.language === targetLanguage
        || record.language === 'JavaScript' && targetLanguage === 'TypeScript';
      const gaps = translation.items.filter(item => !['translated', 'comment', 'provenance'].includes(item.status)
        && !(sameFamily && item.status === 'kept'));
      if (!stable) gaps.push({ diagnostic: { name: 'DependencyRefinementGap', message: 'Source signature refinement did not converge within the explicit budget.' } });
      return { ...base, items: translation.items, gaps,
        // Partial carried envelopes are retained as diagnostics. A consumer
        // receives executable output only when every actual owner translated.
        emittedCode: gaps.length === 0 ? translation.code : null,
        draftCode: translation.code, emittedSha256: digest(translation.code) };
    } catch (error) {
      return { ...base, items: [], gaps: [{ diagnostic: diagnostic(error) }], emittedCode: null, draftCode: null, emittedSha256: null };
    }
  });
  return { schemaVersion: 1, interpretation: 'independent source semantic translation',
    targetLanguage, inventory: records.map(({ source, options: configuration, ...record }) => record),
    dependencyRefinement: { stable, limit, passes }, modules };
}

/** The draft inventory, actual products and complete refusal diagnostics are
 * ordinary inspectable links. No executed observation is recorded here.
 */
export function projectTranslationDraftToLinks(draft, options = {}) {
  return encodeStructuredDataLinks(draft, options);
}
