// Generated from the linked Rust cast syntax policy. Do not edit.
/** @param {string} followingToken @param {boolean} parenthesizedType @returns {boolean} */
export function acceptRustCastFollowingToken(followingToken, parenthesizedType) {
  return followingToken !== "<" || parenthesizedType;
}
/** @returns {string} */
export function readRustCastSyntaxMessage() {
  return "a cast before < requires parentheses";
}
