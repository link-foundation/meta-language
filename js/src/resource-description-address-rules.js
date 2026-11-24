// RFC 3986 IP-literal authority grammar used by the RFC 3987 IRI profile.
// Projection validates the address, then leaves all surrounding IRI spelling
// to the existing Unicode/scheme/userinfo/port/path/query/fragment rules.

/** @param {number[]} units @param {number} at @returns {number} */
export function readResourceAddressUnit(units, at) { return units[at] ?? -1; }
/** @param {number} value @returns {boolean} */
export function acceptResourceAddressHexadecimal(value) {
  return value >= 48 && value <= 57 || value >= 65 && value <= 70 || value >= 97 && value <= 102;
}
/** @param {number[]} units @param {number} start @param {number} end @returns {boolean} */
export function acceptResourceAddressDecimalTail(units, start, end) {
  let parts = 0, position = start;
  while (position < end) {
    const beginning = position;
    let value = 0;
    while (position < end && readResourceAddressUnit(units, position) !== 46) {
      const digit = readResourceAddressUnit(units, position) - 48;
      if (digit < 0 || digit > 9 || position - beginning >= 3) return false;
      value = value * 10 + digit; position += 1;
    }
    if (position === beginning || value > 255 || position - beginning > 1 && readResourceAddressUnit(units, beginning) === 48) return false;
    parts += 1;
    if (position === end) break;
    position += 1;
    if (position === end) return false;
  }
  return parts === 4;
}
/** @param {number[]} units @param {number} start @param {number} end @returns {boolean} */
export function acceptResourceAddressVersionSix(units, start, end) {
  let position = start, groups = 0, compressed = false;
  if (readResourceAddressUnit(units, position) === 58) {
    if (position + 1 >= end || readResourceAddressUnit(units, position + 1) !== 58) return false;
    compressed = true; position += 2;
  }
  while (position < end) {
    const beginning = position;
    let decimal = false;
    while (position < end && readResourceAddressUnit(units, position) !== 58) {
      if (readResourceAddressUnit(units, position) === 46) decimal = true;
      position += 1;
    }
    if (decimal) {
      if (position !== end || !acceptResourceAddressDecimalTail(units, beginning, end)) return false;
      groups += 2;
    } else {
      if (position === beginning || position - beginning > 4) return false;
      for (let at = beginning; at < position; at += 1) if (!acceptResourceAddressHexadecimal(readResourceAddressUnit(units, at))) return false;
      groups += 1;
    }
    if (groups > 8) return false;
    if (position === end) break;
    if (position + 1 < end && readResourceAddressUnit(units, position + 1) === 58) {
      if (compressed) return false;
      compressed = true; position += 2;
    } else {
      position += 1;
      if (position === end) return false;
    }
  }
  return compressed ? groups < 8 : groups === 8;
}
/** @param {number[]} units @param {number} start @param {number} end @returns {boolean} */
export function acceptResourceAddressFutureVersion(units, start, end) {
  const first = readResourceAddressUnit(units, start);
  if (first !== 86 && first !== 118) return false;
  let position = start + 1;
  while (position < end && acceptResourceAddressHexadecimal(readResourceAddressUnit(units, position))) position += 1;
  if (position === start + 1 || position >= end || readResourceAddressUnit(units, position) !== 46) return false;
  position += 1;
  if (position === end) return false;
  for (; position < end; position += 1) {
    const value = readResourceAddressUnit(units, position);
    if (value >= 48 && value <= 57 || value >= 65 && value <= 90 || value >= 97 && value <= 122) continue;
    if (value === 45 || value === 46 || value === 95 || value === 126 || value === 58 || value === 33
      || value === 36 || value === 38 || value === 39 || value === 40 || value === 41 || value === 42
      || value === 43 || value === 44 || value === 59 || value === 61) continue;
    return false;
  }
  return true;
}
/** The returned sequence is a validation projection, never replacement source.
 * An empty sequence means no valid bracketed authority was established.
 * @param {number[]} units @returns {number[]}
 */
export function projectResourceAddressAuthority(units) {
  if (units.length > 65536) return [];
  for (const value of units) if (!Number.isSafeInteger(value) || value < 0 || value > 65535) return [];
  let colon = 0;
  while (colon < units.length && readResourceAddressUnit(units, colon) !== 58) colon += 1;
  if (colon + 2 >= units.length || readResourceAddressUnit(units, colon + 1) !== 47 || readResourceAddressUnit(units, colon + 2) !== 47) return [];
  let end = colon + 3;
  while (end < units.length && readResourceAddressUnit(units, end) !== 47 && readResourceAddressUnit(units, end) !== 63 && readResourceAddressUnit(units, end) !== 35) end += 1;
  let host = colon + 3;
  for (let at = host; at < end; at += 1) if (readResourceAddressUnit(units, at) === 64) host = at + 1;
  if (readResourceAddressUnit(units, host) !== 91) return [];
  let close = host + 1;
  while (close < end && readResourceAddressUnit(units, close) !== 93) close += 1;
  if (close === end || close === host + 1 || close + 1 < end && readResourceAddressUnit(units, close + 1) !== 58) return [];
  if (!acceptResourceAddressVersionSix(units, host + 1, close) && !acceptResourceAddressFutureVersion(units, host + 1, close)) return [];
  return [...units.slice(0, host), 104, ...units.slice(close + 1)];
}

/** @param {boolean} ordinary @param {boolean} address @returns {boolean} */
export function acceptResourceAddressIdentifier(ordinary, address) { return ordinary || address; }
