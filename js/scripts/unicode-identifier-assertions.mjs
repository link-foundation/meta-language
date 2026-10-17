import { readProjectData } from './project-data.mjs';
import { selectVersionedAssertion } from './assertion-version-ranges.mjs';

export function selectUnicodeIdentifierAssertion(activeVersion) {
  const policies = readProjectData(new URL('../../parity/assertion-version-ranges.lino', import.meta.url));
  if (policies?.schemaVersion !== 1 || !Array.isArray(policies.assertions)) {
    throw new Error('Unicode identifier generation requires assertion version policy schema 1');
  }
  const matches = policies.assertions.filter(policy => policy.id === 'unicode-identifier-property-generation');
  if (matches.length !== 1) throw new Error('Unicode identifier generation requires exactly one version assertion policy');
  return selectVersionedAssertion(matches[0], activeVersion);
}
