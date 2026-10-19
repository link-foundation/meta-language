// Complete iterative structural admission, shared with the native constructor path.
// The adapter owns passive input, source types, scalar values and representation facts.
/**
 * @param {number[]} offsets @param {number[]} children
 * @param {number} nodeLimit @param {number} edgeLimit @param {number} depthLimit
 * @returns {number[]}
 */
export function checkOwnedOutcomePreorder(offsets, children, nodeLimit, edgeLimit, depthLimit) {
  if (!Number.isSafeInteger(nodeLimit) || nodeLimit < 1 || !Number.isSafeInteger(edgeLimit)
    || edgeLimit < 1 || !Number.isSafeInteger(depthLimit) || depthLimit < 1) return [-1, 0, 0, 0];
  const count = offsets.length - 1;
  if (count < 1 || count > nodeLimit || children.length > edgeLimit || offsets[0] !== 0
    || offsets[count] !== children.length) return [-2, 0, 0, 0];
  for (let at = 0; at < offsets.length; at += 1) {
    const offset = offsets[at] ?? -1;
    if (!Number.isSafeInteger(offset) || offset < 0 || offset > children.length
      || at > 0 && offset < (offsets[at - 1] ?? -1)) return [-2, 0, 0, 0];
  }
  const identities = [1];
  const depths = [1];
  const successors = [-1];
  let head = 0;
  let visited = 0;
  let maximumDepth = 0;
  while (head >= 0) {
    const identity = identities[head] ?? 0;
    const depth = depths[head] ?? 0;
    head = successors[head] ?? -1;
    if (identity !== visited + 1 || identity > count || depth < 1) return [-3, 0, 0, 0];
    if (depth > depthLimit) return [-5, 0, 0, 0];
    visited += 1;
    maximumDepth = Math.max(maximumDepth, depth);
    const start = offsets[identity - 1] ?? -1;
    const end = offsets[identity] ?? -1;
    if (identities.length + end - start > nodeLimit) return [-4, 0, 0, 0];
    for (let at = end - 1; at >= start; at -= 1) {
      const child = children[at] ?? 0;
      if (!Number.isSafeInteger(child) || child <= identity || child > count) return [-4, 0, 0, 0];
      identities.push(child);
      depths.push(depth + 1);
      successors.push(head);
      head = identities.length - 1;
    }
  }
  if (visited !== count) return [-6, 0, 0, 0];
  return [0, visited, children.length, maximumDepth];
}
