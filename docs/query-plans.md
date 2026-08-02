# Executable query plans and query-language adapters

`meta-language` exposes a language-neutral query plan in Rust and JavaScript.
The plan is deliberately independent of GraphQL, SQL, a database engine, and a
particular application schema. Query frontends map their own names into these
canonical concepts:

- `select`, `insert`, `update`, and `delete` operations;
- resource and projection field names;
- recursive `and`, `or`, and `not` filters with `eq`, `neq`, `lt`, `lte`, `gt`,
  `gte`, `in`, `not-in`, `like`, and `is-null` comparisons;
- ordered sort entries, limit, offset, and grouping fields;
- `count`, `sum`, `avg`, `min`, `max`, population variance, and population
  standard deviation aggregates; and
- canonical mutation assignments.

`QueryPlan::canonical_json()` in Rust and `QueryPlan.canonicalJson()` in
JavaScript omit source-language provenance. Equivalent frontends can therefore
compare the same stable version-1 object. Provenance remains available on the
plan as source evidence and in the returned links network.

## GraphQL schema registry

GraphQL schema names never become executable concepts implicitly. A caller must
register every accepted root, argument, projection/input field, and aggregate.
The shared JSON form is:

```json
{
  "roots": [
    {
      "sourceOperation": "query",
      "sourceField": "users",
      "operation": "select",
      "resource": "user",
      "arguments": {
        "where": "filter",
        "orderBy": "order",
        "first": "limit",
        "skip": "offset",
        "groupBy": "group"
      },
      "fields": {
        "id": "user.id",
        "name": "user.name",
        "status": "user.status"
      },
      "aggregates": {
        "count": "count"
      }
    }
  ]
}
```

Argument roles are `filter`, `order`, `limit`, `offset`, `group`, and
`mutation-input`. GraphQL enum field references are matched case-insensitively
against registered source field names, so `NAME` resolves through a `name`
entry without making the canonical domain name schema-dependent. Duplicate or
case-ambiguous entries fail closed.

Rust:

```rust
use meta_language::{lower_graphql, GraphQlSchemaRegistry};

let registry_json = serde_json::json!({
    "roots": [{
        "sourceOperation": "query",
        "sourceField": "users",
        "operation": "select",
        "resource": "user",
        "fields": {"id": "user.id"}
    }]
});
let registry = GraphQlSchemaRegistry::from_json(&registry_json)?;
let lowered = lower_graphql("query { users { id } }", &registry)?;
println!("{}", lowered.plan().canonical_json());
# Ok::<(), Box<dyn std::error::Error>>(())
```

JavaScript:

```js
import { GraphQlSchemaRegistry, lowerGraphQl } from 'meta-language';

const registry = GraphQlSchemaRegistry.fromJson({
  roots: [{
    sourceOperation: 'query',
    sourceField: 'users',
    operation: 'select',
    resource: 'user',
    fields: { id: 'user.id' },
  }],
});
const lowered = lowerGraphQl('query { users { id } }', registry);
console.log(lowered.plan().canonicalJson());
```

Builder APIs (`GraphQlRootMapping` plus `register_root`/`registerRoot`) provide
the same extension point without JSON.

## GraphQL input conventions and safety boundary

The adapter accepts one validated query or mutation with exactly one root field.
It supports literal values, flat mapped projection fields, the standard filter
object keys above, ordering entries shaped as `{field: NAME, direction: ASC}`,
field lists for grouping, and aggregate selections with an optional
`field: NAME` argument. Mutation assignments come from a mapped
`mutation-input` object. Integer literals must fit JavaScript's exact integer
range (`-(2^53 - 1)` through `2^53 - 1`) so Rust and JavaScript plans cannot
silently disagree.

Unknown roots, arguments, fields, aggregate names, operators, or directions are
errors. Multiple operations/root fields, fragments, directives, variables,
block strings, nested input/projection objects, root aliases, and non-aggregate
projection aliases are also rejected instead of being partially interpreted.
Aggregate aliases are retained explicitly. Callers should bind and validate
variable values before lowering; accepting a syntax tree is not execution
authorization.

Rust first validates the operation with the registered GraphQL tree-sitter
grammar. Both ports then use the same bounded semantic parser and parity
fixtures. The returned network contains the original GraphQL source/CST and
`Semantic` links for plan evidence; those links reference syntax evidence and
carry exact byte/row/column spans.

The shared fixture at
[`parity/fixtures/graphql-query-plans.json`](../parity/fixtures/graphql-query-plans.json)
covers every operation, filter composition, ordering, pagination, grouping, and
common aggregate.

## SQL schema registry and vendor profiles

SQL relation and column names also never become executable concepts implicitly.
`SqlSchemaRegistry` maps each accepted source relation to a canonical resource
and each accepted column to a canonical field. Unknown, duplicate, or
case-ambiguous mappings fail closed.

The adapter accepts these case-insensitive language keys:

- `sql-ansi`
- `sql-postgres`
- `sql-mysql`
- `sql-sqlite`
- `sql-server`
- `sql-oracle`
- `sql-bigquery`
- `sql-snowflake`

All profiles use the `tree-sitter-sequel` grammar as their full-match CST
baseline and normalize the supported common subset. Vendor-only constructs are
not silently guessed: callers can handle them in a separate explicit frontend
that constructs the same public `QueryPlan`.

Rust:

```rust
use meta_language::{
    lower_sql, QueryAuthorization, SqlRelationMapping, SqlSchemaRegistry,
};

let mut registry = SqlSchemaRegistry::new();
registry.register_relation(
    SqlRelationMapping::new("users", "user")
        .with_field("id", "user.id")
        .with_field("status", "user.status"),
)?;
let lowered = lower_sql(
    "SELECT id FROM users WHERE status = 'ACTIVE'",
    "sql-postgres",
    &registry,
)?;
assert_eq!(lowered.plan().authorization(), QueryAuthorization::Required);
# Ok::<(), Box<dyn std::error::Error>>(())
```

JavaScript:

```js
import {
  QueryAuthorization,
  SqlRelationMapping,
  SqlSchemaRegistry,
  lowerSql,
} from 'meta-language';

const registry = new SqlSchemaRegistry().registerRelation(
  new SqlRelationMapping('users', 'user')
    .withField('id', 'user.id')
    .withField('status', 'user.status'),
);
const lowered = lowerSql(
  "SELECT id FROM users WHERE status = 'ACTIVE'",
  'sql-postgres',
  registry,
);
console.assert(lowered.plan().authorization() === QueryAuthorization.Required);
```

The bounded common subset covers one-table CRUD; explicit projections;
recursive boolean comparisons; grouping, ordering, limits, and offsets; and
`COUNT`, `SUM`, `AVG`, `MIN`, `MAX`, `VAR_POP`, and `STDDEV_POP`. Constructs the
version-1 IR cannot express—such as joins, multi-row inserts, non-aggregate
projection aliases, distinct queries, parameters, or vendor extensions—are
rejected. Successful lowering is validation, never authorization: an execution
engine must still apply identity, capability, resource, and mutation policy.

[`parity/fixtures/sql-query-plans.json`](../parity/fixtures/sql-query-plans.json)
drives Rust and JavaScript conformance for CRUD, aggregates, all vendor keys,
invalid input, and provenance. It also lowers the `equivalentSql` entry from the
GraphQL fixture and proves that both frontends produce the identical canonical
version-1 plan.
