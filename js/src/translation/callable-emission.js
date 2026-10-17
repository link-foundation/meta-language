// Retain original checked identities while materializing generated effect plans.
import { createSurfaceReferenceFacts } from './surface-reference-facts.js';
import { readCallableEffectfulDeclarations, planCallableOutputObligations } from './callable-emission-rules.js';

export function readCallableOutputAnalysis(program) {
  const input = { declarations: [...program.declarations.values()], main: program.main };
  const facts = createSurfaceReferenceFacts(input, { retainValues: true });
  const columns = [facts.forms, facts.texts, facts.starts, facts.counts, facts.labels, facts.targets];
  let declarations = -1;
  for (let at = facts.starts[facts.root]; at < facts.starts[facts.root] + facts.counts[facts.root]; at += 1) {
    if (facts.labels[at] === 'declarations') declarations = facts.targets[at];
  }
  const selected = readCallableEffectfulDeclarations(...columns, declarations);
  return {
    effectful: new Set(selected.map(identity => facts.values[identity].fullName)),
    obligations: planCallableOutputObligations(...columns, facts.root, selected).map(identity => facts.values[identity]),
  };
}
