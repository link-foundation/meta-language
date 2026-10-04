// Minimal Rust snippets taken from real published crates that the pinned upstream grammar rejects.
//   node experiments/issue-195-rust-gaps.mjs
import { LinkNetwork } from '../js/src/index.js';

const cases = {
  tildeTokenTree: 'm!(~~ ~);\n',
  dollarTokenTree: 'macro_rules! m { ($mode:ident, $) => { 1 }; [$] => { 2 }; }\n',
  astralCharLiteral: "const C: char = '𐲝';\n",
  bmpCharLiteral: "const C: char = 'é';\n",
  unitStructWhere: 'struct _Test\nwhere\n    Error: Send + Sync;\n',
  attributedFieldPattern: 'fn f(t: T) { match t { S { #[cfg(a)] inner: x, .. } => {} } }\n',
};
for (const [name, source] of Object.entries(cases)) {
  const network = LinkNetwork.parse(source, 'Rust');
  console.log(`${name}: clean=${network.verifyFullMatch().isClean()}`);
}
