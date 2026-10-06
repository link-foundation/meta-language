// Proof obligation gate of issue #195: the discharge every obligation of a
// translation must carry. Into a proof target (Lean, Rocq) each theorem and
// assertion is discharged by the target kernel itself, with no bounded check
// standing in for it; into a program target (JavaScript, Rust) a theorem stays
// proved by the source kernel and is only checked on a bounded domain, and an
// assertion is a runtime assertion. A bounded check relabelled as a proof, or
// a proof obligation relabelled as a bounded check, is reported.
export const PROOF_OBLIGATION_FIELDS = Object.freeze(['check', 'closedGoal', 'discharge', 'kind', 'source', 'target']);

const PROGRAM_DISCHARGE = Object.freeze({
  theorem: Object.freeze({ discharge: 'source-kernel', check: 'bounded' }),
  assertion: Object.freeze({ discharge: 'runtime-assertion', check: null }),
});

/**
 * Problems of the obligations of one translation; `proofTarget` tells whether
 * the target language has a kernel that checks proofs.
 */
export function proofObligationProblems(translation, { proofTarget }) {
  const { sourceLanguage, targetLanguage } = translation;
  const problems = [];
  for (const obligation of translation.semantics?.obligations ?? []) {
    const label = `${sourceLanguage} -> ${targetLanguage} ${obligation.kind} ${obligation.source}`;
    const fields = Object.keys(obligation).sort();
    if (fields.join() !== PROOF_OBLIGATION_FIELDS.join()) problems.push(`${label} has fields ${fields.join(', ')}`);
    if (!Object.hasOwn(PROGRAM_DISCHARGE, obligation.kind)) {
      problems.push(`${label} has unknown kind`);
      continue;
    }
    const expected = proofTarget ? { discharge: 'target-kernel', check: null } : PROGRAM_DISCHARGE[obligation.kind];
    if (obligation.discharge !== expected.discharge || obligation.check !== expected.check) {
      problems.push(`${label} is discharged by ${obligation.discharge} with check ${obligation.check}, ` +
        `not by ${expected.discharge} with check ${expected.check}`);
    }
  }
  return problems;
}
