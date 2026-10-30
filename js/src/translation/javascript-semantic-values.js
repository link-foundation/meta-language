/** Explicit semantic operands for ordinary object operations. Reference IDs
 * belong to the caller's checked global link space; callability is an intrinsic
 * fact of that object model, not a host callback or an inferred property name.
 * Text retains UTF-16 units. Integer values and reference identities remain
 * separate domains, even when their numerical encodings happen to agree.
 * @typedef {{ $:'undefined' } | { $:'null' } | { $:'boolean', value:boolean }
 * | { $:'number', value:number } | { $:'integer', value:bigint }
 * | { $:'text', units:number[] } | { $:'symbol', identity:bigint }
 * | { $:'object', identity:bigint, callable:boolean }} JavaScriptSemanticValue
 */

/** @param {JavaScriptSemanticValue} value @returns {string} */
export function readJavaScriptSemanticValueError(value) {
  switch (value.$) {
    case 'undefined': return '';
    case 'null': return '';
    case 'boolean': return '';
    case 'number': return '';
    case 'integer': return '';
    case 'text': {
      const units = value.units;
      if (units.length > 1048576) return 'semantic text exceeds the explicit UTF16 unit budget';
      for (let at = 0; at < units.length; at += 1) {
        const unit = units[at] ?? -1;
        if (!Number.isInteger(unit) || unit < 0 || unit > 65535) return 'semantic text requires complete ordered UTF16 code units';
      }
      return '';
    }
    case 'symbol': return value.identity > 0n && value.identity <= 18446744073709551615n ? '' : 'a symbol requires a positive unsigned64 global link identity';
    case 'object': return value.identity > 0n && value.identity <= 18446744073709551615n ? '' : 'an object requires a positive unsigned64 global link identity';
  }
}

/** Semantic SameValue, including NaN, signed zero, exact integers, text units,
 * symbol identity and object identity. It does not equate independently named
 * concepts or establish correspondence between different global link spaces.
 * @param {JavaScriptSemanticValue} left @param {JavaScriptSemanticValue} right
 * @returns {boolean} */
export function testJavaScriptSemanticSameValue(left, right) {
  if (readJavaScriptSemanticValueError(left) !== '' || readJavaScriptSemanticValueError(right) !== '')
    throw new TypeError('SameValue requires two complete values in the declared semantic model');
  switch (left.$) {
    case 'undefined': return right.$ === 'undefined';
    case 'null': return right.$ === 'null';
    case 'boolean': {
      if (right.$ !== 'boolean') return false;
      return left.value === right.value;
    }
    case 'number': {
      if (right.$ !== 'number') return false;
      if (left.value === right.value) return left.value !== 0 || 1 / left.value === 1 / right.value;
      return Number.isNaN(left.value) && Number.isNaN(right.value);
    }
    case 'integer': {
      if (right.$ !== 'integer') return false;
      return left.value === right.value;
    }
    case 'text': {
      if (right.$ !== 'text') return false;
      const leftUnits = left.units;
      const rightUnits = right.units;
      if (leftUnits.length !== rightUnits.length) return false;
      for (let at = 0; at < leftUnits.length; at += 1)
        if ((leftUnits[at] ?? -1) !== (rightUnits[at] ?? -1)) return false;
      return true;
    }
    case 'symbol': {
      if (right.$ !== 'symbol') return false;
      return left.identity === right.identity;
    }
    case 'object': {
      if (right.$ !== 'object') return false;
      return left.identity === right.identity;
    }
  }
}

/** @param {JavaScriptSemanticValue} value @returns {boolean} */
export function testJavaScriptSemanticAccessor(value) {
  if (readJavaScriptSemanticValueError(value) !== '') return false;
  if (value.$ === 'undefined') return true;
  if (value.$ !== 'object') return false;
  return value.callable;
}
