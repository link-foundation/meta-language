Reject JavaScript reads and assignments before a later declarator initializes,
including builtin names such as `NaN`, `Infinity` and `undefined`. Preserve actual
block and loop scopes, nearest shadows and left-to-right initializer effects.
Unproved deferred reads and mutable module captures now refuse explicitly instead
of becoming global constants. The shared decision is translated automatically to
Rust; native execution and full requirements acceptance remain pending CI.
