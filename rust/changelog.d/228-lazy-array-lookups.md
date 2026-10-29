Draft lazy `unwrap_or_else` consumers for immutable Rust get/first/last lookups
through shared branch rules and known zero-argument callable results. Preserve
eager callable construction, absence-only invocation and scalar source owners.
Independent source, codec, target and native controls remain deferred.
