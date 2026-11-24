/**
 * The two leading doublets select the archive root and state record. Every
 * following named record owns an ordered chain of anonymous occurrences.
 * Each occurrence names a target record and the next occurrence. The chain
 * returns to its owner, so equal targets and self references stay distinct.
 * @typedef {{$:'archiveDoublets',doublets:number[]} |
 * {$:'archiveDirectory',root:number,state:number,ends:number[],targets:number[]} |
 * {$:'archiveTopologyError',reason:string}} LinkCliArchiveTopology
 */

/** @returns {string} */
export function readLinkCliArchiveStoreName() {
  return 'meta-language complete graph store 1';
}

/** @returns {string} */
export function readLinkCliArchiveStateName() {
  return 'meta-language complete graph store state 1';
}

/** @param {number} count @param {number} root @param {number} state @returns {boolean} */
export function acceptLinkCliArchiveDirectory(count, root, state) {
  return Number.isSafeInteger(count) && count >= 2 && count <= 4294967293
    && Number.isSafeInteger(root) && root >= 3 && root < state
    && Number.isSafeInteger(state) && state <= count + 2;
}

/**
 * The record directory starts at store address three. The state starts the
 * descriptor's independent directory. Targets are store addresses and ends
 * are cumulative occurrence counts, one per record.
 * @param {number[]} ends @param {number[]} targets @param {number} root
 * @param {number} state @param {number} limit @returns {LinkCliArchiveTopology}
 */
export function planLinkCliArchiveDoublets(ends, targets, root, state, limit) {
  const count = ends.length;
  if (!acceptLinkCliArchiveDirectory(count, root, state))
    return {$:'archiveTopologyError',reason:'invalid archive record directory'};
  if (!Number.isSafeInteger(limit) || count + targets.length + 2 > limit
    || count + targets.length + 2 > 4294967295)
    return {$:'archiveTopologyError',reason:'archive store link limit exceeded'};
  const doublets = [1, root, 2, state];
  let start = 0, nextAddress = count + 3;
  for (let index = 0; index < count; index += 1) {
    const end = ends[index] ?? -1, owner = index + 3;
    if (!Number.isSafeInteger(end) || end < start || end > targets.length)
      return {$:'archiveTopologyError',reason:'invalid cumulative occurrence count'};
    doublets.push(start < end ? nextAddress : owner, owner);
    nextAddress += end - start;
    start = end;
  }
  if (start !== targets.length)
    return {$:'archiveTopologyError',reason:'archive occurrence count mismatch'};
  start = 0;
  nextAddress = count + 3;
  for (let index = 0; index < count; index += 1) {
    const end = ends[index] ?? -1, owner = index + 3;
    for (let at = start; at < end; at += 1) {
      const target = targets[at] ?? -1;
      if (!Number.isSafeInteger(target) || target < 3 || target > count + 2
        || (owner < state) !== (target < state))
        return {$:'archiveTopologyError',reason:'archive occurrence crosses its record directory'};
      doublets.push(target, at + 1 === end ? owner : nextAddress + 1);
      nextAddress += 1;
    }
    start = end;
  }
  return {$:'archiveDoublets',doublets};
}

/**
 * Read the complete dense store directory in address order. Each anonymous
 * occurrence must belong to exactly one named record and stay in its scope.
 * @param {number[]} doublets @param {number} count @param {number} limit
 * @returns {LinkCliArchiveTopology}
 */
export function readLinkCliArchiveDoublets(doublets, count, limit) {
  const length = doublets.length / 2;
  if (!Number.isSafeInteger(limit) || !Number.isSafeInteger(length)
    || length > limit || length > 4294967295)
    return {$:'archiveTopologyError',reason:'archive store link limit exceeded'};
  const root = doublets[1] ?? -1, state = doublets[3] ?? -1;
  if (doublets[0] !== 1 || doublets[2] !== 2 || length < count + 2
    || !acceptLinkCliArchiveDirectory(count, root, state))
    return {$:'archiveTopologyError',reason:'invalid archive record directory'};
  const ends = [], targets = [];
  let nextAddress = count + 3;
  for (let owner = 3; owner <= count + 2; owner += 1) {
    const offset = (owner - 1) * 2;
    let address = doublets[offset] ?? -1;
    if (doublets[offset + 1] !== owner)
      return {$:'archiveTopologyError',reason:'invalid archive record owner'};
    while (address !== owner) {
      if (address !== nextAddress || address > length)
        return {$:'archiveTopologyError',reason:'noncanonical archive occurrence chain'};
      const position = (address - 1) * 2, target = doublets[position] ?? -1;
      if (!Number.isSafeInteger(target) || target < 3 || target > count + 2
        || (owner < state) !== (target < state))
        return {$:'archiveTopologyError',reason:'archive occurrence crosses its record directory'};
      targets.push(target);
      address = doublets[position + 1] ?? -1;
      nextAddress += 1;
    }
    ends.push(targets.length);
  }
  if (nextAddress !== length + 1)
    return {$:'archiveTopologyError',reason:'archive store contains unclaimed occurrences'};
  return {$:'archiveDirectory',root,state,ends,targets};
}
