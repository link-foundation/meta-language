// Shared source grouping decisions for independent semantic translation.

/** @param {boolean} comment @param {string} text @returns {number} */
export function readSemanticSourceRole(comment, text) {
  if (!comment) return 0;
  if (text.startsWith('// meta-language:self-translation:v1 ')) return 2;
  if (text.startsWith('// meta-language:carried ')) return 3;
  if (text.startsWith('// meta-language:translated ')) return 4;
  if (text === '// meta-language:prelude begin' || text === '// meta-language:prelude end') return 5;
  if (text.startsWith('// |')) return 6;
  return 1;
}

/** Triples are first item, last item, disposition (provenance, carried,
 * comment, declaration). Every original item occurs in exactly one group.
 * @param {number[]} roles @param {boolean[]} lineBreaks @returns {number[]}
 */
export function planSemanticSourceGroups(roles, lineBreaks) {
  const plan = [];
  for (let index = 0; index < roles.length; index += 1) {
    const role = roles[index];
    let end = index;
    let disposition = 0;
    if (role === 3 || role === 4) {
      while (end + 1 < roles.length && roles[end + 1] === 6 && lineBreaks[end]) end += 1;
    }
    if (role === 3) disposition = 1;
    else if (!(role >= 2 && role <= 5)) {
      while (roles[end] !== 0 && end + 1 < roles.length && lineBreaks[end]
        && !(roles[end + 1] >= 2 && roles[end + 1] <= 5)) end += 1;
      disposition = roles[end] === 0 ? 3 : 2;
    }
    plan.push(index);
    plan.push(end);
    plan.push(disposition);
    index = end;
  }
  return plan;
}
