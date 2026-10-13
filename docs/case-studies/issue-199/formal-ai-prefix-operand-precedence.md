# Formal AI census: prefix and operand precedence

The fourteenth batch's Formal AI workload job, `115301511844`, finished its
selected tests successfully but failed the existing self-AST census comparison.
For the pinned `src/agentic_coding/local_search.rs`, the candidate changed
`binary_expression` from 69 to 67 and `unary_expression` from 32 to 34.
The source revision is
[`d209aac6461b355f1a527831202af3423135f7e6`](https://github.com/link-assistant/formal-ai/blob/d209aac6461b355f1a527831202af3423135f7e6/rust/src/agentic_coding/local_search.rs);
the source contains 23,780 bytes, with Git blob
`82b09d73dda676151ab539ea73ad024c7aa1a935`.

Both affected byte spans, `10784..10794` and `11439..11449`, contain
`..*opening` inside an index expression. The pinned grammar oracle constructs a
binary-expression CST whose left field contains the bare range. The native
executor instead constructed a range with a unary-expression endpoint. These
are concrete syntax observations. They do not establish how the Rust compiler
interprets the expression; the real Formal AI source compiled in the CI job.

The imported grammar retained the original productions: the range has
left-associative precedence 1, the unary expression precedence 12, and the
binary expression nine operator alternatives. The defect was in execution:
operand-suffix precedence looked up the union of first-operand references
across a rule's alternatives. An anonymous prefix could consequently borrow
the operand reference of a different alternative.

The repair requires the actual first child's rule, kind or recorded reduction
to match a source operand reference, directly or through complete whole-rule
unit alternatives, before considering that operand's suffix precedence.
A prefix without that evidence retains its production's own
precedence and associativity. The shared Boolean decision is translated from
JavaScript into Rust and adapted through Links Notation decorators. The source
grammar, node names, fields, aliases and census expectations stay intact.

The bounded JavaScript check of this one pinned source produced a clean parse,
69 binary expressions, 32 unary expressions, 4,070 named nodes and 81 distinct
node kinds. All 7,321 CST rows matched the authenticated independent oracle,
including byte spans and field roles. The generated
[`grammar-prefix-operand-precedence.lino`](../../../parity/fixtures/grammar-prefix-operand-precedence.lino)
retains 13 independent oracle controls and the source provenance. Additional
controls retain actual named and anonymously reduced operands, comma contexts
and scanner precedence. Native compilation and the complete downstream census
comparison remain CI verification steps.

The complete unit witness also preserves HCL's source grammar choices. Its
`expression_term` can instantiate a binary or variable node through complete
alternatives without retaining that intermediate name on the child. The bounded
configuration controls include the actual failed source
`a=x != "" && x.y == 1` and its original `real_world.txt: example 2` input.
Their entire CST rows match the independent oracle. The shared unit-closure
algorithm follows complete alternatives and excludes compound sequences, so it
cannot turn a literal operator prefix into a proved operand. The Rust range
controls and the exact pinned Formal AI source retain their original rows.

Only the pinned source file, its census document and authenticated grammar
assets were fetched into temporary storage for this investigation. No upstream
source or external grammar asset is added to the repository.
