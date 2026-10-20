// Generic grammar selection over an explicitly matched byte region. The
// existing executor still owns grammar matching, recovery and all budgets.
import { acceptGrammarSelectionCount, acceptGrammarSelectionMapping,
  acceptGrammarSelectionMode, reportGrammarSelectionAmbiguity,
  readGrammarSourceDefaultMatching } from '../translation/grammar-selection-rules.js';
import { DEFAULT_SETTLING, checkGrammarSelectionIncludedRangeForm } from '../grammar-feature-forms.js';
import { readGrammarSelectionMappingContext } from '../translation/grammar-selection-mapping-rules.js';
import { acceptGrammarSelectionIncludedRangeCandidate,
  acceptGrammarSelectionIncludedRangeAgreement, readGrammarSelectionIncludedRangeContext } from '../translation/grammar-selection-included-range-rules.js';
import { acceptGrammarSelectionFieldRangeCandidate,
  readGrammarSelectionFieldRangeContext } from '../translation/grammar-selection-field-range-rules.js';
import { mapGrammarSelectionSource, mapGrammarSelectionIncludedRangeSource, mapGrammarSelectionFieldRangeSource, projectGrammarSelectionSpan,
  projectGrammarSelectionDiagnostic, rememberGrammarSelectionMappingFailure } from './selection-source-mapping.js';
import { KeywordLexing } from './executor.js';

/** Validate the inspectable candidates without resolving unknown grammars. */
export function checkGrammarSelection(expression, fail) {
  if (!acceptGrammarSelectionMode(expression.kind)) return;
  if (!Array.isArray(expression.items) || expression.items.length === 0) fail('a grammar selection needs at least one candidate');
  const names = new Set();
  for (const candidate of expression.items) {
    const ranged = checkGrammarSelectionIncludedRangeForm(candidate, fail);
    if (candidate?.kind !== 'grammar-candidate' && !ranged || typeof candidate.name !== 'string' || candidate.name === '' || typeof candidate.mapping !== 'string') fail('a grammar selection needs named grammar-candidate definitions and an explicit mapping');
    if (names.has(candidate.name)) fail(`duplicate selected grammar ${candidate.name}`);
    names.add(candidate.name);
  }
}

function reportSelectionDiagnostic(executor, diagnostic) {
  diagnostic = projectGrammarSelectionDiagnostic(executor, diagnostic);
  const diagnostics = executor.budget.memory.selectionDiagnostics ??= [];
  if (!diagnostics.some(item => item.reason === diagnostic.reason && item.language === diagnostic.language && item.start === diagnostic.start && item.end === diagnostic.end && item.source === diagnostic.source)) {
    executor.retain(1);
    diagnostics.push(diagnostic);
  }
}

/** Run an existing program with shared budgets and a same-region cycle guard. */
export function runEmbeddedGrammar(executor, program, start, end, language, Executor, bytes = executor.bytes) {
  const active = executor.budget.activeEmbeddedGrammars ??= new Map();
  const sources = active.get(program) ?? new Map();
  const ranges = sources.get(bytes) ?? new Set();
  const range = `${start}|${end}`;
  if (ranges.has(range)) {
    reportSelectionDiagnostic(executor, { reason: 'nonConsumingEmbeddingCycle', language, start, end });
    return null;
  }
  active.set(program, sources);
  sources.set(bytes, ranges);
  ranges.add(range);
  try {
    let expectations = null;
    const keywords = bytes !== executor.bytes && program.tokenRanks && program.settling?.tokens !== false
      ? new KeywordLexing({ ...program.tokenRanks, bytes }, program) : null;
    for (;;) {
      const nested = new Executor(program, bytes, start, end, executor.options, executor.budget, executor.maxDepth - executor.depth);
      if (bytes === executor.bytes) nested.expectations = executor.expectations;
      else if (expectations) nested.expectations = expectations.restart();
      else expectations = nested.expectations;
      if (bytes !== executor.bytes) nested.keywords = keywords;
      const outcome = nested.run(program.start);
      // Separate sources cannot share scanner offset requests. Their own
      // fixed point still uses the original parse's step/memory/depth budget.
      if (bytes === executor.bytes) return outcome;
      if (expectations.stale) continue;
      if (keywords && outcome.ok && keywords.conflicts(outcome.root)) continue;
      return outcome;
    }
  } finally {
    ranges.delete(range);
    if (ranges.size === 0) sources.delete(bytes);
    if (sources.size === 0) active.delete(program);
  }
}

// Every retained identity is source-qualified. Global identities come only from
// the caller's existing concept space; equal rule names create no equivalence.
function canonicalSelectionDefinition(value, executor) {
  executor.step();
  if (Array.isArray(value)) return `[${value.map(item => canonicalSelectionDefinition(item, executor)).join(',')}]`;
  if (value && typeof value === 'object') return `{${Object.keys(value).sort().map(key => `${JSON.stringify(key)}:${canonicalSelectionDefinition(value[key], executor)}`).join(',')}}`;
  return JSON.stringify(value);
}

function readSelectionProvenance(executor, program, language, start, end) {
  executor.retain(program.rules.size);
  const space = executor.options.globalConceptSpace;
  const source = space?.sources.find(item => item.id === language);
  let contextMatches;
  return {
    source: language, language: source?.language ?? language, edition: source?.edition ?? '',
    rules: [...program.rules].map(([name, rule]) => {
      const origin = `${language}:${name}`;
      const original = program.selectionSourceRules?.find(item => item.name === name) ?? (space ? null : rule);
      const concept = space?.concepts.find(item => item.id === space.mappings[origin]);
      const stored = concept?.definitions.find(item => item.origin === origin);
      const storedOrigin = concept?.origins.find(item => item.alias === origin);
      const definition = original ? { kind: original.kind ?? 'normal', expression: original.expression } : null;
      if (definition) for (const key of ['parameters', 'channel', 'modes', 'action']) if (original[key] !== undefined) definition[key] = original[key];
      const sourceMatching = source?.context.matching ?? readGrammarSourceDefaultMatching(source?.sourceFormat ?? '');
      const sourceContext = source ? { ...source.context, matching: sourceMatching, settling: source.context.settling ?? DEFAULT_SETTLING[sourceMatching] } : null;
      const actualContext = { matching: program.matching, settling: program.settling.steps, imports: [], modes: [], extras: [], conflicts: [], precedences: [], macros: [], scanners: [], ...program.selectionSourceDeclarations };
      actualContext.matching ??= program.matching;
      actualContext.settling ??= program.settling.steps;
      const matches = stored && storedOrigin && storedOrigin.concept === (original?.concept ?? null)
        && canonicalSelectionDefinition(stored.definition, executor) === canonicalSelectionDefinition(definition, executor)
        && (contextMatches ??= canonicalSelectionDefinition(sourceContext, executor) === canonicalSelectionDefinition(actualContext, executor));
      if (space && !matches) reportSelectionDiagnostic(executor, { reason: 'unprovedGrammarOrigin', language, start, end, rule: name });
      return { rule: name, origin, concept: matches ? concept.id : null,
        declaredConcept: rule.concept ?? null, sourceNames: (rule.sourceNames ?? []).map(item => ({ ...item })) };
    }),
  };
}

/** Execute any/all selection. Only the primary CST owns bytes during rendering;
 * alternative CSTs remain inspectable parallel witnesses, never concatenated.
 */
export function evaluateGrammarSelection(executor, expression, position, state, inToken, Executor) {
  const mappings = expression.items.map(candidate => candidate.mapping);
  const fieldContext = readGrammarSelectionFieldRangeContext(mappings, inToken);
  // Only the generated field profile leaves leading trivia to its actual
  // syntactic item. This does not change that item to token context.
  const { end: start, leaves, state: afterTrivia } = executor.terminalStart(position, state, inToken || fieldContext === 1);
  const rangeContext = fieldContext === 0 ? readGrammarSelectionIncludedRangeContext(mappings, inToken) : 0;
  const context = fieldContext !== 0 ? fieldContext
    : rangeContext === 0 ? readGrammarSelectionMappingContext(mappings, inToken) : rangeContext;
  if (context === 2) {
    reportSelectionDiagnostic(executor, { reason: 'unsupportedEmbeddingMapping', start, end: start,
      mapping: fieldContext === 2 ? 'field-ranges' : rangeContext === 2 ? 'included-ranges' : 'json-string', detail: 'mapped selection requires syntactic context and one common source mapping' });
    return [];
  }
  const fielded = fieldContext === 1;
  let selector = null;
  if (fielded) {
    for (const candidate of expression.items) {
      if (!acceptGrammarSelectionFieldRangeCandidate(candidate.kind, candidate.mapping)) {
        reportSelectionDiagnostic(executor, { reason: 'unsupportedEmbeddingMapping', start, end: start,
          mapping: 'field-ranges', detail: 'field ranges require an explicit grammar-field-candidate' });
        return [];
      }
      executor.step();
      executor.retain(5);
      const current = candidate.selector;
      const parameters = { kind: current.kind, label: current.label, mode: current.mode,
        first: current.first === 0 ? 0 : current.first, count: current.count };
      if (selector && (selector.kind !== parameters.kind || selector.label !== parameters.label
          || selector.mode !== parameters.mode || selector.first !== parameters.first || selector.count !== parameters.count)) {
        reportSelectionDiagnostic(executor, { reason: 'unsupportedEmbeddingMapping', start, end: start,
          mapping: 'field-ranges', detail: 'field range candidates require one common selector' });
        return [];
      }
      selector ??= parameters;
    }
  }
  const ranged = rangeContext === 1;
  let ranges = null;
  if (ranged) {
    if (expression.items.some(candidate => !acceptGrammarSelectionIncludedRangeCandidate(candidate.kind, candidate.mapping))) {
      reportSelectionDiagnostic(executor, { reason: 'unsupportedEmbeddingMapping', start, end: start,
        mapping: 'included-ranges', detail: 'included ranges require an explicit grammar-range-candidate' });
      return [];
    }
    for (const candidate of expression.items) {
      executor.retain(candidate.ranges.length * 2 + 1);
      // Grammar integer fields have one zero representation in the native
      // form. Dynamic source-plane Numbers retain their separate signed zero.
      const parameters = candidate.ranges.map(range => { executor.step(); return [range.start === 0 ? 0 : range.start, range.end]; });
      if (ranges && !acceptGrammarSelectionIncludedRangeAgreement(ranges.flat(), parameters.flat())) {
        reportSelectionDiagnostic(executor, { reason: 'unsupportedEmbeddingMapping', start, end: start,
          mapping: 'included-ranges', detail: 'included range candidates require one common ordered range plan' });
        return [];
      }
      ranges ??= parameters;
    }
  }
  const mapped = context === 1;
  // Identity keeps its exact historic token-context behavior. The NEW mapping
  // matches syntactically so sourceChildren are the actual host CST.
  const regions = executor.evaluate(expression.item, start, afterTrivia, !mapped);
  const results = [];
  for (const region of regions) {
    const accepted = [];
    const candidates = [];
    const derived = fielded ? mapGrammarSelectionFieldRangeSource(executor, start, region.end, selector, region.children)
      : ranged ? mapGrammarSelectionIncludedRangeSource(executor, start, region.end, ranges)
      : mapped ? mapGrammarSelectionSource(executor, start, region.end) : null;
    if (derived?.reason) {
      rememberGrammarSelectionMappingFailure(executor, { reason: derived.reason, start: derived.offset, end: derived.offset,
        mapping: fielded ? 'field-ranges' : ranged ? 'included-ranges' : 'json-string' });
      executor.fail(derived.offset, fielded ? 'a scalar-aligned field-selected source range view'
        : ranged ? 'an ordered scalar-aligned source range view' : 'a Unicode scalar JSON string source');
      continue;
    }
    for (const candidate of expression.items) {
      executor.step();
      executor.retain(1);
      const diagnostic = { language: candidate.name, start, end: region.end };
      if (!fielded && !ranged && !acceptGrammarSelectionMapping(candidate.mapping)) {
        const reason = 'unsupportedEmbeddingMapping';
        reportSelectionDiagnostic(executor, { ...diagnostic, reason, mapping: candidate.mapping });
        candidates.push({ language: candidate.name, relation: 'unknown', reason, mapping: candidate.mapping });
        continue;
      }
      const program = executor.program.selectLanguage(candidate.name);
      if (!program) {
        const reason = 'unknownGrammar';
        reportSelectionDiagnostic(executor, { ...diagnostic, reason });
        candidates.push({ language: candidate.name, relation: 'unknown', reason });
        continue;
      }
      const key = JSON.stringify(['selection', candidate.mapping, candidate.name, start, region.end,
        ...(fielded ? [selector] : ranged ? [ranges] : []), ...(derived ? [derived.mapping.output] : [])]);
      let outcome = executor.embedMemo.get(key);
      if (!outcome) {
        outcome = derived ? runEmbeddedGrammar(executor, program, 0, derived.bytes.length, candidate.name, Executor, derived.bytes)
          : runEmbeddedGrammar(executor, program, start, region.end, candidate.name, Executor);
        if (outcome) executor.embedMemo.set(key, outcome);
      }
      if (!outcome) {
        candidates.push({ language: candidate.name, relation: 'unknown', reason: 'nonConsumingEmbeddingCycle' });
        continue;
      }
      if (!outcome.ok) {
        const cover = derived ? projectGrammarSelectionSpan(derived.mapping, outcome.farthest, outcome.farthest) : null;
        const offset = cover ? cover.displayAnchor : outcome.farthest;
        candidates.push({ language: candidate.name, relation: 'rejected', offset, expected: [...outcome.expected],
          ...(derived ? { source: derived.mapping.output, sourceOffset: outcome.farthest,
            physicalRange: [cover.start, cover.end], offsetSemantics: 'covering-source-spelling' } : {}) });
        if (derived) rememberGrammarSelectionMappingFailure(executor, { reason: 'mappedSyntax', language: candidate.name,
          start: cover.start, end: cover.end, source: derived.mapping.output, sourceStart: outcome.farthest,
          sourceEnd: outcome.farthest, expected: [...outcome.expected], offsetSemantics: 'covering-source-spelling',
          sourceRoute: [{ output: derived.mapping.output, outputStart: outcome.farthest, outputEnd: outcome.farthest,
            input: derived.mapping.input, inputStart: cover.start, inputEnd: cover.end, displayAnchor: offset }] });
        for (const expectation of outcome.expected) executor.fail(offset, expectation);
        continue;
      }
      const provenance = readSelectionProvenance(executor, program, candidate.name, start, region.end);
      candidates.push({ language: candidate.name, relation: 'accepted' });
      accepted.push({ language: candidate.name, root: outcome.root, program, provenance });
    }
    if (!acceptGrammarSelectionCount(expression.kind, accepted.length, expression.items.length)) continue;
    const primary = accepted[0];
    const selection = { mode: expression.kind, mapping: mapped ? expression.items[0].mapping : 'identity', start, end: region.end,
      candidates, provenance: primary.provenance, alternatives: accepted.slice(1) };
    if (derived) {
      selection.sourceMapping = derived.mapping;
      selection.sourceBytes = derived.bytes;
    }
    const child = { type: 'embed', language: primary.language, start, end: region.end,
      root: primary.root, program: primary.program, selection };
    if (derived) child.sourceChildren = region.children;
    const ambiguous = reportGrammarSelectionAmbiguity(expression.kind, accepted.length);
    child.selection.ambiguous = ambiguous;
    results.push({ end: region.end, state: region.state, children: inToken ? [] : [...leaves, child],
      dynamic: region.dynamic, precedence: null, tail: null, ambiguous: ambiguous || (mapped && region.ambiguous), cost: region.cost });
  }
  return results;
}

/** Copy only public metadata; executable programs never enter a stored CST. */
export function publicGrammarSelection(selection, bytes, copyTree) {
  const { sourceBytes, ...metadata } = selection;
  return { ...metadata,
    candidates: selection.candidates.map(item => ({ ...item, ...(item.expected ? { expected: [...item.expected] } : {}) })),
    provenance: { ...selection.provenance, rules: selection.provenance.rules.map(item => ({ ...item, sourceNames: item.sourceNames.map(alias => ({ ...alias })) })) },
    alternatives: selection.alternatives.map(({ language, root, provenance }) => ({ language, root: copyTree(root, sourceBytes ?? bytes), provenance })),
  };
}

/** Store the explicit root invocation as the same concrete grammar boundary.
 * The original root is retained whole; legacy parses acquire no wrapper.
 */
export function qualifyGrammarRoot(executor, root, language) {
  const start = executor.begin, end = executor.end;
  const provenance = readSelectionProvenance(executor, executor.program, language, start, end);
  return { type: 'embed', language, start, end, root, program: executor.program,
    selection: { mode: 'grammar-root', mapping: 'identity', start, end,
      candidates: [{ language, relation: 'accepted' }], provenance, alternatives: [], ambiguous: false } };
}
