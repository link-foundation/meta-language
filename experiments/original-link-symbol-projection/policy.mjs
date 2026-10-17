// Generator inputs. Runtime decisions use the generated policy and source.
export const originalLinkSymbolPolicy = {
 version: 1,
 model: 'ordered original String value symbol projection 1',
 maximumStrings: 100000,
 maximumUnits: 100000,
 input: ['fixed starts', 'fixed lengths', 'fixed UTF16 units', 'source starts', 'source lengths', 'source UTF16 units'],
 transport: 'Dense primitive Number vectors. Offsets are consecutive; all units are consumed. Source length minus one represents a non-String value and consumes no units. Fixed values are distinct Strings. No source link identity is replaced or inferred from equal text.',
 output: ['decision', 'source value count', 'name count including null', 'unit count', 'ordered source symbols', 'name offset and length pairs', 'UTF16 units with one zero delimiter per String'],
 refusals: { fixed: [-1], source: [-2], duplicateFixed: [-3, 'second fixed position'] },
 translation: { referenceSemantics: false, meaning: 'Primitive array values are immutable inputs. Local accumulation selects value semantics explicitly; no shared mutable references are observed.' },
 boundaries: [
  'The null-name row has symbol zero and no delimiter. A non-String source has symbol minus one. Empty Strings have positive symbols and their own delimiter.',
  'Exact ordered UTF16 code units determine value equality. Normalization, spelling aliases, source owners and semantic equivalence are separate decisions.',
  'The ordered fixed dictionary precedes first-observed dynamic values. This kernel does not choose the traversal order or authenticate scalar owners in an original graph.',
  'The complete JavaScript kernel is automatically emitted as Rust source. Native compilation, native execution, resource behavior and the remaining six dynamic certificate tables are unverified.',
  'Repeated scans and immutable accumulation may be quadratic. This is a bounded projection model, not a performance improvement.',
 ],
};

export function originalLinkSymbolRulesSource() {
 const {maximumStrings, maximumUnits} = originalLinkSymbolPolicy;
 return `// Generated from the original String symbol projection policy. Do not edit.
/** @param {number} value @param {number} maximum @returns {boolean} */
function acceptOriginalLinkSymbolInteger(value, maximum) {
 return Number.isFinite(value) && value >= 0 && value <= maximum && value % 1 === 0 && !(value === 0 && 1 / value < 0);
}
/** @param {Array<number>} starts @param {Array<number>} lengths @param {Array<number>} units @param {boolean} allowAbsent @returns {boolean} */
function checkOriginalLinkSymbolTransport(starts, lengths, units, allowAbsent) {
 if (starts.length !== lengths.length || lengths.length > ${maximumStrings} || units.length > ${maximumUnits}) return false;
 let end = 0;
 for (let index = 0; index < lengths.length; index++) {
  if (!acceptOriginalLinkSymbolInteger(starts[index] ?? -2, ${maximumUnits}) || starts[index] !== end) return false;
  if (allowAbsent && lengths[index] === -1) continue;
  if (!acceptOriginalLinkSymbolInteger(lengths[index] ?? -2, ${maximumUnits}) || (lengths[index] ?? -2) > units.length - end) return false;
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
`;
}
