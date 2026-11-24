import {readRequiredNumberFact, readRequiredTextFact} from './fact-column-rules.js';

/** Check declaration order and exclusive packet layout independently of the
 * target runtime's type representation. The caller verifies the actual types.
 * @param {string[]} modes @param {number} formalCount @param {number} callback
 * @param {number} captures @param {number[]} packet @returns {string}
 */
export function readRustFnMutSourcePacketError(modes, formalCount, callback, captures, packet) {
  if (!Number.isInteger(formalCount) || formalCount < 1 || formalCount > 256 ||
      !Number.isInteger(captures) || captures < 0 || captures > 256 ||
      !Number.isInteger(callback) || callback < 0 || callback >= formalCount ||
      formalCount + captures !== modes.length) {
    return 'the expanded source formal inventory adds exactly the actual callback state captures';
  }
  let packetIndex = 0;
  for (let parameter = 0; parameter < modes.length; parameter += 1) {
    const mode = readRequiredTextFact(modes, parameter);
    if ((mode === 'callback') !== (parameter === callback)) {
      return 'the concrete callback ABI binds exactly its actual original formal';
    }
    if (parameter >= formalCount && mode !== 'exclusive') {
      return 'the callback state ABI retains distinct actual captured lexical roots and their exact packet positions';
    }
    if (mode !== 'callback' && mode !== 'value' && mode !== 'shared' && mode !== 'exclusive') {
      return 'concrete callback formals retain actual value, shared, exclusive or callback roles';
    }
    if (mode === 'exclusive') {
      if (packetIndex >= packet.length || readRequiredNumberFact(packet, packetIndex) !== parameter) {
        return 'a FnMut completion publishes the original result then every actual exclusive environment root';
      }
      packetIndex += 1;
    }
  }
  return packetIndex === packet.length ? ''
    : 'a FnMut completion publishes the original result then every actual exclusive environment root';
}
