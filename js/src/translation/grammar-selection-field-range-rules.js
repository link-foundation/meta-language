// Closed decisions for selectors over actual physical CST field occurrences.
// Tree traversal, witness storage and UTF-8 decoding remain existing host roles.

/** @param {string} mapping @returns {boolean} */
export function acceptGrammarSelectionFieldRangeMapping(mapping) {
  return mapping === 'field-ranges';
}

/** @param {string} kind @param {string} mapping @returns {boolean} */
export function acceptGrammarSelectionFieldRangeCandidate(kind, mapping) {
  return kind === 'grammar-field-candidate' && acceptGrammarSelectionFieldRangeMapping(mapping);
}

/** Occurrence indices select a finite window of the topmost physical matches.
 * Zero count never means all. Labels are exact capture labels, not rule names.
 * @param {string} kind @param {string} label @param {string} mode
 * @param {number} first @param {number} count @returns {boolean}
 */
export function acceptGrammarSelectionFieldRangeSelector(kind, label, mode, first, count) {
  return kind === 'source-field-selector' && typeof label === 'string' && label !== ''
    && (mode === 'include' || mode === 'exclude')
    && Number.isInteger(first) && first >= 0 && first <= 4294967295
    && Number.isInteger(count) && count > 0 && count <= 4294967295;
}

/** Zero leaves legacy profiles alone; one admits one common syntactic field
 * profile; two refuses a mixed profile or lexical selection.
 * @param {string[]} mappings @param {boolean} inToken @returns {number}
 */
export function readGrammarSelectionFieldRangeContext(mappings, inToken) {
  let found = false;
  for (let index = 0; index < mappings.length; index += 1) {
    if (acceptGrammarSelectionFieldRangeMapping(mappings[index] ?? 'identity')) found = true;
  }
  if (!found) return 0;
  if (inToken) return 2;
  for (let index = 0; index < mappings.length; index += 1) {
    if (!acceptGrammarSelectionFieldRangeMapping(mappings[index] ?? 'identity')) return 2;
  }
  return 1;
}

/** Unknown roles refuse; legacy operation field orders remain separate.
 * @param {string} role @returns {string}
 */
export function readGrammarSelectionFieldRangeFieldOrder(role) {
  if (role === 'candidate') return 'kind|name|mapping|selector';
  if (role === 'selector') return 'kind|label|mode|first|count';
  if (role === 'occurrence') return 'path|start|end';
  if (role === 'derivation') return 'operation|input|start|end|selector|occurrences|ranges|witness';
  if (role === 'mapping') return 'kind|operation|input|output|selector|occurrences|ranges|intervals';
  return '';
}

/** Only actual non-trivia capture fields match. Child and plane ownership is
 * established by the physical tree traversal, never by this label comparison.
 * @param {string} actualField @param {string} label @param {boolean} trivia
 * @returns {boolean}
 */
export function acceptGrammarSelectionFieldRangeMatch(actualField, label, trivia) {
  return !trivia && typeof actualField === 'string' && typeof label === 'string'
    && label !== '' && actualField === label;
}

/** Every supplied absolute span is checked, including unselected occurrences.
 * The host supplies topmost physical occurrences in their original child order;
 * this operation never sorts, merges overlaps or discovers a CST by callbacks.
 * Output is relative included pairs for the unchanged scalar-aware B68 mapper.
 * @param {number[]} spans @param {number} start @param {number} end
 * @param {number} first @param {number} count @param {string} mode
 * @returns {number[]}
 */
export function readGrammarSelectionFieldRangePlan(spans, start, end, first, count, mode) {
  if (!Number.isInteger(start) || !Number.isInteger(end) || start < 0
      || start >= end || end > 4294967295
      || !Number.isInteger(first) || first < 0 || first > 4294967295
      || !Number.isInteger(count) || count <= 0 || count > 4294967295
      || (mode !== 'include' && mode !== 'exclude')
      || spans.length === 0 || spans.length % 2 !== 0 || spans.length > 2 * (end - start)) return [];
  const available = spans.length / 2;
  if (first >= available || count > available - first) return [];
  const following = first + count;
  const ranges = [];
  let previous = start, cursor = start, width = 0;
  for (let index = 0; index < spans.length; index += 2) {
    const left = spans[index] ?? -1;
    const right = spans[index + 1] ?? -1;
    if (!Number.isInteger(left) || !Number.isInteger(right) || left < previous
        || left < start || left >= right || right > end) return [];
    previous = right;
    const occurrence = index / 2;
    if (occurrence >= first && occurrence < following) {
      if (mode === 'include') {
        ranges.push(left - start, right - start);
        width += right - left;
      } else {
        if (cursor < left) {
          ranges.push(cursor - start, left - start);
          width += left - cursor;
        }
        cursor = right;
      }
    }
  }
  if (mode === 'exclude' && cursor < end) {
    ranges.push(cursor - start, end - start);
    width += end - cursor;
  }
  if (width <= 0 || width >= end - start) return [];
  return ranges;
}

/** Field selection follows physical child roles only. Identity embed roots
 * remain opaque, even though their witness retains them for byte coverage.
 * @param {string} type @param {boolean} sourceChildren @returns {string}
 */
export function readGrammarSelectionFieldRangeChildren(type, sourceChildren) {
  if (type === 'node') return 'children';
  if (type === 'embed') return sourceChildren ? 'sourceChildren' : 'none';
  if (type === 'token' || type === 'error' || type === 'missing') return 'none';
  return '';
}
