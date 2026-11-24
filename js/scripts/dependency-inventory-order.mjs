// Declaration refreshes replace facts by identity and append new identities.
import assert from 'node:assert/strict';

const directRuntimeRequirements = new Map([
  ['crate rust/Cargo.lock base64@0.23.1', '0.23.1'],
  ['crate rust/Cargo.lock crc32fast@1.5.2', '1.5.2'],
  ['crate rust/Cargo.lock miniz_oxide@0.9.1', '0.9.1'],
]);

/** Restore the three transitive declarations in the certified PDF predecessors. */
export function restoreTransitivePdfDependency(current, historical) {
  assert.equal(current.id, historical.id);
  const requirement = directRuntimeRequirements.get(historical.id);
  if (requirement === undefined || current.role === 'transitive') return current;
  const { declaredIn, role, kind, requirement: actualRequirement, ...facts } = current;
  assert.deepEqual({ declaredIn, role, kind, requirement: actualRequirement }, {
    declaredIn: ['rust/Cargo.lock', 'rust/Cargo.toml'], role: 'direct', kind: 'runtime', requirement,
  });
  assert.deepEqual(historical.declaredIn, ['rust/Cargo.lock']);
  assert.equal(historical.role, 'transitive');
  assert.equal(historical.kind, undefined);
  assert.equal(historical.requirement, undefined);
  const restored = { ...facts, declaredIn: historical.declaredIn, role: historical.role };
  assert.deepEqual(restored, historical);
  return Object.fromEntries(Object.keys(historical).map(key => [key, restored[key]]));
}

/** Replay the original declarations after checking the exact current records. */
export function restoreDependencyDeclaration(current, historical) {
  if (current.id !== 'action dtolnay/rust-toolchain@1.99.0') {
    return restoreTransitivePdfDependency(current, historical);
  }
  const originalWorkflows = ['.github/workflows/ci.yml', '.github/workflows/dependency-refresh.yml'];
  assert.deepEqual(historical.declaredIn, originalWorkflows);
  if (current.declaredIn.length === originalWorkflows.length) return current;
  assert.deepEqual(current.declaredIn, [...originalWorkflows, '.github/workflows/rust.yml']);
  const original = { ...current, declaredIn: originalWorkflows };
  assert.deepEqual(original, historical);
  return original;
}

/** Preserve existing record order without retaining absent or obsolete facts. */
export function retainDependencyInventoryOrder(previous,current) {
  const before=new Set(previous.items.map(row=>row.id));
  const actual=new Map(current.items.map(row=>[row.id,row]));
  assert.equal(before.size,previous.items.length,'duplicate previous dependency identities');
  assert.equal(actual.size,current.items.length,'duplicate current dependency identities');
  return{...current,items:[
    ...previous.items.filter(row=>actual.has(row.id)).map(row=>actual.get(row.id)),
    ...current.items.filter(row=>!before.has(row.id)),
  ]};
}
