// Shared decimal and hexadecimal digit identity from actual source.
/** @param {number} unit @returns {number} */
export function readJavaScriptNumericDigitUnit(unit) {
  if(unit>=48&&unit<=57)return unit-48;
  if(unit>=65&&unit<=70)return unit-55;
  return unit>=97&&unit<=102?unit-87:-1;
}
