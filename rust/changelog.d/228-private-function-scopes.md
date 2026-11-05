Draft separate JavaScript private function-expression and parameter scopes in
both CST analyzers. Normalize source-library private names through existing
capture-preserving renames, retaining original source and diagnostics. Add
deferred recursion, defaults, shadowing and capture-conflict controls; generate
the shared syntax definition automatically.
