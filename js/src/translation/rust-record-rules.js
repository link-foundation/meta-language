// Field-order and binding rules for nominal source records encoded by the
// existing owned datatype/constructor algebra, retaining actual source names.

/** @param {string[]} declared @param {string[]} supplied @returns {number[]} */
export function planRustRecordFields(declared, supplied) {
  for (let index = 0; index < declared.length; index += 1) {
    const name = declared[index] ?? '';
    if (declared.indexOf(name) !== index) return [-1, index];
  }
  for (let index = 0; index < supplied.length; index += 1) {
    const name = supplied[index] ?? '';
    if (supplied.indexOf(name) !== index) return [-2, index];
    if (!declared.includes(name)) return [-3, index];
  }
  const result = [];
  for (let index = 0; index < declared.length; index += 1) {
    const position = supplied.indexOf(declared[index] ?? '');
    if (position < 0) return [-4, index];
    result.push(position);
  }
  return result;
}
/** @param {string[]} declared @param {string} field @returns {number} */
export function readRustRecordFieldPosition(declared, field) { return declared.indexOf(field); }
/** @param {string[]} declared @param {string[]} supplied @param {boolean} rest @returns {number[]} */
export function planRustRecordPatternFields(declared, supplied, rest) {
  const checked = planRustRecordFields(declared, supplied);
  const code = checked[0] ?? 0;
  if (code < 0 && !(code === -4 && rest)) return checked;
  const result = [0];
  for (let index = 0; index < declared.length; index += 1) {
    result.push(supplied.indexOf(declared[index] ?? ''));
  }
  return result;
}
/** @param {string[]} declared @param {string[]} supplied @returns {number[]} */
export function planRustRecordUpdateFields(declared, supplied) {
  return planRustRecordPatternFields(declared, supplied, true);
}
/** @param {boolean} structure @param {boolean} known @param {string} actual @param {string} expected @param {number} wrappers @param {boolean} diverging @returns {boolean} */
export function acceptRustRecordUpdateOwner(structure, known, actual, expected, wrappers, diverging) {
  return structure && (diverging || known && actual === expected && wrappers === 0);
}
/** @param {string[]} reserved @param {number} count @returns {string[]} */
export function readRustRecordBindingNames(reserved, count) {
  const names = [];
  let suffix = 0;
  while (names.length < count) {
    const name = 'ml_record_value' + suffix;
    suffix += 1;
    if (!reserved.includes(name)) names.push(name);
  }
  return names;
}
/** @param {string} name @returns {boolean} */
export function acceptRustRecordSourceName(name) {
  return !['bool','char','str','String','Vec','Box','f32','f64','usize','isize',
    'u8','i8','u16','i16','u32','i32','u64','i64','u128','i128','std','core','alloc'].includes(name);
}

/** @param {string} language @param {string[]} names @returns {boolean} */
export function acceptRustNamedConstructorFields(language, names) {
  if (language !== 'Rust' || names.length === 0) return false;
  for (const name of names) {
    if (name === '_' || name.length === 0) return false;
    for (let index = 0; index < name.length; index += 1) {
      const code = name.charCodeAt(index);
      if (!(code === 95 || code >= 65 && code <= 90 || code >= 97 && code <= 122
          || index > 0 && code >= 48 && code <= 57)) return false;
    }
  }
  const keywords = ['as','async','await','break','const','continue','crate','dyn','else','enum','extern','false','fn','for','if','impl','in','let','loop','match','mod','move','mut','pub','ref','return','self','Self','static','struct','super','trait','true','type','unsafe','use','where','while','abstract','become','box','do','final','gen','macro','override','priv','try','typeof','unsized','virtual','yield'];
  for (const name of names) if (keywords.includes(name)) return false;
  return true;
}
/** @param {string} language @param {string} head @param {string[]} names @param {string[]} values @returns {string} */
export function renderRustConstructorFields(language, head, names, values) {
  if (values.length === 0) return head;
  if (!acceptRustNamedConstructorFields(language, names)) return head + '(' + values.join(', ') + ')';
  const fields = [];
  for (let index = 0; index < values.length; index += 1) fields.push((names[index] ?? '') + ': ' + (values[index] ?? ''));
  return head + ' { ' + fields.join(', ') + ' }';
}
/** @param {boolean} named @returns {string} */
export function renderRustDataFieldEncoding(named) {
  return named
    ? 'a data type is an enum whose constructors retain named or positional fields; data-typed fields are boxed, and values are compared structurally'
    : 'a data type is an enum with one tuple variant per constructor; data-typed fields are boxed, and values are compared structurally';
}
