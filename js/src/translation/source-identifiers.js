// Mechanical name census and ordering; shared rules own namespace admission.
import { acceptSourcePrivateIdentifier, readSourceIdentifierPrefixOrdinal, readSourceIdentifierPrefix } from './source-identifier-rules.js';

export function readSourceIdentifierSupply(names) {
  let privateNames = false;
  const ordinals = [];
  for (const name of names) {
    privateNames ||= acceptSourcePrivateIdentifier(name);
    const ordinal = readSourceIdentifierPrefixOrdinal(name);
    if (ordinal >= 1) ordinals.push(ordinal);
  }
  ordinals.sort((left, right) => left - right);
  return readSourceIdentifierPrefix(privateNames, ordinals);
}
export function collectSourceIdentifierTexts(value) {
  const strings = new Set(), seen = new Set(), pending = [value];
  while (pending.length) {
    const current = pending.pop();
    if (typeof current === 'string') { strings.add(current); continue; }
    if (!current || typeof current !== 'object' || seen.has(current)) continue;
    seen.add(current);
    if (current instanceof Map) for (const [key, value] of current) pending.push(key, value);
    else if (current instanceof Set || Array.isArray(current)) for (const item of current) pending.push(item);
    else for (const [key, value] of Object.entries(current)) pending.push(key, value);
  }
  return strings;
}
export function readSourceParserIdentifierSupply(source, tokens, extraNames = []) {
  const names = tokens.filter(token => token.kind === 'identifier').map(token => token.value);
  names.push(...extraNames);
  // Include comment/type/tag spellings as conservative source facts. Thirty-two
  // units cover every canonical safe-integer namespace plus its delimiter.
  for (let offset = source.indexOf('ml_'); offset >= 0; offset = source.indexOf('ml_', offset + 3))
    names.push(source.slice(offset, offset + 32));
  return readSourceIdentifierSupply(names);
}
