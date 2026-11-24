# Source state transition typed Links draft

The additive `sourceStateTransition` expression facet retains an exact Rust
model, initialize/write operation, actual source binder and direct closed
referent position. The source frontend and region interpreter establish source
ownership; structural graph validation alone does not grant that authority.
Region revisions and runtime allocation identities are not serialized as type
positions. The existing signature position validator still checks the actual
referent against the expression type and declaration owners.

The source schema extension adds the optional field to every expression kind
without removing predecessor fields. Checked expression projection retains the
facet. Datatype reference walking visits source positions and transitions before
lexical let/lambda/match early returns, so lexical scoping does not erase their
actual datatype edges. Existing type/binding walks and bounds remain in force.

Deferred controls cover additive/idempotent vocabulary, conflicting labels,
projection copying, unsupported languages/operations and forbidden serialized
revision fields. They are structural inputs, not evidence of admitted original
source. The region batch supplies separate actual-source proof/control paths.

Source generation, graph round trips, JavaScript observations and native products
remain deferred until the full requirements draft permits execution. No tests,
validation gates, generation, formatting or compilation ran for this batch.
