#!/usr/bin/env sh
# Copies the grammar IR (rust/src/grammar/mod.rs, from `use std::...` up to the
# builders) into the stub crate, lints rust/src/grammar/merge.rs against it, and
# runs rust/tests/unit/grammar_merge.rs on the grammars the JavaScript pest
# importer produces for the fixture (export-js-grammars.mjs, src/facade.rs).
set -eu
here=$(cd "$(dirname "$0")" && pwd)
root=$(cd "$here/../.." && pwd)
mod="$root/rust/src/grammar/mod.rs"
start=$(grep -n '^use std::collections::BTreeSet;' "$mod" | cut -d: -f1)
end=$(grep -n '^/// Fluent builder for order-preserving grammars.' "$mod" | cut -d: -f1)
{
  echo '//! Generated stub: grammar IR copy plus rust/src/grammar/merge.rs.'
  echo '#![allow(dead_code, clippy::all, clippy::pedantic, clippy::nursery, missing_docs)]'
  echo 'pub mod grammar {'
  echo '#![allow(dead_code, clippy::all, clippy::pedantic, clippy::nursery, missing_docs)]'
  sed -n "${start},$((end - 1))p" "$mod"
  sed -n '/^fn write_joined(/,/^}/p' "$mod"
  echo '#[derive(Clone, Copy, Debug, Default)] pub struct GrammarBuilder;'
  echo 'impl GrammarBuilder { pub const fn new() -> Self { Self } }'
  echo '#[derive(Clone, Copy, Debug, Default)] pub struct ExprBuilder;'
  echo "#[path = \"$root/rust/src/grammar/merge.rs\"]"
  echo '#[warn(clippy::all, clippy::pedantic, clippy::nursery)]'
  echo '#[allow(clippy::module_name_repetitions, clippy::too_many_lines, clippy::missing_errors_doc, clippy::missing_panics_doc)]'
  echo '#[deny(missing_docs)]'
  echo 'pub mod merge;'
  echo 'pub use merge::*;'
  echo '}'
  echo 'mod facade;'
  echo 'pub use facade::*;'
  echo 'pub use grammar::*;'
} > "$here/src/lib.rs"
mkdir -p "$here/tests/probe"
sed 's|\.join("\.\.")|.join("../..")|' "$root/rust/tests/unit/grammar_merge.rs" > "$here/tests/probe/grammar_merge_copy.rs"
cd "$here"
CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-/tmp/grammar-merge-stub-target}" cargo clippy --quiet -- -D warnings
node "$here/export-js-grammars.mjs" /tmp/grammar-merge-js-grammars.json
CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-/tmp/grammar-merge-stub-target}" cargo test --quiet -- --nocapture "$@"
