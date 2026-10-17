// Bounded record formatting draft. These portable decisions are translated
// automatically; admission of actual objects belongs to the separate adapter.
// The Node model is limited to printable ASCII, shallow ordinary data records,
// and the default inspector's single-line layout. It is not production support.
// Default layout decisions are checked against Node's actual implementation:
// https://github.com/nodejs/node/blob/v22.x/lib/internal/util/inspect.js
// Ordinary coercion follows the standard OrdinaryToPrimitive operation:
// https://tc39.es/ecma262/multipage/abstract-operations.html#sec-ordinarytoprimitive

/** @param {string} model @returns {boolean} */
export function acceptRecordTextModel(model) {
  return model === 'ecmascript-ordinary-record-coercion'
    || model === 'node-console-record-inspection';
}

/** @param {string} text @param {boolean} first @returns {boolean} */
export function acceptRecordKeyCharacter(text, first) {
  const letters = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz_$';
  if (letters.includes(text)) return true;
  return !first && '0123456789'.includes(text);
}

/** @param {string} text @returns {boolean} */
export function acceptUnquotedRecordKey(text) {
  if (text.length === 0) return false;
  let index = 0;
  while (index < text.length) {
    if (!acceptRecordKeyCharacter(text.slice(index, index + 1), index === 0)) return false;
    index += 1;
  }
  return true;
}

/** @param {string} text @returns {string} */
export function quoteRecordText(text) {
  let quote = "'";
  if (text.includes("'")) {
    if (!text.includes('"')) quote = '"';
    else if (!text.includes('`') && !text.includes('${')) quote = '`';
  }
  let rendered = quote;
  let index = 0;
  while (index < text.length) {
    const character = text.slice(index, index + 1);
    if (character === '\\') rendered += '\\\\';
    else if (character === quote) rendered += '\\' + character;
    else if (character === '\n') rendered += '\\n';
    else if (character === '\r') rendered += '\\r';
    else if (character === '\t') rendered += '\\t';
    else rendered += character;
    index += 1;
  }
  return rendered + quote;
}

/** @param {string} key @returns {string} */
export function renderRecordKey(key) {
  if (key === '__proto__') return "['__proto__']";
  return acceptUnquotedRecordKey(key) ? key : quoteRecordText(key);
}

/**
 * Preorder nodes keep each subtree's exclusive end index. A record's direct
 * fields are visited by advancing over complete child subtrees, so recursive
 * definitions are retained rather than flattened into a rendered payload.
 * @param {string[]} kinds @param {string[]} keys @param {string[]} values
 * @param {number[]} ends @param {number} index @param {number} indentation @returns {string}
 */
export function renderRecordNode(kinds, keys, values, ends, index, indentation) {
  const kind = kinds[index] ?? '';
  const value = values[index] ?? '';
  const end = ends[index] ?? index;
  if (kind === 'string') return quoteRecordText(value);
  if (kind === 'bigint') return value + 'n';
  if (kind === 'number' || kind === 'boolean') return value;
  if (kind !== 'record') return '';
  let text = '';
  let child = index + 1;
  while (child < end) {
    if (child > index + 1) text += ', ';
    const renderedChild = renderRecordNode(kinds, keys, values, ends, child, indentation + 2);
    if (renderedChild === '') return '';
    text += renderRecordKey(keys[child] ?? '') + ': ' + renderedChild;
    const next = ends[child] ?? child;
    if (next <= child || next > end) return '';
    child = next;
  }
  if (text === '') return '{}';
  const rendered = '{ ' + text + ' }';
  // For ordinary objects Node's default compact=3 layout counts each field,
  // the current indentation, opening brace and a fixed margin before joining.
  // The bound here is that same decision in terms of the joined text length.
  return rendered.length + indentation + 9 <= 80 ? rendered : '';
}

/**
 * The adapter must validate the complete record tape before calling this rule.
 * An empty result is an unsupported-model/layout signal, never an observation.
 * @param {string[]} kinds @param {string[]} keys @param {string[]} values
 * @param {number[]} ends @param {string} model @returns {string}
 */
export function renderRecordText(kinds, keys, values, ends, model) {
  if (!acceptRecordTextModel(model) || kinds.length === 0 || kinds[0] !== 'record') return '';
  if (model === 'ecmascript-ordinary-record-coercion') return '[object Object]';
  return renderRecordNode(kinds, keys, values, ends, 0, 0);
}
