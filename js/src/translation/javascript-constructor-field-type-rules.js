import { readRequiredNumberFact, readRequiredTextFact } from './fact-column-rules.js';

/** The punctuation column uses the tokenizer's exact tokens and an empty
 * entry for all other token kinds. Quoted punctuation never closes a type.
 * @param {string[]} punctuation @param {number} start @returns {number} */
export function readJavaScriptConstructorFieldBoundary(punctuation, start) {
  if (start < 0 || start % 1 !== 0 || start >= punctuation.length) return -1;
  /** @type {number[]} */
  const closers = [];
  for (let index = start; index < punctuation.length; index += 1) {
    const token = readRequiredTextFact(punctuation, index);
    if (closers.length === 0 && (token === ',' || token === ';' || token === '}')) return index;
    if (token === '{') closers.push(125);
    if (token === '[') closers.push(93);
    if (token === '(') closers.push(41);
    if (token === '<') closers.push(62);
    const angle = token === '>' || token === '>>' || token === '>>>';
    const closing = angle ? 62 : token === '}' ? 125 : token === ']' ? 93 : token === ')' ? 41 : 0;
    const count = angle ? token.length : closing === 0 ? 0 : 1;
    for (let repetition = 0; repetition < count; repetition += 1) {
      if (closers.length === 0 || readRequiredNumberFact(closers, closers.length - 1) !== closing) return -1;
      closers.pop();
    }
  }
  return -1;
}
