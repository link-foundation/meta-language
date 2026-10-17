// Bind tooling assertions to the actual declared ANTLR reference semantics.
// Generic grammar importers and executors do not depend on this test adapter.
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { isDeepStrictEqual } from 'node:util';
import { readStructuredDataLino } from '../src/structured-data-links.js';
import { selectVersionedAssertion } from './assertion-version-ranges.mjs';

const repository = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../..');
export const ASSERTION_VERSION_POLICY_FILE = 'parity/assertion-version-ranges.lino';
export const ANTLR_REFERENCE_POLICY_FILE = 'js/src/translation/data/grammar-lexer-rules.lino';
const nativePolicyFile = 'rust/src/data/grammar-lexer-rules.lino';
const hash = bytes => createHash('sha256').update(bytes).digest('hex');

export function readAssertionVersionPolicy(root = repository) {
  const bytes = readFileSync(path.join(root, ASSERTION_VERSION_POLICY_FILE));
  const policy = readStructuredDataLino(bytes.toString('utf8'));
  if (policy?.schemaVersion !== 1 || !Array.isArray(policy.assertions)) throw new Error('assertion version policy schema 1 is required');
  return { policy, sha256: hash(bytes) };
}

export function selectAntlrReferenceAssertion({ root = repository, registry = readAssertionVersionPolicy(root) } = {}) {
  const matches = registry.policy.assertions.filter(policy => policy.id === 'antlr-lexer-reference-semantics');
  if (matches.length !== 1) throw new Error('ANTLR reference semantics require exactly one registered assertion policy');
  const sourceBytes = readFileSync(path.join(root, ANTLR_REFERENCE_POLICY_FILE));
  if (!sourceBytes.equals(readFileSync(path.join(root, nativePolicyFile)))) throw new Error('ANTLR reference source policies differ between runtime packages');
  const source = readStructuredDataLino(sourceBytes.toString('utf8'));
  const references = source?.upstreamSemantics;
  const active = /^ANTLR ((?:0|[1-9][0-9]*)\.(?:0|[1-9][0-9]*)\.(?:0|[1-9][0-9]*))$/u.exec(references?.version ?? '');
  if (!active) throw new Error('the actual ANTLR reference policy needs an explicit ANTLR X.Y.Z version');
  const selected = selectVersionedAssertion(matches[0], active[1]);
  const { version: sourceVersion, ...sourceReferences } = references;
  if (selected.expected !== sourceVersion || !isDeepStrictEqual(selected.sourceReferences, sourceReferences)) {
    throw new Error('the active ANTLR reference version or primary source URLs differ from the selected expectation');
  }
  return Object.freeze({
    assertionPolicy: matches[0].id, expectation: selected.id, subject: matches[0].subject,
    activeVersion: active[1], sourceVersion,
    minimum: selected.minimum, maximum: selected.maximum,
    includeMinimum: selected.includeMinimum, includeMaximum: selected.includeMaximum,
    applicabilityPolicy: ASSERTION_VERSION_POLICY_FILE, applicabilityPolicySha256: registry.sha256,
    sourcePolicy: ANTLR_REFERENCE_POLICY_FILE, sourcePolicySha256: hash(sourceBytes),
    sourceReferences: Object.freeze(sourceReferences),
  });
}
