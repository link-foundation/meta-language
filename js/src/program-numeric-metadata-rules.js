// Generated from linked integral metadata policy. Do not edit.
/** @param {string} kind @param {string} field @param {number} value @param {boolean} negativeZero @returns {boolean} */
export function acceptProgramIntegralMetadata(kind,field,value,negativeZero){
 return (kind === "fixed" && field === "bits" && (value === 8 || value === 16 || value === 32 || value === 64 || value === 128)) && !negativeZero && value >= 0 && value <= 9007199254740991 && value % 1 === 0;
}
