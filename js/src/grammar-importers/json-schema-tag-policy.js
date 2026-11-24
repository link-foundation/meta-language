/** @param {string[]} left @param {string[]} right @returns {boolean} */
export function stringSetsAreDisjoint(left, right) {
  for (let index = 0; index < left.length; index += 1) {
    for (let other = 0; other < right.length; other += 1) {
      if (left[index] === right[other]) return false;
    }
  }
  return true;
}
