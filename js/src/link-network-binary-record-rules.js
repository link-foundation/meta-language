// Record-local framing uses the existing checked primitive operations.
// The complete algorithm is automatically translated to the native runtime.
import { linkNetworkBinaryUnit, acceptLinkNetworkBinaryInteger, readLinkNetworkBinaryInteger,
  linkNetworkBinaryIntegerByte, renderLinkNetworkBinaryField } from './link-network-binary-rules.js';

/** Validate and transcode one byte sequence or flat host-field sequence.
 * A negative comma-delimited result is an error code and its input offset.
 * Output framing is transient ASCII: encoded bytes are hex; decoded fields
 * are decimal words. No source buffer or token envelope is stored.
 * Decode preserves the legacy version; v1 has no exact cache/allocator sections.
 * @param {number[]} input @param {boolean} binary @param {number} recordLimit @param {number} byteLimit @returns {string}
 */
export function transcodeLinkNetworkBinary(input, binary, recordLimit, byteLimit) {
  let output = '';
  if (!acceptLinkNetworkBinaryInteger(recordLimit) || recordLimit < 1 || !acceptLinkNetworkBinaryInteger(byteLimit) || byteLimit < 16) return '-1,' + 0 + ',';
  if (input.length > byteLimit) return '-2,' + 0 + ',';
  let cursor = 0;
  let version = 0;
  const header = [77, 76, 68, 83, 78, 80, 48];
  if (binary) {
    for (let index = 0; index < 7; index += 1) if (linkNetworkBinaryUnit(input, index) !== linkNetworkBinaryUnit(header, index)) return '-3,' + index + ',';
    version = linkNetworkBinaryUnit(input, 7) - 48;
    cursor = 8;
  } else {
    version = linkNetworkBinaryUnit(input, cursor);
    cursor += 1;
    for (let index = 0; index < 7; index += 1) output += renderLinkNetworkBinaryField(linkNetworkBinaryUnit(header, index), binary);
    output += renderLinkNetworkBinaryField(48 + version, binary);
  }
  if (version !== 1 && version !== 2) return '-4,' + 7 + ',';
  if (binary) output += renderLinkNetworkBinaryField(version, binary);
  let nextIdentity = 0;
  if (version === 2) {
    nextIdentity = readLinkNetworkBinaryInteger(input, cursor, 8, binary);
    if (nextIdentity < 1) return '-5,' + cursor + ',';
    cursor += binary ? 8 : 1;
    if (binary) output += renderLinkNetworkBinaryField(nextIdentity, binary);
    else for (let index = 0; index < 8; index += 1) output += renderLinkNetworkBinaryField(linkNetworkBinaryIntegerByte(nextIdentity, index), binary);
  }
  const count = readLinkNetworkBinaryInteger(input, cursor, 8, binary);
  if (count < 0 || count > recordLimit || count > input.length) return '-6,' + cursor + ',';
  cursor += binary ? 8 : 1;
  if (binary) output += renderLinkNetworkBinaryField(count, binary);
  else for (let index = 0; index < 8; index += 1) output += renderLinkNetworkBinaryField(linkNetworkBinaryIntegerByte(count, index), binary);
  let identities = ',';
  let greatestIdentity = 0;
  for (let record = 0; record < count; record += 1) {
    const identity = readLinkNetworkBinaryInteger(input, cursor, 8, binary);
    if (identity < 1 || identity >= 9007199254740991 || version === 2 && identity >= nextIdentity) return '-7,' + cursor + ',';
    if (identity > greatestIdentity) greatestIdentity = identity;
    else if (identities.includes(',' + identity + ',')) return '-8,' + cursor + ',';
    identities += identity + ',';
    cursor += binary ? 8 : 1;
    if (binary) output += renderLinkNetworkBinaryField(identity, binary);
    else for (let index = 0; index < 8; index += 1) output += renderLinkNetworkBinaryField(linkNetworkBinaryIntegerByte(identity, index), binary);
    const references = readLinkNetworkBinaryInteger(input, cursor, 8, binary);
    if (references < 0 || references > input.length) return '-9,' + cursor + ',';
    cursor += binary ? 8 : 1;
    if (binary) output += renderLinkNetworkBinaryField(references, binary);
    else for (let index = 0; index < 8; index += 1) output += renderLinkNetworkBinaryField(linkNetworkBinaryIntegerByte(references, index), binary);
    for (let reference = 0; reference < references; reference += 1) {
      const target = readLinkNetworkBinaryInteger(input, cursor, 8, binary);
      if (target < 1) return '-10,' + cursor + ',';
      cursor += binary ? 8 : 1;
      if (binary) output += renderLinkNetworkBinaryField(target, binary);
      else for (let index = 0; index < 8; index += 1) output += renderLinkNetworkBinaryField(linkNetworkBinaryIntegerByte(target, index), binary);
    }
    let metadataEnd = 0;
    const recordPrefix = output;
    output = '';
    const lengthSlot = output.length;
    if (binary) {
      const length = readLinkNetworkBinaryInteger(input, cursor, 8, true);
      if (length < 6 || length > input.length - cursor - 8) return '-11,' + cursor + ',';
      cursor += 8;
      metadataEnd = cursor + length;
    } else for (let index = 0; index < 8; index += 1) output += renderLinkNetworkBinaryField(0, binary);
    const metadataStart = output.length;
    for (let field = 0; field < 6; field += 1) {
      const value = readLinkNetworkBinaryInteger(input, cursor, 1, binary);
      if (value < 0 || value > 255) return '-12,' + cursor + ',';
      if (field === 0 && value !== 1 || field === 1 && value > 15
        || (field === 2 || field === 4) && value > 1 || field === 5 && value !== 0
        || field === 3 && (value > 15 || value % 2 === 1 && Math.floor(value / 2) % 2 !== 1 || Math.floor(value / 4) % 2 === 1 && Math.floor(value / 2) % 2 !== 1)) return '-13,' + cursor + ',';
      cursor += 1;
      output += renderLinkNetworkBinaryField(value, binary);
    }
    for (let text = 0; text < 3; text += 1) {
      const present = readLinkNetworkBinaryInteger(input, cursor, 1, binary);
      if (present !== 0 && present !== 1) return '-14,' + cursor + ',';
      cursor += 1;
      output += renderLinkNetworkBinaryField(present, binary);
      if (present === 1) {
        const length = readLinkNetworkBinaryInteger(input, cursor, 8, binary);
        if (length < 0 || length > input.length - cursor - (binary ? 8 : 1)) return '-15,' + cursor + ',';
        cursor += binary ? 8 : 1;
        if (binary) output += renderLinkNetworkBinaryField(length, binary);
        else for (let index = 0; index < 8; index += 1) output += renderLinkNetworkBinaryField(linkNetworkBinaryIntegerByte(length, index), binary);
        for (let index = 0; index < length; index += 1) {
          const value = readLinkNetworkBinaryInteger(input, cursor, 1, binary);
          if (value < 0 || value > 255) return '-16,' + cursor + ',';
          cursor += 1;
          output += renderLinkNetworkBinaryField(value, binary);
        }
      }
    }
    const span = readLinkNetworkBinaryInteger(input, cursor, 1, binary);
    if (span !== 0 && span !== 1) return '-17,' + cursor + ',';
    cursor += 1;
    output += renderLinkNetworkBinaryField(span, binary);
    if (span === 1) for (let field = 0; field < 6; field += 1) {
      const value = readLinkNetworkBinaryInteger(input, cursor, 8, binary);
      if (value < 0) return '-18,' + cursor + ',';
      cursor += binary ? 8 : 1;
      if (binary) output += renderLinkNetworkBinaryField(value, binary);
      else for (let index = 0; index < 8; index += 1) output += renderLinkNetworkBinaryField(linkNetworkBinaryIntegerByte(value, index), binary);
    }
    if (binary && cursor !== metadataEnd) return '-19,' + cursor + ',';
    if (!binary) {
      const length = (output.length - metadataStart) / 2;
      let encodedLength = '';
      for (let index = 0; index < 8; index += 1) encodedLength += renderLinkNetworkBinaryField(linkNetworkBinaryIntegerByte(length, index), false);
      output = output.slice(0, lengthSlot) + encodedLength + output.slice(lengthSlot + 16);
    }
    output = recordPrefix + output;
    if (output.length > byteLimit * (binary ? 20 : 2)) return '-2,' + cursor + ',';
  }
  if (version === 2) for (let section = 0; section < 2; section += 1) {
    const entries = readLinkNetworkBinaryInteger(input, cursor, 8, binary);
    if (entries < 0 || entries > recordLimit || entries > input.length) return '-20,' + cursor + ',';
    cursor += binary ? 8 : 1;
    if (binary) output += renderLinkNetworkBinaryField(entries, binary);
    else for (let index = 0; index < 8; index += 1) output += renderLinkNetworkBinaryField(linkNetworkBinaryIntegerByte(entries, index), binary);
    for (let entry = 0; entry < entries; entry += 1) {
      const strings = section === 0 ? 1 : 3;
      for (let text = 0; text < strings; text += 1) {
        const length = readLinkNetworkBinaryInteger(input, cursor, 8, binary);
        if (length < 0 || length > input.length - cursor - (binary ? 8 : 1)) return '-15,' + cursor + ',';
        cursor += binary ? 8 : 1;
        if (binary) output += renderLinkNetworkBinaryField(length, binary);
        else for (let index = 0; index < 8; index += 1) output += renderLinkNetworkBinaryField(linkNetworkBinaryIntegerByte(length, index), binary);
        for (let index = 0; index < length; index += 1) {
          const value = readLinkNetworkBinaryInteger(input, cursor, 1, binary);
          if (value < 0 || value > 255) return '-16,' + cursor + ',';
          cursor += 1;
          output += renderLinkNetworkBinaryField(value, binary);
        }
      }
      if (section === 0) {
        const identity = readLinkNetworkBinaryInteger(input, cursor, 8, binary);
        if (identity < 1) return '-21,' + cursor + ',';
        const found = identity <= greatestIdentity && identities.includes(',' + identity + ',');
        if (!found) return '-21,' + cursor + ',';
        cursor += binary ? 8 : 1;
        if (binary) output += renderLinkNetworkBinaryField(identity, binary);
        else for (let index = 0; index < 8; index += 1) output += renderLinkNetworkBinaryField(linkNetworkBinaryIntegerByte(identity, index), binary);
      }
      if (output.length > byteLimit * (binary ? 20 : 2)) return '-2,' + cursor + ',';
    }
  }
  if (cursor !== input.length) return '-22,' + cursor + ',';
  if (output.length > byteLimit * (binary ? 20 : 2)) return '-2,' + cursor + ',';
  return output;
}
