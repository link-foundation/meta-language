// Exact admission of integers already represented as binary64 values. Decimal
// spelling, bit equality and source ownership remain independently checked.
/** @param {number} value @param {boolean} signed @param {number} width @returns {boolean} */
export function acceptOwnedOutcomeIntegerValue(value,signed,width) {
  if(width!==32&&width!==64)return false;
  if(!Number.isFinite(value)||!Number.isInteger(value)||value===0&&1/value<0)return false;
  if(signed)return value>=-9223372036854775808&&value<9223372036854775808;
  if(width===32)return value>=0&&value<=4294967295;
  return value>=0&&value<18446744073709551616;
}
