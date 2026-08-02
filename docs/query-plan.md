# Executable Query Plan

`QueryPlan` is a public, engine-neutral semantic representation shared by the
Rust and JavaScript packages. It covers `SELECT`, `INSERT`, `UPDATE`, and
`DELETE`; projections, predicates, grouping, sorting, limits, and offsets; and
`COUNT`, `SUM`, `AVG`, `MIN`, `MAX`, population variance, and population
standard deviation.

The canonical `QueryOperation` is deliberately separate from `SourceEvidence`.
Equivalent statements lowered through different vendor profiles therefore have
the same operation while retaining their original language key, source span,
and (in Rust) the full-match CST syntax link.

## SQL profiles

The built-in registry accepts these case-insensitive keys:

- `sql-ansi`
- `sql-postgres`
- `sql-mysql`
- `sql-sqlite`
- `sql-server`
- `sql-oracle`
- `sql-bigquery`
- `sql-snowflake`

They share the common-subset `tree-sitter-sequel` CST baseline. Common syntax is
normalized; vendor-only syntax is rejected unless a caller registers a custom
frontend and represents the construct with an explicit `Extension` expression.

```rust
use meta_language::{lower_sql, QueryAuthorization};

let plan = lower_sql(
    "SELECT region, COUNT(*) AS total FROM sales GROUP BY region",
    "sql-postgres",
)?;
assert_eq!(plan.authorization(), QueryAuthorization::Required);
# Ok::<(), meta_language::QueryPlanError>(())
```

```js
import { lowerSql, QueryAuthorization } from 'meta-language';

const plan = lowerSql(
  'SELECT region, COUNT(*) AS total FROM sales GROUP BY region',
  'sql-postgres',
);
console.assert(plan.authorization() === QueryAuthorization.Required);
```

## Fail-closed execution boundary

Lowering requires exactly one complete, clean statement. Malformed CSTs,
unsupported trailing input, invalid pagination, and inconsistent `INSERT`
column/value counts return an error. A successful parse is never an execution
authorization: every plan reports `Required`, so the consuming engine must
apply identity, capability, resource, and mutation policy before execution.

`QueryPlan::declare_in` / `QueryPlan.declareIn` materializes operation,
projection, predicate, aggregate, grouping, ordering, pagination, mutation, and
source concepts as regular semantic links. `QueryPlanRegistry` is the extension
point for other query languages; a registered frontend can construct the same
canonical operation and therefore share downstream validation and execution.

Cross-runtime behavior is fixed by
[`parity/query-plan-fixtures.json`](../parity/query-plan-fixtures.json), including
one custom non-SQL frontend that produces the same operation as SQL.
