Improve self-translation report import and export discovery by reading lexical
headers before full checked translation, with grammar fallback for unsupported
lexical syntax. Preserve every source measurement, byte-coverage assertion,
report shard and deadline.

Keep generated decimal experiment code in a separate Rust module so its entry
point does not collide with the native oracle harness. Authenticate temporary
grammar inputs with tracked-index evidence before checking their ignore rules,
and retain Git diagnostics when evidence cannot be obtained. Native decimal
parity and full requirements verification remain pending CI.
