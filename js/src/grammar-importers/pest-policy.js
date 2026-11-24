/** @param {boolean} inherited @param {boolean} triviaRule @param {boolean} atomic @param {boolean} nonAtomic @returns {boolean} */
export function allowPestImplicitTrivia(inherited, triviaRule, atomic, nonAtomic) {
  return !triviaRule && (nonAtomic || (inherited && !atomic));
}
