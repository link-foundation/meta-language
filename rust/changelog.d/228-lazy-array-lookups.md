Draft lazy `unwrap_or_else` consumers for immutable Rust get/first/last lookups
through shared branch rules and known zero-argument callable results. Preserve
eager callable construction, absence-only invocation and scalar source owners.
Independent source, codec, target and native controls remain deferred.

The same source draft includes presence-selected `is_some_and` and `is_none_or`
predicates with checked parameter/result owners and explicit absent defaults.
Actual standard scalar receiver/element owners now supply missing closure
parameter and result types, retaining captures and explicit type distinctions.
