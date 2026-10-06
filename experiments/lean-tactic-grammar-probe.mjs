import { LinkNetwork } from '../js/src/index.js';
const variants = {
  paren: 'try (conv => whnf)',
  bare: 'try conv => whnf',
  convLine: 'conv =>\n    whnf',
  dsimp: 'try dsimp only',
  simpDecide: 'try simp (config := { decide := true })',
  showWhnf: 'try (conv => whnf; skip)',
  convIn: 'try conv in _ => whnf',
  first: 'first | decide | skip',
  unfold: 'try unfold f',
  simpOnly: 'try simp only [f]',
};
for (const [name, tactic] of Object.entries(variants)) {
  const code = `def f (n : Nat) : Nat := n\n\ntheorem t : f 1 = 1 := by\n  try rfl\n  ${tactic}\n  try decide\n`;
  const network = LinkNetwork.parse(code, 'lean');
  console.log(name.padEnd(12), network.verifyFullMatch().isClean());
}
