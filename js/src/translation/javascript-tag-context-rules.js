// Shared JS-first selection decision. Existing unique-owner behavior is kept;
// only an exact known receiver owner may disambiguate multiple candidates.
/** @param {string[]} owners @param {string} receiver @returns {number} */
export function readJavaScriptTagOwnerPosition(owners, receiver) {
  if (owners.length === 1) return 0;
  if (receiver === '') return -1;
  let selected = -1;
  for (let index = 0; index < owners.length; index += 1) {
    if (owners[index] === receiver) {
      if (selected >= 0) return -1;
      selected = index;
    }
  }
  return selected;
}

/** Actual receiver facts: 0 unavailable, 1 rooted named call, 2 local binding.
 * @param {string} kind @param {string} functionKind
 * @param {string[]} path @param {string} root @returns {number}
 */
export function readJavaScriptTagReceiverForm(kind, functionKind, path, root) {
  if (kind === 'app' && functionKind === 'name' && path.length === 2 && path[0] === root) return 1;
  if (kind !== 'name') return 0;
  if (path.length === 2 && path[0] === root) return 1;
  if (path.length === 1) return 2;
  return 0;
}

/** Nearest ownership precedes a boundary: 0 parent, 1 owned fact, 2 stop.
 * An owned binding stops lookup even when its host has no declared type fact.
 * @param {boolean} ownsBinding @param {number} boundary @returns {number}
 */
export function readJavaScriptTagBindingStep(ownsBinding, boundary) {
  if (ownsBinding) return 1;
  if (boundary !== 0) return 2;
  return 0;
}

/** A speculative source header is consulted only when no actual type exists.
 * @param {boolean} hasType @param {boolean} hasHeader
 * @param {boolean} asynchronous @returns {boolean}
 */
export function acceptJavaScriptTagHeaderFact(hasType, hasHeader, asynchronous) {
  return !hasType && hasHeader && !asynchronous;
}

/** Only an exact declared data owner is context; absence is never unwrapped.
 * @param {string} kind @param {string[]} path
 * @param {string} name @param {string} root @returns {string}
 */
export function readJavaScriptTagDeclaredOwner(kind, path, name, root) {
  if (kind === 'data') return name;
  if (kind === 'named' && path.length === 2 && path[0] === root) return path[1] ?? '';
  return '';
}
