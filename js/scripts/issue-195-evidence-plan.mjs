/**
 * Selects the independently executable evidence group that owns a verification
 * cell. The runner only emits a result after this group and its prerequisites
 * have completed successfully.
 */
export function evidenceGroupFor(requirement, cell) {
  if (requirement.area === 'runtime-parity' || requirement.area === 'shared-concepts') {
    return 'runtime-parity';
  }
  if (requirement.area === 'native-validation') {
    return `native:${requirement.scope.language}:${cell.runtime}`;
  }
  if (requirement.area === 'directed-translation') {
    const target = requirement.scope.language.split(' -> ')[1];
    return `translation:${target}:${cell.runtime}`;
  }
  if (requirement.area === 'package-delivery') {
    if (requirement.id.includes('-NPM-')) return 'delivery:npm';
    if (requirement.id.includes('-CRATE-')) return 'delivery:crate';
    if (requirement.id.includes('-RML-')) return 'delivery:rml';
    throw new Error(`unknown package-delivery requirement ${requirement.id}`);
  }
  return `suite:${cell.runtime}`;
}

export function buildEvidencePlan(manifest, checkpoint = 'pre-merge') {
  if (!['pre-merge', 'release-delivery'].includes(checkpoint)) {
    throw new Error(`unknown evidence checkpoint: ${checkpoint}`);
  }
  const groups = new Map();
  for (const requirement of manifest.atomicRequirements) {
    if (requirement.area === 'acceptance-gate-fault-injection') continue;
    for (const cell of requirement.verifications) {
      if (cell.checkpoint !== checkpoint) continue;
      const group = evidenceGroupFor(requirement, cell);
      const cells = groups.get(group) ?? [];
      cells.push({ requirement, cell });
      groups.set(group, cells);
    }
  }
  return [...groups]
    .sort(([left], [right]) => left.localeCompare(right))
    .map(([group, cells]) => ({ group, cells }));
}

/** Counts only callbacks emitted by executed tests for the exact cell and fixtures. */
export function observedEvidenceForCell(cell, records) {
  const executionRecords = records.filter(({ testId }) => testId === cell.testId);
  const passed = new Set(executionRecords
    .filter(({ outcome }) => outcome === 'passed')
    .map(({ assertionId, fixtureId }) => `${assertionId}\u0000${fixtureId}`));
  const assertionsPassed = cell.assertions.filter((assertionId) =>
    cell.fixtureIds.every((fixtureId) => passed.has(`${assertionId}\u0000${fixtureId}`))
  );
  return {
    executionRecords,
    assertionsPassed,
    complete: assertionsPassed.length === cell.assertions.length,
  };
}
