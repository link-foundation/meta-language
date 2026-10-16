// Generic grammar selection over an explicitly matched byte region. The
// existing executor still owns grammar matching, recovery and all budgets.
import { acceptGrammarSelectionCount, acceptGrammarSelectionMapping,
  acceptGrammarSelectionMode, reportGrammarSelectionAmbiguity } from '../translation/grammar-selection-rules.js';
import { DEFAULT_SETTLING } from '../grammar-feature-forms.js';

/** Validate the inspectable candidates without resolving unknown grammars. */
export function checkGrammarSelection(expression, fail) {
  if (!acceptGrammarSelectionMode(expression.kind)) return;
  if (!Array.isArray(expression.items) || expression.items.length === 0) fail('a grammar selection needs at least one candidate');
  const names = new Set();
  for (const candidate of expression.items) {
    if (candidate?.kind !== 'grammar-candidate' || typeof candidate.name !== 'string' || candidate.name === '' || typeof candidate.mapping !== 'string') fail('a grammar selection needs named grammar-candidate definitions and an explicit mapping');
    if (names.has(candidate.name)) fail(`duplicate selected grammar ${candidate.name}`);
    names.add(candidate.name);
  }
}

function reportSelectionDiagnostic(executor, diagnostic) {
  const diagnostics = executor.budget.memory.selectionDiagnostics ??= [];
  if (!diagnostics.some(item => item.reason === diagnostic.reason && item.language === diagnostic.language && item.start === diagnostic.start && item.end === diagnostic.end)) {
    executor.retain(1);
    diagnostics.push(diagnostic);
  }
}

/** Run an existing program with shared budgets and a same-region cycle guard. */
export function runEmbeddedGrammar(executor, program, start, end, language, Executor) {
  const active = executor.budget.activeEmbeddedGrammars ??= new Map();
  const ranges = active.get(program) ?? new Set();
  const range = `${start}|${end}`;
  if (ranges.has(range)) {
    reportSelectionDiagnostic(executor, { reason: 'nonConsumingEmbeddingCycle', language, start, end });
    return null;
  }
  active.set(program, ranges);
  ranges.add(range);
  try {
    const nested = new Executor(program, executor.bytes, start, end, executor.options, executor.budget, executor.maxDepth - executor.depth);
    nested.expectations = executor.expectations;
    return nested.run(program.start);
  } finally {
    ranges.delete(range);
    if (ranges.size === 0) active.delete(program);
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
      const sourceMatching = source?.context.matching ?? 'generalized';
      const sourceContext = source ? { ...source.context, matching: sourceMatching, settling: source.context.settling ?? DEFAULT_SETTLING[sourceMatching] } : null;
      const actualContext = { matching: program.matching, settling: program.settling.steps, imports: [], modes: [], extras: [], conflicts: [], precedences: [], macros: [], scanners: [], ...program.selectionSourceDeclarations };
      actualContext.matching ??= program.matching;
      actualContext.settling ??= program.settling.steps;
      const matches = stored && storedOrigin && storedOrigin.concept === (original?.concept ?? null)
        && canonicalSelectionDefinition(stored.definition, executor) === canonicalSelectionDefinition(definition, executor)
        && canonicalSelectionDefinition(sourceContext, executor) === canonicalSelectionDefinition(actualContext, executor);
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
  const { end: start, leaves, state: afterTrivia } = executor.terminalStart(position, state, inToken);
  const regions = executor.evaluate(expression.item, start, afterTrivia, true);
  const results = [];
  for (const region of regions) {
    const accepted = [];
    const candidates = [];
    for (const candidate of expression.items) {
      executor.step();
      executor.retain(1);
      const diagnostic = { language: candidate.name, start, end: region.end };
      if (!acceptGrammarSelectionMapping(candidate.mapping)) {
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
      const key = `selection|${candidate.name}|${start}|${region.end}`;
      let outcome = executor.embedMemo.get(key);
      if (!outcome) {
        outcome = runEmbeddedGrammar(executor, program, start, region.end, candidate.name, Executor);
        if (outcome) executor.embedMemo.set(key, outcome);
      }
      if (!outcome) {
        candidates.push({ language: candidate.name, relation: 'unknown', reason: 'nonConsumingEmbeddingCycle' });
        continue;
      }
      if (!outcome.ok) {
        candidates.push({ language: candidate.name, relation: 'rejected', offset: outcome.farthest, expected: [...outcome.expected] });
        for (const expectation of outcome.expected) executor.fail(outcome.farthest, expectation);
        continue;
      }
      const provenance = readSelectionProvenance(executor, program, candidate.name, start, region.end);
      candidates.push({ language: candidate.name, relation: 'accepted' });
      accepted.push({ language: candidate.name, root: outcome.root, program, provenance });
    }
    if (!acceptGrammarSelectionCount(expression.kind, accepted.length, expression.items.length)) continue;
    const primary = accepted[0];
    const selection = { mode: expression.kind, mapping: 'identity', start, end: region.end,
      candidates, provenance: primary.provenance, alternatives: accepted.slice(1) };
    const child = { type: 'embed', language: primary.language, start, end: region.end,
      root: primary.root, program: primary.program, selection };
    const ambiguous = reportGrammarSelectionAmbiguity(expression.kind, accepted.length);
    child.selection.ambiguous = ambiguous;
    results.push({ end: region.end, state: region.state, children: inToken ? [] : [...leaves, child],
      dynamic: region.dynamic, precedence: null, tail: null, ambiguous, cost: region.cost });
  }
  return results;
}

/** Copy only public metadata; executable programs never enter a stored CST. */
export function publicGrammarSelection(selection, bytes, copyTree) {
  return { ...selection,
    candidates: selection.candidates.map(item => ({ ...item, ...(item.expected ? { expected: [...item.expected] } : {}) })),
    provenance: { ...selection.provenance, rules: selection.provenance.rules.map(item => ({ ...item, sourceNames: item.sourceNames.map(alias => ({ ...alias })) })) },
    alternatives: selection.alternatives.map(({ language, root, provenance }) => ({ language, root: copyTree(root, bytes), provenance })),
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
