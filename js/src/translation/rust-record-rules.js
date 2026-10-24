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
