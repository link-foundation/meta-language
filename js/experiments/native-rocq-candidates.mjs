// Tries hand-written Rocq sources against the native Rocq grammar and its
// tree-sitter-rocq oracle, as the fixture generator would take them: a match
// is a source both accept with the same rows, a rejection one the oracle
// recovers from and the native grammar rejects and repairs. Also checks the
// inventory source and recovery source of Rocq.
//   node experiments/native-rocq-candidates.mjs
import { readFileSync } from 'node:fs';

import { compileGrammar } from '../src/grammar.js';
import { parseGrammarLinks } from '../src/grammar-links.js';
import { nativeOracleKinds } from '../scripts/build-language-catalog.mjs';
import { hasRecovery, nativeRows, oracleRecovers, oracleRows } from '../scripts/native-grammar-rows.mjs';

const text = readFileSync(new URL('../../parity/grammars/native/rocq.lino', import.meta.url), 'utf8');
const parser = compileGrammar(parseGrammarLinks(text));
const options = { hidden: [], anonymous: ['unnamed_token'], extras: ['comment'], oracleKinds: nativeOracleKinds(text) };
const inventory = JSON.parse(readFileSync(new URL('../../parity/language-grammar-inventory.json', import.meta.url), 'utf8'));
const rocq = inventory.languages.find(({ name }) => name === 'Rocq');

const matches = [
  rocq.source,
  '', 'Definition x := 1.\n', 'Definition f (x : nat) : nat := x + 1.\n', 'Theorem t : 1 = 1.\nProof. reflexivity. Qed.\n',
  'Require Import Arith.\n', 'Check nat.\n', 'Compute 1 + 2.\n', '(* c *)\nDefinition x := 1. (* d *)\n', 'Module M.\nDefinition x := 1.\nEnd M.\n',
  'Inductive t : Type := a | b : nat -> t.\n', 'Fixpoint f (n : nat) : nat := match n with | O => 1 | S m => f m end.\n', 'Definition s := "a".\n',
  'Section S.\nVariable n : nat.\nEnd S.\n', 'Definition f := fun x => x.\n',
  'Lemma l : forall n : nat, n = n.\nProof.\n  intros n.\n  destruct n; reflexivity.\nQed.\n', 'Record P := { x : nat; y : nat }.\n',
  'Notation "x ++ y" := (app x y).\n', 'Ltac t := auto.\n', 'Definition p := (1, 2).\n', 'Definition x := if true then 1 else 2.\n',
  'Definition x := let y := 1 in y.\n', 'Open Scope nat_scope.\n', 'Set Implicit Arguments.\n', '#[local] Definition x := 1.\n',
  'Definition l := [1; 2].\n', 'Example e : 1 + 1 = 2.\nProof. simpl. reflexivity. Qed.\n', 'Axiom a : nat.\n', 'Fail Check x.\n',
  'Goal True.\nProof.\n  - exact I.\nQed.\n', 'Definition x := @id nat 1.\n', 'Definition f {A : Type} (x : A) := x.\n',
];
const rejections = [
  rocq.recoverySource,
  'Definition f :=', 'Definition', 'Definition x := 1', 'Definition f (x : nat := x.\n', 'Compute (1 +.\n', 'Definition s := "abc.\n',
  '(* abc\n', 'Module.\n', 'Inductive.\n', 'Definition x := [1; 2.\n', 'Theorem t : := I.\n',
];
for (const source of matches) {
  let verdict;
  if (oracleRecovers(source, 'Rocq')) verdict = 'ORACLE-ERROR';
  else {
    const outcome = parser.parseTree(source);
    if (!outcome.ok) verdict = 'REJECT';
    else verdict = JSON.stringify(nativeRows(outcome.tree, source, options)) === JSON.stringify(oracleRows(source, 'Rocq')) ? 'MATCH' : 'DIFF';
  }
  console.log(`match ${verdict} ${JSON.stringify(source)}`);
}
for (const source of rejections) {
  let verdict;
  if (!oracleRecovers(source, 'Rocq')) verdict = 'ORACLE-ACCEPTS';
  else if (parser.parseTree(source).ok) verdict = 'NATIVE-ACCEPTS';
  else {
    const repaired = parser.parseTree(source, { errorRecovery: true });
    const accepted = parser.parseTree(source, { errorRecovery: true, recovery: 'accept' });
    verdict = repaired.rejection?.reason !== 'recovered' ? 'NO-REPAIR'
      : hasRecovery(nativeRows(accepted.tree, source, options)) ? 'REJECTION' : 'REPAIR-WITHOUT-ERROR';
  }
  console.log(`rejection ${verdict} ${JSON.stringify(source)}`);
}
