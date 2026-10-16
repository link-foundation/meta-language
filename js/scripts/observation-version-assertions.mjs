// Declared observation identities determine whether context is mandatory.
// Neither the incoming record nor its testName can remove that obligation.
import path from 'node:path';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { isDeepStrictEqual } from 'node:util';
import { readAssertionVersionPolicy, selectAntlrReferenceAssertion } from './antlr-version-assertions.mjs';
import { assertionVersionPolicy } from './generate-assertion-version-policy.mjs';
import { selectPeggyOptimizerAssertion } from './peggy-version-assertions.mjs';

const repository = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../..');
const observationIdentity = record => `${record.testId}\u0000${record.assertionId}\u0000${record.fixtureId}`;

export function createObservationVersionValidator({ root = repository } = {}) {
  const registry = readAssertionVersionPolicy(root);
  if (!isDeepStrictEqual(registry.policy, assertionVersionPolicy())) throw new Error('observation version bindings and ranges differ from their registered authoring source');
  if (!Array.isArray(registry.policy.observationBindings)) throw new Error('explicit observation version bindings are required');
  const bindings = new Map();
  for (const binding of registry.policy.observationBindings) {
    if (!binding || !['testId', 'assertionId', 'fixtureId', 'fixtureFile', 'assertionPolicy'].every(field => typeof binding[field] === 'string' && binding[field])) {
      throw new Error('observation version binding identities must be explicit');
    }
    const key = observationIdentity(binding);
    if (bindings.has(key)) throw new Error('duplicate or conflicting observation version binding');
    if (!['antlr-lexer-reference-semantics', 'peggy-optimized-character-set-shape'].includes(binding.assertionPolicy)) {
      throw new Error(`unsupported observation version policy ${binding.assertionPolicy}`);
    }
    bindings.set(key, binding);
  }
  const selectedContexts = new Map();
  const fixtureDigests = new Map();
  return (record, { fixtureFile } = {}) => {
    const binding = bindings.get(observationIdentity(record));
    if (!binding) {
      if (Object.hasOwn(record, 'versionApplicability')) throw new Error('version applicability is supplied for an unregistered observation scope');
      return undefined;
    }
    if (fixtureFile !== binding.fixtureFile) throw new Error('the version-dependent observation fixture path differs from its declared source');
    if (!fixtureDigests.has(fixtureFile)) fixtureDigests.set(fixtureFile,
      createHash('sha256').update(readFileSync(path.join(root, fixtureFile))).digest('hex'));
    if (record.fixtureDigest !== fixtureDigests.get(fixtureFile)) throw new Error('the version-dependent observation fixture digest differs from its actual source');
    if (!Object.hasOwn(record, 'versionApplicability')) throw new Error('the declared version-dependent observation has no version applicability');
    if (!selectedContexts.has(binding.assertionPolicy)) selectedContexts.set(binding.assertionPolicy,
      binding.assertionPolicy === 'antlr-lexer-reference-semantics'
        ? selectAntlrReferenceAssertion({ root, registry }) : selectPeggyOptimizerAssertion({ root, registry }));
    const selected = [selectedContexts.get(binding.assertionPolicy)];
    if (!isDeepStrictEqual(record.versionApplicability, selected)) throw new Error('observation version applicability differs from its actual selected source policy and explicit range');
    return selected;
  };
}
