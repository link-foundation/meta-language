# Grammar inference

Owned by [F1](../case-studies/issue-93/proposed-issues/F1-grammar-subsystem-docs.md);
stage owners: [D5](../case-studies/issue-93/proposed-issues/D5-blackbox-cfg-inference.md)
for black-box CFG inference,
[D6](../case-studies/issue-93/proposed-issues/D6-delimiter-structural-prior.md)
for delimiter skeleton priors, and
[D1](../case-studies/issue-93/proposed-issues/D1-inference-evaluation-harness.md)
for evaluation.

Inference turns examples, observations, or parser behavior into a candidate
[`Grammar` IR](architecture.md). The current branch provides the target IR and
native grammar surface; inference algorithms are planned follow-up APIs.

## Minimal example

```text
positive examples + negative examples + optional delimiter prior
  -> infer candidate Grammar
  -> evaluate coverage and counterexamples
  -> emit, generate, or register the accepted grammar
```

Inferred grammars should use `GrammarFormat::Inferred` and retain enough rule
documentation or concept alignment for later review. When an inference step
cannot name a rule confidently, it should still produce stable rule identifiers
so downstream tests can compare the result.

## Inputs

| Input | Purpose | Owner |
| --- | --- | --- |
| Positive examples | Show strings or regions the grammar should accept. | [D5](../case-studies/issue-93/proposed-issues/D5-blackbox-cfg-inference.md) |
| Negative examples | Prevent over-general candidates. | [D5](../case-studies/issue-93/proposed-issues/D5-blackbox-cfg-inference.md) |
| Delimiter skeleton | Bias structure toward balanced groups and separators. | [D6](../case-studies/issue-93/proposed-issues/D6-delimiter-structural-prior.md) |
| Evaluation fixtures | Compare inferred candidates by acceptance and reconstruction behavior. | [D1](../case-studies/issue-93/proposed-issues/D1-inference-evaluation-harness.md) |

Related later refinements include lexical classes
([D2](../case-studies/issue-93/proposed-issues/D2-lexical-class-inference.md)),
regular state merging
([D3](../case-studies/issue-93/proposed-issues/D3-state-merging-regular-inference.md)),
Sequitur compression
([D4](../case-studies/issue-93/proposed-issues/D4-sequitur-compression.md)),
MDL minimization
([D7](../case-studies/issue-93/proposed-issues/D7-generalization-mdl-minimization.md)),
semantic constraints
([D8](../case-studies/issue-93/proposed-issues/D8-semantic-constraint-inference.md)),
LLM-assisted naming
([D9](../case-studies/issue-93/proposed-issues/D9-llm-assisted-naming-merge.md)),
and active learning
([D10](../case-studies/issue-93/proposed-issues/D10-active-learning-oracle.md)).

## Output contract

An inference API should return a `Grammar` plus diagnostics. The grammar should
be ready for:

- [Import and export](import-export.md) emitters, so humans can inspect it in a
  familiar notation.
- [Code generation](codegen.md), so the inferred grammar can be executed.
- [CLI and runtime](cli-and-runtime.md), so an accepted grammar can be registered
  as a parser.

The diagnostics should preserve rejected examples, unresolved ambiguities, and
fallbacks so the evaluation harness can explain why one candidate was selected.
