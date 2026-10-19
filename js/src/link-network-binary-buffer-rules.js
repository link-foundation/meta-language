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
  const output = [];
  let outputLength = 0;
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
    for (let index = 0; index < 7; index += 1) { const encoded = renderLinkNetworkBinaryField(linkNetworkBinaryUnit(header, index), binary); output.push(encoded); outputLength += encoded.length; }
    { const encoded = renderLinkNetworkBinaryField(48 + version, binary); output.push(encoded); outputLength += encoded.length; }
  }
  if (version !== 1 && version !== 2) return '-4,' + 7 + ',';
  if (binary) { const encoded = renderLinkNetworkBinaryField(version, binary); output.push(encoded); outputLength += encoded.length; }
  let nextIdentity = 0;
  if (version === 2) {
    nextIdentity = readLinkNetworkBinaryInteger(input, cursor, 8, binary);
    if (nextIdentity < 1) return '-5,' + cursor + ',';
    cursor += binary ? 8 : 1;
    if (binary) { const encoded = renderLinkNetworkBinaryField(nextIdentity, binary); output.push(encoded); outputLength += encoded.length; }
    else for (let index = 0; index < 8; index += 1) { const encoded = renderLinkNetworkBinaryField(linkNetworkBinaryIntegerByte(nextIdentity, index), binary); output.push(encoded); outputLength += encoded.length; }
  }
  const count = readLinkNetworkBinaryInteger(input, cursor, 8, binary);
  if (count < 0 || count > recordLimit || count > input.length) return '-6,' + cursor + ',';
  cursor += binary ? 8 : 1;
  if (binary) { const encoded = renderLinkNetworkBinaryField(count, binary); output.push(encoded); outputLength += encoded.length; }
  else for (let index = 0; index < 8; index += 1) { const encoded = renderLinkNetworkBinaryField(linkNetworkBinaryIntegerByte(count, index), binary); output.push(encoded); outputLength += encoded.length; }
  const identities = [];
  let greatestIdentity = 0;
  for (let record = 0; record < count; record += 1) {
    const identity = readLinkNetworkBinaryInteger(input, cursor, 8, binary);
    if (identity < 1 || identity >= 9007199254740991 || version === 2 && identity >= nextIdentity) return '-7,' + cursor + ',';
    if (identity > greatestIdentity) greatestIdentity = identity;
    else if (identities.includes(identity)) return '-8,' + cursor + ',';
    identities.push(identity);
    cursor += binary ? 8 : 1;
    if (binary) { const encoded = renderLinkNetworkBinaryField(identity, binary); output.push(encoded); outputLength += encoded.length; }
    else for (let index = 0; index < 8; index += 1) { const encoded = renderLinkNetworkBinaryField(linkNetworkBinaryIntegerByte(identity, index), binary); output.push(encoded); outputLength += encoded.length; }
    const references = readLinkNetworkBinaryInteger(input, cursor, 8, binary);
    if (references < 0 || references > input.length) return '-9,' + cursor + ',';
    cursor += binary ? 8 : 1;
    if (binary) { const encoded = renderLinkNetworkBinaryField(references, binary); output.push(encoded); outputLength += encoded.length; }
    else for (let index = 0; index < 8; index += 1) { const encoded = renderLinkNetworkBinaryField(linkNetworkBinaryIntegerByte(references, index), binary); output.push(encoded); outputLength += encoded.length; }
    for (let reference = 0; reference < references; reference += 1) {
      const target = readLinkNetworkBinaryInteger(input, cursor, 8, binary);
      if (target < 1) return '-10,' + cursor + ',';
      cursor += binary ? 8 : 1;
      if (binary) { const encoded = renderLinkNetworkBinaryField(target, binary); output.push(encoded); outputLength += encoded.length; }
      else for (let index = 0; index < 8; index += 1) { const encoded = renderLinkNetworkBinaryField(linkNetworkBinaryIntegerByte(target, index), binary); output.push(encoded); outputLength += encoded.length; }
    }
    let metadataEnd = 0;
    const metadata = [];
    let metadataLength = 0;
    const lengthSlot = 0;
    if (binary) {
      const length = readLinkNetworkBinaryInteger(input, cursor, 8, true);
      if (length < 6 || length > input.length - cursor - 8) return '-11,' + cursor + ',';
      cursor += 8;
      metadataEnd = cursor + length;
    } else for (let index = 0; index < 8; index += 1) { const encoded = renderLinkNetworkBinaryField(0, binary); metadata.push(encoded); metadataLength += encoded.length; }
    const metadataStart = metadataLength;
    for (let field = 0; field < 6; field += 1) {
      const value = readLinkNetworkBinaryInteger(input, cursor, 1, binary);
      if (value < 0 || value > 255) return '-12,' + cursor + ',';
      if (field === 0 && value !== 1 || field === 1 && value > 15
        || (field === 2 || field === 4) && value > 1 || field === 5 && value !== 0
        || field === 3 && (value > 15 || value % 2 === 1 && Math.floor(value / 2) % 2 !== 1 || Math.floor(value / 4) % 2 === 1 && Math.floor(value / 2) % 2 !== 1)) return '-13,' + cursor + ',';
      cursor += 1;
      { const encoded = renderLinkNetworkBinaryField(value, binary); metadata.push(encoded); metadataLength += encoded.length; }
    }
    for (let text = 0; text < 3; text += 1) {
      const present = readLinkNetworkBinaryInteger(input, cursor, 1, binary);
      if (present !== 0 && present !== 1) return '-14,' + cursor + ',';
      cursor += 1;
      { const encoded = renderLinkNetworkBinaryField(present, binary); metadata.push(encoded); metadataLength += encoded.length; }
      if (present === 1) {
        const length = readLinkNetworkBinaryInteger(input, cursor, 8, binary);
        if (length < 0 || length > input.length - cursor - (binary ? 8 : 1)) return '-15,' + cursor + ',';
        cursor += binary ? 8 : 1;
        if (binary) { const encoded = renderLinkNetworkBinaryField(length, binary); metadata.push(encoded); metadataLength += encoded.length; }
        else for (let index = 0; index < 8; index += 1) { const encoded = renderLinkNetworkBinaryField(linkNetworkBinaryIntegerByte(length, index), binary); metadata.push(encoded); metadataLength += encoded.length; }
        for (let index = 0; index < length; index += 1) {
          const value = readLinkNetworkBinaryInteger(input, cursor, 1, binary);
          if (value < 0 || value > 255) return '-16,' + cursor + ',';
          cursor += 1;
          { const encoded = renderLinkNetworkBinaryField(value, binary); metadata.push(encoded); metadataLength += encoded.length; }
        }
      }
    }
    const span = readLinkNetworkBinaryInteger(input, cursor, 1, binary);
    if (span !== 0 && span !== 1) return '-17,' + cursor + ',';
    cursor += 1;
    { const encoded = renderLinkNetworkBinaryField(span, binary); metadata.push(encoded); metadataLength += encoded.length; }
    if (span === 1) for (let field = 0; field < 6; field += 1) {
      const value = readLinkNetworkBinaryInteger(input, cursor, 8, binary);
      if (value < 0) return '-18,' + cursor + ',';
      cursor += binary ? 8 : 1;
      if (binary) { const encoded = renderLinkNetworkBinaryField(value, binary); metadata.push(encoded); metadataLength += encoded.length; }
      else for (let index = 0; index < 8; index += 1) { const encoded = renderLinkNetworkBinaryField(linkNetworkBinaryIntegerByte(value, index), binary); metadata.push(encoded); metadataLength += encoded.length; }
    }
    if (binary && cursor !== metadataEnd) return '-19,' + cursor + ',';
    const recordOutput = metadata.join('');
    let completedRecord = recordOutput;
    if (!binary) {
      const length = (metadataLength - metadataStart) / 2;
      let encodedLength = '';
      for (let index = 0; index < 8; index += 1) encodedLength += renderLinkNetworkBinaryField(linkNetworkBinaryIntegerByte(length, index), false);
      completedRecord = recordOutput.slice(0, lengthSlot) + encodedLength + recordOutput.slice(lengthSlot + 16);
    }
    output.push(completedRecord);
    outputLength += completedRecord.length;
    if (outputLength > byteLimit * (binary ? 20 : 2)) return '-2,' + cursor + ',';
  }
  if (version === 2) for (let section = 0; section < 2; section += 1) {
    const entries = readLinkNetworkBinaryInteger(input, cursor, 8, binary);
    if (entries < 0 || entries > recordLimit || entries > input.length) return '-20,' + cursor + ',';
    cursor += binary ? 8 : 1;
    if (binary) { const encoded = renderLinkNetworkBinaryField(entries, binary); output.push(encoded); outputLength += encoded.length; }
    else for (let index = 0; index < 8; index += 1) { const encoded = renderLinkNetworkBinaryField(linkNetworkBinaryIntegerByte(entries, index), binary); output.push(encoded); outputLength += encoded.length; }
    for (let entry = 0; entry < entries; entry += 1) {
      const strings = section === 0 ? 1 : 3;
      for (let text = 0; text < strings; text += 1) {
        const length = readLinkNetworkBinaryInteger(input, cursor, 8, binary);
        if (length < 0 || length > input.length - cursor - (binary ? 8 : 1)) return '-15,' + cursor + ',';
        cursor += binary ? 8 : 1;
        if (binary) { const encoded = renderLinkNetworkBinaryField(length, binary); output.push(encoded); outputLength += encoded.length; }
        else for (let index = 0; index < 8; index += 1) { const encoded = renderLinkNetworkBinaryField(linkNetworkBinaryIntegerByte(length, index), binary); output.push(encoded); outputLength += encoded.length; }
        for (let index = 0; index < length; index += 1) {
          const value = readLinkNetworkBinaryInteger(input, cursor, 1, binary);
          if (value < 0 || value > 255) return '-16,' + cursor + ',';
          cursor += 1;
          { const encoded = renderLinkNetworkBinaryField(value, binary); output.push(encoded); outputLength += encoded.length; }
        }
      }
      if (section === 0) {
        const identity = readLinkNetworkBinaryInteger(input, cursor, 8, binary);
        if (identity < 1) return '-21,' + cursor + ',';
        const found = identity <= greatestIdentity && identities.includes(identity);
        if (!found) return '-21,' + cursor + ',';
        cursor += binary ? 8 : 1;
        if (binary) { const encoded = renderLinkNetworkBinaryField(identity, binary); output.push(encoded); outputLength += encoded.length; }
        else for (let index = 0; index < 8; index += 1) { const encoded = renderLinkNetworkBinaryField(linkNetworkBinaryIntegerByte(identity, index), binary); output.push(encoded); outputLength += encoded.length; }
      }
      if (outputLength > byteLimit * (binary ? 20 : 2)) return '-2,' + cursor + ',';
    }
  }
  if (cursor !== input.length) return '-22,' + cursor + ',';
  if (outputLength > byteLimit * (binary ? 20 : 2)) return '-2,' + cursor + ',';
  return output.join('');
}
