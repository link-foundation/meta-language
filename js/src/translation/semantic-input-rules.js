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

/** Select an unrestricted public free function's actual name token after its
 * complete attribute prefix. The caller supplies a parsed function item.
 * Restricted visibility retains a separate module-scope obligation.
 * @param {string[]} kinds @param {string[]} values @returns {number}
 */
export function readRustPublicSignatureNamePosition(kinds, values) {
  let position = 0;
  while (position < values.length && kinds[position] === 'punct' && values[position] === '#') {
    position += 1;
    if (position < values.length && kinds[position] === 'punct' && values[position] === '!') position += 1;
    if (position >= values.length || kinds[position] !== 'punct' || values[position] !== '[') return -1;
    let depth = 1;
    position += 1;
    while (position < values.length && depth > 0) {
      if (kinds[position] === 'punct' && values[position] === '[') depth += 1;
      if (kinds[position] === 'punct' && values[position] === ']') depth -= 1;
      position += 1;
    }
    if (depth !== 0) return -1;
  }
  if (position >= values.length || kinds[position] !== 'identifier' || values[position] !== 'pub') return -1;
  position += 1;
  if (position < values.length && kinds[position] === 'punct' && values[position] === '(') return -1;
  while (position < values.length) {
    if (kinds[position] === 'identifier' && values[position] === 'fn') {
      if (position + 1 < values.length && kinds[position + 1] === 'identifier') return position + 1;
      return -1;
    }
    if (kinds[position] === 'punct' && (values[position] === '{' || values[position] === ';')) return -1;
    position += 1;
  }
  return -1;
}

/** Ordinary free-function modules may share private and forward dependencies.
 * A carried source envelope is still an unresolved owner, never an empty body.
 * @param {string} term @param {boolean} comment @param {number} role @returns {boolean}
 */
export function acceptRustSemanticModuleItem(term, comment, role) {
  if (role === 3) return false;
  return comment || term === 'function_item' || term === 'attribute_item' || term === 'inner_attribute_item';
}

/** Both names come from actual checked source declarations/emitter mappings.
 * Export aliases preserve source visibility even when target names are renamed.
 * @param {string} source @param {string} target @returns {string}
 */
export function renderRustSemanticModuleExport(source, target) {
  return 'export { ' + target + ' as ' + source + ' };';
}
