/** @param {string} value @returns {number} */
function codeUnits(value) { return value.length; }
/** @returns {string} */
function observe() { console.log('once'); return '😀'; }
console.log(codeUnits(''), codeUnits('abc'), codeUnits('é中'), codeUnits('😀𝄞'), codeUnits('e\u0301'), codeUnits('\u0000'), codeUnits('\u{10ffff}'), observe().length);
