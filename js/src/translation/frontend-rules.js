// Frontend decisions expressed in JavaScript and translated into Rust by
// js/scripts/generate-frontend-rules.mjs. Inputs use code units and token
// records so both host runtimes can use the same decisions.

/** @typedef {{ $: 'scalar', code: number, end: number } | { $: 'malformed' } | { $: 'unsupported', end: number, escapeLength: number }} UnicodeEscape */

/**
 * Read one Unicode escape from at most twelve UTF-16 code units, starting
 * with its backslash. Offsets in the result are relative to that backslash.
 * @param {number[]} units
 * @param {boolean} allowFixed
 * @returns {UnicodeEscape}
 */
export function decodeUnicodeEscape(units, allowFixed) {
  let index = 2;
  let code = 0;
  let digits = 0;
  let braced = false;
  if (units.length > 2 && units[2] === 123) {
    braced = true;
    index = 3;
  } else if (!allowFixed) return { $: 'malformed' };
  while (index < units.length && digits < (braced ? 6 : 4)) {
    const unit = units[index];
    let digit = -1;
    if (unit >= 48 && unit <= 57) digit = unit - 48;
    else if (unit >= 65 && unit <= 70) digit = unit - 55;
    else if (unit >= 97 && unit <= 102) digit = unit - 87;
    if (digit < 0) break;
    code = code * 16 + digit;
    digits += 1;
    index += 1;
  }
  if (braced) {
    if (digits === 0 || index >= units.length || units[index] !== 125) return { $: 'malformed' };
    index += 1;
  } else if (digits !== 4) return { $: 'malformed' };
  const escapeLength = index;
  if (!braced && code >= 55296 && code <= 56319 && units.length >= index + 6 && units[index] === 92 && units[index + 1] === 117) {
    let low = 0;
    let offset = 2;
    while (offset < 6) {
      const unit = units[index + offset];
      let digit = -1;
      if (unit >= 48 && unit <= 57) digit = unit - 48;
      else if (unit >= 65 && unit <= 70) digit = unit - 55;
      else if (unit >= 97 && unit <= 102) digit = unit - 87;
      if (digit < 0) break;
      low = low * 16 + digit;
      offset += 1;
    }
    if (offset === 6 && low >= 56320 && low <= 57343) {
      code = 65536 + (code - 55296) * 1024 + low - 56320;
      index += 6;
    }
  }
  if (code > 1114111 || (code >= 55296 && code <= 57343)) return { $: 'unsupported', end: index, escapeLength };
  return { $: 'scalar', code, end: index };
}

/**
 * Whether every omitted argument has a trailing default.
 * @param {boolean[]} defaults
 * @param {number} count
 * @returns {boolean}
 */
export function acceptArgumentCount(defaults, count) {
  if (count > defaults.length) return false;
  let index = count;
  while (index < defaults.length) {
    if (!defaults[index]) return false;
    index += 1;
  }
  return true;
}

/**
 * Find a reference to an earlier parameter in a default expression. Member
 * names after a dot are not references, and nested delimiters keep commas
 * and closing parentheses inside the expression.
 * @param {string[]} kinds
 * @param {string[]} values
 * @param {string[]} parameters
 * @returns {number}
 */
export function findDefaultParameterReference(kinds, values, parameters) {
  let at = 0;
  let depth = 0;
  while (at < values.length && kinds[at] !== 'eof' && (depth > 0 || (values[at] !== ',' && values[at] !== ')'))) {
    const value = values[at];
    if (value === '(' || value === '[' || value === '{') depth += 1;
    if (value === ')' || value === ']' || value === '}') depth -= 1;
    if (kinds[at] === 'identifier' && !(at > 0 && values[at - 1] === '.')) {
      let parameterIndex = 0;
      while (parameterIndex < parameters.length) {
        if (parameters[parameterIndex] === value) return at;
        parameterIndex += 1;
      }
    }
    at += 1;
  }
  return -1;
}

/**
 * Whether a slash follows a token that starts an expression rather than one
 * that ends it. Comments are removed before this decision.
 * @param {string} kind
 * @param {string} value
 * @returns {boolean}
 */
export function startRegularExpression(kind, value) {
  if (kind === '') return true;
  if (kind === 'identifier') {
    return value === 'return' || value === 'throw' || value === 'case'
      || value === 'delete' || value === 'void' || value === 'typeof'
      || value === 'yield' || value === 'await' || value === 'in' || value === 'of';
  }
  if (kind !== 'punct') return false;
  return value !== ')' && value !== ']' && value !== '}' && value !== '.'
    && value !== '?.' && value !== '++' && value !== '--';
}

/**
 * Read a complete regular expression literal without interpreting its body
 * as numbers, strings or template literals. Offsets are UTF-16 code units.
 * @param {number[]} units
 * @param {number} start
 * @returns {number}
 */
export function regularExpressionEnd(units, start) {
  let index = start + 1;
  let escaped = false;
  let characterClass = false;
  while (index < units.length) {
    const unit = units[index];
    if (unit === 10 || unit === 13 || unit === 8232 || unit === 8233) return -1;
    if (escaped) escaped = false;
    else if (unit === 92) escaped = true;
    else if (unit === 91) characterClass = true;
    else if (unit === 93) characterClass = false;
    else if (unit === 47 && !characterClass) {
      index += 1;
      while (index < units.length) {
        const flag = units[index];
        if (!((flag >= 65 && flag <= 90) || (flag >= 97 && flag <= 122))) break;
        index += 1;
      }
      return index;
    }
    index += 1;
  }
  return -1;
}

/**
 * Find the end of a contiguous declaration run. Non-item groups supply an
 * empty term, so comments and provenance boundaries cannot be absorbed.
 * @param {string[]} terms
 * @param {number} start
 * @returns {number}
 */
export function findBindingRunEnd(terms, start) {
  let end = start;
  while (end < terms.length) {
    const term = terms[end];
    if (term !== 'function_declaration' && term !== 'lexical_declaration' && term !== 'export_statement') break;
    end += 1;
  }
  return end === start ? start + 1 : end;
}

/**
 * Retry a declaration run only when an isolated item failed type resolution.
 * @param {string[]} statuses
 * @param {string[]} reasons
 * @returns {boolean}
 */
export function acceptBindingScope(statuses, reasons) {
  if (statuses.length < 2) return false;
  for (let index = 0; index < statuses.length; index += 1) {
    if (statuses[index] === 'carried' && reasons[index] === 'type') return true;
  }
  return false;
}

/**
 * Immutable primitive literals have no identity, mutation or initialisation
 * effect, so their value can be read from a sibling function's scope.
 * @param {string} valueKind
 * @param {boolean} constant
 * @param {number} effectCount
 * @param {number} declarationCount
 * @returns {boolean}
 */
export function acceptLiteralBinding(valueKind, constant, effectCount, declarationCount) {
  return constant && effectCount === 1 && declarationCount === 0
    && (valueKind === 'num' || valueKind === 'bool' || valueKind === 'str');
}

/** @param {string} valueKind @param {string} typeKind @returns {string} */
export function constantBindingForm(valueKind, typeKind) {
  if (valueKind === 'lit' && (typeKind === 'float' || typeKind === 'bool' || typeKind === 'fixed')) return 'scalar';
  if (valueKind === 'lit' && typeKind === 'string') return 'string';
  return 'lazy';
}

/** @param {boolean} constant @param {number} effects @param {number} declarations @param {boolean} legalName @returns {boolean} */
export function acceptConstantEmission(constant, effects, declarations, legalName) {
  return constant && effects === 1 && declarations === 0 && legalName;
}

/** @param {string} form @param {string} name @param {string} type @param {string} value @returns {string} */
export function renderConstantBinding(form, name, type, value) {
  if (form === 'lazy') return 'pub static ' + name + ': std::sync::LazyLock<' + type + '> = std::sync::LazyLock::new(|| ' + value + ');';
  return 'pub const ' + name + ': ' + type + ' = ' + value + ';';
}

/**
 * Retry a complete declaration module when isolated groups still carry items.
 * Imports, statements and provenance blocks need other binding mechanisms.
 * @param {string[]} terms
 * @param {boolean} carried
 * @returns {boolean}
 */
export function acceptModuleBindingScope(terms, carried) {
  if (!carried) return false;
  let declarations = 0;
  for (let index = 0; index < terms.length; index += 1) {
    const term = terms[index];
    if (term !== '') {
      if (term !== 'function_declaration' && term !== 'lexical_declaration' && term !== 'export_statement') return false;
      declarations += 1;
    }
  }
  return declarations > 0;
}

/**
 * Locate an ASCII parameter name after JSDoc whitespace and an optional `[`.
 * The documented default is metadata; only the function supplies defaults.
 * @param {number[]} units
 * @returns {number[]}
 */
export function findDocumentationParameterRange(units) {
  let start = 0;
  while (start < units.length && (units[start] === 32 || units[start] === 9)) start += 1;
  let optional = false;
  if (start < units.length && units[start] === 91) {
    optional = true;
    start += 1;
  }
  let end = start;
  while (end < units.length) {
    const code = units[end];
    if ((code >= 65 && code <= 90) || (code >= 97 && code <= 122) || code === 95 || code === 36 || (end > start && code >= 48 && code <= 57)) end += 1;
    else break;
  }
  if (end === start) return [];
  if (optional) {
    let after = end;
    while (after < units.length && (units[after] === 32 || units[after] === 9)) after += 1;
    if (after >= units.length || (units[after] !== 93 && units[after] !== 61)) return [];
  }
  return [start, end];
}

/** @param {string} kind @returns {boolean} */
export function acceptTypeQueryOperand(kind) {
  return kind === 'var' || kind === 'lit' || kind === 'name' || kind === 'num' || kind === 'bool' || kind === 'str';
}

/** @param {string} kind @returns {string} */
export function readTypeQueryResult(kind) {
  if (kind === 'float') return 'number';
  if (kind === 'nat' || kind === 'int') return 'bigint';
  if (kind === 'bool') return 'boolean';
  if (kind === 'string') return 'string';
  if (kind === 'array' || kind === 'data') return 'object';
  return '';
}
