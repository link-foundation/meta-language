// Generated from the original String symbol projection policy. Do not edit.
/** @param {number} value @param {number} maximum @returns {boolean} */
function acceptOriginalLinkSymbolInteger(value, maximum) {
 return Number.isFinite(value) && value >= 0 && value <= maximum && value % 1 === 0 && !(value === 0 && 1 / value < 0);
}
/** @param {Array<number>} starts @param {Array<number>} lengths @param {Array<number>} units @param {boolean} allowAbsent @returns {boolean} */
function checkOriginalLinkSymbolTransport(starts, lengths, units, allowAbsent) {
 if (starts.length !== lengths.length || lengths.length > 100000 || units.length > 100000) return false;
 let end = 0;
 for (let index = 0; index < lengths.length; index++) {
  if (!acceptOriginalLinkSymbolInteger(starts[index] ?? -2, 100000) || starts[index] !== end) return false;
  if (allowAbsent && lengths[index] === -1) continue;
  if (!acceptOriginalLinkSymbolInteger(lengths[index] ?? -2, 100000) || (lengths[index] ?? -2) > units.length - end) return false;
  end = end + (lengths[index] ?? -2);
 }
 if (end !== units.length) return false;
 for (let index = 0; index < units.length; index++) if (!acceptOriginalLinkSymbolInteger(units[index] ?? -2, 65535)) return false;
 return true;
}
/** @param {Array<number>} first @param {number} firstStart @param {number} firstLength @param {Array<number>} second @param {number} secondStart @param {number} secondLength @returns {boolean} */
function equalOriginalLinkSymbolUnits(first, firstStart, firstLength, second, secondStart, secondLength) {
 if (firstLength !== secondLength) return false;
 for (let index = 0; index < firstLength; index++) if (first[firstStart + index] !== second[secondStart + index]) return false;
 return true;
}
/** @param {Array<number>} fixedStarts @param {Array<number>} fixedLengths @param {Array<number>} fixedUnits @param {Array<number>} sourceStarts @param {Array<number>} sourceLengths @param {Array<number>} sourceUnits @returns {Array<number>} */
export function projectOriginalLinkSymbolValues(fixedStarts, fixedLengths, fixedUnits, sourceStarts, sourceLengths, sourceUnits) {
 if (!checkOriginalLinkSymbolTransport(fixedStarts, fixedLengths, fixedUnits, false)) return [-1];
 if (!checkOriginalLinkSymbolTransport(sourceStarts, sourceLengths, sourceUnits, true)) return [-2];
 for (let index = 0; index < fixedLengths.length; index++) {
  for (let prior = 0; prior < index; prior++) {
   if (equalOriginalLinkSymbolUnits(fixedUnits, fixedStarts[index] ?? -2, fixedLengths[index] ?? -2, fixedUnits, fixedStarts[prior] ?? -2, fixedLengths[prior] ?? -2)) return [-3, index];
  }
 }
 let names = [0, 0];
 let units = [];
 let symbols = [];
 for (let index = 0; index < fixedLengths.length; index++) {
  names = [...names, units.length, fixedLengths[index] ?? -2];
  for (let at = 0; at < (fixedLengths[index] ?? -2); at++) units = [...units, fixedUnits[(fixedStarts[index] ?? -2) + at] ?? -2];
  units = [...units, 0];
 }
 for (let index = 0; index < sourceLengths.length; index++) {
  if (sourceLengths[index] === -1) {
   symbols = [...symbols, -1];
  } else {
   let symbol = 0;
   for (let name = 1; name < names.length / 2; name++) {
    if (symbol === 0 && equalOriginalLinkSymbolUnits(sourceUnits, sourceStarts[index] ?? -2, sourceLengths[index] ?? -2, units, names[name * 2] ?? -2, names[name * 2 + 1] ?? -2)) symbol = name;
   }
   if (symbol === 0) {
    symbol = names.length / 2;
    names = [...names, units.length, sourceLengths[index] ?? -2];
    for (let at = 0; at < (sourceLengths[index] ?? -2); at++) units = [...units, sourceUnits[(sourceStarts[index] ?? -2) + at] ?? -2];
    units = [...units, 0];
   }
   symbols = [...symbols, symbol];
  }
 }
 return [1, sourceLengths.length, names.length / 2, units.length, ...symbols, ...names, ...units];
}
