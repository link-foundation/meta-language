Repair the 18 all-target test compile errors and 55 Clippy diagnostic blocks
reported by CI run 38459623719. Tests use current diagnostic fields and public
imports, retain explicit fixture types, and reach the exact generated cache
decisions through a test-only provider. The shared schema has its intended root
export. Recorded generation styles and JavaScript selector factoring address
the binary, RDF, product, completion and typed-program lint errors. Adapter
visibility and continuation projection preserve their existing checks.

Surface source metadata uses transparent boxes to reduce the enum's size;
serialized field shapes and omission remain unchanged. Direct Rust callers
constructing these optional surface fields use boxed metadata. Existing
fixtures, refusal oracles, numeric boundaries and deadlines remain required.
Native compilation, lint and execution verification run in CI.
