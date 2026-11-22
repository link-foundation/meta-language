/** @param {number} encoded @param {number} decoded @param {number} previous @returns {boolean} */
export function documentStreamFitsBudget(encoded, decoded, previous) {
  return encoded <= 16777216 && decoded <= 16777216 && previous + decoded <= 33554432;
}

/** @param {string} name @returns {number} */
export function pdfStreamFilterKind(name) {
  if (name === 'FlateDecode' || name === 'Fl') return 1;
  if (name === 'ASCIIHexDecode' || name === 'AHx') return 2;
  if (name === 'ASCII85Decode' || name === 'A85') return 3;
  return 0;
}

/** @param {string} operator @returns {number} */
export function pdfTextOperatorKind(operator) {
  if (operator === 'Tf') return 1;
  if (operator === 'Do') return 2;
  if (operator === 'Tj') return 3;
  if (operator === 'TJ') return 4;
  if (operator === "'" || operator === '"') return 5;
  if (operator === 'BT' || operator === 'ET' || operator === 'T*') return 6;
  if (operator === 'Td' || operator === 'TD' || operator === 'Tm') return 7;
  return 0;
}

/** @param {string} operator @returns {boolean} */
export function pdfDrawingOperator(operator) {
  return operator === 'q' || operator === 'Q' || operator === 'cm' || operator === 'm' || operator === 'l' || operator === 'c' || operator === 're' || operator === 'S' || operator === 's' || operator === 'f' || operator === 'f*' || operator === 'F' || operator === 'B' || operator === 'B*' || operator === 'b' || operator === 'b*' || operator === 'n' || operator === 'sh';
}

/** @param {string} type @returns {number} */
export function wordprocessingRelationshipOrder(type) {
  if (type === 'http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument' || type === 'http://purl.oclc.org/ooxml/officeDocument/relationships/officeDocument') return 0;
  if (type === 'http://schemas.openxmlformats.org/officeDocument/2006/relationships/header' || type === 'http://purl.oclc.org/ooxml/officeDocument/relationships/header') return 1;
  if (type === 'http://schemas.openxmlformats.org/officeDocument/2006/relationships/footer' || type === 'http://purl.oclc.org/ooxml/officeDocument/relationships/footer') return 2;
  if (type === 'http://schemas.openxmlformats.org/officeDocument/2006/relationships/footnotes' || type === 'http://purl.oclc.org/ooxml/officeDocument/relationships/footnotes' ||
      type === 'http://schemas.openxmlformats.org/officeDocument/2006/relationships/endnotes' || type === 'http://purl.oclc.org/ooxml/officeDocument/relationships/endnotes') return 3;
  return 4;
}
