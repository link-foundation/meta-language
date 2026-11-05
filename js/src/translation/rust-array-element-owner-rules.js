// Select an actual admitted source owner; never infer source spelling from width.
/** @param {string[]} identities @param {boolean[]} admitted @returns {number} */
export function selectRustArrayElementOwnerIndex(identities, admitted) {
  if (identities.length === 0 || identities.length !== admitted.length) return -1;
  for (let index = 0; index < identities.length; index += 1) {
    if (!admitted[index] || identities[index] !== identities[0]) return -1;
  }
  return 0;
}
