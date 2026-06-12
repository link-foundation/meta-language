# Issue #47 research: data-exchange format grammars, links-ecosystem storage targets, and Rust API styles

Research date: **2026-06-10**. All crate versions, repository states, and dates below were verified
against the crates.io API, the npm registry, and the GitHub API on that date unless explicitly
marked *(unverified)*.

Implementation update (2026-06-12): PR
[#48](https://github.com/link-foundation/meta-language/pull/48) adopted the
seven ready tree-sitter data-format grammars and resolved the CSV/JSON5
binding gap with in-repo lossless parsers. CSV is validated with the Rust
`csv` crate; JSON5 is validated with `json5_nodes`. The compatibility table
below remains the research record explaining why the stale tree-sitter CSV and
JSON5 crates were not used directly.

Context: [issue #47](https://github.com/link-foundation/meta-language/issues/47) asks
`meta-language` to (1) add full lossless CST/AST support for popular data-exchange formats,
(2) add storage backends — [links notation](https://github.com/link-foundation/links-notation)
text, binary doublets via [doublets-rs](https://github.com/linksplatform/doublets-rs) /
[doublets-web](https://github.com/linksplatform/doublets-web), and a native Rust traits/types
representation — and (3) expose the same operations through several API styles (fluent chaining,
direct OOP methods, and [link-cli](https://github.com/link-foundation/link-cli)-style
match-and-substitute), so that [formal-ai](https://github.com/link-assistant/formal-ai) can use
the crate heavily. `meta-language` currently pins `tree-sitter = "0.25.8"` in `Cargo.toml`.

---

## Part A: data-exchange format grammars

### Compatibility note: the `tree-sitter-language` binding model

Modern grammar crates no longer take a hard dependency on a specific `tree-sitter` version.
They depend on the tiny ABI crate [`tree-sitter-language` `^0.1`](https://crates.io/crates/tree-sitter-language)
and list `tree-sitter` only as a **dev-dependency** (verified via the crates.io dependencies API
for every crate marked "ts-language ^0.1" below). Such crates link cleanly against the project's
`tree-sitter 0.25.8` even when their dev-dependency says `^0.24` or `^0.26`. Crates that still
declare `tree-sitter ~0.20` as a *normal* dependency (CSV, JSON5 below) are **not** usable with
0.25 without vendoring.

### Summary table

| Format | Crate | Version (crates.io) | License | Binding | Last publish | Status / verdict |
|---|---|---|---|---|---|---|
| JSON | [`tree-sitter-json`](https://crates.io/crates/tree-sitter-json) | 0.24.8 | MIT | ts-language ^0.1 (dev ts ^0.24) | 2024-11-11 | **Ready.** Official [tree-sitter org grammar](https://github.com/tree-sitter/tree-sitter-json); 2.7 M downloads (1.2 M recent). |
| YAML | [`tree-sitter-yaml`](https://crates.io/crates/tree-sitter-yaml) | 0.7.2 | MIT | ts-language ^0.1 (dev ts ^0.25.4) | 2025-10-07 | **Ready.** [tree-sitter-grammars org](https://github.com/tree-sitter-grammars/tree-sitter-yaml); 2.2 M downloads. Uses an external scanner; edge cases on exotic YAML 1.2 constructs exist *(unverified — based on the format's known difficulty, not a tested defect list)*. |
| TOML | [`tree-sitter-toml-ng`](https://crates.io/crates/tree-sitter-toml-ng) | 0.7.0 | MIT | ts-language ^0.1 (dev ts ^0.24) | 2024-12-03 | **Ready.** The maintained fork at [tree-sitter-grammars/tree-sitter-toml](https://github.com/tree-sitter-grammars/tree-sitter-toml); 842 k downloads. Avoid the original [`tree-sitter-toml`](https://crates.io/crates/tree-sitter-toml) 0.20.0 — abandoned (last publish 2022-01-05, pins ts ^0.20). |
| XML (+DTD) | [`tree-sitter-xml`](https://crates.io/crates/tree-sitter-xml) | 0.7.0 | MIT | ts-language ^0.1 (dev ts ^0.24) | 2024-11-13 | **Ready.** [tree-sitter-grammars org](https://github.com/tree-sitter-grammars/tree-sitter-xml); ships both XML and DTD languages; 282 k downloads. |
| CSV/TSV/PSV | [`tree-sitter-csv`](https://crates.io/crates/tree-sitter-csv) | 1.2.0 | MIT | **normal dep `tree-sitter ~0.20.10`** | 2024-01-24 | **Gap.** Incompatible with ts 0.25 as published. The repo moved to [tree-sitter-grammars/tree-sitter-csv](https://github.com/tree-sitter-grammars/tree-sitter-csv) (last push 2025-11-13) but its `Cargo.toml` on master *still* pins `~0.20.10`; no fixed release exists. Recommend vendoring the generated `parser.c` behind `tree-sitter-language`, or a hand-rolled lossless CSV lexer (RFC 4180 is trivial: records, fields, quotes, separators). |
| INI | [`tree-sitter-ini`](https://crates.io/crates/tree-sitter-ini) | 1.4.0 | Apache-2.0 | ts-language ^0.1 (dev ts ^0.25.9) | 2025-12-08 | **Ready.** By [justinmk](https://github.com/justinmk/tree-sitter-ini) (Neovim maintainer). Caveat: INI has no formal spec, so dialect coverage (e.g. `;` vs `#` comments) should be fixture-tested. |
| Protocol Buffers | [`tree-sitter-proto`](https://crates.io/crates/tree-sitter-proto) | 0.4.0 | MIT | ts-language ^0.1 (dev ts ^0.26) | 2025-12-24 | **Ready.** By [coder3101](https://github.com/coder3101/tree-sitter-proto); 172 k downloads. No `tree-sitter-protobuf` crate exists on crates.io (verified: 404). |
| GraphQL (SDL + queries) | [`tree-sitter-graphql`](https://crates.io/crates/tree-sitter-graphql) | 0.1.0 | MIT via `license-file` (crates.io shows "non-standard") | ts-language ^0.1 (dev ts ^0.25.3) | 2025-04-30 | **Usable, young.** Published by [joowani](https://github.com/joowani/tree-sitter-graphql) (repo: 7 commits, MIT, last push 2026-02-13, based on the older bkegley/dralletje grammars); 99 k downloads, 67 k recent. Modern bindings; adopt with fixture coverage for both schema and executable documents. |
| Markdown | [`tree-sitter-md`](https://crates.io/crates/tree-sitter-md) | 0.5.3 | MIT | ts-language ^0.1 (dev ts ^0.26.3) | 2026-02-26 | **Ready** (already supported in `meta-language` via its own pipeline — `src/parity.rs` lists Markdown). Two-grammar design (block + inline) from [tree-sitter-grammars/tree-sitter-markdown](https://github.com/tree-sitter-grammars/tree-sitter-markdown). Avoid the abandoned [`tree-sitter-markdown`](https://crates.io/crates/tree-sitter-markdown) 0.7.1 (ikatyang, last publish 2021-04-18). |
| JSON5 | [`tree-sitter-json5`](https://crates.io/crates/tree-sitter-json5) | 0.1.0 | MIT | **normal dep `tree-sitter ~0.20.0`** | 2025-09-20 | **Gap on crates.io.** The published crate is incompatible with ts 0.25. Upstream [Joakker/tree-sitter-json5](https://github.com/Joakker/tree-sitter-json5) (MIT, last push 2026-05-05) already targets `tree-sitter = "0.25"` on master — usable as a git dependency or vendored. (The crate metadata points at `github.com/tree-sitter/tree-sitter-json5`, which returns 404.) |

### Non-tree-sitter lossless alternatives (for cross-checking or fallback)

- TOML: [`toml_edit`](https://crates.io/crates/toml_edit) 0.25.12+spec-1.1.0 (2026-05-27) — the
  format-preserving parser used by Cargo itself; a strong round-trip oracle for TOML fixtures.
- JSON/JSONC: [`jsonc-parser`](https://crates.io/crates/jsonc-parser) 0.32.4 (2026-05-10) — CST
  mode preserves comments/whitespace.
- XML: [`quick-xml`](https://crates.io/crates/quick-xml) 0.40.1 (2026-05-15) — event-based,
  round-trip-capable, not a CST.
- CSV: [`csv`](https://crates.io/crates/csv) 1.4.0 (2025-10-17) — fast but lossy (no quote/spacing
  preservation); only useful for semantic cross-checks.

**Formats lacking a good tree-sitter crate today: CSV and JSON5** (both have grammars but stale
crates.io bindings pinned to tree-sitter 0.20). PR #48 handles them with
lossless in-repo parsers instead of waiting for compatible tree-sitter crates.
Everything else is adoptable as-is.

---

## Part B: links ecosystem storage targets

### links-notation (LiNo text format)

Repo: <https://github.com/link-foundation/links-notation> — Unlicense, last push 2026-06-03,
active. It is a polyglot monorepo; per the README, parsers ship for **six languages**:

| Language | Package |
|---|---|
| Rust | [`links-notation`](https://crates.io/crates/links-notation) **0.13.0** (published 2025-12-01) |
| JavaScript | npm [`links-notation`](https://www.npmjs.com/package/links-notation) |
| C# | NuGet [`Link.Foundation.Links.Notation`](https://www.nuget.org/packages/Link.Foundation.Links.Notation) |
| Python | PyPI [`links-notation`](https://pypi.org/project/links-notation/) |
| Go | `github.com/link-foundation/links-notation/go` |
| Java | Maven Central `io.github.link-foundation:links-notation` |

Rust entry point: `links_notation::parse_lino("papa (lovesMama: loves mama)")`. The notation
covers doublets (2-tuples), triplets, arbitrary N-tuple sequences, named links (`id: a b`), and
an indented (significant-whitespace) syntax equivalent to the inline parenthesized form. This is
the natural **text storage backend** for `meta-language`: a CST serialized as nested links.

### doublets-rs (binary doublets store)

Repo: <https://github.com/linksplatform/doublets-rs> — Unlicense, last push 2026-05-29.
Crate: [`doublets`](https://crates.io/crates/doublets) **0.4.0** (published 2026-05-29; 43 k
downloads). **Actively maintained** and — important change from its early history — now builds on
**stable Rust** (`rust-version = "1.85"`, stable toolchain in `rust-toolchain.toml`). The README's
install snippet still says `doublets = "0.3.0"` (stale doc; 0.4.0 is latest, and formal-ai also
pins 0.3.0). Workspace members: `doublets` (core), `doublets-decorators`, `doublets-ffi`.

A link is `(index, source, target)` where every element is itself a link reference; generic over
any unsigned integer (`u8`…`usize`). Storage backends: `mem::FileMapped` (memory-mapped file
persistence — this is the **binary doublets** format issue #47 wants) and `mem::Global` (heap),
with `unit::Store` (combined region) and `split::Store` (separate data/index trees) layouts, via
[`platform-mem`](https://crates.io/crates/platform-mem) 0.3.0 and
[`platform-data`](https://crates.io/crates/platform-data) 2.0.0 (both republished 2026-04).

The Rust analog of C#'s `ILinks` is a **two-layer trait stack** (verified from
`doublets/src/data/traits.rs`):

```rust
pub trait Links<T: LinkReference>: Send + Sync {
    fn constants(&self) -> &LinksConstants<T>;          // any/null constants
    fn count_links(&self, query: &[T]) -> T;
    fn create_links(&mut self, query: &[T], handler: WriteHandler<'_, T>) -> Result<Flow, Error<T>>;
    fn each_links(&self, query: &[T], handler: ReadHandler<'_, T>) -> Flow;
    fn update_links(&mut self, query: &[T], change: &[T], handler: WriteHandler<'_, T>) -> Result<Flow, Error<T>>;
    fn delete_links(&mut self, query: &[T], handler: WriteHandler<'_, T>) -> Result<Flow, Error<T>>;
}
```

- `Links<T>` — five raw operations over slice queries (`[index, source, target]` patterns with an
  `any` wildcard constant) plus callback handlers returning `Flow::Continue/Break`. Read ops take
  `&self`, write ops `&mut self`.
- `Doublets<T>: Links<T>` — ergonomic defaults built entirely on the raw layer: `create()`,
  `create_point()`, `create_link(s, t)`, `count()`, `count_by(query)`, `get_link(i)`,
  `search(s, t)`, `each(..)`/`each_by(..)`, `update(..)`, `delete(..)`, `delete_all()`.
- `DoubletsExt` — iterator (`iter()`, `each_iter(query)`) and optional rayon parallel extensions.

**Verdict: viable as the binary backend.** Stable-Rust, active in 2026, slice-query model maps
1:1 onto link-cli's match patterns, and `Send + Sync` is required of implementors.

### doublets-web (WASM doublets)

Repo: <https://github.com/linksplatform/doublets-web> — Unlicense, TypeScript + Rust/`wasm-pack`,
last push 2026-05-23. npm package [`doublets-web`](https://www.npmjs.com/package/doublets-web)
**0.1.3** (last modified 2026-05-08): "WebAssembly bindings for the LinksPlatform Doublets
associative storage library." JS API mirrors the Rust one in OOP form: `new LinksConstants()`,
`new UnitedLinks(constants)`, `links.create()/update()/count(new Link(any, s, t))`. Live
playground: <https://linksplatform.github.io/doublets-web/>. Relevance: if `meta-language`
targets the `doublets` crate, a browser build path already exists; formal-ai names `DoubletsWeb`
as one of its three `LinkStoreBackend` variants.

### link-cli (match-and-substitute semantics)

Repo: <https://github.com/link-foundation/link-cli> — Unlicense, last push 2026-05-20. Dual
implementation: C# (NuGet `clink` + `Foundation.Data.Doublets.Cli`) and Rust
([`link-cli`](https://crates.io/crates/link-cli) **0.2.7**, published 2026-05-20, docs at
<https://docs.rs/link-cli>), plus a WASM browser workbench built on `doublets-web`.

Every CRUD operation is **one substitution operation** over LiNo patterns (explicitly framed as a
[Markov algorithm](https://en.wikipedia.org/wiki/Markov_algorithm), Turing-complete):
`(match patterns) (substitution patterns)`.

| Operation | Query | Reading |
|---|---|---|
| Create | `() ((1 1))` | nothing → link with source 1, target 1 (index assigned by store) |
| Read | `((1: 1 1)) ((1: 1 1))` | identity substitution = no-op; with `--changes` it echoes matches |
| Update | `((1: 1 1)) ((1: 1 2))` | same index on both sides → in-place change of target |
| Delete | `((1 1)) ()` | match → nothing |

Variables, named references, LiNo import/export, structure formatting, and (C#-only for now)
persistent transformation triggers layer on top. This is the exact semantics issue #47's third
API style should reproduce as a library API.

### formal-ai (the primary consumer)

Repo: <https://github.com/link-assistant/formal-ai> — Unlicense, **very active** (pushed
2026-06-10, the research date). "100% symbolic, logical, and data driven formal language
processor" exposing OpenAI-shaped chat/HTTP interfaces without neural inference.

How it consumes a links engine (verified from `Cargo.toml` and `src/link_store.rs`):

- Dependencies: `links-notation = "0.13.0"`, `lino-objects-codec = "0.2.1"`,
  `lino-arguments = "0.3"`, `link-calculator = "0.17.2"` (crates.io latest 0.18.0), and
  `doublets = "0.3.0"` + `platform-mem` **optional**, enabled by the default feature
  `doublets-native`.
- `src/link_store.rs` defines a swappable storage boundary: `trait LinkStore` with
  `backend()`, `append_memory_event(..)`, `import_memory_links_notation(..)`,
  `export_memory_links_notation()`; `enum LinkStoreBackend { LinoProjection, DoubletsRs,
  DoubletsWeb }`. `.lino` text stays the deterministic, human-reviewable import/export
  projection; doublets is the native physical store; browsers mirror via IndexedDB
  (`src/web/memory.js`). Its `DoubletLink { index, from, to }` is a string-keyed doublet
  projection.
- Data it would feed through `meta-language`: `data/seed/*.lino` (30+ files — `concepts.lino`
  46 KB, `hello-world-programs.lino` 46 KB, `facts.lino` 22 KB, `intent-routing.lino`,
  per-domain `meanings-*.lino`, …), `data/benchmarks/industry-suite.lino` and
  `coding-modification-suite.lino`, and `data/source-index.lino`. All use the indented untyped
  format from `lino-objects-codec`, capped at 1500 lines per file by a check script.

Implication for `meta-language`: matching formal-ai's existing stack means consuming
`links-notation` 0.13 for text, `doublets` 0.3/0.4 behind a feature gate, and emitting
`lino-objects-codec`-compatible indented output.

### lino-objects-codec

Repo: <https://github.com/link-foundation/lino-objects-codec> — Unlicense, last push 2026-05-10.
Dual-published: npm [`lino-objects-codec`](https://www.npmjs.com/package/lino-objects-codec)
**0.4.0** (2026-05-10) and crates.io [`lino-objects-codec`](https://crates.io/crates/lino-objects-codec)
**0.2.1** (2026-05-03; note Rust lags npm). Encodes/decodes structured objects to/from LiNo;
formal-ai calls `lino_objects_codec::format::parse_indented`. For `meta-language`, this is the
reference for *typed object* projections onto LiNo, distinct from raw link dumps.

### Comparison: RDF triple stores (Oxigraph) vs doublets

[Oxigraph](https://github.com/oxigraph/oxigraph) — crates.io [`oxigraph`](https://crates.io/crates/oxigraph)
**0.5.8** (2026-04-28), MIT OR Apache-2.0 — is the mature Rust RDF/SPARQL store (214 k downloads,
RocksDB-backed persistence, SPARQL 1.1).

Key structural differences:

| | RDF triples (Oxigraph) | Doublets (doublets-rs) |
|---|---|---|
| Unit | `(subject, predicate, object)` — fixed arity 3 | `index: (source, target)` — arity 2 plus identity |
| Element types | heterogeneous atoms: IRIs, blank nodes, literals | **homogeneous**: every element is itself a link reference; values are links too |
| Statement identity | a triple has no first-class id (reification/RDF-star needed to talk about a triple) | every link has an index and can be the source/target of other links — self-describing by construction |
| N-ary data | predicates fixed at 3; n-ary needs reification patterns | any arity via nesting doublets (a triple ≈ 2 doublets, `(a, (b, c))`) |
| Query | SPARQL (standard, rich, heavyweight) | `[index, source, target]` patterns with `any` wildcard; link-cli adds Markov-style substitution |

Oxigraph is the benchmark for maturity and standards interop, but its fixed-arity,
typed-atom model cannot represent a lossless CST as uniformly as doublets, where the "everything
is a link" invariant matches `meta-language`'s self-describing links-network design. An RDF
export could later be a *projection*, not a core backend.

---

## Part C: API style patterns in Rust

How leading crates expose the same operations through multiple styles — and what `meta-language`
should copy.

### 1. Core trait + ergonomic extension trait (the pattern doublets-rs itself uses)

`doublets` layers `Links<T>` (5 raw slice-query methods) → `Doublets<T>` (default-implemented
ergonomic methods) → `DoubletsExt` (iterator adapters). The std-library analog is
`Iterator`: one required method (`next`), ~75 default fluent adapters, and `Itertools` as an
external extension trait. **Recommendation:** `meta-language` should define one raw trait per
backend capability and put *all three API styles* in default-implemented or extension layers, so
every style provably hits the same five primitive operations.

### 2. syn / quote — direct typed AST vs macro DSL ([syn](https://crates.io/crates/syn) 2.0.117, [quote](https://crates.io/crates/quote) 1.0.45)

The same token stream is consumed via a fully typed AST (`syn::ItemFn`, direct field access,
OOP-style) and produced via the `quote!` template DSL (declarative, pattern-shaped). syn also
models read-only vs mutating vs consuming traversal as three parallel trait families:
`visit::Visit<'ast>` (`&self`-style), `visit_mut::VisitMut` (`&mut`), `fold::Fold` (owned) — a
direct template for `meta-language`'s read-only vs mutable CST access.

### 3. Database stack — three crates, three styles over the same SQL operations

- [`sqlx`](https://crates.io/crates/sqlx) 0.9.0: raw SQL strings + compile-time-checked
  `query!` macros — the "match pattern as text" style (closest analog to link-cli queries).
- [`diesel`](https://crates.io/crates/diesel) 2.3.10: a typed, composable query-builder DSL
  (`users.filter(name.eq("x")).load(..)`) — fluent chaining with compile-time schema.
- [`sea-orm`](https://crates.io/crates/sea-orm) 1.1.20 layered on
  [`sea-query`](https://crates.io/crates/sea-query) 1.0.1: ActiveRecord-style OOP entity methods
  (`Entity::find_by_id(..).one(db)`) generating into a runtime fluent builder, with a raw-SQL
  escape hatch.

The lesson: all three coexist because they **lower into one executor layer**. For issue #47:
fluent builder, direct methods, and LiNo match-substitute queries should all lower into the
`Links<T>`-like primitive set.

### 4. Builder style specifics

[`derive_builder`](https://crates.io/crates/derive_builder) 0.20.2 (runtime-checked) and
[`typed-builder`](https://crates.io/crates/typed-builder) 0.23.2 (compile-time typestate: missing
required field = type error) show the two fluent-construction idioms. Typestate builders suit
`meta-language`'s "construct a link/CST node" path where well-formedness can be static.

### 5. Read-only vs mutable access modeling

- **`&self` / `&mut self` method pairs** are the canonical split: `get`/`get_mut`,
  `iter`/`iter_mut` (std collections), syn's `Visit`/`VisitMut`. Doublets follows it
  (`each_links(&self, ..)` vs `update_links(&mut self, ..)`).
- **Persistent (immutable) structures:** [`rpds`](https://crates.io/crates/rpds) 1.2.1
  (actively maintained, published 2026-05-15) offers persistent vectors/maps with structural
  sharing; [`im`](https://crates.io/crates/im) 15.1.0 is more popular historically but dormant
  (last publish 2022-04-29). If `meta-language` wants cheap CST snapshots/undo, prefer `rpds`.
- **Freeze pattern:** build with `&mut` (or a builder), then publish as `Arc<Frozen>` exposing
  only `&self` methods — how `tree-sitter`'s own `Tree` works (parse mutates a parser; the
  resulting tree is cheaply cloneable and read-only, with edits producing new trees).

### Recommended structure for issue #47

1. **Primitive trait** (`LinkStore`-like, mirroring `Links<T>`): `constants`, `count`, `create`,
   `each`, `update`, `delete` over `[index, source, target]` patterns; `&self` for reads,
   `&mut self` for writes. Implementations: in-memory native Rust types, LiNo text
   (via `links-notation` 0.13 + `lino-objects-codec`), binary doublets (feature-gated
   `doublets` 0.4 with `mem::FileMapped`).
2. **Fluent layer** as a default-implemented extension trait (chaining, iterator adapters,
   typestate builders for node construction).
3. **Query layer** implementing link-cli's `(match) (substitution)` LiNo semantics as a library
   API, parsed with `links-notation`, lowered onto the primitive trait — giving formal-ai (whose
   `LinkStoreBackend` enum already names `LinoProjection | DoubletsRs | DoubletsWeb`) a drop-in
   engine.
