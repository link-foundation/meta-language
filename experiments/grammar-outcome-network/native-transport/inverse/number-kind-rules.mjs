// Predicates describe the original native Number, before any binary64 projection.
// The caller must authenticate the actual linked artifact and its feature set.

/** @param {boolean} profileAvailable @param {string} version @param {boolean} arbitraryPrecision @param {boolean} signed @param {boolean} unsigned @param {boolean} floating @returns {string} */
export function readOwnedOutcomeNativeNumberKind(profileAvailable,version,arbitraryPrecision,signed,unsigned,floating) {
  if(profileAvailable!==true||version!=='1.0.151'||arbitraryPrecision!==false)return '';
  if(signed!==true&&signed!==false)return '';
  if(unsigned!==true&&unsigned!==false)return '';
  if(floating!==true&&floating!==false)return '';
  if(floating){
    if(signed||unsigned)return '';
    return 'floating point';
  }
  if(unsigned)return 'unsigned integer';
  if(signed)return 'signed integer';
  return '';
}
