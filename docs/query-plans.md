# Executable query plans and the GraphQL adapter

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

## Explicit schema registry

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
common aggregate. Its `equivalentSql` entry is the conformance anchor for the
SQL frontend tracked by issue #187; that adapter can emit and compare the same
canonical plan without changing this IR.
