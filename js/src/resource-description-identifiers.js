// Compose the existing IRI grammar with the source-generated IP-literal rule.
import { acceptResourceDescriptionIdentifier } from './resource-description-rules.js';
import { projectResourceAddressAuthority, acceptResourceAddressIdentifier } from './resource-description-address-rules.js';

export function acceptResourceDescriptionIdentifierText(value) {
  if (typeof value !== 'string' || value.length > 65536) return false;
  const units = Array.from({ length: value.length }, (_, index) => value.charCodeAt(index));
  const projection = projectResourceAddressAuthority(units);
  return acceptResourceAddressIdentifier(acceptResourceDescriptionIdentifier(units),
    projection.length > 0 && acceptResourceDescriptionIdentifier(projection));
}
