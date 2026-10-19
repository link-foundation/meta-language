// Complete primitive method checking and rendering decisions. Host adapters
// evaluate actual children in order and construct their ordinary typed nodes.

/** @param {boolean} units @param {string} form @param {string} value @param {string} other @param {string} operation @returns {string} */
export function renderPrimitiveStringRepresentation(units, form, value, other, operation) {
  if (form === 'type') return units ? 'crate::ml_string::Text' : 'String';
  if (form === 'literal') return units ? 'crate::ml_string::Text::from_units(vec![' + value + '])' : 'String::from(' + value + ')';
  if (form === 'constant') return units ? 'crate::ml_string::Text::from_text(' + value + ')' : 'String::from(' + value + ')';
  if (form === 'examples') return units ? 'vec![crate::ml_string::Text::from_text(""), crate::ml_string::Text::from_text("a"), crate::ml_string::Text::from_text("ab")]' : 'vec![String::new(), String::from("a"), String::from("ab")]';
  if (form === 'text') return units ? 'crate::ml_string::Text::from_text(&' + value + ')' : value;
  if (form === 'concat') return units ? value + '.concat(' + other + ')' : 'format!("{}{}", ' + value + ', ' + other + ')';
  if (!units) return '';
  if (form === 'length') return value + '.length()';
  if (form === 'test') {
    const method = operation === 'startsWith' ? 'starts_with' : operation === 'endsWith' ? 'ends_with' : 'contains';
    return value + '.' + method + '(' + other + ')';
  }
  if (form === 'map') {
    if (operation === 'toLowerCase' || operation === 'toUpperCase') return value + '.map_case(' + (operation === 'toUpperCase' ? 'true' : 'false') + ')';
    const start = operation !== 'trimEnd' && operation !== 'rustTrimEnd';
    const end = operation !== 'trimStart' && operation !== 'rustTrimStart';
    const rust = operation.startsWith('rust');
    return value + '.trim(' + (start ? 'true' : 'false') + ', ' + (end ? 'true' : 'false') + ', ' + (rust ? 'true' : 'false') + ')';
  }
  return '';
}

/** @param {string} kind @returns {string} */
export function readPrimitiveCopySliceForm(kind) {
  if (kind === 'array' || kind === 'string') return 'identity';
  return kind === 'record' ? 'method' : 'refuse';
}

/** @param {string} receiver @param {string} operation @returns {number} */
export function readPrimitiveMethodMinimumArguments(receiver, operation) {
  if (receiver === 'array' && (operation === 'slice' || operation === 'join')) return 0;
  if (receiver === 'array' && (operation === 'includes' || operation === 'indexOf' || operation === 'lastIndexOf')) return 1;
  if (receiver !== 'string') return -1;
  if (operation === 'split' || operation === 'slice' || operation === 'charCodeAt' || operation === 'charAt') return 0;
  if (operation === 'indexOf' || operation === 'lastIndexOf' || operation === 'repeat' || operation === 'padStart' || operation === 'padEnd' || operation === 'codePointAt') return 1;
  if (operation === 'replace' || operation === 'replaceAll') return 2;
  return -1;
}

/** @param {string} receiver @param {string} operation @returns {number} */
export function readPrimitiveMethodMaximumArguments(receiver, operation) {
  if (receiver === 'array') {
    if (operation === 'join') return 1;
    if (operation === 'slice' || operation === 'includes' || operation === 'indexOf' || operation === 'lastIndexOf') return 2;
    return -1;
  }
  if (receiver !== 'string') return -1;
  if (operation === 'repeat' || operation === 'codePointAt' || operation === 'charCodeAt' || operation === 'charAt') return 1;
  if (operation === 'split' || operation === 'slice' || operation === 'indexOf' || operation === 'lastIndexOf' || operation === 'replace' || operation === 'replaceAll' || operation === 'padStart' || operation === 'padEnd') return 2;
  return -1;
}

/** @param {string} receiver @param {string} operation @returns {number} */
export function readPrimitiveMethodArgumentCount(receiver, operation) {
  return readPrimitiveMethodMaximumArguments(receiver, operation);
}

/** @param {string} receiver @param {string} operation @param {number} index @returns {string} */
export function readPrimitiveMethodCheckedArgumentKind(receiver, operation, index) {
  if (receiver === 'array' && operation === 'join') return 'string';
  if (receiver === 'array' && operation === 'slice') return 'float';
  if (receiver === 'array' && (operation === 'includes' || operation === 'indexOf' || operation === 'lastIndexOf')) return index === 0 ? 'element' : 'float';
  if (receiver !== 'string') return '';
  if (operation === 'charCodeAt' || operation === 'charAt') return 'number-value';
  if (operation === 'replace' || operation === 'replaceAll') return 'string';
  if (index === 0 && (operation === 'split' || operation === 'indexOf' || operation === 'lastIndexOf')) return 'string';
  if (index === 1 && (operation === 'padStart' || operation === 'padEnd')) return 'string';
  return 'float';
}

/** @param {string} receiver @param {string} operation @param {number} index @returns {string} */
export function readPrimitiveMethodDefaultForm(receiver, operation, index) {
  if (receiver === 'array' && operation === 'join') return 'string';
  if (receiver === 'array' && (operation === 'includes' || operation === 'indexOf' || operation === 'lastIndexOf') && index === 1) return 'float';
  if (receiver === 'string' && operation === 'split' && index === 0) return 'absent';
  if (receiver === 'string' && operation === 'split' && index === 1) return 'float';
  if (receiver === 'string' && (operation === 'charCodeAt' || operation === 'charAt') && index === 0) return 'float';
  if ((receiver === 'string' || receiver === 'array') && operation === 'slice') return 'float';
  if (receiver === 'string' && (operation === 'indexOf' || operation === 'lastIndexOf') && index === 1) return 'float';
  if (receiver === 'string' && (operation === 'padStart' || operation === 'padEnd') && index === 1) return 'string';
  return 'required';
}

/** @param {string} receiver @param {string} operation @param {number} index @returns {string} */
export function readPrimitiveMethodDefaultValue(receiver, operation, index) {
  if (receiver === 'array' && operation === 'join') return ',';
  if (receiver === 'array' && (operation === 'includes' || operation === 'indexOf') && index === 1) return '0';
  if (receiver === 'array' && operation === 'lastIndexOf' && index === 1) return 'Infinity';
  if (receiver === 'string' && operation === 'split' && index === 1) return '4294967295';
  if (receiver === 'string' && (operation === 'charCodeAt' || operation === 'charAt') && index === 0) return '0';
  if ((receiver === 'string' || receiver === 'array') && operation === 'slice') return index === 0 ? '0' : 'Infinity';
  if (receiver === 'string' && operation === 'indexOf' && index === 1) return '0';
  if (receiver === 'string' && operation === 'lastIndexOf' && index === 1) return 'Infinity';
  if (receiver === 'string' && (operation === 'padStart' || operation === 'padEnd') && index === 1) return ' ';
  return '';
}

/** @param {string} receiver @param {string} operation @returns {string} */
export function readPrimitiveMethodCheckedResultForm(receiver, operation) {
  if (receiver === 'array' && operation === 'join') return 'string';
  if (receiver === 'array' && operation === 'slice') return 'receiver';
  if (receiver === 'array' && operation === 'includes') return 'bool';
  if (receiver === 'array' && (operation === 'indexOf' || operation === 'lastIndexOf')) return 'float';
  if (receiver !== 'string') return '';
  if (operation === 'split') return 'array-string';
  if (operation === 'indexOf' || operation === 'lastIndexOf' || operation === 'charCodeAt') return 'float';
  if (operation === 'codePointAt') return 'optional-float';
  return 'string';
}

/** @param {string} receiver @param {string} operation @param {string} element @returns {string} */
export function readPrimitiveMethodArgumentObligation(receiver, operation, element) {
  if (receiver === 'array' && operation === 'join') {
    const form = readPrimitiveArrayJoinElementForm(element);
    return form === 'refuse' || form === 'optional' ? 'joining structured values needs their JavaScript string conversion and cycle semantics' : '';
  }
  if (receiver !== 'array' || (operation !== 'includes' && operation !== 'indexOf' && operation !== 'lastIndexOf')) return '';
  if (element === 'array' || element === 'record' || element === 'map' || element === 'set' || element === 'data' || element === 'function') return 'object identity must be represented before searching arrays of objects';
  return '';
}

/** @param {string} operation @param {string} element @param {string} absence @returns {string} */
export function renderPrimitiveArraySearchComparator(operation, element, absence) {
  const present = operation === 'includes' && element === 'float' ? 'a == b || (a.is_nan() && b.is_nan())' : 'a == b';
  if (absence === 'both') return '|a, b| match (a, b) { (crate::MlOptional::Value(a), crate::MlOptional::Value(b)) => ' + present + ', (crate::MlOptional::Null, crate::MlOptional::Null) | (crate::MlOptional::Undefined, crate::MlOptional::Undefined) => true, _ => false }';
  if (absence === 'null' || absence === 'undefined') return '|a, b| match (a, b) { (Some(a), Some(b)) => ' + present + ', (None, None) => true, _ => false }';
  if (operation === 'includes' && element === 'float') return '|a: &f64, b: &f64| ' + present;
  return '|a, b| ' + present;
}

/** @param {string} operation @param {string} object @param {string} search @param {string} from @param {string} element @param {string} absence @returns {string} */
export function renderPrimitiveArraySearchExpression(operation, object, search, from, element, absence) {
  const comparator = renderPrimitiveArraySearchComparator(operation, element, absence);
  return 'crate::ml_array::' + (operation === 'includes' ? 'includes' : 'index_of') + '(' + object + ', &' + search + ', ' + from + ', ' + (operation === 'lastIndexOf' ? 'true' : 'false') + ', ' + comparator + ')';
}

/** @param {string} target @param {string} receiver @param {string} operation @returns {string} */
export function readPrimitiveMethodRenderingRefusal(target, receiver, operation) {
  if (target !== 'Lean' && target !== 'Rocq') return '';
  if (receiver !== 'array') return 'this target needs a UTF-16 operational string encoding for this operation';
  if (operation === 'slice' || operation === 'includes' || operation === 'indexOf' || operation === 'lastIndexOf') return 'this array operation needs an operational encoding in ' + target;
  return '';
}

/** @param {string} target @param {string} receiver @param {string} operation @param {number} index @param {boolean} absent @param {string} text @returns {string} */
export function renderPrimitiveMethodArgument(target, receiver, operation, index, absent, text) {
  if (target === 'JavaScript' && absent) return 'undefined';
  if (target === 'Rust' && receiver === 'string' && operation === 'split' && index === 0) return absent ? 'None' : 'Some(' + text + ')';
  return text;
}

/** @param {string} target @param {string} receiver @param {string} operation @param {string} object @param {string} argumentsText @returns {string} */
export function renderPrimitiveMethodExpression(target, receiver, operation, object, argumentsText) {
  if (target === 'JavaScript') return '(' + object + ').' + operation + '(' + argumentsText + ')';
  if (target === 'Rust' && receiver === 'array' && operation === 'slice') return 'crate::ml_array::slice(' + object + ', ' + argumentsText + ')';
  if (target !== 'Rust' || receiver !== 'string') return '';
  if (operation === 'indexOf') return object + '.index_of(' + argumentsText + ')';
  if (operation === 'lastIndexOf') return object + '.last_index_of(' + argumentsText + ')';
  if (operation === 'codePointAt') return object + '.code_point_at(' + argumentsText + ')';
  if (operation === 'replace' || operation === 'replaceAll') return object + '.replace(' + argumentsText + ', ' + (operation === 'replaceAll' ? 'true' : 'false') + ')';
  if (operation === 'padStart' || operation === 'padEnd') return object + '.pad(' + argumentsText + ', ' + (operation === 'padStart' ? 'true' : 'false') + ')';
  return object + '.' + operation + '(' + argumentsText + ')';
}

/** @param {string} expression @param {string} receiver @returns {boolean} */
export function requirePrimitiveMethodUtf16(expression, receiver) {
  return expression === 'method' && receiver === 'string';
}

/** @param {string} receiver @param {string} operation @returns {string} */
export function readPrimitiveMethodLoweringForm(receiver, operation) {
  if (receiver === 'string' && operation === 'charAt') return 'character-template';
  return receiver === 'string' && operation === 'charCodeAt' ? 'code-unit-template' : 'method';
}

/** @param {string} kind @returns {string} */
export function readPrimitiveArrayJoinElementForm(kind) {
  if (kind === 'string') return 'string';
  if (kind === 'unit') return 'empty';
  if (kind === 'option') return 'optional';
  if (kind === 'bool' || kind === 'float' || kind === 'fixed' || kind === 'int' || kind === 'nat') return 'text';
  return 'refuse';
}

/** @param {string} operation @returns {string} */
export function readPrimitiveMethodArgumentObligationFeature(operation) {
  return operation === 'join' ? 'array join' : 'array identity search';
}

/** @param {boolean} units @param {boolean} stringElement @param {string} operand @returns {string} */
export function readPrimitiveArrayJoinOperandForm(units, stringElement, operand) {
  return units && (operand === 'separator' || stringElement) ? 'borrow' : 'receiver';
}

/** @param {string} kind @param {string} absence @param {string} name @param {string} present @param {string} empty @returns {string} */
export function renderPrimitiveArrayJoinElement(kind, absence, name, present, empty) {
  const form = readPrimitiveArrayJoinElementForm(kind);
  if (form === 'empty') return empty;
  if (form === 'optional') {
    if (absence === 'both') return 'match ' + name + ' { crate::MlOptional::Value(ml_present) => ' + present + ', _ => ' + empty + ' }';
    return 'match ' + name + ' { Some(ml_present) => ' + present + ', None => ' + empty + ' }';
  }
  return present;
}

/** @param {boolean} units @param {boolean} stringElement @param {string} object @param {string} separator @param {string} elementText @returns {string} */
export function renderPrimitiveArrayJoinExpression(units, stringElement, object, separator, elementText) {
  if (stringElement) return units ? 'crate::ml_string::join(' + object + ', ' + separator + ')' : object + '.join(' + separator + '.as_str())';
  const values = object + '.iter().cloned().map(|ml_value| ' + elementText + ').collect::<Vec<_>>()';
  return units ? 'crate::ml_string::join(&' + values + ', ' + separator + ')' : '(' + values + ').join(' + separator + '.as_str())';
}
