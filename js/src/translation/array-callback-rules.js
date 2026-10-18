/** @param {string} operation @returns {string} */
export function readArrayCallbackResultForm(operation) {
  if (operation === 'map') return 'mapped array';
  if (operation === 'filter') return 'receiver';
  if (operation === 'find') return 'optional element';
  if (operation === 'findIndex') return 'float';
  if (operation === 'some' || operation === 'every') return 'bool';
  if (operation === 'flatMap') return 'callback array';
  if (operation === 'forEach') return 'unit';
  if (operation === 'reduce') return 'accumulator';
  return '';
}

/** @param {string} operation @returns {string} */
export function readArrayCallbackExpectedResultForm(operation) {
  if (operation === 'filter' || operation === 'find' || operation === 'findIndex' || operation === 'some' || operation === 'every') return 'bool';
  if (operation === 'reduce') return 'accumulator';
  if (operation === 'flatMap') return 'array';
  return '';
}

/** @param {string} operation @returns {number} */
export function readArrayCallbackMaximumArguments(operation) {
  if (operation === 'reduce') return 2;
  return readArrayCallbackResultForm(operation) === '' ? -1 : 1;
}

/** @param {string} operation @param {string} kind @param {number} declaredCount @param {number} providedCount @param {boolean} parametersMatch @param {boolean} resultMatches @param {string} expectedResult @returns {string} */
export function readArrayCallbackSignatureError(operation, kind, declaredCount, providedCount, parametersMatch, resultMatches, expectedResult) {
  if (kind !== 'function') return '.' + operation + '() requires a function';
  if (declaredCount > providedCount || !parametersMatch) return '.' + operation + '() callback parameters differ';
  return expectedResult !== '' && !resultMatches ? '.' + operation + '() callback result must be ' + expectedResult : '';
}

/** @param {string} operation @param {number} position @returns {string} */
export function readArrayCallbackParameterRole(operation, position) {
  if (operation === 'reduce') {
    if (position === 0) return 'accumulator';
    if (position === 1) return 'element';
    if (position === 2) return 'index';
    if (position === 3) return 'receiver';
    return '';
  }
  if (position === 0) return 'element';
  if (position === 1) return 'index';
  if (position === 2) return 'receiver';
  return '';
}

/** @param {string} operation @param {number} position @param {string} suffix @param {boolean} cloneElement @returns {string} */
export function renderArrayCallbackArgument(operation, position, suffix, cloneElement) {
  const role = readArrayCallbackParameterRole(operation, position);
  if (role === 'accumulator') return 'ml_accumulator' + suffix;
  if (role === 'element') return 'ml_value' + suffix + (cloneElement ? '.clone()' : '');
  if (role === 'index') return '(ml_index' + suffix + ' as f64)';
  if (role === 'receiver') return 'ml_values' + suffix + '.clone()';
  return '';
}

/** @param {string} operation @param {number} parameterCount @param {string} suffix @param {boolean} cloneElement @returns {string} */
export function renderArrayCallbackCall(operation, parameterCount, suffix, cloneElement) {
  let argumentsText = '';
  let position = 0;
  const maximum = operation === 'reduce' ? 4 : 3;
  while (position < parameterCount && position < maximum) {
    if (position > 0) argumentsText = argumentsText + ', ';
    argumentsText = argumentsText + renderArrayCallbackArgument(operation, position, suffix, cloneElement);
    position += 1;
  }
  return 'ml_callback' + suffix + '(' + argumentsText + ')';
}

/** @param {string} operation @param {string} object @param {string} callback @param {number} parameterCount @param {string} suffix @param {string} initial @param {boolean} hasInitial @returns {string} */
export function renderArrayCallbackExpression(operation, object, callback, parameterCount, suffix, initial, hasInitial) {
  const values = 'ml_values' + suffix;
  const index = 'ml_index' + suffix;
  const value = 'ml_value' + suffix;
  const call = renderArrayCallbackCall(operation, parameterCount, suffix, false);
  const head = 'let ' + values + ' = ' + object + '; let ml_callback' + suffix + ' = ' + callback + ';';
  const iterator = values + '.iter().cloned().enumerate()';
  if (operation === 'some' || operation === 'every') return '{ ' + head + ' ' + iterator + '.' + (operation === 'some' ? 'any' : 'all') + '(|(' + index + ', ' + value + ')| ' + call + ') }';
  if (operation === 'findIndex') return '{ ' + head + ' ' + iterator + '.position(|(' + index + ', ' + value + ')| ' + call + ').map_or(-1f64, |ml_position| ml_position as f64) }';
  if (operation === 'flatMap') return '{ ' + head + ' ' + iterator + '.flat_map(|(' + index + ', ' + value + ')| ' + call + ').collect::<Vec<_>>() }';
  if (operation === 'filter' || operation === 'find') {
    const predicate = renderArrayCallbackCall(operation, parameterCount, suffix, true);
    if (operation === 'filter') return '{ ' + head + ' ' + iterator + '.filter_map(|(' + index + ', ' + value + ')| if ' + predicate + ' { Some(' + value + ') } else { None }).collect::<Vec<_>>() }';
    return '{ ' + head + ' ' + iterator + '.find_map(|(' + index + ', ' + value + ')| if ' + predicate + ' { Some(' + value + ') } else { None }) }';
  }
  if (operation === 'forEach') return '{ ' + head + ' for (' + index + ', ' + value + ') in ' + iterator + ' { ' + call + '; } () }';
  if (operation === 'reduce') {
    if (!hasInitial) return '{ ' + head + ' let mut ml_iterator' + suffix + ' = ' + iterator + '; let ml_initial' + suffix + ' = ml_iterator' + suffix + '.next().map(|(_, ml_first)| ml_first).expect("Reduce of empty array with no initial value"); ml_iterator' + suffix + '.fold(ml_initial' + suffix + ', |ml_accumulator' + suffix + ', (' + index + ', ' + value + ')| ' + call + ') }';
    return '{ ' + head + ' ' + iterator + '.fold(' + initial + ', |ml_accumulator' + suffix + ', (' + index + ', ' + value + ')| ' + call + ') }';
  }
  return '';
}


/** @param {string} operation @param {number} count @returns {string} */
export function readArrayCallbackArgumentCountError(operation, count) {
  const maximum = readArrayCallbackMaximumArguments(operation);
  if (maximum < 0 || count >= 1 && count <= maximum) return '';
  const expected = maximum === 1 ? '1' : '1 to 2';
  return '.' + operation + '() expects ' + expected + ' arguments';
}

/** @param {string} operation @returns {string} */
export function readArrayCallbackCheckedResultContext(operation) {
  const expected = readArrayCallbackExpectedResultForm(operation);
  return expected === 'array' ? '' : expected;
}

/** @param {string} operation @param {string} resultKind @returns {string} */
export function readArrayCallbackResultError(operation, resultKind) {
  return operation === 'flatMap' && resultKind !== 'array' ? 'flatMap callback must return an array' : '';
}

/** @param {string} operation @param {boolean} hasInitial @param {number} position @returns {number} */
export function readArrayCallbackCheckedArgumentPosition(operation, hasInitial, position) {
  if (operation === 'reduce' && hasInitial) {
    if (position === 0) return 1;
    if (position === 1) return 0;
    return -1;
  }
  return position === 0 ? 0 : -1;
}
