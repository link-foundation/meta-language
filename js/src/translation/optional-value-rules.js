// Portable optional value decisions and rendering. Host adapters check child
// expressions and provide their actual types and already rendered values.

/** @param {string} kind @param {string} element @returns {boolean} */
export function requireOptionalFlattening(kind, element) {
  return kind === 'option' && element === 'option';
}

/** @param {string} kind @returns {boolean} */
export function requireOptionalCoalescing(kind) {
  return kind === 'option';
}

/** @param {string} language @param {string} kind @param {string} element @returns {string} */
export function readOptionalNumberOperation(language, kind, element) {
  if (language !== 'JavaScript' || kind !== 'option') return 'identity';
  return element === 'float' || element === 'unit' || element === 'bool' ? 'number' : 'unsupported';
}

/** @param {string} language @param {string} kind @returns {string} */
export function readOptionalBooleanOperation(language, kind) {
  return kind === 'bool' ? 'identity' : language === 'JavaScript' ? 'truthy' : 'coerce';
}

/** @param {string} position @param {boolean} coalesced @param {boolean} parenthesized @param {string} kind @param {string} operation @returns {string} */
export function readOptionalLogicalMixingError(position, coalesced, parenthesized, kind, operation) {
  if (position === 'left' && !parenthesized && kind === 'binary' && (operation === 'and' || operation === 'or')) return 'mixing ?? with && or || needs parentheses';
  if (position === 'right' && !parenthesized && kind === 'binary' && operation === 'and') return 'mixing ?? with && needs parentheses';
  return position === 'or' && coalesced ? 'mixing ?? with || needs parentheses' : '';
}

/** @param {string} token @param {boolean} declared @returns {string} */
export function readOptionalSourceLiteral(token, declared) {
  return token === 'null' || token === 'undefined' && !declared ? token : '';
}

/** @param {string} kind @param {string} element @param {boolean} integer @returns {string} */
export function readOptionalBigIntConstraint(kind, element, integer) {
  if (kind === 'option' && element === 'var') return 'mark';
  return kind === 'option' && integer ? 'accept' : 'error';
}

/** @param {boolean} declared @param {boolean} additional @param {string} actual @param {string} result @param {boolean} same @param {boolean} recursive @returns {boolean} */
export function requireOptionalResultBinding(declared, additional, actual, result, same, recursive) {
  return (!declared || additional) && actual !== 'var' && result === 'var' && !same && !recursive;
}

/** @param {string} parameter @param {string} actual @param {string} current @param {string} currentAbsence @param {string} actualAbsence @param {boolean} recursive @returns {string} */
export function readOptionalCallBinding(parameter, actual, current, currentAbsence, actualAbsence, recursive) {
  if (parameter !== 'var' || actual !== 'option') return '';
  if (current === 'option') return currentAbsence !== actualAbsence && currentAbsence !== 'both' ? 'merge' : 'elements';
  return recursive ? '' : 'bind';
}

/** @param {boolean} original @param {boolean} same @param {boolean} recursive @returns {string} */
export function readOptionalJoinBinding(original, same, recursive) {
  if (original) return 'identity';
  if (same) return 'changes';
  return recursive ? 'identity' : 'bind';
}

/** @param {string} kind @returns {string} */
export function readOptionalNumericTerm(kind) {
  return kind === 'option' ? 'element' : kind === 'unit' ? 'float' : 'identity';
}

/** @param {string} language @param {string} expected @param {string} actual @param {string} expression @param {boolean} acceptsActual @param {boolean} acceptsUndefined @param {boolean} sameElement @returns {string} */
export function readOptionalCoercionOperation(language, expected, actual, expression, acceptsActual, acceptsUndefined, sameElement) {
  if (expected !== 'option') return '';
  if (expression === 'none') return acceptsActual ? 'none' : 'absence-error';
  if (language === 'JavaScript' && actual === 'unit' && acceptsUndefined) return expression === 'unit' ? 'unit' : 'discard-unit';
  return sameElement ? 'some' : '';
}

/** @param {string} kind @param {string} element @param {string} field @returns {string} */
export function readOptionalFieldOperation(kind, element, field) {
  if (kind !== 'option') return 'field';
  if (field === 'length' && (element === 'string' || element === 'array')) return 'project';
  if (field === 'size' && (element === 'set' || element === 'map')) return 'project';
  return element === 'record' ? 'optional-field' : 'unsupported';
}

/** @param {string} format @param {string} policy @param {string} absence @param {string} element @returns {string} */
export function renderOptionalAbsentValue(format, policy, absence, element) {
  if (format === 'JavaScript') return absence === 'null' ? 'null' : 'undefined';
  if (format === 'Rust') return policy === 'both' ? 'crate::MlOptional::' + (absence === 'null' ? 'Null' : 'Undefined') : 'None::<' + element + '>';
  if (format === 'Lean') return policy === 'both' && absence === 'undefined' ? '(some none)' : 'none';
  if (format === 'Rocq') return policy === 'both' && absence === 'undefined' ? '(Some (@None ' + element + '))' : '(@None ' + (policy === 'both' ? '(option ' + element + ')' : element) + ')';
  return '';
}

/** @param {string} format @param {string} policy @param {string} value @returns {string} */
export function renderOptionalPresentValue(format, policy, value) {
  if (format === 'JavaScript') return value;
  if (format === 'Rust') return policy === 'both' ? 'crate::MlOptional::Value(' + value + ')' : 'Some(' + value + ')';
  if (format === 'Lean') return policy === 'both' ? '(some (some ' + value + '))' : '(some ' + value + ')';
  if (format === 'Rocq') return policy === 'both' ? '(Some (Some ' + value + '))' : '(Some ' + value + ')';
  return '';
}

/** @param {string} format @param {string} policy @param {string} value @param {string} binder @param {string} present @param {string} nullCase @param {string} undefinedCase @returns {string} */
export function renderOptionalMatch(format, policy, value, binder, present, nullCase, undefinedCase) {
  if (format === 'Rust') return policy === 'both'
    ? 'match ' + value + ' { crate::MlOptional::Value(' + binder + ') => ' + present + ', crate::MlOptional::Null => ' + nullCase + ', crate::MlOptional::Undefined => ' + undefinedCase + ' }'
    : 'match ' + value + ' { Some(' + binder + ') => ' + present + ', None => ' + (policy === 'null' ? nullCase : undefinedCase) + ' }';
  if (format === 'Lean') return '(match ' + value + ' with ' + (policy === 'both'
    ? '| some (some ' + binder + ') => ' + present + ' | none => ' + nullCase + ' | some none => ' + undefinedCase
    : '| some ' + binder + ' => ' + present + ' | none => ' + (policy === 'null' ? nullCase : undefinedCase)) + ')';
  if (format === 'Rocq') return '(match ' + value + ' with ' + (policy === 'both'
    ? '| Some (Some ' + binder + ') => ' + present + ' | None => ' + nullCase + ' | Some None => ' + undefinedCase
    : '| Some ' + binder + ' => ' + present + ' | None => ' + (policy === 'null' ? nullCase : undefinedCase)) + ' end)';
  return '';
}

/** @param {string} format @param {string} policy @param {string} value @param {string} receiver @param {string} fallback @returns {string} */
export function renderOptionalCoalescing(format, policy, value, receiver, fallback) {
  if (format === 'JavaScript') return '(' + value + ' ?? ' + fallback + ')';
  if (format === 'Rust') return policy === 'both' ? 'match ' + value + ' { crate::MlOptional::Value(ml_value) => ml_value, _ => ' + fallback + ' }' : receiver + '.clone().unwrap_or_else(|| ' + fallback + ')';
  if (format === 'Lean') return '(match ' + value + ' with | ' + (policy === 'both' ? 'some (some ml_value)' : 'some ml_value') + ' => ml_value | _ => ' + fallback + ')';
  if (format === 'Rocq') return '(match ' + value + ' with | ' + (policy === 'both' ? 'Some (Some ml_value)' : 'Some ml_value') + ' => ml_value | _ => ' + fallback + ' end)';
  return '';
}

/** @param {string} format @param {string} policy @param {string} absence @returns {string} */
function renderOptionalFlattenAbsence(format, policy, absence) {
  if (format === 'Rust') return policy === 'both' ? 'crate::MlOptional::' + (absence === 'null' ? 'Null' : 'Undefined') : 'None';
  if (format === 'Lean') return policy === 'both' && absence === 'undefined' ? '(some none)' : 'none';
  if (format === 'Rocq') return policy === 'both' && absence === 'undefined' ? '(Some None)' : 'None';
  return '';
}

/** @param {string} format @param {string} outer @param {string} inner @param {string} result @param {string} value @returns {string} */
export function renderOptionalFlattening(format, outer, inner, result, value) {
  if (format === 'JavaScript') return value;
  const nullCase = renderOptionalFlattenAbsence(format, result, 'null');
  const undefinedCase = renderOptionalFlattenAbsence(format, result, 'undefined');
  const present = renderOptionalPresentValue(format, result, 'ml_present');
  const nested = renderOptionalMatch(format, inner, 'ml_nested', 'ml_present', present, nullCase, undefinedCase);
  return renderOptionalMatch(format, outer, value, 'ml_nested', nested, nullCase, undefinedCase);
}

/** @param {string} format @param {string} policy @param {string} value @param {string} receiver @param {string} binder @param {string} present @param {string} field @returns {string} */
export function renderOptionalProjection(format, policy, value, receiver, binder, present, field) {
  if (format === 'JavaScript') return '(' + value + ')?.[' + field + ']';
  if (format === 'Rust') return policy === 'both' ? 'match ' + value + ' { crate::MlOptional::Value(' + binder + ') => Some(' + present + '), _ => None }' : receiver + '.clone().map(|' + binder + '| ' + present + ')';
  if (format === 'Lean') return '(match ' + value + ' with | ' + (policy === 'both' ? 'some (some ' + binder + ')' : 'some ' + binder) + ' => some ' + present + ' | _ => none)';
  if (format === 'Rocq') return '(match ' + value + ' with | ' + (policy === 'both' ? 'Some (Some ' + binder + ')' : 'Some ' + binder) + ' => Some ' + present + ' | _ => None end)';
  return '';
}

/** @param {string} format @param {string} kind @param {string} value @param {string} receiver @param {boolean} utf16 @param {boolean} signed @returns {string} */
export function renderOptionalPrimitiveTruth(format, kind, value, receiver, utf16, signed) {
  if (format === 'JavaScript') return '!!(' + value + ')';
  if (kind === 'bool') return value;
  if (format === 'Rust') {
    if (kind === 'float') return '{ let ml_value = ' + value + '; ml_value != 0f64 && !ml_value.is_nan() }';
    if (kind === 'int' || kind === 'nat') return '!' + receiver + '.is_zero()';
    if (kind === 'fixed') return '(' + receiver + ' != 0)';
    if (kind === 'string') return '!' + receiver + (utf16 ? '.0.is_empty()' : '.is_empty()');
    return '{ let _ = ' + value + '; ' + (kind !== 'unit' ? 'true' : 'false') + ' }';
  }
  if (format === 'Lean') {
    if (kind === 'float') return '(let ml_value := ' + value + '; ml_value != (0 : Float) && !ml_value.isNaN)';
    if (kind === 'int' || kind === 'nat' || kind === 'fixed') return '(' + value + ' != 0)';
    if (kind === 'string') return '(!' + value + '.isEmpty)';
    return '(let ml_ignored := ' + value + '; ' + (kind !== 'unit' ? 'true' : 'false') + ')';
  }
  if (format === 'Rocq') {
    if (kind === 'float') return '(let ml_value := ' + value + ' in andb (negb (PrimFloat.eqb ml_value 0%float)) (negb (PrimFloat.is_nan ml_value)))';
    if (kind === 'int' || kind === 'fixed' && signed) return '(negb (Z.eqb ' + value + ' 0))';
    if (kind === 'nat' || kind === 'fixed') return '(negb (N.eqb ' + value + ' 0))';
    if (kind === 'string') return '(negb (String.eqb ' + value + ' EmptyString))';
    return '(let ml_ignored := ' + value + ' in ' + (kind !== 'unit' ? 'true' : 'false') + ')';
  }
  return '';
}

/** @param {string} format @param {string} kind @param {string} policy @param {string} name @returns {string} */
function renderOptionalEqualityPresent(format, kind, policy, name) {
  if (kind !== 'option') return name;
  if (format === 'Rust') return policy === 'both' ? 'crate::MlOptional::Value(' + name + ')' : 'Some(' + name + ')';
  if (format === 'Lean') return policy === 'both' ? 'some (some ' + name + ')' : 'some ' + name;
  return policy === 'both' ? 'Some (Some ' + name + ')' : 'Some ' + name;
}

/** @param {string} policy @param {number} index @returns {string} */
function readOptionalEqualityAbsence(policy, index) {
  return policy === 'both' ? index === 0 ? 'null' : 'undefined' : policy;
}

/** @param {string} format @param {string} policy @param {string} absence @returns {string} */
function renderOptionalEqualityAbsent(format, policy, absence) {
  if (format === 'Rust') return policy === 'both' ? 'crate::MlOptional::' + (absence === 'null' ? 'Null' : 'Undefined') : 'None';
  if (format === 'Lean') return policy === 'both' && absence === 'undefined' ? 'some none' : 'none';
  return policy === 'both' && absence === 'undefined' ? 'Some None' : 'None';
}

/** @param {string} format @param {string} leftKind @param {string} leftPolicy @param {string} rightKind @param {string} rightPolicy @param {string} compare @param {string} left @param {string} right @param {boolean} negate @returns {string} */
export function renderOptionalStrictEquality(format, leftKind, leftPolicy, rightKind, rightPolicy, compare, left, right, negate) {
  if (format === 'JavaScript') return '(' + left + (negate ? ' !== ' : ' === ') + right + ')';
  const leftPresent = renderOptionalEqualityPresent(format, leftKind, leftPolicy, compare === 'false' ? '_ml_left' : 'ml_left');
  const rightPresent = renderOptionalEqualityPresent(format, rightKind, rightPolicy, compare === 'false' ? '_ml_right' : 'ml_right');
  const prefix = format === 'Rust' ? '' : '| ';
  const separator = format === 'Rust' ? ', ' : ' ';
  let cases = prefix + '(' + leftPresent + ', ' + rightPresent + ') => ' + compare;
  const leftCount = leftKind !== 'option' ? 0 : leftPolicy === 'both' ? 2 : 1;
  const rightCount = rightKind !== 'option' ? 0 : rightPolicy === 'both' ? 2 : 1;
  let first = 0;
  while (first < leftCount) {
    const leftAbsence = readOptionalEqualityAbsence(leftPolicy, first);
    let second = 0;
    while (second < rightCount) {
      const rightAbsence = readOptionalEqualityAbsence(rightPolicy, second);
      const same = leftAbsence === rightAbsence ? 'true' : 'false';
      cases += separator + prefix + '(' + renderOptionalEqualityAbsent(format, leftPolicy, leftAbsence) + ', ' + renderOptionalEqualityAbsent(format, rightPolicy, rightAbsence) + ') => ' + same;
      second += 1;
    }
    first += 1;
  }
  cases += separator + prefix + '_ => false';
  if (format === 'Rust') {
    const result = 'match (' + left + ', ' + right + ') { ' + cases + ' }';
    return negate ? '!(' + result + ')' : result;
  }
  if (format === 'Lean') {
    const result = '(match (' + left + ', ' + right + ') with ' + cases + ')';
    return negate ? '(!' + result + ')' : result;
  }
  if (format === 'Rocq') {
    const result = '(match (' + left + ', ' + right + ') with ' + cases + ' end)';
    return negate ? '(negb ' + result + ')' : result;
  }
  return '';
}

/** @param {string} format @param {string} policy @param {string} value @param {string} receiver @param {string} fallback @returns {string} */
export function renderOptionalUnwrapping(format, policy, value, receiver, fallback) {
  if (format === 'JavaScript') return value;
  if (format === 'Rust') return policy === 'both' ? 'match ' + value + ' { crate::MlOptional::Value(ml_present) => ml_present, _ => unreachable!("a proven present value") }' : receiver + '.clone().expect("a proven present value")';
  if (format === 'Lean') return policy === 'both' ? '(match ' + value + ' with | some (some ml_present) => ml_present | _ => default)' : '(' + value + '.get!)';
  if (format === 'Rocq') return '(match ' + value + ' with | ' + (policy === 'both' ? 'Some (Some ml_present)' : 'Some ml_present') + ' => ml_present | _ => ' + fallback + ' end)';
  return '';
}

/** @param {string} operation @param {string} value @param {string} name @param {boolean} console @returns {string} */
export function renderOptionalJavaScriptOperation(operation, value, name, console) {
  if (operation === 'nullishNumber') return 'Number(' + value + ')';
  if (operation === 'optionalIndexNumber') return value;
  if (operation === 'optionalText') return (console ? 'ml_showOptional(' : 'String(') + value + ')';
  if (operation === 'optionalTypeOf') return '(typeof ' + value + ')';
  if (operation === 'optionalPredicate') return 'Number.' + name + '(' + value + ')';
  return '';
}

/** @param {string} format @param {string} kind @param {string} operation @returns {string} */
export function renderOptionalNumericPresent(format, kind, operation) {
  if (kind === 'unit') return format === 'Rust' ? 'f64::NAN' : format === 'Lean' ? '(0 / 0 : Float)' : 'PrimFloat.nan';
  if (kind === 'bool' && operation === 'arithmetic') return format === 'Rust' ? 'if ml_present { 1f64 } else { 0f64 }' : format === 'Lean' ? '(if ml_present then (1 : Float) else (0 : Float))' : '(if ml_present then 1%float else 0%float)';
  return 'ml_present';
}

/** @param {string} format @param {string} literal @returns {string} */
export function renderOptionalTypeQueryPresent(format, literal) {
  return format === 'Rust' ? '{ let _ = ml_present; ' + literal + ' }' : literal;
}

/** @param {string} kind @returns {boolean} */
export function requireOptionalNumericPredicate(kind) {
  return kind === 'float';
}

/** @param {string} format @param {string} present @param {boolean} bigint @param {boolean} console @param {boolean} utf16 @returns {string} */
export function renderOptionalConsoleText(format, present, bigint, console, utf16) {
  if (!bigint || !console) return present;
  if (format === 'Rust') return utf16 ? '(' + present + ').concat(&crate::ml_string::Text::from_text("n"))' : 'format!("{}n", ' + present + ')';
  if (format === 'Lean') return '(' + present + ' ++ "n")';
  if (format === 'Rocq') return '(String.append ' + present + ' "n"%string)';
  return present;
}

/** @returns {string} */
export function readOptionalJavaScriptTextPrelude() {
  return "function ml_showOptional(value) {\n  if (value === null) return 'null';\n  if (value === undefined) return 'undefined';\n  if (typeof value === 'bigint') return String(value) + 'n';\n  return Object.is(value, -0) ? '-0' : String(value);\n}";
}


/** @param {string} kind @returns {boolean} */
export function acceptOptionalPrimitiveKind(kind) {
  return kind === 'bool' || kind === 'float' || kind === 'fixed' || kind === 'int' || kind === 'nat' || kind === 'string' || kind === 'unit';
}

/** @param {string} language @param {string} left @param {string} right @param {boolean} leftElement @param {boolean} rightElement @param {boolean} same @param {string} operation @returns {string} */
export function readOptionalLogicalOperation(language, left, right, leftElement, rightElement, same, operation) {
  if (left === 'bool' && right === 'bool') return 'boolean';
  if (language !== 'JavaScript') return 'language-error';
  if (left === 'option' && leftElement) return operation === 'or' ? 'unwrap-left' : 'wrap-right';
  if (right === 'option' && rightElement) return 'wrap-left';
  return same ? 'identity' : 'union-error';
}

/** @param {string} language @param {string} left @param {string} right @param {string} operation @param {boolean} same @param {boolean} primitive @returns {string} */
export function readOptionalComparisonOperation(language, left, right, operation, same, primitive) {
  if (language !== 'JavaScript' || left !== 'option' && right !== 'option') return '';
  if (operation === 'eq' || operation === 'ne') return same && !primitive ? 'object-error' : 'strict';
  return 'number';
}


/** @param {string} policy @returns {string[]} */
export function readOptionalOutputAbsences(policy) {
  return policy === 'both' ? ['null', 'undefined'] : [policy];
}


/** Preserve an existing unresolved alias when a deferred branch join gains
 * its value after a call has already linked the result to a parameter.
 * @param {string} kind @param {number} alias @param {number} original @returns {boolean} */
export function requireInferenceJoinAlias(kind, alias, original) {
  return kind === 'var' && alias !== original;
}


/** A type policy may admit both cases; an actual absent value has one tag.
 * @param {string} absence @returns {boolean} */
export function acceptOptionalLiteralAbsence(absence) {
  return absence === 'null' || absence === 'undefined';
}
