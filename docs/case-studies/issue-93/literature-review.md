# Grammar-Inference Literature Review (issue #93, requirement P-6)

> "Search online best papers on the topic of grammar inference." — issue #93

This review is organised by algorithm family, newest-relevant-first within each.
Every entry gives the citation, the core idea, and the **takeaway for the
meta-language design** (cross-referenced to the proposed issues in
[`solution-plans.md`](./solution-plans.md) and the licence/port notes in
[`library-survey.md`](./library-survey.md)). Researched and verified
2026-06-19.

---

## 0. The theoretical frame — why a structural prior is mandatory

- **Gold, "Language Identification in the Limit," *Information and Control* 10(5):447–474, 1967.**
  ([Semantic Scholar](https://www.semanticscholar.org/paper/Language-Identification-in-the-Limit-Gold/20cc59e8879305cbe18409c77464eff272e1cf55))
  Foundational negative result: **the class of regular (and a fortiori
  context-free) languages is *not* identifiable in the limit from positive data
  alone.** Any learner given only correct strings can be fooled into
  over-generalising.
  **Takeaway (drives P-3, P-4, D6).** Issue #93's "reconstruct a grammar from
  *only* correct texts" is, in full generality, *impossible without inductive
  bias*. Every practical positive-only system therefore injects a bias:
  a structural prior, an Occam/MDL preference for small grammars, or
  approximate/statistical acceptance. **meta-notation's delimiter skeleton is
  exactly such a bias** — which is why P-4 ("inherited from meta-notation") is
  not decoration but the principled core of the approach. The modern SOTA (§4)
  independently converged on the same idea (bracket-guided structuring).

- **de la Higuera, *Grammatical Inference: Learning Automata and Grammars*, Cambridge University Press, 2010.**
  ([review](https://www.researchgate.net/publication/220419140))
  The standard textbook; canonical reference for PTA construction, RPNI, EDSM,
  ALERGIA, OSTIA, and the learning-model taxonomy (identification in the limit,
  PAC, MAT).
  **Takeaway.** Primary clean-room source for D3 (state-merging) and the metric
  definitions in D1.

- **Survey on Grammatical Inference of Formal Languages, *SN Computer Science*, 2025, DOI 10.1007/s42979-025-04715-6.**
  ([Springer](https://link.springer.com/article/10.1007/s42979-025-04715-6))
  Recent (2025) consolidated survey across automata/CFG/picture-language
  inference — used to confirm the family taxonomy below is current.

---

## 1. Query-based / active learning (needs a membership oracle)

- **Angluin, "Learning Regular Sets from Queries and Counterexamples," *Information and Computation* 75(2):87–106, 1987.**
  ([ScienceDirect](https://www.sciencedirect.com/science/article/pii/0890540187900526))
  The **L\*** algorithm and the **MAT (Minimally Adequate Teacher)** model:
  with *membership* + *equivalence* queries, regular languages are *exactly*
  learnable in polynomial time. Maintains an observation table
  (prefixes × distinguishing suffixes), enforces closedness/consistency, refines
  on counterexamples.
- **Isberner, Howar, Steffen, "The TTT Algorithm: A Redundancy-Free Approach to Active Automata Learning," RV 2014.** Space-optimal in counterexample length via three coupled discrimination trees; implemented in **LearnLib** (Apache-2.0).
  **Takeaway (D10, optional).** If the system is allowed an oracle (e.g. an
  existing parser, or a tree-sitter grammar acting as acceptor), L\*/TTT give
  *exact* learning. This is the optional active-learning path; the core
  deliverable (P-3) must not require it. Port from LearnLib (clean-room or JVM
  bridge) — see [`library-survey.md`](./library-survey.md) §C.3.

---

## 2. State-merging / regular inference (passive, from labelled strings)

- **Oncina & García, "Inferring Regular Languages in Polynomial Updated Time," 1992 — RPNI.** Build an (Augmented) Prefix Tree Acceptor from samples, then greedily merge states in a fixed order, accepting a merge only if it keeps the automaton consistent with the negative examples and deterministic.
- **Lang, Pearlmutter, Price, "Results of the Abbadingo One DFA Learning Competition … EDSM," ICGI 1998.** **Evidence-Driven State Merging** scores candidate merges by overlapping accept/reject evidence; won the Abbadingo contest and is the standard high-accuracy upgrade to RPNI.
- **Carrasco & Oncina, "Learning Stochastic Regular Grammars … ALERGIA," ICGI 1994.** Merges *probabilistic* states when symbol/final-state frequencies are statistically compatible under a Hoeffding bound — enables positive-only learning of stochastic automata.
- **Benchmarking State-Merging Algorithms for Learning Regular Languages, PMLR v217, 2023.** ([PDF](https://proceedings.mlr.press/v217/soubki23a/soubki23a.pdf)) Modern empirical comparison; **flexfringe** (GPL) is the maintained reference implementation (APTA + red-blue + EDSM/ALERGIA/MDI, optional SAT via DFASAT).
  **Takeaway (D3).** Port RPNI + EDSM (and optionally ALERGIA for the stochastic
  case) into Rust from the papers / Apache LearnLib / MIT GIToolbox. These cover
  the *regular* sublanguages (token classes, lexer-level structure) that feed
  the CFG layer. Avoid linking GPL flexfringe; it may be run as an external
  oracle for benchmarking only.

---

## 3. CFG induction by compression / Bayesian merging (positive-only, no oracle)

- **Nevill-Manning & Witten, "Identifying Hierarchical Structure in Sequences: A Linear-Time Algorithm (Sequitur)," *JAIR* 7:67–82, 1997.** ([arXiv cs/9709102](https://arxiv.org/abs/cs/9709102)) Online, linear-time CFG induction from a *single* sequence by maintaining two invariants: **digram uniqueness** (no repeated adjacent pair) and **rule utility** (every rule used > once).
- **Solan, Horn, Ruppin, Edelman, "Unsupervised Learning of Natural Languages (ADIOS)," *PNAS* 102(33), 2005.** Pattern-and-equivalence-class distillation over a graph of sequences.
- **Stolcke & Omohundro, "Inducing Probabilistic Grammars by Bayesian Model Merging," ICGI 1994.** MDL/Bayesian prior balancing data fit against grammar size — the formal basis for the Occam objective.
  **Takeaway (D4, D7).** Sequitur is a cheap, unencumbered first pass for
  single-sequence structure; Bayesian/MDL merging is the principled objective
  for the generalization/minimization stage (D7) that turns an over-fit parse
  forest into a compact grammar. Port from the papers (algorithms are free).

---

## 4. Black-box program-input CFG inference — the core line for P-3

This is the family that directly targets "infer a programming-language grammar
from example programs," newest first.

- **NatGI — Arefin, Rahman, Csallner, "Black-box Context-free Grammar Inference for Readable & Natural Grammars," 2025.** ([arXiv 2509.26616](https://arxiv.org/abs/2509.26616)) **Current SOTA.** Extends TreeVada's parse-tree recovery with three innovations: **(1) bracket-guided bubble exploration** (uses delimiters `() [] {}` to propose well-structured fragments), **(2) LLM-driven processing** (generates *meaningful non-terminal names* and selects promising rule merges), **(3) hierarchical delta debugging** to simplify parse trees. Reported **average F1 ≈ 0.57, +25 percentage points over TreeVada**, on a suite up to `lua`, `c`, `mysql`, with more human-readable grammars.
  **Takeaway (D5, D6, D9 — the strategic centre).** NatGI's three innovations
  map *one-to-one* onto assets this project already has or is mandated to use:
  (1) bracket-guidance **is** meta-notation's delimiter skeleton (P-4); (2)
  "meaningful non-terminal names + merge selection" **is** the shared concept
  ontology (P-11) plus the LLM-interop layer; (3) HDD is a standard minimizer
  (D7). The design target is therefore: *match NatGI's pipeline but ground the
  structural prior in meta-notation and the naming/merging in the concept layer*
  — exactly the route to P-12 ("beat all competitors").

- **Kedavra — Li et al., "Incremental Context-free Grammar Inference in Black Box Settings," ASE 2024.** ([arXiv 2408.16706](https://arxiv.org/abs/2408.16706)) Segments example strings into smaller units and infers rules **incrementally** rather than whole-string; reports better precision/recall, runtime, and readability than Arvada/TreeVada under limited example access.
  **Takeaway (D5).** Incremental segmentation is the scalability technique for
  large inputs; fold into the D5 design and the incremental `apply_edit()` path.

- **TreeVada — Arefin, Rahman, Csallner, "Fast Deterministic Black-box Context-free Grammar Inference," ICSE 2024.** ([arXiv 2308.06163](https://arxiv.org/abs/2308.06163); MIT) Deterministic redesign of Arvada: uses bracket/paren priors to pre-structure parse trees and removes nondeterministic search → higher precision/recall/F1 and ~2.4× faster.
  **Takeaway (D5, primary port).** MIT-licensed, deterministic, recent, and its
  bracket prior is precisely the meta-notation hook. **Primary port-to-Rust
  target.**

- **Arvada — Kulkarni, Lemieux, Sen, "Learning Highly Recursive Input Grammars," ASE 2021.** ([arXiv 2108.13340](https://arxiv.org/abs/2108.13340); MIT) Tree-based "bubble-and-merge": start from flat parse trees, iteratively bubble sibling sequences into new non-terminals (introducing recursion) and merge, each validated by a boolean oracle. ~4.98× recall over GLADE on recursive grammars.
- **GLADE — Bastani, Sharma, Aiken, Liang, "Synthesizing Program Input Grammars," PLDI 2017.** ([repo](https://github.com/obastani/glade); Apache-2.0) Oracle-guided regex generalization per seed, then merge subexpressions into a recursive CFG. **The baseline everyone cites**; read the PLDI 2022 critical replication before relying on its numbers.
  **Takeaway (E3).** Arvada and GLADE are the **regression baselines** for the
  benchmark suite. Their corpora define the metrics for P-12.

- **Mimid — Gopinath, Mathis, Zeller, "Mining Input Grammars from Dynamic Control Flow," FSE 2020.** ([ACM](https://dl.acm.org/doi/abs/10.1145/3368089.3409679)) **White-box**: instruments a parser and reads dynamic control flow to recover structure. ⚠️ CC-BY-NC-SA content — **study the idea only** (do not vendor). Different problem (needs the parser's source), but the control-flow signal is a useful idea if a reference parser exists.
- **REINAM — Wu et al., FSE 2019.** RL-guided generalization seeded by symbolic execution. ⚠️ No public repo/licence — **clean-room study only.**

The recall/F1 progression to beat: **GLADE → Arvada (≈5× recall) → TreeVada
(higher F1, 2.4× faster) → Kedavra (incremental, better all-round) → NatGI
(F1 ≈ 0.57, +25 pts over TreeVada).**

---

## 5. Constraint / semantic-invariant inference (beyond context-free)

- **Steinhöfel & Zeller, "Input Invariants (ISLa)," ESEC/FSE 2022.** ([arXiv 2208.12049](https://arxiv.org/abs/2208.12049)) A grammar-aware string-constraint language: first-order logic with quantifiers over *derivation trees* + SMT for atomic string/numeric predicates — expresses things a CFG cannot (def-before-use, length fields, equal counts). ⚠️ GPL — reimplement from the paper.
- **ISLearn (companion).** Mines ISLa invariants from examples: augment via grammar+k-path mutation, instantiate from a *pattern catalog*, filter patterns that don't hold across samples, combine into DNF, rank by specificity & recall. ⚠️ GPL — **clean-room port** of the instantiate→filter→DNF→rank pipeline.
- **Passive Model Learning of Visibly Deterministic Context-free Grammars, arXiv 2508.16305, 2025.** ([PDF](https://arxiv.org/pdf/2508.16305)) State-merging extended to visibly-pushdown / nested-word languages — the bridge between regular state-merging (§2) and full CFGs, and a natural fit for delimiter-structured (meta-notation) inputs.
  **Takeaway (D8, P-5).** "Maximum freedom of grammar inference" (P-5) means the
  representation must carry *semantic* constraints, not just context-free shape.
  D8 adds an ISLearn-style invariant layer over the inferred CFG, reusing the
  existing `TruthValue`/`ProbabilisticTruthValue` semantics and `LinkQuery`.

---

## 6. LLM-assisted inference & grammar-constrained generation

- **Wang, Zhang, et al., "Grammar Prompting for Domain-Specific Language Generation with LLMs," NeurIPS 2023.** Few-shot BNF in the prompt steers an LLM to emit/learn grammar fragments.
- **NatGI (2025, §4)** — the production example of LLMs *inside* an inference loop (naming + merge selection), not just decoding.
- **Grammar-constrained decoding** (GBNF/llama.cpp, XGrammar, Outlines, Guidance/llguidance, SynCode 2024) — consume a portable CFG to constrain generation. See [`library-survey.md`](./library-survey.md) §E.
  **Takeaway (D9, C3, P-11).** Two uses: (a) an LLM as an *optional* heuristic
  inside D5/D9 to propose merges and concept-aligned non-terminal names
  (mirroring NatGI) — always behind a deterministic-fallback flag so P-7's
  "working Rust implementation" never *requires* a model; (b) **emit GBNF** (C3)
  so an inferred meta-grammar can immediately constrain any LLM — a metric NatGI
  et al. do not even report, a free P-12 win.

---

## 7. Synthesis — the design through-line

1. **Gold (1967) makes a structural prior non-negotiable for positive-only
   inference.** meta-notation supplies that prior natively (P-4 → D6).
2. **The SOTA (NatGI, 2025) independently validates the prior**: its top
   innovation is bracket/delimiter guidance — what meta-notation parses by
   construction. The project starts where the SOTA had to bolt structure on.
3. **NatGI's second innovation (meaningful non-terminal names + merge
   selection)** is what the **shared concept ontology** (P-11) provides
   deterministically, with an LLM as an optional accelerator (D9).
4. **The metrics ladder to beat is published** (GLADE→Arvada→TreeVada→
   Kedavra→NatGI); D1 pins them and E3 reproduces the corpora, so "beat all
   competitors in all metrics" (P-12) is measured, not asserted — and the
   project adds metrics the competitors omit (round-trip fidelity across N
   grammar formats, cross-language translation accuracy, GBNF emit for LLM
   constraint).
5. **Licence hygiene is settled** ([`library-survey.md`](./library-survey.md)):
   port the MIT/Apache work (TreeVada, Arvada, GLADE, LearnLib, Sequitur,
   GIToolbox), clean-room the GPL work (ISLa/ISLearn, flexfringe), study-only
   the CC-NC/no-licence work (Mimid, REINAM).
</content>
