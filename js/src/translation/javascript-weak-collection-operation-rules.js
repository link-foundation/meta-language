/** @param {string} operation @returns {string} */
export function readJavaScriptWeakCollectionMethodOwner(operation) {
  if (['WeakMap.get', 'WeakMap.set', 'WeakMap.has', 'WeakMap.delete'].includes(operation)) return 'WeakMap';
  if (['WeakSet.add', 'WeakSet.has', 'WeakSet.delete'].includes(operation)) return 'WeakSet';
  return '';
}
