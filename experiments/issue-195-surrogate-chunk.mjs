// Places an astral character so its UTF-16 surrogate pair straddles web-tree-sitter's
// 5119-code-unit input chunk boundary and reports whether the public parser stays clean.
//   node experiments/issue-195-surrogate-chunk.mjs
import { LinkNetwork } from '../js/src/index.js';

for (const [language, prefix, suffix] of [
  ['JavaScript', 'const s = "', '";\n'],
  ['Rust', "const C: char = '", "';\n"],
  ['Lean', 'def s : String := "', '"\n'],
  ['Rocq', 'Definition s := "', '".\n'],
]) {
  for (const offset of [5117, 5118, 5119]) {
    const padding = offset - prefix.length;
    const source = language === 'Rust'
      ? `${'/'.repeat(2)}${' '.repeat(padding - 3)}\n${prefix}😀${suffix}`
      : `${prefix}${'a'.repeat(padding)}😀${suffix}`;
    const network = LinkNetwork.parse(source, language);
    console.log(`${language} high surrogate at ${source.indexOf('😀')}: clean=${network.verifyFullMatch().isClean()}`);
  }
}
