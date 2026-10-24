// Source ownership decisions for pure inherent functions and immutable methods.
/** @param {string[]} models @returns {boolean} */
export function acceptRustInherentSourceProfiles(models) {
  for (const model of models) if (model !== '') return false;
  return true;
}
/** @param {string} spelling @returns {number} */
export function readRustInherentReceiverMode(spelling) {
  return spelling === 'self' ? 1 : spelling === '&self' ? 2 : 0;
}
/** @param {number} mode @param {number} wrappers @param {boolean} references @param {string} actual @param {string} expected @returns {boolean} */
export function acceptRustInherentReceiver(mode, wrappers, references, actual, expected) {
  return actual === expected && (mode === 1 && wrappers === 0
    || mode === 2 && (wrappers === 0 || wrappers === 1 && references));
}
/** @param {number} mode @param {number} wrappers @returns {boolean} */
export function requireRustInherentReceiverBorrow(mode, wrappers) { return mode === 2 && wrappers === 0; }
/** @param {string[]} module @param {string[]} owner @returns {boolean} */
export function acceptRustInherentOwnerModule(module, owner) {
  if (owner.length !== module.length + 1) return false;
  for (let index = 0; index < module.length; index += 1) {
    if (owner[index] !== module[index]) return false;
  }
  return true;
}
/** @param {string[]} module @param {number} depth @param {number} count @param {string[]} names @returns {string[]} */
export function readRustInherentParentPath(module, depth, count, names) {
  const path = [];
  if (count > module.length) {
    for (let index = 0; index <= depth; index += 1) path.push('super');
  } else {
    path.push('crate');
    for (let index = 0; index < module.length - count; index += 1) path.push(module[index] ?? '');
  }
  for (const name of names) path.push(name);
  return path;
}
