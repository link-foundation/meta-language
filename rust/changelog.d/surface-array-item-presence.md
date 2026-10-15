`SArrayItem` retains omitted `spread` fields separately from explicit `false`.
Construct array items with `SArrayItem::new(value, spread)` in place of two-field
struct literals, and use `..` in destructuring patterns. The `value` and Boolean
`spread` fields remain public; explicit null and non-Boolean flags remain invalid.
