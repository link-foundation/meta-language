/** @param {boolean} replacement @returns {boolean} */
export function localAliasMutation(replacement) {
  const original = [false, false];
  const alias = original;
  const distinct = [false, false];
  alias.fill(replacement);
  return original === alias && original !== distinct && original.includes(true) && !distinct.includes(true);
}
