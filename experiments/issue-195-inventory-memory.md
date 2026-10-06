# Finite inventory memory probe

The script parses each committed language-inventory source once and checks
exact reconstruction. It stops if the observed process RSS exceeds 1,200 MB.
The command also bounds the JavaScript heap and WebAssembly linear memory:

```sh
node --max-old-space-size=512 --wasm-max-mem-pages=8192 --expose-gc experiments/issue-195-inventory-memory.mjs
```

On Node 24.21.0 in the 3 GB contributor workspace, the optimizing WebAssembly
compiler increases RSS while the JavaScript heap remains near 11 MB. Limiting
compilation to one task still exceeds the probe's RSS bound:

```sh
node --wasm-num-compilation-tasks=1 --max-old-space-size=512 --wasm-max-mem-pages=8192 --expose-gc experiments/issue-195-inventory-memory.mjs
```

The same finite inventory completes near 295 MB when WebAssembly uses only
its baseline compiler:

```sh
node --liftoff-only --max-old-space-size=512 --wasm-max-mem-pages=8192 --expose-gc experiments/issue-195-inventory-memory.mjs
```

This diagnoses a local verification resource limit. It does not establish
native grammar support, change parser behavior, or provide acceptance records.
Normal CI keeps its usual compiler configuration.
