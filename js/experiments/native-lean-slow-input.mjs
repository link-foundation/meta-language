// Times the native Lean parse of the generative run-time case 37 that took 20 s, and of
// prefixes and reductions of it, to find what makes native recovery slow on it.
// Usage: node experiments/native-lean-slow-input.mjs [source]
import { LinkNetwork } from '../src/index.js';

const sources = process.argv[2] !== undefined ? [process.argv[2]] : [
  '\nelab "ferm : ​"ferm : term <= bar =>doLean.Elab.Term.elabTerm x (',
  'elab "ferm : ​"ferm : term <= bar =>doLean.Elab.Term.elabTerm x (',
  'elab "a"ferm : term <= bar =>doLean.Elab.Term.elabTerm x (',
  'elab "a"ferm : term <= bar =>do x (',
  'elab "a"f : term <= bar =>do x (',
  'elab "a"f : term <= bar =>do (',
  'elab "a"f : term <= bar => (',
  'elab "a"f : term <= bar => x (',
  'elab "a"f : term <= bar =>doLean.Elab.Term.elabTerm x',
];
for (const source of sources) {
  const started = performance.now();
  const network = LinkNetwork.parse(source, 'Lean');
  const ms = Math.round(performance.now() - started);
  console.log(`${ms} ms ${JSON.stringify(source)} clean=${network.verifyFullMatch().isClean()}`);
}
