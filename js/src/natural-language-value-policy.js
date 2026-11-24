/** @param {number} year @param {number} month @returns {number} */
export function calendarMonthDays(year, month) {
  if (month === 2) {
    if (year % 4 === 0 && (year % 100 !== 0 || year % 400 === 0)) return 29;
    return 28;
  }
  if (month === 4 || month === 6 || month === 9 || month === 11) return 30;
  if (month >= 1 && month <= 12) return 31;
  return 0;
}

/** @param {string} locale @param {string} delimiter @returns {boolean} */
export function preferredGroupingDelimiter(locale, delimiter) {
  if (locale === 'ru') return delimiter === '.';
  return delimiter === ',';
}
