# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- changelog-insert-here -->



















































## [0.51.0] - 2026-06-28

### Added
- Changeset-style fragment format with frontmatter for specifying version bump type
- New `get-bump-type.mjs` script to automatically determine version bump from fragments
- Automatic version bumping on merge to main based on changelog fragments
- Detailed documentation for the changelog fragment system in `changelog.d/README.md`

### Changed
- Updated `collect-changelog.mjs` to strip frontmatter when collecting fragments
- Updated `version-and-commit.mjs` to handle frontmatter in fragments
- Enhanced release workflow to automatically determine bump type from changesets

### Changed
- Add `detect-changes` job with cross-platform `detect-code-changes.mjs` script
- Make lint job independent of changelog check (runs based on file changes only)
- Allow docs-only PRs without changelog fragment requirement
- Handle changelog check 'skipped' state in dependent jobs
- Exclude `changelog.d/`, `docs/`, `experiments/`, `examples/` folders and markdown files from code changes detection

### Fixed
- Fixed README.md to correctly reference Node.js scripts (`.mjs`) instead of Python scripts (`.py`)
- Updated project structure in README.md to match actual script files in `scripts/` directory
- Fixed example code in README.md that had invalid Rust with two `main` functions

### Added

- Added crates.io publishing support to CI/CD workflow
- Added `release_mode` input with "instant" and "changelog-pr" options for manual releases
- Added `--tag-prefix` and `--crates-io-url` options to create-github-release.mjs script
- Added comprehensive case study documentation for Issue #11 in docs/case-studies/issue-11/

### Changed

- Changed changelog fragment check from warning to error (exit 1) to enforce changelog requirements
- Updated job conditions with `always() && !cancelled()` to fix workflow_dispatch job skipping issue
- Renamed manual-release job to "Instant Release" for clarity

### Fixed

- Fixed deprecated `::set-output` GitHub Actions command in version-and-commit.mjs
- Fixed workflow_dispatch triggering issues where lint/build/release jobs were incorrectly skipped

### Fixed

- Fixed changelog fragment check to validate that a fragment is **added in the PR diff** rather than just checking if any fragments exist in the directory. This prevents the check from incorrectly passing when there are leftover fragments from previous PRs that haven't been released yet.

### Changed

- Converted shell scripts in `release.yml` to cross-platform `.mjs` scripts for improved portability and performance:
  - `check-changelog-fragment.mjs` - validates changelog fragment is added in PR diff
  - `git-config.mjs` - configures git user for CI/CD
  - `check-release-needed.mjs` - checks if release is needed
  - `publish-crate.mjs` - publishes package to crates.io
  - `create-changelog-fragment.mjs` - creates changelog fragments for manual releases
  - `get-version.mjs` - gets current version from Cargo.toml

### Added

- Added `check-version-modification.mjs` script to detect manual version changes in Cargo.toml
- Added `version-check` job to CI/CD workflow that runs on pull requests
- Added skip logic for automated release branches (changelog-manual-release-*, changeset-release/*, release/*, automated-release/*)

### Changed

- Version modifications in Cargo.toml are now blocked in pull requests to enforce automated release pipeline

### Added

- Added support for `CARGO_REGISTRY_TOKEN` as alternative to `CARGO_TOKEN` for crates.io publishing
- Added case study documentation for Issue #17 (yargs reserved word and dual token support)

### Changed

- Updated workflow to use fallback logic: `${{ secrets.CARGO_REGISTRY_TOKEN || secrets.CARGO_TOKEN }}`
- Improved publish-crate.mjs to check both `CARGO_REGISTRY_TOKEN` and `CARGO_TOKEN` environment variables
- Added warning message when neither token is set

### Added
- New `scripts/rust-paths.mjs` utility for automatic Rust package root detection
- Support for both single-language and multi-language repository structures in all CI/CD scripts
- Configuration options via `--rust-root` CLI argument and `RUST_ROOT` environment variable
- Comprehensive case study documentation in `docs/case-studies/issue-19/`

### Changed
- Updated all release scripts to use the new path detection utility:
  - `scripts/bump-version.mjs`
  - `scripts/check-release-needed.mjs`
  - `scripts/collect-changelog.mjs`
  - `scripts/get-bump-type.mjs`
  - `scripts/get-version.mjs`
  - `scripts/publish-crate.mjs`
  - `scripts/version-and-commit.mjs`

### Changed

- **check-release-needed.mjs**: Now checks crates.io API directly instead of git tags to determine if a version is already released. This prevents false positives where git tags exist but the package was never actually published to crates.io.

### Added

- **CI/CD Troubleshooting Guide**: New documentation at `docs/ci-cd/troubleshooting.md` covering common issues like skipped jobs, false positive version checks, publishing failures, and secret configuration.

- **Enhanced Error Handling in publish-crate.mjs**: Added specific detection and helpful error messages for authentication failures, including guidance on secret configuration and workflow setup.

- **Case Study Documentation**: Added comprehensive case study at `docs/case-studies/issue-21/` analyzing CI/CD failures from browser-commander repository (issues #27, #29, #31, #33) with timeline, root causes, and lessons learned.

### Fixed

- **Prevent False Positive Version Checks**: The release workflow now correctly identifies unpublished versions by checking crates.io instead of relying on git tags, which can exist without the package being published.

### Changed

- Translated all CI/CD scripts from JavaScript (.mjs) to Rust (.rs) using rust-script
- Scripts now use native Rust with rust-script for execution in shell
- Removed Node.js dependency from CI/CD pipeline
- Updated GitHub Actions workflow to use rust-script instead of node
- Updated README and CONTRIBUTING documentation with new script references

### Added

- Cache `restore-keys` for partial cache hits across all workflow jobs
- Explicit `token` parameter in checkout for release jobs
- Code coverage job with `cargo-llvm-cov` and Codecov integration
- Codecov badge in README.md
- Pre-release version support (e.g., `0.1.0-beta.1`) in version parsing
- `--release-label` parameter for multi-language release disambiguation
- `ensure_version_exceeds_published()` logic to prevent publishing duplicate versions
- `get_max_published_version()` to query highest non-yanked version from crates.io
- `max_published_version` output from check-release-needed for downstream use
- Version fallback logic in auto-release Create GitHub Release step

### Changed

- Updated `actions/checkout` from v4 to v6
- Updated `actions/cache` from v4 to v5
- Updated `peter-evans/create-pull-request` from v7 to v8
- Made `publish-crate.rs` fail (exit 1) when version already exists on crates.io
- Improved `create-github-release.rs` to check combined stdout+stderr and detect "Validation Failed"

### Fixed

- Fix publish steps overriding workflow-level CARGO_TOKEN fallback, breaking CARGO_REGISTRY_TOKEN-only configurations (#32)
- Fix non-fast-forward push failures in multi-workflow repos by adding fetch/rebase and push retry logic (#31)
- Add mono-repo path support to check-changelog-fragment.rs, check-version-modification.rs, and create-changelog-fragment.rs
- Add `!cancelled()` guard to test job condition to respect workflow cancellation

### Fixed

- Fixed `version-and-commit.rs` to check crates.io instead of git tags for determining if a version is already released
- This prevents the release pipeline from getting stuck when git tags exist without corresponding crates.io publication

### Added

- Added `--tag-prefix` support to `version-and-commit.rs` for multi-language repository compatibility
- Added crates.io and docs.rs badges to README.md
- Added automatic crates.io and docs.rs badge injection in GitHub release notes
- Added documentation deployment job to CI/CD pipeline (deploys to GitHub Pages after release)
- Added case study documentation for issue #25

### Fixed
- Fixed unsupported look-ahead regex in `create-github-release.rs` that caused a panic when parsing CHANGELOG.md. Replaced with a two-step approach using only features supported by Rust's `regex` crate.

### Changed
- Restructured example application as a simple CLI sum calculator using `lino-arguments`
- Renamed default package to `example-sum-package-name` with Unlicense license
- Reorganized test structure: `tests/unit/sum.rs`, `tests/integration/sum.rs`, `tests/unit/ci-cd/`
- Converted experiment scripts into proper unit tests in `tests/unit/ci-cd/changelog_parsing.rs`
- Added CI/CD skip logic for template default package name `example-sum-package-name`
- Updated README.md badges and documentation

### Fixed

- Change detection script now uses per-commit diff instead of full PR diff, so commits touching only non-code files correctly skip CI jobs even when earlier commits in the same PR changed code files

### Fixed
- Make release scripts resolve the publishable crate manifest when the repository root uses a Cargo workspace manifest.

### Fixed
- Decoupled GitHub Pages documentation deployment from package release publication and fixed release-script warning failures under `RUSTFLAGS=-Dwarnings`.

### Changed
- Added explicit GitHub Actions job timeouts and documented Rust test timeout guidance.

### Fixed
- Added a non-blocking warning threshold to the Rust file-size check so near-limit files are surfaced before concurrent PR merges can exceed the hard limit.

### Fixed
- Made `create-github-release.rs` build GitHub release titles as `[Language] X.Y.Z` instead of reusing the tag prefix.

### Added
- Added optional Docker Hub image publishing tied to Rust crate releases, including crates.io visibility waiting, version/latest image tags, and Docker Hub badges in GitHub release notes.

### Changed
- Release completeness checks now self-heal when crates.io exists but configured Docker Hub or GitHub release artifacts are missing.

### Fixed
- Switched documentation deployment to the official GitHub Pages artifact workflow so repositories using GitHub Actions as their Pages source do not get false-positive branch-push deploys.

### Added
- Documented the one-time `Settings → Pages → Source = GitHub Actions` prerequisite for the `deploy-docs` job in `README.md` and as a comment above the `deploy-docs` job in `release.yml`, so downstream template users hit a documented setup step instead of a `Get Pages site failed` error on the first deploy.

### Added
- Tracking case study at `docs/case-studies/issue-52/` registering the `browser-commander` + Playwright preview-regeneration pattern from [`konard/vk-bot-desktop#52`](https://github.com/konard/vk-bot-desktop/pull/52), with an activation checklist for when an example-app surface lands in this template. Documentation only — no workflow, script, or runtime code changes. Primary upstream tracking issue: [`link-foundation/js-ai-driven-development-pipeline-template#62`](https://github.com/link-foundation/js-ai-driven-development-pipeline-template/issues/62).

### Fixed
- Release automation now keeps the workspace package entry in `Cargo.lock` synchronized when `scripts/version-and-commit.rs` bumps `Cargo.toml`, preventing stale lock-file version diffs in later pull requests.

### Fixed
- `scripts/publish-crate.rs` now treats crates.io HTTP 429 throttle responses ("You have published too many versions of this crate in the last 24 hours") as a deferred `publish_result=rate_limited` outcome (it writes the output, prints an explanatory banner and exits successfully) instead of a hard CI failure reported as a generic `failed` ("Failed to publish for unknown reason"). Authentication, already-published and unknown failures still exit non-zero. Failed-publish classification is consolidated through a single `classify_failure` function and `FailureKind` enum (with an `is_deferred` predicate), covered by unit tests runnable via `rust-script --test scripts/publish-crate.rs`.
- The release workflow (`.github/workflows/release.yml`) now gates crate-availability waiting, Docker Hub publishing and GitHub release creation on either an already-published crate or `publish_result=success`, so a deferred (rate-limited) crate upload no longer produces partial downstream release artifacts and the same version is retried automatically on the next push to `main`.

### Fixed
- Fixed reversed `cancel-in-progress` concurrency condition in `release.yml` that cancelled in-flight releases on `main` and never superseded older PR runs. The condition now uses `!=` so `main` releases run to completion while newer PR pushes cancel stale runs.

### Added
- Added a `scripts/check-crate-size.rs` guard that builds the `.crate` archive and fails the release before publishing when it exceeds the crates.io 10 MiB upload limit. The check runs in the build job and before publishing in both the auto-release and manual-release jobs.

### Changed
- Added a narrow `include` allowlist to `Cargo.toml` so docs, case studies, generated CI artifacts, changelog fragments, scripts, and experiments no longer inflate the published release archive.

### Fixed
- Prevented GitHub release creation from treating generic API validation failures as existing releases, and capped oversized release notes with a link to the full tagged changelog.

### Added
- Added the initial `meta-language` links-network core with self-description roots,
  verification, trivia attachment configuration, and a minimal CLI.
- Added the default lossless `parse` entry point, projection views, and a
  tested parity roadmap for competitor features, grammar embedding, and language
  coverage targets.
- Added exact reconstruction, mixed-region detection, query matching,
  substitution rules, concept reconstruction, object identity helpers,
  many-valued truth values, and executable parity fixtures for every tracked
  competitor target.
- Added executable language fixtures and tests for every requested Markdown,
  HTML, top-ten programming-language, and top-ten natural-language target.

### Fixed
- Added a CI guard that rejects Rust test modules and test attributes under `src/`, keeping tests in the `tests/` tree.

## Minor Changes

- Add immutable and mutable network snapshots with provenance and forward
  version commits for roadmap snapshot/versioning coverage.

### Added
- Added `txt` as a first-class markup/container target with a UTF-8 lossless
  fixture and content-sniffing fallback regions.

### Fixed
- Reconciled the natural-language target ordering with the Ethnologue 2025
  total-speaker order cited by the parity roadmap.

### Added
- Added a grammar-backed parser front end for Python, C, Java, C++, C#, JavaScript, and R using official tree-sitter grammar crates.

### Added
- Parse supported mixed-mode embedded regions into the host links network with grammar-backed syntax links.

### Added
- Added natural-language segmentation, identification, normalization, and bidi annotation links over lossless text parses.

### Added
- Added grammar-backed Visual Basic parsing with byte-exact reconstruction and recovery flag coverage.

bump: minor

- Add a grammar-backed `sql-ansi` SQL-family dialect fixture using
  `tree-sitter-sequel`.

### Added
- Added grammar-backed Delphi/Object Pascal parsing through `tree-sitter-pascal`.

### Added
- Enriched link queries with S-expression structural matching, captures, by-type construction, host predicate hooks, and link-cli-style variable substitution bindings.

### Added
- Added persistent snapshot structural sharing with interned metadata text storage.

### Added
- Added a query-transform surface for selecting captured links and replacing
  their source text while preserving unchanged bytes.

### Added
- Materialized self-description roots as controlled links with complete root-definition closure and round-trippable `describe` output.

### Added
- Added common concept ontology seeding from meta-expression's semantic lexicon, with shared concept links, syntax mappings, and structural programming-language concepts.

### Added
- Added semantic cross-language reconstruction for the Hawaii statehood fixture,
  including English/Russian naturalization and configurable formalization levels.

### Added
- Expanded `PARITY_FIXTURES` with multiple provenance-tracked fixtures for tree-sitter, LibCST, Recast, jscodeshift, Rowan, cstree, and Roslyn, including executable recovery and query/transform expectations.

### Added

- Expanded `PARITY_FIXTURES` with upstream-provenanced internal ecosystem corpora for links-notation, link-cli, lino-objects-codec, relative-meta-logic, formal-ai, and meta-expression.

### Added

- Added structural LiNo parsing for links-notation doublets, triplets, named links, indented IDs, and self-references while preserving byte-exact reconstruction.

### Added
- Added fixed-point probabilistic truth values for relative-meta-logic-style
  semantic confidence evaluation.

### Added
- Added grammar-backed Go parsing via the official `tree-sitter-go` grammar
  (accepts `Go`, `go`, and `golang`) so `LinkNetwork::parse` emits real
  `LinkType::Syntax` concrete-syntax links with byte-exact reconstruction.

### Added
- `AccessMode { Mutable, ReadOnly }` setting on `ParseConfiguration`
  (`with_access_mode` / `access_mode`), defaulting to `Mutable` so existing
  callers are unaffected.
- `LinkNetwork::freeze` / `as_read_only` yielding a `ReadOnlyNetwork` view that
  exposes only `&self` operations (query, project, reconstruct, verify,
  serialize); mutators are unreachable at compile time because the view never
  hands out `&mut LinkNetwork`.
- `LinkNetwork::parse_engine`, returning an `EngineNetwork` handle that honours
  the configured access mode: read-only parsing returns the frozen form and
  `EngineNetwork::as_mutable` rejects mutation with a `ReadOnlyViolation`
  diagnostic.
- Snapshot interop: `NetworkSnapshot::as_read_only` / `from_read_only` reuse the
  snapshot's `Arc<LinkNetwork>`, so the frozen form composes with snapshot
  versioning instead of duplicating it.

### Added
- Added grammar-backed Ruby parsing through `tree-sitter-ruby`, so
  `LinkNetwork::parse(source, "Ruby", ...)` (and the `rb` alias) now emits real
  `LinkType::Syntax` links instead of falling back to lossless plain text.

bump: minor

- Add a grammar-backed `TypeScript` front end using `tree-sitter-typescript`,
  wiring the `typescript`/`ts` labels to `LANGUAGE_TYPESCRIPT` and the `tsx`
  label to `LANGUAGE_TSX` so `LinkNetwork::parse` emits real `LinkType::Syntax`
  links for TypeScript and TSX sources.

### Added
- Added grammar-backed parsing for seven data-exchange / interchange formats
  through `src/tree_sitter_adapter.rs`: JSON (`tree-sitter-json`), YAML
  (`tree-sitter-yaml`, accepts `yaml`/`yml`), TOML (`tree-sitter-toml-ng`), XML
  and DTD (`tree-sitter-xml`), INI (`tree-sitter-ini`), Protocol Buffers
  (`tree-sitter-proto`, accepts `protobuf`/`proto`/`Protocol Buffers`), and
  GraphQL (`tree-sitter-graphql`, accepts `graphql`/`gql`). Each parses into
  real `LinkType::Syntax` concrete-syntax links and reconstructs byte-for-byte.
- Added a `DATA_FORMAT_TARGETS` registry in `src/parity.rs` (with the new
  `LanguageFamily::DataFormat`) gated by parity tests, mirroring
  `MARKUP_LANGUAGE_TARGETS`.
- Added per-format UTF-8 `LANGUAGE_FIXTURES` round-trip entries plus
  recovery-case and mixed-region tests (a `json` fence embedded in Markdown
  parses into the host links network).
- Added lossless CSV and JSON5 parsers for the two formats whose published
  tree-sitter crates still pin the incompatible `tree-sitter ~0.20` runtime.
  CSV is validated with the Rust `csv` crate and JSON5 is validated with
  `json5_nodes`; both emit structured syntax links and reconstruct
  byte-for-byte.

### Documentation
- Documented the nine wired data-format parsers (parser, version, license, root
  node) in `docs/parity-roadmap.md`, including the tree-sitter compatibility
  rationale for the in-repo CSV and JSON5 parsers.

### Added
- `LinkNetwork::to_lino` serializes an entire links network to canonical
  links-notation text, keyed by each link's numeric id (doublets-style id
  discipline), covering references, names, types, terms, definitions,
  languages, source spans, parse flags, and term registration.
- `LinkNetwork::from_lino` reconstructs the exact network from that text,
  forming a lossless round-trip (`from_lino(to_lino(n))` is isomorphic to `n`).
- `LinoSerializationError` reports parse and schema failures from `from_lino`.
- A round-trip property test over every language fixture plus synthetic
  networks, and a test that `to_lino` output is accepted by the
  `links-notation` 0.13 crate parser.
- `ParityCapability::LinoSerialization` and an output-side serialization
  parity fixture for the `links-notation` target.

### Changed
- Added the `links-notation` 0.13 crate as a dependency so serialized output
  aligns with the wider links-notation ecosystem.

### Added
- `ParserRegistry` maps language keys to `Arc<dyn LanguageParser>` with the
  built-in parser as a fallback. User registrations shadow the built-in
  dispatch for the same (case-insensitive) key; unregistered keys still route
  through the built-in set.
- `LinkNetwork::parse_with_registry` dispatches a parse through a registry.
- `examples/custom_parser_registry.rs` documents registering a custom parser.

### Added
- Added grammar-backed parsing for five popular programming languages
  immediately below the TIOBE top ten through `src/tree_sitter_adapter.rs`: PHP
  (`tree-sitter-php`, via its `LANGUAGE_PHP` symbol), Swift
  (`tree-sitter-swift`), Kotlin (`tree-sitter-kotlin-ng`, accepts `kotlin`/`kt`),
  Scala (`tree-sitter-scala`), and Lua (`tree-sitter-lua`). Each parses into real
  `LinkType::Syntax` concrete-syntax links and reconstructs byte-for-byte.
- Added a `SECOND_TIER_PROGRAMMING_LANGUAGE_TARGETS` registry in `src/parity.rs`
  gated by parity tests, mirroring `DATA_FORMAT_TARGETS`.
- Added per-language UTF-8 `LANGUAGE_FIXTURES` round-trip entries plus
  case-insensitive alias coverage and a per-language recovery fixture whose
  malformed source still reconstructs while exposing error/missing diagnostics.

### Documentation
- Documented the wired grammars (crate, version, license, root node) in
  `docs/parity-roadmap.md` and kept the temporary Perl follow-up tracked
  explicitly until `ts-parser-perl` was adopted.

### Added
- Add a Rust `ToLinks`/`FromLinks` codec with queryable type-shape links and shared/circular object graph round-trips through LiNo serialization.

### Added

- Added the `LinkStore` storage trait, read-only-aware storage wrappers, and an
  optional file-mapped `doublets` backend with LiNo/binary round-trip coverage.

### Added
- Added exact-match concept interning, language-bound expression links, external-id alias links, and LiNo concept-set import for the shared concept ontology.

### Added
- Added starter natural-language grammaticality parsing with UD-style morphosyntax links, pass/fail fixtures for the ten natural-language targets, and recoverable error links for ungrammatical fixtures.

### Added
- Added configurable `TranslationRuleSet` values, a runtime
  `TranslationRuleRegistry`, LiNo-backed rule-set loading, template
  placeholders, and missing-rule diagnostics for from-meta reconstruction.

### Added
- Added queryable language profiles with JavaScript transform enforcement and rule-set-derived profile domains.

### Added
- Added incremental source edit reparsing, stable outside-edit link IDs, and structural snapshot diff reporting.

### Added
- Added composable `LinkRule` query algebra with relational rules, boolean composition, named sub-rules, ellipsis gap matching, typed metavariables, plain-text token patterns, traversal strategies, and valid/invalid rule snapshot suites.
- Added quasiquote replacement templates with placeholder validation and parenthesization-conservative captured-text replacement.

### Added
- Add the API operation/style parity registry, fluent network pipeline, and link-cli-style substitution text runner.

### Added
- Wire the canonical `ts-parser-perl` tree-sitter grammar as a second-tier
  programming-language target without upgrading the project-wide tree-sitter
  runtime.

### Added
- Added source generation helpers for constructed syntax networks: `insert_source_token`, `insert_syntax_node`, `render_source`, `render_source_from`, and `render_source_from_document`.

### Added
- Added wave-two competitor and ecosystem parity fixtures with executable transform, reconstruction, grammar, and storage gates.
- Added a recorded `cargo llvm-cov` line-coverage floor to CI so coverage cannot silently regress.

### Added
- Shared, language-free document-formatting concept ontology (`document_formatting` module): inline concepts (emphasis, strong, strikethrough, inline-code, hyperlink, image, line-break) and block concepts (heading with level, paragraph, blockquote, bullet/ordered lists, list-item, code-block with language, thematic-break, table/row/cell), each seeded with Markdown and HTML syntax mappings.
- `seed_common_concept_ontology()` now also seeds the formatting concepts, and `ConceptOntologySeedReport::formatting_concepts()` reports how many were added.
- `LinkNetwork::resolve_document_format` / `render_document_format` / `translate_document_format` so the same concept link reconstructs as `**…**` in Markdown and `<strong>…</strong>` in HTML; Markdown `**bold**` and HTML `<strong>bold</strong>` reach the one shared `strong` concept under semantic projection.
- `FormattingDocument` concept layer with `parse_markup_document` and `translate_markup_document` for full Markdown ⇄ HTML document round-trips through one concept ontology (heading/paragraph/list/bold/italic/link).

### Added
- PDF document-format support (issue #84): a documented, uncompressed text PDF profile (`document_formatting::render_pdf_document` / `parse_pdf_document`) that renders a language-free `FormattingDocument` to a valid single-page PDF (correct `xref` offsets, object table, and stream `Length`) and parses it back into the same concept tree. Block role is carried by marked content (`/H1`…`/H6`, `/P`, `/UL`/`/OL`, `/LI`) and inline bold/italic by the selected font resource (`/F1` regular, `/F2` strong, `/F3` emphasis).
- `parse("…", "pdf", …)` dispatches to a new `pdf_parser` that builds a byte-exact lossless network (`reconstruct_text()` returns the input verbatim) and adds additive `Concept`/`Object` structure links recovering heading/paragraph/list/list-item and bold/italic.
- `reconstruct_text_as("PDF", …)` renders a structurally equivalent PDF through the shared formatting concept layer: a PDF source re-renders byte-for-byte, while a Markdown/HTML source is translated into an equivalent PDF, and `translate_markup_document` now bridges Markdown/HTML ⇄ PDF.
- `PDF` markup target in `MARKUP_LANGUAGE_TARGETS` with a bold + heading + paragraph round-trip `LANGUAGE_FIXTURES` entry, plus `docs/pdf-fidelity.md` documenting the round-trip fidelity matrix for supported and lossy/unsupported PDF features.

### Added
- DOCX (OOXML) document-format support (issue #85): a documented OOXML text profile (`document_formatting::render_docx_document` / `parse_docx_document`) that renders a language-free `FormattingDocument` to `word/document.xml` WordprocessingML and parses it back into the same concept tree. Block role is carried by paragraph properties (`<w:pStyle w:val="HeadingN"/>` headings, bare `<w:p>` paragraphs, `<w:numPr>` `numId` 1/2 bullet/ordered list items) and inline bold/italic by run properties (`<w:b/>` → `strong`, `<w:i/>` → `emphasis`).
- A binary OPC packaging layer (`document_formatting::render_docx_package` / `parse_docx_package`) that assembles a valid `.docx` ZIP (stored entries with a self-implemented CRC-32, no new dependencies) containing `[Content_Types].xml`, the relationship parts, `word/document.xml`, `word/styles.xml`, and `word/numbering.xml`, and reads `word/document.xml` back out.
- `parse("…", "docx", …)` dispatches to a new `docx_parser` that builds a byte-exact lossless network (`reconstruct_text()` returns the input verbatim) and adds additive `Concept`/`Object` structure links recovering heading/paragraph/list/list-item and bold/italic.
- `reconstruct_text_as("DOCX", …)` renders structurally equivalent OOXML through the shared formatting concept layer: a DOCX source re-renders byte-for-byte, while a Markdown/HTML/PDF source is translated into equivalent OOXML, and `translate_markup_document` now bridges Markdown/HTML/PDF ⇄ DOCX.
- `DOCX` markup target in `MARKUP_LANGUAGE_TARGETS` with a bold + heading + bullet-list round-trip `LANGUAGE_FIXTURES` entry, plus `docs/docx-fidelity.md` documenting the two-layer round-trip fidelity matrix for supported and lossy/unsupported OOXML features.

### Fixed
- Fixed the GitHub Pages site returning HTTP 404 at the root URL
  (<https://link-foundation.github.io/meta-language>). The deploy job uploaded
  raw `cargo doc` output, which has no root `index.html` (rustdoc only emits
  `target/doc/<crate>/index.html`), so the Pages root 404'd while docs were only
  reachable at `/meta_language/`. See issue #90.

### Added
- A project website with a landing page (description + docs links) under
  `docs/site/`, deployed at the Pages root.
- An interactive WebAssembly "Links Notation playground" demo in a new
  standalone `web/` crate (wraps the wasm-compatible `links-notation` crate),
  served at `/demo/`.
- `scripts/build-site.rs` to assemble the website (`_site/`) locally and in CI:
  landing page at `/`, demo at `/demo/`, and `rustdoc` under `/api/` with a root
  redirect.
- `docs/case-studies/issue-90` documenting the timeline, requirements, root
  cause analysis, and solution plans.

### Changed
- The `Deploy Website` CI job (formerly `Deploy Rust Documentation`) now builds
  the WebAssembly demo and assembles the full site instead of deploying
  `target/doc` directly.

### Added
- Cross-format document reconstruction and round-trip translation (issue #86): `reconstruct_text_as("txt" | "Markdown" | "HTML" | "PDF" | "DOCX", …)` now works over the shared, language-free formatting concept layer (issue #83), so a document parsed from any supported format reconstructs into any other when the source uses only concepts both formats support. A same-format target re-renders byte-for-byte; a cross-format target is translated through the concept tree, preserving heading/paragraph/list and bold/italic/link structure.
- `txt` joins Markdown, HTML, PDF, and DOCX as a first-class document format in `parse_markup_document` / `render_markup_document`: blank-line-separated paragraphs parse into the concept layer, and the concept layer flattens to plain text (headings to plain lines, lists to `- `/`N. ` markers, inline styling dropped) as the documented lossy fallback target.
- Per-format capability profiles (`document_format_profile`, `DOCUMENT_FORMATS`, `CROSS_FORMAT_CONCEPTS`, `canonical_document_format`) expose each format's `LanguageProfile` over the formatting concept ontology, reporting for every cross-format concept either native support or a documented lossy fallback rather than silent data loss.
- `LanguageProfile` gained `with_concept_fallback` / `concept_fallback` / `fallbacks` to declare and query the lossy fallback for concepts a target cannot represent natively.
- A round-trip matrix test covering every ordered pair of `{txt, Markdown, HTML, PDF, DOCX}` (a sample built from the concepts both formats share survives `A → concepts → B → concepts → A`), plus `docs/cross-format-fidelity.md` documenting the cross-format translation entry point and the per-format fidelity matrix.

### Added
- Case study and implementation backlog for grammar extensibility & inference (issue #93): `docs/case-studies/issue-93/` collects the requirement register, a grammar-inference literature review (Gold (1967), RPNI/EDSM, Sequitur, the GLADE→Arvada→TreeVada→Kedavra→NatGI line, ISLearn), a licence-vetted library/ecosystem survey, an existing-capability gap analysis, a competitive analysis pinning the metrics to beat, and per-epic solution plans with the issue dependency DAG. The 34 maximally-detailed sub-issue specs under `proposed-issues/` were filed as GitHub issues #95–#128, each attached as a native sub-issue of #93 with all 51 `blocked-by` dependency edges wired via the GitHub REST API. This is a planning/research deliverable only — no library code changes.

### Added
- Added a public grammar IR with expression builders and links round-trip encoding.

### Added
- Added grammar surface syntax parsing, writing, and LiNo bridge helpers for the grammar IR.

### Added
- Seed grammar-construct concepts and expose grammar expression concept alignment helpers.

### Added
- Added grammar subsystem documentation, README integration, and tests for the grammar docs page set and relative links.

### Added
- Added a public BNF grammar importer that lowers classic BNF productions into the grammar IR.

### Added
- Added an EBNF grammar importer that lowers ISO-style EBNF constructs into the grammar IR.

### Added
- Added an ABNF grammar importer with RFC 5234 core rules, RFC 7405 string sensitivity, numeric terminals, repetition, and incremental alternative lowering.

### Added
- Added a PEG `.pest` grammar importer backed by `pest_meta`.

### Added
- Add a tree-sitter `grammar.json` importer for lowering generated tree-sitter grammars into the grammar IR.

### Added
- Add a clean-room ANTLR v4 `.g4` importer for lowering grammar files into the grammar IR.

### Added
- Add clean-room Lark and GBNF grammar importers that lower into the grammar IR.

### Added
- Added public BNF, EBNF, and ABNF grammar emitters for the grammar IR.

### Added
- Add a public GBNF grammar emitter for LLM grammar-constraint interop.

### Added
- Added a public pest PEG grammar emitter for the grammar IR.

### Added
- Added Rust parser codegen artifacts for grammar IR, including pest derive stubs and AST type rendering.

### Added
- Added Peggy grammar and JavaScript parser module code generation for the grammar IR.

### Added
- Added a public tree-sitter `grammar.js` emitter for the grammar IR.

### Added

- Add state-merging regular inference with RPNI, EDSM, and ALERGIA.

### Added

- Add MDL/Occam minimization for inferred grammars.

### Added

- Add semantic constraint inference for grammar corpora

### Added
- Added concept-aligned grammar surface translation with deterministic rule-name, non-terminal, and doc-comment rewrites.

### Added
- Added a deterministic grammar inference evaluation harness with sampling, oracle scoring, MDL size metrics, and smoke corpus reports.

### Added
- Added deterministic lexical class inference with category-based tokenisation and grammar IR token rules.

### Added
- Added a Sequitur structural-compression inference pass that emits deterministic inferred grammar IR.

### Added
- Added a delimiter-skeleton structural prior API for positive grammar inference.

### Added
- Add deterministic CFG inference over delimiter structural priors with oracle-checked positive recall.

### Added
- Add deterministic and optional LLM-assisted grammar inference advisors for rule naming and merge ranking.

### Added
- Add a runtime `GrammarParser` and registry helpers for parsing with imported or inferred grammars.

### Added
- Add opt-in active regular-language learning with L*, DFA output, parser-backed oracles, and right-linear grammar lowering.

### Added
- Added CLI grammar subcommands for inference, import, emit, and concept-aligned translation.

### Added
- Add the E3 competitor corpus manifest, vendored benchmark fixtures, and a D1/D5 benchmark gate for included subjects.

### Added
- Added grammar-format fidelity profiles and the generated BNF round-trip matrix documentation.

### Added
- Added semantic grammar validation diagnostics for undefined references, left recursion, unreachable rules, nullable repetitions, duplicate rules, and unused captures.

### Added
- Added end-to-end grammar inference examples and integration tests for Rust, JavaScript, GBNF, and CLI emit pipelines.

### Added
- Added the `js/` JavaScript implementation package for the meta-language core.
- Added Rust/JavaScript parity manifest checks and separate JS/Rust workflows.

### Added
- Added JavaScript truth-value semantics parity for `TruthValue`,
  `Probability`, and `ProbabilisticTruthValue`.

### Added
- Added a module-level Rust/JavaScript parity gate: every `pub mod` in
  `rust/src/lib.rs` must now be classified in `parity/language-features.json`
  (`rustModules`) as either `ported` (naming an implemented feature row) or
  `rust-only` (with a justification). The JavaScript checker
  (`js/scripts/check-js-rust-parity.mjs`) and the Rust test
  (`rust/tests/unit/parity_manifest.rs`) both fail when the public Rust surface
  and the manifest drift apart, so a new Rust module can no longer slip in
  without an explicit JavaScript parity decision.
- Ported four previously Rust-only modules to JavaScript with full test
  coverage and parity manifest rows: read-only access (`ReadOnlyNetwork`,
  `EngineNetwork`, `AccessMode`), embedded-region detection (`EmbeddedRegion`,
  `detectEmbeddedRegions`, `RegionDetectionPolicy`), language profiles
  (`LanguageProfile`, `LanguageProfileLinks`, `LanguageProfileViolation`), and
  the link-rule query algebra (`LinkRule`, `LinkRuleRegistry`,
  `TraversalStrategy`, and the rule snapshot suite).
- Added `LinkMetadata.withDefinition`/`definition` to the JavaScript primitives
  for parity with the Rust `LinkMetadata` definition field, and recorded
  `parse-configuration` and `link-flags` feature rows that already existed in
  both languages but were previously untracked.

### Added
- Added the JavaScript `LinkType.Token` alias for lossless source-token links to match Rust `LinkType::Token` naming.

### Fixed
- Made JavaScript `TranslationRuleSet.toLino()` and `fromLino()` use the Rust-compatible canonical LiNo rule-set schema instead of JSON.

## [0.50.0] - 2026-06-28

### Added
- Changeset-style fragment format with frontmatter for specifying version bump type
- New `get-bump-type.mjs` script to automatically determine version bump from fragments
- Automatic version bumping on merge to main based on changelog fragments
- Detailed documentation for the changelog fragment system in `changelog.d/README.md`

### Changed
- Updated `collect-changelog.mjs` to strip frontmatter when collecting fragments
- Updated `version-and-commit.mjs` to handle frontmatter in fragments
- Enhanced release workflow to automatically determine bump type from changesets

### Changed
- Add `detect-changes` job with cross-platform `detect-code-changes.mjs` script
- Make lint job independent of changelog check (runs based on file changes only)
- Allow docs-only PRs without changelog fragment requirement
- Handle changelog check 'skipped' state in dependent jobs
- Exclude `changelog.d/`, `docs/`, `experiments/`, `examples/` folders and markdown files from code changes detection

### Fixed
- Fixed README.md to correctly reference Node.js scripts (`.mjs`) instead of Python scripts (`.py`)
- Updated project structure in README.md to match actual script files in `scripts/` directory
- Fixed example code in README.md that had invalid Rust with two `main` functions

### Added

- Added crates.io publishing support to CI/CD workflow
- Added `release_mode` input with "instant" and "changelog-pr" options for manual releases
- Added `--tag-prefix` and `--crates-io-url` options to create-github-release.mjs script
- Added comprehensive case study documentation for Issue #11 in docs/case-studies/issue-11/

### Changed

- Changed changelog fragment check from warning to error (exit 1) to enforce changelog requirements
- Updated job conditions with `always() && !cancelled()` to fix workflow_dispatch job skipping issue
- Renamed manual-release job to "Instant Release" for clarity

### Fixed

- Fixed deprecated `::set-output` GitHub Actions command in version-and-commit.mjs
- Fixed workflow_dispatch triggering issues where lint/build/release jobs were incorrectly skipped

### Fixed

- Fixed changelog fragment check to validate that a fragment is **added in the PR diff** rather than just checking if any fragments exist in the directory. This prevents the check from incorrectly passing when there are leftover fragments from previous PRs that haven't been released yet.

### Changed

- Converted shell scripts in `release.yml` to cross-platform `.mjs` scripts for improved portability and performance:
  - `check-changelog-fragment.mjs` - validates changelog fragment is added in PR diff
  - `git-config.mjs` - configures git user for CI/CD
  - `check-release-needed.mjs` - checks if release is needed
  - `publish-crate.mjs` - publishes package to crates.io
  - `create-changelog-fragment.mjs` - creates changelog fragments for manual releases
  - `get-version.mjs` - gets current version from Cargo.toml

### Added

- Added `check-version-modification.mjs` script to detect manual version changes in Cargo.toml
- Added `version-check` job to CI/CD workflow that runs on pull requests
- Added skip logic for automated release branches (changelog-manual-release-*, changeset-release/*, release/*, automated-release/*)

### Changed

- Version modifications in Cargo.toml are now blocked in pull requests to enforce automated release pipeline

### Added

- Added support for `CARGO_REGISTRY_TOKEN` as alternative to `CARGO_TOKEN` for crates.io publishing
- Added case study documentation for Issue #17 (yargs reserved word and dual token support)

### Changed

- Updated workflow to use fallback logic: `${{ secrets.CARGO_REGISTRY_TOKEN || secrets.CARGO_TOKEN }}`
- Improved publish-crate.mjs to check both `CARGO_REGISTRY_TOKEN` and `CARGO_TOKEN` environment variables
- Added warning message when neither token is set

### Added
- New `scripts/rust-paths.mjs` utility for automatic Rust package root detection
- Support for both single-language and multi-language repository structures in all CI/CD scripts
- Configuration options via `--rust-root` CLI argument and `RUST_ROOT` environment variable
- Comprehensive case study documentation in `docs/case-studies/issue-19/`

### Changed
- Updated all release scripts to use the new path detection utility:
  - `scripts/bump-version.mjs`
  - `scripts/check-release-needed.mjs`
  - `scripts/collect-changelog.mjs`
  - `scripts/get-bump-type.mjs`
  - `scripts/get-version.mjs`
  - `scripts/publish-crate.mjs`
  - `scripts/version-and-commit.mjs`

### Changed

- **check-release-needed.mjs**: Now checks crates.io API directly instead of git tags to determine if a version is already released. This prevents false positives where git tags exist but the package was never actually published to crates.io.

### Added

- **CI/CD Troubleshooting Guide**: New documentation at `docs/ci-cd/troubleshooting.md` covering common issues like skipped jobs, false positive version checks, publishing failures, and secret configuration.

- **Enhanced Error Handling in publish-crate.mjs**: Added specific detection and helpful error messages for authentication failures, including guidance on secret configuration and workflow setup.

- **Case Study Documentation**: Added comprehensive case study at `docs/case-studies/issue-21/` analyzing CI/CD failures from browser-commander repository (issues #27, #29, #31, #33) with timeline, root causes, and lessons learned.

### Fixed

- **Prevent False Positive Version Checks**: The release workflow now correctly identifies unpublished versions by checking crates.io instead of relying on git tags, which can exist without the package being published.

### Changed

- Translated all CI/CD scripts from JavaScript (.mjs) to Rust (.rs) using rust-script
- Scripts now use native Rust with rust-script for execution in shell
- Removed Node.js dependency from CI/CD pipeline
- Updated GitHub Actions workflow to use rust-script instead of node
- Updated README and CONTRIBUTING documentation with new script references

### Added

- Cache `restore-keys` for partial cache hits across all workflow jobs
- Explicit `token` parameter in checkout for release jobs
- Code coverage job with `cargo-llvm-cov` and Codecov integration
- Codecov badge in README.md
- Pre-release version support (e.g., `0.1.0-beta.1`) in version parsing
- `--release-label` parameter for multi-language release disambiguation
- `ensure_version_exceeds_published()` logic to prevent publishing duplicate versions
- `get_max_published_version()` to query highest non-yanked version from crates.io
- `max_published_version` output from check-release-needed for downstream use
- Version fallback logic in auto-release Create GitHub Release step

### Changed

- Updated `actions/checkout` from v4 to v6
- Updated `actions/cache` from v4 to v5
- Updated `peter-evans/create-pull-request` from v7 to v8
- Made `publish-crate.rs` fail (exit 1) when version already exists on crates.io
- Improved `create-github-release.rs` to check combined stdout+stderr and detect "Validation Failed"

### Fixed

- Fix publish steps overriding workflow-level CARGO_TOKEN fallback, breaking CARGO_REGISTRY_TOKEN-only configurations (#32)
- Fix non-fast-forward push failures in multi-workflow repos by adding fetch/rebase and push retry logic (#31)
- Add mono-repo path support to check-changelog-fragment.rs, check-version-modification.rs, and create-changelog-fragment.rs
- Add `!cancelled()` guard to test job condition to respect workflow cancellation

### Fixed

- Fixed `version-and-commit.rs` to check crates.io instead of git tags for determining if a version is already released
- This prevents the release pipeline from getting stuck when git tags exist without corresponding crates.io publication

### Added

- Added `--tag-prefix` support to `version-and-commit.rs` for multi-language repository compatibility
- Added crates.io and docs.rs badges to README.md
- Added automatic crates.io and docs.rs badge injection in GitHub release notes
- Added documentation deployment job to CI/CD pipeline (deploys to GitHub Pages after release)
- Added case study documentation for issue #25

### Fixed
- Fixed unsupported look-ahead regex in `create-github-release.rs` that caused a panic when parsing CHANGELOG.md. Replaced with a two-step approach using only features supported by Rust's `regex` crate.

### Changed
- Restructured example application as a simple CLI sum calculator using `lino-arguments`
- Renamed default package to `example-sum-package-name` with Unlicense license
- Reorganized test structure: `tests/unit/sum.rs`, `tests/integration/sum.rs`, `tests/unit/ci-cd/`
- Converted experiment scripts into proper unit tests in `tests/unit/ci-cd/changelog_parsing.rs`
- Added CI/CD skip logic for template default package name `example-sum-package-name`
- Updated README.md badges and documentation

### Fixed

- Change detection script now uses per-commit diff instead of full PR diff, so commits touching only non-code files correctly skip CI jobs even when earlier commits in the same PR changed code files

### Fixed
- Make release scripts resolve the publishable crate manifest when the repository root uses a Cargo workspace manifest.

### Fixed
- Decoupled GitHub Pages documentation deployment from package release publication and fixed release-script warning failures under `RUSTFLAGS=-Dwarnings`.

### Changed
- Added explicit GitHub Actions job timeouts and documented Rust test timeout guidance.

### Fixed
- Added a non-blocking warning threshold to the Rust file-size check so near-limit files are surfaced before concurrent PR merges can exceed the hard limit.

### Fixed
- Made `create-github-release.rs` build GitHub release titles as `[Language] X.Y.Z` instead of reusing the tag prefix.

### Added
- Added optional Docker Hub image publishing tied to Rust crate releases, including crates.io visibility waiting, version/latest image tags, and Docker Hub badges in GitHub release notes.

### Changed
- Release completeness checks now self-heal when crates.io exists but configured Docker Hub or GitHub release artifacts are missing.

### Fixed
- Switched documentation deployment to the official GitHub Pages artifact workflow so repositories using GitHub Actions as their Pages source do not get false-positive branch-push deploys.

### Added
- Documented the one-time `Settings → Pages → Source = GitHub Actions` prerequisite for the `deploy-docs` job in `README.md` and as a comment above the `deploy-docs` job in `release.yml`, so downstream template users hit a documented setup step instead of a `Get Pages site failed` error on the first deploy.

### Added
- Tracking case study at `docs/case-studies/issue-52/` registering the `browser-commander` + Playwright preview-regeneration pattern from [`konard/vk-bot-desktop#52`](https://github.com/konard/vk-bot-desktop/pull/52), with an activation checklist for when an example-app surface lands in this template. Documentation only — no workflow, script, or runtime code changes. Primary upstream tracking issue: [`link-foundation/js-ai-driven-development-pipeline-template#62`](https://github.com/link-foundation/js-ai-driven-development-pipeline-template/issues/62).

### Fixed
- Release automation now keeps the workspace package entry in `Cargo.lock` synchronized when `scripts/version-and-commit.rs` bumps `Cargo.toml`, preventing stale lock-file version diffs in later pull requests.

### Fixed
- `scripts/publish-crate.rs` now treats crates.io HTTP 429 throttle responses ("You have published too many versions of this crate in the last 24 hours") as a deferred `publish_result=rate_limited` outcome (it writes the output, prints an explanatory banner and exits successfully) instead of a hard CI failure reported as a generic `failed` ("Failed to publish for unknown reason"). Authentication, already-published and unknown failures still exit non-zero. Failed-publish classification is consolidated through a single `classify_failure` function and `FailureKind` enum (with an `is_deferred` predicate), covered by unit tests runnable via `rust-script --test scripts/publish-crate.rs`.
- The release workflow (`.github/workflows/release.yml`) now gates crate-availability waiting, Docker Hub publishing and GitHub release creation on either an already-published crate or `publish_result=success`, so a deferred (rate-limited) crate upload no longer produces partial downstream release artifacts and the same version is retried automatically on the next push to `main`.

### Fixed
- Fixed reversed `cancel-in-progress` concurrency condition in `release.yml` that cancelled in-flight releases on `main` and never superseded older PR runs. The condition now uses `!=` so `main` releases run to completion while newer PR pushes cancel stale runs.

### Added
- Added a `scripts/check-crate-size.rs` guard that builds the `.crate` archive and fails the release before publishing when it exceeds the crates.io 10 MiB upload limit. The check runs in the build job and before publishing in both the auto-release and manual-release jobs.

### Changed
- Added a narrow `include` allowlist to `Cargo.toml` so docs, case studies, generated CI artifacts, changelog fragments, scripts, and experiments no longer inflate the published release archive.

### Fixed
- Prevented GitHub release creation from treating generic API validation failures as existing releases, and capped oversized release notes with a link to the full tagged changelog.

### Added
- Added the initial `meta-language` links-network core with self-description roots,
  verification, trivia attachment configuration, and a minimal CLI.
- Added the default lossless `parse` entry point, projection views, and a
  tested parity roadmap for competitor features, grammar embedding, and language
  coverage targets.
- Added exact reconstruction, mixed-region detection, query matching,
  substitution rules, concept reconstruction, object identity helpers,
  many-valued truth values, and executable parity fixtures for every tracked
  competitor target.
- Added executable language fixtures and tests for every requested Markdown,
  HTML, top-ten programming-language, and top-ten natural-language target.

### Fixed
- Added a CI guard that rejects Rust test modules and test attributes under `src/`, keeping tests in the `tests/` tree.

## Minor Changes

- Add immutable and mutable network snapshots with provenance and forward
  version commits for roadmap snapshot/versioning coverage.

### Added
- Added `txt` as a first-class markup/container target with a UTF-8 lossless
  fixture and content-sniffing fallback regions.

### Fixed
- Reconciled the natural-language target ordering with the Ethnologue 2025
  total-speaker order cited by the parity roadmap.

### Added
- Added a grammar-backed parser front end for Python, C, Java, C++, C#, JavaScript, and R using official tree-sitter grammar crates.

### Added
- Parse supported mixed-mode embedded regions into the host links network with grammar-backed syntax links.

### Added
- Added natural-language segmentation, identification, normalization, and bidi annotation links over lossless text parses.

### Added
- Added grammar-backed Visual Basic parsing with byte-exact reconstruction and recovery flag coverage.

bump: minor

- Add a grammar-backed `sql-ansi` SQL-family dialect fixture using
  `tree-sitter-sequel`.

### Added
- Added grammar-backed Delphi/Object Pascal parsing through `tree-sitter-pascal`.

### Added
- Enriched link queries with S-expression structural matching, captures, by-type construction, host predicate hooks, and link-cli-style variable substitution bindings.

### Added
- Added persistent snapshot structural sharing with interned metadata text storage.

### Added
- Added a query-transform surface for selecting captured links and replacing
  their source text while preserving unchanged bytes.

### Added
- Materialized self-description roots as controlled links with complete root-definition closure and round-trippable `describe` output.

### Added
- Added common concept ontology seeding from meta-expression's semantic lexicon, with shared concept links, syntax mappings, and structural programming-language concepts.

### Added
- Added semantic cross-language reconstruction for the Hawaii statehood fixture,
  including English/Russian naturalization and configurable formalization levels.

### Added
- Expanded `PARITY_FIXTURES` with multiple provenance-tracked fixtures for tree-sitter, LibCST, Recast, jscodeshift, Rowan, cstree, and Roslyn, including executable recovery and query/transform expectations.

### Added

- Expanded `PARITY_FIXTURES` with upstream-provenanced internal ecosystem corpora for links-notation, link-cli, lino-objects-codec, relative-meta-logic, formal-ai, and meta-expression.

### Added

- Added structural LiNo parsing for links-notation doublets, triplets, named links, indented IDs, and self-references while preserving byte-exact reconstruction.

### Added
- Added fixed-point probabilistic truth values for relative-meta-logic-style
  semantic confidence evaluation.

### Added
- Added grammar-backed Go parsing via the official `tree-sitter-go` grammar
  (accepts `Go`, `go`, and `golang`) so `LinkNetwork::parse` emits real
  `LinkType::Syntax` concrete-syntax links with byte-exact reconstruction.

### Added
- `AccessMode { Mutable, ReadOnly }` setting on `ParseConfiguration`
  (`with_access_mode` / `access_mode`), defaulting to `Mutable` so existing
  callers are unaffected.
- `LinkNetwork::freeze` / `as_read_only` yielding a `ReadOnlyNetwork` view that
  exposes only `&self` operations (query, project, reconstruct, verify,
  serialize); mutators are unreachable at compile time because the view never
  hands out `&mut LinkNetwork`.
- `LinkNetwork::parse_engine`, returning an `EngineNetwork` handle that honours
  the configured access mode: read-only parsing returns the frozen form and
  `EngineNetwork::as_mutable` rejects mutation with a `ReadOnlyViolation`
  diagnostic.
- Snapshot interop: `NetworkSnapshot::as_read_only` / `from_read_only` reuse the
  snapshot's `Arc<LinkNetwork>`, so the frozen form composes with snapshot
  versioning instead of duplicating it.

### Added
- Added grammar-backed Ruby parsing through `tree-sitter-ruby`, so
  `LinkNetwork::parse(source, "Ruby", ...)` (and the `rb` alias) now emits real
  `LinkType::Syntax` links instead of falling back to lossless plain text.

bump: minor

- Add a grammar-backed `TypeScript` front end using `tree-sitter-typescript`,
  wiring the `typescript`/`ts` labels to `LANGUAGE_TYPESCRIPT` and the `tsx`
  label to `LANGUAGE_TSX` so `LinkNetwork::parse` emits real `LinkType::Syntax`
  links for TypeScript and TSX sources.

### Added
- Added grammar-backed parsing for seven data-exchange / interchange formats
  through `src/tree_sitter_adapter.rs`: JSON (`tree-sitter-json`), YAML
  (`tree-sitter-yaml`, accepts `yaml`/`yml`), TOML (`tree-sitter-toml-ng`), XML
  and DTD (`tree-sitter-xml`), INI (`tree-sitter-ini`), Protocol Buffers
  (`tree-sitter-proto`, accepts `protobuf`/`proto`/`Protocol Buffers`), and
  GraphQL (`tree-sitter-graphql`, accepts `graphql`/`gql`). Each parses into
  real `LinkType::Syntax` concrete-syntax links and reconstructs byte-for-byte.
- Added a `DATA_FORMAT_TARGETS` registry in `src/parity.rs` (with the new
  `LanguageFamily::DataFormat`) gated by parity tests, mirroring
  `MARKUP_LANGUAGE_TARGETS`.
- Added per-format UTF-8 `LANGUAGE_FIXTURES` round-trip entries plus
  recovery-case and mixed-region tests (a `json` fence embedded in Markdown
  parses into the host links network).
- Added lossless CSV and JSON5 parsers for the two formats whose published
  tree-sitter crates still pin the incompatible `tree-sitter ~0.20` runtime.
  CSV is validated with the Rust `csv` crate and JSON5 is validated with
  `json5_nodes`; both emit structured syntax links and reconstruct
  byte-for-byte.

### Documentation
- Documented the nine wired data-format parsers (parser, version, license, root
  node) in `docs/parity-roadmap.md`, including the tree-sitter compatibility
  rationale for the in-repo CSV and JSON5 parsers.

### Added
- `LinkNetwork::to_lino` serializes an entire links network to canonical
  links-notation text, keyed by each link's numeric id (doublets-style id
  discipline), covering references, names, types, terms, definitions,
  languages, source spans, parse flags, and term registration.
- `LinkNetwork::from_lino` reconstructs the exact network from that text,
  forming a lossless round-trip (`from_lino(to_lino(n))` is isomorphic to `n`).
- `LinoSerializationError` reports parse and schema failures from `from_lino`.
- A round-trip property test over every language fixture plus synthetic
  networks, and a test that `to_lino` output is accepted by the
  `links-notation` 0.13 crate parser.
- `ParityCapability::LinoSerialization` and an output-side serialization
  parity fixture for the `links-notation` target.

### Changed
- Added the `links-notation` 0.13 crate as a dependency so serialized output
  aligns with the wider links-notation ecosystem.

### Added
- `ParserRegistry` maps language keys to `Arc<dyn LanguageParser>` with the
  built-in parser as a fallback. User registrations shadow the built-in
  dispatch for the same (case-insensitive) key; unregistered keys still route
  through the built-in set.
- `LinkNetwork::parse_with_registry` dispatches a parse through a registry.
- `examples/custom_parser_registry.rs` documents registering a custom parser.

### Added
- Added grammar-backed parsing for five popular programming languages
  immediately below the TIOBE top ten through `src/tree_sitter_adapter.rs`: PHP
  (`tree-sitter-php`, via its `LANGUAGE_PHP` symbol), Swift
  (`tree-sitter-swift`), Kotlin (`tree-sitter-kotlin-ng`, accepts `kotlin`/`kt`),
  Scala (`tree-sitter-scala`), and Lua (`tree-sitter-lua`). Each parses into real
  `LinkType::Syntax` concrete-syntax links and reconstructs byte-for-byte.
- Added a `SECOND_TIER_PROGRAMMING_LANGUAGE_TARGETS` registry in `src/parity.rs`
  gated by parity tests, mirroring `DATA_FORMAT_TARGETS`.
- Added per-language UTF-8 `LANGUAGE_FIXTURES` round-trip entries plus
  case-insensitive alias coverage and a per-language recovery fixture whose
  malformed source still reconstructs while exposing error/missing diagnostics.

### Documentation
- Documented the wired grammars (crate, version, license, root node) in
  `docs/parity-roadmap.md` and kept the temporary Perl follow-up tracked
  explicitly until `ts-parser-perl` was adopted.

### Added
- Add a Rust `ToLinks`/`FromLinks` codec with queryable type-shape links and shared/circular object graph round-trips through LiNo serialization.

### Added

- Added the `LinkStore` storage trait, read-only-aware storage wrappers, and an
  optional file-mapped `doublets` backend with LiNo/binary round-trip coverage.

### Added
- Added exact-match concept interning, language-bound expression links, external-id alias links, and LiNo concept-set import for the shared concept ontology.

### Added
- Added starter natural-language grammaticality parsing with UD-style morphosyntax links, pass/fail fixtures for the ten natural-language targets, and recoverable error links for ungrammatical fixtures.

### Added
- Added configurable `TranslationRuleSet` values, a runtime
  `TranslationRuleRegistry`, LiNo-backed rule-set loading, template
  placeholders, and missing-rule diagnostics for from-meta reconstruction.

### Added
- Added queryable language profiles with JavaScript transform enforcement and rule-set-derived profile domains.

### Added
- Added incremental source edit reparsing, stable outside-edit link IDs, and structural snapshot diff reporting.

### Added
- Added composable `LinkRule` query algebra with relational rules, boolean composition, named sub-rules, ellipsis gap matching, typed metavariables, plain-text token patterns, traversal strategies, and valid/invalid rule snapshot suites.
- Added quasiquote replacement templates with placeholder validation and parenthesization-conservative captured-text replacement.

### Added
- Add the API operation/style parity registry, fluent network pipeline, and link-cli-style substitution text runner.

### Added
- Wire the canonical `ts-parser-perl` tree-sitter grammar as a second-tier
  programming-language target without upgrading the project-wide tree-sitter
  runtime.

### Added
- Added source generation helpers for constructed syntax networks: `insert_source_token`, `insert_syntax_node`, `render_source`, `render_source_from`, and `render_source_from_document`.

### Added
- Added wave-two competitor and ecosystem parity fixtures with executable transform, reconstruction, grammar, and storage gates.
- Added a recorded `cargo llvm-cov` line-coverage floor to CI so coverage cannot silently regress.

### Added
- Shared, language-free document-formatting concept ontology (`document_formatting` module): inline concepts (emphasis, strong, strikethrough, inline-code, hyperlink, image, line-break) and block concepts (heading with level, paragraph, blockquote, bullet/ordered lists, list-item, code-block with language, thematic-break, table/row/cell), each seeded with Markdown and HTML syntax mappings.
- `seed_common_concept_ontology()` now also seeds the formatting concepts, and `ConceptOntologySeedReport::formatting_concepts()` reports how many were added.
- `LinkNetwork::resolve_document_format` / `render_document_format` / `translate_document_format` so the same concept link reconstructs as `**…**` in Markdown and `<strong>…</strong>` in HTML; Markdown `**bold**` and HTML `<strong>bold</strong>` reach the one shared `strong` concept under semantic projection.
- `FormattingDocument` concept layer with `parse_markup_document` and `translate_markup_document` for full Markdown ⇄ HTML document round-trips through one concept ontology (heading/paragraph/list/bold/italic/link).

### Added
- PDF document-format support (issue #84): a documented, uncompressed text PDF profile (`document_formatting::render_pdf_document` / `parse_pdf_document`) that renders a language-free `FormattingDocument` to a valid single-page PDF (correct `xref` offsets, object table, and stream `Length`) and parses it back into the same concept tree. Block role is carried by marked content (`/H1`…`/H6`, `/P`, `/UL`/`/OL`, `/LI`) and inline bold/italic by the selected font resource (`/F1` regular, `/F2` strong, `/F3` emphasis).
- `parse("…", "pdf", …)` dispatches to a new `pdf_parser` that builds a byte-exact lossless network (`reconstruct_text()` returns the input verbatim) and adds additive `Concept`/`Object` structure links recovering heading/paragraph/list/list-item and bold/italic.
- `reconstruct_text_as("PDF", …)` renders a structurally equivalent PDF through the shared formatting concept layer: a PDF source re-renders byte-for-byte, while a Markdown/HTML source is translated into an equivalent PDF, and `translate_markup_document` now bridges Markdown/HTML ⇄ PDF.
- `PDF` markup target in `MARKUP_LANGUAGE_TARGETS` with a bold + heading + paragraph round-trip `LANGUAGE_FIXTURES` entry, plus `docs/pdf-fidelity.md` documenting the round-trip fidelity matrix for supported and lossy/unsupported PDF features.

### Added
- DOCX (OOXML) document-format support (issue #85): a documented OOXML text profile (`document_formatting::render_docx_document` / `parse_docx_document`) that renders a language-free `FormattingDocument` to `word/document.xml` WordprocessingML and parses it back into the same concept tree. Block role is carried by paragraph properties (`<w:pStyle w:val="HeadingN"/>` headings, bare `<w:p>` paragraphs, `<w:numPr>` `numId` 1/2 bullet/ordered list items) and inline bold/italic by run properties (`<w:b/>` → `strong`, `<w:i/>` → `emphasis`).
- A binary OPC packaging layer (`document_formatting::render_docx_package` / `parse_docx_package`) that assembles a valid `.docx` ZIP (stored entries with a self-implemented CRC-32, no new dependencies) containing `[Content_Types].xml`, the relationship parts, `word/document.xml`, `word/styles.xml`, and `word/numbering.xml`, and reads `word/document.xml` back out.
- `parse("…", "docx", …)` dispatches to a new `docx_parser` that builds a byte-exact lossless network (`reconstruct_text()` returns the input verbatim) and adds additive `Concept`/`Object` structure links recovering heading/paragraph/list/list-item and bold/italic.
- `reconstruct_text_as("DOCX", …)` renders structurally equivalent OOXML through the shared formatting concept layer: a DOCX source re-renders byte-for-byte, while a Markdown/HTML/PDF source is translated into equivalent OOXML, and `translate_markup_document` now bridges Markdown/HTML/PDF ⇄ DOCX.
- `DOCX` markup target in `MARKUP_LANGUAGE_TARGETS` with a bold + heading + bullet-list round-trip `LANGUAGE_FIXTURES` entry, plus `docs/docx-fidelity.md` documenting the two-layer round-trip fidelity matrix for supported and lossy/unsupported OOXML features.

### Fixed
- Fixed the GitHub Pages site returning HTTP 404 at the root URL
  (<https://link-foundation.github.io/meta-language>). The deploy job uploaded
  raw `cargo doc` output, which has no root `index.html` (rustdoc only emits
  `target/doc/<crate>/index.html`), so the Pages root 404'd while docs were only
  reachable at `/meta_language/`. See issue #90.

### Added
- A project website with a landing page (description + docs links) under
  `docs/site/`, deployed at the Pages root.
- An interactive WebAssembly "Links Notation playground" demo in a new
  standalone `web/` crate (wraps the wasm-compatible `links-notation` crate),
  served at `/demo/`.
- `scripts/build-site.rs` to assemble the website (`_site/`) locally and in CI:
  landing page at `/`, demo at `/demo/`, and `rustdoc` under `/api/` with a root
  redirect.
- `docs/case-studies/issue-90` documenting the timeline, requirements, root
  cause analysis, and solution plans.

### Changed
- The `Deploy Website` CI job (formerly `Deploy Rust Documentation`) now builds
  the WebAssembly demo and assembles the full site instead of deploying
  `target/doc` directly.

### Added
- Cross-format document reconstruction and round-trip translation (issue #86): `reconstruct_text_as("txt" | "Markdown" | "HTML" | "PDF" | "DOCX", …)` now works over the shared, language-free formatting concept layer (issue #83), so a document parsed from any supported format reconstructs into any other when the source uses only concepts both formats support. A same-format target re-renders byte-for-byte; a cross-format target is translated through the concept tree, preserving heading/paragraph/list and bold/italic/link structure.
- `txt` joins Markdown, HTML, PDF, and DOCX as a first-class document format in `parse_markup_document` / `render_markup_document`: blank-line-separated paragraphs parse into the concept layer, and the concept layer flattens to plain text (headings to plain lines, lists to `- `/`N. ` markers, inline styling dropped) as the documented lossy fallback target.
- Per-format capability profiles (`document_format_profile`, `DOCUMENT_FORMATS`, `CROSS_FORMAT_CONCEPTS`, `canonical_document_format`) expose each format's `LanguageProfile` over the formatting concept ontology, reporting for every cross-format concept either native support or a documented lossy fallback rather than silent data loss.
- `LanguageProfile` gained `with_concept_fallback` / `concept_fallback` / `fallbacks` to declare and query the lossy fallback for concepts a target cannot represent natively.
- A round-trip matrix test covering every ordered pair of `{txt, Markdown, HTML, PDF, DOCX}` (a sample built from the concepts both formats share survives `A → concepts → B → concepts → A`), plus `docs/cross-format-fidelity.md` documenting the cross-format translation entry point and the per-format fidelity matrix.

### Added
- Case study and implementation backlog for grammar extensibility & inference (issue #93): `docs/case-studies/issue-93/` collects the requirement register, a grammar-inference literature review (Gold (1967), RPNI/EDSM, Sequitur, the GLADE→Arvada→TreeVada→Kedavra→NatGI line, ISLearn), a licence-vetted library/ecosystem survey, an existing-capability gap analysis, a competitive analysis pinning the metrics to beat, and per-epic solution plans with the issue dependency DAG. The 34 maximally-detailed sub-issue specs under `proposed-issues/` were filed as GitHub issues #95–#128, each attached as a native sub-issue of #93 with all 51 `blocked-by` dependency edges wired via the GitHub REST API. This is a planning/research deliverable only — no library code changes.

### Added
- Added a public grammar IR with expression builders and links round-trip encoding.

### Added
- Added grammar surface syntax parsing, writing, and LiNo bridge helpers for the grammar IR.

### Added
- Seed grammar-construct concepts and expose grammar expression concept alignment helpers.

### Added
- Added grammar subsystem documentation, README integration, and tests for the grammar docs page set and relative links.

### Added
- Added a public BNF grammar importer that lowers classic BNF productions into the grammar IR.

### Added
- Added an EBNF grammar importer that lowers ISO-style EBNF constructs into the grammar IR.

### Added
- Added an ABNF grammar importer with RFC 5234 core rules, RFC 7405 string sensitivity, numeric terminals, repetition, and incremental alternative lowering.

### Added
- Added a PEG `.pest` grammar importer backed by `pest_meta`.

### Added
- Add a tree-sitter `grammar.json` importer for lowering generated tree-sitter grammars into the grammar IR.

### Added
- Add a clean-room ANTLR v4 `.g4` importer for lowering grammar files into the grammar IR.

### Added
- Add clean-room Lark and GBNF grammar importers that lower into the grammar IR.

### Added
- Added public BNF, EBNF, and ABNF grammar emitters for the grammar IR.

### Added
- Add a public GBNF grammar emitter for LLM grammar-constraint interop.

### Added
- Added a public pest PEG grammar emitter for the grammar IR.

### Added
- Added Rust parser codegen artifacts for grammar IR, including pest derive stubs and AST type rendering.

### Added
- Added Peggy grammar and JavaScript parser module code generation for the grammar IR.

### Added
- Added a public tree-sitter `grammar.js` emitter for the grammar IR.

### Added

- Add state-merging regular inference with RPNI, EDSM, and ALERGIA.

### Added

- Add MDL/Occam minimization for inferred grammars.

### Added

- Add semantic constraint inference for grammar corpora

### Added
- Added concept-aligned grammar surface translation with deterministic rule-name, non-terminal, and doc-comment rewrites.

### Added
- Added a deterministic grammar inference evaluation harness with sampling, oracle scoring, MDL size metrics, and smoke corpus reports.

### Added
- Added deterministic lexical class inference with category-based tokenisation and grammar IR token rules.

### Added
- Added a Sequitur structural-compression inference pass that emits deterministic inferred grammar IR.

### Added
- Added a delimiter-skeleton structural prior API for positive grammar inference.

### Added
- Add deterministic CFG inference over delimiter structural priors with oracle-checked positive recall.

### Added
- Add deterministic and optional LLM-assisted grammar inference advisors for rule naming and merge ranking.

### Added
- Add a runtime `GrammarParser` and registry helpers for parsing with imported or inferred grammars.

### Added
- Add opt-in active regular-language learning with L*, DFA output, parser-backed oracles, and right-linear grammar lowering.

### Added
- Added CLI grammar subcommands for inference, import, emit, and concept-aligned translation.

### Added
- Add the E3 competitor corpus manifest, vendored benchmark fixtures, and a D1/D5 benchmark gate for included subjects.

### Added
- Added grammar-format fidelity profiles and the generated BNF round-trip matrix documentation.

### Added
- Added semantic grammar validation diagnostics for undefined references, left recursion, unreachable rules, nullable repetitions, duplicate rules, and unused captures.

### Added
- Added end-to-end grammar inference examples and integration tests for Rust, JavaScript, GBNF, and CLI emit pipelines.

### Added
- Added the `js/` JavaScript implementation package for the meta-language core.
- Added Rust/JavaScript parity manifest checks and separate JS/Rust workflows.

### Added
- Added JavaScript truth-value semantics parity for `TruthValue`,
  `Probability`, and `ProbabilisticTruthValue`.

### Added
- Added a module-level Rust/JavaScript parity gate: every `pub mod` in
  `rust/src/lib.rs` must now be classified in `parity/language-features.json`
  (`rustModules`) as either `ported` (naming an implemented feature row) or
  `rust-only` (with a justification). The JavaScript checker
  (`js/scripts/check-js-rust-parity.mjs`) and the Rust test
  (`rust/tests/unit/parity_manifest.rs`) both fail when the public Rust surface
  and the manifest drift apart, so a new Rust module can no longer slip in
  without an explicit JavaScript parity decision.
- Ported four previously Rust-only modules to JavaScript with full test
  coverage and parity manifest rows: read-only access (`ReadOnlyNetwork`,
  `EngineNetwork`, `AccessMode`), embedded-region detection (`EmbeddedRegion`,
  `detectEmbeddedRegions`, `RegionDetectionPolicy`), language profiles
  (`LanguageProfile`, `LanguageProfileLinks`, `LanguageProfileViolation`), and
  the link-rule query algebra (`LinkRule`, `LinkRuleRegistry`,
  `TraversalStrategy`, and the rule snapshot suite).
- Added `LinkMetadata.withDefinition`/`definition` to the JavaScript primitives
  for parity with the Rust `LinkMetadata` definition field, and recorded
  `parse-configuration` and `link-flags` feature rows that already existed in
  both languages but were previously untracked.

### Added
- Added the JavaScript `LinkType.Token` alias for lossless source-token links to match Rust `LinkType::Token` naming.

## [0.49.0] - 2026-06-28

### Added
- Changeset-style fragment format with frontmatter for specifying version bump type
- New `get-bump-type.mjs` script to automatically determine version bump from fragments
- Automatic version bumping on merge to main based on changelog fragments
- Detailed documentation for the changelog fragment system in `changelog.d/README.md`

### Changed
- Updated `collect-changelog.mjs` to strip frontmatter when collecting fragments
- Updated `version-and-commit.mjs` to handle frontmatter in fragments
- Enhanced release workflow to automatically determine bump type from changesets

### Changed
- Add `detect-changes` job with cross-platform `detect-code-changes.mjs` script
- Make lint job independent of changelog check (runs based on file changes only)
- Allow docs-only PRs without changelog fragment requirement
- Handle changelog check 'skipped' state in dependent jobs
- Exclude `changelog.d/`, `docs/`, `experiments/`, `examples/` folders and markdown files from code changes detection

### Fixed
- Fixed README.md to correctly reference Node.js scripts (`.mjs`) instead of Python scripts (`.py`)
- Updated project structure in README.md to match actual script files in `scripts/` directory
- Fixed example code in README.md that had invalid Rust with two `main` functions

### Added

- Added crates.io publishing support to CI/CD workflow
- Added `release_mode` input with "instant" and "changelog-pr" options for manual releases
- Added `--tag-prefix` and `--crates-io-url` options to create-github-release.mjs script
- Added comprehensive case study documentation for Issue #11 in docs/case-studies/issue-11/

### Changed

- Changed changelog fragment check from warning to error (exit 1) to enforce changelog requirements
- Updated job conditions with `always() && !cancelled()` to fix workflow_dispatch job skipping issue
- Renamed manual-release job to "Instant Release" for clarity

### Fixed

- Fixed deprecated `::set-output` GitHub Actions command in version-and-commit.mjs
- Fixed workflow_dispatch triggering issues where lint/build/release jobs were incorrectly skipped

### Fixed

- Fixed changelog fragment check to validate that a fragment is **added in the PR diff** rather than just checking if any fragments exist in the directory. This prevents the check from incorrectly passing when there are leftover fragments from previous PRs that haven't been released yet.

### Changed

- Converted shell scripts in `release.yml` to cross-platform `.mjs` scripts for improved portability and performance:
  - `check-changelog-fragment.mjs` - validates changelog fragment is added in PR diff
  - `git-config.mjs` - configures git user for CI/CD
  - `check-release-needed.mjs` - checks if release is needed
  - `publish-crate.mjs` - publishes package to crates.io
  - `create-changelog-fragment.mjs` - creates changelog fragments for manual releases
  - `get-version.mjs` - gets current version from Cargo.toml

### Added

- Added `check-version-modification.mjs` script to detect manual version changes in Cargo.toml
- Added `version-check` job to CI/CD workflow that runs on pull requests
- Added skip logic for automated release branches (changelog-manual-release-*, changeset-release/*, release/*, automated-release/*)

### Changed

- Version modifications in Cargo.toml are now blocked in pull requests to enforce automated release pipeline

### Added

- Added support for `CARGO_REGISTRY_TOKEN` as alternative to `CARGO_TOKEN` for crates.io publishing
- Added case study documentation for Issue #17 (yargs reserved word and dual token support)

### Changed

- Updated workflow to use fallback logic: `${{ secrets.CARGO_REGISTRY_TOKEN || secrets.CARGO_TOKEN }}`
- Improved publish-crate.mjs to check both `CARGO_REGISTRY_TOKEN` and `CARGO_TOKEN` environment variables
- Added warning message when neither token is set

### Added
- New `scripts/rust-paths.mjs` utility for automatic Rust package root detection
- Support for both single-language and multi-language repository structures in all CI/CD scripts
- Configuration options via `--rust-root` CLI argument and `RUST_ROOT` environment variable
- Comprehensive case study documentation in `docs/case-studies/issue-19/`

### Changed
- Updated all release scripts to use the new path detection utility:
  - `scripts/bump-version.mjs`
  - `scripts/check-release-needed.mjs`
  - `scripts/collect-changelog.mjs`
  - `scripts/get-bump-type.mjs`
  - `scripts/get-version.mjs`
  - `scripts/publish-crate.mjs`
  - `scripts/version-and-commit.mjs`

### Changed

- **check-release-needed.mjs**: Now checks crates.io API directly instead of git tags to determine if a version is already released. This prevents false positives where git tags exist but the package was never actually published to crates.io.

### Added

- **CI/CD Troubleshooting Guide**: New documentation at `docs/ci-cd/troubleshooting.md` covering common issues like skipped jobs, false positive version checks, publishing failures, and secret configuration.

- **Enhanced Error Handling in publish-crate.mjs**: Added specific detection and helpful error messages for authentication failures, including guidance on secret configuration and workflow setup.

- **Case Study Documentation**: Added comprehensive case study at `docs/case-studies/issue-21/` analyzing CI/CD failures from browser-commander repository (issues #27, #29, #31, #33) with timeline, root causes, and lessons learned.

### Fixed

- **Prevent False Positive Version Checks**: The release workflow now correctly identifies unpublished versions by checking crates.io instead of relying on git tags, which can exist without the package being published.

### Changed

- Translated all CI/CD scripts from JavaScript (.mjs) to Rust (.rs) using rust-script
- Scripts now use native Rust with rust-script for execution in shell
- Removed Node.js dependency from CI/CD pipeline
- Updated GitHub Actions workflow to use rust-script instead of node
- Updated README and CONTRIBUTING documentation with new script references

### Added

- Cache `restore-keys` for partial cache hits across all workflow jobs
- Explicit `token` parameter in checkout for release jobs
- Code coverage job with `cargo-llvm-cov` and Codecov integration
- Codecov badge in README.md
- Pre-release version support (e.g., `0.1.0-beta.1`) in version parsing
- `--release-label` parameter for multi-language release disambiguation
- `ensure_version_exceeds_published()` logic to prevent publishing duplicate versions
- `get_max_published_version()` to query highest non-yanked version from crates.io
- `max_published_version` output from check-release-needed for downstream use
- Version fallback logic in auto-release Create GitHub Release step

### Changed

- Updated `actions/checkout` from v4 to v6
- Updated `actions/cache` from v4 to v5
- Updated `peter-evans/create-pull-request` from v7 to v8
- Made `publish-crate.rs` fail (exit 1) when version already exists on crates.io
- Improved `create-github-release.rs` to check combined stdout+stderr and detect "Validation Failed"

### Fixed

- Fix publish steps overriding workflow-level CARGO_TOKEN fallback, breaking CARGO_REGISTRY_TOKEN-only configurations (#32)
- Fix non-fast-forward push failures in multi-workflow repos by adding fetch/rebase and push retry logic (#31)
- Add mono-repo path support to check-changelog-fragment.rs, check-version-modification.rs, and create-changelog-fragment.rs
- Add `!cancelled()` guard to test job condition to respect workflow cancellation

### Fixed

- Fixed `version-and-commit.rs` to check crates.io instead of git tags for determining if a version is already released
- This prevents the release pipeline from getting stuck when git tags exist without corresponding crates.io publication

### Added

- Added `--tag-prefix` support to `version-and-commit.rs` for multi-language repository compatibility
- Added crates.io and docs.rs badges to README.md
- Added automatic crates.io and docs.rs badge injection in GitHub release notes
- Added documentation deployment job to CI/CD pipeline (deploys to GitHub Pages after release)
- Added case study documentation for issue #25

### Fixed
- Fixed unsupported look-ahead regex in `create-github-release.rs` that caused a panic when parsing CHANGELOG.md. Replaced with a two-step approach using only features supported by Rust's `regex` crate.

### Changed
- Restructured example application as a simple CLI sum calculator using `lino-arguments`
- Renamed default package to `example-sum-package-name` with Unlicense license
- Reorganized test structure: `tests/unit/sum.rs`, `tests/integration/sum.rs`, `tests/unit/ci-cd/`
- Converted experiment scripts into proper unit tests in `tests/unit/ci-cd/changelog_parsing.rs`
- Added CI/CD skip logic for template default package name `example-sum-package-name`
- Updated README.md badges and documentation

### Fixed

- Change detection script now uses per-commit diff instead of full PR diff, so commits touching only non-code files correctly skip CI jobs even when earlier commits in the same PR changed code files

### Fixed
- Make release scripts resolve the publishable crate manifest when the repository root uses a Cargo workspace manifest.

### Fixed
- Decoupled GitHub Pages documentation deployment from package release publication and fixed release-script warning failures under `RUSTFLAGS=-Dwarnings`.

### Changed
- Added explicit GitHub Actions job timeouts and documented Rust test timeout guidance.

### Fixed
- Added a non-blocking warning threshold to the Rust file-size check so near-limit files are surfaced before concurrent PR merges can exceed the hard limit.

### Fixed
- Made `create-github-release.rs` build GitHub release titles as `[Language] X.Y.Z` instead of reusing the tag prefix.

### Added
- Added optional Docker Hub image publishing tied to Rust crate releases, including crates.io visibility waiting, version/latest image tags, and Docker Hub badges in GitHub release notes.

### Changed
- Release completeness checks now self-heal when crates.io exists but configured Docker Hub or GitHub release artifacts are missing.

### Fixed
- Switched documentation deployment to the official GitHub Pages artifact workflow so repositories using GitHub Actions as their Pages source do not get false-positive branch-push deploys.

### Added
- Documented the one-time `Settings → Pages → Source = GitHub Actions` prerequisite for the `deploy-docs` job in `README.md` and as a comment above the `deploy-docs` job in `release.yml`, so downstream template users hit a documented setup step instead of a `Get Pages site failed` error on the first deploy.

### Added
- Tracking case study at `docs/case-studies/issue-52/` registering the `browser-commander` + Playwright preview-regeneration pattern from [`konard/vk-bot-desktop#52`](https://github.com/konard/vk-bot-desktop/pull/52), with an activation checklist for when an example-app surface lands in this template. Documentation only — no workflow, script, or runtime code changes. Primary upstream tracking issue: [`link-foundation/js-ai-driven-development-pipeline-template#62`](https://github.com/link-foundation/js-ai-driven-development-pipeline-template/issues/62).

### Fixed
- Release automation now keeps the workspace package entry in `Cargo.lock` synchronized when `scripts/version-and-commit.rs` bumps `Cargo.toml`, preventing stale lock-file version diffs in later pull requests.

### Fixed
- `scripts/publish-crate.rs` now treats crates.io HTTP 429 throttle responses ("You have published too many versions of this crate in the last 24 hours") as a deferred `publish_result=rate_limited` outcome (it writes the output, prints an explanatory banner and exits successfully) instead of a hard CI failure reported as a generic `failed` ("Failed to publish for unknown reason"). Authentication, already-published and unknown failures still exit non-zero. Failed-publish classification is consolidated through a single `classify_failure` function and `FailureKind` enum (with an `is_deferred` predicate), covered by unit tests runnable via `rust-script --test scripts/publish-crate.rs`.
- The release workflow (`.github/workflows/release.yml`) now gates crate-availability waiting, Docker Hub publishing and GitHub release creation on either an already-published crate or `publish_result=success`, so a deferred (rate-limited) crate upload no longer produces partial downstream release artifacts and the same version is retried automatically on the next push to `main`.

### Fixed
- Fixed reversed `cancel-in-progress` concurrency condition in `release.yml` that cancelled in-flight releases on `main` and never superseded older PR runs. The condition now uses `!=` so `main` releases run to completion while newer PR pushes cancel stale runs.

### Added
- Added a `scripts/check-crate-size.rs` guard that builds the `.crate` archive and fails the release before publishing when it exceeds the crates.io 10 MiB upload limit. The check runs in the build job and before publishing in both the auto-release and manual-release jobs.

### Changed
- Added a narrow `include` allowlist to `Cargo.toml` so docs, case studies, generated CI artifacts, changelog fragments, scripts, and experiments no longer inflate the published release archive.

### Fixed
- Prevented GitHub release creation from treating generic API validation failures as existing releases, and capped oversized release notes with a link to the full tagged changelog.

### Added
- Added the initial `meta-language` links-network core with self-description roots,
  verification, trivia attachment configuration, and a minimal CLI.
- Added the default lossless `parse` entry point, projection views, and a
  tested parity roadmap for competitor features, grammar embedding, and language
  coverage targets.
- Added exact reconstruction, mixed-region detection, query matching,
  substitution rules, concept reconstruction, object identity helpers,
  many-valued truth values, and executable parity fixtures for every tracked
  competitor target.
- Added executable language fixtures and tests for every requested Markdown,
  HTML, top-ten programming-language, and top-ten natural-language target.

### Fixed
- Added a CI guard that rejects Rust test modules and test attributes under `src/`, keeping tests in the `tests/` tree.

## Minor Changes

- Add immutable and mutable network snapshots with provenance and forward
  version commits for roadmap snapshot/versioning coverage.

### Added
- Added `txt` as a first-class markup/container target with a UTF-8 lossless
  fixture and content-sniffing fallback regions.

### Fixed
- Reconciled the natural-language target ordering with the Ethnologue 2025
  total-speaker order cited by the parity roadmap.

### Added
- Added a grammar-backed parser front end for Python, C, Java, C++, C#, JavaScript, and R using official tree-sitter grammar crates.

### Added
- Parse supported mixed-mode embedded regions into the host links network with grammar-backed syntax links.

### Added
- Added natural-language segmentation, identification, normalization, and bidi annotation links over lossless text parses.

### Added
- Added grammar-backed Visual Basic parsing with byte-exact reconstruction and recovery flag coverage.

bump: minor

- Add a grammar-backed `sql-ansi` SQL-family dialect fixture using
  `tree-sitter-sequel`.

### Added
- Added grammar-backed Delphi/Object Pascal parsing through `tree-sitter-pascal`.

### Added
- Enriched link queries with S-expression structural matching, captures, by-type construction, host predicate hooks, and link-cli-style variable substitution bindings.

### Added
- Added persistent snapshot structural sharing with interned metadata text storage.

### Added
- Added a query-transform surface for selecting captured links and replacing
  their source text while preserving unchanged bytes.

### Added
- Materialized self-description roots as controlled links with complete root-definition closure and round-trippable `describe` output.

### Added
- Added common concept ontology seeding from meta-expression's semantic lexicon, with shared concept links, syntax mappings, and structural programming-language concepts.

### Added
- Added semantic cross-language reconstruction for the Hawaii statehood fixture,
  including English/Russian naturalization and configurable formalization levels.

### Added
- Expanded `PARITY_FIXTURES` with multiple provenance-tracked fixtures for tree-sitter, LibCST, Recast, jscodeshift, Rowan, cstree, and Roslyn, including executable recovery and query/transform expectations.

### Added

- Expanded `PARITY_FIXTURES` with upstream-provenanced internal ecosystem corpora for links-notation, link-cli, lino-objects-codec, relative-meta-logic, formal-ai, and meta-expression.

### Added

- Added structural LiNo parsing for links-notation doublets, triplets, named links, indented IDs, and self-references while preserving byte-exact reconstruction.

### Added
- Added fixed-point probabilistic truth values for relative-meta-logic-style
  semantic confidence evaluation.

### Added
- Added grammar-backed Go parsing via the official `tree-sitter-go` grammar
  (accepts `Go`, `go`, and `golang`) so `LinkNetwork::parse` emits real
  `LinkType::Syntax` concrete-syntax links with byte-exact reconstruction.

### Added
- `AccessMode { Mutable, ReadOnly }` setting on `ParseConfiguration`
  (`with_access_mode` / `access_mode`), defaulting to `Mutable` so existing
  callers are unaffected.
- `LinkNetwork::freeze` / `as_read_only` yielding a `ReadOnlyNetwork` view that
  exposes only `&self` operations (query, project, reconstruct, verify,
  serialize); mutators are unreachable at compile time because the view never
  hands out `&mut LinkNetwork`.
- `LinkNetwork::parse_engine`, returning an `EngineNetwork` handle that honours
  the configured access mode: read-only parsing returns the frozen form and
  `EngineNetwork::as_mutable` rejects mutation with a `ReadOnlyViolation`
  diagnostic.
- Snapshot interop: `NetworkSnapshot::as_read_only` / `from_read_only` reuse the
  snapshot's `Arc<LinkNetwork>`, so the frozen form composes with snapshot
  versioning instead of duplicating it.

### Added
- Added grammar-backed Ruby parsing through `tree-sitter-ruby`, so
  `LinkNetwork::parse(source, "Ruby", ...)` (and the `rb` alias) now emits real
  `LinkType::Syntax` links instead of falling back to lossless plain text.

bump: minor

- Add a grammar-backed `TypeScript` front end using `tree-sitter-typescript`,
  wiring the `typescript`/`ts` labels to `LANGUAGE_TYPESCRIPT` and the `tsx`
  label to `LANGUAGE_TSX` so `LinkNetwork::parse` emits real `LinkType::Syntax`
  links for TypeScript and TSX sources.

### Added
- Added grammar-backed parsing for seven data-exchange / interchange formats
  through `src/tree_sitter_adapter.rs`: JSON (`tree-sitter-json`), YAML
  (`tree-sitter-yaml`, accepts `yaml`/`yml`), TOML (`tree-sitter-toml-ng`), XML
  and DTD (`tree-sitter-xml`), INI (`tree-sitter-ini`), Protocol Buffers
  (`tree-sitter-proto`, accepts `protobuf`/`proto`/`Protocol Buffers`), and
  GraphQL (`tree-sitter-graphql`, accepts `graphql`/`gql`). Each parses into
  real `LinkType::Syntax` concrete-syntax links and reconstructs byte-for-byte.
- Added a `DATA_FORMAT_TARGETS` registry in `src/parity.rs` (with the new
  `LanguageFamily::DataFormat`) gated by parity tests, mirroring
  `MARKUP_LANGUAGE_TARGETS`.
- Added per-format UTF-8 `LANGUAGE_FIXTURES` round-trip entries plus
  recovery-case and mixed-region tests (a `json` fence embedded in Markdown
  parses into the host links network).
- Added lossless CSV and JSON5 parsers for the two formats whose published
  tree-sitter crates still pin the incompatible `tree-sitter ~0.20` runtime.
  CSV is validated with the Rust `csv` crate and JSON5 is validated with
  `json5_nodes`; both emit structured syntax links and reconstruct
  byte-for-byte.

### Documentation
- Documented the nine wired data-format parsers (parser, version, license, root
  node) in `docs/parity-roadmap.md`, including the tree-sitter compatibility
  rationale for the in-repo CSV and JSON5 parsers.

### Added
- `LinkNetwork::to_lino` serializes an entire links network to canonical
  links-notation text, keyed by each link's numeric id (doublets-style id
  discipline), covering references, names, types, terms, definitions,
  languages, source spans, parse flags, and term registration.
- `LinkNetwork::from_lino` reconstructs the exact network from that text,
  forming a lossless round-trip (`from_lino(to_lino(n))` is isomorphic to `n`).
- `LinoSerializationError` reports parse and schema failures from `from_lino`.
- A round-trip property test over every language fixture plus synthetic
  networks, and a test that `to_lino` output is accepted by the
  `links-notation` 0.13 crate parser.
- `ParityCapability::LinoSerialization` and an output-side serialization
  parity fixture for the `links-notation` target.

### Changed
- Added the `links-notation` 0.13 crate as a dependency so serialized output
  aligns with the wider links-notation ecosystem.

### Added
- `ParserRegistry` maps language keys to `Arc<dyn LanguageParser>` with the
  built-in parser as a fallback. User registrations shadow the built-in
  dispatch for the same (case-insensitive) key; unregistered keys still route
  through the built-in set.
- `LinkNetwork::parse_with_registry` dispatches a parse through a registry.
- `examples/custom_parser_registry.rs` documents registering a custom parser.

### Added
- Added grammar-backed parsing for five popular programming languages
  immediately below the TIOBE top ten through `src/tree_sitter_adapter.rs`: PHP
  (`tree-sitter-php`, via its `LANGUAGE_PHP` symbol), Swift
  (`tree-sitter-swift`), Kotlin (`tree-sitter-kotlin-ng`, accepts `kotlin`/`kt`),
  Scala (`tree-sitter-scala`), and Lua (`tree-sitter-lua`). Each parses into real
  `LinkType::Syntax` concrete-syntax links and reconstructs byte-for-byte.
- Added a `SECOND_TIER_PROGRAMMING_LANGUAGE_TARGETS` registry in `src/parity.rs`
  gated by parity tests, mirroring `DATA_FORMAT_TARGETS`.
- Added per-language UTF-8 `LANGUAGE_FIXTURES` round-trip entries plus
  case-insensitive alias coverage and a per-language recovery fixture whose
  malformed source still reconstructs while exposing error/missing diagnostics.

### Documentation
- Documented the wired grammars (crate, version, license, root node) in
  `docs/parity-roadmap.md` and kept the temporary Perl follow-up tracked
  explicitly until `ts-parser-perl` was adopted.

### Added
- Add a Rust `ToLinks`/`FromLinks` codec with queryable type-shape links and shared/circular object graph round-trips through LiNo serialization.

### Added

- Added the `LinkStore` storage trait, read-only-aware storage wrappers, and an
  optional file-mapped `doublets` backend with LiNo/binary round-trip coverage.

### Added
- Added exact-match concept interning, language-bound expression links, external-id alias links, and LiNo concept-set import for the shared concept ontology.

### Added
- Added starter natural-language grammaticality parsing with UD-style morphosyntax links, pass/fail fixtures for the ten natural-language targets, and recoverable error links for ungrammatical fixtures.

### Added
- Added configurable `TranslationRuleSet` values, a runtime
  `TranslationRuleRegistry`, LiNo-backed rule-set loading, template
  placeholders, and missing-rule diagnostics for from-meta reconstruction.

### Added
- Added queryable language profiles with JavaScript transform enforcement and rule-set-derived profile domains.

### Added
- Added incremental source edit reparsing, stable outside-edit link IDs, and structural snapshot diff reporting.

### Added
- Added composable `LinkRule` query algebra with relational rules, boolean composition, named sub-rules, ellipsis gap matching, typed metavariables, plain-text token patterns, traversal strategies, and valid/invalid rule snapshot suites.
- Added quasiquote replacement templates with placeholder validation and parenthesization-conservative captured-text replacement.

### Added
- Add the API operation/style parity registry, fluent network pipeline, and link-cli-style substitution text runner.

### Added
- Wire the canonical `ts-parser-perl` tree-sitter grammar as a second-tier
  programming-language target without upgrading the project-wide tree-sitter
  runtime.

### Added
- Added source generation helpers for constructed syntax networks: `insert_source_token`, `insert_syntax_node`, `render_source`, `render_source_from`, and `render_source_from_document`.

### Added
- Added wave-two competitor and ecosystem parity fixtures with executable transform, reconstruction, grammar, and storage gates.
- Added a recorded `cargo llvm-cov` line-coverage floor to CI so coverage cannot silently regress.

### Added
- Shared, language-free document-formatting concept ontology (`document_formatting` module): inline concepts (emphasis, strong, strikethrough, inline-code, hyperlink, image, line-break) and block concepts (heading with level, paragraph, blockquote, bullet/ordered lists, list-item, code-block with language, thematic-break, table/row/cell), each seeded with Markdown and HTML syntax mappings.
- `seed_common_concept_ontology()` now also seeds the formatting concepts, and `ConceptOntologySeedReport::formatting_concepts()` reports how many were added.
- `LinkNetwork::resolve_document_format` / `render_document_format` / `translate_document_format` so the same concept link reconstructs as `**…**` in Markdown and `<strong>…</strong>` in HTML; Markdown `**bold**` and HTML `<strong>bold</strong>` reach the one shared `strong` concept under semantic projection.
- `FormattingDocument` concept layer with `parse_markup_document` and `translate_markup_document` for full Markdown ⇄ HTML document round-trips through one concept ontology (heading/paragraph/list/bold/italic/link).

### Added
- PDF document-format support (issue #84): a documented, uncompressed text PDF profile (`document_formatting::render_pdf_document` / `parse_pdf_document`) that renders a language-free `FormattingDocument` to a valid single-page PDF (correct `xref` offsets, object table, and stream `Length`) and parses it back into the same concept tree. Block role is carried by marked content (`/H1`…`/H6`, `/P`, `/UL`/`/OL`, `/LI`) and inline bold/italic by the selected font resource (`/F1` regular, `/F2` strong, `/F3` emphasis).
- `parse("…", "pdf", …)` dispatches to a new `pdf_parser` that builds a byte-exact lossless network (`reconstruct_text()` returns the input verbatim) and adds additive `Concept`/`Object` structure links recovering heading/paragraph/list/list-item and bold/italic.
- `reconstruct_text_as("PDF", …)` renders a structurally equivalent PDF through the shared formatting concept layer: a PDF source re-renders byte-for-byte, while a Markdown/HTML source is translated into an equivalent PDF, and `translate_markup_document` now bridges Markdown/HTML ⇄ PDF.
- `PDF` markup target in `MARKUP_LANGUAGE_TARGETS` with a bold + heading + paragraph round-trip `LANGUAGE_FIXTURES` entry, plus `docs/pdf-fidelity.md` documenting the round-trip fidelity matrix for supported and lossy/unsupported PDF features.

### Added
- DOCX (OOXML) document-format support (issue #85): a documented OOXML text profile (`document_formatting::render_docx_document` / `parse_docx_document`) that renders a language-free `FormattingDocument` to `word/document.xml` WordprocessingML and parses it back into the same concept tree. Block role is carried by paragraph properties (`<w:pStyle w:val="HeadingN"/>` headings, bare `<w:p>` paragraphs, `<w:numPr>` `numId` 1/2 bullet/ordered list items) and inline bold/italic by run properties (`<w:b/>` → `strong`, `<w:i/>` → `emphasis`).
- A binary OPC packaging layer (`document_formatting::render_docx_package` / `parse_docx_package`) that assembles a valid `.docx` ZIP (stored entries with a self-implemented CRC-32, no new dependencies) containing `[Content_Types].xml`, the relationship parts, `word/document.xml`, `word/styles.xml`, and `word/numbering.xml`, and reads `word/document.xml` back out.
- `parse("…", "docx", …)` dispatches to a new `docx_parser` that builds a byte-exact lossless network (`reconstruct_text()` returns the input verbatim) and adds additive `Concept`/`Object` structure links recovering heading/paragraph/list/list-item and bold/italic.
- `reconstruct_text_as("DOCX", …)` renders structurally equivalent OOXML through the shared formatting concept layer: a DOCX source re-renders byte-for-byte, while a Markdown/HTML/PDF source is translated into equivalent OOXML, and `translate_markup_document` now bridges Markdown/HTML/PDF ⇄ DOCX.
- `DOCX` markup target in `MARKUP_LANGUAGE_TARGETS` with a bold + heading + bullet-list round-trip `LANGUAGE_FIXTURES` entry, plus `docs/docx-fidelity.md` documenting the two-layer round-trip fidelity matrix for supported and lossy/unsupported OOXML features.

### Fixed
- Fixed the GitHub Pages site returning HTTP 404 at the root URL
  (<https://link-foundation.github.io/meta-language>). The deploy job uploaded
  raw `cargo doc` output, which has no root `index.html` (rustdoc only emits
  `target/doc/<crate>/index.html`), so the Pages root 404'd while docs were only
  reachable at `/meta_language/`. See issue #90.

### Added
- A project website with a landing page (description + docs links) under
  `docs/site/`, deployed at the Pages root.
- An interactive WebAssembly "Links Notation playground" demo in a new
  standalone `web/` crate (wraps the wasm-compatible `links-notation` crate),
  served at `/demo/`.
- `scripts/build-site.rs` to assemble the website (`_site/`) locally and in CI:
  landing page at `/`, demo at `/demo/`, and `rustdoc` under `/api/` with a root
  redirect.
- `docs/case-studies/issue-90` documenting the timeline, requirements, root
  cause analysis, and solution plans.

### Changed
- The `Deploy Website` CI job (formerly `Deploy Rust Documentation`) now builds
  the WebAssembly demo and assembles the full site instead of deploying
  `target/doc` directly.

### Added
- Cross-format document reconstruction and round-trip translation (issue #86): `reconstruct_text_as("txt" | "Markdown" | "HTML" | "PDF" | "DOCX", …)` now works over the shared, language-free formatting concept layer (issue #83), so a document parsed from any supported format reconstructs into any other when the source uses only concepts both formats support. A same-format target re-renders byte-for-byte; a cross-format target is translated through the concept tree, preserving heading/paragraph/list and bold/italic/link structure.
- `txt` joins Markdown, HTML, PDF, and DOCX as a first-class document format in `parse_markup_document` / `render_markup_document`: blank-line-separated paragraphs parse into the concept layer, and the concept layer flattens to plain text (headings to plain lines, lists to `- `/`N. ` markers, inline styling dropped) as the documented lossy fallback target.
- Per-format capability profiles (`document_format_profile`, `DOCUMENT_FORMATS`, `CROSS_FORMAT_CONCEPTS`, `canonical_document_format`) expose each format's `LanguageProfile` over the formatting concept ontology, reporting for every cross-format concept either native support or a documented lossy fallback rather than silent data loss.
- `LanguageProfile` gained `with_concept_fallback` / `concept_fallback` / `fallbacks` to declare and query the lossy fallback for concepts a target cannot represent natively.
- A round-trip matrix test covering every ordered pair of `{txt, Markdown, HTML, PDF, DOCX}` (a sample built from the concepts both formats share survives `A → concepts → B → concepts → A`), plus `docs/cross-format-fidelity.md` documenting the cross-format translation entry point and the per-format fidelity matrix.

### Added
- Case study and implementation backlog for grammar extensibility & inference (issue #93): `docs/case-studies/issue-93/` collects the requirement register, a grammar-inference literature review (Gold (1967), RPNI/EDSM, Sequitur, the GLADE→Arvada→TreeVada→Kedavra→NatGI line, ISLearn), a licence-vetted library/ecosystem survey, an existing-capability gap analysis, a competitive analysis pinning the metrics to beat, and per-epic solution plans with the issue dependency DAG. The 34 maximally-detailed sub-issue specs under `proposed-issues/` were filed as GitHub issues #95–#128, each attached as a native sub-issue of #93 with all 51 `blocked-by` dependency edges wired via the GitHub REST API. This is a planning/research deliverable only — no library code changes.

### Added
- Added a public grammar IR with expression builders and links round-trip encoding.

### Added
- Added grammar surface syntax parsing, writing, and LiNo bridge helpers for the grammar IR.

### Added
- Seed grammar-construct concepts and expose grammar expression concept alignment helpers.

### Added
- Added grammar subsystem documentation, README integration, and tests for the grammar docs page set and relative links.

### Added
- Added a public BNF grammar importer that lowers classic BNF productions into the grammar IR.

### Added
- Added an EBNF grammar importer that lowers ISO-style EBNF constructs into the grammar IR.

### Added
- Added an ABNF grammar importer with RFC 5234 core rules, RFC 7405 string sensitivity, numeric terminals, repetition, and incremental alternative lowering.

### Added
- Added a PEG `.pest` grammar importer backed by `pest_meta`.

### Added
- Add a tree-sitter `grammar.json` importer for lowering generated tree-sitter grammars into the grammar IR.

### Added
- Add a clean-room ANTLR v4 `.g4` importer for lowering grammar files into the grammar IR.

### Added
- Add clean-room Lark and GBNF grammar importers that lower into the grammar IR.

### Added
- Added public BNF, EBNF, and ABNF grammar emitters for the grammar IR.

### Added
- Add a public GBNF grammar emitter for LLM grammar-constraint interop.

### Added
- Added a public pest PEG grammar emitter for the grammar IR.

### Added
- Added Rust parser codegen artifacts for grammar IR, including pest derive stubs and AST type rendering.

### Added
- Added Peggy grammar and JavaScript parser module code generation for the grammar IR.

### Added
- Added a public tree-sitter `grammar.js` emitter for the grammar IR.

### Added

- Add state-merging regular inference with RPNI, EDSM, and ALERGIA.

### Added

- Add MDL/Occam minimization for inferred grammars.

### Added

- Add semantic constraint inference for grammar corpora

### Added
- Added concept-aligned grammar surface translation with deterministic rule-name, non-terminal, and doc-comment rewrites.

### Added
- Added a deterministic grammar inference evaluation harness with sampling, oracle scoring, MDL size metrics, and smoke corpus reports.

### Added
- Added deterministic lexical class inference with category-based tokenisation and grammar IR token rules.

### Added
- Added a Sequitur structural-compression inference pass that emits deterministic inferred grammar IR.

### Added
- Added a delimiter-skeleton structural prior API for positive grammar inference.

### Added
- Add deterministic CFG inference over delimiter structural priors with oracle-checked positive recall.

### Added
- Add deterministic and optional LLM-assisted grammar inference advisors for rule naming and merge ranking.

### Added
- Add a runtime `GrammarParser` and registry helpers for parsing with imported or inferred grammars.

### Added
- Add opt-in active regular-language learning with L*, DFA output, parser-backed oracles, and right-linear grammar lowering.

### Added
- Added CLI grammar subcommands for inference, import, emit, and concept-aligned translation.

### Added
- Add the E3 competitor corpus manifest, vendored benchmark fixtures, and a D1/D5 benchmark gate for included subjects.

### Added
- Added grammar-format fidelity profiles and the generated BNF round-trip matrix documentation.

### Added
- Added semantic grammar validation diagnostics for undefined references, left recursion, unreachable rules, nullable repetitions, duplicate rules, and unused captures.

### Added
- Added end-to-end grammar inference examples and integration tests for Rust, JavaScript, GBNF, and CLI emit pipelines.

### Added
- Added the `js/` JavaScript implementation package for the meta-language core.
- Added Rust/JavaScript parity manifest checks and separate JS/Rust workflows.

### Added
- Added JavaScript truth-value semantics parity for `TruthValue`,
  `Probability`, and `ProbabilisticTruthValue`.

### Added
- Added a module-level Rust/JavaScript parity gate: every `pub mod` in
  `rust/src/lib.rs` must now be classified in `parity/language-features.json`
  (`rustModules`) as either `ported` (naming an implemented feature row) or
  `rust-only` (with a justification). The JavaScript checker
  (`js/scripts/check-js-rust-parity.mjs`) and the Rust test
  (`rust/tests/unit/parity_manifest.rs`) both fail when the public Rust surface
  and the manifest drift apart, so a new Rust module can no longer slip in
  without an explicit JavaScript parity decision.
- Ported four previously Rust-only modules to JavaScript with full test
  coverage and parity manifest rows: read-only access (`ReadOnlyNetwork`,
  `EngineNetwork`, `AccessMode`), embedded-region detection (`EmbeddedRegion`,
  `detectEmbeddedRegions`, `RegionDetectionPolicy`), language profiles
  (`LanguageProfile`, `LanguageProfileLinks`, `LanguageProfileViolation`), and
  the link-rule query algebra (`LinkRule`, `LinkRuleRegistry`,
  `TraversalStrategy`, and the rule snapshot suite).
- Added `LinkMetadata.withDefinition`/`definition` to the JavaScript primitives
  for parity with the Rust `LinkMetadata` definition field, and recorded
  `parse-configuration` and `link-flags` feature rows that already existed in
  both languages but were previously untracked.

## [0.48.0] - 2026-06-22

### Added
- Changeset-style fragment format with frontmatter for specifying version bump type
- New `get-bump-type.mjs` script to automatically determine version bump from fragments
- Automatic version bumping on merge to main based on changelog fragments
- Detailed documentation for the changelog fragment system in `changelog.d/README.md`

### Changed
- Updated `collect-changelog.mjs` to strip frontmatter when collecting fragments
- Updated `version-and-commit.mjs` to handle frontmatter in fragments
- Enhanced release workflow to automatically determine bump type from changesets

### Changed
- Add `detect-changes` job with cross-platform `detect-code-changes.mjs` script
- Make lint job independent of changelog check (runs based on file changes only)
- Allow docs-only PRs without changelog fragment requirement
- Handle changelog check 'skipped' state in dependent jobs
- Exclude `changelog.d/`, `docs/`, `experiments/`, `examples/` folders and markdown files from code changes detection

### Fixed
- Fixed README.md to correctly reference Node.js scripts (`.mjs`) instead of Python scripts (`.py`)
- Updated project structure in README.md to match actual script files in `scripts/` directory
- Fixed example code in README.md that had invalid Rust with two `main` functions

### Added

- Added crates.io publishing support to CI/CD workflow
- Added `release_mode` input with "instant" and "changelog-pr" options for manual releases
- Added `--tag-prefix` and `--crates-io-url` options to create-github-release.mjs script
- Added comprehensive case study documentation for Issue #11 in docs/case-studies/issue-11/

### Changed

- Changed changelog fragment check from warning to error (exit 1) to enforce changelog requirements
- Updated job conditions with `always() && !cancelled()` to fix workflow_dispatch job skipping issue
- Renamed manual-release job to "Instant Release" for clarity

### Fixed

- Fixed deprecated `::set-output` GitHub Actions command in version-and-commit.mjs
- Fixed workflow_dispatch triggering issues where lint/build/release jobs were incorrectly skipped

### Fixed

- Fixed changelog fragment check to validate that a fragment is **added in the PR diff** rather than just checking if any fragments exist in the directory. This prevents the check from incorrectly passing when there are leftover fragments from previous PRs that haven't been released yet.

### Changed

- Converted shell scripts in `release.yml` to cross-platform `.mjs` scripts for improved portability and performance:
  - `check-changelog-fragment.mjs` - validates changelog fragment is added in PR diff
  - `git-config.mjs` - configures git user for CI/CD
  - `check-release-needed.mjs` - checks if release is needed
  - `publish-crate.mjs` - publishes package to crates.io
  - `create-changelog-fragment.mjs` - creates changelog fragments for manual releases
  - `get-version.mjs` - gets current version from Cargo.toml

### Added

- Added `check-version-modification.mjs` script to detect manual version changes in Cargo.toml
- Added `version-check` job to CI/CD workflow that runs on pull requests
- Added skip logic for automated release branches (changelog-manual-release-*, changeset-release/*, release/*, automated-release/*)

### Changed

- Version modifications in Cargo.toml are now blocked in pull requests to enforce automated release pipeline

### Added

- Added support for `CARGO_REGISTRY_TOKEN` as alternative to `CARGO_TOKEN` for crates.io publishing
- Added case study documentation for Issue #17 (yargs reserved word and dual token support)

### Changed

- Updated workflow to use fallback logic: `${{ secrets.CARGO_REGISTRY_TOKEN || secrets.CARGO_TOKEN }}`
- Improved publish-crate.mjs to check both `CARGO_REGISTRY_TOKEN` and `CARGO_TOKEN` environment variables
- Added warning message when neither token is set

### Added
- New `scripts/rust-paths.mjs` utility for automatic Rust package root detection
- Support for both single-language and multi-language repository structures in all CI/CD scripts
- Configuration options via `--rust-root` CLI argument and `RUST_ROOT` environment variable
- Comprehensive case study documentation in `docs/case-studies/issue-19/`

### Changed
- Updated all release scripts to use the new path detection utility:
  - `scripts/bump-version.mjs`
  - `scripts/check-release-needed.mjs`
  - `scripts/collect-changelog.mjs`
  - `scripts/get-bump-type.mjs`
  - `scripts/get-version.mjs`
  - `scripts/publish-crate.mjs`
  - `scripts/version-and-commit.mjs`

### Changed

- **check-release-needed.mjs**: Now checks crates.io API directly instead of git tags to determine if a version is already released. This prevents false positives where git tags exist but the package was never actually published to crates.io.

### Added

- **CI/CD Troubleshooting Guide**: New documentation at `docs/ci-cd/troubleshooting.md` covering common issues like skipped jobs, false positive version checks, publishing failures, and secret configuration.

- **Enhanced Error Handling in publish-crate.mjs**: Added specific detection and helpful error messages for authentication failures, including guidance on secret configuration and workflow setup.

- **Case Study Documentation**: Added comprehensive case study at `docs/case-studies/issue-21/` analyzing CI/CD failures from browser-commander repository (issues #27, #29, #31, #33) with timeline, root causes, and lessons learned.

### Fixed

- **Prevent False Positive Version Checks**: The release workflow now correctly identifies unpublished versions by checking crates.io instead of relying on git tags, which can exist without the package being published.

### Changed

- Translated all CI/CD scripts from JavaScript (.mjs) to Rust (.rs) using rust-script
- Scripts now use native Rust with rust-script for execution in shell
- Removed Node.js dependency from CI/CD pipeline
- Updated GitHub Actions workflow to use rust-script instead of node
- Updated README and CONTRIBUTING documentation with new script references

### Added

- Cache `restore-keys` for partial cache hits across all workflow jobs
- Explicit `token` parameter in checkout for release jobs
- Code coverage job with `cargo-llvm-cov` and Codecov integration
- Codecov badge in README.md
- Pre-release version support (e.g., `0.1.0-beta.1`) in version parsing
- `--release-label` parameter for multi-language release disambiguation
- `ensure_version_exceeds_published()` logic to prevent publishing duplicate versions
- `get_max_published_version()` to query highest non-yanked version from crates.io
- `max_published_version` output from check-release-needed for downstream use
- Version fallback logic in auto-release Create GitHub Release step

### Changed

- Updated `actions/checkout` from v4 to v6
- Updated `actions/cache` from v4 to v5
- Updated `peter-evans/create-pull-request` from v7 to v8
- Made `publish-crate.rs` fail (exit 1) when version already exists on crates.io
- Improved `create-github-release.rs` to check combined stdout+stderr and detect "Validation Failed"

### Fixed

- Fix publish steps overriding workflow-level CARGO_TOKEN fallback, breaking CARGO_REGISTRY_TOKEN-only configurations (#32)
- Fix non-fast-forward push failures in multi-workflow repos by adding fetch/rebase and push retry logic (#31)
- Add mono-repo path support to check-changelog-fragment.rs, check-version-modification.rs, and create-changelog-fragment.rs
- Add `!cancelled()` guard to test job condition to respect workflow cancellation

### Fixed

- Fixed `version-and-commit.rs` to check crates.io instead of git tags for determining if a version is already released
- This prevents the release pipeline from getting stuck when git tags exist without corresponding crates.io publication

### Added

- Added `--tag-prefix` support to `version-and-commit.rs` for multi-language repository compatibility
- Added crates.io and docs.rs badges to README.md
- Added automatic crates.io and docs.rs badge injection in GitHub release notes
- Added documentation deployment job to CI/CD pipeline (deploys to GitHub Pages after release)
- Added case study documentation for issue #25

### Fixed
- Fixed unsupported look-ahead regex in `create-github-release.rs` that caused a panic when parsing CHANGELOG.md. Replaced with a two-step approach using only features supported by Rust's `regex` crate.

### Changed
- Restructured example application as a simple CLI sum calculator using `lino-arguments`
- Renamed default package to `example-sum-package-name` with Unlicense license
- Reorganized test structure: `tests/unit/sum.rs`, `tests/integration/sum.rs`, `tests/unit/ci-cd/`
- Converted experiment scripts into proper unit tests in `tests/unit/ci-cd/changelog_parsing.rs`
- Added CI/CD skip logic for template default package name `example-sum-package-name`
- Updated README.md badges and documentation

### Fixed

- Change detection script now uses per-commit diff instead of full PR diff, so commits touching only non-code files correctly skip CI jobs even when earlier commits in the same PR changed code files

### Fixed
- Make release scripts resolve the publishable crate manifest when the repository root uses a Cargo workspace manifest.

### Fixed
- Decoupled GitHub Pages documentation deployment from package release publication and fixed release-script warning failures under `RUSTFLAGS=-Dwarnings`.

### Changed
- Added explicit GitHub Actions job timeouts and documented Rust test timeout guidance.

### Fixed
- Added a non-blocking warning threshold to the Rust file-size check so near-limit files are surfaced before concurrent PR merges can exceed the hard limit.

### Fixed
- Made `create-github-release.rs` build GitHub release titles as `[Language] X.Y.Z` instead of reusing the tag prefix.

### Added
- Added optional Docker Hub image publishing tied to Rust crate releases, including crates.io visibility waiting, version/latest image tags, and Docker Hub badges in GitHub release notes.

### Changed
- Release completeness checks now self-heal when crates.io exists but configured Docker Hub or GitHub release artifacts are missing.

### Fixed
- Switched documentation deployment to the official GitHub Pages artifact workflow so repositories using GitHub Actions as their Pages source do not get false-positive branch-push deploys.

### Added
- Documented the one-time `Settings → Pages → Source = GitHub Actions` prerequisite for the `deploy-docs` job in `README.md` and as a comment above the `deploy-docs` job in `release.yml`, so downstream template users hit a documented setup step instead of a `Get Pages site failed` error on the first deploy.

### Added
- Tracking case study at `docs/case-studies/issue-52/` registering the `browser-commander` + Playwright preview-regeneration pattern from [`konard/vk-bot-desktop#52`](https://github.com/konard/vk-bot-desktop/pull/52), with an activation checklist for when an example-app surface lands in this template. Documentation only — no workflow, script, or runtime code changes. Primary upstream tracking issue: [`link-foundation/js-ai-driven-development-pipeline-template#62`](https://github.com/link-foundation/js-ai-driven-development-pipeline-template/issues/62).

### Fixed
- Release automation now keeps the workspace package entry in `Cargo.lock` synchronized when `scripts/version-and-commit.rs` bumps `Cargo.toml`, preventing stale lock-file version diffs in later pull requests.

### Fixed
- `scripts/publish-crate.rs` now treats crates.io HTTP 429 throttle responses ("You have published too many versions of this crate in the last 24 hours") as a deferred `publish_result=rate_limited` outcome (it writes the output, prints an explanatory banner and exits successfully) instead of a hard CI failure reported as a generic `failed` ("Failed to publish for unknown reason"). Authentication, already-published and unknown failures still exit non-zero. Failed-publish classification is consolidated through a single `classify_failure` function and `FailureKind` enum (with an `is_deferred` predicate), covered by unit tests runnable via `rust-script --test scripts/publish-crate.rs`.
- The release workflow (`.github/workflows/release.yml`) now gates crate-availability waiting, Docker Hub publishing and GitHub release creation on either an already-published crate or `publish_result=success`, so a deferred (rate-limited) crate upload no longer produces partial downstream release artifacts and the same version is retried automatically on the next push to `main`.

### Fixed
- Fixed reversed `cancel-in-progress` concurrency condition in `release.yml` that cancelled in-flight releases on `main` and never superseded older PR runs. The condition now uses `!=` so `main` releases run to completion while newer PR pushes cancel stale runs.

### Added
- Added a `scripts/check-crate-size.rs` guard that builds the `.crate` archive and fails the release before publishing when it exceeds the crates.io 10 MiB upload limit. The check runs in the build job and before publishing in both the auto-release and manual-release jobs.

### Changed
- Added a narrow `include` allowlist to `Cargo.toml` so docs, case studies, generated CI artifacts, changelog fragments, scripts, and experiments no longer inflate the published release archive.

### Fixed
- Prevented GitHub release creation from treating generic API validation failures as existing releases, and capped oversized release notes with a link to the full tagged changelog.

### Added
- Added the initial `meta-language` links-network core with self-description roots,
  verification, trivia attachment configuration, and a minimal CLI.
- Added the default lossless `parse` entry point, projection views, and a
  tested parity roadmap for competitor features, grammar embedding, and language
  coverage targets.
- Added exact reconstruction, mixed-region detection, query matching,
  substitution rules, concept reconstruction, object identity helpers,
  many-valued truth values, and executable parity fixtures for every tracked
  competitor target.
- Added executable language fixtures and tests for every requested Markdown,
  HTML, top-ten programming-language, and top-ten natural-language target.

### Fixed
- Added a CI guard that rejects Rust test modules and test attributes under `src/`, keeping tests in the `tests/` tree.

## Minor Changes

- Add immutable and mutable network snapshots with provenance and forward
  version commits for roadmap snapshot/versioning coverage.

### Added
- Added `txt` as a first-class markup/container target with a UTF-8 lossless
  fixture and content-sniffing fallback regions.

### Fixed
- Reconciled the natural-language target ordering with the Ethnologue 2025
  total-speaker order cited by the parity roadmap.

### Added
- Added a grammar-backed parser front end for Python, C, Java, C++, C#, JavaScript, and R using official tree-sitter grammar crates.

### Added
- Parse supported mixed-mode embedded regions into the host links network with grammar-backed syntax links.

### Added
- Added natural-language segmentation, identification, normalization, and bidi annotation links over lossless text parses.

### Added
- Added grammar-backed Visual Basic parsing with byte-exact reconstruction and recovery flag coverage.

bump: minor

- Add a grammar-backed `sql-ansi` SQL-family dialect fixture using
  `tree-sitter-sequel`.

### Added
- Added grammar-backed Delphi/Object Pascal parsing through `tree-sitter-pascal`.

### Added
- Enriched link queries with S-expression structural matching, captures, by-type construction, host predicate hooks, and link-cli-style variable substitution bindings.

### Added
- Added persistent snapshot structural sharing with interned metadata text storage.

### Added
- Added a query-transform surface for selecting captured links and replacing
  their source text while preserving unchanged bytes.

### Added
- Materialized self-description roots as controlled links with complete root-definition closure and round-trippable `describe` output.

### Added
- Added common concept ontology seeding from meta-expression's semantic lexicon, with shared concept links, syntax mappings, and structural programming-language concepts.

### Added
- Added semantic cross-language reconstruction for the Hawaii statehood fixture,
  including English/Russian naturalization and configurable formalization levels.

### Added
- Expanded `PARITY_FIXTURES` with multiple provenance-tracked fixtures for tree-sitter, LibCST, Recast, jscodeshift, Rowan, cstree, and Roslyn, including executable recovery and query/transform expectations.

### Added

- Expanded `PARITY_FIXTURES` with upstream-provenanced internal ecosystem corpora for links-notation, link-cli, lino-objects-codec, relative-meta-logic, formal-ai, and meta-expression.

### Added

- Added structural LiNo parsing for links-notation doublets, triplets, named links, indented IDs, and self-references while preserving byte-exact reconstruction.

### Added
- Added fixed-point probabilistic truth values for relative-meta-logic-style
  semantic confidence evaluation.

### Added
- Added grammar-backed Go parsing via the official `tree-sitter-go` grammar
  (accepts `Go`, `go`, and `golang`) so `LinkNetwork::parse` emits real
  `LinkType::Syntax` concrete-syntax links with byte-exact reconstruction.

### Added
- `AccessMode { Mutable, ReadOnly }` setting on `ParseConfiguration`
  (`with_access_mode` / `access_mode`), defaulting to `Mutable` so existing
  callers are unaffected.
- `LinkNetwork::freeze` / `as_read_only` yielding a `ReadOnlyNetwork` view that
  exposes only `&self` operations (query, project, reconstruct, verify,
  serialize); mutators are unreachable at compile time because the view never
  hands out `&mut LinkNetwork`.
- `LinkNetwork::parse_engine`, returning an `EngineNetwork` handle that honours
  the configured access mode: read-only parsing returns the frozen form and
  `EngineNetwork::as_mutable` rejects mutation with a `ReadOnlyViolation`
  diagnostic.
- Snapshot interop: `NetworkSnapshot::as_read_only` / `from_read_only` reuse the
  snapshot's `Arc<LinkNetwork>`, so the frozen form composes with snapshot
  versioning instead of duplicating it.

### Added
- Added grammar-backed Ruby parsing through `tree-sitter-ruby`, so
  `LinkNetwork::parse(source, "Ruby", ...)` (and the `rb` alias) now emits real
  `LinkType::Syntax` links instead of falling back to lossless plain text.

bump: minor

- Add a grammar-backed `TypeScript` front end using `tree-sitter-typescript`,
  wiring the `typescript`/`ts` labels to `LANGUAGE_TYPESCRIPT` and the `tsx`
  label to `LANGUAGE_TSX` so `LinkNetwork::parse` emits real `LinkType::Syntax`
  links for TypeScript and TSX sources.

### Added
- Added grammar-backed parsing for seven data-exchange / interchange formats
  through `src/tree_sitter_adapter.rs`: JSON (`tree-sitter-json`), YAML
  (`tree-sitter-yaml`, accepts `yaml`/`yml`), TOML (`tree-sitter-toml-ng`), XML
  and DTD (`tree-sitter-xml`), INI (`tree-sitter-ini`), Protocol Buffers
  (`tree-sitter-proto`, accepts `protobuf`/`proto`/`Protocol Buffers`), and
  GraphQL (`tree-sitter-graphql`, accepts `graphql`/`gql`). Each parses into
  real `LinkType::Syntax` concrete-syntax links and reconstructs byte-for-byte.
- Added a `DATA_FORMAT_TARGETS` registry in `src/parity.rs` (with the new
  `LanguageFamily::DataFormat`) gated by parity tests, mirroring
  `MARKUP_LANGUAGE_TARGETS`.
- Added per-format UTF-8 `LANGUAGE_FIXTURES` round-trip entries plus
  recovery-case and mixed-region tests (a `json` fence embedded in Markdown
  parses into the host links network).
- Added lossless CSV and JSON5 parsers for the two formats whose published
  tree-sitter crates still pin the incompatible `tree-sitter ~0.20` runtime.
  CSV is validated with the Rust `csv` crate and JSON5 is validated with
  `json5_nodes`; both emit structured syntax links and reconstruct
  byte-for-byte.

### Documentation
- Documented the nine wired data-format parsers (parser, version, license, root
  node) in `docs/parity-roadmap.md`, including the tree-sitter compatibility
  rationale for the in-repo CSV and JSON5 parsers.

### Added
- `LinkNetwork::to_lino` serializes an entire links network to canonical
  links-notation text, keyed by each link's numeric id (doublets-style id
  discipline), covering references, names, types, terms, definitions,
  languages, source spans, parse flags, and term registration.
- `LinkNetwork::from_lino` reconstructs the exact network from that text,
  forming a lossless round-trip (`from_lino(to_lino(n))` is isomorphic to `n`).
- `LinoSerializationError` reports parse and schema failures from `from_lino`.
- A round-trip property test over every language fixture plus synthetic
  networks, and a test that `to_lino` output is accepted by the
  `links-notation` 0.13 crate parser.
- `ParityCapability::LinoSerialization` and an output-side serialization
  parity fixture for the `links-notation` target.

### Changed
- Added the `links-notation` 0.13 crate as a dependency so serialized output
  aligns with the wider links-notation ecosystem.

### Added
- `ParserRegistry` maps language keys to `Arc<dyn LanguageParser>` with the
  built-in parser as a fallback. User registrations shadow the built-in
  dispatch for the same (case-insensitive) key; unregistered keys still route
  through the built-in set.
- `LinkNetwork::parse_with_registry` dispatches a parse through a registry.
- `examples/custom_parser_registry.rs` documents registering a custom parser.

### Added
- Added grammar-backed parsing for five popular programming languages
  immediately below the TIOBE top ten through `src/tree_sitter_adapter.rs`: PHP
  (`tree-sitter-php`, via its `LANGUAGE_PHP` symbol), Swift
  (`tree-sitter-swift`), Kotlin (`tree-sitter-kotlin-ng`, accepts `kotlin`/`kt`),
  Scala (`tree-sitter-scala`), and Lua (`tree-sitter-lua`). Each parses into real
  `LinkType::Syntax` concrete-syntax links and reconstructs byte-for-byte.
- Added a `SECOND_TIER_PROGRAMMING_LANGUAGE_TARGETS` registry in `src/parity.rs`
  gated by parity tests, mirroring `DATA_FORMAT_TARGETS`.
- Added per-language UTF-8 `LANGUAGE_FIXTURES` round-trip entries plus
  case-insensitive alias coverage and a per-language recovery fixture whose
  malformed source still reconstructs while exposing error/missing diagnostics.

### Documentation
- Documented the wired grammars (crate, version, license, root node) in
  `docs/parity-roadmap.md` and kept the temporary Perl follow-up tracked
  explicitly until `ts-parser-perl` was adopted.

### Added
- Add a Rust `ToLinks`/`FromLinks` codec with queryable type-shape links and shared/circular object graph round-trips through LiNo serialization.

### Added

- Added the `LinkStore` storage trait, read-only-aware storage wrappers, and an
  optional file-mapped `doublets` backend with LiNo/binary round-trip coverage.

### Added
- Added exact-match concept interning, language-bound expression links, external-id alias links, and LiNo concept-set import for the shared concept ontology.

### Added
- Added starter natural-language grammaticality parsing with UD-style morphosyntax links, pass/fail fixtures for the ten natural-language targets, and recoverable error links for ungrammatical fixtures.

### Added
- Added configurable `TranslationRuleSet` values, a runtime
  `TranslationRuleRegistry`, LiNo-backed rule-set loading, template
  placeholders, and missing-rule diagnostics for from-meta reconstruction.

### Added
- Added queryable language profiles with JavaScript transform enforcement and rule-set-derived profile domains.

### Added
- Added incremental source edit reparsing, stable outside-edit link IDs, and structural snapshot diff reporting.

### Added
- Added composable `LinkRule` query algebra with relational rules, boolean composition, named sub-rules, ellipsis gap matching, typed metavariables, plain-text token patterns, traversal strategies, and valid/invalid rule snapshot suites.
- Added quasiquote replacement templates with placeholder validation and parenthesization-conservative captured-text replacement.

### Added
- Add the API operation/style parity registry, fluent network pipeline, and link-cli-style substitution text runner.

### Added
- Wire the canonical `ts-parser-perl` tree-sitter grammar as a second-tier
  programming-language target without upgrading the project-wide tree-sitter
  runtime.

### Added
- Added source generation helpers for constructed syntax networks: `insert_source_token`, `insert_syntax_node`, `render_source`, `render_source_from`, and `render_source_from_document`.

### Added
- Added wave-two competitor and ecosystem parity fixtures with executable transform, reconstruction, grammar, and storage gates.
- Added a recorded `cargo llvm-cov` line-coverage floor to CI so coverage cannot silently regress.

### Added
- Shared, language-free document-formatting concept ontology (`document_formatting` module): inline concepts (emphasis, strong, strikethrough, inline-code, hyperlink, image, line-break) and block concepts (heading with level, paragraph, blockquote, bullet/ordered lists, list-item, code-block with language, thematic-break, table/row/cell), each seeded with Markdown and HTML syntax mappings.
- `seed_common_concept_ontology()` now also seeds the formatting concepts, and `ConceptOntologySeedReport::formatting_concepts()` reports how many were added.
- `LinkNetwork::resolve_document_format` / `render_document_format` / `translate_document_format` so the same concept link reconstructs as `**…**` in Markdown and `<strong>…</strong>` in HTML; Markdown `**bold**` and HTML `<strong>bold</strong>` reach the one shared `strong` concept under semantic projection.
- `FormattingDocument` concept layer with `parse_markup_document` and `translate_markup_document` for full Markdown ⇄ HTML document round-trips through one concept ontology (heading/paragraph/list/bold/italic/link).

### Added
- PDF document-format support (issue #84): a documented, uncompressed text PDF profile (`document_formatting::render_pdf_document` / `parse_pdf_document`) that renders a language-free `FormattingDocument` to a valid single-page PDF (correct `xref` offsets, object table, and stream `Length`) and parses it back into the same concept tree. Block role is carried by marked content (`/H1`…`/H6`, `/P`, `/UL`/`/OL`, `/LI`) and inline bold/italic by the selected font resource (`/F1` regular, `/F2` strong, `/F3` emphasis).
- `parse("…", "pdf", …)` dispatches to a new `pdf_parser` that builds a byte-exact lossless network (`reconstruct_text()` returns the input verbatim) and adds additive `Concept`/`Object` structure links recovering heading/paragraph/list/list-item and bold/italic.
- `reconstruct_text_as("PDF", …)` renders a structurally equivalent PDF through the shared formatting concept layer: a PDF source re-renders byte-for-byte, while a Markdown/HTML source is translated into an equivalent PDF, and `translate_markup_document` now bridges Markdown/HTML ⇄ PDF.
- `PDF` markup target in `MARKUP_LANGUAGE_TARGETS` with a bold + heading + paragraph round-trip `LANGUAGE_FIXTURES` entry, plus `docs/pdf-fidelity.md` documenting the round-trip fidelity matrix for supported and lossy/unsupported PDF features.

### Added
- DOCX (OOXML) document-format support (issue #85): a documented OOXML text profile (`document_formatting::render_docx_document` / `parse_docx_document`) that renders a language-free `FormattingDocument` to `word/document.xml` WordprocessingML and parses it back into the same concept tree. Block role is carried by paragraph properties (`<w:pStyle w:val="HeadingN"/>` headings, bare `<w:p>` paragraphs, `<w:numPr>` `numId` 1/2 bullet/ordered list items) and inline bold/italic by run properties (`<w:b/>` → `strong`, `<w:i/>` → `emphasis`).
- A binary OPC packaging layer (`document_formatting::render_docx_package` / `parse_docx_package`) that assembles a valid `.docx` ZIP (stored entries with a self-implemented CRC-32, no new dependencies) containing `[Content_Types].xml`, the relationship parts, `word/document.xml`, `word/styles.xml`, and `word/numbering.xml`, and reads `word/document.xml` back out.
- `parse("…", "docx", …)` dispatches to a new `docx_parser` that builds a byte-exact lossless network (`reconstruct_text()` returns the input verbatim) and adds additive `Concept`/`Object` structure links recovering heading/paragraph/list/list-item and bold/italic.
- `reconstruct_text_as("DOCX", …)` renders structurally equivalent OOXML through the shared formatting concept layer: a DOCX source re-renders byte-for-byte, while a Markdown/HTML/PDF source is translated into equivalent OOXML, and `translate_markup_document` now bridges Markdown/HTML/PDF ⇄ DOCX.
- `DOCX` markup target in `MARKUP_LANGUAGE_TARGETS` with a bold + heading + bullet-list round-trip `LANGUAGE_FIXTURES` entry, plus `docs/docx-fidelity.md` documenting the two-layer round-trip fidelity matrix for supported and lossy/unsupported OOXML features.

### Fixed
- Fixed the GitHub Pages site returning HTTP 404 at the root URL
  (<https://link-foundation.github.io/meta-language>). The deploy job uploaded
  raw `cargo doc` output, which has no root `index.html` (rustdoc only emits
  `target/doc/<crate>/index.html`), so the Pages root 404'd while docs were only
  reachable at `/meta_language/`. See issue #90.

### Added
- A project website with a landing page (description + docs links) under
  `docs/site/`, deployed at the Pages root.
- An interactive WebAssembly "Links Notation playground" demo in a new
  standalone `web/` crate (wraps the wasm-compatible `links-notation` crate),
  served at `/demo/`.
- `scripts/build-site.rs` to assemble the website (`_site/`) locally and in CI:
  landing page at `/`, demo at `/demo/`, and `rustdoc` under `/api/` with a root
  redirect.
- `docs/case-studies/issue-90` documenting the timeline, requirements, root
  cause analysis, and solution plans.

### Changed
- The `Deploy Website` CI job (formerly `Deploy Rust Documentation`) now builds
  the WebAssembly demo and assembles the full site instead of deploying
  `target/doc` directly.

### Added
- Cross-format document reconstruction and round-trip translation (issue #86): `reconstruct_text_as("txt" | "Markdown" | "HTML" | "PDF" | "DOCX", …)` now works over the shared, language-free formatting concept layer (issue #83), so a document parsed from any supported format reconstructs into any other when the source uses only concepts both formats support. A same-format target re-renders byte-for-byte; a cross-format target is translated through the concept tree, preserving heading/paragraph/list and bold/italic/link structure.
- `txt` joins Markdown, HTML, PDF, and DOCX as a first-class document format in `parse_markup_document` / `render_markup_document`: blank-line-separated paragraphs parse into the concept layer, and the concept layer flattens to plain text (headings to plain lines, lists to `- `/`N. ` markers, inline styling dropped) as the documented lossy fallback target.
- Per-format capability profiles (`document_format_profile`, `DOCUMENT_FORMATS`, `CROSS_FORMAT_CONCEPTS`, `canonical_document_format`) expose each format's `LanguageProfile` over the formatting concept ontology, reporting for every cross-format concept either native support or a documented lossy fallback rather than silent data loss.
- `LanguageProfile` gained `with_concept_fallback` / `concept_fallback` / `fallbacks` to declare and query the lossy fallback for concepts a target cannot represent natively.
- A round-trip matrix test covering every ordered pair of `{txt, Markdown, HTML, PDF, DOCX}` (a sample built from the concepts both formats share survives `A → concepts → B → concepts → A`), plus `docs/cross-format-fidelity.md` documenting the cross-format translation entry point and the per-format fidelity matrix.

### Added
- Case study and implementation backlog for grammar extensibility & inference (issue #93): `docs/case-studies/issue-93/` collects the requirement register, a grammar-inference literature review (Gold (1967), RPNI/EDSM, Sequitur, the GLADE→Arvada→TreeVada→Kedavra→NatGI line, ISLearn), a licence-vetted library/ecosystem survey, an existing-capability gap analysis, a competitive analysis pinning the metrics to beat, and per-epic solution plans with the issue dependency DAG. The 34 maximally-detailed sub-issue specs under `proposed-issues/` were filed as GitHub issues #95–#128, each attached as a native sub-issue of #93 with all 51 `blocked-by` dependency edges wired via the GitHub REST API. This is a planning/research deliverable only — no library code changes.

### Added
- Added a public grammar IR with expression builders and links round-trip encoding.

### Added
- Added grammar surface syntax parsing, writing, and LiNo bridge helpers for the grammar IR.

### Added
- Seed grammar-construct concepts and expose grammar expression concept alignment helpers.

### Added
- Added grammar subsystem documentation, README integration, and tests for the grammar docs page set and relative links.

### Added
- Added a public BNF grammar importer that lowers classic BNF productions into the grammar IR.

### Added
- Added an EBNF grammar importer that lowers ISO-style EBNF constructs into the grammar IR.

### Added
- Added an ABNF grammar importer with RFC 5234 core rules, RFC 7405 string sensitivity, numeric terminals, repetition, and incremental alternative lowering.

### Added
- Added a PEG `.pest` grammar importer backed by `pest_meta`.

### Added
- Add a tree-sitter `grammar.json` importer for lowering generated tree-sitter grammars into the grammar IR.

### Added
- Add a clean-room ANTLR v4 `.g4` importer for lowering grammar files into the grammar IR.

### Added
- Add clean-room Lark and GBNF grammar importers that lower into the grammar IR.

### Added
- Added public BNF, EBNF, and ABNF grammar emitters for the grammar IR.

### Added
- Add a public GBNF grammar emitter for LLM grammar-constraint interop.

### Added
- Added a public pest PEG grammar emitter for the grammar IR.

### Added
- Added Rust parser codegen artifacts for grammar IR, including pest derive stubs and AST type rendering.

### Added
- Added Peggy grammar and JavaScript parser module code generation for the grammar IR.

### Added
- Added a public tree-sitter `grammar.js` emitter for the grammar IR.

### Added

- Add state-merging regular inference with RPNI, EDSM, and ALERGIA.

### Added

- Add MDL/Occam minimization for inferred grammars.

### Added

- Add semantic constraint inference for grammar corpora

### Added
- Added concept-aligned grammar surface translation with deterministic rule-name, non-terminal, and doc-comment rewrites.

### Added
- Added a deterministic grammar inference evaluation harness with sampling, oracle scoring, MDL size metrics, and smoke corpus reports.

### Added
- Added deterministic lexical class inference with category-based tokenisation and grammar IR token rules.

### Added
- Added a Sequitur structural-compression inference pass that emits deterministic inferred grammar IR.

### Added
- Added a delimiter-skeleton structural prior API for positive grammar inference.

### Added
- Add deterministic CFG inference over delimiter structural priors with oracle-checked positive recall.

### Added
- Add deterministic and optional LLM-assisted grammar inference advisors for rule naming and merge ranking.

### Added
- Add a runtime `GrammarParser` and registry helpers for parsing with imported or inferred grammars.

### Added
- Add opt-in active regular-language learning with L*, DFA output, parser-backed oracles, and right-linear grammar lowering.

### Added
- Added CLI grammar subcommands for inference, import, emit, and concept-aligned translation.

### Added
- Add the E3 competitor corpus manifest, vendored benchmark fixtures, and a D1/D5 benchmark gate for included subjects.

### Added
- Added grammar-format fidelity profiles and the generated BNF round-trip matrix documentation.

### Added
- Added semantic grammar validation diagnostics for undefined references, left recursion, unreachable rules, nullable repetitions, duplicate rules, and unused captures.

### Added
- Added end-to-end grammar inference examples and integration tests for Rust, JavaScript, GBNF, and CLI emit pipelines.

### Added
- Added the `js/` JavaScript implementation package for the meta-language core.
- Added Rust/JavaScript parity manifest checks and separate JS/Rust workflows.

### Added
- Added JavaScript truth-value semantics parity for `TruthValue`,
  `Probability`, and `ProbabilisticTruthValue`.

### Added
- Added a module-level Rust/JavaScript parity gate: every `pub mod` in
  `rust/src/lib.rs` must now be classified in `parity/language-features.json`
  (`rustModules`) as either `ported` (naming an implemented feature row) or
  `rust-only` (with a justification). The JavaScript checker
  (`js/scripts/check-js-rust-parity.mjs`) and the Rust test
  (`rust/tests/unit/parity_manifest.rs`) both fail when the public Rust surface
  and the manifest drift apart, so a new Rust module can no longer slip in
  without an explicit JavaScript parity decision.
- Ported four previously Rust-only modules to JavaScript with full test
  coverage and parity manifest rows: read-only access (`ReadOnlyNetwork`,
  `EngineNetwork`, `AccessMode`), embedded-region detection (`EmbeddedRegion`,
  `detectEmbeddedRegions`, `RegionDetectionPolicy`), language profiles
  (`LanguageProfile`, `LanguageProfileLinks`, `LanguageProfileViolation`), and
  the link-rule query algebra (`LinkRule`, `LinkRuleRegistry`,
  `TraversalStrategy`, and the rule snapshot suite).
- Added `LinkMetadata.withDefinition`/`definition` to the JavaScript primitives
  for parity with the Rust `LinkMetadata` definition field, and recorded
  `parse-configuration` and `link-flags` feature rows that already existed in
  both languages but were previously untracked.

## [0.47.0] - 2026-06-21

### Added
- Changeset-style fragment format with frontmatter for specifying version bump type
- New `get-bump-type.mjs` script to automatically determine version bump from fragments
- Automatic version bumping on merge to main based on changelog fragments
- Detailed documentation for the changelog fragment system in `changelog.d/README.md`

### Changed
- Updated `collect-changelog.mjs` to strip frontmatter when collecting fragments
- Updated `version-and-commit.mjs` to handle frontmatter in fragments
- Enhanced release workflow to automatically determine bump type from changesets

### Changed
- Add `detect-changes` job with cross-platform `detect-code-changes.mjs` script
- Make lint job independent of changelog check (runs based on file changes only)
- Allow docs-only PRs without changelog fragment requirement
- Handle changelog check 'skipped' state in dependent jobs
- Exclude `changelog.d/`, `docs/`, `experiments/`, `examples/` folders and markdown files from code changes detection

### Fixed
- Fixed README.md to correctly reference Node.js scripts (`.mjs`) instead of Python scripts (`.py`)
- Updated project structure in README.md to match actual script files in `scripts/` directory
- Fixed example code in README.md that had invalid Rust with two `main` functions

### Added

- Added crates.io publishing support to CI/CD workflow
- Added `release_mode` input with "instant" and "changelog-pr" options for manual releases
- Added `--tag-prefix` and `--crates-io-url` options to create-github-release.mjs script
- Added comprehensive case study documentation for Issue #11 in docs/case-studies/issue-11/

### Changed

- Changed changelog fragment check from warning to error (exit 1) to enforce changelog requirements
- Updated job conditions with `always() && !cancelled()` to fix workflow_dispatch job skipping issue
- Renamed manual-release job to "Instant Release" for clarity

### Fixed

- Fixed deprecated `::set-output` GitHub Actions command in version-and-commit.mjs
- Fixed workflow_dispatch triggering issues where lint/build/release jobs were incorrectly skipped

### Fixed

- Fixed changelog fragment check to validate that a fragment is **added in the PR diff** rather than just checking if any fragments exist in the directory. This prevents the check from incorrectly passing when there are leftover fragments from previous PRs that haven't been released yet.

### Changed

- Converted shell scripts in `release.yml` to cross-platform `.mjs` scripts for improved portability and performance:
  - `check-changelog-fragment.mjs` - validates changelog fragment is added in PR diff
  - `git-config.mjs` - configures git user for CI/CD
  - `check-release-needed.mjs` - checks if release is needed
  - `publish-crate.mjs` - publishes package to crates.io
  - `create-changelog-fragment.mjs` - creates changelog fragments for manual releases
  - `get-version.mjs` - gets current version from Cargo.toml

### Added

- Added `check-version-modification.mjs` script to detect manual version changes in Cargo.toml
- Added `version-check` job to CI/CD workflow that runs on pull requests
- Added skip logic for automated release branches (changelog-manual-release-*, changeset-release/*, release/*, automated-release/*)

### Changed

- Version modifications in Cargo.toml are now blocked in pull requests to enforce automated release pipeline

### Added

- Added support for `CARGO_REGISTRY_TOKEN` as alternative to `CARGO_TOKEN` for crates.io publishing
- Added case study documentation for Issue #17 (yargs reserved word and dual token support)

### Changed

- Updated workflow to use fallback logic: `${{ secrets.CARGO_REGISTRY_TOKEN || secrets.CARGO_TOKEN }}`
- Improved publish-crate.mjs to check both `CARGO_REGISTRY_TOKEN` and `CARGO_TOKEN` environment variables
- Added warning message when neither token is set

### Added
- New `scripts/rust-paths.mjs` utility for automatic Rust package root detection
- Support for both single-language and multi-language repository structures in all CI/CD scripts
- Configuration options via `--rust-root` CLI argument and `RUST_ROOT` environment variable
- Comprehensive case study documentation in `docs/case-studies/issue-19/`

### Changed
- Updated all release scripts to use the new path detection utility:
  - `scripts/bump-version.mjs`
  - `scripts/check-release-needed.mjs`
  - `scripts/collect-changelog.mjs`
  - `scripts/get-bump-type.mjs`
  - `scripts/get-version.mjs`
  - `scripts/publish-crate.mjs`
  - `scripts/version-and-commit.mjs`

### Changed

- **check-release-needed.mjs**: Now checks crates.io API directly instead of git tags to determine if a version is already released. This prevents false positives where git tags exist but the package was never actually published to crates.io.

### Added

- **CI/CD Troubleshooting Guide**: New documentation at `docs/ci-cd/troubleshooting.md` covering common issues like skipped jobs, false positive version checks, publishing failures, and secret configuration.

- **Enhanced Error Handling in publish-crate.mjs**: Added specific detection and helpful error messages for authentication failures, including guidance on secret configuration and workflow setup.

- **Case Study Documentation**: Added comprehensive case study at `docs/case-studies/issue-21/` analyzing CI/CD failures from browser-commander repository (issues #27, #29, #31, #33) with timeline, root causes, and lessons learned.

### Fixed

- **Prevent False Positive Version Checks**: The release workflow now correctly identifies unpublished versions by checking crates.io instead of relying on git tags, which can exist without the package being published.

### Changed

- Translated all CI/CD scripts from JavaScript (.mjs) to Rust (.rs) using rust-script
- Scripts now use native Rust with rust-script for execution in shell
- Removed Node.js dependency from CI/CD pipeline
- Updated GitHub Actions workflow to use rust-script instead of node
- Updated README and CONTRIBUTING documentation with new script references

### Added

- Cache `restore-keys` for partial cache hits across all workflow jobs
- Explicit `token` parameter in checkout for release jobs
- Code coverage job with `cargo-llvm-cov` and Codecov integration
- Codecov badge in README.md
- Pre-release version support (e.g., `0.1.0-beta.1`) in version parsing
- `--release-label` parameter for multi-language release disambiguation
- `ensure_version_exceeds_published()` logic to prevent publishing duplicate versions
- `get_max_published_version()` to query highest non-yanked version from crates.io
- `max_published_version` output from check-release-needed for downstream use
- Version fallback logic in auto-release Create GitHub Release step

### Changed

- Updated `actions/checkout` from v4 to v6
- Updated `actions/cache` from v4 to v5
- Updated `peter-evans/create-pull-request` from v7 to v8
- Made `publish-crate.rs` fail (exit 1) when version already exists on crates.io
- Improved `create-github-release.rs` to check combined stdout+stderr and detect "Validation Failed"

### Fixed

- Fix publish steps overriding workflow-level CARGO_TOKEN fallback, breaking CARGO_REGISTRY_TOKEN-only configurations (#32)
- Fix non-fast-forward push failures in multi-workflow repos by adding fetch/rebase and push retry logic (#31)
- Add mono-repo path support to check-changelog-fragment.rs, check-version-modification.rs, and create-changelog-fragment.rs
- Add `!cancelled()` guard to test job condition to respect workflow cancellation

### Fixed

- Fixed `version-and-commit.rs` to check crates.io instead of git tags for determining if a version is already released
- This prevents the release pipeline from getting stuck when git tags exist without corresponding crates.io publication

### Added

- Added `--tag-prefix` support to `version-and-commit.rs` for multi-language repository compatibility
- Added crates.io and docs.rs badges to README.md
- Added automatic crates.io and docs.rs badge injection in GitHub release notes
- Added documentation deployment job to CI/CD pipeline (deploys to GitHub Pages after release)
- Added case study documentation for issue #25

### Fixed
- Fixed unsupported look-ahead regex in `create-github-release.rs` that caused a panic when parsing CHANGELOG.md. Replaced with a two-step approach using only features supported by Rust's `regex` crate.

### Changed
- Restructured example application as a simple CLI sum calculator using `lino-arguments`
- Renamed default package to `example-sum-package-name` with Unlicense license
- Reorganized test structure: `tests/unit/sum.rs`, `tests/integration/sum.rs`, `tests/unit/ci-cd/`
- Converted experiment scripts into proper unit tests in `tests/unit/ci-cd/changelog_parsing.rs`
- Added CI/CD skip logic for template default package name `example-sum-package-name`
- Updated README.md badges and documentation

### Fixed

- Change detection script now uses per-commit diff instead of full PR diff, so commits touching only non-code files correctly skip CI jobs even when earlier commits in the same PR changed code files

### Fixed
- Make release scripts resolve the publishable crate manifest when the repository root uses a Cargo workspace manifest.

### Fixed
- Decoupled GitHub Pages documentation deployment from package release publication and fixed release-script warning failures under `RUSTFLAGS=-Dwarnings`.

### Changed
- Added explicit GitHub Actions job timeouts and documented Rust test timeout guidance.

### Fixed
- Added a non-blocking warning threshold to the Rust file-size check so near-limit files are surfaced before concurrent PR merges can exceed the hard limit.

### Fixed
- Made `create-github-release.rs` build GitHub release titles as `[Language] X.Y.Z` instead of reusing the tag prefix.

### Added
- Added optional Docker Hub image publishing tied to Rust crate releases, including crates.io visibility waiting, version/latest image tags, and Docker Hub badges in GitHub release notes.

### Changed
- Release completeness checks now self-heal when crates.io exists but configured Docker Hub or GitHub release artifacts are missing.

### Fixed
- Switched documentation deployment to the official GitHub Pages artifact workflow so repositories using GitHub Actions as their Pages source do not get false-positive branch-push deploys.

### Added
- Documented the one-time `Settings → Pages → Source = GitHub Actions` prerequisite for the `deploy-docs` job in `README.md` and as a comment above the `deploy-docs` job in `release.yml`, so downstream template users hit a documented setup step instead of a `Get Pages site failed` error on the first deploy.

### Added
- Tracking case study at `docs/case-studies/issue-52/` registering the `browser-commander` + Playwright preview-regeneration pattern from [`konard/vk-bot-desktop#52`](https://github.com/konard/vk-bot-desktop/pull/52), with an activation checklist for when an example-app surface lands in this template. Documentation only — no workflow, script, or runtime code changes. Primary upstream tracking issue: [`link-foundation/js-ai-driven-development-pipeline-template#62`](https://github.com/link-foundation/js-ai-driven-development-pipeline-template/issues/62).

### Fixed
- Release automation now keeps the workspace package entry in `Cargo.lock` synchronized when `scripts/version-and-commit.rs` bumps `Cargo.toml`, preventing stale lock-file version diffs in later pull requests.

### Fixed
- `scripts/publish-crate.rs` now treats crates.io HTTP 429 throttle responses ("You have published too many versions of this crate in the last 24 hours") as a deferred `publish_result=rate_limited` outcome (it writes the output, prints an explanatory banner and exits successfully) instead of a hard CI failure reported as a generic `failed` ("Failed to publish for unknown reason"). Authentication, already-published and unknown failures still exit non-zero. Failed-publish classification is consolidated through a single `classify_failure` function and `FailureKind` enum (with an `is_deferred` predicate), covered by unit tests runnable via `rust-script --test scripts/publish-crate.rs`.
- The release workflow (`.github/workflows/release.yml`) now gates crate-availability waiting, Docker Hub publishing and GitHub release creation on either an already-published crate or `publish_result=success`, so a deferred (rate-limited) crate upload no longer produces partial downstream release artifacts and the same version is retried automatically on the next push to `main`.

### Fixed
- Fixed reversed `cancel-in-progress` concurrency condition in `release.yml` that cancelled in-flight releases on `main` and never superseded older PR runs. The condition now uses `!=` so `main` releases run to completion while newer PR pushes cancel stale runs.

### Added
- Added a `scripts/check-crate-size.rs` guard that builds the `.crate` archive and fails the release before publishing when it exceeds the crates.io 10 MiB upload limit. The check runs in the build job and before publishing in both the auto-release and manual-release jobs.

### Changed
- Added a narrow `include` allowlist to `Cargo.toml` so docs, case studies, generated CI artifacts, changelog fragments, scripts, and experiments no longer inflate the published release archive.

### Fixed
- Prevented GitHub release creation from treating generic API validation failures as existing releases, and capped oversized release notes with a link to the full tagged changelog.

### Added
- Added the initial `meta-language` links-network core with self-description roots,
  verification, trivia attachment configuration, and a minimal CLI.
- Added the default lossless `parse` entry point, projection views, and a
  tested parity roadmap for competitor features, grammar embedding, and language
  coverage targets.
- Added exact reconstruction, mixed-region detection, query matching,
  substitution rules, concept reconstruction, object identity helpers,
  many-valued truth values, and executable parity fixtures for every tracked
  competitor target.
- Added executable language fixtures and tests for every requested Markdown,
  HTML, top-ten programming-language, and top-ten natural-language target.

### Fixed
- Added a CI guard that rejects Rust test modules and test attributes under `src/`, keeping tests in the `tests/` tree.

## Minor Changes

- Add immutable and mutable network snapshots with provenance and forward
  version commits for roadmap snapshot/versioning coverage.

### Added
- Added `txt` as a first-class markup/container target with a UTF-8 lossless
  fixture and content-sniffing fallback regions.

### Fixed
- Reconciled the natural-language target ordering with the Ethnologue 2025
  total-speaker order cited by the parity roadmap.

### Added
- Added a grammar-backed parser front end for Python, C, Java, C++, C#, JavaScript, and R using official tree-sitter grammar crates.

### Added
- Parse supported mixed-mode embedded regions into the host links network with grammar-backed syntax links.

### Added
- Added natural-language segmentation, identification, normalization, and bidi annotation links over lossless text parses.

### Added
- Added grammar-backed Visual Basic parsing with byte-exact reconstruction and recovery flag coverage.

bump: minor

- Add a grammar-backed `sql-ansi` SQL-family dialect fixture using
  `tree-sitter-sequel`.

### Added
- Added grammar-backed Delphi/Object Pascal parsing through `tree-sitter-pascal`.

### Added
- Enriched link queries with S-expression structural matching, captures, by-type construction, host predicate hooks, and link-cli-style variable substitution bindings.

### Added
- Added persistent snapshot structural sharing with interned metadata text storage.

### Added
- Added a query-transform surface for selecting captured links and replacing
  their source text while preserving unchanged bytes.

### Added
- Materialized self-description roots as controlled links with complete root-definition closure and round-trippable `describe` output.

### Added
- Added common concept ontology seeding from meta-expression's semantic lexicon, with shared concept links, syntax mappings, and structural programming-language concepts.

### Added
- Added semantic cross-language reconstruction for the Hawaii statehood fixture,
  including English/Russian naturalization and configurable formalization levels.

### Added
- Expanded `PARITY_FIXTURES` with multiple provenance-tracked fixtures for tree-sitter, LibCST, Recast, jscodeshift, Rowan, cstree, and Roslyn, including executable recovery and query/transform expectations.

### Added

- Expanded `PARITY_FIXTURES` with upstream-provenanced internal ecosystem corpora for links-notation, link-cli, lino-objects-codec, relative-meta-logic, formal-ai, and meta-expression.

### Added

- Added structural LiNo parsing for links-notation doublets, triplets, named links, indented IDs, and self-references while preserving byte-exact reconstruction.

### Added
- Added fixed-point probabilistic truth values for relative-meta-logic-style
  semantic confidence evaluation.

### Added
- Added grammar-backed Go parsing via the official `tree-sitter-go` grammar
  (accepts `Go`, `go`, and `golang`) so `LinkNetwork::parse` emits real
  `LinkType::Syntax` concrete-syntax links with byte-exact reconstruction.

### Added
- `AccessMode { Mutable, ReadOnly }` setting on `ParseConfiguration`
  (`with_access_mode` / `access_mode`), defaulting to `Mutable` so existing
  callers are unaffected.
- `LinkNetwork::freeze` / `as_read_only` yielding a `ReadOnlyNetwork` view that
  exposes only `&self` operations (query, project, reconstruct, verify,
  serialize); mutators are unreachable at compile time because the view never
  hands out `&mut LinkNetwork`.
- `LinkNetwork::parse_engine`, returning an `EngineNetwork` handle that honours
  the configured access mode: read-only parsing returns the frozen form and
  `EngineNetwork::as_mutable` rejects mutation with a `ReadOnlyViolation`
  diagnostic.
- Snapshot interop: `NetworkSnapshot::as_read_only` / `from_read_only` reuse the
  snapshot's `Arc<LinkNetwork>`, so the frozen form composes with snapshot
  versioning instead of duplicating it.

### Added
- Added grammar-backed Ruby parsing through `tree-sitter-ruby`, so
  `LinkNetwork::parse(source, "Ruby", ...)` (and the `rb` alias) now emits real
  `LinkType::Syntax` links instead of falling back to lossless plain text.

bump: minor

- Add a grammar-backed `TypeScript` front end using `tree-sitter-typescript`,
  wiring the `typescript`/`ts` labels to `LANGUAGE_TYPESCRIPT` and the `tsx`
  label to `LANGUAGE_TSX` so `LinkNetwork::parse` emits real `LinkType::Syntax`
  links for TypeScript and TSX sources.

### Added
- Added grammar-backed parsing for seven data-exchange / interchange formats
  through `src/tree_sitter_adapter.rs`: JSON (`tree-sitter-json`), YAML
  (`tree-sitter-yaml`, accepts `yaml`/`yml`), TOML (`tree-sitter-toml-ng`), XML
  and DTD (`tree-sitter-xml`), INI (`tree-sitter-ini`), Protocol Buffers
  (`tree-sitter-proto`, accepts `protobuf`/`proto`/`Protocol Buffers`), and
  GraphQL (`tree-sitter-graphql`, accepts `graphql`/`gql`). Each parses into
  real `LinkType::Syntax` concrete-syntax links and reconstructs byte-for-byte.
- Added a `DATA_FORMAT_TARGETS` registry in `src/parity.rs` (with the new
  `LanguageFamily::DataFormat`) gated by parity tests, mirroring
  `MARKUP_LANGUAGE_TARGETS`.
- Added per-format UTF-8 `LANGUAGE_FIXTURES` round-trip entries plus
  recovery-case and mixed-region tests (a `json` fence embedded in Markdown
  parses into the host links network).
- Added lossless CSV and JSON5 parsers for the two formats whose published
  tree-sitter crates still pin the incompatible `tree-sitter ~0.20` runtime.
  CSV is validated with the Rust `csv` crate and JSON5 is validated with
  `json5_nodes`; both emit structured syntax links and reconstruct
  byte-for-byte.

### Documentation
- Documented the nine wired data-format parsers (parser, version, license, root
  node) in `docs/parity-roadmap.md`, including the tree-sitter compatibility
  rationale for the in-repo CSV and JSON5 parsers.

### Added
- `LinkNetwork::to_lino` serializes an entire links network to canonical
  links-notation text, keyed by each link's numeric id (doublets-style id
  discipline), covering references, names, types, terms, definitions,
  languages, source spans, parse flags, and term registration.
- `LinkNetwork::from_lino` reconstructs the exact network from that text,
  forming a lossless round-trip (`from_lino(to_lino(n))` is isomorphic to `n`).
- `LinoSerializationError` reports parse and schema failures from `from_lino`.
- A round-trip property test over every language fixture plus synthetic
  networks, and a test that `to_lino` output is accepted by the
  `links-notation` 0.13 crate parser.
- `ParityCapability::LinoSerialization` and an output-side serialization
  parity fixture for the `links-notation` target.

### Changed
- Added the `links-notation` 0.13 crate as a dependency so serialized output
  aligns with the wider links-notation ecosystem.

### Added
- `ParserRegistry` maps language keys to `Arc<dyn LanguageParser>` with the
  built-in parser as a fallback. User registrations shadow the built-in
  dispatch for the same (case-insensitive) key; unregistered keys still route
  through the built-in set.
- `LinkNetwork::parse_with_registry` dispatches a parse through a registry.
- `examples/custom_parser_registry.rs` documents registering a custom parser.

### Added
- Added grammar-backed parsing for five popular programming languages
  immediately below the TIOBE top ten through `src/tree_sitter_adapter.rs`: PHP
  (`tree-sitter-php`, via its `LANGUAGE_PHP` symbol), Swift
  (`tree-sitter-swift`), Kotlin (`tree-sitter-kotlin-ng`, accepts `kotlin`/`kt`),
  Scala (`tree-sitter-scala`), and Lua (`tree-sitter-lua`). Each parses into real
  `LinkType::Syntax` concrete-syntax links and reconstructs byte-for-byte.
- Added a `SECOND_TIER_PROGRAMMING_LANGUAGE_TARGETS` registry in `src/parity.rs`
  gated by parity tests, mirroring `DATA_FORMAT_TARGETS`.
- Added per-language UTF-8 `LANGUAGE_FIXTURES` round-trip entries plus
  case-insensitive alias coverage and a per-language recovery fixture whose
  malformed source still reconstructs while exposing error/missing diagnostics.

### Documentation
- Documented the wired grammars (crate, version, license, root node) in
  `docs/parity-roadmap.md` and kept the temporary Perl follow-up tracked
  explicitly until `ts-parser-perl` was adopted.

### Added
- Add a Rust `ToLinks`/`FromLinks` codec with queryable type-shape links and shared/circular object graph round-trips through LiNo serialization.

### Added

- Added the `LinkStore` storage trait, read-only-aware storage wrappers, and an
  optional file-mapped `doublets` backend with LiNo/binary round-trip coverage.

### Added
- Added exact-match concept interning, language-bound expression links, external-id alias links, and LiNo concept-set import for the shared concept ontology.

### Added
- Added starter natural-language grammaticality parsing with UD-style morphosyntax links, pass/fail fixtures for the ten natural-language targets, and recoverable error links for ungrammatical fixtures.

### Added
- Added configurable `TranslationRuleSet` values, a runtime
  `TranslationRuleRegistry`, LiNo-backed rule-set loading, template
  placeholders, and missing-rule diagnostics for from-meta reconstruction.

### Added
- Added queryable language profiles with JavaScript transform enforcement and rule-set-derived profile domains.

### Added
- Added incremental source edit reparsing, stable outside-edit link IDs, and structural snapshot diff reporting.

### Added
- Added composable `LinkRule` query algebra with relational rules, boolean composition, named sub-rules, ellipsis gap matching, typed metavariables, plain-text token patterns, traversal strategies, and valid/invalid rule snapshot suites.
- Added quasiquote replacement templates with placeholder validation and parenthesization-conservative captured-text replacement.

### Added
- Add the API operation/style parity registry, fluent network pipeline, and link-cli-style substitution text runner.

### Added
- Wire the canonical `ts-parser-perl` tree-sitter grammar as a second-tier
  programming-language target without upgrading the project-wide tree-sitter
  runtime.

### Added
- Added source generation helpers for constructed syntax networks: `insert_source_token`, `insert_syntax_node`, `render_source`, `render_source_from`, and `render_source_from_document`.

### Added
- Added wave-two competitor and ecosystem parity fixtures with executable transform, reconstruction, grammar, and storage gates.
- Added a recorded `cargo llvm-cov` line-coverage floor to CI so coverage cannot silently regress.

### Added
- Shared, language-free document-formatting concept ontology (`document_formatting` module): inline concepts (emphasis, strong, strikethrough, inline-code, hyperlink, image, line-break) and block concepts (heading with level, paragraph, blockquote, bullet/ordered lists, list-item, code-block with language, thematic-break, table/row/cell), each seeded with Markdown and HTML syntax mappings.
- `seed_common_concept_ontology()` now also seeds the formatting concepts, and `ConceptOntologySeedReport::formatting_concepts()` reports how many were added.
- `LinkNetwork::resolve_document_format` / `render_document_format` / `translate_document_format` so the same concept link reconstructs as `**…**` in Markdown and `<strong>…</strong>` in HTML; Markdown `**bold**` and HTML `<strong>bold</strong>` reach the one shared `strong` concept under semantic projection.
- `FormattingDocument` concept layer with `parse_markup_document` and `translate_markup_document` for full Markdown ⇄ HTML document round-trips through one concept ontology (heading/paragraph/list/bold/italic/link).

### Added
- PDF document-format support (issue #84): a documented, uncompressed text PDF profile (`document_formatting::render_pdf_document` / `parse_pdf_document`) that renders a language-free `FormattingDocument` to a valid single-page PDF (correct `xref` offsets, object table, and stream `Length`) and parses it back into the same concept tree. Block role is carried by marked content (`/H1`…`/H6`, `/P`, `/UL`/`/OL`, `/LI`) and inline bold/italic by the selected font resource (`/F1` regular, `/F2` strong, `/F3` emphasis).
- `parse("…", "pdf", …)` dispatches to a new `pdf_parser` that builds a byte-exact lossless network (`reconstruct_text()` returns the input verbatim) and adds additive `Concept`/`Object` structure links recovering heading/paragraph/list/list-item and bold/italic.
- `reconstruct_text_as("PDF", …)` renders a structurally equivalent PDF through the shared formatting concept layer: a PDF source re-renders byte-for-byte, while a Markdown/HTML source is translated into an equivalent PDF, and `translate_markup_document` now bridges Markdown/HTML ⇄ PDF.
- `PDF` markup target in `MARKUP_LANGUAGE_TARGETS` with a bold + heading + paragraph round-trip `LANGUAGE_FIXTURES` entry, plus `docs/pdf-fidelity.md` documenting the round-trip fidelity matrix for supported and lossy/unsupported PDF features.

### Added
- DOCX (OOXML) document-format support (issue #85): a documented OOXML text profile (`document_formatting::render_docx_document` / `parse_docx_document`) that renders a language-free `FormattingDocument` to `word/document.xml` WordprocessingML and parses it back into the same concept tree. Block role is carried by paragraph properties (`<w:pStyle w:val="HeadingN"/>` headings, bare `<w:p>` paragraphs, `<w:numPr>` `numId` 1/2 bullet/ordered list items) and inline bold/italic by run properties (`<w:b/>` → `strong`, `<w:i/>` → `emphasis`).
- A binary OPC packaging layer (`document_formatting::render_docx_package` / `parse_docx_package`) that assembles a valid `.docx` ZIP (stored entries with a self-implemented CRC-32, no new dependencies) containing `[Content_Types].xml`, the relationship parts, `word/document.xml`, `word/styles.xml`, and `word/numbering.xml`, and reads `word/document.xml` back out.
- `parse("…", "docx", …)` dispatches to a new `docx_parser` that builds a byte-exact lossless network (`reconstruct_text()` returns the input verbatim) and adds additive `Concept`/`Object` structure links recovering heading/paragraph/list/list-item and bold/italic.
- `reconstruct_text_as("DOCX", …)` renders structurally equivalent OOXML through the shared formatting concept layer: a DOCX source re-renders byte-for-byte, while a Markdown/HTML/PDF source is translated into equivalent OOXML, and `translate_markup_document` now bridges Markdown/HTML/PDF ⇄ DOCX.
- `DOCX` markup target in `MARKUP_LANGUAGE_TARGETS` with a bold + heading + bullet-list round-trip `LANGUAGE_FIXTURES` entry, plus `docs/docx-fidelity.md` documenting the two-layer round-trip fidelity matrix for supported and lossy/unsupported OOXML features.

### Fixed
- Fixed the GitHub Pages site returning HTTP 404 at the root URL
  (<https://link-foundation.github.io/meta-language>). The deploy job uploaded
  raw `cargo doc` output, which has no root `index.html` (rustdoc only emits
  `target/doc/<crate>/index.html`), so the Pages root 404'd while docs were only
  reachable at `/meta_language/`. See issue #90.

### Added
- A project website with a landing page (description + docs links) under
  `docs/site/`, deployed at the Pages root.
- An interactive WebAssembly "Links Notation playground" demo in a new
  standalone `web/` crate (wraps the wasm-compatible `links-notation` crate),
  served at `/demo/`.
- `scripts/build-site.rs` to assemble the website (`_site/`) locally and in CI:
  landing page at `/`, demo at `/demo/`, and `rustdoc` under `/api/` with a root
  redirect.
- `docs/case-studies/issue-90` documenting the timeline, requirements, root
  cause analysis, and solution plans.

### Changed
- The `Deploy Website` CI job (formerly `Deploy Rust Documentation`) now builds
  the WebAssembly demo and assembles the full site instead of deploying
  `target/doc` directly.

### Added
- Cross-format document reconstruction and round-trip translation (issue #86): `reconstruct_text_as("txt" | "Markdown" | "HTML" | "PDF" | "DOCX", …)` now works over the shared, language-free formatting concept layer (issue #83), so a document parsed from any supported format reconstructs into any other when the source uses only concepts both formats support. A same-format target re-renders byte-for-byte; a cross-format target is translated through the concept tree, preserving heading/paragraph/list and bold/italic/link structure.
- `txt` joins Markdown, HTML, PDF, and DOCX as a first-class document format in `parse_markup_document` / `render_markup_document`: blank-line-separated paragraphs parse into the concept layer, and the concept layer flattens to plain text (headings to plain lines, lists to `- `/`N. ` markers, inline styling dropped) as the documented lossy fallback target.
- Per-format capability profiles (`document_format_profile`, `DOCUMENT_FORMATS`, `CROSS_FORMAT_CONCEPTS`, `canonical_document_format`) expose each format's `LanguageProfile` over the formatting concept ontology, reporting for every cross-format concept either native support or a documented lossy fallback rather than silent data loss.
- `LanguageProfile` gained `with_concept_fallback` / `concept_fallback` / `fallbacks` to declare and query the lossy fallback for concepts a target cannot represent natively.
- A round-trip matrix test covering every ordered pair of `{txt, Markdown, HTML, PDF, DOCX}` (a sample built from the concepts both formats share survives `A → concepts → B → concepts → A`), plus `docs/cross-format-fidelity.md` documenting the cross-format translation entry point and the per-format fidelity matrix.

### Added
- Case study and implementation backlog for grammar extensibility & inference (issue #93): `docs/case-studies/issue-93/` collects the requirement register, a grammar-inference literature review (Gold (1967), RPNI/EDSM, Sequitur, the GLADE→Arvada→TreeVada→Kedavra→NatGI line, ISLearn), a licence-vetted library/ecosystem survey, an existing-capability gap analysis, a competitive analysis pinning the metrics to beat, and per-epic solution plans with the issue dependency DAG. The 34 maximally-detailed sub-issue specs under `proposed-issues/` were filed as GitHub issues #95–#128, each attached as a native sub-issue of #93 with all 51 `blocked-by` dependency edges wired via the GitHub REST API. This is a planning/research deliverable only — no library code changes.

### Added
- Added a public grammar IR with expression builders and links round-trip encoding.

### Added
- Added grammar surface syntax parsing, writing, and LiNo bridge helpers for the grammar IR.

### Added
- Seed grammar-construct concepts and expose grammar expression concept alignment helpers.

### Added
- Added grammar subsystem documentation, README integration, and tests for the grammar docs page set and relative links.

### Added
- Added a public BNF grammar importer that lowers classic BNF productions into the grammar IR.

### Added
- Added an EBNF grammar importer that lowers ISO-style EBNF constructs into the grammar IR.

### Added
- Added an ABNF grammar importer with RFC 5234 core rules, RFC 7405 string sensitivity, numeric terminals, repetition, and incremental alternative lowering.

### Added
- Added a PEG `.pest` grammar importer backed by `pest_meta`.

### Added
- Add a tree-sitter `grammar.json` importer for lowering generated tree-sitter grammars into the grammar IR.

### Added
- Add a clean-room ANTLR v4 `.g4` importer for lowering grammar files into the grammar IR.

### Added
- Add clean-room Lark and GBNF grammar importers that lower into the grammar IR.

### Added
- Added public BNF, EBNF, and ABNF grammar emitters for the grammar IR.

### Added
- Add a public GBNF grammar emitter for LLM grammar-constraint interop.

### Added
- Added a public pest PEG grammar emitter for the grammar IR.

### Added
- Added Rust parser codegen artifacts for grammar IR, including pest derive stubs and AST type rendering.

### Added
- Added Peggy grammar and JavaScript parser module code generation for the grammar IR.

### Added
- Added a public tree-sitter `grammar.js` emitter for the grammar IR.

### Added

- Add state-merging regular inference with RPNI, EDSM, and ALERGIA.

### Added

- Add MDL/Occam minimization for inferred grammars.

### Added

- Add semantic constraint inference for grammar corpora

### Added
- Added concept-aligned grammar surface translation with deterministic rule-name, non-terminal, and doc-comment rewrites.

### Added
- Added a deterministic grammar inference evaluation harness with sampling, oracle scoring, MDL size metrics, and smoke corpus reports.

### Added
- Added deterministic lexical class inference with category-based tokenisation and grammar IR token rules.

### Added
- Added a Sequitur structural-compression inference pass that emits deterministic inferred grammar IR.

### Added
- Added a delimiter-skeleton structural prior API for positive grammar inference.

### Added
- Add deterministic CFG inference over delimiter structural priors with oracle-checked positive recall.

### Added
- Add deterministic and optional LLM-assisted grammar inference advisors for rule naming and merge ranking.

### Added
- Add a runtime `GrammarParser` and registry helpers for parsing with imported or inferred grammars.

### Added
- Add opt-in active regular-language learning with L*, DFA output, parser-backed oracles, and right-linear grammar lowering.

### Added
- Added CLI grammar subcommands for inference, import, emit, and concept-aligned translation.

### Added
- Add the E3 competitor corpus manifest, vendored benchmark fixtures, and a D1/D5 benchmark gate for included subjects.

### Added
- Added grammar-format fidelity profiles and the generated BNF round-trip matrix documentation.

### Added
- Added semantic grammar validation diagnostics for undefined references, left recursion, unreachable rules, nullable repetitions, duplicate rules, and unused captures.

### Added
- Added end-to-end grammar inference examples and integration tests for Rust, JavaScript, GBNF, and CLI emit pipelines.

### Added
- Added the `js/` JavaScript implementation package for the meta-language core.
- Added Rust/JavaScript parity manifest checks and separate JS/Rust workflows.

## [0.46.0] - 2026-06-20

### Added
- Changeset-style fragment format with frontmatter for specifying version bump type
- New `get-bump-type.mjs` script to automatically determine version bump from fragments
- Automatic version bumping on merge to main based on changelog fragments
- Detailed documentation for the changelog fragment system in `changelog.d/README.md`

### Changed
- Updated `collect-changelog.mjs` to strip frontmatter when collecting fragments
- Updated `version-and-commit.mjs` to handle frontmatter in fragments
- Enhanced release workflow to automatically determine bump type from changesets

### Changed
- Add `detect-changes` job with cross-platform `detect-code-changes.mjs` script
- Make lint job independent of changelog check (runs based on file changes only)
- Allow docs-only PRs without changelog fragment requirement
- Handle changelog check 'skipped' state in dependent jobs
- Exclude `changelog.d/`, `docs/`, `experiments/`, `examples/` folders and markdown files from code changes detection

### Fixed
- Fixed README.md to correctly reference Node.js scripts (`.mjs`) instead of Python scripts (`.py`)
- Updated project structure in README.md to match actual script files in `scripts/` directory
- Fixed example code in README.md that had invalid Rust with two `main` functions

### Added

- Added crates.io publishing support to CI/CD workflow
- Added `release_mode` input with "instant" and "changelog-pr" options for manual releases
- Added `--tag-prefix` and `--crates-io-url` options to create-github-release.mjs script
- Added comprehensive case study documentation for Issue #11 in docs/case-studies/issue-11/

### Changed

- Changed changelog fragment check from warning to error (exit 1) to enforce changelog requirements
- Updated job conditions with `always() && !cancelled()` to fix workflow_dispatch job skipping issue
- Renamed manual-release job to "Instant Release" for clarity

### Fixed

- Fixed deprecated `::set-output` GitHub Actions command in version-and-commit.mjs
- Fixed workflow_dispatch triggering issues where lint/build/release jobs were incorrectly skipped

### Fixed

- Fixed changelog fragment check to validate that a fragment is **added in the PR diff** rather than just checking if any fragments exist in the directory. This prevents the check from incorrectly passing when there are leftover fragments from previous PRs that haven't been released yet.

### Changed

- Converted shell scripts in `release.yml` to cross-platform `.mjs` scripts for improved portability and performance:
  - `check-changelog-fragment.mjs` - validates changelog fragment is added in PR diff
  - `git-config.mjs` - configures git user for CI/CD
  - `check-release-needed.mjs` - checks if release is needed
  - `publish-crate.mjs` - publishes package to crates.io
  - `create-changelog-fragment.mjs` - creates changelog fragments for manual releases
  - `get-version.mjs` - gets current version from Cargo.toml

### Added

- Added `check-version-modification.mjs` script to detect manual version changes in Cargo.toml
- Added `version-check` job to CI/CD workflow that runs on pull requests
- Added skip logic for automated release branches (changelog-manual-release-*, changeset-release/*, release/*, automated-release/*)

### Changed

- Version modifications in Cargo.toml are now blocked in pull requests to enforce automated release pipeline

### Added

- Added support for `CARGO_REGISTRY_TOKEN` as alternative to `CARGO_TOKEN` for crates.io publishing
- Added case study documentation for Issue #17 (yargs reserved word and dual token support)

### Changed

- Updated workflow to use fallback logic: `${{ secrets.CARGO_REGISTRY_TOKEN || secrets.CARGO_TOKEN }}`
- Improved publish-crate.mjs to check both `CARGO_REGISTRY_TOKEN` and `CARGO_TOKEN` environment variables
- Added warning message when neither token is set

### Added
- New `scripts/rust-paths.mjs` utility for automatic Rust package root detection
- Support for both single-language and multi-language repository structures in all CI/CD scripts
- Configuration options via `--rust-root` CLI argument and `RUST_ROOT` environment variable
- Comprehensive case study documentation in `docs/case-studies/issue-19/`

### Changed
- Updated all release scripts to use the new path detection utility:
  - `scripts/bump-version.mjs`
  - `scripts/check-release-needed.mjs`
  - `scripts/collect-changelog.mjs`
  - `scripts/get-bump-type.mjs`
  - `scripts/get-version.mjs`
  - `scripts/publish-crate.mjs`
  - `scripts/version-and-commit.mjs`

### Changed

- **check-release-needed.mjs**: Now checks crates.io API directly instead of git tags to determine if a version is already released. This prevents false positives where git tags exist but the package was never actually published to crates.io.

### Added

- **CI/CD Troubleshooting Guide**: New documentation at `docs/ci-cd/troubleshooting.md` covering common issues like skipped jobs, false positive version checks, publishing failures, and secret configuration.

- **Enhanced Error Handling in publish-crate.mjs**: Added specific detection and helpful error messages for authentication failures, including guidance on secret configuration and workflow setup.

- **Case Study Documentation**: Added comprehensive case study at `docs/case-studies/issue-21/` analyzing CI/CD failures from browser-commander repository (issues #27, #29, #31, #33) with timeline, root causes, and lessons learned.

### Fixed

- **Prevent False Positive Version Checks**: The release workflow now correctly identifies unpublished versions by checking crates.io instead of relying on git tags, which can exist without the package being published.

### Changed

- Translated all CI/CD scripts from JavaScript (.mjs) to Rust (.rs) using rust-script
- Scripts now use native Rust with rust-script for execution in shell
- Removed Node.js dependency from CI/CD pipeline
- Updated GitHub Actions workflow to use rust-script instead of node
- Updated README and CONTRIBUTING documentation with new script references

### Added

- Cache `restore-keys` for partial cache hits across all workflow jobs
- Explicit `token` parameter in checkout for release jobs
- Code coverage job with `cargo-llvm-cov` and Codecov integration
- Codecov badge in README.md
- Pre-release version support (e.g., `0.1.0-beta.1`) in version parsing
- `--release-label` parameter for multi-language release disambiguation
- `ensure_version_exceeds_published()` logic to prevent publishing duplicate versions
- `get_max_published_version()` to query highest non-yanked version from crates.io
- `max_published_version` output from check-release-needed for downstream use
- Version fallback logic in auto-release Create GitHub Release step

### Changed

- Updated `actions/checkout` from v4 to v6
- Updated `actions/cache` from v4 to v5
- Updated `peter-evans/create-pull-request` from v7 to v8
- Made `publish-crate.rs` fail (exit 1) when version already exists on crates.io
- Improved `create-github-release.rs` to check combined stdout+stderr and detect "Validation Failed"

### Fixed

- Fix publish steps overriding workflow-level CARGO_TOKEN fallback, breaking CARGO_REGISTRY_TOKEN-only configurations (#32)
- Fix non-fast-forward push failures in multi-workflow repos by adding fetch/rebase and push retry logic (#31)
- Add mono-repo path support to check-changelog-fragment.rs, check-version-modification.rs, and create-changelog-fragment.rs
- Add `!cancelled()` guard to test job condition to respect workflow cancellation

### Fixed

- Fixed `version-and-commit.rs` to check crates.io instead of git tags for determining if a version is already released
- This prevents the release pipeline from getting stuck when git tags exist without corresponding crates.io publication

### Added

- Added `--tag-prefix` support to `version-and-commit.rs` for multi-language repository compatibility
- Added crates.io and docs.rs badges to README.md
- Added automatic crates.io and docs.rs badge injection in GitHub release notes
- Added documentation deployment job to CI/CD pipeline (deploys to GitHub Pages after release)
- Added case study documentation for issue #25

### Fixed
- Fixed unsupported look-ahead regex in `create-github-release.rs` that caused a panic when parsing CHANGELOG.md. Replaced with a two-step approach using only features supported by Rust's `regex` crate.

### Changed
- Restructured example application as a simple CLI sum calculator using `lino-arguments`
- Renamed default package to `example-sum-package-name` with Unlicense license
- Reorganized test structure: `tests/unit/sum.rs`, `tests/integration/sum.rs`, `tests/unit/ci-cd/`
- Converted experiment scripts into proper unit tests in `tests/unit/ci-cd/changelog_parsing.rs`
- Added CI/CD skip logic for template default package name `example-sum-package-name`
- Updated README.md badges and documentation

### Fixed

- Change detection script now uses per-commit diff instead of full PR diff, so commits touching only non-code files correctly skip CI jobs even when earlier commits in the same PR changed code files

### Fixed
- Make release scripts resolve the publishable crate manifest when the repository root uses a Cargo workspace manifest.

### Fixed
- Decoupled GitHub Pages documentation deployment from package release publication and fixed release-script warning failures under `RUSTFLAGS=-Dwarnings`.

### Changed
- Added explicit GitHub Actions job timeouts and documented Rust test timeout guidance.

### Fixed
- Added a non-blocking warning threshold to the Rust file-size check so near-limit files are surfaced before concurrent PR merges can exceed the hard limit.

### Fixed
- Made `create-github-release.rs` build GitHub release titles as `[Language] X.Y.Z` instead of reusing the tag prefix.

### Added
- Added optional Docker Hub image publishing tied to Rust crate releases, including crates.io visibility waiting, version/latest image tags, and Docker Hub badges in GitHub release notes.

### Changed
- Release completeness checks now self-heal when crates.io exists but configured Docker Hub or GitHub release artifacts are missing.

### Fixed
- Switched documentation deployment to the official GitHub Pages artifact workflow so repositories using GitHub Actions as their Pages source do not get false-positive branch-push deploys.

### Added
- Documented the one-time `Settings → Pages → Source = GitHub Actions` prerequisite for the `deploy-docs` job in `README.md` and as a comment above the `deploy-docs` job in `release.yml`, so downstream template users hit a documented setup step instead of a `Get Pages site failed` error on the first deploy.

### Added
- Tracking case study at `docs/case-studies/issue-52/` registering the `browser-commander` + Playwright preview-regeneration pattern from [`konard/vk-bot-desktop#52`](https://github.com/konard/vk-bot-desktop/pull/52), with an activation checklist for when an example-app surface lands in this template. Documentation only — no workflow, script, or runtime code changes. Primary upstream tracking issue: [`link-foundation/js-ai-driven-development-pipeline-template#62`](https://github.com/link-foundation/js-ai-driven-development-pipeline-template/issues/62).

### Fixed
- Release automation now keeps the workspace package entry in `Cargo.lock` synchronized when `scripts/version-and-commit.rs` bumps `Cargo.toml`, preventing stale lock-file version diffs in later pull requests.

### Fixed
- `scripts/publish-crate.rs` now treats crates.io HTTP 429 throttle responses ("You have published too many versions of this crate in the last 24 hours") as a deferred `publish_result=rate_limited` outcome (it writes the output, prints an explanatory banner and exits successfully) instead of a hard CI failure reported as a generic `failed` ("Failed to publish for unknown reason"). Authentication, already-published and unknown failures still exit non-zero. Failed-publish classification is consolidated through a single `classify_failure` function and `FailureKind` enum (with an `is_deferred` predicate), covered by unit tests runnable via `rust-script --test scripts/publish-crate.rs`.
- The release workflow (`.github/workflows/release.yml`) now gates crate-availability waiting, Docker Hub publishing and GitHub release creation on either an already-published crate or `publish_result=success`, so a deferred (rate-limited) crate upload no longer produces partial downstream release artifacts and the same version is retried automatically on the next push to `main`.

### Fixed
- Fixed reversed `cancel-in-progress` concurrency condition in `release.yml` that cancelled in-flight releases on `main` and never superseded older PR runs. The condition now uses `!=` so `main` releases run to completion while newer PR pushes cancel stale runs.

### Added
- Added a `scripts/check-crate-size.rs` guard that builds the `.crate` archive and fails the release before publishing when it exceeds the crates.io 10 MiB upload limit. The check runs in the build job and before publishing in both the auto-release and manual-release jobs.

### Changed
- Added a narrow `include` allowlist to `Cargo.toml` so docs, case studies, generated CI artifacts, changelog fragments, scripts, and experiments no longer inflate the published release archive.

### Fixed
- Prevented GitHub release creation from treating generic API validation failures as existing releases, and capped oversized release notes with a link to the full tagged changelog.

### Added
- Added the initial `meta-language` links-network core with self-description roots,
  verification, trivia attachment configuration, and a minimal CLI.
- Added the default lossless `parse` entry point, projection views, and a
  tested parity roadmap for competitor features, grammar embedding, and language
  coverage targets.
- Added exact reconstruction, mixed-region detection, query matching,
  substitution rules, concept reconstruction, object identity helpers,
  many-valued truth values, and executable parity fixtures for every tracked
  competitor target.
- Added executable language fixtures and tests for every requested Markdown,
  HTML, top-ten programming-language, and top-ten natural-language target.

### Fixed
- Added a CI guard that rejects Rust test modules and test attributes under `src/`, keeping tests in the `tests/` tree.

## Minor Changes

- Add immutable and mutable network snapshots with provenance and forward
  version commits for roadmap snapshot/versioning coverage.

### Added
- Added `txt` as a first-class markup/container target with a UTF-8 lossless
  fixture and content-sniffing fallback regions.

### Fixed
- Reconciled the natural-language target ordering with the Ethnologue 2025
  total-speaker order cited by the parity roadmap.

### Added
- Added a grammar-backed parser front end for Python, C, Java, C++, C#, JavaScript, and R using official tree-sitter grammar crates.

### Added
- Parse supported mixed-mode embedded regions into the host links network with grammar-backed syntax links.

### Added
- Added natural-language segmentation, identification, normalization, and bidi annotation links over lossless text parses.

### Added
- Added grammar-backed Visual Basic parsing with byte-exact reconstruction and recovery flag coverage.

bump: minor

- Add a grammar-backed `sql-ansi` SQL-family dialect fixture using
  `tree-sitter-sequel`.

### Added
- Added grammar-backed Delphi/Object Pascal parsing through `tree-sitter-pascal`.

### Added
- Enriched link queries with S-expression structural matching, captures, by-type construction, host predicate hooks, and link-cli-style variable substitution bindings.

### Added
- Added persistent snapshot structural sharing with interned metadata text storage.

### Added
- Added a query-transform surface for selecting captured links and replacing
  their source text while preserving unchanged bytes.

### Added
- Materialized self-description roots as controlled links with complete root-definition closure and round-trippable `describe` output.

### Added
- Added common concept ontology seeding from meta-expression's semantic lexicon, with shared concept links, syntax mappings, and structural programming-language concepts.

### Added
- Added semantic cross-language reconstruction for the Hawaii statehood fixture,
  including English/Russian naturalization and configurable formalization levels.

### Added
- Expanded `PARITY_FIXTURES` with multiple provenance-tracked fixtures for tree-sitter, LibCST, Recast, jscodeshift, Rowan, cstree, and Roslyn, including executable recovery and query/transform expectations.

### Added

- Expanded `PARITY_FIXTURES` with upstream-provenanced internal ecosystem corpora for links-notation, link-cli, lino-objects-codec, relative-meta-logic, formal-ai, and meta-expression.

### Added

- Added structural LiNo parsing for links-notation doublets, triplets, named links, indented IDs, and self-references while preserving byte-exact reconstruction.

### Added
- Added fixed-point probabilistic truth values for relative-meta-logic-style
  semantic confidence evaluation.

### Added
- Added grammar-backed Go parsing via the official `tree-sitter-go` grammar
  (accepts `Go`, `go`, and `golang`) so `LinkNetwork::parse` emits real
  `LinkType::Syntax` concrete-syntax links with byte-exact reconstruction.

### Added
- `AccessMode { Mutable, ReadOnly }` setting on `ParseConfiguration`
  (`with_access_mode` / `access_mode`), defaulting to `Mutable` so existing
  callers are unaffected.
- `LinkNetwork::freeze` / `as_read_only` yielding a `ReadOnlyNetwork` view that
  exposes only `&self` operations (query, project, reconstruct, verify,
  serialize); mutators are unreachable at compile time because the view never
  hands out `&mut LinkNetwork`.
- `LinkNetwork::parse_engine`, returning an `EngineNetwork` handle that honours
  the configured access mode: read-only parsing returns the frozen form and
  `EngineNetwork::as_mutable` rejects mutation with a `ReadOnlyViolation`
  diagnostic.
- Snapshot interop: `NetworkSnapshot::as_read_only` / `from_read_only` reuse the
  snapshot's `Arc<LinkNetwork>`, so the frozen form composes with snapshot
  versioning instead of duplicating it.

### Added
- Added grammar-backed Ruby parsing through `tree-sitter-ruby`, so
  `LinkNetwork::parse(source, "Ruby", ...)` (and the `rb` alias) now emits real
  `LinkType::Syntax` links instead of falling back to lossless plain text.

bump: minor

- Add a grammar-backed `TypeScript` front end using `tree-sitter-typescript`,
  wiring the `typescript`/`ts` labels to `LANGUAGE_TYPESCRIPT` and the `tsx`
  label to `LANGUAGE_TSX` so `LinkNetwork::parse` emits real `LinkType::Syntax`
  links for TypeScript and TSX sources.

### Added
- Added grammar-backed parsing for seven data-exchange / interchange formats
  through `src/tree_sitter_adapter.rs`: JSON (`tree-sitter-json`), YAML
  (`tree-sitter-yaml`, accepts `yaml`/`yml`), TOML (`tree-sitter-toml-ng`), XML
  and DTD (`tree-sitter-xml`), INI (`tree-sitter-ini`), Protocol Buffers
  (`tree-sitter-proto`, accepts `protobuf`/`proto`/`Protocol Buffers`), and
  GraphQL (`tree-sitter-graphql`, accepts `graphql`/`gql`). Each parses into
  real `LinkType::Syntax` concrete-syntax links and reconstructs byte-for-byte.
- Added a `DATA_FORMAT_TARGETS` registry in `src/parity.rs` (with the new
  `LanguageFamily::DataFormat`) gated by parity tests, mirroring
  `MARKUP_LANGUAGE_TARGETS`.
- Added per-format UTF-8 `LANGUAGE_FIXTURES` round-trip entries plus
  recovery-case and mixed-region tests (a `json` fence embedded in Markdown
  parses into the host links network).
- Added lossless CSV and JSON5 parsers for the two formats whose published
  tree-sitter crates still pin the incompatible `tree-sitter ~0.20` runtime.
  CSV is validated with the Rust `csv` crate and JSON5 is validated with
  `json5_nodes`; both emit structured syntax links and reconstruct
  byte-for-byte.

### Documentation
- Documented the nine wired data-format parsers (parser, version, license, root
  node) in `docs/parity-roadmap.md`, including the tree-sitter compatibility
  rationale for the in-repo CSV and JSON5 parsers.

### Added
- `LinkNetwork::to_lino` serializes an entire links network to canonical
  links-notation text, keyed by each link's numeric id (doublets-style id
  discipline), covering references, names, types, terms, definitions,
  languages, source spans, parse flags, and term registration.
- `LinkNetwork::from_lino` reconstructs the exact network from that text,
  forming a lossless round-trip (`from_lino(to_lino(n))` is isomorphic to `n`).
- `LinoSerializationError` reports parse and schema failures from `from_lino`.
- A round-trip property test over every language fixture plus synthetic
  networks, and a test that `to_lino` output is accepted by the
  `links-notation` 0.13 crate parser.
- `ParityCapability::LinoSerialization` and an output-side serialization
  parity fixture for the `links-notation` target.

### Changed
- Added the `links-notation` 0.13 crate as a dependency so serialized output
  aligns with the wider links-notation ecosystem.

### Added
- `ParserRegistry` maps language keys to `Arc<dyn LanguageParser>` with the
  built-in parser as a fallback. User registrations shadow the built-in
  dispatch for the same (case-insensitive) key; unregistered keys still route
  through the built-in set.
- `LinkNetwork::parse_with_registry` dispatches a parse through a registry.
- `examples/custom_parser_registry.rs` documents registering a custom parser.

### Added
- Added grammar-backed parsing for five popular programming languages
  immediately below the TIOBE top ten through `src/tree_sitter_adapter.rs`: PHP
  (`tree-sitter-php`, via its `LANGUAGE_PHP` symbol), Swift
  (`tree-sitter-swift`), Kotlin (`tree-sitter-kotlin-ng`, accepts `kotlin`/`kt`),
  Scala (`tree-sitter-scala`), and Lua (`tree-sitter-lua`). Each parses into real
  `LinkType::Syntax` concrete-syntax links and reconstructs byte-for-byte.
- Added a `SECOND_TIER_PROGRAMMING_LANGUAGE_TARGETS` registry in `src/parity.rs`
  gated by parity tests, mirroring `DATA_FORMAT_TARGETS`.
- Added per-language UTF-8 `LANGUAGE_FIXTURES` round-trip entries plus
  case-insensitive alias coverage and a per-language recovery fixture whose
  malformed source still reconstructs while exposing error/missing diagnostics.

### Documentation
- Documented the wired grammars (crate, version, license, root node) in
  `docs/parity-roadmap.md` and kept the temporary Perl follow-up tracked
  explicitly until `ts-parser-perl` was adopted.

### Added
- Add a Rust `ToLinks`/`FromLinks` codec with queryable type-shape links and shared/circular object graph round-trips through LiNo serialization.

### Added

- Added the `LinkStore` storage trait, read-only-aware storage wrappers, and an
  optional file-mapped `doublets` backend with LiNo/binary round-trip coverage.

### Added
- Added exact-match concept interning, language-bound expression links, external-id alias links, and LiNo concept-set import for the shared concept ontology.

### Added
- Added starter natural-language grammaticality parsing with UD-style morphosyntax links, pass/fail fixtures for the ten natural-language targets, and recoverable error links for ungrammatical fixtures.

### Added
- Added configurable `TranslationRuleSet` values, a runtime
  `TranslationRuleRegistry`, LiNo-backed rule-set loading, template
  placeholders, and missing-rule diagnostics for from-meta reconstruction.

### Added
- Added queryable language profiles with JavaScript transform enforcement and rule-set-derived profile domains.

### Added
- Added incremental source edit reparsing, stable outside-edit link IDs, and structural snapshot diff reporting.

### Added
- Added composable `LinkRule` query algebra with relational rules, boolean composition, named sub-rules, ellipsis gap matching, typed metavariables, plain-text token patterns, traversal strategies, and valid/invalid rule snapshot suites.
- Added quasiquote replacement templates with placeholder validation and parenthesization-conservative captured-text replacement.

### Added
- Add the API operation/style parity registry, fluent network pipeline, and link-cli-style substitution text runner.

### Added
- Wire the canonical `ts-parser-perl` tree-sitter grammar as a second-tier
  programming-language target without upgrading the project-wide tree-sitter
  runtime.

### Added
- Added source generation helpers for constructed syntax networks: `insert_source_token`, `insert_syntax_node`, `render_source`, `render_source_from`, and `render_source_from_document`.

### Added
- Added wave-two competitor and ecosystem parity fixtures with executable transform, reconstruction, grammar, and storage gates.
- Added a recorded `cargo llvm-cov` line-coverage floor to CI so coverage cannot silently regress.

### Added
- Shared, language-free document-formatting concept ontology (`document_formatting` module): inline concepts (emphasis, strong, strikethrough, inline-code, hyperlink, image, line-break) and block concepts (heading with level, paragraph, blockquote, bullet/ordered lists, list-item, code-block with language, thematic-break, table/row/cell), each seeded with Markdown and HTML syntax mappings.
- `seed_common_concept_ontology()` now also seeds the formatting concepts, and `ConceptOntologySeedReport::formatting_concepts()` reports how many were added.
- `LinkNetwork::resolve_document_format` / `render_document_format` / `translate_document_format` so the same concept link reconstructs as `**…**` in Markdown and `<strong>…</strong>` in HTML; Markdown `**bold**` and HTML `<strong>bold</strong>` reach the one shared `strong` concept under semantic projection.
- `FormattingDocument` concept layer with `parse_markup_document` and `translate_markup_document` for full Markdown ⇄ HTML document round-trips through one concept ontology (heading/paragraph/list/bold/italic/link).

### Added
- PDF document-format support (issue #84): a documented, uncompressed text PDF profile (`document_formatting::render_pdf_document` / `parse_pdf_document`) that renders a language-free `FormattingDocument` to a valid single-page PDF (correct `xref` offsets, object table, and stream `Length`) and parses it back into the same concept tree. Block role is carried by marked content (`/H1`…`/H6`, `/P`, `/UL`/`/OL`, `/LI`) and inline bold/italic by the selected font resource (`/F1` regular, `/F2` strong, `/F3` emphasis).
- `parse("…", "pdf", …)` dispatches to a new `pdf_parser` that builds a byte-exact lossless network (`reconstruct_text()` returns the input verbatim) and adds additive `Concept`/`Object` structure links recovering heading/paragraph/list/list-item and bold/italic.
- `reconstruct_text_as("PDF", …)` renders a structurally equivalent PDF through the shared formatting concept layer: a PDF source re-renders byte-for-byte, while a Markdown/HTML source is translated into an equivalent PDF, and `translate_markup_document` now bridges Markdown/HTML ⇄ PDF.
- `PDF` markup target in `MARKUP_LANGUAGE_TARGETS` with a bold + heading + paragraph round-trip `LANGUAGE_FIXTURES` entry, plus `docs/pdf-fidelity.md` documenting the round-trip fidelity matrix for supported and lossy/unsupported PDF features.

### Added
- DOCX (OOXML) document-format support (issue #85): a documented OOXML text profile (`document_formatting::render_docx_document` / `parse_docx_document`) that renders a language-free `FormattingDocument` to `word/document.xml` WordprocessingML and parses it back into the same concept tree. Block role is carried by paragraph properties (`<w:pStyle w:val="HeadingN"/>` headings, bare `<w:p>` paragraphs, `<w:numPr>` `numId` 1/2 bullet/ordered list items) and inline bold/italic by run properties (`<w:b/>` → `strong`, `<w:i/>` → `emphasis`).
- A binary OPC packaging layer (`document_formatting::render_docx_package` / `parse_docx_package`) that assembles a valid `.docx` ZIP (stored entries with a self-implemented CRC-32, no new dependencies) containing `[Content_Types].xml`, the relationship parts, `word/document.xml`, `word/styles.xml`, and `word/numbering.xml`, and reads `word/document.xml` back out.
- `parse("…", "docx", …)` dispatches to a new `docx_parser` that builds a byte-exact lossless network (`reconstruct_text()` returns the input verbatim) and adds additive `Concept`/`Object` structure links recovering heading/paragraph/list/list-item and bold/italic.
- `reconstruct_text_as("DOCX", …)` renders structurally equivalent OOXML through the shared formatting concept layer: a DOCX source re-renders byte-for-byte, while a Markdown/HTML/PDF source is translated into equivalent OOXML, and `translate_markup_document` now bridges Markdown/HTML/PDF ⇄ DOCX.
- `DOCX` markup target in `MARKUP_LANGUAGE_TARGETS` with a bold + heading + bullet-list round-trip `LANGUAGE_FIXTURES` entry, plus `docs/docx-fidelity.md` documenting the two-layer round-trip fidelity matrix for supported and lossy/unsupported OOXML features.

### Fixed
- Fixed the GitHub Pages site returning HTTP 404 at the root URL
  (<https://link-foundation.github.io/meta-language>). The deploy job uploaded
  raw `cargo doc` output, which has no root `index.html` (rustdoc only emits
  `target/doc/<crate>/index.html`), so the Pages root 404'd while docs were only
  reachable at `/meta_language/`. See issue #90.

### Added
- A project website with a landing page (description + docs links) under
  `docs/site/`, deployed at the Pages root.
- An interactive WebAssembly "Links Notation playground" demo in a new
  standalone `web/` crate (wraps the wasm-compatible `links-notation` crate),
  served at `/demo/`.
- `scripts/build-site.rs` to assemble the website (`_site/`) locally and in CI:
  landing page at `/`, demo at `/demo/`, and `rustdoc` under `/api/` with a root
  redirect.
- `docs/case-studies/issue-90` documenting the timeline, requirements, root
  cause analysis, and solution plans.

### Changed
- The `Deploy Website` CI job (formerly `Deploy Rust Documentation`) now builds
  the WebAssembly demo and assembles the full site instead of deploying
  `target/doc` directly.

### Added
- Cross-format document reconstruction and round-trip translation (issue #86): `reconstruct_text_as("txt" | "Markdown" | "HTML" | "PDF" | "DOCX", …)` now works over the shared, language-free formatting concept layer (issue #83), so a document parsed from any supported format reconstructs into any other when the source uses only concepts both formats support. A same-format target re-renders byte-for-byte; a cross-format target is translated through the concept tree, preserving heading/paragraph/list and bold/italic/link structure.
- `txt` joins Markdown, HTML, PDF, and DOCX as a first-class document format in `parse_markup_document` / `render_markup_document`: blank-line-separated paragraphs parse into the concept layer, and the concept layer flattens to plain text (headings to plain lines, lists to `- `/`N. ` markers, inline styling dropped) as the documented lossy fallback target.
- Per-format capability profiles (`document_format_profile`, `DOCUMENT_FORMATS`, `CROSS_FORMAT_CONCEPTS`, `canonical_document_format`) expose each format's `LanguageProfile` over the formatting concept ontology, reporting for every cross-format concept either native support or a documented lossy fallback rather than silent data loss.
- `LanguageProfile` gained `with_concept_fallback` / `concept_fallback` / `fallbacks` to declare and query the lossy fallback for concepts a target cannot represent natively.
- A round-trip matrix test covering every ordered pair of `{txt, Markdown, HTML, PDF, DOCX}` (a sample built from the concepts both formats share survives `A → concepts → B → concepts → A`), plus `docs/cross-format-fidelity.md` documenting the cross-format translation entry point and the per-format fidelity matrix.

### Added
- Case study and implementation backlog for grammar extensibility & inference (issue #93): `docs/case-studies/issue-93/` collects the requirement register, a grammar-inference literature review (Gold (1967), RPNI/EDSM, Sequitur, the GLADE→Arvada→TreeVada→Kedavra→NatGI line, ISLearn), a licence-vetted library/ecosystem survey, an existing-capability gap analysis, a competitive analysis pinning the metrics to beat, and per-epic solution plans with the issue dependency DAG. The 34 maximally-detailed sub-issue specs under `proposed-issues/` were filed as GitHub issues #95–#128, each attached as a native sub-issue of #93 with all 51 `blocked-by` dependency edges wired via the GitHub REST API. This is a planning/research deliverable only — no library code changes.

### Added
- Added a public grammar IR with expression builders and links round-trip encoding.

### Added
- Added grammar surface syntax parsing, writing, and LiNo bridge helpers for the grammar IR.

### Added
- Seed grammar-construct concepts and expose grammar expression concept alignment helpers.

### Added
- Added grammar subsystem documentation, README integration, and tests for the grammar docs page set and relative links.

### Added
- Added a public BNF grammar importer that lowers classic BNF productions into the grammar IR.

### Added
- Added an EBNF grammar importer that lowers ISO-style EBNF constructs into the grammar IR.

### Added
- Added an ABNF grammar importer with RFC 5234 core rules, RFC 7405 string sensitivity, numeric terminals, repetition, and incremental alternative lowering.

### Added
- Added a PEG `.pest` grammar importer backed by `pest_meta`.

### Added
- Add a tree-sitter `grammar.json` importer for lowering generated tree-sitter grammars into the grammar IR.

### Added
- Add a clean-room ANTLR v4 `.g4` importer for lowering grammar files into the grammar IR.

### Added
- Add clean-room Lark and GBNF grammar importers that lower into the grammar IR.

### Added
- Added public BNF, EBNF, and ABNF grammar emitters for the grammar IR.

### Added
- Add a public GBNF grammar emitter for LLM grammar-constraint interop.

### Added
- Added a public pest PEG grammar emitter for the grammar IR.

### Added
- Added Rust parser codegen artifacts for grammar IR, including pest derive stubs and AST type rendering.

### Added
- Added Peggy grammar and JavaScript parser module code generation for the grammar IR.

### Added
- Added a public tree-sitter `grammar.js` emitter for the grammar IR.

### Added

- Add state-merging regular inference with RPNI, EDSM, and ALERGIA.

### Added

- Add MDL/Occam minimization for inferred grammars.

### Added

- Add semantic constraint inference for grammar corpora

### Added
- Added concept-aligned grammar surface translation with deterministic rule-name, non-terminal, and doc-comment rewrites.

### Added
- Added a deterministic grammar inference evaluation harness with sampling, oracle scoring, MDL size metrics, and smoke corpus reports.

### Added
- Added deterministic lexical class inference with category-based tokenisation and grammar IR token rules.

### Added
- Added a Sequitur structural-compression inference pass that emits deterministic inferred grammar IR.

### Added
- Added a delimiter-skeleton structural prior API for positive grammar inference.

### Added
- Add deterministic CFG inference over delimiter structural priors with oracle-checked positive recall.

### Added
- Add deterministic and optional LLM-assisted grammar inference advisors for rule naming and merge ranking.

### Added
- Add a runtime `GrammarParser` and registry helpers for parsing with imported or inferred grammars.

### Added
- Add opt-in active regular-language learning with L*, DFA output, parser-backed oracles, and right-linear grammar lowering.

### Added
- Added CLI grammar subcommands for inference, import, emit, and concept-aligned translation.

### Added
- Add the E3 competitor corpus manifest, vendored benchmark fixtures, and a D1/D5 benchmark gate for included subjects.

### Added
- Added grammar-format fidelity profiles and the generated BNF round-trip matrix documentation.

### Added
- Added semantic grammar validation diagnostics for undefined references, left recursion, unreachable rules, nullable repetitions, duplicate rules, and unused captures.

### Added
- Added end-to-end grammar inference examples and integration tests for Rust, JavaScript, GBNF, and CLI emit pipelines.

## [0.45.0] - 2026-06-14

### Added
- Changeset-style fragment format with frontmatter for specifying version bump type
- New `get-bump-type.mjs` script to automatically determine version bump from fragments
- Automatic version bumping on merge to main based on changelog fragments
- Detailed documentation for the changelog fragment system in `changelog.d/README.md`

### Changed
- Updated `collect-changelog.mjs` to strip frontmatter when collecting fragments
- Updated `version-and-commit.mjs` to handle frontmatter in fragments
- Enhanced release workflow to automatically determine bump type from changesets

### Changed
- Add `detect-changes` job with cross-platform `detect-code-changes.mjs` script
- Make lint job independent of changelog check (runs based on file changes only)
- Allow docs-only PRs without changelog fragment requirement
- Handle changelog check 'skipped' state in dependent jobs
- Exclude `changelog.d/`, `docs/`, `experiments/`, `examples/` folders and markdown files from code changes detection

### Fixed
- Fixed README.md to correctly reference Node.js scripts (`.mjs`) instead of Python scripts (`.py`)
- Updated project structure in README.md to match actual script files in `scripts/` directory
- Fixed example code in README.md that had invalid Rust with two `main` functions

### Added

- Added crates.io publishing support to CI/CD workflow
- Added `release_mode` input with "instant" and "changelog-pr" options for manual releases
- Added `--tag-prefix` and `--crates-io-url` options to create-github-release.mjs script
- Added comprehensive case study documentation for Issue #11 in docs/case-studies/issue-11/

### Changed

- Changed changelog fragment check from warning to error (exit 1) to enforce changelog requirements
- Updated job conditions with `always() && !cancelled()` to fix workflow_dispatch job skipping issue
- Renamed manual-release job to "Instant Release" for clarity

### Fixed

- Fixed deprecated `::set-output` GitHub Actions command in version-and-commit.mjs
- Fixed workflow_dispatch triggering issues where lint/build/release jobs were incorrectly skipped

### Fixed

- Fixed changelog fragment check to validate that a fragment is **added in the PR diff** rather than just checking if any fragments exist in the directory. This prevents the check from incorrectly passing when there are leftover fragments from previous PRs that haven't been released yet.

### Changed

- Converted shell scripts in `release.yml` to cross-platform `.mjs` scripts for improved portability and performance:
  - `check-changelog-fragment.mjs` - validates changelog fragment is added in PR diff
  - `git-config.mjs` - configures git user for CI/CD
  - `check-release-needed.mjs` - checks if release is needed
  - `publish-crate.mjs` - publishes package to crates.io
  - `create-changelog-fragment.mjs` - creates changelog fragments for manual releases
  - `get-version.mjs` - gets current version from Cargo.toml

### Added

- Added `check-version-modification.mjs` script to detect manual version changes in Cargo.toml
- Added `version-check` job to CI/CD workflow that runs on pull requests
- Added skip logic for automated release branches (changelog-manual-release-*, changeset-release/*, release/*, automated-release/*)

### Changed

- Version modifications in Cargo.toml are now blocked in pull requests to enforce automated release pipeline

### Added

- Added support for `CARGO_REGISTRY_TOKEN` as alternative to `CARGO_TOKEN` for crates.io publishing
- Added case study documentation for Issue #17 (yargs reserved word and dual token support)

### Changed

- Updated workflow to use fallback logic: `${{ secrets.CARGO_REGISTRY_TOKEN || secrets.CARGO_TOKEN }}`
- Improved publish-crate.mjs to check both `CARGO_REGISTRY_TOKEN` and `CARGO_TOKEN` environment variables
- Added warning message when neither token is set

### Added
- New `scripts/rust-paths.mjs` utility for automatic Rust package root detection
- Support for both single-language and multi-language repository structures in all CI/CD scripts
- Configuration options via `--rust-root` CLI argument and `RUST_ROOT` environment variable
- Comprehensive case study documentation in `docs/case-studies/issue-19/`

### Changed
- Updated all release scripts to use the new path detection utility:
  - `scripts/bump-version.mjs`
  - `scripts/check-release-needed.mjs`
  - `scripts/collect-changelog.mjs`
  - `scripts/get-bump-type.mjs`
  - `scripts/get-version.mjs`
  - `scripts/publish-crate.mjs`
  - `scripts/version-and-commit.mjs`

### Changed

- **check-release-needed.mjs**: Now checks crates.io API directly instead of git tags to determine if a version is already released. This prevents false positives where git tags exist but the package was never actually published to crates.io.

### Added

- **CI/CD Troubleshooting Guide**: New documentation at `docs/ci-cd/troubleshooting.md` covering common issues like skipped jobs, false positive version checks, publishing failures, and secret configuration.

- **Enhanced Error Handling in publish-crate.mjs**: Added specific detection and helpful error messages for authentication failures, including guidance on secret configuration and workflow setup.

- **Case Study Documentation**: Added comprehensive case study at `docs/case-studies/issue-21/` analyzing CI/CD failures from browser-commander repository (issues #27, #29, #31, #33) with timeline, root causes, and lessons learned.

### Fixed

- **Prevent False Positive Version Checks**: The release workflow now correctly identifies unpublished versions by checking crates.io instead of relying on git tags, which can exist without the package being published.

### Changed

- Translated all CI/CD scripts from JavaScript (.mjs) to Rust (.rs) using rust-script
- Scripts now use native Rust with rust-script for execution in shell
- Removed Node.js dependency from CI/CD pipeline
- Updated GitHub Actions workflow to use rust-script instead of node
- Updated README and CONTRIBUTING documentation with new script references

### Added

- Cache `restore-keys` for partial cache hits across all workflow jobs
- Explicit `token` parameter in checkout for release jobs
- Code coverage job with `cargo-llvm-cov` and Codecov integration
- Codecov badge in README.md
- Pre-release version support (e.g., `0.1.0-beta.1`) in version parsing
- `--release-label` parameter for multi-language release disambiguation
- `ensure_version_exceeds_published()` logic to prevent publishing duplicate versions
- `get_max_published_version()` to query highest non-yanked version from crates.io
- `max_published_version` output from check-release-needed for downstream use
- Version fallback logic in auto-release Create GitHub Release step

### Changed

- Updated `actions/checkout` from v4 to v6
- Updated `actions/cache` from v4 to v5
- Updated `peter-evans/create-pull-request` from v7 to v8
- Made `publish-crate.rs` fail (exit 1) when version already exists on crates.io
- Improved `create-github-release.rs` to check combined stdout+stderr and detect "Validation Failed"

### Fixed

- Fix publish steps overriding workflow-level CARGO_TOKEN fallback, breaking CARGO_REGISTRY_TOKEN-only configurations (#32)
- Fix non-fast-forward push failures in multi-workflow repos by adding fetch/rebase and push retry logic (#31)
- Add mono-repo path support to check-changelog-fragment.rs, check-version-modification.rs, and create-changelog-fragment.rs
- Add `!cancelled()` guard to test job condition to respect workflow cancellation

### Fixed

- Fixed `version-and-commit.rs` to check crates.io instead of git tags for determining if a version is already released
- This prevents the release pipeline from getting stuck when git tags exist without corresponding crates.io publication

### Added

- Added `--tag-prefix` support to `version-and-commit.rs` for multi-language repository compatibility
- Added crates.io and docs.rs badges to README.md
- Added automatic crates.io and docs.rs badge injection in GitHub release notes
- Added documentation deployment job to CI/CD pipeline (deploys to GitHub Pages after release)
- Added case study documentation for issue #25

### Fixed
- Fixed unsupported look-ahead regex in `create-github-release.rs` that caused a panic when parsing CHANGELOG.md. Replaced with a two-step approach using only features supported by Rust's `regex` crate.

### Changed
- Restructured example application as a simple CLI sum calculator using `lino-arguments`
- Renamed default package to `example-sum-package-name` with Unlicense license
- Reorganized test structure: `tests/unit/sum.rs`, `tests/integration/sum.rs`, `tests/unit/ci-cd/`
- Converted experiment scripts into proper unit tests in `tests/unit/ci-cd/changelog_parsing.rs`
- Added CI/CD skip logic for template default package name `example-sum-package-name`
- Updated README.md badges and documentation

### Fixed

- Change detection script now uses per-commit diff instead of full PR diff, so commits touching only non-code files correctly skip CI jobs even when earlier commits in the same PR changed code files

### Fixed
- Make release scripts resolve the publishable crate manifest when the repository root uses a Cargo workspace manifest.

### Fixed
- Decoupled GitHub Pages documentation deployment from package release publication and fixed release-script warning failures under `RUSTFLAGS=-Dwarnings`.

### Changed
- Added explicit GitHub Actions job timeouts and documented Rust test timeout guidance.

### Fixed
- Added a non-blocking warning threshold to the Rust file-size check so near-limit files are surfaced before concurrent PR merges can exceed the hard limit.

### Fixed
- Made `create-github-release.rs` build GitHub release titles as `[Language] X.Y.Z` instead of reusing the tag prefix.

### Added
- Added optional Docker Hub image publishing tied to Rust crate releases, including crates.io visibility waiting, version/latest image tags, and Docker Hub badges in GitHub release notes.

### Changed
- Release completeness checks now self-heal when crates.io exists but configured Docker Hub or GitHub release artifacts are missing.

### Fixed
- Switched documentation deployment to the official GitHub Pages artifact workflow so repositories using GitHub Actions as their Pages source do not get false-positive branch-push deploys.

### Added
- Documented the one-time `Settings → Pages → Source = GitHub Actions` prerequisite for the `deploy-docs` job in `README.md` and as a comment above the `deploy-docs` job in `release.yml`, so downstream template users hit a documented setup step instead of a `Get Pages site failed` error on the first deploy.

### Added
- Tracking case study at `docs/case-studies/issue-52/` registering the `browser-commander` + Playwright preview-regeneration pattern from [`konard/vk-bot-desktop#52`](https://github.com/konard/vk-bot-desktop/pull/52), with an activation checklist for when an example-app surface lands in this template. Documentation only — no workflow, script, or runtime code changes. Primary upstream tracking issue: [`link-foundation/js-ai-driven-development-pipeline-template#62`](https://github.com/link-foundation/js-ai-driven-development-pipeline-template/issues/62).

### Fixed
- Release automation now keeps the workspace package entry in `Cargo.lock` synchronized when `scripts/version-and-commit.rs` bumps `Cargo.toml`, preventing stale lock-file version diffs in later pull requests.

### Fixed
- `scripts/publish-crate.rs` now treats crates.io HTTP 429 throttle responses ("You have published too many versions of this crate in the last 24 hours") as a deferred `publish_result=rate_limited` outcome (it writes the output, prints an explanatory banner and exits successfully) instead of a hard CI failure reported as a generic `failed` ("Failed to publish for unknown reason"). Authentication, already-published and unknown failures still exit non-zero. Failed-publish classification is consolidated through a single `classify_failure` function and `FailureKind` enum (with an `is_deferred` predicate), covered by unit tests runnable via `rust-script --test scripts/publish-crate.rs`.
- The release workflow (`.github/workflows/release.yml`) now gates crate-availability waiting, Docker Hub publishing and GitHub release creation on either an already-published crate or `publish_result=success`, so a deferred (rate-limited) crate upload no longer produces partial downstream release artifacts and the same version is retried automatically on the next push to `main`.

### Fixed
- Fixed reversed `cancel-in-progress` concurrency condition in `release.yml` that cancelled in-flight releases on `main` and never superseded older PR runs. The condition now uses `!=` so `main` releases run to completion while newer PR pushes cancel stale runs.

### Added
- Added a `scripts/check-crate-size.rs` guard that builds the `.crate` archive and fails the release before publishing when it exceeds the crates.io 10 MiB upload limit. The check runs in the build job and before publishing in both the auto-release and manual-release jobs.

### Changed
- Added a narrow `include` allowlist to `Cargo.toml` so docs, case studies, generated CI artifacts, changelog fragments, scripts, and experiments no longer inflate the published release archive.

### Fixed
- Prevented GitHub release creation from treating generic API validation failures as existing releases, and capped oversized release notes with a link to the full tagged changelog.

### Added
- Added the initial `meta-language` links-network core with self-description roots,
  verification, trivia attachment configuration, and a minimal CLI.
- Added the default lossless `parse` entry point, projection views, and a
  tested parity roadmap for competitor features, grammar embedding, and language
  coverage targets.
- Added exact reconstruction, mixed-region detection, query matching,
  substitution rules, concept reconstruction, object identity helpers,
  many-valued truth values, and executable parity fixtures for every tracked
  competitor target.
- Added executable language fixtures and tests for every requested Markdown,
  HTML, top-ten programming-language, and top-ten natural-language target.

### Fixed
- Added a CI guard that rejects Rust test modules and test attributes under `src/`, keeping tests in the `tests/` tree.

## Minor Changes

- Add immutable and mutable network snapshots with provenance and forward
  version commits for roadmap snapshot/versioning coverage.

### Added
- Added `txt` as a first-class markup/container target with a UTF-8 lossless
  fixture and content-sniffing fallback regions.

### Fixed
- Reconciled the natural-language target ordering with the Ethnologue 2025
  total-speaker order cited by the parity roadmap.

### Added
- Added a grammar-backed parser front end for Python, C, Java, C++, C#, JavaScript, and R using official tree-sitter grammar crates.

### Added
- Parse supported mixed-mode embedded regions into the host links network with grammar-backed syntax links.

### Added
- Added natural-language segmentation, identification, normalization, and bidi annotation links over lossless text parses.

### Added
- Added grammar-backed Visual Basic parsing with byte-exact reconstruction and recovery flag coverage.

bump: minor

- Add a grammar-backed `sql-ansi` SQL-family dialect fixture using
  `tree-sitter-sequel`.

### Added
- Added grammar-backed Delphi/Object Pascal parsing through `tree-sitter-pascal`.

### Added
- Enriched link queries with S-expression structural matching, captures, by-type construction, host predicate hooks, and link-cli-style variable substitution bindings.

### Added
- Added persistent snapshot structural sharing with interned metadata text storage.

### Added
- Added a query-transform surface for selecting captured links and replacing
  their source text while preserving unchanged bytes.

### Added
- Materialized self-description roots as controlled links with complete root-definition closure and round-trippable `describe` output.

### Added
- Added common concept ontology seeding from meta-expression's semantic lexicon, with shared concept links, syntax mappings, and structural programming-language concepts.

### Added
- Added semantic cross-language reconstruction for the Hawaii statehood fixture,
  including English/Russian naturalization and configurable formalization levels.

### Added
- Expanded `PARITY_FIXTURES` with multiple provenance-tracked fixtures for tree-sitter, LibCST, Recast, jscodeshift, Rowan, cstree, and Roslyn, including executable recovery and query/transform expectations.

### Added

- Expanded `PARITY_FIXTURES` with upstream-provenanced internal ecosystem corpora for links-notation, link-cli, lino-objects-codec, relative-meta-logic, formal-ai, and meta-expression.

### Added

- Added structural LiNo parsing for links-notation doublets, triplets, named links, indented IDs, and self-references while preserving byte-exact reconstruction.

### Added
- Added fixed-point probabilistic truth values for relative-meta-logic-style
  semantic confidence evaluation.

### Added
- Added grammar-backed Go parsing via the official `tree-sitter-go` grammar
  (accepts `Go`, `go`, and `golang`) so `LinkNetwork::parse` emits real
  `LinkType::Syntax` concrete-syntax links with byte-exact reconstruction.

### Added
- `AccessMode { Mutable, ReadOnly }` setting on `ParseConfiguration`
  (`with_access_mode` / `access_mode`), defaulting to `Mutable` so existing
  callers are unaffected.
- `LinkNetwork::freeze` / `as_read_only` yielding a `ReadOnlyNetwork` view that
  exposes only `&self` operations (query, project, reconstruct, verify,
  serialize); mutators are unreachable at compile time because the view never
  hands out `&mut LinkNetwork`.
- `LinkNetwork::parse_engine`, returning an `EngineNetwork` handle that honours
  the configured access mode: read-only parsing returns the frozen form and
  `EngineNetwork::as_mutable` rejects mutation with a `ReadOnlyViolation`
  diagnostic.
- Snapshot interop: `NetworkSnapshot::as_read_only` / `from_read_only` reuse the
  snapshot's `Arc<LinkNetwork>`, so the frozen form composes with snapshot
  versioning instead of duplicating it.

### Added
- Added grammar-backed Ruby parsing through `tree-sitter-ruby`, so
  `LinkNetwork::parse(source, "Ruby", ...)` (and the `rb` alias) now emits real
  `LinkType::Syntax` links instead of falling back to lossless plain text.

bump: minor

- Add a grammar-backed `TypeScript` front end using `tree-sitter-typescript`,
  wiring the `typescript`/`ts` labels to `LANGUAGE_TYPESCRIPT` and the `tsx`
  label to `LANGUAGE_TSX` so `LinkNetwork::parse` emits real `LinkType::Syntax`
  links for TypeScript and TSX sources.

### Added
- Added grammar-backed parsing for seven data-exchange / interchange formats
  through `src/tree_sitter_adapter.rs`: JSON (`tree-sitter-json`), YAML
  (`tree-sitter-yaml`, accepts `yaml`/`yml`), TOML (`tree-sitter-toml-ng`), XML
  and DTD (`tree-sitter-xml`), INI (`tree-sitter-ini`), Protocol Buffers
  (`tree-sitter-proto`, accepts `protobuf`/`proto`/`Protocol Buffers`), and
  GraphQL (`tree-sitter-graphql`, accepts `graphql`/`gql`). Each parses into
  real `LinkType::Syntax` concrete-syntax links and reconstructs byte-for-byte.
- Added a `DATA_FORMAT_TARGETS` registry in `src/parity.rs` (with the new
  `LanguageFamily::DataFormat`) gated by parity tests, mirroring
  `MARKUP_LANGUAGE_TARGETS`.
- Added per-format UTF-8 `LANGUAGE_FIXTURES` round-trip entries plus
  recovery-case and mixed-region tests (a `json` fence embedded in Markdown
  parses into the host links network).
- Added lossless CSV and JSON5 parsers for the two formats whose published
  tree-sitter crates still pin the incompatible `tree-sitter ~0.20` runtime.
  CSV is validated with the Rust `csv` crate and JSON5 is validated with
  `json5_nodes`; both emit structured syntax links and reconstruct
  byte-for-byte.

### Documentation
- Documented the nine wired data-format parsers (parser, version, license, root
  node) in `docs/parity-roadmap.md`, including the tree-sitter compatibility
  rationale for the in-repo CSV and JSON5 parsers.

### Added
- `LinkNetwork::to_lino` serializes an entire links network to canonical
  links-notation text, keyed by each link's numeric id (doublets-style id
  discipline), covering references, names, types, terms, definitions,
  languages, source spans, parse flags, and term registration.
- `LinkNetwork::from_lino` reconstructs the exact network from that text,
  forming a lossless round-trip (`from_lino(to_lino(n))` is isomorphic to `n`).
- `LinoSerializationError` reports parse and schema failures from `from_lino`.
- A round-trip property test over every language fixture plus synthetic
  networks, and a test that `to_lino` output is accepted by the
  `links-notation` 0.13 crate parser.
- `ParityCapability::LinoSerialization` and an output-side serialization
  parity fixture for the `links-notation` target.

### Changed
- Added the `links-notation` 0.13 crate as a dependency so serialized output
  aligns with the wider links-notation ecosystem.

### Added
- `ParserRegistry` maps language keys to `Arc<dyn LanguageParser>` with the
  built-in parser as a fallback. User registrations shadow the built-in
  dispatch for the same (case-insensitive) key; unregistered keys still route
  through the built-in set.
- `LinkNetwork::parse_with_registry` dispatches a parse through a registry.
- `examples/custom_parser_registry.rs` documents registering a custom parser.

### Added
- Added grammar-backed parsing for five popular programming languages
  immediately below the TIOBE top ten through `src/tree_sitter_adapter.rs`: PHP
  (`tree-sitter-php`, via its `LANGUAGE_PHP` symbol), Swift
  (`tree-sitter-swift`), Kotlin (`tree-sitter-kotlin-ng`, accepts `kotlin`/`kt`),
  Scala (`tree-sitter-scala`), and Lua (`tree-sitter-lua`). Each parses into real
  `LinkType::Syntax` concrete-syntax links and reconstructs byte-for-byte.
- Added a `SECOND_TIER_PROGRAMMING_LANGUAGE_TARGETS` registry in `src/parity.rs`
  gated by parity tests, mirroring `DATA_FORMAT_TARGETS`.
- Added per-language UTF-8 `LANGUAGE_FIXTURES` round-trip entries plus
  case-insensitive alias coverage and a per-language recovery fixture whose
  malformed source still reconstructs while exposing error/missing diagnostics.

### Documentation
- Documented the wired grammars (crate, version, license, root node) in
  `docs/parity-roadmap.md` and kept the temporary Perl follow-up tracked
  explicitly until `ts-parser-perl` was adopted.

### Added
- Add a Rust `ToLinks`/`FromLinks` codec with queryable type-shape links and shared/circular object graph round-trips through LiNo serialization.

### Added

- Added the `LinkStore` storage trait, read-only-aware storage wrappers, and an
  optional file-mapped `doublets` backend with LiNo/binary round-trip coverage.

### Added
- Added exact-match concept interning, language-bound expression links, external-id alias links, and LiNo concept-set import for the shared concept ontology.

### Added
- Added starter natural-language grammaticality parsing with UD-style morphosyntax links, pass/fail fixtures for the ten natural-language targets, and recoverable error links for ungrammatical fixtures.

### Added
- Added configurable `TranslationRuleSet` values, a runtime
  `TranslationRuleRegistry`, LiNo-backed rule-set loading, template
  placeholders, and missing-rule diagnostics for from-meta reconstruction.

### Added
- Added queryable language profiles with JavaScript transform enforcement and rule-set-derived profile domains.

### Added
- Added incremental source edit reparsing, stable outside-edit link IDs, and structural snapshot diff reporting.

### Added
- Added composable `LinkRule` query algebra with relational rules, boolean composition, named sub-rules, ellipsis gap matching, typed metavariables, plain-text token patterns, traversal strategies, and valid/invalid rule snapshot suites.
- Added quasiquote replacement templates with placeholder validation and parenthesization-conservative captured-text replacement.

### Added
- Add the API operation/style parity registry, fluent network pipeline, and link-cli-style substitution text runner.

### Added
- Wire the canonical `ts-parser-perl` tree-sitter grammar as a second-tier
  programming-language target without upgrading the project-wide tree-sitter
  runtime.

### Added
- Added source generation helpers for constructed syntax networks: `insert_source_token`, `insert_syntax_node`, `render_source`, `render_source_from`, and `render_source_from_document`.

### Added
- Added wave-two competitor and ecosystem parity fixtures with executable transform, reconstruction, grammar, and storage gates.
- Added a recorded `cargo llvm-cov` line-coverage floor to CI so coverage cannot silently regress.

### Added
- Shared, language-free document-formatting concept ontology (`document_formatting` module): inline concepts (emphasis, strong, strikethrough, inline-code, hyperlink, image, line-break) and block concepts (heading with level, paragraph, blockquote, bullet/ordered lists, list-item, code-block with language, thematic-break, table/row/cell), each seeded with Markdown and HTML syntax mappings.
- `seed_common_concept_ontology()` now also seeds the formatting concepts, and `ConceptOntologySeedReport::formatting_concepts()` reports how many were added.
- `LinkNetwork::resolve_document_format` / `render_document_format` / `translate_document_format` so the same concept link reconstructs as `**…**` in Markdown and `<strong>…</strong>` in HTML; Markdown `**bold**` and HTML `<strong>bold</strong>` reach the one shared `strong` concept under semantic projection.
- `FormattingDocument` concept layer with `parse_markup_document` and `translate_markup_document` for full Markdown ⇄ HTML document round-trips through one concept ontology (heading/paragraph/list/bold/italic/link).

### Added
- PDF document-format support (issue #84): a documented, uncompressed text PDF profile (`document_formatting::render_pdf_document` / `parse_pdf_document`) that renders a language-free `FormattingDocument` to a valid single-page PDF (correct `xref` offsets, object table, and stream `Length`) and parses it back into the same concept tree. Block role is carried by marked content (`/H1`…`/H6`, `/P`, `/UL`/`/OL`, `/LI`) and inline bold/italic by the selected font resource (`/F1` regular, `/F2` strong, `/F3` emphasis).
- `parse("…", "pdf", …)` dispatches to a new `pdf_parser` that builds a byte-exact lossless network (`reconstruct_text()` returns the input verbatim) and adds additive `Concept`/`Object` structure links recovering heading/paragraph/list/list-item and bold/italic.
- `reconstruct_text_as("PDF", …)` renders a structurally equivalent PDF through the shared formatting concept layer: a PDF source re-renders byte-for-byte, while a Markdown/HTML source is translated into an equivalent PDF, and `translate_markup_document` now bridges Markdown/HTML ⇄ PDF.
- `PDF` markup target in `MARKUP_LANGUAGE_TARGETS` with a bold + heading + paragraph round-trip `LANGUAGE_FIXTURES` entry, plus `docs/pdf-fidelity.md` documenting the round-trip fidelity matrix for supported and lossy/unsupported PDF features.

### Added
- DOCX (OOXML) document-format support (issue #85): a documented OOXML text profile (`document_formatting::render_docx_document` / `parse_docx_document`) that renders a language-free `FormattingDocument` to `word/document.xml` WordprocessingML and parses it back into the same concept tree. Block role is carried by paragraph properties (`<w:pStyle w:val="HeadingN"/>` headings, bare `<w:p>` paragraphs, `<w:numPr>` `numId` 1/2 bullet/ordered list items) and inline bold/italic by run properties (`<w:b/>` → `strong`, `<w:i/>` → `emphasis`).
- A binary OPC packaging layer (`document_formatting::render_docx_package` / `parse_docx_package`) that assembles a valid `.docx` ZIP (stored entries with a self-implemented CRC-32, no new dependencies) containing `[Content_Types].xml`, the relationship parts, `word/document.xml`, `word/styles.xml`, and `word/numbering.xml`, and reads `word/document.xml` back out.
- `parse("…", "docx", …)` dispatches to a new `docx_parser` that builds a byte-exact lossless network (`reconstruct_text()` returns the input verbatim) and adds additive `Concept`/`Object` structure links recovering heading/paragraph/list/list-item and bold/italic.
- `reconstruct_text_as("DOCX", …)` renders structurally equivalent OOXML through the shared formatting concept layer: a DOCX source re-renders byte-for-byte, while a Markdown/HTML/PDF source is translated into equivalent OOXML, and `translate_markup_document` now bridges Markdown/HTML/PDF ⇄ DOCX.
- `DOCX` markup target in `MARKUP_LANGUAGE_TARGETS` with a bold + heading + bullet-list round-trip `LANGUAGE_FIXTURES` entry, plus `docs/docx-fidelity.md` documenting the two-layer round-trip fidelity matrix for supported and lossy/unsupported OOXML features.

### Fixed
- Fixed the GitHub Pages site returning HTTP 404 at the root URL
  (<https://link-foundation.github.io/meta-language>). The deploy job uploaded
  raw `cargo doc` output, which has no root `index.html` (rustdoc only emits
  `target/doc/<crate>/index.html`), so the Pages root 404'd while docs were only
  reachable at `/meta_language/`. See issue #90.

### Added
- A project website with a landing page (description + docs links) under
  `docs/site/`, deployed at the Pages root.
- An interactive WebAssembly "Links Notation playground" demo in a new
  standalone `web/` crate (wraps the wasm-compatible `links-notation` crate),
  served at `/demo/`.
- `scripts/build-site.rs` to assemble the website (`_site/`) locally and in CI:
  landing page at `/`, demo at `/demo/`, and `rustdoc` under `/api/` with a root
  redirect.
- `docs/case-studies/issue-90` documenting the timeline, requirements, root
  cause analysis, and solution plans.

### Changed
- The `Deploy Website` CI job (formerly `Deploy Rust Documentation`) now builds
  the WebAssembly demo and assembles the full site instead of deploying
  `target/doc` directly.

### Added
- Cross-format document reconstruction and round-trip translation (issue #86): `reconstruct_text_as("txt" | "Markdown" | "HTML" | "PDF" | "DOCX", …)` now works over the shared, language-free formatting concept layer (issue #83), so a document parsed from any supported format reconstructs into any other when the source uses only concepts both formats support. A same-format target re-renders byte-for-byte; a cross-format target is translated through the concept tree, preserving heading/paragraph/list and bold/italic/link structure.
- `txt` joins Markdown, HTML, PDF, and DOCX as a first-class document format in `parse_markup_document` / `render_markup_document`: blank-line-separated paragraphs parse into the concept layer, and the concept layer flattens to plain text (headings to plain lines, lists to `- `/`N. ` markers, inline styling dropped) as the documented lossy fallback target.
- Per-format capability profiles (`document_format_profile`, `DOCUMENT_FORMATS`, `CROSS_FORMAT_CONCEPTS`, `canonical_document_format`) expose each format's `LanguageProfile` over the formatting concept ontology, reporting for every cross-format concept either native support or a documented lossy fallback rather than silent data loss.
- `LanguageProfile` gained `with_concept_fallback` / `concept_fallback` / `fallbacks` to declare and query the lossy fallback for concepts a target cannot represent natively.
- A round-trip matrix test covering every ordered pair of `{txt, Markdown, HTML, PDF, DOCX}` (a sample built from the concepts both formats share survives `A → concepts → B → concepts → A`), plus `docs/cross-format-fidelity.md` documenting the cross-format translation entry point and the per-format fidelity matrix.

## [0.44.0] - 2026-06-14

### Added
- Changeset-style fragment format with frontmatter for specifying version bump type
- New `get-bump-type.mjs` script to automatically determine version bump from fragments
- Automatic version bumping on merge to main based on changelog fragments
- Detailed documentation for the changelog fragment system in `changelog.d/README.md`

### Changed
- Updated `collect-changelog.mjs` to strip frontmatter when collecting fragments
- Updated `version-and-commit.mjs` to handle frontmatter in fragments
- Enhanced release workflow to automatically determine bump type from changesets

### Changed
- Add `detect-changes` job with cross-platform `detect-code-changes.mjs` script
- Make lint job independent of changelog check (runs based on file changes only)
- Allow docs-only PRs without changelog fragment requirement
- Handle changelog check 'skipped' state in dependent jobs
- Exclude `changelog.d/`, `docs/`, `experiments/`, `examples/` folders and markdown files from code changes detection

### Fixed
- Fixed README.md to correctly reference Node.js scripts (`.mjs`) instead of Python scripts (`.py`)
- Updated project structure in README.md to match actual script files in `scripts/` directory
- Fixed example code in README.md that had invalid Rust with two `main` functions

### Added

- Added crates.io publishing support to CI/CD workflow
- Added `release_mode` input with "instant" and "changelog-pr" options for manual releases
- Added `--tag-prefix` and `--crates-io-url` options to create-github-release.mjs script
- Added comprehensive case study documentation for Issue #11 in docs/case-studies/issue-11/

### Changed

- Changed changelog fragment check from warning to error (exit 1) to enforce changelog requirements
- Updated job conditions with `always() && !cancelled()` to fix workflow_dispatch job skipping issue
- Renamed manual-release job to "Instant Release" for clarity

### Fixed

- Fixed deprecated `::set-output` GitHub Actions command in version-and-commit.mjs
- Fixed workflow_dispatch triggering issues where lint/build/release jobs were incorrectly skipped

### Fixed

- Fixed changelog fragment check to validate that a fragment is **added in the PR diff** rather than just checking if any fragments exist in the directory. This prevents the check from incorrectly passing when there are leftover fragments from previous PRs that haven't been released yet.

### Changed

- Converted shell scripts in `release.yml` to cross-platform `.mjs` scripts for improved portability and performance:
  - `check-changelog-fragment.mjs` - validates changelog fragment is added in PR diff
  - `git-config.mjs` - configures git user for CI/CD
  - `check-release-needed.mjs` - checks if release is needed
  - `publish-crate.mjs` - publishes package to crates.io
  - `create-changelog-fragment.mjs` - creates changelog fragments for manual releases
  - `get-version.mjs` - gets current version from Cargo.toml

### Added

- Added `check-version-modification.mjs` script to detect manual version changes in Cargo.toml
- Added `version-check` job to CI/CD workflow that runs on pull requests
- Added skip logic for automated release branches (changelog-manual-release-*, changeset-release/*, release/*, automated-release/*)

### Changed

- Version modifications in Cargo.toml are now blocked in pull requests to enforce automated release pipeline

### Added

- Added support for `CARGO_REGISTRY_TOKEN` as alternative to `CARGO_TOKEN` for crates.io publishing
- Added case study documentation for Issue #17 (yargs reserved word and dual token support)

### Changed

- Updated workflow to use fallback logic: `${{ secrets.CARGO_REGISTRY_TOKEN || secrets.CARGO_TOKEN }}`
- Improved publish-crate.mjs to check both `CARGO_REGISTRY_TOKEN` and `CARGO_TOKEN` environment variables
- Added warning message when neither token is set

### Added
- New `scripts/rust-paths.mjs` utility for automatic Rust package root detection
- Support for both single-language and multi-language repository structures in all CI/CD scripts
- Configuration options via `--rust-root` CLI argument and `RUST_ROOT` environment variable
- Comprehensive case study documentation in `docs/case-studies/issue-19/`

### Changed
- Updated all release scripts to use the new path detection utility:
  - `scripts/bump-version.mjs`
  - `scripts/check-release-needed.mjs`
  - `scripts/collect-changelog.mjs`
  - `scripts/get-bump-type.mjs`
  - `scripts/get-version.mjs`
  - `scripts/publish-crate.mjs`
  - `scripts/version-and-commit.mjs`

### Changed

- **check-release-needed.mjs**: Now checks crates.io API directly instead of git tags to determine if a version is already released. This prevents false positives where git tags exist but the package was never actually published to crates.io.

### Added

- **CI/CD Troubleshooting Guide**: New documentation at `docs/ci-cd/troubleshooting.md` covering common issues like skipped jobs, false positive version checks, publishing failures, and secret configuration.

- **Enhanced Error Handling in publish-crate.mjs**: Added specific detection and helpful error messages for authentication failures, including guidance on secret configuration and workflow setup.

- **Case Study Documentation**: Added comprehensive case study at `docs/case-studies/issue-21/` analyzing CI/CD failures from browser-commander repository (issues #27, #29, #31, #33) with timeline, root causes, and lessons learned.

### Fixed

- **Prevent False Positive Version Checks**: The release workflow now correctly identifies unpublished versions by checking crates.io instead of relying on git tags, which can exist without the package being published.

### Changed

- Translated all CI/CD scripts from JavaScript (.mjs) to Rust (.rs) using rust-script
- Scripts now use native Rust with rust-script for execution in shell
- Removed Node.js dependency from CI/CD pipeline
- Updated GitHub Actions workflow to use rust-script instead of node
- Updated README and CONTRIBUTING documentation with new script references

### Added

- Cache `restore-keys` for partial cache hits across all workflow jobs
- Explicit `token` parameter in checkout for release jobs
- Code coverage job with `cargo-llvm-cov` and Codecov integration
- Codecov badge in README.md
- Pre-release version support (e.g., `0.1.0-beta.1`) in version parsing
- `--release-label` parameter for multi-language release disambiguation
- `ensure_version_exceeds_published()` logic to prevent publishing duplicate versions
- `get_max_published_version()` to query highest non-yanked version from crates.io
- `max_published_version` output from check-release-needed for downstream use
- Version fallback logic in auto-release Create GitHub Release step

### Changed

- Updated `actions/checkout` from v4 to v6
- Updated `actions/cache` from v4 to v5
- Updated `peter-evans/create-pull-request` from v7 to v8
- Made `publish-crate.rs` fail (exit 1) when version already exists on crates.io
- Improved `create-github-release.rs` to check combined stdout+stderr and detect "Validation Failed"

### Fixed

- Fix publish steps overriding workflow-level CARGO_TOKEN fallback, breaking CARGO_REGISTRY_TOKEN-only configurations (#32)
- Fix non-fast-forward push failures in multi-workflow repos by adding fetch/rebase and push retry logic (#31)
- Add mono-repo path support to check-changelog-fragment.rs, check-version-modification.rs, and create-changelog-fragment.rs
- Add `!cancelled()` guard to test job condition to respect workflow cancellation

### Fixed

- Fixed `version-and-commit.rs` to check crates.io instead of git tags for determining if a version is already released
- This prevents the release pipeline from getting stuck when git tags exist without corresponding crates.io publication

### Added

- Added `--tag-prefix` support to `version-and-commit.rs` for multi-language repository compatibility
- Added crates.io and docs.rs badges to README.md
- Added automatic crates.io and docs.rs badge injection in GitHub release notes
- Added documentation deployment job to CI/CD pipeline (deploys to GitHub Pages after release)
- Added case study documentation for issue #25

### Fixed
- Fixed unsupported look-ahead regex in `create-github-release.rs` that caused a panic when parsing CHANGELOG.md. Replaced with a two-step approach using only features supported by Rust's `regex` crate.

### Changed
- Restructured example application as a simple CLI sum calculator using `lino-arguments`
- Renamed default package to `example-sum-package-name` with Unlicense license
- Reorganized test structure: `tests/unit/sum.rs`, `tests/integration/sum.rs`, `tests/unit/ci-cd/`
- Converted experiment scripts into proper unit tests in `tests/unit/ci-cd/changelog_parsing.rs`
- Added CI/CD skip logic for template default package name `example-sum-package-name`
- Updated README.md badges and documentation

### Fixed

- Change detection script now uses per-commit diff instead of full PR diff, so commits touching only non-code files correctly skip CI jobs even when earlier commits in the same PR changed code files

### Fixed
- Make release scripts resolve the publishable crate manifest when the repository root uses a Cargo workspace manifest.

### Fixed
- Decoupled GitHub Pages documentation deployment from package release publication and fixed release-script warning failures under `RUSTFLAGS=-Dwarnings`.

### Changed
- Added explicit GitHub Actions job timeouts and documented Rust test timeout guidance.

### Fixed
- Added a non-blocking warning threshold to the Rust file-size check so near-limit files are surfaced before concurrent PR merges can exceed the hard limit.

### Fixed
- Made `create-github-release.rs` build GitHub release titles as `[Language] X.Y.Z` instead of reusing the tag prefix.

### Added
- Added optional Docker Hub image publishing tied to Rust crate releases, including crates.io visibility waiting, version/latest image tags, and Docker Hub badges in GitHub release notes.

### Changed
- Release completeness checks now self-heal when crates.io exists but configured Docker Hub or GitHub release artifacts are missing.

### Fixed
- Switched documentation deployment to the official GitHub Pages artifact workflow so repositories using GitHub Actions as their Pages source do not get false-positive branch-push deploys.

### Added
- Documented the one-time `Settings → Pages → Source = GitHub Actions` prerequisite for the `deploy-docs` job in `README.md` and as a comment above the `deploy-docs` job in `release.yml`, so downstream template users hit a documented setup step instead of a `Get Pages site failed` error on the first deploy.

### Added
- Tracking case study at `docs/case-studies/issue-52/` registering the `browser-commander` + Playwright preview-regeneration pattern from [`konard/vk-bot-desktop#52`](https://github.com/konard/vk-bot-desktop/pull/52), with an activation checklist for when an example-app surface lands in this template. Documentation only — no workflow, script, or runtime code changes. Primary upstream tracking issue: [`link-foundation/js-ai-driven-development-pipeline-template#62`](https://github.com/link-foundation/js-ai-driven-development-pipeline-template/issues/62).

### Fixed
- Release automation now keeps the workspace package entry in `Cargo.lock` synchronized when `scripts/version-and-commit.rs` bumps `Cargo.toml`, preventing stale lock-file version diffs in later pull requests.

### Fixed
- `scripts/publish-crate.rs` now treats crates.io HTTP 429 throttle responses ("You have published too many versions of this crate in the last 24 hours") as a deferred `publish_result=rate_limited` outcome (it writes the output, prints an explanatory banner and exits successfully) instead of a hard CI failure reported as a generic `failed` ("Failed to publish for unknown reason"). Authentication, already-published and unknown failures still exit non-zero. Failed-publish classification is consolidated through a single `classify_failure` function and `FailureKind` enum (with an `is_deferred` predicate), covered by unit tests runnable via `rust-script --test scripts/publish-crate.rs`.
- The release workflow (`.github/workflows/release.yml`) now gates crate-availability waiting, Docker Hub publishing and GitHub release creation on either an already-published crate or `publish_result=success`, so a deferred (rate-limited) crate upload no longer produces partial downstream release artifacts and the same version is retried automatically on the next push to `main`.

### Fixed
- Fixed reversed `cancel-in-progress` concurrency condition in `release.yml` that cancelled in-flight releases on `main` and never superseded older PR runs. The condition now uses `!=` so `main` releases run to completion while newer PR pushes cancel stale runs.

### Added
- Added a `scripts/check-crate-size.rs` guard that builds the `.crate` archive and fails the release before publishing when it exceeds the crates.io 10 MiB upload limit. The check runs in the build job and before publishing in both the auto-release and manual-release jobs.

### Changed
- Added a narrow `include` allowlist to `Cargo.toml` so docs, case studies, generated CI artifacts, changelog fragments, scripts, and experiments no longer inflate the published release archive.

### Fixed
- Prevented GitHub release creation from treating generic API validation failures as existing releases, and capped oversized release notes with a link to the full tagged changelog.

### Added
- Added the initial `meta-language` links-network core with self-description roots,
  verification, trivia attachment configuration, and a minimal CLI.
- Added the default lossless `parse` entry point, projection views, and a
  tested parity roadmap for competitor features, grammar embedding, and language
  coverage targets.
- Added exact reconstruction, mixed-region detection, query matching,
  substitution rules, concept reconstruction, object identity helpers,
  many-valued truth values, and executable parity fixtures for every tracked
  competitor target.
- Added executable language fixtures and tests for every requested Markdown,
  HTML, top-ten programming-language, and top-ten natural-language target.

### Fixed
- Added a CI guard that rejects Rust test modules and test attributes under `src/`, keeping tests in the `tests/` tree.

## Minor Changes

- Add immutable and mutable network snapshots with provenance and forward
  version commits for roadmap snapshot/versioning coverage.

### Added
- Added `txt` as a first-class markup/container target with a UTF-8 lossless
  fixture and content-sniffing fallback regions.

### Fixed
- Reconciled the natural-language target ordering with the Ethnologue 2025
  total-speaker order cited by the parity roadmap.

### Added
- Added a grammar-backed parser front end for Python, C, Java, C++, C#, JavaScript, and R using official tree-sitter grammar crates.

### Added
- Parse supported mixed-mode embedded regions into the host links network with grammar-backed syntax links.

### Added
- Added natural-language segmentation, identification, normalization, and bidi annotation links over lossless text parses.

### Added
- Added grammar-backed Visual Basic parsing with byte-exact reconstruction and recovery flag coverage.

bump: minor

- Add a grammar-backed `sql-ansi` SQL-family dialect fixture using
  `tree-sitter-sequel`.

### Added
- Added grammar-backed Delphi/Object Pascal parsing through `tree-sitter-pascal`.

### Added
- Enriched link queries with S-expression structural matching, captures, by-type construction, host predicate hooks, and link-cli-style variable substitution bindings.

### Added
- Added persistent snapshot structural sharing with interned metadata text storage.

### Added
- Added a query-transform surface for selecting captured links and replacing
  their source text while preserving unchanged bytes.

### Added
- Materialized self-description roots as controlled links with complete root-definition closure and round-trippable `describe` output.

### Added
- Added common concept ontology seeding from meta-expression's semantic lexicon, with shared concept links, syntax mappings, and structural programming-language concepts.

### Added
- Added semantic cross-language reconstruction for the Hawaii statehood fixture,
  including English/Russian naturalization and configurable formalization levels.

### Added
- Expanded `PARITY_FIXTURES` with multiple provenance-tracked fixtures for tree-sitter, LibCST, Recast, jscodeshift, Rowan, cstree, and Roslyn, including executable recovery and query/transform expectations.

### Added

- Expanded `PARITY_FIXTURES` with upstream-provenanced internal ecosystem corpora for links-notation, link-cli, lino-objects-codec, relative-meta-logic, formal-ai, and meta-expression.

### Added

- Added structural LiNo parsing for links-notation doublets, triplets, named links, indented IDs, and self-references while preserving byte-exact reconstruction.

### Added
- Added fixed-point probabilistic truth values for relative-meta-logic-style
  semantic confidence evaluation.

### Added
- Added grammar-backed Go parsing via the official `tree-sitter-go` grammar
  (accepts `Go`, `go`, and `golang`) so `LinkNetwork::parse` emits real
  `LinkType::Syntax` concrete-syntax links with byte-exact reconstruction.

### Added
- `AccessMode { Mutable, ReadOnly }` setting on `ParseConfiguration`
  (`with_access_mode` / `access_mode`), defaulting to `Mutable` so existing
  callers are unaffected.
- `LinkNetwork::freeze` / `as_read_only` yielding a `ReadOnlyNetwork` view that
  exposes only `&self` operations (query, project, reconstruct, verify,
  serialize); mutators are unreachable at compile time because the view never
  hands out `&mut LinkNetwork`.
- `LinkNetwork::parse_engine`, returning an `EngineNetwork` handle that honours
  the configured access mode: read-only parsing returns the frozen form and
  `EngineNetwork::as_mutable` rejects mutation with a `ReadOnlyViolation`
  diagnostic.
- Snapshot interop: `NetworkSnapshot::as_read_only` / `from_read_only` reuse the
  snapshot's `Arc<LinkNetwork>`, so the frozen form composes with snapshot
  versioning instead of duplicating it.

### Added
- Added grammar-backed Ruby parsing through `tree-sitter-ruby`, so
  `LinkNetwork::parse(source, "Ruby", ...)` (and the `rb` alias) now emits real
  `LinkType::Syntax` links instead of falling back to lossless plain text.

bump: minor

- Add a grammar-backed `TypeScript` front end using `tree-sitter-typescript`,
  wiring the `typescript`/`ts` labels to `LANGUAGE_TYPESCRIPT` and the `tsx`
  label to `LANGUAGE_TSX` so `LinkNetwork::parse` emits real `LinkType::Syntax`
  links for TypeScript and TSX sources.

### Added
- Added grammar-backed parsing for seven data-exchange / interchange formats
  through `src/tree_sitter_adapter.rs`: JSON (`tree-sitter-json`), YAML
  (`tree-sitter-yaml`, accepts `yaml`/`yml`), TOML (`tree-sitter-toml-ng`), XML
  and DTD (`tree-sitter-xml`), INI (`tree-sitter-ini`), Protocol Buffers
  (`tree-sitter-proto`, accepts `protobuf`/`proto`/`Protocol Buffers`), and
  GraphQL (`tree-sitter-graphql`, accepts `graphql`/`gql`). Each parses into
  real `LinkType::Syntax` concrete-syntax links and reconstructs byte-for-byte.
- Added a `DATA_FORMAT_TARGETS` registry in `src/parity.rs` (with the new
  `LanguageFamily::DataFormat`) gated by parity tests, mirroring
  `MARKUP_LANGUAGE_TARGETS`.
- Added per-format UTF-8 `LANGUAGE_FIXTURES` round-trip entries plus
  recovery-case and mixed-region tests (a `json` fence embedded in Markdown
  parses into the host links network).
- Added lossless CSV and JSON5 parsers for the two formats whose published
  tree-sitter crates still pin the incompatible `tree-sitter ~0.20` runtime.
  CSV is validated with the Rust `csv` crate and JSON5 is validated with
  `json5_nodes`; both emit structured syntax links and reconstruct
  byte-for-byte.

### Documentation
- Documented the nine wired data-format parsers (parser, version, license, root
  node) in `docs/parity-roadmap.md`, including the tree-sitter compatibility
  rationale for the in-repo CSV and JSON5 parsers.

### Added
- `LinkNetwork::to_lino` serializes an entire links network to canonical
  links-notation text, keyed by each link's numeric id (doublets-style id
  discipline), covering references, names, types, terms, definitions,
  languages, source spans, parse flags, and term registration.
- `LinkNetwork::from_lino` reconstructs the exact network from that text,
  forming a lossless round-trip (`from_lino(to_lino(n))` is isomorphic to `n`).
- `LinoSerializationError` reports parse and schema failures from `from_lino`.
- A round-trip property test over every language fixture plus synthetic
  networks, and a test that `to_lino` output is accepted by the
  `links-notation` 0.13 crate parser.
- `ParityCapability::LinoSerialization` and an output-side serialization
  parity fixture for the `links-notation` target.

### Changed
- Added the `links-notation` 0.13 crate as a dependency so serialized output
  aligns with the wider links-notation ecosystem.

### Added
- `ParserRegistry` maps language keys to `Arc<dyn LanguageParser>` with the
  built-in parser as a fallback. User registrations shadow the built-in
  dispatch for the same (case-insensitive) key; unregistered keys still route
  through the built-in set.
- `LinkNetwork::parse_with_registry` dispatches a parse through a registry.
- `examples/custom_parser_registry.rs` documents registering a custom parser.

### Added
- Added grammar-backed parsing for five popular programming languages
  immediately below the TIOBE top ten through `src/tree_sitter_adapter.rs`: PHP
  (`tree-sitter-php`, via its `LANGUAGE_PHP` symbol), Swift
  (`tree-sitter-swift`), Kotlin (`tree-sitter-kotlin-ng`, accepts `kotlin`/`kt`),
  Scala (`tree-sitter-scala`), and Lua (`tree-sitter-lua`). Each parses into real
  `LinkType::Syntax` concrete-syntax links and reconstructs byte-for-byte.
- Added a `SECOND_TIER_PROGRAMMING_LANGUAGE_TARGETS` registry in `src/parity.rs`
  gated by parity tests, mirroring `DATA_FORMAT_TARGETS`.
- Added per-language UTF-8 `LANGUAGE_FIXTURES` round-trip entries plus
  case-insensitive alias coverage and a per-language recovery fixture whose
  malformed source still reconstructs while exposing error/missing diagnostics.

### Documentation
- Documented the wired grammars (crate, version, license, root node) in
  `docs/parity-roadmap.md` and kept the temporary Perl follow-up tracked
  explicitly until `ts-parser-perl` was adopted.

### Added
- Add a Rust `ToLinks`/`FromLinks` codec with queryable type-shape links and shared/circular object graph round-trips through LiNo serialization.

### Added

- Added the `LinkStore` storage trait, read-only-aware storage wrappers, and an
  optional file-mapped `doublets` backend with LiNo/binary round-trip coverage.

### Added
- Added exact-match concept interning, language-bound expression links, external-id alias links, and LiNo concept-set import for the shared concept ontology.

### Added
- Added starter natural-language grammaticality parsing with UD-style morphosyntax links, pass/fail fixtures for the ten natural-language targets, and recoverable error links for ungrammatical fixtures.

### Added
- Added configurable `TranslationRuleSet` values, a runtime
  `TranslationRuleRegistry`, LiNo-backed rule-set loading, template
  placeholders, and missing-rule diagnostics for from-meta reconstruction.

### Added
- Added queryable language profiles with JavaScript transform enforcement and rule-set-derived profile domains.

### Added
- Added incremental source edit reparsing, stable outside-edit link IDs, and structural snapshot diff reporting.

### Added
- Added composable `LinkRule` query algebra with relational rules, boolean composition, named sub-rules, ellipsis gap matching, typed metavariables, plain-text token patterns, traversal strategies, and valid/invalid rule snapshot suites.
- Added quasiquote replacement templates with placeholder validation and parenthesization-conservative captured-text replacement.

### Added
- Add the API operation/style parity registry, fluent network pipeline, and link-cli-style substitution text runner.

### Added
- Wire the canonical `ts-parser-perl` tree-sitter grammar as a second-tier
  programming-language target without upgrading the project-wide tree-sitter
  runtime.

### Added
- Added source generation helpers for constructed syntax networks: `insert_source_token`, `insert_syntax_node`, `render_source`, `render_source_from`, and `render_source_from_document`.

### Added
- Added wave-two competitor and ecosystem parity fixtures with executable transform, reconstruction, grammar, and storage gates.
- Added a recorded `cargo llvm-cov` line-coverage floor to CI so coverage cannot silently regress.

### Added
- Shared, language-free document-formatting concept ontology (`document_formatting` module): inline concepts (emphasis, strong, strikethrough, inline-code, hyperlink, image, line-break) and block concepts (heading with level, paragraph, blockquote, bullet/ordered lists, list-item, code-block with language, thematic-break, table/row/cell), each seeded with Markdown and HTML syntax mappings.
- `seed_common_concept_ontology()` now also seeds the formatting concepts, and `ConceptOntologySeedReport::formatting_concepts()` reports how many were added.
- `LinkNetwork::resolve_document_format` / `render_document_format` / `translate_document_format` so the same concept link reconstructs as `**…**` in Markdown and `<strong>…</strong>` in HTML; Markdown `**bold**` and HTML `<strong>bold</strong>` reach the one shared `strong` concept under semantic projection.
- `FormattingDocument` concept layer with `parse_markup_document` and `translate_markup_document` for full Markdown ⇄ HTML document round-trips through one concept ontology (heading/paragraph/list/bold/italic/link).

### Added
- PDF document-format support (issue #84): a documented, uncompressed text PDF profile (`document_formatting::render_pdf_document` / `parse_pdf_document`) that renders a language-free `FormattingDocument` to a valid single-page PDF (correct `xref` offsets, object table, and stream `Length`) and parses it back into the same concept tree. Block role is carried by marked content (`/H1`…`/H6`, `/P`, `/UL`/`/OL`, `/LI`) and inline bold/italic by the selected font resource (`/F1` regular, `/F2` strong, `/F3` emphasis).
- `parse("…", "pdf", …)` dispatches to a new `pdf_parser` that builds a byte-exact lossless network (`reconstruct_text()` returns the input verbatim) and adds additive `Concept`/`Object` structure links recovering heading/paragraph/list/list-item and bold/italic.
- `reconstruct_text_as("PDF", …)` renders a structurally equivalent PDF through the shared formatting concept layer: a PDF source re-renders byte-for-byte, while a Markdown/HTML source is translated into an equivalent PDF, and `translate_markup_document` now bridges Markdown/HTML ⇄ PDF.
- `PDF` markup target in `MARKUP_LANGUAGE_TARGETS` with a bold + heading + paragraph round-trip `LANGUAGE_FIXTURES` entry, plus `docs/pdf-fidelity.md` documenting the round-trip fidelity matrix for supported and lossy/unsupported PDF features.

### Added
- DOCX (OOXML) document-format support (issue #85): a documented OOXML text profile (`document_formatting::render_docx_document` / `parse_docx_document`) that renders a language-free `FormattingDocument` to `word/document.xml` WordprocessingML and parses it back into the same concept tree. Block role is carried by paragraph properties (`<w:pStyle w:val="HeadingN"/>` headings, bare `<w:p>` paragraphs, `<w:numPr>` `numId` 1/2 bullet/ordered list items) and inline bold/italic by run properties (`<w:b/>` → `strong`, `<w:i/>` → `emphasis`).
- A binary OPC packaging layer (`document_formatting::render_docx_package` / `parse_docx_package`) that assembles a valid `.docx` ZIP (stored entries with a self-implemented CRC-32, no new dependencies) containing `[Content_Types].xml`, the relationship parts, `word/document.xml`, `word/styles.xml`, and `word/numbering.xml`, and reads `word/document.xml` back out.
- `parse("…", "docx", …)` dispatches to a new `docx_parser` that builds a byte-exact lossless network (`reconstruct_text()` returns the input verbatim) and adds additive `Concept`/`Object` structure links recovering heading/paragraph/list/list-item and bold/italic.
- `reconstruct_text_as("DOCX", …)` renders structurally equivalent OOXML through the shared formatting concept layer: a DOCX source re-renders byte-for-byte, while a Markdown/HTML/PDF source is translated into equivalent OOXML, and `translate_markup_document` now bridges Markdown/HTML/PDF ⇄ DOCX.
- `DOCX` markup target in `MARKUP_LANGUAGE_TARGETS` with a bold + heading + bullet-list round-trip `LANGUAGE_FIXTURES` entry, plus `docs/docx-fidelity.md` documenting the two-layer round-trip fidelity matrix for supported and lossy/unsupported OOXML features.

### Fixed
- Fixed the GitHub Pages site returning HTTP 404 at the root URL
  (<https://link-foundation.github.io/meta-language>). The deploy job uploaded
  raw `cargo doc` output, which has no root `index.html` (rustdoc only emits
  `target/doc/<crate>/index.html`), so the Pages root 404'd while docs were only
  reachable at `/meta_language/`. See issue #90.

### Added
- A project website with a landing page (description + docs links) under
  `docs/site/`, deployed at the Pages root.
- An interactive WebAssembly "Links Notation playground" demo in a new
  standalone `web/` crate (wraps the wasm-compatible `links-notation` crate),
  served at `/demo/`.
- `scripts/build-site.rs` to assemble the website (`_site/`) locally and in CI:
  landing page at `/`, demo at `/demo/`, and `rustdoc` under `/api/` with a root
  redirect.
- `docs/case-studies/issue-90` documenting the timeline, requirements, root
  cause analysis, and solution plans.

### Changed
- The `Deploy Website` CI job (formerly `Deploy Rust Documentation`) now builds
  the WebAssembly demo and assembles the full site instead of deploying
  `target/doc` directly.

## [0.43.0] - 2026-06-14

### Added
- Changeset-style fragment format with frontmatter for specifying version bump type
- New `get-bump-type.mjs` script to automatically determine version bump from fragments
- Automatic version bumping on merge to main based on changelog fragments
- Detailed documentation for the changelog fragment system in `changelog.d/README.md`

### Changed
- Updated `collect-changelog.mjs` to strip frontmatter when collecting fragments
- Updated `version-and-commit.mjs` to handle frontmatter in fragments
- Enhanced release workflow to automatically determine bump type from changesets

### Changed
- Add `detect-changes` job with cross-platform `detect-code-changes.mjs` script
- Make lint job independent of changelog check (runs based on file changes only)
- Allow docs-only PRs without changelog fragment requirement
- Handle changelog check 'skipped' state in dependent jobs
- Exclude `changelog.d/`, `docs/`, `experiments/`, `examples/` folders and markdown files from code changes detection

### Fixed
- Fixed README.md to correctly reference Node.js scripts (`.mjs`) instead of Python scripts (`.py`)
- Updated project structure in README.md to match actual script files in `scripts/` directory
- Fixed example code in README.md that had invalid Rust with two `main` functions

### Added

- Added crates.io publishing support to CI/CD workflow
- Added `release_mode` input with "instant" and "changelog-pr" options for manual releases
- Added `--tag-prefix` and `--crates-io-url` options to create-github-release.mjs script
- Added comprehensive case study documentation for Issue #11 in docs/case-studies/issue-11/

### Changed

- Changed changelog fragment check from warning to error (exit 1) to enforce changelog requirements
- Updated job conditions with `always() && !cancelled()` to fix workflow_dispatch job skipping issue
- Renamed manual-release job to "Instant Release" for clarity

### Fixed

- Fixed deprecated `::set-output` GitHub Actions command in version-and-commit.mjs
- Fixed workflow_dispatch triggering issues where lint/build/release jobs were incorrectly skipped

### Fixed

- Fixed changelog fragment check to validate that a fragment is **added in the PR diff** rather than just checking if any fragments exist in the directory. This prevents the check from incorrectly passing when there are leftover fragments from previous PRs that haven't been released yet.

### Changed

- Converted shell scripts in `release.yml` to cross-platform `.mjs` scripts for improved portability and performance:
  - `check-changelog-fragment.mjs` - validates changelog fragment is added in PR diff
  - `git-config.mjs` - configures git user for CI/CD
  - `check-release-needed.mjs` - checks if release is needed
  - `publish-crate.mjs` - publishes package to crates.io
  - `create-changelog-fragment.mjs` - creates changelog fragments for manual releases
  - `get-version.mjs` - gets current version from Cargo.toml

### Added

- Added `check-version-modification.mjs` script to detect manual version changes in Cargo.toml
- Added `version-check` job to CI/CD workflow that runs on pull requests
- Added skip logic for automated release branches (changelog-manual-release-*, changeset-release/*, release/*, automated-release/*)

### Changed

- Version modifications in Cargo.toml are now blocked in pull requests to enforce automated release pipeline

### Added

- Added support for `CARGO_REGISTRY_TOKEN` as alternative to `CARGO_TOKEN` for crates.io publishing
- Added case study documentation for Issue #17 (yargs reserved word and dual token support)

### Changed

- Updated workflow to use fallback logic: `${{ secrets.CARGO_REGISTRY_TOKEN || secrets.CARGO_TOKEN }}`
- Improved publish-crate.mjs to check both `CARGO_REGISTRY_TOKEN` and `CARGO_TOKEN` environment variables
- Added warning message when neither token is set

### Added
- New `scripts/rust-paths.mjs` utility for automatic Rust package root detection
- Support for both single-language and multi-language repository structures in all CI/CD scripts
- Configuration options via `--rust-root` CLI argument and `RUST_ROOT` environment variable
- Comprehensive case study documentation in `docs/case-studies/issue-19/`

### Changed
- Updated all release scripts to use the new path detection utility:
  - `scripts/bump-version.mjs`
  - `scripts/check-release-needed.mjs`
  - `scripts/collect-changelog.mjs`
  - `scripts/get-bump-type.mjs`
  - `scripts/get-version.mjs`
  - `scripts/publish-crate.mjs`
  - `scripts/version-and-commit.mjs`

### Changed

- **check-release-needed.mjs**: Now checks crates.io API directly instead of git tags to determine if a version is already released. This prevents false positives where git tags exist but the package was never actually published to crates.io.

### Added

- **CI/CD Troubleshooting Guide**: New documentation at `docs/ci-cd/troubleshooting.md` covering common issues like skipped jobs, false positive version checks, publishing failures, and secret configuration.

- **Enhanced Error Handling in publish-crate.mjs**: Added specific detection and helpful error messages for authentication failures, including guidance on secret configuration and workflow setup.

- **Case Study Documentation**: Added comprehensive case study at `docs/case-studies/issue-21/` analyzing CI/CD failures from browser-commander repository (issues #27, #29, #31, #33) with timeline, root causes, and lessons learned.

### Fixed

- **Prevent False Positive Version Checks**: The release workflow now correctly identifies unpublished versions by checking crates.io instead of relying on git tags, which can exist without the package being published.

### Changed

- Translated all CI/CD scripts from JavaScript (.mjs) to Rust (.rs) using rust-script
- Scripts now use native Rust with rust-script for execution in shell
- Removed Node.js dependency from CI/CD pipeline
- Updated GitHub Actions workflow to use rust-script instead of node
- Updated README and CONTRIBUTING documentation with new script references

### Added

- Cache `restore-keys` for partial cache hits across all workflow jobs
- Explicit `token` parameter in checkout for release jobs
- Code coverage job with `cargo-llvm-cov` and Codecov integration
- Codecov badge in README.md
- Pre-release version support (e.g., `0.1.0-beta.1`) in version parsing
- `--release-label` parameter for multi-language release disambiguation
- `ensure_version_exceeds_published()` logic to prevent publishing duplicate versions
- `get_max_published_version()` to query highest non-yanked version from crates.io
- `max_published_version` output from check-release-needed for downstream use
- Version fallback logic in auto-release Create GitHub Release step

### Changed

- Updated `actions/checkout` from v4 to v6
- Updated `actions/cache` from v4 to v5
- Updated `peter-evans/create-pull-request` from v7 to v8
- Made `publish-crate.rs` fail (exit 1) when version already exists on crates.io
- Improved `create-github-release.rs` to check combined stdout+stderr and detect "Validation Failed"

### Fixed

- Fix publish steps overriding workflow-level CARGO_TOKEN fallback, breaking CARGO_REGISTRY_TOKEN-only configurations (#32)
- Fix non-fast-forward push failures in multi-workflow repos by adding fetch/rebase and push retry logic (#31)
- Add mono-repo path support to check-changelog-fragment.rs, check-version-modification.rs, and create-changelog-fragment.rs
- Add `!cancelled()` guard to test job condition to respect workflow cancellation

### Fixed

- Fixed `version-and-commit.rs` to check crates.io instead of git tags for determining if a version is already released
- This prevents the release pipeline from getting stuck when git tags exist without corresponding crates.io publication

### Added

- Added `--tag-prefix` support to `version-and-commit.rs` for multi-language repository compatibility
- Added crates.io and docs.rs badges to README.md
- Added automatic crates.io and docs.rs badge injection in GitHub release notes
- Added documentation deployment job to CI/CD pipeline (deploys to GitHub Pages after release)
- Added case study documentation for issue #25

### Fixed
- Fixed unsupported look-ahead regex in `create-github-release.rs` that caused a panic when parsing CHANGELOG.md. Replaced with a two-step approach using only features supported by Rust's `regex` crate.

### Changed
- Restructured example application as a simple CLI sum calculator using `lino-arguments`
- Renamed default package to `example-sum-package-name` with Unlicense license
- Reorganized test structure: `tests/unit/sum.rs`, `tests/integration/sum.rs`, `tests/unit/ci-cd/`
- Converted experiment scripts into proper unit tests in `tests/unit/ci-cd/changelog_parsing.rs`
- Added CI/CD skip logic for template default package name `example-sum-package-name`
- Updated README.md badges and documentation

### Fixed

- Change detection script now uses per-commit diff instead of full PR diff, so commits touching only non-code files correctly skip CI jobs even when earlier commits in the same PR changed code files

### Fixed
- Make release scripts resolve the publishable crate manifest when the repository root uses a Cargo workspace manifest.

### Fixed
- Decoupled GitHub Pages documentation deployment from package release publication and fixed release-script warning failures under `RUSTFLAGS=-Dwarnings`.

### Changed
- Added explicit GitHub Actions job timeouts and documented Rust test timeout guidance.

### Fixed
- Added a non-blocking warning threshold to the Rust file-size check so near-limit files are surfaced before concurrent PR merges can exceed the hard limit.

### Fixed
- Made `create-github-release.rs` build GitHub release titles as `[Language] X.Y.Z` instead of reusing the tag prefix.

### Added
- Added optional Docker Hub image publishing tied to Rust crate releases, including crates.io visibility waiting, version/latest image tags, and Docker Hub badges in GitHub release notes.

### Changed
- Release completeness checks now self-heal when crates.io exists but configured Docker Hub or GitHub release artifacts are missing.

### Fixed
- Switched documentation deployment to the official GitHub Pages artifact workflow so repositories using GitHub Actions as their Pages source do not get false-positive branch-push deploys.

### Added
- Documented the one-time `Settings → Pages → Source = GitHub Actions` prerequisite for the `deploy-docs` job in `README.md` and as a comment above the `deploy-docs` job in `release.yml`, so downstream template users hit a documented setup step instead of a `Get Pages site failed` error on the first deploy.

### Added
- Tracking case study at `docs/case-studies/issue-52/` registering the `browser-commander` + Playwright preview-regeneration pattern from [`konard/vk-bot-desktop#52`](https://github.com/konard/vk-bot-desktop/pull/52), with an activation checklist for when an example-app surface lands in this template. Documentation only — no workflow, script, or runtime code changes. Primary upstream tracking issue: [`link-foundation/js-ai-driven-development-pipeline-template#62`](https://github.com/link-foundation/js-ai-driven-development-pipeline-template/issues/62).

### Fixed
- Release automation now keeps the workspace package entry in `Cargo.lock` synchronized when `scripts/version-and-commit.rs` bumps `Cargo.toml`, preventing stale lock-file version diffs in later pull requests.

### Fixed
- `scripts/publish-crate.rs` now treats crates.io HTTP 429 throttle responses ("You have published too many versions of this crate in the last 24 hours") as a deferred `publish_result=rate_limited` outcome (it writes the output, prints an explanatory banner and exits successfully) instead of a hard CI failure reported as a generic `failed` ("Failed to publish for unknown reason"). Authentication, already-published and unknown failures still exit non-zero. Failed-publish classification is consolidated through a single `classify_failure` function and `FailureKind` enum (with an `is_deferred` predicate), covered by unit tests runnable via `rust-script --test scripts/publish-crate.rs`.
- The release workflow (`.github/workflows/release.yml`) now gates crate-availability waiting, Docker Hub publishing and GitHub release creation on either an already-published crate or `publish_result=success`, so a deferred (rate-limited) crate upload no longer produces partial downstream release artifacts and the same version is retried automatically on the next push to `main`.

### Fixed
- Fixed reversed `cancel-in-progress` concurrency condition in `release.yml` that cancelled in-flight releases on `main` and never superseded older PR runs. The condition now uses `!=` so `main` releases run to completion while newer PR pushes cancel stale runs.

### Added
- Added a `scripts/check-crate-size.rs` guard that builds the `.crate` archive and fails the release before publishing when it exceeds the crates.io 10 MiB upload limit. The check runs in the build job and before publishing in both the auto-release and manual-release jobs.

### Changed
- Added a narrow `include` allowlist to `Cargo.toml` so docs, case studies, generated CI artifacts, changelog fragments, scripts, and experiments no longer inflate the published release archive.

### Fixed
- Prevented GitHub release creation from treating generic API validation failures as existing releases, and capped oversized release notes with a link to the full tagged changelog.

### Added
- Added the initial `meta-language` links-network core with self-description roots,
  verification, trivia attachment configuration, and a minimal CLI.
- Added the default lossless `parse` entry point, projection views, and a
  tested parity roadmap for competitor features, grammar embedding, and language
  coverage targets.
- Added exact reconstruction, mixed-region detection, query matching,
  substitution rules, concept reconstruction, object identity helpers,
  many-valued truth values, and executable parity fixtures for every tracked
  competitor target.
- Added executable language fixtures and tests for every requested Markdown,
  HTML, top-ten programming-language, and top-ten natural-language target.

### Fixed
- Added a CI guard that rejects Rust test modules and test attributes under `src/`, keeping tests in the `tests/` tree.

## Minor Changes

- Add immutable and mutable network snapshots with provenance and forward
  version commits for roadmap snapshot/versioning coverage.

### Added
- Added `txt` as a first-class markup/container target with a UTF-8 lossless
  fixture and content-sniffing fallback regions.

### Fixed
- Reconciled the natural-language target ordering with the Ethnologue 2025
  total-speaker order cited by the parity roadmap.

### Added
- Added a grammar-backed parser front end for Python, C, Java, C++, C#, JavaScript, and R using official tree-sitter grammar crates.

### Added
- Parse supported mixed-mode embedded regions into the host links network with grammar-backed syntax links.

### Added
- Added natural-language segmentation, identification, normalization, and bidi annotation links over lossless text parses.

### Added
- Added grammar-backed Visual Basic parsing with byte-exact reconstruction and recovery flag coverage.

bump: minor

- Add a grammar-backed `sql-ansi` SQL-family dialect fixture using
  `tree-sitter-sequel`.

### Added
- Added grammar-backed Delphi/Object Pascal parsing through `tree-sitter-pascal`.

### Added
- Enriched link queries with S-expression structural matching, captures, by-type construction, host predicate hooks, and link-cli-style variable substitution bindings.

### Added
- Added persistent snapshot structural sharing with interned metadata text storage.

### Added
- Added a query-transform surface for selecting captured links and replacing
  their source text while preserving unchanged bytes.

### Added
- Materialized self-description roots as controlled links with complete root-definition closure and round-trippable `describe` output.

### Added
- Added common concept ontology seeding from meta-expression's semantic lexicon, with shared concept links, syntax mappings, and structural programming-language concepts.

### Added
- Added semantic cross-language reconstruction for the Hawaii statehood fixture,
  including English/Russian naturalization and configurable formalization levels.

### Added
- Expanded `PARITY_FIXTURES` with multiple provenance-tracked fixtures for tree-sitter, LibCST, Recast, jscodeshift, Rowan, cstree, and Roslyn, including executable recovery and query/transform expectations.

### Added

- Expanded `PARITY_FIXTURES` with upstream-provenanced internal ecosystem corpora for links-notation, link-cli, lino-objects-codec, relative-meta-logic, formal-ai, and meta-expression.

### Added

- Added structural LiNo parsing for links-notation doublets, triplets, named links, indented IDs, and self-references while preserving byte-exact reconstruction.

### Added
- Added fixed-point probabilistic truth values for relative-meta-logic-style
  semantic confidence evaluation.

### Added
- Added grammar-backed Go parsing via the official `tree-sitter-go` grammar
  (accepts `Go`, `go`, and `golang`) so `LinkNetwork::parse` emits real
  `LinkType::Syntax` concrete-syntax links with byte-exact reconstruction.

### Added
- `AccessMode { Mutable, ReadOnly }` setting on `ParseConfiguration`
  (`with_access_mode` / `access_mode`), defaulting to `Mutable` so existing
  callers are unaffected.
- `LinkNetwork::freeze` / `as_read_only` yielding a `ReadOnlyNetwork` view that
  exposes only `&self` operations (query, project, reconstruct, verify,
  serialize); mutators are unreachable at compile time because the view never
  hands out `&mut LinkNetwork`.
- `LinkNetwork::parse_engine`, returning an `EngineNetwork` handle that honours
  the configured access mode: read-only parsing returns the frozen form and
  `EngineNetwork::as_mutable` rejects mutation with a `ReadOnlyViolation`
  diagnostic.
- Snapshot interop: `NetworkSnapshot::as_read_only` / `from_read_only` reuse the
  snapshot's `Arc<LinkNetwork>`, so the frozen form composes with snapshot
  versioning instead of duplicating it.

### Added
- Added grammar-backed Ruby parsing through `tree-sitter-ruby`, so
  `LinkNetwork::parse(source, "Ruby", ...)` (and the `rb` alias) now emits real
  `LinkType::Syntax` links instead of falling back to lossless plain text.

bump: minor

- Add a grammar-backed `TypeScript` front end using `tree-sitter-typescript`,
  wiring the `typescript`/`ts` labels to `LANGUAGE_TYPESCRIPT` and the `tsx`
  label to `LANGUAGE_TSX` so `LinkNetwork::parse` emits real `LinkType::Syntax`
  links for TypeScript and TSX sources.

### Added
- Added grammar-backed parsing for seven data-exchange / interchange formats
  through `src/tree_sitter_adapter.rs`: JSON (`tree-sitter-json`), YAML
  (`tree-sitter-yaml`, accepts `yaml`/`yml`), TOML (`tree-sitter-toml-ng`), XML
  and DTD (`tree-sitter-xml`), INI (`tree-sitter-ini`), Protocol Buffers
  (`tree-sitter-proto`, accepts `protobuf`/`proto`/`Protocol Buffers`), and
  GraphQL (`tree-sitter-graphql`, accepts `graphql`/`gql`). Each parses into
  real `LinkType::Syntax` concrete-syntax links and reconstructs byte-for-byte.
- Added a `DATA_FORMAT_TARGETS` registry in `src/parity.rs` (with the new
  `LanguageFamily::DataFormat`) gated by parity tests, mirroring
  `MARKUP_LANGUAGE_TARGETS`.
- Added per-format UTF-8 `LANGUAGE_FIXTURES` round-trip entries plus
  recovery-case and mixed-region tests (a `json` fence embedded in Markdown
  parses into the host links network).
- Added lossless CSV and JSON5 parsers for the two formats whose published
  tree-sitter crates still pin the incompatible `tree-sitter ~0.20` runtime.
  CSV is validated with the Rust `csv` crate and JSON5 is validated with
  `json5_nodes`; both emit structured syntax links and reconstruct
  byte-for-byte.

### Documentation
- Documented the nine wired data-format parsers (parser, version, license, root
  node) in `docs/parity-roadmap.md`, including the tree-sitter compatibility
  rationale for the in-repo CSV and JSON5 parsers.

### Added
- `LinkNetwork::to_lino` serializes an entire links network to canonical
  links-notation text, keyed by each link's numeric id (doublets-style id
  discipline), covering references, names, types, terms, definitions,
  languages, source spans, parse flags, and term registration.
- `LinkNetwork::from_lino` reconstructs the exact network from that text,
  forming a lossless round-trip (`from_lino(to_lino(n))` is isomorphic to `n`).
- `LinoSerializationError` reports parse and schema failures from `from_lino`.
- A round-trip property test over every language fixture plus synthetic
  networks, and a test that `to_lino` output is accepted by the
  `links-notation` 0.13 crate parser.
- `ParityCapability::LinoSerialization` and an output-side serialization
  parity fixture for the `links-notation` target.

### Changed
- Added the `links-notation` 0.13 crate as a dependency so serialized output
  aligns with the wider links-notation ecosystem.

### Added
- `ParserRegistry` maps language keys to `Arc<dyn LanguageParser>` with the
  built-in parser as a fallback. User registrations shadow the built-in
  dispatch for the same (case-insensitive) key; unregistered keys still route
  through the built-in set.
- `LinkNetwork::parse_with_registry` dispatches a parse through a registry.
- `examples/custom_parser_registry.rs` documents registering a custom parser.

### Added
- Added grammar-backed parsing for five popular programming languages
  immediately below the TIOBE top ten through `src/tree_sitter_adapter.rs`: PHP
  (`tree-sitter-php`, via its `LANGUAGE_PHP` symbol), Swift
  (`tree-sitter-swift`), Kotlin (`tree-sitter-kotlin-ng`, accepts `kotlin`/`kt`),
  Scala (`tree-sitter-scala`), and Lua (`tree-sitter-lua`). Each parses into real
  `LinkType::Syntax` concrete-syntax links and reconstructs byte-for-byte.
- Added a `SECOND_TIER_PROGRAMMING_LANGUAGE_TARGETS` registry in `src/parity.rs`
  gated by parity tests, mirroring `DATA_FORMAT_TARGETS`.
- Added per-language UTF-8 `LANGUAGE_FIXTURES` round-trip entries plus
  case-insensitive alias coverage and a per-language recovery fixture whose
  malformed source still reconstructs while exposing error/missing diagnostics.

### Documentation
- Documented the wired grammars (crate, version, license, root node) in
  `docs/parity-roadmap.md` and kept the temporary Perl follow-up tracked
  explicitly until `ts-parser-perl` was adopted.

### Added
- Add a Rust `ToLinks`/`FromLinks` codec with queryable type-shape links and shared/circular object graph round-trips through LiNo serialization.

### Added

- Added the `LinkStore` storage trait, read-only-aware storage wrappers, and an
  optional file-mapped `doublets` backend with LiNo/binary round-trip coverage.

### Added
- Added exact-match concept interning, language-bound expression links, external-id alias links, and LiNo concept-set import for the shared concept ontology.

### Added
- Added starter natural-language grammaticality parsing with UD-style morphosyntax links, pass/fail fixtures for the ten natural-language targets, and recoverable error links for ungrammatical fixtures.

### Added
- Added configurable `TranslationRuleSet` values, a runtime
  `TranslationRuleRegistry`, LiNo-backed rule-set loading, template
  placeholders, and missing-rule diagnostics for from-meta reconstruction.

### Added
- Added queryable language profiles with JavaScript transform enforcement and rule-set-derived profile domains.

### Added
- Added incremental source edit reparsing, stable outside-edit link IDs, and structural snapshot diff reporting.

### Added
- Added composable `LinkRule` query algebra with relational rules, boolean composition, named sub-rules, ellipsis gap matching, typed metavariables, plain-text token patterns, traversal strategies, and valid/invalid rule snapshot suites.
- Added quasiquote replacement templates with placeholder validation and parenthesization-conservative captured-text replacement.

### Added
- Add the API operation/style parity registry, fluent network pipeline, and link-cli-style substitution text runner.

### Added
- Wire the canonical `ts-parser-perl` tree-sitter grammar as a second-tier
  programming-language target without upgrading the project-wide tree-sitter
  runtime.

### Added
- Added source generation helpers for constructed syntax networks: `insert_source_token`, `insert_syntax_node`, `render_source`, `render_source_from`, and `render_source_from_document`.

### Added
- Added wave-two competitor and ecosystem parity fixtures with executable transform, reconstruction, grammar, and storage gates.
- Added a recorded `cargo llvm-cov` line-coverage floor to CI so coverage cannot silently regress.

### Added
- Shared, language-free document-formatting concept ontology (`document_formatting` module): inline concepts (emphasis, strong, strikethrough, inline-code, hyperlink, image, line-break) and block concepts (heading with level, paragraph, blockquote, bullet/ordered lists, list-item, code-block with language, thematic-break, table/row/cell), each seeded with Markdown and HTML syntax mappings.
- `seed_common_concept_ontology()` now also seeds the formatting concepts, and `ConceptOntologySeedReport::formatting_concepts()` reports how many were added.
- `LinkNetwork::resolve_document_format` / `render_document_format` / `translate_document_format` so the same concept link reconstructs as `**…**` in Markdown and `<strong>…</strong>` in HTML; Markdown `**bold**` and HTML `<strong>bold</strong>` reach the one shared `strong` concept under semantic projection.
- `FormattingDocument` concept layer with `parse_markup_document` and `translate_markup_document` for full Markdown ⇄ HTML document round-trips through one concept ontology (heading/paragraph/list/bold/italic/link).

### Added
- PDF document-format support (issue #84): a documented, uncompressed text PDF profile (`document_formatting::render_pdf_document` / `parse_pdf_document`) that renders a language-free `FormattingDocument` to a valid single-page PDF (correct `xref` offsets, object table, and stream `Length`) and parses it back into the same concept tree. Block role is carried by marked content (`/H1`…`/H6`, `/P`, `/UL`/`/OL`, `/LI`) and inline bold/italic by the selected font resource (`/F1` regular, `/F2` strong, `/F3` emphasis).
- `parse("…", "pdf", …)` dispatches to a new `pdf_parser` that builds a byte-exact lossless network (`reconstruct_text()` returns the input verbatim) and adds additive `Concept`/`Object` structure links recovering heading/paragraph/list/list-item and bold/italic.
- `reconstruct_text_as("PDF", …)` renders a structurally equivalent PDF through the shared formatting concept layer: a PDF source re-renders byte-for-byte, while a Markdown/HTML source is translated into an equivalent PDF, and `translate_markup_document` now bridges Markdown/HTML ⇄ PDF.
- `PDF` markup target in `MARKUP_LANGUAGE_TARGETS` with a bold + heading + paragraph round-trip `LANGUAGE_FIXTURES` entry, plus `docs/pdf-fidelity.md` documenting the round-trip fidelity matrix for supported and lossy/unsupported PDF features.

### Added
- DOCX (OOXML) document-format support (issue #85): a documented OOXML text profile (`document_formatting::render_docx_document` / `parse_docx_document`) that renders a language-free `FormattingDocument` to `word/document.xml` WordprocessingML and parses it back into the same concept tree. Block role is carried by paragraph properties (`<w:pStyle w:val="HeadingN"/>` headings, bare `<w:p>` paragraphs, `<w:numPr>` `numId` 1/2 bullet/ordered list items) and inline bold/italic by run properties (`<w:b/>` → `strong`, `<w:i/>` → `emphasis`).
- A binary OPC packaging layer (`document_formatting::render_docx_package` / `parse_docx_package`) that assembles a valid `.docx` ZIP (stored entries with a self-implemented CRC-32, no new dependencies) containing `[Content_Types].xml`, the relationship parts, `word/document.xml`, `word/styles.xml`, and `word/numbering.xml`, and reads `word/document.xml` back out.
- `parse("…", "docx", …)` dispatches to a new `docx_parser` that builds a byte-exact lossless network (`reconstruct_text()` returns the input verbatim) and adds additive `Concept`/`Object` structure links recovering heading/paragraph/list/list-item and bold/italic.
- `reconstruct_text_as("DOCX", …)` renders structurally equivalent OOXML through the shared formatting concept layer: a DOCX source re-renders byte-for-byte, while a Markdown/HTML/PDF source is translated into equivalent OOXML, and `translate_markup_document` now bridges Markdown/HTML/PDF ⇄ DOCX.
- `DOCX` markup target in `MARKUP_LANGUAGE_TARGETS` with a bold + heading + bullet-list round-trip `LANGUAGE_FIXTURES` entry, plus `docs/docx-fidelity.md` documenting the two-layer round-trip fidelity matrix for supported and lossy/unsupported OOXML features.

## [0.42.0] - 2026-06-14

### Added
- Changeset-style fragment format with frontmatter for specifying version bump type
- New `get-bump-type.mjs` script to automatically determine version bump from fragments
- Automatic version bumping on merge to main based on changelog fragments
- Detailed documentation for the changelog fragment system in `changelog.d/README.md`

### Changed
- Updated `collect-changelog.mjs` to strip frontmatter when collecting fragments
- Updated `version-and-commit.mjs` to handle frontmatter in fragments
- Enhanced release workflow to automatically determine bump type from changesets

### Changed
- Add `detect-changes` job with cross-platform `detect-code-changes.mjs` script
- Make lint job independent of changelog check (runs based on file changes only)
- Allow docs-only PRs without changelog fragment requirement
- Handle changelog check 'skipped' state in dependent jobs
- Exclude `changelog.d/`, `docs/`, `experiments/`, `examples/` folders and markdown files from code changes detection

### Fixed
- Fixed README.md to correctly reference Node.js scripts (`.mjs`) instead of Python scripts (`.py`)
- Updated project structure in README.md to match actual script files in `scripts/` directory
- Fixed example code in README.md that had invalid Rust with two `main` functions

### Added

- Added crates.io publishing support to CI/CD workflow
- Added `release_mode` input with "instant" and "changelog-pr" options for manual releases
- Added `--tag-prefix` and `--crates-io-url` options to create-github-release.mjs script
- Added comprehensive case study documentation for Issue #11 in docs/case-studies/issue-11/

### Changed

- Changed changelog fragment check from warning to error (exit 1) to enforce changelog requirements
- Updated job conditions with `always() && !cancelled()` to fix workflow_dispatch job skipping issue
- Renamed manual-release job to "Instant Release" for clarity

### Fixed

- Fixed deprecated `::set-output` GitHub Actions command in version-and-commit.mjs
- Fixed workflow_dispatch triggering issues where lint/build/release jobs were incorrectly skipped

### Fixed

- Fixed changelog fragment check to validate that a fragment is **added in the PR diff** rather than just checking if any fragments exist in the directory. This prevents the check from incorrectly passing when there are leftover fragments from previous PRs that haven't been released yet.

### Changed

- Converted shell scripts in `release.yml` to cross-platform `.mjs` scripts for improved portability and performance:
  - `check-changelog-fragment.mjs` - validates changelog fragment is added in PR diff
  - `git-config.mjs` - configures git user for CI/CD
  - `check-release-needed.mjs` - checks if release is needed
  - `publish-crate.mjs` - publishes package to crates.io
  - `create-changelog-fragment.mjs` - creates changelog fragments for manual releases
  - `get-version.mjs` - gets current version from Cargo.toml

### Added

- Added `check-version-modification.mjs` script to detect manual version changes in Cargo.toml
- Added `version-check` job to CI/CD workflow that runs on pull requests
- Added skip logic for automated release branches (changelog-manual-release-*, changeset-release/*, release/*, automated-release/*)

### Changed

- Version modifications in Cargo.toml are now blocked in pull requests to enforce automated release pipeline

### Added

- Added support for `CARGO_REGISTRY_TOKEN` as alternative to `CARGO_TOKEN` for crates.io publishing
- Added case study documentation for Issue #17 (yargs reserved word and dual token support)

### Changed

- Updated workflow to use fallback logic: `${{ secrets.CARGO_REGISTRY_TOKEN || secrets.CARGO_TOKEN }}`
- Improved publish-crate.mjs to check both `CARGO_REGISTRY_TOKEN` and `CARGO_TOKEN` environment variables
- Added warning message when neither token is set

### Added
- New `scripts/rust-paths.mjs` utility for automatic Rust package root detection
- Support for both single-language and multi-language repository structures in all CI/CD scripts
- Configuration options via `--rust-root` CLI argument and `RUST_ROOT` environment variable
- Comprehensive case study documentation in `docs/case-studies/issue-19/`

### Changed
- Updated all release scripts to use the new path detection utility:
  - `scripts/bump-version.mjs`
  - `scripts/check-release-needed.mjs`
  - `scripts/collect-changelog.mjs`
  - `scripts/get-bump-type.mjs`
  - `scripts/get-version.mjs`
  - `scripts/publish-crate.mjs`
  - `scripts/version-and-commit.mjs`

### Changed

- **check-release-needed.mjs**: Now checks crates.io API directly instead of git tags to determine if a version is already released. This prevents false positives where git tags exist but the package was never actually published to crates.io.

### Added

- **CI/CD Troubleshooting Guide**: New documentation at `docs/ci-cd/troubleshooting.md` covering common issues like skipped jobs, false positive version checks, publishing failures, and secret configuration.

- **Enhanced Error Handling in publish-crate.mjs**: Added specific detection and helpful error messages for authentication failures, including guidance on secret configuration and workflow setup.

- **Case Study Documentation**: Added comprehensive case study at `docs/case-studies/issue-21/` analyzing CI/CD failures from browser-commander repository (issues #27, #29, #31, #33) with timeline, root causes, and lessons learned.

### Fixed

- **Prevent False Positive Version Checks**: The release workflow now correctly identifies unpublished versions by checking crates.io instead of relying on git tags, which can exist without the package being published.

### Changed

- Translated all CI/CD scripts from JavaScript (.mjs) to Rust (.rs) using rust-script
- Scripts now use native Rust with rust-script for execution in shell
- Removed Node.js dependency from CI/CD pipeline
- Updated GitHub Actions workflow to use rust-script instead of node
- Updated README and CONTRIBUTING documentation with new script references

### Added

- Cache `restore-keys` for partial cache hits across all workflow jobs
- Explicit `token` parameter in checkout for release jobs
- Code coverage job with `cargo-llvm-cov` and Codecov integration
- Codecov badge in README.md
- Pre-release version support (e.g., `0.1.0-beta.1`) in version parsing
- `--release-label` parameter for multi-language release disambiguation
- `ensure_version_exceeds_published()` logic to prevent publishing duplicate versions
- `get_max_published_version()` to query highest non-yanked version from crates.io
- `max_published_version` output from check-release-needed for downstream use
- Version fallback logic in auto-release Create GitHub Release step

### Changed

- Updated `actions/checkout` from v4 to v6
- Updated `actions/cache` from v4 to v5
- Updated `peter-evans/create-pull-request` from v7 to v8
- Made `publish-crate.rs` fail (exit 1) when version already exists on crates.io
- Improved `create-github-release.rs` to check combined stdout+stderr and detect "Validation Failed"

### Fixed

- Fix publish steps overriding workflow-level CARGO_TOKEN fallback, breaking CARGO_REGISTRY_TOKEN-only configurations (#32)
- Fix non-fast-forward push failures in multi-workflow repos by adding fetch/rebase and push retry logic (#31)
- Add mono-repo path support to check-changelog-fragment.rs, check-version-modification.rs, and create-changelog-fragment.rs
- Add `!cancelled()` guard to test job condition to respect workflow cancellation

### Fixed

- Fixed `version-and-commit.rs` to check crates.io instead of git tags for determining if a version is already released
- This prevents the release pipeline from getting stuck when git tags exist without corresponding crates.io publication

### Added

- Added `--tag-prefix` support to `version-and-commit.rs` for multi-language repository compatibility
- Added crates.io and docs.rs badges to README.md
- Added automatic crates.io and docs.rs badge injection in GitHub release notes
- Added documentation deployment job to CI/CD pipeline (deploys to GitHub Pages after release)
- Added case study documentation for issue #25

### Fixed
- Fixed unsupported look-ahead regex in `create-github-release.rs` that caused a panic when parsing CHANGELOG.md. Replaced with a two-step approach using only features supported by Rust's `regex` crate.

### Changed
- Restructured example application as a simple CLI sum calculator using `lino-arguments`
- Renamed default package to `example-sum-package-name` with Unlicense license
- Reorganized test structure: `tests/unit/sum.rs`, `tests/integration/sum.rs`, `tests/unit/ci-cd/`
- Converted experiment scripts into proper unit tests in `tests/unit/ci-cd/changelog_parsing.rs`
- Added CI/CD skip logic for template default package name `example-sum-package-name`
- Updated README.md badges and documentation

### Fixed

- Change detection script now uses per-commit diff instead of full PR diff, so commits touching only non-code files correctly skip CI jobs even when earlier commits in the same PR changed code files

### Fixed
- Make release scripts resolve the publishable crate manifest when the repository root uses a Cargo workspace manifest.

### Fixed
- Decoupled GitHub Pages documentation deployment from package release publication and fixed release-script warning failures under `RUSTFLAGS=-Dwarnings`.

### Changed
- Added explicit GitHub Actions job timeouts and documented Rust test timeout guidance.

### Fixed
- Added a non-blocking warning threshold to the Rust file-size check so near-limit files are surfaced before concurrent PR merges can exceed the hard limit.

### Fixed
- Made `create-github-release.rs` build GitHub release titles as `[Language] X.Y.Z` instead of reusing the tag prefix.

### Added
- Added optional Docker Hub image publishing tied to Rust crate releases, including crates.io visibility waiting, version/latest image tags, and Docker Hub badges in GitHub release notes.

### Changed
- Release completeness checks now self-heal when crates.io exists but configured Docker Hub or GitHub release artifacts are missing.

### Fixed
- Switched documentation deployment to the official GitHub Pages artifact workflow so repositories using GitHub Actions as their Pages source do not get false-positive branch-push deploys.

### Added
- Documented the one-time `Settings → Pages → Source = GitHub Actions` prerequisite for the `deploy-docs` job in `README.md` and as a comment above the `deploy-docs` job in `release.yml`, so downstream template users hit a documented setup step instead of a `Get Pages site failed` error on the first deploy.

### Added
- Tracking case study at `docs/case-studies/issue-52/` registering the `browser-commander` + Playwright preview-regeneration pattern from [`konard/vk-bot-desktop#52`](https://github.com/konard/vk-bot-desktop/pull/52), with an activation checklist for when an example-app surface lands in this template. Documentation only — no workflow, script, or runtime code changes. Primary upstream tracking issue: [`link-foundation/js-ai-driven-development-pipeline-template#62`](https://github.com/link-foundation/js-ai-driven-development-pipeline-template/issues/62).

### Fixed
- Release automation now keeps the workspace package entry in `Cargo.lock` synchronized when `scripts/version-and-commit.rs` bumps `Cargo.toml`, preventing stale lock-file version diffs in later pull requests.

### Fixed
- `scripts/publish-crate.rs` now treats crates.io HTTP 429 throttle responses ("You have published too many versions of this crate in the last 24 hours") as a deferred `publish_result=rate_limited` outcome (it writes the output, prints an explanatory banner and exits successfully) instead of a hard CI failure reported as a generic `failed` ("Failed to publish for unknown reason"). Authentication, already-published and unknown failures still exit non-zero. Failed-publish classification is consolidated through a single `classify_failure` function and `FailureKind` enum (with an `is_deferred` predicate), covered by unit tests runnable via `rust-script --test scripts/publish-crate.rs`.
- The release workflow (`.github/workflows/release.yml`) now gates crate-availability waiting, Docker Hub publishing and GitHub release creation on either an already-published crate or `publish_result=success`, so a deferred (rate-limited) crate upload no longer produces partial downstream release artifacts and the same version is retried automatically on the next push to `main`.

### Fixed
- Fixed reversed `cancel-in-progress` concurrency condition in `release.yml` that cancelled in-flight releases on `main` and never superseded older PR runs. The condition now uses `!=` so `main` releases run to completion while newer PR pushes cancel stale runs.

### Added
- Added a `scripts/check-crate-size.rs` guard that builds the `.crate` archive and fails the release before publishing when it exceeds the crates.io 10 MiB upload limit. The check runs in the build job and before publishing in both the auto-release and manual-release jobs.

### Changed
- Added a narrow `include` allowlist to `Cargo.toml` so docs, case studies, generated CI artifacts, changelog fragments, scripts, and experiments no longer inflate the published release archive.

### Fixed
- Prevented GitHub release creation from treating generic API validation failures as existing releases, and capped oversized release notes with a link to the full tagged changelog.

### Added
- Added the initial `meta-language` links-network core with self-description roots,
  verification, trivia attachment configuration, and a minimal CLI.
- Added the default lossless `parse` entry point, projection views, and a
  tested parity roadmap for competitor features, grammar embedding, and language
  coverage targets.
- Added exact reconstruction, mixed-region detection, query matching,
  substitution rules, concept reconstruction, object identity helpers,
  many-valued truth values, and executable parity fixtures for every tracked
  competitor target.
- Added executable language fixtures and tests for every requested Markdown,
  HTML, top-ten programming-language, and top-ten natural-language target.

### Fixed
- Added a CI guard that rejects Rust test modules and test attributes under `src/`, keeping tests in the `tests/` tree.

## Minor Changes

- Add immutable and mutable network snapshots with provenance and forward
  version commits for roadmap snapshot/versioning coverage.

### Added
- Added `txt` as a first-class markup/container target with a UTF-8 lossless
  fixture and content-sniffing fallback regions.

### Fixed
- Reconciled the natural-language target ordering with the Ethnologue 2025
  total-speaker order cited by the parity roadmap.

### Added
- Added a grammar-backed parser front end for Python, C, Java, C++, C#, JavaScript, and R using official tree-sitter grammar crates.

### Added
- Parse supported mixed-mode embedded regions into the host links network with grammar-backed syntax links.

### Added
- Added natural-language segmentation, identification, normalization, and bidi annotation links over lossless text parses.

### Added
- Added grammar-backed Visual Basic parsing with byte-exact reconstruction and recovery flag coverage.

bump: minor

- Add a grammar-backed `sql-ansi` SQL-family dialect fixture using
  `tree-sitter-sequel`.

### Added
- Added grammar-backed Delphi/Object Pascal parsing through `tree-sitter-pascal`.

### Added
- Enriched link queries with S-expression structural matching, captures, by-type construction, host predicate hooks, and link-cli-style variable substitution bindings.

### Added
- Added persistent snapshot structural sharing with interned metadata text storage.

### Added
- Added a query-transform surface for selecting captured links and replacing
  their source text while preserving unchanged bytes.

### Added
- Materialized self-description roots as controlled links with complete root-definition closure and round-trippable `describe` output.

### Added
- Added common concept ontology seeding from meta-expression's semantic lexicon, with shared concept links, syntax mappings, and structural programming-language concepts.

### Added
- Added semantic cross-language reconstruction for the Hawaii statehood fixture,
  including English/Russian naturalization and configurable formalization levels.

### Added
- Expanded `PARITY_FIXTURES` with multiple provenance-tracked fixtures for tree-sitter, LibCST, Recast, jscodeshift, Rowan, cstree, and Roslyn, including executable recovery and query/transform expectations.

### Added

- Expanded `PARITY_FIXTURES` with upstream-provenanced internal ecosystem corpora for links-notation, link-cli, lino-objects-codec, relative-meta-logic, formal-ai, and meta-expression.

### Added

- Added structural LiNo parsing for links-notation doublets, triplets, named links, indented IDs, and self-references while preserving byte-exact reconstruction.

### Added
- Added fixed-point probabilistic truth values for relative-meta-logic-style
  semantic confidence evaluation.

### Added
- Added grammar-backed Go parsing via the official `tree-sitter-go` grammar
  (accepts `Go`, `go`, and `golang`) so `LinkNetwork::parse` emits real
  `LinkType::Syntax` concrete-syntax links with byte-exact reconstruction.

### Added
- `AccessMode { Mutable, ReadOnly }` setting on `ParseConfiguration`
  (`with_access_mode` / `access_mode`), defaulting to `Mutable` so existing
  callers are unaffected.
- `LinkNetwork::freeze` / `as_read_only` yielding a `ReadOnlyNetwork` view that
  exposes only `&self` operations (query, project, reconstruct, verify,
  serialize); mutators are unreachable at compile time because the view never
  hands out `&mut LinkNetwork`.
- `LinkNetwork::parse_engine`, returning an `EngineNetwork` handle that honours
  the configured access mode: read-only parsing returns the frozen form and
  `EngineNetwork::as_mutable` rejects mutation with a `ReadOnlyViolation`
  diagnostic.
- Snapshot interop: `NetworkSnapshot::as_read_only` / `from_read_only` reuse the
  snapshot's `Arc<LinkNetwork>`, so the frozen form composes with snapshot
  versioning instead of duplicating it.

### Added
- Added grammar-backed Ruby parsing through `tree-sitter-ruby`, so
  `LinkNetwork::parse(source, "Ruby", ...)` (and the `rb` alias) now emits real
  `LinkType::Syntax` links instead of falling back to lossless plain text.

bump: minor

- Add a grammar-backed `TypeScript` front end using `tree-sitter-typescript`,
  wiring the `typescript`/`ts` labels to `LANGUAGE_TYPESCRIPT` and the `tsx`
  label to `LANGUAGE_TSX` so `LinkNetwork::parse` emits real `LinkType::Syntax`
  links for TypeScript and TSX sources.

### Added
- Added grammar-backed parsing for seven data-exchange / interchange formats
  through `src/tree_sitter_adapter.rs`: JSON (`tree-sitter-json`), YAML
  (`tree-sitter-yaml`, accepts `yaml`/`yml`), TOML (`tree-sitter-toml-ng`), XML
  and DTD (`tree-sitter-xml`), INI (`tree-sitter-ini`), Protocol Buffers
  (`tree-sitter-proto`, accepts `protobuf`/`proto`/`Protocol Buffers`), and
  GraphQL (`tree-sitter-graphql`, accepts `graphql`/`gql`). Each parses into
  real `LinkType::Syntax` concrete-syntax links and reconstructs byte-for-byte.
- Added a `DATA_FORMAT_TARGETS` registry in `src/parity.rs` (with the new
  `LanguageFamily::DataFormat`) gated by parity tests, mirroring
  `MARKUP_LANGUAGE_TARGETS`.
- Added per-format UTF-8 `LANGUAGE_FIXTURES` round-trip entries plus
  recovery-case and mixed-region tests (a `json` fence embedded in Markdown
  parses into the host links network).
- Added lossless CSV and JSON5 parsers for the two formats whose published
  tree-sitter crates still pin the incompatible `tree-sitter ~0.20` runtime.
  CSV is validated with the Rust `csv` crate and JSON5 is validated with
  `json5_nodes`; both emit structured syntax links and reconstruct
  byte-for-byte.

### Documentation
- Documented the nine wired data-format parsers (parser, version, license, root
  node) in `docs/parity-roadmap.md`, including the tree-sitter compatibility
  rationale for the in-repo CSV and JSON5 parsers.

### Added
- `LinkNetwork::to_lino` serializes an entire links network to canonical
  links-notation text, keyed by each link's numeric id (doublets-style id
  discipline), covering references, names, types, terms, definitions,
  languages, source spans, parse flags, and term registration.
- `LinkNetwork::from_lino` reconstructs the exact network from that text,
  forming a lossless round-trip (`from_lino(to_lino(n))` is isomorphic to `n`).
- `LinoSerializationError` reports parse and schema failures from `from_lino`.
- A round-trip property test over every language fixture plus synthetic
  networks, and a test that `to_lino` output is accepted by the
  `links-notation` 0.13 crate parser.
- `ParityCapability::LinoSerialization` and an output-side serialization
  parity fixture for the `links-notation` target.

### Changed
- Added the `links-notation` 0.13 crate as a dependency so serialized output
  aligns with the wider links-notation ecosystem.

### Added
- `ParserRegistry` maps language keys to `Arc<dyn LanguageParser>` with the
  built-in parser as a fallback. User registrations shadow the built-in
  dispatch for the same (case-insensitive) key; unregistered keys still route
  through the built-in set.
- `LinkNetwork::parse_with_registry` dispatches a parse through a registry.
- `examples/custom_parser_registry.rs` documents registering a custom parser.

### Added
- Added grammar-backed parsing for five popular programming languages
  immediately below the TIOBE top ten through `src/tree_sitter_adapter.rs`: PHP
  (`tree-sitter-php`, via its `LANGUAGE_PHP` symbol), Swift
  (`tree-sitter-swift`), Kotlin (`tree-sitter-kotlin-ng`, accepts `kotlin`/`kt`),
  Scala (`tree-sitter-scala`), and Lua (`tree-sitter-lua`). Each parses into real
  `LinkType::Syntax` concrete-syntax links and reconstructs byte-for-byte.
- Added a `SECOND_TIER_PROGRAMMING_LANGUAGE_TARGETS` registry in `src/parity.rs`
  gated by parity tests, mirroring `DATA_FORMAT_TARGETS`.
- Added per-language UTF-8 `LANGUAGE_FIXTURES` round-trip entries plus
  case-insensitive alias coverage and a per-language recovery fixture whose
  malformed source still reconstructs while exposing error/missing diagnostics.

### Documentation
- Documented the wired grammars (crate, version, license, root node) in
  `docs/parity-roadmap.md` and kept the temporary Perl follow-up tracked
  explicitly until `ts-parser-perl` was adopted.

### Added
- Add a Rust `ToLinks`/`FromLinks` codec with queryable type-shape links and shared/circular object graph round-trips through LiNo serialization.

### Added

- Added the `LinkStore` storage trait, read-only-aware storage wrappers, and an
  optional file-mapped `doublets` backend with LiNo/binary round-trip coverage.

### Added
- Added exact-match concept interning, language-bound expression links, external-id alias links, and LiNo concept-set import for the shared concept ontology.

### Added
- Added starter natural-language grammaticality parsing with UD-style morphosyntax links, pass/fail fixtures for the ten natural-language targets, and recoverable error links for ungrammatical fixtures.

### Added
- Added configurable `TranslationRuleSet` values, a runtime
  `TranslationRuleRegistry`, LiNo-backed rule-set loading, template
  placeholders, and missing-rule diagnostics for from-meta reconstruction.

### Added
- Added queryable language profiles with JavaScript transform enforcement and rule-set-derived profile domains.

### Added
- Added incremental source edit reparsing, stable outside-edit link IDs, and structural snapshot diff reporting.

### Added
- Added composable `LinkRule` query algebra with relational rules, boolean composition, named sub-rules, ellipsis gap matching, typed metavariables, plain-text token patterns, traversal strategies, and valid/invalid rule snapshot suites.
- Added quasiquote replacement templates with placeholder validation and parenthesization-conservative captured-text replacement.

### Added
- Add the API operation/style parity registry, fluent network pipeline, and link-cli-style substitution text runner.

### Added
- Wire the canonical `ts-parser-perl` tree-sitter grammar as a second-tier
  programming-language target without upgrading the project-wide tree-sitter
  runtime.

### Added
- Added source generation helpers for constructed syntax networks: `insert_source_token`, `insert_syntax_node`, `render_source`, `render_source_from`, and `render_source_from_document`.

### Added
- Added wave-two competitor and ecosystem parity fixtures with executable transform, reconstruction, grammar, and storage gates.
- Added a recorded `cargo llvm-cov` line-coverage floor to CI so coverage cannot silently regress.

### Added
- Shared, language-free document-formatting concept ontology (`document_formatting` module): inline concepts (emphasis, strong, strikethrough, inline-code, hyperlink, image, line-break) and block concepts (heading with level, paragraph, blockquote, bullet/ordered lists, list-item, code-block with language, thematic-break, table/row/cell), each seeded with Markdown and HTML syntax mappings.
- `seed_common_concept_ontology()` now also seeds the formatting concepts, and `ConceptOntologySeedReport::formatting_concepts()` reports how many were added.
- `LinkNetwork::resolve_document_format` / `render_document_format` / `translate_document_format` so the same concept link reconstructs as `**…**` in Markdown and `<strong>…</strong>` in HTML; Markdown `**bold**` and HTML `<strong>bold</strong>` reach the one shared `strong` concept under semantic projection.
- `FormattingDocument` concept layer with `parse_markup_document` and `translate_markup_document` for full Markdown ⇄ HTML document round-trips through one concept ontology (heading/paragraph/list/bold/italic/link).

### Added
- PDF document-format support (issue #84): a documented, uncompressed text PDF profile (`document_formatting::render_pdf_document` / `parse_pdf_document`) that renders a language-free `FormattingDocument` to a valid single-page PDF (correct `xref` offsets, object table, and stream `Length`) and parses it back into the same concept tree. Block role is carried by marked content (`/H1`…`/H6`, `/P`, `/UL`/`/OL`, `/LI`) and inline bold/italic by the selected font resource (`/F1` regular, `/F2` strong, `/F3` emphasis).
- `parse("…", "pdf", …)` dispatches to a new `pdf_parser` that builds a byte-exact lossless network (`reconstruct_text()` returns the input verbatim) and adds additive `Concept`/`Object` structure links recovering heading/paragraph/list/list-item and bold/italic.
- `reconstruct_text_as("PDF", …)` renders a structurally equivalent PDF through the shared formatting concept layer: a PDF source re-renders byte-for-byte, while a Markdown/HTML source is translated into an equivalent PDF, and `translate_markup_document` now bridges Markdown/HTML ⇄ PDF.
- `PDF` markup target in `MARKUP_LANGUAGE_TARGETS` with a bold + heading + paragraph round-trip `LANGUAGE_FIXTURES` entry, plus `docs/pdf-fidelity.md` documenting the round-trip fidelity matrix for supported and lossy/unsupported PDF features.

## [0.41.0] - 2026-06-14

### Added
- Changeset-style fragment format with frontmatter for specifying version bump type
- New `get-bump-type.mjs` script to automatically determine version bump from fragments
- Automatic version bumping on merge to main based on changelog fragments
- Detailed documentation for the changelog fragment system in `changelog.d/README.md`

### Changed
- Updated `collect-changelog.mjs` to strip frontmatter when collecting fragments
- Updated `version-and-commit.mjs` to handle frontmatter in fragments
- Enhanced release workflow to automatically determine bump type from changesets

### Changed
- Add `detect-changes` job with cross-platform `detect-code-changes.mjs` script
- Make lint job independent of changelog check (runs based on file changes only)
- Allow docs-only PRs without changelog fragment requirement
- Handle changelog check 'skipped' state in dependent jobs
- Exclude `changelog.d/`, `docs/`, `experiments/`, `examples/` folders and markdown files from code changes detection

### Fixed
- Fixed README.md to correctly reference Node.js scripts (`.mjs`) instead of Python scripts (`.py`)
- Updated project structure in README.md to match actual script files in `scripts/` directory
- Fixed example code in README.md that had invalid Rust with two `main` functions

### Added

- Added crates.io publishing support to CI/CD workflow
- Added `release_mode` input with "instant" and "changelog-pr" options for manual releases
- Added `--tag-prefix` and `--crates-io-url` options to create-github-release.mjs script
- Added comprehensive case study documentation for Issue #11 in docs/case-studies/issue-11/

### Changed

- Changed changelog fragment check from warning to error (exit 1) to enforce changelog requirements
- Updated job conditions with `always() && !cancelled()` to fix workflow_dispatch job skipping issue
- Renamed manual-release job to "Instant Release" for clarity

### Fixed

- Fixed deprecated `::set-output` GitHub Actions command in version-and-commit.mjs
- Fixed workflow_dispatch triggering issues where lint/build/release jobs were incorrectly skipped

### Fixed

- Fixed changelog fragment check to validate that a fragment is **added in the PR diff** rather than just checking if any fragments exist in the directory. This prevents the check from incorrectly passing when there are leftover fragments from previous PRs that haven't been released yet.

### Changed

- Converted shell scripts in `release.yml` to cross-platform `.mjs` scripts for improved portability and performance:
  - `check-changelog-fragment.mjs` - validates changelog fragment is added in PR diff
  - `git-config.mjs` - configures git user for CI/CD
  - `check-release-needed.mjs` - checks if release is needed
  - `publish-crate.mjs` - publishes package to crates.io
  - `create-changelog-fragment.mjs` - creates changelog fragments for manual releases
  - `get-version.mjs` - gets current version from Cargo.toml

### Added

- Added `check-version-modification.mjs` script to detect manual version changes in Cargo.toml
- Added `version-check` job to CI/CD workflow that runs on pull requests
- Added skip logic for automated release branches (changelog-manual-release-*, changeset-release/*, release/*, automated-release/*)

### Changed

- Version modifications in Cargo.toml are now blocked in pull requests to enforce automated release pipeline

### Added

- Added support for `CARGO_REGISTRY_TOKEN` as alternative to `CARGO_TOKEN` for crates.io publishing
- Added case study documentation for Issue #17 (yargs reserved word and dual token support)

### Changed

- Updated workflow to use fallback logic: `${{ secrets.CARGO_REGISTRY_TOKEN || secrets.CARGO_TOKEN }}`
- Improved publish-crate.mjs to check both `CARGO_REGISTRY_TOKEN` and `CARGO_TOKEN` environment variables
- Added warning message when neither token is set

### Added
- New `scripts/rust-paths.mjs` utility for automatic Rust package root detection
- Support for both single-language and multi-language repository structures in all CI/CD scripts
- Configuration options via `--rust-root` CLI argument and `RUST_ROOT` environment variable
- Comprehensive case study documentation in `docs/case-studies/issue-19/`

### Changed
- Updated all release scripts to use the new path detection utility:
  - `scripts/bump-version.mjs`
  - `scripts/check-release-needed.mjs`
  - `scripts/collect-changelog.mjs`
  - `scripts/get-bump-type.mjs`
  - `scripts/get-version.mjs`
  - `scripts/publish-crate.mjs`
  - `scripts/version-and-commit.mjs`

### Changed

- **check-release-needed.mjs**: Now checks crates.io API directly instead of git tags to determine if a version is already released. This prevents false positives where git tags exist but the package was never actually published to crates.io.

### Added

- **CI/CD Troubleshooting Guide**: New documentation at `docs/ci-cd/troubleshooting.md` covering common issues like skipped jobs, false positive version checks, publishing failures, and secret configuration.

- **Enhanced Error Handling in publish-crate.mjs**: Added specific detection and helpful error messages for authentication failures, including guidance on secret configuration and workflow setup.

- **Case Study Documentation**: Added comprehensive case study at `docs/case-studies/issue-21/` analyzing CI/CD failures from browser-commander repository (issues #27, #29, #31, #33) with timeline, root causes, and lessons learned.

### Fixed

- **Prevent False Positive Version Checks**: The release workflow now correctly identifies unpublished versions by checking crates.io instead of relying on git tags, which can exist without the package being published.

### Changed

- Translated all CI/CD scripts from JavaScript (.mjs) to Rust (.rs) using rust-script
- Scripts now use native Rust with rust-script for execution in shell
- Removed Node.js dependency from CI/CD pipeline
- Updated GitHub Actions workflow to use rust-script instead of node
- Updated README and CONTRIBUTING documentation with new script references

### Added

- Cache `restore-keys` for partial cache hits across all workflow jobs
- Explicit `token` parameter in checkout for release jobs
- Code coverage job with `cargo-llvm-cov` and Codecov integration
- Codecov badge in README.md
- Pre-release version support (e.g., `0.1.0-beta.1`) in version parsing
- `--release-label` parameter for multi-language release disambiguation
- `ensure_version_exceeds_published()` logic to prevent publishing duplicate versions
- `get_max_published_version()` to query highest non-yanked version from crates.io
- `max_published_version` output from check-release-needed for downstream use
- Version fallback logic in auto-release Create GitHub Release step

### Changed

- Updated `actions/checkout` from v4 to v6
- Updated `actions/cache` from v4 to v5
- Updated `peter-evans/create-pull-request` from v7 to v8
- Made `publish-crate.rs` fail (exit 1) when version already exists on crates.io
- Improved `create-github-release.rs` to check combined stdout+stderr and detect "Validation Failed"

### Fixed

- Fix publish steps overriding workflow-level CARGO_TOKEN fallback, breaking CARGO_REGISTRY_TOKEN-only configurations (#32)
- Fix non-fast-forward push failures in multi-workflow repos by adding fetch/rebase and push retry logic (#31)
- Add mono-repo path support to check-changelog-fragment.rs, check-version-modification.rs, and create-changelog-fragment.rs
- Add `!cancelled()` guard to test job condition to respect workflow cancellation

### Fixed

- Fixed `version-and-commit.rs` to check crates.io instead of git tags for determining if a version is already released
- This prevents the release pipeline from getting stuck when git tags exist without corresponding crates.io publication

### Added

- Added `--tag-prefix` support to `version-and-commit.rs` for multi-language repository compatibility
- Added crates.io and docs.rs badges to README.md
- Added automatic crates.io and docs.rs badge injection in GitHub release notes
- Added documentation deployment job to CI/CD pipeline (deploys to GitHub Pages after release)
- Added case study documentation for issue #25

### Fixed
- Fixed unsupported look-ahead regex in `create-github-release.rs` that caused a panic when parsing CHANGELOG.md. Replaced with a two-step approach using only features supported by Rust's `regex` crate.

### Changed
- Restructured example application as a simple CLI sum calculator using `lino-arguments`
- Renamed default package to `example-sum-package-name` with Unlicense license
- Reorganized test structure: `tests/unit/sum.rs`, `tests/integration/sum.rs`, `tests/unit/ci-cd/`
- Converted experiment scripts into proper unit tests in `tests/unit/ci-cd/changelog_parsing.rs`
- Added CI/CD skip logic for template default package name `example-sum-package-name`
- Updated README.md badges and documentation

### Fixed

- Change detection script now uses per-commit diff instead of full PR diff, so commits touching only non-code files correctly skip CI jobs even when earlier commits in the same PR changed code files

### Fixed
- Make release scripts resolve the publishable crate manifest when the repository root uses a Cargo workspace manifest.

### Fixed
- Decoupled GitHub Pages documentation deployment from package release publication and fixed release-script warning failures under `RUSTFLAGS=-Dwarnings`.

### Changed
- Added explicit GitHub Actions job timeouts and documented Rust test timeout guidance.

### Fixed
- Added a non-blocking warning threshold to the Rust file-size check so near-limit files are surfaced before concurrent PR merges can exceed the hard limit.

### Fixed
- Made `create-github-release.rs` build GitHub release titles as `[Language] X.Y.Z` instead of reusing the tag prefix.

### Added
- Added optional Docker Hub image publishing tied to Rust crate releases, including crates.io visibility waiting, version/latest image tags, and Docker Hub badges in GitHub release notes.

### Changed
- Release completeness checks now self-heal when crates.io exists but configured Docker Hub or GitHub release artifacts are missing.

### Fixed
- Switched documentation deployment to the official GitHub Pages artifact workflow so repositories using GitHub Actions as their Pages source do not get false-positive branch-push deploys.

### Added
- Documented the one-time `Settings → Pages → Source = GitHub Actions` prerequisite for the `deploy-docs` job in `README.md` and as a comment above the `deploy-docs` job in `release.yml`, so downstream template users hit a documented setup step instead of a `Get Pages site failed` error on the first deploy.

### Added
- Tracking case study at `docs/case-studies/issue-52/` registering the `browser-commander` + Playwright preview-regeneration pattern from [`konard/vk-bot-desktop#52`](https://github.com/konard/vk-bot-desktop/pull/52), with an activation checklist for when an example-app surface lands in this template. Documentation only — no workflow, script, or runtime code changes. Primary upstream tracking issue: [`link-foundation/js-ai-driven-development-pipeline-template#62`](https://github.com/link-foundation/js-ai-driven-development-pipeline-template/issues/62).

### Fixed
- Release automation now keeps the workspace package entry in `Cargo.lock` synchronized when `scripts/version-and-commit.rs` bumps `Cargo.toml`, preventing stale lock-file version diffs in later pull requests.

### Fixed
- `scripts/publish-crate.rs` now treats crates.io HTTP 429 throttle responses ("You have published too many versions of this crate in the last 24 hours") as a deferred `publish_result=rate_limited` outcome (it writes the output, prints an explanatory banner and exits successfully) instead of a hard CI failure reported as a generic `failed` ("Failed to publish for unknown reason"). Authentication, already-published and unknown failures still exit non-zero. Failed-publish classification is consolidated through a single `classify_failure` function and `FailureKind` enum (with an `is_deferred` predicate), covered by unit tests runnable via `rust-script --test scripts/publish-crate.rs`.
- The release workflow (`.github/workflows/release.yml`) now gates crate-availability waiting, Docker Hub publishing and GitHub release creation on either an already-published crate or `publish_result=success`, so a deferred (rate-limited) crate upload no longer produces partial downstream release artifacts and the same version is retried automatically on the next push to `main`.

### Fixed
- Fixed reversed `cancel-in-progress` concurrency condition in `release.yml` that cancelled in-flight releases on `main` and never superseded older PR runs. The condition now uses `!=` so `main` releases run to completion while newer PR pushes cancel stale runs.

### Added
- Added a `scripts/check-crate-size.rs` guard that builds the `.crate` archive and fails the release before publishing when it exceeds the crates.io 10 MiB upload limit. The check runs in the build job and before publishing in both the auto-release and manual-release jobs.

### Changed
- Added a narrow `include` allowlist to `Cargo.toml` so docs, case studies, generated CI artifacts, changelog fragments, scripts, and experiments no longer inflate the published release archive.

### Fixed
- Prevented GitHub release creation from treating generic API validation failures as existing releases, and capped oversized release notes with a link to the full tagged changelog.

### Added
- Added the initial `meta-language` links-network core with self-description roots,
  verification, trivia attachment configuration, and a minimal CLI.
- Added the default lossless `parse` entry point, projection views, and a
  tested parity roadmap for competitor features, grammar embedding, and language
  coverage targets.
- Added exact reconstruction, mixed-region detection, query matching,
  substitution rules, concept reconstruction, object identity helpers,
  many-valued truth values, and executable parity fixtures for every tracked
  competitor target.
- Added executable language fixtures and tests for every requested Markdown,
  HTML, top-ten programming-language, and top-ten natural-language target.

### Fixed
- Added a CI guard that rejects Rust test modules and test attributes under `src/`, keeping tests in the `tests/` tree.

## Minor Changes

- Add immutable and mutable network snapshots with provenance and forward
  version commits for roadmap snapshot/versioning coverage.

### Added
- Added `txt` as a first-class markup/container target with a UTF-8 lossless
  fixture and content-sniffing fallback regions.

### Fixed
- Reconciled the natural-language target ordering with the Ethnologue 2025
  total-speaker order cited by the parity roadmap.

### Added
- Added a grammar-backed parser front end for Python, C, Java, C++, C#, JavaScript, and R using official tree-sitter grammar crates.

### Added
- Parse supported mixed-mode embedded regions into the host links network with grammar-backed syntax links.

### Added
- Added natural-language segmentation, identification, normalization, and bidi annotation links over lossless text parses.

### Added
- Added grammar-backed Visual Basic parsing with byte-exact reconstruction and recovery flag coverage.

bump: minor

- Add a grammar-backed `sql-ansi` SQL-family dialect fixture using
  `tree-sitter-sequel`.

### Added
- Added grammar-backed Delphi/Object Pascal parsing through `tree-sitter-pascal`.

### Added
- Enriched link queries with S-expression structural matching, captures, by-type construction, host predicate hooks, and link-cli-style variable substitution bindings.

### Added
- Added persistent snapshot structural sharing with interned metadata text storage.

### Added
- Added a query-transform surface for selecting captured links and replacing
  their source text while preserving unchanged bytes.

### Added
- Materialized self-description roots as controlled links with complete root-definition closure and round-trippable `describe` output.

### Added
- Added common concept ontology seeding from meta-expression's semantic lexicon, with shared concept links, syntax mappings, and structural programming-language concepts.

### Added
- Added semantic cross-language reconstruction for the Hawaii statehood fixture,
  including English/Russian naturalization and configurable formalization levels.

### Added
- Expanded `PARITY_FIXTURES` with multiple provenance-tracked fixtures for tree-sitter, LibCST, Recast, jscodeshift, Rowan, cstree, and Roslyn, including executable recovery and query/transform expectations.

### Added

- Expanded `PARITY_FIXTURES` with upstream-provenanced internal ecosystem corpora for links-notation, link-cli, lino-objects-codec, relative-meta-logic, formal-ai, and meta-expression.

### Added

- Added structural LiNo parsing for links-notation doublets, triplets, named links, indented IDs, and self-references while preserving byte-exact reconstruction.

### Added
- Added fixed-point probabilistic truth values for relative-meta-logic-style
  semantic confidence evaluation.

### Added
- Added grammar-backed Go parsing via the official `tree-sitter-go` grammar
  (accepts `Go`, `go`, and `golang`) so `LinkNetwork::parse` emits real
  `LinkType::Syntax` concrete-syntax links with byte-exact reconstruction.

### Added
- `AccessMode { Mutable, ReadOnly }` setting on `ParseConfiguration`
  (`with_access_mode` / `access_mode`), defaulting to `Mutable` so existing
  callers are unaffected.
- `LinkNetwork::freeze` / `as_read_only` yielding a `ReadOnlyNetwork` view that
  exposes only `&self` operations (query, project, reconstruct, verify,
  serialize); mutators are unreachable at compile time because the view never
  hands out `&mut LinkNetwork`.
- `LinkNetwork::parse_engine`, returning an `EngineNetwork` handle that honours
  the configured access mode: read-only parsing returns the frozen form and
  `EngineNetwork::as_mutable` rejects mutation with a `ReadOnlyViolation`
  diagnostic.
- Snapshot interop: `NetworkSnapshot::as_read_only` / `from_read_only` reuse the
  snapshot's `Arc<LinkNetwork>`, so the frozen form composes with snapshot
  versioning instead of duplicating it.

### Added
- Added grammar-backed Ruby parsing through `tree-sitter-ruby`, so
  `LinkNetwork::parse(source, "Ruby", ...)` (and the `rb` alias) now emits real
  `LinkType::Syntax` links instead of falling back to lossless plain text.

bump: minor

- Add a grammar-backed `TypeScript` front end using `tree-sitter-typescript`,
  wiring the `typescript`/`ts` labels to `LANGUAGE_TYPESCRIPT` and the `tsx`
  label to `LANGUAGE_TSX` so `LinkNetwork::parse` emits real `LinkType::Syntax`
  links for TypeScript and TSX sources.

### Added
- Added grammar-backed parsing for seven data-exchange / interchange formats
  through `src/tree_sitter_adapter.rs`: JSON (`tree-sitter-json`), YAML
  (`tree-sitter-yaml`, accepts `yaml`/`yml`), TOML (`tree-sitter-toml-ng`), XML
  and DTD (`tree-sitter-xml`), INI (`tree-sitter-ini`), Protocol Buffers
  (`tree-sitter-proto`, accepts `protobuf`/`proto`/`Protocol Buffers`), and
  GraphQL (`tree-sitter-graphql`, accepts `graphql`/`gql`). Each parses into
  real `LinkType::Syntax` concrete-syntax links and reconstructs byte-for-byte.
- Added a `DATA_FORMAT_TARGETS` registry in `src/parity.rs` (with the new
  `LanguageFamily::DataFormat`) gated by parity tests, mirroring
  `MARKUP_LANGUAGE_TARGETS`.
- Added per-format UTF-8 `LANGUAGE_FIXTURES` round-trip entries plus
  recovery-case and mixed-region tests (a `json` fence embedded in Markdown
  parses into the host links network).
- Added lossless CSV and JSON5 parsers for the two formats whose published
  tree-sitter crates still pin the incompatible `tree-sitter ~0.20` runtime.
  CSV is validated with the Rust `csv` crate and JSON5 is validated with
  `json5_nodes`; both emit structured syntax links and reconstruct
  byte-for-byte.

### Documentation
- Documented the nine wired data-format parsers (parser, version, license, root
  node) in `docs/parity-roadmap.md`, including the tree-sitter compatibility
  rationale for the in-repo CSV and JSON5 parsers.

### Added
- `LinkNetwork::to_lino` serializes an entire links network to canonical
  links-notation text, keyed by each link's numeric id (doublets-style id
  discipline), covering references, names, types, terms, definitions,
  languages, source spans, parse flags, and term registration.
- `LinkNetwork::from_lino` reconstructs the exact network from that text,
  forming a lossless round-trip (`from_lino(to_lino(n))` is isomorphic to `n`).
- `LinoSerializationError` reports parse and schema failures from `from_lino`.
- A round-trip property test over every language fixture plus synthetic
  networks, and a test that `to_lino` output is accepted by the
  `links-notation` 0.13 crate parser.
- `ParityCapability::LinoSerialization` and an output-side serialization
  parity fixture for the `links-notation` target.

### Changed
- Added the `links-notation` 0.13 crate as a dependency so serialized output
  aligns with the wider links-notation ecosystem.

### Added
- `ParserRegistry` maps language keys to `Arc<dyn LanguageParser>` with the
  built-in parser as a fallback. User registrations shadow the built-in
  dispatch for the same (case-insensitive) key; unregistered keys still route
  through the built-in set.
- `LinkNetwork::parse_with_registry` dispatches a parse through a registry.
- `examples/custom_parser_registry.rs` documents registering a custom parser.

### Added
- Added grammar-backed parsing for five popular programming languages
  immediately below the TIOBE top ten through `src/tree_sitter_adapter.rs`: PHP
  (`tree-sitter-php`, via its `LANGUAGE_PHP` symbol), Swift
  (`tree-sitter-swift`), Kotlin (`tree-sitter-kotlin-ng`, accepts `kotlin`/`kt`),
  Scala (`tree-sitter-scala`), and Lua (`tree-sitter-lua`). Each parses into real
  `LinkType::Syntax` concrete-syntax links and reconstructs byte-for-byte.
- Added a `SECOND_TIER_PROGRAMMING_LANGUAGE_TARGETS` registry in `src/parity.rs`
  gated by parity tests, mirroring `DATA_FORMAT_TARGETS`.
- Added per-language UTF-8 `LANGUAGE_FIXTURES` round-trip entries plus
  case-insensitive alias coverage and a per-language recovery fixture whose
  malformed source still reconstructs while exposing error/missing diagnostics.

### Documentation
- Documented the wired grammars (crate, version, license, root node) in
  `docs/parity-roadmap.md` and kept the temporary Perl follow-up tracked
  explicitly until `ts-parser-perl` was adopted.

### Added
- Add a Rust `ToLinks`/`FromLinks` codec with queryable type-shape links and shared/circular object graph round-trips through LiNo serialization.

### Added

- Added the `LinkStore` storage trait, read-only-aware storage wrappers, and an
  optional file-mapped `doublets` backend with LiNo/binary round-trip coverage.

### Added
- Added exact-match concept interning, language-bound expression links, external-id alias links, and LiNo concept-set import for the shared concept ontology.

### Added
- Added starter natural-language grammaticality parsing with UD-style morphosyntax links, pass/fail fixtures for the ten natural-language targets, and recoverable error links for ungrammatical fixtures.

### Added
- Added configurable `TranslationRuleSet` values, a runtime
  `TranslationRuleRegistry`, LiNo-backed rule-set loading, template
  placeholders, and missing-rule diagnostics for from-meta reconstruction.

### Added
- Added queryable language profiles with JavaScript transform enforcement and rule-set-derived profile domains.

### Added
- Added incremental source edit reparsing, stable outside-edit link IDs, and structural snapshot diff reporting.

### Added
- Added composable `LinkRule` query algebra with relational rules, boolean composition, named sub-rules, ellipsis gap matching, typed metavariables, plain-text token patterns, traversal strategies, and valid/invalid rule snapshot suites.
- Added quasiquote replacement templates with placeholder validation and parenthesization-conservative captured-text replacement.

### Added
- Add the API operation/style parity registry, fluent network pipeline, and link-cli-style substitution text runner.

### Added
- Wire the canonical `ts-parser-perl` tree-sitter grammar as a second-tier
  programming-language target without upgrading the project-wide tree-sitter
  runtime.

### Added
- Added source generation helpers for constructed syntax networks: `insert_source_token`, `insert_syntax_node`, `render_source`, `render_source_from`, and `render_source_from_document`.

### Added
- Added wave-two competitor and ecosystem parity fixtures with executable transform, reconstruction, grammar, and storage gates.
- Added a recorded `cargo llvm-cov` line-coverage floor to CI so coverage cannot silently regress.

### Added
- Shared, language-free document-formatting concept ontology (`document_formatting` module): inline concepts (emphasis, strong, strikethrough, inline-code, hyperlink, image, line-break) and block concepts (heading with level, paragraph, blockquote, bullet/ordered lists, list-item, code-block with language, thematic-break, table/row/cell), each seeded with Markdown and HTML syntax mappings.
- `seed_common_concept_ontology()` now also seeds the formatting concepts, and `ConceptOntologySeedReport::formatting_concepts()` reports how many were added.
- `LinkNetwork::resolve_document_format` / `render_document_format` / `translate_document_format` so the same concept link reconstructs as `**…**` in Markdown and `<strong>…</strong>` in HTML; Markdown `**bold**` and HTML `<strong>bold</strong>` reach the one shared `strong` concept under semantic projection.
- `FormattingDocument` concept layer with `parse_markup_document` and `translate_markup_document` for full Markdown ⇄ HTML document round-trips through one concept ontology (heading/paragraph/list/bold/italic/link).

## [0.40.0] - 2026-06-12

### Added
- Changeset-style fragment format with frontmatter for specifying version bump type
- New `get-bump-type.mjs` script to automatically determine version bump from fragments
- Automatic version bumping on merge to main based on changelog fragments
- Detailed documentation for the changelog fragment system in `changelog.d/README.md`

### Changed
- Updated `collect-changelog.mjs` to strip frontmatter when collecting fragments
- Updated `version-and-commit.mjs` to handle frontmatter in fragments
- Enhanced release workflow to automatically determine bump type from changesets

### Changed
- Add `detect-changes` job with cross-platform `detect-code-changes.mjs` script
- Make lint job independent of changelog check (runs based on file changes only)
- Allow docs-only PRs without changelog fragment requirement
- Handle changelog check 'skipped' state in dependent jobs
- Exclude `changelog.d/`, `docs/`, `experiments/`, `examples/` folders and markdown files from code changes detection

### Fixed
- Fixed README.md to correctly reference Node.js scripts (`.mjs`) instead of Python scripts (`.py`)
- Updated project structure in README.md to match actual script files in `scripts/` directory
- Fixed example code in README.md that had invalid Rust with two `main` functions

### Added

- Added crates.io publishing support to CI/CD workflow
- Added `release_mode` input with "instant" and "changelog-pr" options for manual releases
- Added `--tag-prefix` and `--crates-io-url` options to create-github-release.mjs script
- Added comprehensive case study documentation for Issue #11 in docs/case-studies/issue-11/

### Changed

- Changed changelog fragment check from warning to error (exit 1) to enforce changelog requirements
- Updated job conditions with `always() && !cancelled()` to fix workflow_dispatch job skipping issue
- Renamed manual-release job to "Instant Release" for clarity

### Fixed

- Fixed deprecated `::set-output` GitHub Actions command in version-and-commit.mjs
- Fixed workflow_dispatch triggering issues where lint/build/release jobs were incorrectly skipped

### Fixed

- Fixed changelog fragment check to validate that a fragment is **added in the PR diff** rather than just checking if any fragments exist in the directory. This prevents the check from incorrectly passing when there are leftover fragments from previous PRs that haven't been released yet.

### Changed

- Converted shell scripts in `release.yml` to cross-platform `.mjs` scripts for improved portability and performance:
  - `check-changelog-fragment.mjs` - validates changelog fragment is added in PR diff
  - `git-config.mjs` - configures git user for CI/CD
  - `check-release-needed.mjs` - checks if release is needed
  - `publish-crate.mjs` - publishes package to crates.io
  - `create-changelog-fragment.mjs` - creates changelog fragments for manual releases
  - `get-version.mjs` - gets current version from Cargo.toml

### Added

- Added `check-version-modification.mjs` script to detect manual version changes in Cargo.toml
- Added `version-check` job to CI/CD workflow that runs on pull requests
- Added skip logic for automated release branches (changelog-manual-release-*, changeset-release/*, release/*, automated-release/*)

### Changed

- Version modifications in Cargo.toml are now blocked in pull requests to enforce automated release pipeline

### Added

- Added support for `CARGO_REGISTRY_TOKEN` as alternative to `CARGO_TOKEN` for crates.io publishing
- Added case study documentation for Issue #17 (yargs reserved word and dual token support)

### Changed

- Updated workflow to use fallback logic: `${{ secrets.CARGO_REGISTRY_TOKEN || secrets.CARGO_TOKEN }}`
- Improved publish-crate.mjs to check both `CARGO_REGISTRY_TOKEN` and `CARGO_TOKEN` environment variables
- Added warning message when neither token is set

### Added
- New `scripts/rust-paths.mjs` utility for automatic Rust package root detection
- Support for both single-language and multi-language repository structures in all CI/CD scripts
- Configuration options via `--rust-root` CLI argument and `RUST_ROOT` environment variable
- Comprehensive case study documentation in `docs/case-studies/issue-19/`

### Changed
- Updated all release scripts to use the new path detection utility:
  - `scripts/bump-version.mjs`
  - `scripts/check-release-needed.mjs`
  - `scripts/collect-changelog.mjs`
  - `scripts/get-bump-type.mjs`
  - `scripts/get-version.mjs`
  - `scripts/publish-crate.mjs`
  - `scripts/version-and-commit.mjs`

### Changed

- **check-release-needed.mjs**: Now checks crates.io API directly instead of git tags to determine if a version is already released. This prevents false positives where git tags exist but the package was never actually published to crates.io.

### Added

- **CI/CD Troubleshooting Guide**: New documentation at `docs/ci-cd/troubleshooting.md` covering common issues like skipped jobs, false positive version checks, publishing failures, and secret configuration.

- **Enhanced Error Handling in publish-crate.mjs**: Added specific detection and helpful error messages for authentication failures, including guidance on secret configuration and workflow setup.

- **Case Study Documentation**: Added comprehensive case study at `docs/case-studies/issue-21/` analyzing CI/CD failures from browser-commander repository (issues #27, #29, #31, #33) with timeline, root causes, and lessons learned.

### Fixed

- **Prevent False Positive Version Checks**: The release workflow now correctly identifies unpublished versions by checking crates.io instead of relying on git tags, which can exist without the package being published.

### Changed

- Translated all CI/CD scripts from JavaScript (.mjs) to Rust (.rs) using rust-script
- Scripts now use native Rust with rust-script for execution in shell
- Removed Node.js dependency from CI/CD pipeline
- Updated GitHub Actions workflow to use rust-script instead of node
- Updated README and CONTRIBUTING documentation with new script references

### Added

- Cache `restore-keys` for partial cache hits across all workflow jobs
- Explicit `token` parameter in checkout for release jobs
- Code coverage job with `cargo-llvm-cov` and Codecov integration
- Codecov badge in README.md
- Pre-release version support (e.g., `0.1.0-beta.1`) in version parsing
- `--release-label` parameter for multi-language release disambiguation
- `ensure_version_exceeds_published()` logic to prevent publishing duplicate versions
- `get_max_published_version()` to query highest non-yanked version from crates.io
- `max_published_version` output from check-release-needed for downstream use
- Version fallback logic in auto-release Create GitHub Release step

### Changed

- Updated `actions/checkout` from v4 to v6
- Updated `actions/cache` from v4 to v5
- Updated `peter-evans/create-pull-request` from v7 to v8
- Made `publish-crate.rs` fail (exit 1) when version already exists on crates.io
- Improved `create-github-release.rs` to check combined stdout+stderr and detect "Validation Failed"

### Fixed

- Fix publish steps overriding workflow-level CARGO_TOKEN fallback, breaking CARGO_REGISTRY_TOKEN-only configurations (#32)
- Fix non-fast-forward push failures in multi-workflow repos by adding fetch/rebase and push retry logic (#31)
- Add mono-repo path support to check-changelog-fragment.rs, check-version-modification.rs, and create-changelog-fragment.rs
- Add `!cancelled()` guard to test job condition to respect workflow cancellation

### Fixed

- Fixed `version-and-commit.rs` to check crates.io instead of git tags for determining if a version is already released
- This prevents the release pipeline from getting stuck when git tags exist without corresponding crates.io publication

### Added

- Added `--tag-prefix` support to `version-and-commit.rs` for multi-language repository compatibility
- Added crates.io and docs.rs badges to README.md
- Added automatic crates.io and docs.rs badge injection in GitHub release notes
- Added documentation deployment job to CI/CD pipeline (deploys to GitHub Pages after release)
- Added case study documentation for issue #25

### Fixed
- Fixed unsupported look-ahead regex in `create-github-release.rs` that caused a panic when parsing CHANGELOG.md. Replaced with a two-step approach using only features supported by Rust's `regex` crate.

### Changed
- Restructured example application as a simple CLI sum calculator using `lino-arguments`
- Renamed default package to `example-sum-package-name` with Unlicense license
- Reorganized test structure: `tests/unit/sum.rs`, `tests/integration/sum.rs`, `tests/unit/ci-cd/`
- Converted experiment scripts into proper unit tests in `tests/unit/ci-cd/changelog_parsing.rs`
- Added CI/CD skip logic for template default package name `example-sum-package-name`
- Updated README.md badges and documentation

### Fixed

- Change detection script now uses per-commit diff instead of full PR diff, so commits touching only non-code files correctly skip CI jobs even when earlier commits in the same PR changed code files

### Fixed
- Make release scripts resolve the publishable crate manifest when the repository root uses a Cargo workspace manifest.

### Fixed
- Decoupled GitHub Pages documentation deployment from package release publication and fixed release-script warning failures under `RUSTFLAGS=-Dwarnings`.

### Changed
- Added explicit GitHub Actions job timeouts and documented Rust test timeout guidance.

### Fixed
- Added a non-blocking warning threshold to the Rust file-size check so near-limit files are surfaced before concurrent PR merges can exceed the hard limit.

### Fixed
- Made `create-github-release.rs` build GitHub release titles as `[Language] X.Y.Z` instead of reusing the tag prefix.

### Added
- Added optional Docker Hub image publishing tied to Rust crate releases, including crates.io visibility waiting, version/latest image tags, and Docker Hub badges in GitHub release notes.

### Changed
- Release completeness checks now self-heal when crates.io exists but configured Docker Hub or GitHub release artifacts are missing.

### Fixed
- Switched documentation deployment to the official GitHub Pages artifact workflow so repositories using GitHub Actions as their Pages source do not get false-positive branch-push deploys.

### Added
- Documented the one-time `Settings → Pages → Source = GitHub Actions` prerequisite for the `deploy-docs` job in `README.md` and as a comment above the `deploy-docs` job in `release.yml`, so downstream template users hit a documented setup step instead of a `Get Pages site failed` error on the first deploy.

### Added
- Tracking case study at `docs/case-studies/issue-52/` registering the `browser-commander` + Playwright preview-regeneration pattern from [`konard/vk-bot-desktop#52`](https://github.com/konard/vk-bot-desktop/pull/52), with an activation checklist for when an example-app surface lands in this template. Documentation only — no workflow, script, or runtime code changes. Primary upstream tracking issue: [`link-foundation/js-ai-driven-development-pipeline-template#62`](https://github.com/link-foundation/js-ai-driven-development-pipeline-template/issues/62).

### Fixed
- Release automation now keeps the workspace package entry in `Cargo.lock` synchronized when `scripts/version-and-commit.rs` bumps `Cargo.toml`, preventing stale lock-file version diffs in later pull requests.

### Fixed
- `scripts/publish-crate.rs` now treats crates.io HTTP 429 throttle responses ("You have published too many versions of this crate in the last 24 hours") as a deferred `publish_result=rate_limited` outcome (it writes the output, prints an explanatory banner and exits successfully) instead of a hard CI failure reported as a generic `failed` ("Failed to publish for unknown reason"). Authentication, already-published and unknown failures still exit non-zero. Failed-publish classification is consolidated through a single `classify_failure` function and `FailureKind` enum (with an `is_deferred` predicate), covered by unit tests runnable via `rust-script --test scripts/publish-crate.rs`.
- The release workflow (`.github/workflows/release.yml`) now gates crate-availability waiting, Docker Hub publishing and GitHub release creation on either an already-published crate or `publish_result=success`, so a deferred (rate-limited) crate upload no longer produces partial downstream release artifacts and the same version is retried automatically on the next push to `main`.

### Fixed
- Fixed reversed `cancel-in-progress` concurrency condition in `release.yml` that cancelled in-flight releases on `main` and never superseded older PR runs. The condition now uses `!=` so `main` releases run to completion while newer PR pushes cancel stale runs.

### Added
- Added a `scripts/check-crate-size.rs` guard that builds the `.crate` archive and fails the release before publishing when it exceeds the crates.io 10 MiB upload limit. The check runs in the build job and before publishing in both the auto-release and manual-release jobs.

### Changed
- Added a narrow `include` allowlist to `Cargo.toml` so docs, case studies, generated CI artifacts, changelog fragments, scripts, and experiments no longer inflate the published release archive.

### Fixed
- Prevented GitHub release creation from treating generic API validation failures as existing releases, and capped oversized release notes with a link to the full tagged changelog.

### Added
- Added the initial `meta-language` links-network core with self-description roots,
  verification, trivia attachment configuration, and a minimal CLI.
- Added the default lossless `parse` entry point, projection views, and a
  tested parity roadmap for competitor features, grammar embedding, and language
  coverage targets.
- Added exact reconstruction, mixed-region detection, query matching,
  substitution rules, concept reconstruction, object identity helpers,
  many-valued truth values, and executable parity fixtures for every tracked
  competitor target.
- Added executable language fixtures and tests for every requested Markdown,
  HTML, top-ten programming-language, and top-ten natural-language target.

### Fixed
- Added a CI guard that rejects Rust test modules and test attributes under `src/`, keeping tests in the `tests/` tree.

## Minor Changes

- Add immutable and mutable network snapshots with provenance and forward
  version commits for roadmap snapshot/versioning coverage.

### Added
- Added `txt` as a first-class markup/container target with a UTF-8 lossless
  fixture and content-sniffing fallback regions.

### Fixed
- Reconciled the natural-language target ordering with the Ethnologue 2025
  total-speaker order cited by the parity roadmap.

### Added
- Added a grammar-backed parser front end for Python, C, Java, C++, C#, JavaScript, and R using official tree-sitter grammar crates.

### Added
- Parse supported mixed-mode embedded regions into the host links network with grammar-backed syntax links.

### Added
- Added natural-language segmentation, identification, normalization, and bidi annotation links over lossless text parses.

### Added
- Added grammar-backed Visual Basic parsing with byte-exact reconstruction and recovery flag coverage.

bump: minor

- Add a grammar-backed `sql-ansi` SQL-family dialect fixture using
  `tree-sitter-sequel`.

### Added
- Added grammar-backed Delphi/Object Pascal parsing through `tree-sitter-pascal`.

### Added
- Enriched link queries with S-expression structural matching, captures, by-type construction, host predicate hooks, and link-cli-style variable substitution bindings.

### Added
- Added persistent snapshot structural sharing with interned metadata text storage.

### Added
- Added a query-transform surface for selecting captured links and replacing
  their source text while preserving unchanged bytes.

### Added
- Materialized self-description roots as controlled links with complete root-definition closure and round-trippable `describe` output.

### Added
- Added common concept ontology seeding from meta-expression's semantic lexicon, with shared concept links, syntax mappings, and structural programming-language concepts.

### Added
- Added semantic cross-language reconstruction for the Hawaii statehood fixture,
  including English/Russian naturalization and configurable formalization levels.

### Added
- Expanded `PARITY_FIXTURES` with multiple provenance-tracked fixtures for tree-sitter, LibCST, Recast, jscodeshift, Rowan, cstree, and Roslyn, including executable recovery and query/transform expectations.

### Added

- Expanded `PARITY_FIXTURES` with upstream-provenanced internal ecosystem corpora for links-notation, link-cli, lino-objects-codec, relative-meta-logic, formal-ai, and meta-expression.

### Added

- Added structural LiNo parsing for links-notation doublets, triplets, named links, indented IDs, and self-references while preserving byte-exact reconstruction.

### Added
- Added fixed-point probabilistic truth values for relative-meta-logic-style
  semantic confidence evaluation.

### Added
- Added grammar-backed Go parsing via the official `tree-sitter-go` grammar
  (accepts `Go`, `go`, and `golang`) so `LinkNetwork::parse` emits real
  `LinkType::Syntax` concrete-syntax links with byte-exact reconstruction.

### Added
- `AccessMode { Mutable, ReadOnly }` setting on `ParseConfiguration`
  (`with_access_mode` / `access_mode`), defaulting to `Mutable` so existing
  callers are unaffected.
- `LinkNetwork::freeze` / `as_read_only` yielding a `ReadOnlyNetwork` view that
  exposes only `&self` operations (query, project, reconstruct, verify,
  serialize); mutators are unreachable at compile time because the view never
  hands out `&mut LinkNetwork`.
- `LinkNetwork::parse_engine`, returning an `EngineNetwork` handle that honours
  the configured access mode: read-only parsing returns the frozen form and
  `EngineNetwork::as_mutable` rejects mutation with a `ReadOnlyViolation`
  diagnostic.
- Snapshot interop: `NetworkSnapshot::as_read_only` / `from_read_only` reuse the
  snapshot's `Arc<LinkNetwork>`, so the frozen form composes with snapshot
  versioning instead of duplicating it.

### Added
- Added grammar-backed Ruby parsing through `tree-sitter-ruby`, so
  `LinkNetwork::parse(source, "Ruby", ...)` (and the `rb` alias) now emits real
  `LinkType::Syntax` links instead of falling back to lossless plain text.

bump: minor

- Add a grammar-backed `TypeScript` front end using `tree-sitter-typescript`,
  wiring the `typescript`/`ts` labels to `LANGUAGE_TYPESCRIPT` and the `tsx`
  label to `LANGUAGE_TSX` so `LinkNetwork::parse` emits real `LinkType::Syntax`
  links for TypeScript and TSX sources.

### Added
- Added grammar-backed parsing for seven data-exchange / interchange formats
  through `src/tree_sitter_adapter.rs`: JSON (`tree-sitter-json`), YAML
  (`tree-sitter-yaml`, accepts `yaml`/`yml`), TOML (`tree-sitter-toml-ng`), XML
  and DTD (`tree-sitter-xml`), INI (`tree-sitter-ini`), Protocol Buffers
  (`tree-sitter-proto`, accepts `protobuf`/`proto`/`Protocol Buffers`), and
  GraphQL (`tree-sitter-graphql`, accepts `graphql`/`gql`). Each parses into
  real `LinkType::Syntax` concrete-syntax links and reconstructs byte-for-byte.
- Added a `DATA_FORMAT_TARGETS` registry in `src/parity.rs` (with the new
  `LanguageFamily::DataFormat`) gated by parity tests, mirroring
  `MARKUP_LANGUAGE_TARGETS`.
- Added per-format UTF-8 `LANGUAGE_FIXTURES` round-trip entries plus
  recovery-case and mixed-region tests (a `json` fence embedded in Markdown
  parses into the host links network).
- Added lossless CSV and JSON5 parsers for the two formats whose published
  tree-sitter crates still pin the incompatible `tree-sitter ~0.20` runtime.
  CSV is validated with the Rust `csv` crate and JSON5 is validated with
  `json5_nodes`; both emit structured syntax links and reconstruct
  byte-for-byte.

### Documentation
- Documented the nine wired data-format parsers (parser, version, license, root
  node) in `docs/parity-roadmap.md`, including the tree-sitter compatibility
  rationale for the in-repo CSV and JSON5 parsers.

### Added
- `LinkNetwork::to_lino` serializes an entire links network to canonical
  links-notation text, keyed by each link's numeric id (doublets-style id
  discipline), covering references, names, types, terms, definitions,
  languages, source spans, parse flags, and term registration.
- `LinkNetwork::from_lino` reconstructs the exact network from that text,
  forming a lossless round-trip (`from_lino(to_lino(n))` is isomorphic to `n`).
- `LinoSerializationError` reports parse and schema failures from `from_lino`.
- A round-trip property test over every language fixture plus synthetic
  networks, and a test that `to_lino` output is accepted by the
  `links-notation` 0.13 crate parser.
- `ParityCapability::LinoSerialization` and an output-side serialization
  parity fixture for the `links-notation` target.

### Changed
- Added the `links-notation` 0.13 crate as a dependency so serialized output
  aligns with the wider links-notation ecosystem.

### Added
- `ParserRegistry` maps language keys to `Arc<dyn LanguageParser>` with the
  built-in parser as a fallback. User registrations shadow the built-in
  dispatch for the same (case-insensitive) key; unregistered keys still route
  through the built-in set.
- `LinkNetwork::parse_with_registry` dispatches a parse through a registry.
- `examples/custom_parser_registry.rs` documents registering a custom parser.

### Added
- Added grammar-backed parsing for five popular programming languages
  immediately below the TIOBE top ten through `src/tree_sitter_adapter.rs`: PHP
  (`tree-sitter-php`, via its `LANGUAGE_PHP` symbol), Swift
  (`tree-sitter-swift`), Kotlin (`tree-sitter-kotlin-ng`, accepts `kotlin`/`kt`),
  Scala (`tree-sitter-scala`), and Lua (`tree-sitter-lua`). Each parses into real
  `LinkType::Syntax` concrete-syntax links and reconstructs byte-for-byte.
- Added a `SECOND_TIER_PROGRAMMING_LANGUAGE_TARGETS` registry in `src/parity.rs`
  gated by parity tests, mirroring `DATA_FORMAT_TARGETS`.
- Added per-language UTF-8 `LANGUAGE_FIXTURES` round-trip entries plus
  case-insensitive alias coverage and a per-language recovery fixture whose
  malformed source still reconstructs while exposing error/missing diagnostics.

### Documentation
- Documented the wired grammars (crate, version, license, root node) in
  `docs/parity-roadmap.md` and kept the temporary Perl follow-up tracked
  explicitly until `ts-parser-perl` was adopted.

### Added
- Add a Rust `ToLinks`/`FromLinks` codec with queryable type-shape links and shared/circular object graph round-trips through LiNo serialization.

### Added

- Added the `LinkStore` storage trait, read-only-aware storage wrappers, and an
  optional file-mapped `doublets` backend with LiNo/binary round-trip coverage.

### Added
- Added exact-match concept interning, language-bound expression links, external-id alias links, and LiNo concept-set import for the shared concept ontology.

### Added
- Added starter natural-language grammaticality parsing with UD-style morphosyntax links, pass/fail fixtures for the ten natural-language targets, and recoverable error links for ungrammatical fixtures.

### Added
- Added configurable `TranslationRuleSet` values, a runtime
  `TranslationRuleRegistry`, LiNo-backed rule-set loading, template
  placeholders, and missing-rule diagnostics for from-meta reconstruction.

### Added
- Added queryable language profiles with JavaScript transform enforcement and rule-set-derived profile domains.

### Added
- Added incremental source edit reparsing, stable outside-edit link IDs, and structural snapshot diff reporting.

### Added
- Added composable `LinkRule` query algebra with relational rules, boolean composition, named sub-rules, ellipsis gap matching, typed metavariables, plain-text token patterns, traversal strategies, and valid/invalid rule snapshot suites.
- Added quasiquote replacement templates with placeholder validation and parenthesization-conservative captured-text replacement.

### Added
- Add the API operation/style parity registry, fluent network pipeline, and link-cli-style substitution text runner.

### Added
- Wire the canonical `ts-parser-perl` tree-sitter grammar as a second-tier
  programming-language target without upgrading the project-wide tree-sitter
  runtime.

### Added
- Added source generation helpers for constructed syntax networks: `insert_source_token`, `insert_syntax_node`, `render_source`, `render_source_from`, and `render_source_from_document`.

### Added
- Added wave-two competitor and ecosystem parity fixtures with executable transform, reconstruction, grammar, and storage gates.
- Added a recorded `cargo llvm-cov` line-coverage floor to CI so coverage cannot silently regress.

## [0.39.0] - 2026-06-10

### Added
- Changeset-style fragment format with frontmatter for specifying version bump type
- New `get-bump-type.mjs` script to automatically determine version bump from fragments
- Automatic version bumping on merge to main based on changelog fragments
- Detailed documentation for the changelog fragment system in `changelog.d/README.md`

### Changed
- Updated `collect-changelog.mjs` to strip frontmatter when collecting fragments
- Updated `version-and-commit.mjs` to handle frontmatter in fragments
- Enhanced release workflow to automatically determine bump type from changesets

### Changed
- Add `detect-changes` job with cross-platform `detect-code-changes.mjs` script
- Make lint job independent of changelog check (runs based on file changes only)
- Allow docs-only PRs without changelog fragment requirement
- Handle changelog check 'skipped' state in dependent jobs
- Exclude `changelog.d/`, `docs/`, `experiments/`, `examples/` folders and markdown files from code changes detection

### Fixed
- Fixed README.md to correctly reference Node.js scripts (`.mjs`) instead of Python scripts (`.py`)
- Updated project structure in README.md to match actual script files in `scripts/` directory
- Fixed example code in README.md that had invalid Rust with two `main` functions

### Added

- Added crates.io publishing support to CI/CD workflow
- Added `release_mode` input with "instant" and "changelog-pr" options for manual releases
- Added `--tag-prefix` and `--crates-io-url` options to create-github-release.mjs script
- Added comprehensive case study documentation for Issue #11 in docs/case-studies/issue-11/

### Changed

- Changed changelog fragment check from warning to error (exit 1) to enforce changelog requirements
- Updated job conditions with `always() && !cancelled()` to fix workflow_dispatch job skipping issue
- Renamed manual-release job to "Instant Release" for clarity

### Fixed

- Fixed deprecated `::set-output` GitHub Actions command in version-and-commit.mjs
- Fixed workflow_dispatch triggering issues where lint/build/release jobs were incorrectly skipped

### Fixed

- Fixed changelog fragment check to validate that a fragment is **added in the PR diff** rather than just checking if any fragments exist in the directory. This prevents the check from incorrectly passing when there are leftover fragments from previous PRs that haven't been released yet.

### Changed

- Converted shell scripts in `release.yml` to cross-platform `.mjs` scripts for improved portability and performance:
  - `check-changelog-fragment.mjs` - validates changelog fragment is added in PR diff
  - `git-config.mjs` - configures git user for CI/CD
  - `check-release-needed.mjs` - checks if release is needed
  - `publish-crate.mjs` - publishes package to crates.io
  - `create-changelog-fragment.mjs` - creates changelog fragments for manual releases
  - `get-version.mjs` - gets current version from Cargo.toml

### Added

- Added `check-version-modification.mjs` script to detect manual version changes in Cargo.toml
- Added `version-check` job to CI/CD workflow that runs on pull requests
- Added skip logic for automated release branches (changelog-manual-release-*, changeset-release/*, release/*, automated-release/*)

### Changed

- Version modifications in Cargo.toml are now blocked in pull requests to enforce automated release pipeline

### Added

- Added support for `CARGO_REGISTRY_TOKEN` as alternative to `CARGO_TOKEN` for crates.io publishing
- Added case study documentation for Issue #17 (yargs reserved word and dual token support)

### Changed

- Updated workflow to use fallback logic: `${{ secrets.CARGO_REGISTRY_TOKEN || secrets.CARGO_TOKEN }}`
- Improved publish-crate.mjs to check both `CARGO_REGISTRY_TOKEN` and `CARGO_TOKEN` environment variables
- Added warning message when neither token is set

### Added
- New `scripts/rust-paths.mjs` utility for automatic Rust package root detection
- Support for both single-language and multi-language repository structures in all CI/CD scripts
- Configuration options via `--rust-root` CLI argument and `RUST_ROOT` environment variable
- Comprehensive case study documentation in `docs/case-studies/issue-19/`

### Changed
- Updated all release scripts to use the new path detection utility:
  - `scripts/bump-version.mjs`
  - `scripts/check-release-needed.mjs`
  - `scripts/collect-changelog.mjs`
  - `scripts/get-bump-type.mjs`
  - `scripts/get-version.mjs`
  - `scripts/publish-crate.mjs`
  - `scripts/version-and-commit.mjs`

### Changed

- **check-release-needed.mjs**: Now checks crates.io API directly instead of git tags to determine if a version is already released. This prevents false positives where git tags exist but the package was never actually published to crates.io.

### Added

- **CI/CD Troubleshooting Guide**: New documentation at `docs/ci-cd/troubleshooting.md` covering common issues like skipped jobs, false positive version checks, publishing failures, and secret configuration.

- **Enhanced Error Handling in publish-crate.mjs**: Added specific detection and helpful error messages for authentication failures, including guidance on secret configuration and workflow setup.

- **Case Study Documentation**: Added comprehensive case study at `docs/case-studies/issue-21/` analyzing CI/CD failures from browser-commander repository (issues #27, #29, #31, #33) with timeline, root causes, and lessons learned.

### Fixed

- **Prevent False Positive Version Checks**: The release workflow now correctly identifies unpublished versions by checking crates.io instead of relying on git tags, which can exist without the package being published.

### Changed

- Translated all CI/CD scripts from JavaScript (.mjs) to Rust (.rs) using rust-script
- Scripts now use native Rust with rust-script for execution in shell
- Removed Node.js dependency from CI/CD pipeline
- Updated GitHub Actions workflow to use rust-script instead of node
- Updated README and CONTRIBUTING documentation with new script references

### Added

- Cache `restore-keys` for partial cache hits across all workflow jobs
- Explicit `token` parameter in checkout for release jobs
- Code coverage job with `cargo-llvm-cov` and Codecov integration
- Codecov badge in README.md
- Pre-release version support (e.g., `0.1.0-beta.1`) in version parsing
- `--release-label` parameter for multi-language release disambiguation
- `ensure_version_exceeds_published()` logic to prevent publishing duplicate versions
- `get_max_published_version()` to query highest non-yanked version from crates.io
- `max_published_version` output from check-release-needed for downstream use
- Version fallback logic in auto-release Create GitHub Release step

### Changed

- Updated `actions/checkout` from v4 to v6
- Updated `actions/cache` from v4 to v5
- Updated `peter-evans/create-pull-request` from v7 to v8
- Made `publish-crate.rs` fail (exit 1) when version already exists on crates.io
- Improved `create-github-release.rs` to check combined stdout+stderr and detect "Validation Failed"

### Fixed

- Fix publish steps overriding workflow-level CARGO_TOKEN fallback, breaking CARGO_REGISTRY_TOKEN-only configurations (#32)
- Fix non-fast-forward push failures in multi-workflow repos by adding fetch/rebase and push retry logic (#31)
- Add mono-repo path support to check-changelog-fragment.rs, check-version-modification.rs, and create-changelog-fragment.rs
- Add `!cancelled()` guard to test job condition to respect workflow cancellation

### Fixed

- Fixed `version-and-commit.rs` to check crates.io instead of git tags for determining if a version is already released
- This prevents the release pipeline from getting stuck when git tags exist without corresponding crates.io publication

### Added

- Added `--tag-prefix` support to `version-and-commit.rs` for multi-language repository compatibility
- Added crates.io and docs.rs badges to README.md
- Added automatic crates.io and docs.rs badge injection in GitHub release notes
- Added documentation deployment job to CI/CD pipeline (deploys to GitHub Pages after release)
- Added case study documentation for issue #25

### Fixed
- Fixed unsupported look-ahead regex in `create-github-release.rs` that caused a panic when parsing CHANGELOG.md. Replaced with a two-step approach using only features supported by Rust's `regex` crate.

### Changed
- Restructured example application as a simple CLI sum calculator using `lino-arguments`
- Renamed default package to `example-sum-package-name` with Unlicense license
- Reorganized test structure: `tests/unit/sum.rs`, `tests/integration/sum.rs`, `tests/unit/ci-cd/`
- Converted experiment scripts into proper unit tests in `tests/unit/ci-cd/changelog_parsing.rs`
- Added CI/CD skip logic for template default package name `example-sum-package-name`
- Updated README.md badges and documentation

### Fixed

- Change detection script now uses per-commit diff instead of full PR diff, so commits touching only non-code files correctly skip CI jobs even when earlier commits in the same PR changed code files

### Fixed
- Make release scripts resolve the publishable crate manifest when the repository root uses a Cargo workspace manifest.

### Fixed
- Decoupled GitHub Pages documentation deployment from package release publication and fixed release-script warning failures under `RUSTFLAGS=-Dwarnings`.

### Changed
- Added explicit GitHub Actions job timeouts and documented Rust test timeout guidance.

### Fixed
- Added a non-blocking warning threshold to the Rust file-size check so near-limit files are surfaced before concurrent PR merges can exceed the hard limit.

### Fixed
- Made `create-github-release.rs` build GitHub release titles as `[Language] X.Y.Z` instead of reusing the tag prefix.

### Added
- Added optional Docker Hub image publishing tied to Rust crate releases, including crates.io visibility waiting, version/latest image tags, and Docker Hub badges in GitHub release notes.

### Changed
- Release completeness checks now self-heal when crates.io exists but configured Docker Hub or GitHub release artifacts are missing.

### Fixed
- Switched documentation deployment to the official GitHub Pages artifact workflow so repositories using GitHub Actions as their Pages source do not get false-positive branch-push deploys.

### Added
- Documented the one-time `Settings → Pages → Source = GitHub Actions` prerequisite for the `deploy-docs` job in `README.md` and as a comment above the `deploy-docs` job in `release.yml`, so downstream template users hit a documented setup step instead of a `Get Pages site failed` error on the first deploy.

### Added
- Tracking case study at `docs/case-studies/issue-52/` registering the `browser-commander` + Playwright preview-regeneration pattern from [`konard/vk-bot-desktop#52`](https://github.com/konard/vk-bot-desktop/pull/52), with an activation checklist for when an example-app surface lands in this template. Documentation only — no workflow, script, or runtime code changes. Primary upstream tracking issue: [`link-foundation/js-ai-driven-development-pipeline-template#62`](https://github.com/link-foundation/js-ai-driven-development-pipeline-template/issues/62).

### Fixed
- Release automation now keeps the workspace package entry in `Cargo.lock` synchronized when `scripts/version-and-commit.rs` bumps `Cargo.toml`, preventing stale lock-file version diffs in later pull requests.

### Fixed
- `scripts/publish-crate.rs` now treats crates.io HTTP 429 throttle responses ("You have published too many versions of this crate in the last 24 hours") as a deferred `publish_result=rate_limited` outcome (it writes the output, prints an explanatory banner and exits successfully) instead of a hard CI failure reported as a generic `failed` ("Failed to publish for unknown reason"). Authentication, already-published and unknown failures still exit non-zero. Failed-publish classification is consolidated through a single `classify_failure` function and `FailureKind` enum (with an `is_deferred` predicate), covered by unit tests runnable via `rust-script --test scripts/publish-crate.rs`.
- The release workflow (`.github/workflows/release.yml`) now gates crate-availability waiting, Docker Hub publishing and GitHub release creation on either an already-published crate or `publish_result=success`, so a deferred (rate-limited) crate upload no longer produces partial downstream release artifacts and the same version is retried automatically on the next push to `main`.

### Fixed
- Fixed reversed `cancel-in-progress` concurrency condition in `release.yml` that cancelled in-flight releases on `main` and never superseded older PR runs. The condition now uses `!=` so `main` releases run to completion while newer PR pushes cancel stale runs.

### Added
- Added a `scripts/check-crate-size.rs` guard that builds the `.crate` archive and fails the release before publishing when it exceeds the crates.io 10 MiB upload limit. The check runs in the build job and before publishing in both the auto-release and manual-release jobs.

### Changed
- Added a narrow `include` allowlist to `Cargo.toml` so docs, case studies, generated CI artifacts, changelog fragments, scripts, and experiments no longer inflate the published release archive.

### Fixed
- Prevented GitHub release creation from treating generic API validation failures as existing releases, and capped oversized release notes with a link to the full tagged changelog.

### Added
- Added the initial `meta-language` links-network core with self-description roots,
  verification, trivia attachment configuration, and a minimal CLI.
- Added the default lossless `parse` entry point, projection views, and a
  tested parity roadmap for competitor features, grammar embedding, and language
  coverage targets.
- Added exact reconstruction, mixed-region detection, query matching,
  substitution rules, concept reconstruction, object identity helpers,
  many-valued truth values, and executable parity fixtures for every tracked
  competitor target.
- Added executable language fixtures and tests for every requested Markdown,
  HTML, top-ten programming-language, and top-ten natural-language target.

### Fixed
- Added a CI guard that rejects Rust test modules and test attributes under `src/`, keeping tests in the `tests/` tree.

## Minor Changes

- Add immutable and mutable network snapshots with provenance and forward
  version commits for roadmap snapshot/versioning coverage.

### Added
- Added `txt` as a first-class markup/container target with a UTF-8 lossless
  fixture and content-sniffing fallback regions.

### Fixed
- Reconciled the natural-language target ordering with the Ethnologue 2025
  total-speaker order cited by the parity roadmap.

### Added
- Added a grammar-backed parser front end for Python, C, Java, C++, C#, JavaScript, and R using official tree-sitter grammar crates.

### Added
- Parse supported mixed-mode embedded regions into the host links network with grammar-backed syntax links.

### Added
- Added natural-language segmentation, identification, normalization, and bidi annotation links over lossless text parses.

### Added
- Added grammar-backed Visual Basic parsing with byte-exact reconstruction and recovery flag coverage.

bump: minor

- Add a grammar-backed `sql-ansi` SQL-family dialect fixture using
  `tree-sitter-sequel`.

### Added
- Added grammar-backed Delphi/Object Pascal parsing through `tree-sitter-pascal`.

### Added
- Enriched link queries with S-expression structural matching, captures, by-type construction, host predicate hooks, and link-cli-style variable substitution bindings.

### Added
- Added persistent snapshot structural sharing with interned metadata text storage.

### Added
- Added a query-transform surface for selecting captured links and replacing
  their source text while preserving unchanged bytes.

### Added
- Materialized self-description roots as controlled links with complete root-definition closure and round-trippable `describe` output.

### Added
- Added common concept ontology seeding from meta-expression's semantic lexicon, with shared concept links, syntax mappings, and structural programming-language concepts.

### Added
- Added semantic cross-language reconstruction for the Hawaii statehood fixture,
  including English/Russian naturalization and configurable formalization levels.

### Added
- Expanded `PARITY_FIXTURES` with multiple provenance-tracked fixtures for tree-sitter, LibCST, Recast, jscodeshift, Rowan, cstree, and Roslyn, including executable recovery and query/transform expectations.

### Added

- Expanded `PARITY_FIXTURES` with upstream-provenanced internal ecosystem corpora for links-notation, link-cli, lino-objects-codec, relative-meta-logic, formal-ai, and meta-expression.

### Added

- Added structural LiNo parsing for links-notation doublets, triplets, named links, indented IDs, and self-references while preserving byte-exact reconstruction.

### Added
- Added fixed-point probabilistic truth values for relative-meta-logic-style
  semantic confidence evaluation.

### Added
- Added grammar-backed Go parsing via the official `tree-sitter-go` grammar
  (accepts `Go`, `go`, and `golang`) so `LinkNetwork::parse` emits real
  `LinkType::Syntax` concrete-syntax links with byte-exact reconstruction.

### Added
- Added grammar-backed Ruby parsing through `tree-sitter-ruby`, so
  `LinkNetwork::parse(source, "Ruby", ...)` (and the `rb` alias) now emits real
  `LinkType::Syntax` links instead of falling back to lossless plain text.

bump: minor

- Add a grammar-backed `TypeScript` front end using `tree-sitter-typescript`,
  wiring the `typescript`/`ts` labels to `LANGUAGE_TYPESCRIPT` and the `tsx`
  label to `LANGUAGE_TSX` so `LinkNetwork::parse` emits real `LinkType::Syntax`
  links for TypeScript and TSX sources.

## [0.38.0] - 2026-06-08

### Added
- Changeset-style fragment format with frontmatter for specifying version bump type
- New `get-bump-type.mjs` script to automatically determine version bump from fragments
- Automatic version bumping on merge to main based on changelog fragments
- Detailed documentation for the changelog fragment system in `changelog.d/README.md`

### Changed
- Updated `collect-changelog.mjs` to strip frontmatter when collecting fragments
- Updated `version-and-commit.mjs` to handle frontmatter in fragments
- Enhanced release workflow to automatically determine bump type from changesets

### Changed
- Add `detect-changes` job with cross-platform `detect-code-changes.mjs` script
- Make lint job independent of changelog check (runs based on file changes only)
- Allow docs-only PRs without changelog fragment requirement
- Handle changelog check 'skipped' state in dependent jobs
- Exclude `changelog.d/`, `docs/`, `experiments/`, `examples/` folders and markdown files from code changes detection

### Fixed
- Fixed README.md to correctly reference Node.js scripts (`.mjs`) instead of Python scripts (`.py`)
- Updated project structure in README.md to match actual script files in `scripts/` directory
- Fixed example code in README.md that had invalid Rust with two `main` functions

### Added

- Added crates.io publishing support to CI/CD workflow
- Added `release_mode` input with "instant" and "changelog-pr" options for manual releases
- Added `--tag-prefix` and `--crates-io-url` options to create-github-release.mjs script
- Added comprehensive case study documentation for Issue #11 in docs/case-studies/issue-11/

### Changed

- Changed changelog fragment check from warning to error (exit 1) to enforce changelog requirements
- Updated job conditions with `always() && !cancelled()` to fix workflow_dispatch job skipping issue
- Renamed manual-release job to "Instant Release" for clarity

### Fixed

- Fixed deprecated `::set-output` GitHub Actions command in version-and-commit.mjs
- Fixed workflow_dispatch triggering issues where lint/build/release jobs were incorrectly skipped

### Fixed

- Fixed changelog fragment check to validate that a fragment is **added in the PR diff** rather than just checking if any fragments exist in the directory. This prevents the check from incorrectly passing when there are leftover fragments from previous PRs that haven't been released yet.

### Changed

- Converted shell scripts in `release.yml` to cross-platform `.mjs` scripts for improved portability and performance:
  - `check-changelog-fragment.mjs` - validates changelog fragment is added in PR diff
  - `git-config.mjs` - configures git user for CI/CD
  - `check-release-needed.mjs` - checks if release is needed
  - `publish-crate.mjs` - publishes package to crates.io
  - `create-changelog-fragment.mjs` - creates changelog fragments for manual releases
  - `get-version.mjs` - gets current version from Cargo.toml

### Added

- Added `check-version-modification.mjs` script to detect manual version changes in Cargo.toml
- Added `version-check` job to CI/CD workflow that runs on pull requests
- Added skip logic for automated release branches (changelog-manual-release-*, changeset-release/*, release/*, automated-release/*)

### Changed

- Version modifications in Cargo.toml are now blocked in pull requests to enforce automated release pipeline

### Added

- Added support for `CARGO_REGISTRY_TOKEN` as alternative to `CARGO_TOKEN` for crates.io publishing
- Added case study documentation for Issue #17 (yargs reserved word and dual token support)

### Changed

- Updated workflow to use fallback logic: `${{ secrets.CARGO_REGISTRY_TOKEN || secrets.CARGO_TOKEN }}`
- Improved publish-crate.mjs to check both `CARGO_REGISTRY_TOKEN` and `CARGO_TOKEN` environment variables
- Added warning message when neither token is set

### Added
- New `scripts/rust-paths.mjs` utility for automatic Rust package root detection
- Support for both single-language and multi-language repository structures in all CI/CD scripts
- Configuration options via `--rust-root` CLI argument and `RUST_ROOT` environment variable
- Comprehensive case study documentation in `docs/case-studies/issue-19/`

### Changed
- Updated all release scripts to use the new path detection utility:
  - `scripts/bump-version.mjs`
  - `scripts/check-release-needed.mjs`
  - `scripts/collect-changelog.mjs`
  - `scripts/get-bump-type.mjs`
  - `scripts/get-version.mjs`
  - `scripts/publish-crate.mjs`
  - `scripts/version-and-commit.mjs`

### Changed

- **check-release-needed.mjs**: Now checks crates.io API directly instead of git tags to determine if a version is already released. This prevents false positives where git tags exist but the package was never actually published to crates.io.

### Added

- **CI/CD Troubleshooting Guide**: New documentation at `docs/ci-cd/troubleshooting.md` covering common issues like skipped jobs, false positive version checks, publishing failures, and secret configuration.

- **Enhanced Error Handling in publish-crate.mjs**: Added specific detection and helpful error messages for authentication failures, including guidance on secret configuration and workflow setup.

- **Case Study Documentation**: Added comprehensive case study at `docs/case-studies/issue-21/` analyzing CI/CD failures from browser-commander repository (issues #27, #29, #31, #33) with timeline, root causes, and lessons learned.

### Fixed

- **Prevent False Positive Version Checks**: The release workflow now correctly identifies unpublished versions by checking crates.io instead of relying on git tags, which can exist without the package being published.

### Changed

- Translated all CI/CD scripts from JavaScript (.mjs) to Rust (.rs) using rust-script
- Scripts now use native Rust with rust-script for execution in shell
- Removed Node.js dependency from CI/CD pipeline
- Updated GitHub Actions workflow to use rust-script instead of node
- Updated README and CONTRIBUTING documentation with new script references

### Added

- Cache `restore-keys` for partial cache hits across all workflow jobs
- Explicit `token` parameter in checkout for release jobs
- Code coverage job with `cargo-llvm-cov` and Codecov integration
- Codecov badge in README.md
- Pre-release version support (e.g., `0.1.0-beta.1`) in version parsing
- `--release-label` parameter for multi-language release disambiguation
- `ensure_version_exceeds_published()` logic to prevent publishing duplicate versions
- `get_max_published_version()` to query highest non-yanked version from crates.io
- `max_published_version` output from check-release-needed for downstream use
- Version fallback logic in auto-release Create GitHub Release step

### Changed

- Updated `actions/checkout` from v4 to v6
- Updated `actions/cache` from v4 to v5
- Updated `peter-evans/create-pull-request` from v7 to v8
- Made `publish-crate.rs` fail (exit 1) when version already exists on crates.io
- Improved `create-github-release.rs` to check combined stdout+stderr and detect "Validation Failed"

### Fixed

- Fix publish steps overriding workflow-level CARGO_TOKEN fallback, breaking CARGO_REGISTRY_TOKEN-only configurations (#32)
- Fix non-fast-forward push failures in multi-workflow repos by adding fetch/rebase and push retry logic (#31)
- Add mono-repo path support to check-changelog-fragment.rs, check-version-modification.rs, and create-changelog-fragment.rs
- Add `!cancelled()` guard to test job condition to respect workflow cancellation

### Fixed

- Fixed `version-and-commit.rs` to check crates.io instead of git tags for determining if a version is already released
- This prevents the release pipeline from getting stuck when git tags exist without corresponding crates.io publication

### Added

- Added `--tag-prefix` support to `version-and-commit.rs` for multi-language repository compatibility
- Added crates.io and docs.rs badges to README.md
- Added automatic crates.io and docs.rs badge injection in GitHub release notes
- Added documentation deployment job to CI/CD pipeline (deploys to GitHub Pages after release)
- Added case study documentation for issue #25

### Fixed
- Fixed unsupported look-ahead regex in `create-github-release.rs` that caused a panic when parsing CHANGELOG.md. Replaced with a two-step approach using only features supported by Rust's `regex` crate.

### Changed
- Restructured example application as a simple CLI sum calculator using `lino-arguments`
- Renamed default package to `example-sum-package-name` with Unlicense license
- Reorganized test structure: `tests/unit/sum.rs`, `tests/integration/sum.rs`, `tests/unit/ci-cd/`
- Converted experiment scripts into proper unit tests in `tests/unit/ci-cd/changelog_parsing.rs`
- Added CI/CD skip logic for template default package name `example-sum-package-name`
- Updated README.md badges and documentation

### Fixed

- Change detection script now uses per-commit diff instead of full PR diff, so commits touching only non-code files correctly skip CI jobs even when earlier commits in the same PR changed code files

### Fixed
- Make release scripts resolve the publishable crate manifest when the repository root uses a Cargo workspace manifest.

### Fixed
- Decoupled GitHub Pages documentation deployment from package release publication and fixed release-script warning failures under `RUSTFLAGS=-Dwarnings`.

### Changed
- Added explicit GitHub Actions job timeouts and documented Rust test timeout guidance.

### Fixed
- Added a non-blocking warning threshold to the Rust file-size check so near-limit files are surfaced before concurrent PR merges can exceed the hard limit.

### Fixed
- Made `create-github-release.rs` build GitHub release titles as `[Language] X.Y.Z` instead of reusing the tag prefix.

### Added
- Added optional Docker Hub image publishing tied to Rust crate releases, including crates.io visibility waiting, version/latest image tags, and Docker Hub badges in GitHub release notes.

### Changed
- Release completeness checks now self-heal when crates.io exists but configured Docker Hub or GitHub release artifacts are missing.

### Fixed
- Switched documentation deployment to the official GitHub Pages artifact workflow so repositories using GitHub Actions as their Pages source do not get false-positive branch-push deploys.

### Added
- Documented the one-time `Settings → Pages → Source = GitHub Actions` prerequisite for the `deploy-docs` job in `README.md` and as a comment above the `deploy-docs` job in `release.yml`, so downstream template users hit a documented setup step instead of a `Get Pages site failed` error on the first deploy.

### Added
- Tracking case study at `docs/case-studies/issue-52/` registering the `browser-commander` + Playwright preview-regeneration pattern from [`konard/vk-bot-desktop#52`](https://github.com/konard/vk-bot-desktop/pull/52), with an activation checklist for when an example-app surface lands in this template. Documentation only — no workflow, script, or runtime code changes. Primary upstream tracking issue: [`link-foundation/js-ai-driven-development-pipeline-template#62`](https://github.com/link-foundation/js-ai-driven-development-pipeline-template/issues/62).

### Fixed
- Release automation now keeps the workspace package entry in `Cargo.lock` synchronized when `scripts/version-and-commit.rs` bumps `Cargo.toml`, preventing stale lock-file version diffs in later pull requests.

### Fixed
- `scripts/publish-crate.rs` now treats crates.io HTTP 429 throttle responses ("You have published too many versions of this crate in the last 24 hours") as a deferred `publish_result=rate_limited` outcome (it writes the output, prints an explanatory banner and exits successfully) instead of a hard CI failure reported as a generic `failed` ("Failed to publish for unknown reason"). Authentication, already-published and unknown failures still exit non-zero. Failed-publish classification is consolidated through a single `classify_failure` function and `FailureKind` enum (with an `is_deferred` predicate), covered by unit tests runnable via `rust-script --test scripts/publish-crate.rs`.
- The release workflow (`.github/workflows/release.yml`) now gates crate-availability waiting, Docker Hub publishing and GitHub release creation on either an already-published crate or `publish_result=success`, so a deferred (rate-limited) crate upload no longer produces partial downstream release artifacts and the same version is retried automatically on the next push to `main`.

### Fixed
- Fixed reversed `cancel-in-progress` concurrency condition in `release.yml` that cancelled in-flight releases on `main` and never superseded older PR runs. The condition now uses `!=` so `main` releases run to completion while newer PR pushes cancel stale runs.

### Added
- Added a `scripts/check-crate-size.rs` guard that builds the `.crate` archive and fails the release before publishing when it exceeds the crates.io 10 MiB upload limit. The check runs in the build job and before publishing in both the auto-release and manual-release jobs.

### Changed
- Added a narrow `include` allowlist to `Cargo.toml` so docs, case studies, generated CI artifacts, changelog fragments, scripts, and experiments no longer inflate the published release archive.

### Fixed
- Prevented GitHub release creation from treating generic API validation failures as existing releases, and capped oversized release notes with a link to the full tagged changelog.

### Added
- Added the initial `meta-language` links-network core with self-description roots,
  verification, trivia attachment configuration, and a minimal CLI.
- Added the default lossless `parse` entry point, projection views, and a
  tested parity roadmap for competitor features, grammar embedding, and language
  coverage targets.
- Added exact reconstruction, mixed-region detection, query matching,
  substitution rules, concept reconstruction, object identity helpers,
  many-valued truth values, and executable parity fixtures for every tracked
  competitor target.
- Added executable language fixtures and tests for every requested Markdown,
  HTML, top-ten programming-language, and top-ten natural-language target.

### Fixed
- Added a CI guard that rejects Rust test modules and test attributes under `src/`, keeping tests in the `tests/` tree.

## Minor Changes

- Add immutable and mutable network snapshots with provenance and forward
  version commits for roadmap snapshot/versioning coverage.

### Added
- Added `txt` as a first-class markup/container target with a UTF-8 lossless
  fixture and content-sniffing fallback regions.

### Fixed
- Reconciled the natural-language target ordering with the Ethnologue 2025
  total-speaker order cited by the parity roadmap.

### Added
- Added a grammar-backed parser front end for Python, C, Java, C++, C#, JavaScript, and R using official tree-sitter grammar crates.

### Added
- Parse supported mixed-mode embedded regions into the host links network with grammar-backed syntax links.

### Added
- Added natural-language segmentation, identification, normalization, and bidi annotation links over lossless text parses.

### Added
- Added grammar-backed Visual Basic parsing with byte-exact reconstruction and recovery flag coverage.

bump: minor

- Add a grammar-backed `sql-ansi` SQL-family dialect fixture using
  `tree-sitter-sequel`.

### Added
- Added grammar-backed Delphi/Object Pascal parsing through `tree-sitter-pascal`.

### Added
- Enriched link queries with S-expression structural matching, captures, by-type construction, host predicate hooks, and link-cli-style variable substitution bindings.

### Added
- Added persistent snapshot structural sharing with interned metadata text storage.

### Added
- Added a query-transform surface for selecting captured links and replacing
  their source text while preserving unchanged bytes.

### Added
- Materialized self-description roots as controlled links with complete root-definition closure and round-trippable `describe` output.

### Added
- Added common concept ontology seeding from meta-expression's semantic lexicon, with shared concept links, syntax mappings, and structural programming-language concepts.

### Added
- Added semantic cross-language reconstruction for the Hawaii statehood fixture,
  including English/Russian naturalization and configurable formalization levels.

### Added
- Expanded `PARITY_FIXTURES` with multiple provenance-tracked fixtures for tree-sitter, LibCST, Recast, jscodeshift, Rowan, cstree, and Roslyn, including executable recovery and query/transform expectations.

### Added

- Expanded `PARITY_FIXTURES` with upstream-provenanced internal ecosystem corpora for links-notation, link-cli, lino-objects-codec, relative-meta-logic, formal-ai, and meta-expression.

### Added

- Added structural LiNo parsing for links-notation doublets, triplets, named links, indented IDs, and self-references while preserving byte-exact reconstruction.

### Added
- Added fixed-point probabilistic truth values for relative-meta-logic-style
  semantic confidence evaluation.

## [0.37.0] - 2026-06-08

### Added
- Changeset-style fragment format with frontmatter for specifying version bump type
- New `get-bump-type.mjs` script to automatically determine version bump from fragments
- Automatic version bumping on merge to main based on changelog fragments
- Detailed documentation for the changelog fragment system in `changelog.d/README.md`

### Changed
- Updated `collect-changelog.mjs` to strip frontmatter when collecting fragments
- Updated `version-and-commit.mjs` to handle frontmatter in fragments
- Enhanced release workflow to automatically determine bump type from changesets

### Changed
- Add `detect-changes` job with cross-platform `detect-code-changes.mjs` script
- Make lint job independent of changelog check (runs based on file changes only)
- Allow docs-only PRs without changelog fragment requirement
- Handle changelog check 'skipped' state in dependent jobs
- Exclude `changelog.d/`, `docs/`, `experiments/`, `examples/` folders and markdown files from code changes detection

### Fixed
- Fixed README.md to correctly reference Node.js scripts (`.mjs`) instead of Python scripts (`.py`)
- Updated project structure in README.md to match actual script files in `scripts/` directory
- Fixed example code in README.md that had invalid Rust with two `main` functions

### Added

- Added crates.io publishing support to CI/CD workflow
- Added `release_mode` input with "instant" and "changelog-pr" options for manual releases
- Added `--tag-prefix` and `--crates-io-url` options to create-github-release.mjs script
- Added comprehensive case study documentation for Issue #11 in docs/case-studies/issue-11/

### Changed

- Changed changelog fragment check from warning to error (exit 1) to enforce changelog requirements
- Updated job conditions with `always() && !cancelled()` to fix workflow_dispatch job skipping issue
- Renamed manual-release job to "Instant Release" for clarity

### Fixed

- Fixed deprecated `::set-output` GitHub Actions command in version-and-commit.mjs
- Fixed workflow_dispatch triggering issues where lint/build/release jobs were incorrectly skipped

### Fixed

- Fixed changelog fragment check to validate that a fragment is **added in the PR diff** rather than just checking if any fragments exist in the directory. This prevents the check from incorrectly passing when there are leftover fragments from previous PRs that haven't been released yet.

### Changed

- Converted shell scripts in `release.yml` to cross-platform `.mjs` scripts for improved portability and performance:
  - `check-changelog-fragment.mjs` - validates changelog fragment is added in PR diff
  - `git-config.mjs` - configures git user for CI/CD
  - `check-release-needed.mjs` - checks if release is needed
  - `publish-crate.mjs` - publishes package to crates.io
  - `create-changelog-fragment.mjs` - creates changelog fragments for manual releases
  - `get-version.mjs` - gets current version from Cargo.toml

### Added

- Added `check-version-modification.mjs` script to detect manual version changes in Cargo.toml
- Added `version-check` job to CI/CD workflow that runs on pull requests
- Added skip logic for automated release branches (changelog-manual-release-*, changeset-release/*, release/*, automated-release/*)

### Changed

- Version modifications in Cargo.toml are now blocked in pull requests to enforce automated release pipeline

### Added

- Added support for `CARGO_REGISTRY_TOKEN` as alternative to `CARGO_TOKEN` for crates.io publishing
- Added case study documentation for Issue #17 (yargs reserved word and dual token support)

### Changed

- Updated workflow to use fallback logic: `${{ secrets.CARGO_REGISTRY_TOKEN || secrets.CARGO_TOKEN }}`
- Improved publish-crate.mjs to check both `CARGO_REGISTRY_TOKEN` and `CARGO_TOKEN` environment variables
- Added warning message when neither token is set

### Added
- New `scripts/rust-paths.mjs` utility for automatic Rust package root detection
- Support for both single-language and multi-language repository structures in all CI/CD scripts
- Configuration options via `--rust-root` CLI argument and `RUST_ROOT` environment variable
- Comprehensive case study documentation in `docs/case-studies/issue-19/`

### Changed
- Updated all release scripts to use the new path detection utility:
  - `scripts/bump-version.mjs`
  - `scripts/check-release-needed.mjs`
  - `scripts/collect-changelog.mjs`
  - `scripts/get-bump-type.mjs`
  - `scripts/get-version.mjs`
  - `scripts/publish-crate.mjs`
  - `scripts/version-and-commit.mjs`

### Changed

- **check-release-needed.mjs**: Now checks crates.io API directly instead of git tags to determine if a version is already released. This prevents false positives where git tags exist but the package was never actually published to crates.io.

### Added

- **CI/CD Troubleshooting Guide**: New documentation at `docs/ci-cd/troubleshooting.md` covering common issues like skipped jobs, false positive version checks, publishing failures, and secret configuration.

- **Enhanced Error Handling in publish-crate.mjs**: Added specific detection and helpful error messages for authentication failures, including guidance on secret configuration and workflow setup.

- **Case Study Documentation**: Added comprehensive case study at `docs/case-studies/issue-21/` analyzing CI/CD failures from browser-commander repository (issues #27, #29, #31, #33) with timeline, root causes, and lessons learned.

### Fixed

- **Prevent False Positive Version Checks**: The release workflow now correctly identifies unpublished versions by checking crates.io instead of relying on git tags, which can exist without the package being published.

### Changed

- Translated all CI/CD scripts from JavaScript (.mjs) to Rust (.rs) using rust-script
- Scripts now use native Rust with rust-script for execution in shell
- Removed Node.js dependency from CI/CD pipeline
- Updated GitHub Actions workflow to use rust-script instead of node
- Updated README and CONTRIBUTING documentation with new script references

### Added

- Cache `restore-keys` for partial cache hits across all workflow jobs
- Explicit `token` parameter in checkout for release jobs
- Code coverage job with `cargo-llvm-cov` and Codecov integration
- Codecov badge in README.md
- Pre-release version support (e.g., `0.1.0-beta.1`) in version parsing
- `--release-label` parameter for multi-language release disambiguation
- `ensure_version_exceeds_published()` logic to prevent publishing duplicate versions
- `get_max_published_version()` to query highest non-yanked version from crates.io
- `max_published_version` output from check-release-needed for downstream use
- Version fallback logic in auto-release Create GitHub Release step

### Changed

- Updated `actions/checkout` from v4 to v6
- Updated `actions/cache` from v4 to v5
- Updated `peter-evans/create-pull-request` from v7 to v8
- Made `publish-crate.rs` fail (exit 1) when version already exists on crates.io
- Improved `create-github-release.rs` to check combined stdout+stderr and detect "Validation Failed"

### Fixed

- Fix publish steps overriding workflow-level CARGO_TOKEN fallback, breaking CARGO_REGISTRY_TOKEN-only configurations (#32)
- Fix non-fast-forward push failures in multi-workflow repos by adding fetch/rebase and push retry logic (#31)
- Add mono-repo path support to check-changelog-fragment.rs, check-version-modification.rs, and create-changelog-fragment.rs
- Add `!cancelled()` guard to test job condition to respect workflow cancellation

### Fixed

- Fixed `version-and-commit.rs` to check crates.io instead of git tags for determining if a version is already released
- This prevents the release pipeline from getting stuck when git tags exist without corresponding crates.io publication

### Added

- Added `--tag-prefix` support to `version-and-commit.rs` for multi-language repository compatibility
- Added crates.io and docs.rs badges to README.md
- Added automatic crates.io and docs.rs badge injection in GitHub release notes
- Added documentation deployment job to CI/CD pipeline (deploys to GitHub Pages after release)
- Added case study documentation for issue #25

### Fixed
- Fixed unsupported look-ahead regex in `create-github-release.rs` that caused a panic when parsing CHANGELOG.md. Replaced with a two-step approach using only features supported by Rust's `regex` crate.

### Changed
- Restructured example application as a simple CLI sum calculator using `lino-arguments`
- Renamed default package to `example-sum-package-name` with Unlicense license
- Reorganized test structure: `tests/unit/sum.rs`, `tests/integration/sum.rs`, `tests/unit/ci-cd/`
- Converted experiment scripts into proper unit tests in `tests/unit/ci-cd/changelog_parsing.rs`
- Added CI/CD skip logic for template default package name `example-sum-package-name`
- Updated README.md badges and documentation

### Fixed

- Change detection script now uses per-commit diff instead of full PR diff, so commits touching only non-code files correctly skip CI jobs even when earlier commits in the same PR changed code files

### Fixed
- Make release scripts resolve the publishable crate manifest when the repository root uses a Cargo workspace manifest.

### Fixed
- Decoupled GitHub Pages documentation deployment from package release publication and fixed release-script warning failures under `RUSTFLAGS=-Dwarnings`.

### Changed
- Added explicit GitHub Actions job timeouts and documented Rust test timeout guidance.

### Fixed
- Added a non-blocking warning threshold to the Rust file-size check so near-limit files are surfaced before concurrent PR merges can exceed the hard limit.

### Fixed
- Made `create-github-release.rs` build GitHub release titles as `[Language] X.Y.Z` instead of reusing the tag prefix.

### Added
- Added optional Docker Hub image publishing tied to Rust crate releases, including crates.io visibility waiting, version/latest image tags, and Docker Hub badges in GitHub release notes.

### Changed
- Release completeness checks now self-heal when crates.io exists but configured Docker Hub or GitHub release artifacts are missing.

### Fixed
- Switched documentation deployment to the official GitHub Pages artifact workflow so repositories using GitHub Actions as their Pages source do not get false-positive branch-push deploys.

### Added
- Documented the one-time `Settings → Pages → Source = GitHub Actions` prerequisite for the `deploy-docs` job in `README.md` and as a comment above the `deploy-docs` job in `release.yml`, so downstream template users hit a documented setup step instead of a `Get Pages site failed` error on the first deploy.

### Added
- Tracking case study at `docs/case-studies/issue-52/` registering the `browser-commander` + Playwright preview-regeneration pattern from [`konard/vk-bot-desktop#52`](https://github.com/konard/vk-bot-desktop/pull/52), with an activation checklist for when an example-app surface lands in this template. Documentation only — no workflow, script, or runtime code changes. Primary upstream tracking issue: [`link-foundation/js-ai-driven-development-pipeline-template#62`](https://github.com/link-foundation/js-ai-driven-development-pipeline-template/issues/62).

### Fixed
- Release automation now keeps the workspace package entry in `Cargo.lock` synchronized when `scripts/version-and-commit.rs` bumps `Cargo.toml`, preventing stale lock-file version diffs in later pull requests.

### Fixed
- `scripts/publish-crate.rs` now treats crates.io HTTP 429 throttle responses ("You have published too many versions of this crate in the last 24 hours") as a deferred `publish_result=rate_limited` outcome (it writes the output, prints an explanatory banner and exits successfully) instead of a hard CI failure reported as a generic `failed` ("Failed to publish for unknown reason"). Authentication, already-published and unknown failures still exit non-zero. Failed-publish classification is consolidated through a single `classify_failure` function and `FailureKind` enum (with an `is_deferred` predicate), covered by unit tests runnable via `rust-script --test scripts/publish-crate.rs`.
- The release workflow (`.github/workflows/release.yml`) now gates crate-availability waiting, Docker Hub publishing and GitHub release creation on either an already-published crate or `publish_result=success`, so a deferred (rate-limited) crate upload no longer produces partial downstream release artifacts and the same version is retried automatically on the next push to `main`.

### Fixed
- Fixed reversed `cancel-in-progress` concurrency condition in `release.yml` that cancelled in-flight releases on `main` and never superseded older PR runs. The condition now uses `!=` so `main` releases run to completion while newer PR pushes cancel stale runs.

### Added
- Added a `scripts/check-crate-size.rs` guard that builds the `.crate` archive and fails the release before publishing when it exceeds the crates.io 10 MiB upload limit. The check runs in the build job and before publishing in both the auto-release and manual-release jobs.

### Changed
- Added a narrow `include` allowlist to `Cargo.toml` so docs, case studies, generated CI artifacts, changelog fragments, scripts, and experiments no longer inflate the published release archive.

### Fixed
- Prevented GitHub release creation from treating generic API validation failures as existing releases, and capped oversized release notes with a link to the full tagged changelog.

### Added
- Added the initial `meta-language` links-network core with self-description roots,
  verification, trivia attachment configuration, and a minimal CLI.
- Added the default lossless `parse` entry point, projection views, and a
  tested parity roadmap for competitor features, grammar embedding, and language
  coverage targets.
- Added exact reconstruction, mixed-region detection, query matching,
  substitution rules, concept reconstruction, object identity helpers,
  many-valued truth values, and executable parity fixtures for every tracked
  competitor target.
- Added executable language fixtures and tests for every requested Markdown,
  HTML, top-ten programming-language, and top-ten natural-language target.

### Fixed
- Added a CI guard that rejects Rust test modules and test attributes under `src/`, keeping tests in the `tests/` tree.

## Minor Changes

- Add immutable and mutable network snapshots with provenance and forward
  version commits for roadmap snapshot/versioning coverage.

### Added
- Added `txt` as a first-class markup/container target with a UTF-8 lossless
  fixture and content-sniffing fallback regions.

### Fixed
- Reconciled the natural-language target ordering with the Ethnologue 2025
  total-speaker order cited by the parity roadmap.

### Added
- Added a grammar-backed parser front end for Python, C, Java, C++, C#, JavaScript, and R using official tree-sitter grammar crates.

### Added
- Parse supported mixed-mode embedded regions into the host links network with grammar-backed syntax links.

### Added
- Added natural-language segmentation, identification, normalization, and bidi annotation links over lossless text parses.

### Added
- Added grammar-backed Visual Basic parsing with byte-exact reconstruction and recovery flag coverage.

bump: minor

- Add a grammar-backed `sql-ansi` SQL-family dialect fixture using
  `tree-sitter-sequel`.

### Added
- Added grammar-backed Delphi/Object Pascal parsing through `tree-sitter-pascal`.

### Added
- Enriched link queries with S-expression structural matching, captures, by-type construction, host predicate hooks, and link-cli-style variable substitution bindings.

### Added
- Added persistent snapshot structural sharing with interned metadata text storage.

### Added
- Added a query-transform surface for selecting captured links and replacing
  their source text while preserving unchanged bytes.

### Added
- Materialized self-description roots as controlled links with complete root-definition closure and round-trippable `describe` output.

### Added
- Added common concept ontology seeding from meta-expression's semantic lexicon, with shared concept links, syntax mappings, and structural programming-language concepts.

### Added
- Added semantic cross-language reconstruction for the Hawaii statehood fixture,
  including English/Russian naturalization and configurable formalization levels.

### Added
- Expanded `PARITY_FIXTURES` with multiple provenance-tracked fixtures for tree-sitter, LibCST, Recast, jscodeshift, Rowan, cstree, and Roslyn, including executable recovery and query/transform expectations.

### Added

- Expanded `PARITY_FIXTURES` with upstream-provenanced internal ecosystem corpora for links-notation, link-cli, lino-objects-codec, relative-meta-logic, formal-ai, and meta-expression.

### Added

- Added structural LiNo parsing for links-notation doublets, triplets, named links, indented IDs, and self-references while preserving byte-exact reconstruction.

## [0.36.0] - 2026-06-07

### Added
- Changeset-style fragment format with frontmatter for specifying version bump type
- New `get-bump-type.mjs` script to automatically determine version bump from fragments
- Automatic version bumping on merge to main based on changelog fragments
- Detailed documentation for the changelog fragment system in `changelog.d/README.md`

### Changed
- Updated `collect-changelog.mjs` to strip frontmatter when collecting fragments
- Updated `version-and-commit.mjs` to handle frontmatter in fragments
- Enhanced release workflow to automatically determine bump type from changesets

### Changed
- Add `detect-changes` job with cross-platform `detect-code-changes.mjs` script
- Make lint job independent of changelog check (runs based on file changes only)
- Allow docs-only PRs without changelog fragment requirement
- Handle changelog check 'skipped' state in dependent jobs
- Exclude `changelog.d/`, `docs/`, `experiments/`, `examples/` folders and markdown files from code changes detection

### Fixed
- Fixed README.md to correctly reference Node.js scripts (`.mjs`) instead of Python scripts (`.py`)
- Updated project structure in README.md to match actual script files in `scripts/` directory
- Fixed example code in README.md that had invalid Rust with two `main` functions

### Added

- Added crates.io publishing support to CI/CD workflow
- Added `release_mode` input with "instant" and "changelog-pr" options for manual releases
- Added `--tag-prefix` and `--crates-io-url` options to create-github-release.mjs script
- Added comprehensive case study documentation for Issue #11 in docs/case-studies/issue-11/

### Changed

- Changed changelog fragment check from warning to error (exit 1) to enforce changelog requirements
- Updated job conditions with `always() && !cancelled()` to fix workflow_dispatch job skipping issue
- Renamed manual-release job to "Instant Release" for clarity

### Fixed

- Fixed deprecated `::set-output` GitHub Actions command in version-and-commit.mjs
- Fixed workflow_dispatch triggering issues where lint/build/release jobs were incorrectly skipped

### Fixed

- Fixed changelog fragment check to validate that a fragment is **added in the PR diff** rather than just checking if any fragments exist in the directory. This prevents the check from incorrectly passing when there are leftover fragments from previous PRs that haven't been released yet.

### Changed

- Converted shell scripts in `release.yml` to cross-platform `.mjs` scripts for improved portability and performance:
  - `check-changelog-fragment.mjs` - validates changelog fragment is added in PR diff
  - `git-config.mjs` - configures git user for CI/CD
  - `check-release-needed.mjs` - checks if release is needed
  - `publish-crate.mjs` - publishes package to crates.io
  - `create-changelog-fragment.mjs` - creates changelog fragments for manual releases
  - `get-version.mjs` - gets current version from Cargo.toml

### Added

- Added `check-version-modification.mjs` script to detect manual version changes in Cargo.toml
- Added `version-check` job to CI/CD workflow that runs on pull requests
- Added skip logic for automated release branches (changelog-manual-release-*, changeset-release/*, release/*, automated-release/*)

### Changed

- Version modifications in Cargo.toml are now blocked in pull requests to enforce automated release pipeline

### Added

- Added support for `CARGO_REGISTRY_TOKEN` as alternative to `CARGO_TOKEN` for crates.io publishing
- Added case study documentation for Issue #17 (yargs reserved word and dual token support)

### Changed

- Updated workflow to use fallback logic: `${{ secrets.CARGO_REGISTRY_TOKEN || secrets.CARGO_TOKEN }}`
- Improved publish-crate.mjs to check both `CARGO_REGISTRY_TOKEN` and `CARGO_TOKEN` environment variables
- Added warning message when neither token is set

### Added
- New `scripts/rust-paths.mjs` utility for automatic Rust package root detection
- Support for both single-language and multi-language repository structures in all CI/CD scripts
- Configuration options via `--rust-root` CLI argument and `RUST_ROOT` environment variable
- Comprehensive case study documentation in `docs/case-studies/issue-19/`

### Changed
- Updated all release scripts to use the new path detection utility:
  - `scripts/bump-version.mjs`
  - `scripts/check-release-needed.mjs`
  - `scripts/collect-changelog.mjs`
  - `scripts/get-bump-type.mjs`
  - `scripts/get-version.mjs`
  - `scripts/publish-crate.mjs`
  - `scripts/version-and-commit.mjs`

### Changed

- **check-release-needed.mjs**: Now checks crates.io API directly instead of git tags to determine if a version is already released. This prevents false positives where git tags exist but the package was never actually published to crates.io.

### Added

- **CI/CD Troubleshooting Guide**: New documentation at `docs/ci-cd/troubleshooting.md` covering common issues like skipped jobs, false positive version checks, publishing failures, and secret configuration.

- **Enhanced Error Handling in publish-crate.mjs**: Added specific detection and helpful error messages for authentication failures, including guidance on secret configuration and workflow setup.

- **Case Study Documentation**: Added comprehensive case study at `docs/case-studies/issue-21/` analyzing CI/CD failures from browser-commander repository (issues #27, #29, #31, #33) with timeline, root causes, and lessons learned.

### Fixed

- **Prevent False Positive Version Checks**: The release workflow now correctly identifies unpublished versions by checking crates.io instead of relying on git tags, which can exist without the package being published.

### Changed

- Translated all CI/CD scripts from JavaScript (.mjs) to Rust (.rs) using rust-script
- Scripts now use native Rust with rust-script for execution in shell
- Removed Node.js dependency from CI/CD pipeline
- Updated GitHub Actions workflow to use rust-script instead of node
- Updated README and CONTRIBUTING documentation with new script references

### Added

- Cache `restore-keys` for partial cache hits across all workflow jobs
- Explicit `token` parameter in checkout for release jobs
- Code coverage job with `cargo-llvm-cov` and Codecov integration
- Codecov badge in README.md
- Pre-release version support (e.g., `0.1.0-beta.1`) in version parsing
- `--release-label` parameter for multi-language release disambiguation
- `ensure_version_exceeds_published()` logic to prevent publishing duplicate versions
- `get_max_published_version()` to query highest non-yanked version from crates.io
- `max_published_version` output from check-release-needed for downstream use
- Version fallback logic in auto-release Create GitHub Release step

### Changed

- Updated `actions/checkout` from v4 to v6
- Updated `actions/cache` from v4 to v5
- Updated `peter-evans/create-pull-request` from v7 to v8
- Made `publish-crate.rs` fail (exit 1) when version already exists on crates.io
- Improved `create-github-release.rs` to check combined stdout+stderr and detect "Validation Failed"

### Fixed

- Fix publish steps overriding workflow-level CARGO_TOKEN fallback, breaking CARGO_REGISTRY_TOKEN-only configurations (#32)
- Fix non-fast-forward push failures in multi-workflow repos by adding fetch/rebase and push retry logic (#31)
- Add mono-repo path support to check-changelog-fragment.rs, check-version-modification.rs, and create-changelog-fragment.rs
- Add `!cancelled()` guard to test job condition to respect workflow cancellation

### Fixed

- Fixed `version-and-commit.rs` to check crates.io instead of git tags for determining if a version is already released
- This prevents the release pipeline from getting stuck when git tags exist without corresponding crates.io publication

### Added

- Added `--tag-prefix` support to `version-and-commit.rs` for multi-language repository compatibility
- Added crates.io and docs.rs badges to README.md
- Added automatic crates.io and docs.rs badge injection in GitHub release notes
- Added documentation deployment job to CI/CD pipeline (deploys to GitHub Pages after release)
- Added case study documentation for issue #25

### Fixed
- Fixed unsupported look-ahead regex in `create-github-release.rs` that caused a panic when parsing CHANGELOG.md. Replaced with a two-step approach using only features supported by Rust's `regex` crate.

### Changed
- Restructured example application as a simple CLI sum calculator using `lino-arguments`
- Renamed default package to `example-sum-package-name` with Unlicense license
- Reorganized test structure: `tests/unit/sum.rs`, `tests/integration/sum.rs`, `tests/unit/ci-cd/`
- Converted experiment scripts into proper unit tests in `tests/unit/ci-cd/changelog_parsing.rs`
- Added CI/CD skip logic for template default package name `example-sum-package-name`
- Updated README.md badges and documentation

### Fixed

- Change detection script now uses per-commit diff instead of full PR diff, so commits touching only non-code files correctly skip CI jobs even when earlier commits in the same PR changed code files

### Fixed
- Make release scripts resolve the publishable crate manifest when the repository root uses a Cargo workspace manifest.

### Fixed
- Decoupled GitHub Pages documentation deployment from package release publication and fixed release-script warning failures under `RUSTFLAGS=-Dwarnings`.

### Changed
- Added explicit GitHub Actions job timeouts and documented Rust test timeout guidance.

### Fixed
- Added a non-blocking warning threshold to the Rust file-size check so near-limit files are surfaced before concurrent PR merges can exceed the hard limit.

### Fixed
- Made `create-github-release.rs` build GitHub release titles as `[Language] X.Y.Z` instead of reusing the tag prefix.

### Added
- Added optional Docker Hub image publishing tied to Rust crate releases, including crates.io visibility waiting, version/latest image tags, and Docker Hub badges in GitHub release notes.

### Changed
- Release completeness checks now self-heal when crates.io exists but configured Docker Hub or GitHub release artifacts are missing.

### Fixed
- Switched documentation deployment to the official GitHub Pages artifact workflow so repositories using GitHub Actions as their Pages source do not get false-positive branch-push deploys.

### Added
- Documented the one-time `Settings → Pages → Source = GitHub Actions` prerequisite for the `deploy-docs` job in `README.md` and as a comment above the `deploy-docs` job in `release.yml`, so downstream template users hit a documented setup step instead of a `Get Pages site failed` error on the first deploy.

### Added
- Tracking case study at `docs/case-studies/issue-52/` registering the `browser-commander` + Playwright preview-regeneration pattern from [`konard/vk-bot-desktop#52`](https://github.com/konard/vk-bot-desktop/pull/52), with an activation checklist for when an example-app surface lands in this template. Documentation only — no workflow, script, or runtime code changes. Primary upstream tracking issue: [`link-foundation/js-ai-driven-development-pipeline-template#62`](https://github.com/link-foundation/js-ai-driven-development-pipeline-template/issues/62).

### Fixed
- Release automation now keeps the workspace package entry in `Cargo.lock` synchronized when `scripts/version-and-commit.rs` bumps `Cargo.toml`, preventing stale lock-file version diffs in later pull requests.

### Fixed
- `scripts/publish-crate.rs` now treats crates.io HTTP 429 throttle responses ("You have published too many versions of this crate in the last 24 hours") as a deferred `publish_result=rate_limited` outcome (it writes the output, prints an explanatory banner and exits successfully) instead of a hard CI failure reported as a generic `failed` ("Failed to publish for unknown reason"). Authentication, already-published and unknown failures still exit non-zero. Failed-publish classification is consolidated through a single `classify_failure` function and `FailureKind` enum (with an `is_deferred` predicate), covered by unit tests runnable via `rust-script --test scripts/publish-crate.rs`.
- The release workflow (`.github/workflows/release.yml`) now gates crate-availability waiting, Docker Hub publishing and GitHub release creation on either an already-published crate or `publish_result=success`, so a deferred (rate-limited) crate upload no longer produces partial downstream release artifacts and the same version is retried automatically on the next push to `main`.

### Fixed
- Fixed reversed `cancel-in-progress` concurrency condition in `release.yml` that cancelled in-flight releases on `main` and never superseded older PR runs. The condition now uses `!=` so `main` releases run to completion while newer PR pushes cancel stale runs.

### Added
- Added a `scripts/check-crate-size.rs` guard that builds the `.crate` archive and fails the release before publishing when it exceeds the crates.io 10 MiB upload limit. The check runs in the build job and before publishing in both the auto-release and manual-release jobs.

### Changed
- Added a narrow `include` allowlist to `Cargo.toml` so docs, case studies, generated CI artifacts, changelog fragments, scripts, and experiments no longer inflate the published release archive.

### Fixed
- Prevented GitHub release creation from treating generic API validation failures as existing releases, and capped oversized release notes with a link to the full tagged changelog.

### Added
- Added the initial `meta-language` links-network core with self-description roots,
  verification, trivia attachment configuration, and a minimal CLI.
- Added the default lossless `parse` entry point, projection views, and a
  tested parity roadmap for competitor features, grammar embedding, and language
  coverage targets.
- Added exact reconstruction, mixed-region detection, query matching,
  substitution rules, concept reconstruction, object identity helpers,
  many-valued truth values, and executable parity fixtures for every tracked
  competitor target.
- Added executable language fixtures and tests for every requested Markdown,
  HTML, top-ten programming-language, and top-ten natural-language target.

### Fixed
- Added a CI guard that rejects Rust test modules and test attributes under `src/`, keeping tests in the `tests/` tree.

## Minor Changes

- Add immutable and mutable network snapshots with provenance and forward
  version commits for roadmap snapshot/versioning coverage.

### Added
- Added `txt` as a first-class markup/container target with a UTF-8 lossless
  fixture and content-sniffing fallback regions.

### Fixed
- Reconciled the natural-language target ordering with the Ethnologue 2025
  total-speaker order cited by the parity roadmap.

### Added
- Added a grammar-backed parser front end for Python, C, Java, C++, C#, JavaScript, and R using official tree-sitter grammar crates.

### Added
- Parse supported mixed-mode embedded regions into the host links network with grammar-backed syntax links.

### Added
- Added natural-language segmentation, identification, normalization, and bidi annotation links over lossless text parses.

### Added
- Added grammar-backed Visual Basic parsing with byte-exact reconstruction and recovery flag coverage.

bump: minor

- Add a grammar-backed `sql-ansi` SQL-family dialect fixture using
  `tree-sitter-sequel`.

### Added
- Added grammar-backed Delphi/Object Pascal parsing through `tree-sitter-pascal`.

### Added
- Enriched link queries with S-expression structural matching, captures, by-type construction, host predicate hooks, and link-cli-style variable substitution bindings.

### Added
- Added persistent snapshot structural sharing with interned metadata text storage.

### Added
- Added a query-transform surface for selecting captured links and replacing
  their source text while preserving unchanged bytes.

### Added
- Materialized self-description roots as controlled links with complete root-definition closure and round-trippable `describe` output.

### Added
- Added common concept ontology seeding from meta-expression's semantic lexicon, with shared concept links, syntax mappings, and structural programming-language concepts.

### Added
- Added semantic cross-language reconstruction for the Hawaii statehood fixture,
  including English/Russian naturalization and configurable formalization levels.

### Added
- Expanded `PARITY_FIXTURES` with multiple provenance-tracked fixtures for tree-sitter, LibCST, Recast, jscodeshift, Rowan, cstree, and Roslyn, including executable recovery and query/transform expectations.

### Added

- Expanded `PARITY_FIXTURES` with upstream-provenanced internal ecosystem corpora for links-notation, link-cli, lino-objects-codec, relative-meta-logic, formal-ai, and meta-expression.

## [0.35.0] - 2026-06-07

### Added
- Changeset-style fragment format with frontmatter for specifying version bump type
- New `get-bump-type.mjs` script to automatically determine version bump from fragments
- Automatic version bumping on merge to main based on changelog fragments
- Detailed documentation for the changelog fragment system in `changelog.d/README.md`

### Changed
- Updated `collect-changelog.mjs` to strip frontmatter when collecting fragments
- Updated `version-and-commit.mjs` to handle frontmatter in fragments
- Enhanced release workflow to automatically determine bump type from changesets

### Changed
- Add `detect-changes` job with cross-platform `detect-code-changes.mjs` script
- Make lint job independent of changelog check (runs based on file changes only)
- Allow docs-only PRs without changelog fragment requirement
- Handle changelog check 'skipped' state in dependent jobs
- Exclude `changelog.d/`, `docs/`, `experiments/`, `examples/` folders and markdown files from code changes detection

### Fixed
- Fixed README.md to correctly reference Node.js scripts (`.mjs`) instead of Python scripts (`.py`)
- Updated project structure in README.md to match actual script files in `scripts/` directory
- Fixed example code in README.md that had invalid Rust with two `main` functions

### Added

- Added crates.io publishing support to CI/CD workflow
- Added `release_mode` input with "instant" and "changelog-pr" options for manual releases
- Added `--tag-prefix` and `--crates-io-url` options to create-github-release.mjs script
- Added comprehensive case study documentation for Issue #11 in docs/case-studies/issue-11/

### Changed

- Changed changelog fragment check from warning to error (exit 1) to enforce changelog requirements
- Updated job conditions with `always() && !cancelled()` to fix workflow_dispatch job skipping issue
- Renamed manual-release job to "Instant Release" for clarity

### Fixed

- Fixed deprecated `::set-output` GitHub Actions command in version-and-commit.mjs
- Fixed workflow_dispatch triggering issues where lint/build/release jobs were incorrectly skipped

### Fixed

- Fixed changelog fragment check to validate that a fragment is **added in the PR diff** rather than just checking if any fragments exist in the directory. This prevents the check from incorrectly passing when there are leftover fragments from previous PRs that haven't been released yet.

### Changed

- Converted shell scripts in `release.yml` to cross-platform `.mjs` scripts for improved portability and performance:
  - `check-changelog-fragment.mjs` - validates changelog fragment is added in PR diff
  - `git-config.mjs` - configures git user for CI/CD
  - `check-release-needed.mjs` - checks if release is needed
  - `publish-crate.mjs` - publishes package to crates.io
  - `create-changelog-fragment.mjs` - creates changelog fragments for manual releases
  - `get-version.mjs` - gets current version from Cargo.toml

### Added

- Added `check-version-modification.mjs` script to detect manual version changes in Cargo.toml
- Added `version-check` job to CI/CD workflow that runs on pull requests
- Added skip logic for automated release branches (changelog-manual-release-*, changeset-release/*, release/*, automated-release/*)

### Changed

- Version modifications in Cargo.toml are now blocked in pull requests to enforce automated release pipeline

### Added

- Added support for `CARGO_REGISTRY_TOKEN` as alternative to `CARGO_TOKEN` for crates.io publishing
- Added case study documentation for Issue #17 (yargs reserved word and dual token support)

### Changed

- Updated workflow to use fallback logic: `${{ secrets.CARGO_REGISTRY_TOKEN || secrets.CARGO_TOKEN }}`
- Improved publish-crate.mjs to check both `CARGO_REGISTRY_TOKEN` and `CARGO_TOKEN` environment variables
- Added warning message when neither token is set

### Added
- New `scripts/rust-paths.mjs` utility for automatic Rust package root detection
- Support for both single-language and multi-language repository structures in all CI/CD scripts
- Configuration options via `--rust-root` CLI argument and `RUST_ROOT` environment variable
- Comprehensive case study documentation in `docs/case-studies/issue-19/`

### Changed
- Updated all release scripts to use the new path detection utility:
  - `scripts/bump-version.mjs`
  - `scripts/check-release-needed.mjs`
  - `scripts/collect-changelog.mjs`
  - `scripts/get-bump-type.mjs`
  - `scripts/get-version.mjs`
  - `scripts/publish-crate.mjs`
  - `scripts/version-and-commit.mjs`

### Changed

- **check-release-needed.mjs**: Now checks crates.io API directly instead of git tags to determine if a version is already released. This prevents false positives where git tags exist but the package was never actually published to crates.io.

### Added

- **CI/CD Troubleshooting Guide**: New documentation at `docs/ci-cd/troubleshooting.md` covering common issues like skipped jobs, false positive version checks, publishing failures, and secret configuration.

- **Enhanced Error Handling in publish-crate.mjs**: Added specific detection and helpful error messages for authentication failures, including guidance on secret configuration and workflow setup.

- **Case Study Documentation**: Added comprehensive case study at `docs/case-studies/issue-21/` analyzing CI/CD failures from browser-commander repository (issues #27, #29, #31, #33) with timeline, root causes, and lessons learned.

### Fixed

- **Prevent False Positive Version Checks**: The release workflow now correctly identifies unpublished versions by checking crates.io instead of relying on git tags, which can exist without the package being published.

### Changed

- Translated all CI/CD scripts from JavaScript (.mjs) to Rust (.rs) using rust-script
- Scripts now use native Rust with rust-script for execution in shell
- Removed Node.js dependency from CI/CD pipeline
- Updated GitHub Actions workflow to use rust-script instead of node
- Updated README and CONTRIBUTING documentation with new script references

### Added

- Cache `restore-keys` for partial cache hits across all workflow jobs
- Explicit `token` parameter in checkout for release jobs
- Code coverage job with `cargo-llvm-cov` and Codecov integration
- Codecov badge in README.md
- Pre-release version support (e.g., `0.1.0-beta.1`) in version parsing
- `--release-label` parameter for multi-language release disambiguation
- `ensure_version_exceeds_published()` logic to prevent publishing duplicate versions
- `get_max_published_version()` to query highest non-yanked version from crates.io
- `max_published_version` output from check-release-needed for downstream use
- Version fallback logic in auto-release Create GitHub Release step

### Changed

- Updated `actions/checkout` from v4 to v6
- Updated `actions/cache` from v4 to v5
- Updated `peter-evans/create-pull-request` from v7 to v8
- Made `publish-crate.rs` fail (exit 1) when version already exists on crates.io
- Improved `create-github-release.rs` to check combined stdout+stderr and detect "Validation Failed"

### Fixed

- Fix publish steps overriding workflow-level CARGO_TOKEN fallback, breaking CARGO_REGISTRY_TOKEN-only configurations (#32)
- Fix non-fast-forward push failures in multi-workflow repos by adding fetch/rebase and push retry logic (#31)
- Add mono-repo path support to check-changelog-fragment.rs, check-version-modification.rs, and create-changelog-fragment.rs
- Add `!cancelled()` guard to test job condition to respect workflow cancellation

### Fixed

- Fixed `version-and-commit.rs` to check crates.io instead of git tags for determining if a version is already released
- This prevents the release pipeline from getting stuck when git tags exist without corresponding crates.io publication

### Added

- Added `--tag-prefix` support to `version-and-commit.rs` for multi-language repository compatibility
- Added crates.io and docs.rs badges to README.md
- Added automatic crates.io and docs.rs badge injection in GitHub release notes
- Added documentation deployment job to CI/CD pipeline (deploys to GitHub Pages after release)
- Added case study documentation for issue #25

### Fixed
- Fixed unsupported look-ahead regex in `create-github-release.rs` that caused a panic when parsing CHANGELOG.md. Replaced with a two-step approach using only features supported by Rust's `regex` crate.

### Changed
- Restructured example application as a simple CLI sum calculator using `lino-arguments`
- Renamed default package to `example-sum-package-name` with Unlicense license
- Reorganized test structure: `tests/unit/sum.rs`, `tests/integration/sum.rs`, `tests/unit/ci-cd/`
- Converted experiment scripts into proper unit tests in `tests/unit/ci-cd/changelog_parsing.rs`
- Added CI/CD skip logic for template default package name `example-sum-package-name`
- Updated README.md badges and documentation

### Fixed

- Change detection script now uses per-commit diff instead of full PR diff, so commits touching only non-code files correctly skip CI jobs even when earlier commits in the same PR changed code files

### Fixed
- Make release scripts resolve the publishable crate manifest when the repository root uses a Cargo workspace manifest.

### Fixed
- Decoupled GitHub Pages documentation deployment from package release publication and fixed release-script warning failures under `RUSTFLAGS=-Dwarnings`.

### Changed
- Added explicit GitHub Actions job timeouts and documented Rust test timeout guidance.

### Fixed
- Added a non-blocking warning threshold to the Rust file-size check so near-limit files are surfaced before concurrent PR merges can exceed the hard limit.

### Fixed
- Made `create-github-release.rs` build GitHub release titles as `[Language] X.Y.Z` instead of reusing the tag prefix.

### Added
- Added optional Docker Hub image publishing tied to Rust crate releases, including crates.io visibility waiting, version/latest image tags, and Docker Hub badges in GitHub release notes.

### Changed
- Release completeness checks now self-heal when crates.io exists but configured Docker Hub or GitHub release artifacts are missing.

### Fixed
- Switched documentation deployment to the official GitHub Pages artifact workflow so repositories using GitHub Actions as their Pages source do not get false-positive branch-push deploys.

### Added
- Documented the one-time `Settings → Pages → Source = GitHub Actions` prerequisite for the `deploy-docs` job in `README.md` and as a comment above the `deploy-docs` job in `release.yml`, so downstream template users hit a documented setup step instead of a `Get Pages site failed` error on the first deploy.

### Added
- Tracking case study at `docs/case-studies/issue-52/` registering the `browser-commander` + Playwright preview-regeneration pattern from [`konard/vk-bot-desktop#52`](https://github.com/konard/vk-bot-desktop/pull/52), with an activation checklist for when an example-app surface lands in this template. Documentation only — no workflow, script, or runtime code changes. Primary upstream tracking issue: [`link-foundation/js-ai-driven-development-pipeline-template#62`](https://github.com/link-foundation/js-ai-driven-development-pipeline-template/issues/62).

### Fixed
- Release automation now keeps the workspace package entry in `Cargo.lock` synchronized when `scripts/version-and-commit.rs` bumps `Cargo.toml`, preventing stale lock-file version diffs in later pull requests.

### Fixed
- `scripts/publish-crate.rs` now treats crates.io HTTP 429 throttle responses ("You have published too many versions of this crate in the last 24 hours") as a deferred `publish_result=rate_limited` outcome (it writes the output, prints an explanatory banner and exits successfully) instead of a hard CI failure reported as a generic `failed` ("Failed to publish for unknown reason"). Authentication, already-published and unknown failures still exit non-zero. Failed-publish classification is consolidated through a single `classify_failure` function and `FailureKind` enum (with an `is_deferred` predicate), covered by unit tests runnable via `rust-script --test scripts/publish-crate.rs`.
- The release workflow (`.github/workflows/release.yml`) now gates crate-availability waiting, Docker Hub publishing and GitHub release creation on either an already-published crate or `publish_result=success`, so a deferred (rate-limited) crate upload no longer produces partial downstream release artifacts and the same version is retried automatically on the next push to `main`.

### Fixed
- Fixed reversed `cancel-in-progress` concurrency condition in `release.yml` that cancelled in-flight releases on `main` and never superseded older PR runs. The condition now uses `!=` so `main` releases run to completion while newer PR pushes cancel stale runs.

### Added
- Added a `scripts/check-crate-size.rs` guard that builds the `.crate` archive and fails the release before publishing when it exceeds the crates.io 10 MiB upload limit. The check runs in the build job and before publishing in both the auto-release and manual-release jobs.

### Changed
- Added a narrow `include` allowlist to `Cargo.toml` so docs, case studies, generated CI artifacts, changelog fragments, scripts, and experiments no longer inflate the published release archive.

### Fixed
- Prevented GitHub release creation from treating generic API validation failures as existing releases, and capped oversized release notes with a link to the full tagged changelog.

### Added
- Added the initial `meta-language` links-network core with self-description roots,
  verification, trivia attachment configuration, and a minimal CLI.
- Added the default lossless `parse` entry point, projection views, and a
  tested parity roadmap for competitor features, grammar embedding, and language
  coverage targets.
- Added exact reconstruction, mixed-region detection, query matching,
  substitution rules, concept reconstruction, object identity helpers,
  many-valued truth values, and executable parity fixtures for every tracked
  competitor target.
- Added executable language fixtures and tests for every requested Markdown,
  HTML, top-ten programming-language, and top-ten natural-language target.

### Fixed
- Added a CI guard that rejects Rust test modules and test attributes under `src/`, keeping tests in the `tests/` tree.

## Minor Changes

- Add immutable and mutable network snapshots with provenance and forward
  version commits for roadmap snapshot/versioning coverage.

### Added
- Added `txt` as a first-class markup/container target with a UTF-8 lossless
  fixture and content-sniffing fallback regions.

### Fixed
- Reconciled the natural-language target ordering with the Ethnologue 2025
  total-speaker order cited by the parity roadmap.

### Added
- Added a grammar-backed parser front end for Python, C, Java, C++, C#, JavaScript, and R using official tree-sitter grammar crates.

### Added
- Parse supported mixed-mode embedded regions into the host links network with grammar-backed syntax links.

### Added
- Added natural-language segmentation, identification, normalization, and bidi annotation links over lossless text parses.

### Added
- Added grammar-backed Visual Basic parsing with byte-exact reconstruction and recovery flag coverage.

bump: minor

- Add a grammar-backed `sql-ansi` SQL-family dialect fixture using
  `tree-sitter-sequel`.

### Added
- Added grammar-backed Delphi/Object Pascal parsing through `tree-sitter-pascal`.

### Added
- Enriched link queries with S-expression structural matching, captures, by-type construction, host predicate hooks, and link-cli-style variable substitution bindings.

### Added
- Added persistent snapshot structural sharing with interned metadata text storage.

### Added
- Added a query-transform surface for selecting captured links and replacing
  their source text while preserving unchanged bytes.

### Added
- Materialized self-description roots as controlled links with complete root-definition closure and round-trippable `describe` output.

### Added
- Added common concept ontology seeding from meta-expression's semantic lexicon, with shared concept links, syntax mappings, and structural programming-language concepts.

### Added
- Added semantic cross-language reconstruction for the Hawaii statehood fixture,
  including English/Russian naturalization and configurable formalization levels.

### Added
- Expanded `PARITY_FIXTURES` with multiple provenance-tracked fixtures for tree-sitter, LibCST, Recast, jscodeshift, Rowan, cstree, and Roslyn, including executable recovery and query/transform expectations.

### Added

- Expanded `PARITY_FIXTURES` with upstream-provenanced internal ecosystem corpora for links-notation, link-cli, lino-objects-codec, relative-meta-logic, formal-ai, and meta-expression.

## [0.34.0] - 2026-06-07

### Added
- Changeset-style fragment format with frontmatter for specifying version bump type
- New `get-bump-type.mjs` script to automatically determine version bump from fragments
- Automatic version bumping on merge to main based on changelog fragments
- Detailed documentation for the changelog fragment system in `changelog.d/README.md`

### Changed
- Updated `collect-changelog.mjs` to strip frontmatter when collecting fragments
- Updated `version-and-commit.mjs` to handle frontmatter in fragments
- Enhanced release workflow to automatically determine bump type from changesets

### Changed
- Add `detect-changes` job with cross-platform `detect-code-changes.mjs` script
- Make lint job independent of changelog check (runs based on file changes only)
- Allow docs-only PRs without changelog fragment requirement
- Handle changelog check 'skipped' state in dependent jobs
- Exclude `changelog.d/`, `docs/`, `experiments/`, `examples/` folders and markdown files from code changes detection

### Fixed
- Fixed README.md to correctly reference Node.js scripts (`.mjs`) instead of Python scripts (`.py`)
- Updated project structure in README.md to match actual script files in `scripts/` directory
- Fixed example code in README.md that had invalid Rust with two `main` functions

### Added

- Added crates.io publishing support to CI/CD workflow
- Added `release_mode` input with "instant" and "changelog-pr" options for manual releases
- Added `--tag-prefix` and `--crates-io-url` options to create-github-release.mjs script
- Added comprehensive case study documentation for Issue #11 in docs/case-studies/issue-11/

### Changed

- Changed changelog fragment check from warning to error (exit 1) to enforce changelog requirements
- Updated job conditions with `always() && !cancelled()` to fix workflow_dispatch job skipping issue
- Renamed manual-release job to "Instant Release" for clarity

### Fixed

- Fixed deprecated `::set-output` GitHub Actions command in version-and-commit.mjs
- Fixed workflow_dispatch triggering issues where lint/build/release jobs were incorrectly skipped

### Fixed

- Fixed changelog fragment check to validate that a fragment is **added in the PR diff** rather than just checking if any fragments exist in the directory. This prevents the check from incorrectly passing when there are leftover fragments from previous PRs that haven't been released yet.

### Changed

- Converted shell scripts in `release.yml` to cross-platform `.mjs` scripts for improved portability and performance:
  - `check-changelog-fragment.mjs` - validates changelog fragment is added in PR diff
  - `git-config.mjs` - configures git user for CI/CD
  - `check-release-needed.mjs` - checks if release is needed
  - `publish-crate.mjs` - publishes package to crates.io
  - `create-changelog-fragment.mjs` - creates changelog fragments for manual releases
  - `get-version.mjs` - gets current version from Cargo.toml

### Added

- Added `check-version-modification.mjs` script to detect manual version changes in Cargo.toml
- Added `version-check` job to CI/CD workflow that runs on pull requests
- Added skip logic for automated release branches (changelog-manual-release-*, changeset-release/*, release/*, automated-release/*)

### Changed

- Version modifications in Cargo.toml are now blocked in pull requests to enforce automated release pipeline

### Added

- Added support for `CARGO_REGISTRY_TOKEN` as alternative to `CARGO_TOKEN` for crates.io publishing
- Added case study documentation for Issue #17 (yargs reserved word and dual token support)

### Changed

- Updated workflow to use fallback logic: `${{ secrets.CARGO_REGISTRY_TOKEN || secrets.CARGO_TOKEN }}`
- Improved publish-crate.mjs to check both `CARGO_REGISTRY_TOKEN` and `CARGO_TOKEN` environment variables
- Added warning message when neither token is set

### Added
- New `scripts/rust-paths.mjs` utility for automatic Rust package root detection
- Support for both single-language and multi-language repository structures in all CI/CD scripts
- Configuration options via `--rust-root` CLI argument and `RUST_ROOT` environment variable
- Comprehensive case study documentation in `docs/case-studies/issue-19/`

### Changed
- Updated all release scripts to use the new path detection utility:
  - `scripts/bump-version.mjs`
  - `scripts/check-release-needed.mjs`
  - `scripts/collect-changelog.mjs`
  - `scripts/get-bump-type.mjs`
  - `scripts/get-version.mjs`
  - `scripts/publish-crate.mjs`
  - `scripts/version-and-commit.mjs`

### Changed

- **check-release-needed.mjs**: Now checks crates.io API directly instead of git tags to determine if a version is already released. This prevents false positives where git tags exist but the package was never actually published to crates.io.

### Added

- **CI/CD Troubleshooting Guide**: New documentation at `docs/ci-cd/troubleshooting.md` covering common issues like skipped jobs, false positive version checks, publishing failures, and secret configuration.

- **Enhanced Error Handling in publish-crate.mjs**: Added specific detection and helpful error messages for authentication failures, including guidance on secret configuration and workflow setup.

- **Case Study Documentation**: Added comprehensive case study at `docs/case-studies/issue-21/` analyzing CI/CD failures from browser-commander repository (issues #27, #29, #31, #33) with timeline, root causes, and lessons learned.

### Fixed

- **Prevent False Positive Version Checks**: The release workflow now correctly identifies unpublished versions by checking crates.io instead of relying on git tags, which can exist without the package being published.

### Changed

- Translated all CI/CD scripts from JavaScript (.mjs) to Rust (.rs) using rust-script
- Scripts now use native Rust with rust-script for execution in shell
- Removed Node.js dependency from CI/CD pipeline
- Updated GitHub Actions workflow to use rust-script instead of node
- Updated README and CONTRIBUTING documentation with new script references

### Added

- Cache `restore-keys` for partial cache hits across all workflow jobs
- Explicit `token` parameter in checkout for release jobs
- Code coverage job with `cargo-llvm-cov` and Codecov integration
- Codecov badge in README.md
- Pre-release version support (e.g., `0.1.0-beta.1`) in version parsing
- `--release-label` parameter for multi-language release disambiguation
- `ensure_version_exceeds_published()` logic to prevent publishing duplicate versions
- `get_max_published_version()` to query highest non-yanked version from crates.io
- `max_published_version` output from check-release-needed for downstream use
- Version fallback logic in auto-release Create GitHub Release step

### Changed

- Updated `actions/checkout` from v4 to v6
- Updated `actions/cache` from v4 to v5
- Updated `peter-evans/create-pull-request` from v7 to v8
- Made `publish-crate.rs` fail (exit 1) when version already exists on crates.io
- Improved `create-github-release.rs` to check combined stdout+stderr and detect "Validation Failed"

### Fixed

- Fix publish steps overriding workflow-level CARGO_TOKEN fallback, breaking CARGO_REGISTRY_TOKEN-only configurations (#32)
- Fix non-fast-forward push failures in multi-workflow repos by adding fetch/rebase and push retry logic (#31)
- Add mono-repo path support to check-changelog-fragment.rs, check-version-modification.rs, and create-changelog-fragment.rs
- Add `!cancelled()` guard to test job condition to respect workflow cancellation

### Fixed

- Fixed `version-and-commit.rs` to check crates.io instead of git tags for determining if a version is already released
- This prevents the release pipeline from getting stuck when git tags exist without corresponding crates.io publication

### Added

- Added `--tag-prefix` support to `version-and-commit.rs` for multi-language repository compatibility
- Added crates.io and docs.rs badges to README.md
- Added automatic crates.io and docs.rs badge injection in GitHub release notes
- Added documentation deployment job to CI/CD pipeline (deploys to GitHub Pages after release)
- Added case study documentation for issue #25

### Fixed
- Fixed unsupported look-ahead regex in `create-github-release.rs` that caused a panic when parsing CHANGELOG.md. Replaced with a two-step approach using only features supported by Rust's `regex` crate.

### Changed
- Restructured example application as a simple CLI sum calculator using `lino-arguments`
- Renamed default package to `example-sum-package-name` with Unlicense license
- Reorganized test structure: `tests/unit/sum.rs`, `tests/integration/sum.rs`, `tests/unit/ci-cd/`
- Converted experiment scripts into proper unit tests in `tests/unit/ci-cd/changelog_parsing.rs`
- Added CI/CD skip logic for template default package name `example-sum-package-name`
- Updated README.md badges and documentation

### Fixed

- Change detection script now uses per-commit diff instead of full PR diff, so commits touching only non-code files correctly skip CI jobs even when earlier commits in the same PR changed code files

### Fixed
- Make release scripts resolve the publishable crate manifest when the repository root uses a Cargo workspace manifest.

### Fixed
- Decoupled GitHub Pages documentation deployment from package release publication and fixed release-script warning failures under `RUSTFLAGS=-Dwarnings`.

### Changed
- Added explicit GitHub Actions job timeouts and documented Rust test timeout guidance.

### Fixed
- Added a non-blocking warning threshold to the Rust file-size check so near-limit files are surfaced before concurrent PR merges can exceed the hard limit.

### Fixed
- Made `create-github-release.rs` build GitHub release titles as `[Language] X.Y.Z` instead of reusing the tag prefix.

### Added
- Added optional Docker Hub image publishing tied to Rust crate releases, including crates.io visibility waiting, version/latest image tags, and Docker Hub badges in GitHub release notes.

### Changed
- Release completeness checks now self-heal when crates.io exists but configured Docker Hub or GitHub release artifacts are missing.

### Fixed
- Switched documentation deployment to the official GitHub Pages artifact workflow so repositories using GitHub Actions as their Pages source do not get false-positive branch-push deploys.

### Added
- Documented the one-time `Settings → Pages → Source = GitHub Actions` prerequisite for the `deploy-docs` job in `README.md` and as a comment above the `deploy-docs` job in `release.yml`, so downstream template users hit a documented setup step instead of a `Get Pages site failed` error on the first deploy.

### Added
- Tracking case study at `docs/case-studies/issue-52/` registering the `browser-commander` + Playwright preview-regeneration pattern from [`konard/vk-bot-desktop#52`](https://github.com/konard/vk-bot-desktop/pull/52), with an activation checklist for when an example-app surface lands in this template. Documentation only — no workflow, script, or runtime code changes. Primary upstream tracking issue: [`link-foundation/js-ai-driven-development-pipeline-template#62`](https://github.com/link-foundation/js-ai-driven-development-pipeline-template/issues/62).

### Fixed
- Release automation now keeps the workspace package entry in `Cargo.lock` synchronized when `scripts/version-and-commit.rs` bumps `Cargo.toml`, preventing stale lock-file version diffs in later pull requests.

### Fixed
- `scripts/publish-crate.rs` now treats crates.io HTTP 429 throttle responses ("You have published too many versions of this crate in the last 24 hours") as a deferred `publish_result=rate_limited` outcome (it writes the output, prints an explanatory banner and exits successfully) instead of a hard CI failure reported as a generic `failed` ("Failed to publish for unknown reason"). Authentication, already-published and unknown failures still exit non-zero. Failed-publish classification is consolidated through a single `classify_failure` function and `FailureKind` enum (with an `is_deferred` predicate), covered by unit tests runnable via `rust-script --test scripts/publish-crate.rs`.
- The release workflow (`.github/workflows/release.yml`) now gates crate-availability waiting, Docker Hub publishing and GitHub release creation on either an already-published crate or `publish_result=success`, so a deferred (rate-limited) crate upload no longer produces partial downstream release artifacts and the same version is retried automatically on the next push to `main`.

### Fixed
- Fixed reversed `cancel-in-progress` concurrency condition in `release.yml` that cancelled in-flight releases on `main` and never superseded older PR runs. The condition now uses `!=` so `main` releases run to completion while newer PR pushes cancel stale runs.

### Added
- Added a `scripts/check-crate-size.rs` guard that builds the `.crate` archive and fails the release before publishing when it exceeds the crates.io 10 MiB upload limit. The check runs in the build job and before publishing in both the auto-release and manual-release jobs.

### Changed
- Added a narrow `include` allowlist to `Cargo.toml` so docs, case studies, generated CI artifacts, changelog fragments, scripts, and experiments no longer inflate the published release archive.

### Fixed
- Prevented GitHub release creation from treating generic API validation failures as existing releases, and capped oversized release notes with a link to the full tagged changelog.

### Added
- Added the initial `meta-language` links-network core with self-description roots,
  verification, trivia attachment configuration, and a minimal CLI.
- Added the default lossless `parse` entry point, projection views, and a
  tested parity roadmap for competitor features, grammar embedding, and language
  coverage targets.
- Added exact reconstruction, mixed-region detection, query matching,
  substitution rules, concept reconstruction, object identity helpers,
  many-valued truth values, and executable parity fixtures for every tracked
  competitor target.
- Added executable language fixtures and tests for every requested Markdown,
  HTML, top-ten programming-language, and top-ten natural-language target.

### Fixed
- Added a CI guard that rejects Rust test modules and test attributes under `src/`, keeping tests in the `tests/` tree.

## Minor Changes

- Add immutable and mutable network snapshots with provenance and forward
  version commits for roadmap snapshot/versioning coverage.

### Added
- Added `txt` as a first-class markup/container target with a UTF-8 lossless
  fixture and content-sniffing fallback regions.

### Fixed
- Reconciled the natural-language target ordering with the Ethnologue 2025
  total-speaker order cited by the parity roadmap.

### Added
- Added a grammar-backed parser front end for Python, C, Java, C++, C#, JavaScript, and R using official tree-sitter grammar crates.

### Added
- Parse supported mixed-mode embedded regions into the host links network with grammar-backed syntax links.

### Added
- Added natural-language segmentation, identification, normalization, and bidi annotation links over lossless text parses.

### Added
- Added grammar-backed Visual Basic parsing with byte-exact reconstruction and recovery flag coverage.

bump: minor

- Add a grammar-backed `sql-ansi` SQL-family dialect fixture using
  `tree-sitter-sequel`.

### Added
- Added grammar-backed Delphi/Object Pascal parsing through `tree-sitter-pascal`.

### Added
- Enriched link queries with S-expression structural matching, captures, by-type construction, host predicate hooks, and link-cli-style variable substitution bindings.

### Added
- Added persistent snapshot structural sharing with interned metadata text storage.

### Added
- Added a query-transform surface for selecting captured links and replacing
  their source text while preserving unchanged bytes.

### Added
- Materialized self-description roots as controlled links with complete root-definition closure and round-trippable `describe` output.

### Added
- Added common concept ontology seeding from meta-expression's semantic lexicon, with shared concept links, syntax mappings, and structural programming-language concepts.

### Added
- Added semantic cross-language reconstruction for the Hawaii statehood fixture,
  including English/Russian naturalization and configurable formalization levels.

### Added
- Expanded `PARITY_FIXTURES` with multiple provenance-tracked fixtures for tree-sitter, LibCST, Recast, jscodeshift, Rowan, cstree, and Roslyn, including executable recovery and query/transform expectations.

## [0.33.0] - 2026-06-07

### Added
- Changeset-style fragment format with frontmatter for specifying version bump type
- New `get-bump-type.mjs` script to automatically determine version bump from fragments
- Automatic version bumping on merge to main based on changelog fragments
- Detailed documentation for the changelog fragment system in `changelog.d/README.md`

### Changed
- Updated `collect-changelog.mjs` to strip frontmatter when collecting fragments
- Updated `version-and-commit.mjs` to handle frontmatter in fragments
- Enhanced release workflow to automatically determine bump type from changesets

### Changed
- Add `detect-changes` job with cross-platform `detect-code-changes.mjs` script
- Make lint job independent of changelog check (runs based on file changes only)
- Allow docs-only PRs without changelog fragment requirement
- Handle changelog check 'skipped' state in dependent jobs
- Exclude `changelog.d/`, `docs/`, `experiments/`, `examples/` folders and markdown files from code changes detection

### Fixed
- Fixed README.md to correctly reference Node.js scripts (`.mjs`) instead of Python scripts (`.py`)
- Updated project structure in README.md to match actual script files in `scripts/` directory
- Fixed example code in README.md that had invalid Rust with two `main` functions

### Added

- Added crates.io publishing support to CI/CD workflow
- Added `release_mode` input with "instant" and "changelog-pr" options for manual releases
- Added `--tag-prefix` and `--crates-io-url` options to create-github-release.mjs script
- Added comprehensive case study documentation for Issue #11 in docs/case-studies/issue-11/

### Changed

- Changed changelog fragment check from warning to error (exit 1) to enforce changelog requirements
- Updated job conditions with `always() && !cancelled()` to fix workflow_dispatch job skipping issue
- Renamed manual-release job to "Instant Release" for clarity

### Fixed

- Fixed deprecated `::set-output` GitHub Actions command in version-and-commit.mjs
- Fixed workflow_dispatch triggering issues where lint/build/release jobs were incorrectly skipped

### Fixed

- Fixed changelog fragment check to validate that a fragment is **added in the PR diff** rather than just checking if any fragments exist in the directory. This prevents the check from incorrectly passing when there are leftover fragments from previous PRs that haven't been released yet.

### Changed

- Converted shell scripts in `release.yml` to cross-platform `.mjs` scripts for improved portability and performance:
  - `check-changelog-fragment.mjs` - validates changelog fragment is added in PR diff
  - `git-config.mjs` - configures git user for CI/CD
  - `check-release-needed.mjs` - checks if release is needed
  - `publish-crate.mjs` - publishes package to crates.io
  - `create-changelog-fragment.mjs` - creates changelog fragments for manual releases
  - `get-version.mjs` - gets current version from Cargo.toml

### Added

- Added `check-version-modification.mjs` script to detect manual version changes in Cargo.toml
- Added `version-check` job to CI/CD workflow that runs on pull requests
- Added skip logic for automated release branches (changelog-manual-release-*, changeset-release/*, release/*, automated-release/*)

### Changed

- Version modifications in Cargo.toml are now blocked in pull requests to enforce automated release pipeline

### Added

- Added support for `CARGO_REGISTRY_TOKEN` as alternative to `CARGO_TOKEN` for crates.io publishing
- Added case study documentation for Issue #17 (yargs reserved word and dual token support)

### Changed

- Updated workflow to use fallback logic: `${{ secrets.CARGO_REGISTRY_TOKEN || secrets.CARGO_TOKEN }}`
- Improved publish-crate.mjs to check both `CARGO_REGISTRY_TOKEN` and `CARGO_TOKEN` environment variables
- Added warning message when neither token is set

### Added
- New `scripts/rust-paths.mjs` utility for automatic Rust package root detection
- Support for both single-language and multi-language repository structures in all CI/CD scripts
- Configuration options via `--rust-root` CLI argument and `RUST_ROOT` environment variable
- Comprehensive case study documentation in `docs/case-studies/issue-19/`

### Changed
- Updated all release scripts to use the new path detection utility:
  - `scripts/bump-version.mjs`
  - `scripts/check-release-needed.mjs`
  - `scripts/collect-changelog.mjs`
  - `scripts/get-bump-type.mjs`
  - `scripts/get-version.mjs`
  - `scripts/publish-crate.mjs`
  - `scripts/version-and-commit.mjs`

### Changed

- **check-release-needed.mjs**: Now checks crates.io API directly instead of git tags to determine if a version is already released. This prevents false positives where git tags exist but the package was never actually published to crates.io.

### Added

- **CI/CD Troubleshooting Guide**: New documentation at `docs/ci-cd/troubleshooting.md` covering common issues like skipped jobs, false positive version checks, publishing failures, and secret configuration.

- **Enhanced Error Handling in publish-crate.mjs**: Added specific detection and helpful error messages for authentication failures, including guidance on secret configuration and workflow setup.

- **Case Study Documentation**: Added comprehensive case study at `docs/case-studies/issue-21/` analyzing CI/CD failures from browser-commander repository (issues #27, #29, #31, #33) with timeline, root causes, and lessons learned.

### Fixed

- **Prevent False Positive Version Checks**: The release workflow now correctly identifies unpublished versions by checking crates.io instead of relying on git tags, which can exist without the package being published.

### Changed

- Translated all CI/CD scripts from JavaScript (.mjs) to Rust (.rs) using rust-script
- Scripts now use native Rust with rust-script for execution in shell
- Removed Node.js dependency from CI/CD pipeline
- Updated GitHub Actions workflow to use rust-script instead of node
- Updated README and CONTRIBUTING documentation with new script references

### Added

- Cache `restore-keys` for partial cache hits across all workflow jobs
- Explicit `token` parameter in checkout for release jobs
- Code coverage job with `cargo-llvm-cov` and Codecov integration
- Codecov badge in README.md
- Pre-release version support (e.g., `0.1.0-beta.1`) in version parsing
- `--release-label` parameter for multi-language release disambiguation
- `ensure_version_exceeds_published()` logic to prevent publishing duplicate versions
- `get_max_published_version()` to query highest non-yanked version from crates.io
- `max_published_version` output from check-release-needed for downstream use
- Version fallback logic in auto-release Create GitHub Release step

### Changed

- Updated `actions/checkout` from v4 to v6
- Updated `actions/cache` from v4 to v5
- Updated `peter-evans/create-pull-request` from v7 to v8
- Made `publish-crate.rs` fail (exit 1) when version already exists on crates.io
- Improved `create-github-release.rs` to check combined stdout+stderr and detect "Validation Failed"

### Fixed

- Fix publish steps overriding workflow-level CARGO_TOKEN fallback, breaking CARGO_REGISTRY_TOKEN-only configurations (#32)
- Fix non-fast-forward push failures in multi-workflow repos by adding fetch/rebase and push retry logic (#31)
- Add mono-repo path support to check-changelog-fragment.rs, check-version-modification.rs, and create-changelog-fragment.rs
- Add `!cancelled()` guard to test job condition to respect workflow cancellation

### Fixed

- Fixed `version-and-commit.rs` to check crates.io instead of git tags for determining if a version is already released
- This prevents the release pipeline from getting stuck when git tags exist without corresponding crates.io publication

### Added

- Added `--tag-prefix` support to `version-and-commit.rs` for multi-language repository compatibility
- Added crates.io and docs.rs badges to README.md
- Added automatic crates.io and docs.rs badge injection in GitHub release notes
- Added documentation deployment job to CI/CD pipeline (deploys to GitHub Pages after release)
- Added case study documentation for issue #25

### Fixed
- Fixed unsupported look-ahead regex in `create-github-release.rs` that caused a panic when parsing CHANGELOG.md. Replaced with a two-step approach using only features supported by Rust's `regex` crate.

### Changed
- Restructured example application as a simple CLI sum calculator using `lino-arguments`
- Renamed default package to `example-sum-package-name` with Unlicense license
- Reorganized test structure: `tests/unit/sum.rs`, `tests/integration/sum.rs`, `tests/unit/ci-cd/`
- Converted experiment scripts into proper unit tests in `tests/unit/ci-cd/changelog_parsing.rs`
- Added CI/CD skip logic for template default package name `example-sum-package-name`
- Updated README.md badges and documentation

### Fixed

- Change detection script now uses per-commit diff instead of full PR diff, so commits touching only non-code files correctly skip CI jobs even when earlier commits in the same PR changed code files

### Fixed
- Make release scripts resolve the publishable crate manifest when the repository root uses a Cargo workspace manifest.

### Fixed
- Decoupled GitHub Pages documentation deployment from package release publication and fixed release-script warning failures under `RUSTFLAGS=-Dwarnings`.

### Changed
- Added explicit GitHub Actions job timeouts and documented Rust test timeout guidance.

### Fixed
- Added a non-blocking warning threshold to the Rust file-size check so near-limit files are surfaced before concurrent PR merges can exceed the hard limit.

### Fixed
- Made `create-github-release.rs` build GitHub release titles as `[Language] X.Y.Z` instead of reusing the tag prefix.

### Added
- Added optional Docker Hub image publishing tied to Rust crate releases, including crates.io visibility waiting, version/latest image tags, and Docker Hub badges in GitHub release notes.

### Changed
- Release completeness checks now self-heal when crates.io exists but configured Docker Hub or GitHub release artifacts are missing.

### Fixed
- Switched documentation deployment to the official GitHub Pages artifact workflow so repositories using GitHub Actions as their Pages source do not get false-positive branch-push deploys.

### Added
- Documented the one-time `Settings → Pages → Source = GitHub Actions` prerequisite for the `deploy-docs` job in `README.md` and as a comment above the `deploy-docs` job in `release.yml`, so downstream template users hit a documented setup step instead of a `Get Pages site failed` error on the first deploy.

### Added
- Tracking case study at `docs/case-studies/issue-52/` registering the `browser-commander` + Playwright preview-regeneration pattern from [`konard/vk-bot-desktop#52`](https://github.com/konard/vk-bot-desktop/pull/52), with an activation checklist for when an example-app surface lands in this template. Documentation only — no workflow, script, or runtime code changes. Primary upstream tracking issue: [`link-foundation/js-ai-driven-development-pipeline-template#62`](https://github.com/link-foundation/js-ai-driven-development-pipeline-template/issues/62).

### Fixed
- Release automation now keeps the workspace package entry in `Cargo.lock` synchronized when `scripts/version-and-commit.rs` bumps `Cargo.toml`, preventing stale lock-file version diffs in later pull requests.

### Fixed
- `scripts/publish-crate.rs` now treats crates.io HTTP 429 throttle responses ("You have published too many versions of this crate in the last 24 hours") as a deferred `publish_result=rate_limited` outcome (it writes the output, prints an explanatory banner and exits successfully) instead of a hard CI failure reported as a generic `failed` ("Failed to publish for unknown reason"). Authentication, already-published and unknown failures still exit non-zero. Failed-publish classification is consolidated through a single `classify_failure` function and `FailureKind` enum (with an `is_deferred` predicate), covered by unit tests runnable via `rust-script --test scripts/publish-crate.rs`.
- The release workflow (`.github/workflows/release.yml`) now gates crate-availability waiting, Docker Hub publishing and GitHub release creation on either an already-published crate or `publish_result=success`, so a deferred (rate-limited) crate upload no longer produces partial downstream release artifacts and the same version is retried automatically on the next push to `main`.

### Fixed
- Fixed reversed `cancel-in-progress` concurrency condition in `release.yml` that cancelled in-flight releases on `main` and never superseded older PR runs. The condition now uses `!=` so `main` releases run to completion while newer PR pushes cancel stale runs.

### Added
- Added a `scripts/check-crate-size.rs` guard that builds the `.crate` archive and fails the release before publishing when it exceeds the crates.io 10 MiB upload limit. The check runs in the build job and before publishing in both the auto-release and manual-release jobs.

### Changed
- Added a narrow `include` allowlist to `Cargo.toml` so docs, case studies, generated CI artifacts, changelog fragments, scripts, and experiments no longer inflate the published release archive.

### Fixed
- Prevented GitHub release creation from treating generic API validation failures as existing releases, and capped oversized release notes with a link to the full tagged changelog.

### Added
- Added the initial `meta-language` links-network core with self-description roots,
  verification, trivia attachment configuration, and a minimal CLI.
- Added the default lossless `parse` entry point, projection views, and a
  tested parity roadmap for competitor features, grammar embedding, and language
  coverage targets.
- Added exact reconstruction, mixed-region detection, query matching,
  substitution rules, concept reconstruction, object identity helpers,
  many-valued truth values, and executable parity fixtures for every tracked
  competitor target.
- Added executable language fixtures and tests for every requested Markdown,
  HTML, top-ten programming-language, and top-ten natural-language target.

### Fixed
- Added a CI guard that rejects Rust test modules and test attributes under `src/`, keeping tests in the `tests/` tree.

## Minor Changes

- Add immutable and mutable network snapshots with provenance and forward
  version commits for roadmap snapshot/versioning coverage.

### Added
- Added `txt` as a first-class markup/container target with a UTF-8 lossless
  fixture and content-sniffing fallback regions.

### Fixed
- Reconciled the natural-language target ordering with the Ethnologue 2025
  total-speaker order cited by the parity roadmap.

### Added
- Added a grammar-backed parser front end for Python, C, Java, C++, C#, JavaScript, and R using official tree-sitter grammar crates.

### Added
- Parse supported mixed-mode embedded regions into the host links network with grammar-backed syntax links.

### Added
- Added natural-language segmentation, identification, normalization, and bidi annotation links over lossless text parses.

### Added
- Added grammar-backed Visual Basic parsing with byte-exact reconstruction and recovery flag coverage.

bump: minor

- Add a grammar-backed `sql-ansi` SQL-family dialect fixture using
  `tree-sitter-sequel`.

### Added
- Added grammar-backed Delphi/Object Pascal parsing through `tree-sitter-pascal`.

### Added
- Enriched link queries with S-expression structural matching, captures, by-type construction, host predicate hooks, and link-cli-style variable substitution bindings.

### Added
- Added persistent snapshot structural sharing with interned metadata text storage.

### Added
- Added a query-transform surface for selecting captured links and replacing
  their source text while preserving unchanged bytes.

### Added
- Materialized self-description roots as controlled links with complete root-definition closure and round-trippable `describe` output.

### Added
- Added common concept ontology seeding from meta-expression's semantic lexicon, with shared concept links, syntax mappings, and structural programming-language concepts.

### Added
- Added semantic cross-language reconstruction for the Hawaii statehood fixture,
  including English/Russian naturalization and configurable formalization levels.

## [0.32.0] - 2026-06-07

### Added
- Changeset-style fragment format with frontmatter for specifying version bump type
- New `get-bump-type.mjs` script to automatically determine version bump from fragments
- Automatic version bumping on merge to main based on changelog fragments
- Detailed documentation for the changelog fragment system in `changelog.d/README.md`

### Changed
- Updated `collect-changelog.mjs` to strip frontmatter when collecting fragments
- Updated `version-and-commit.mjs` to handle frontmatter in fragments
- Enhanced release workflow to automatically determine bump type from changesets

### Changed
- Add `detect-changes` job with cross-platform `detect-code-changes.mjs` script
- Make lint job independent of changelog check (runs based on file changes only)
- Allow docs-only PRs without changelog fragment requirement
- Handle changelog check 'skipped' state in dependent jobs
- Exclude `changelog.d/`, `docs/`, `experiments/`, `examples/` folders and markdown files from code changes detection

### Fixed
- Fixed README.md to correctly reference Node.js scripts (`.mjs`) instead of Python scripts (`.py`)
- Updated project structure in README.md to match actual script files in `scripts/` directory
- Fixed example code in README.md that had invalid Rust with two `main` functions

### Added

- Added crates.io publishing support to CI/CD workflow
- Added `release_mode` input with "instant" and "changelog-pr" options for manual releases
- Added `--tag-prefix` and `--crates-io-url` options to create-github-release.mjs script
- Added comprehensive case study documentation for Issue #11 in docs/case-studies/issue-11/

### Changed

- Changed changelog fragment check from warning to error (exit 1) to enforce changelog requirements
- Updated job conditions with `always() && !cancelled()` to fix workflow_dispatch job skipping issue
- Renamed manual-release job to "Instant Release" for clarity

### Fixed

- Fixed deprecated `::set-output` GitHub Actions command in version-and-commit.mjs
- Fixed workflow_dispatch triggering issues where lint/build/release jobs were incorrectly skipped

### Fixed

- Fixed changelog fragment check to validate that a fragment is **added in the PR diff** rather than just checking if any fragments exist in the directory. This prevents the check from incorrectly passing when there are leftover fragments from previous PRs that haven't been released yet.

### Changed

- Converted shell scripts in `release.yml` to cross-platform `.mjs` scripts for improved portability and performance:
  - `check-changelog-fragment.mjs` - validates changelog fragment is added in PR diff
  - `git-config.mjs` - configures git user for CI/CD
  - `check-release-needed.mjs` - checks if release is needed
  - `publish-crate.mjs` - publishes package to crates.io
  - `create-changelog-fragment.mjs` - creates changelog fragments for manual releases
  - `get-version.mjs` - gets current version from Cargo.toml

### Added

- Added `check-version-modification.mjs` script to detect manual version changes in Cargo.toml
- Added `version-check` job to CI/CD workflow that runs on pull requests
- Added skip logic for automated release branches (changelog-manual-release-*, changeset-release/*, release/*, automated-release/*)

### Changed

- Version modifications in Cargo.toml are now blocked in pull requests to enforce automated release pipeline

### Added

- Added support for `CARGO_REGISTRY_TOKEN` as alternative to `CARGO_TOKEN` for crates.io publishing
- Added case study documentation for Issue #17 (yargs reserved word and dual token support)

### Changed

- Updated workflow to use fallback logic: `${{ secrets.CARGO_REGISTRY_TOKEN || secrets.CARGO_TOKEN }}`
- Improved publish-crate.mjs to check both `CARGO_REGISTRY_TOKEN` and `CARGO_TOKEN` environment variables
- Added warning message when neither token is set

### Added
- New `scripts/rust-paths.mjs` utility for automatic Rust package root detection
- Support for both single-language and multi-language repository structures in all CI/CD scripts
- Configuration options via `--rust-root` CLI argument and `RUST_ROOT` environment variable
- Comprehensive case study documentation in `docs/case-studies/issue-19/`

### Changed
- Updated all release scripts to use the new path detection utility:
  - `scripts/bump-version.mjs`
  - `scripts/check-release-needed.mjs`
  - `scripts/collect-changelog.mjs`
  - `scripts/get-bump-type.mjs`
  - `scripts/get-version.mjs`
  - `scripts/publish-crate.mjs`
  - `scripts/version-and-commit.mjs`

### Changed

- **check-release-needed.mjs**: Now checks crates.io API directly instead of git tags to determine if a version is already released. This prevents false positives where git tags exist but the package was never actually published to crates.io.

### Added

- **CI/CD Troubleshooting Guide**: New documentation at `docs/ci-cd/troubleshooting.md` covering common issues like skipped jobs, false positive version checks, publishing failures, and secret configuration.

- **Enhanced Error Handling in publish-crate.mjs**: Added specific detection and helpful error messages for authentication failures, including guidance on secret configuration and workflow setup.

- **Case Study Documentation**: Added comprehensive case study at `docs/case-studies/issue-21/` analyzing CI/CD failures from browser-commander repository (issues #27, #29, #31, #33) with timeline, root causes, and lessons learned.

### Fixed

- **Prevent False Positive Version Checks**: The release workflow now correctly identifies unpublished versions by checking crates.io instead of relying on git tags, which can exist without the package being published.

### Changed

- Translated all CI/CD scripts from JavaScript (.mjs) to Rust (.rs) using rust-script
- Scripts now use native Rust with rust-script for execution in shell
- Removed Node.js dependency from CI/CD pipeline
- Updated GitHub Actions workflow to use rust-script instead of node
- Updated README and CONTRIBUTING documentation with new script references

### Added

- Cache `restore-keys` for partial cache hits across all workflow jobs
- Explicit `token` parameter in checkout for release jobs
- Code coverage job with `cargo-llvm-cov` and Codecov integration
- Codecov badge in README.md
- Pre-release version support (e.g., `0.1.0-beta.1`) in version parsing
- `--release-label` parameter for multi-language release disambiguation
- `ensure_version_exceeds_published()` logic to prevent publishing duplicate versions
- `get_max_published_version()` to query highest non-yanked version from crates.io
- `max_published_version` output from check-release-needed for downstream use
- Version fallback logic in auto-release Create GitHub Release step

### Changed

- Updated `actions/checkout` from v4 to v6
- Updated `actions/cache` from v4 to v5
- Updated `peter-evans/create-pull-request` from v7 to v8
- Made `publish-crate.rs` fail (exit 1) when version already exists on crates.io
- Improved `create-github-release.rs` to check combined stdout+stderr and detect "Validation Failed"

### Fixed

- Fix publish steps overriding workflow-level CARGO_TOKEN fallback, breaking CARGO_REGISTRY_TOKEN-only configurations (#32)
- Fix non-fast-forward push failures in multi-workflow repos by adding fetch/rebase and push retry logic (#31)
- Add mono-repo path support to check-changelog-fragment.rs, check-version-modification.rs, and create-changelog-fragment.rs
- Add `!cancelled()` guard to test job condition to respect workflow cancellation

### Fixed

- Fixed `version-and-commit.rs` to check crates.io instead of git tags for determining if a version is already released
- This prevents the release pipeline from getting stuck when git tags exist without corresponding crates.io publication

### Added

- Added `--tag-prefix` support to `version-and-commit.rs` for multi-language repository compatibility
- Added crates.io and docs.rs badges to README.md
- Added automatic crates.io and docs.rs badge injection in GitHub release notes
- Added documentation deployment job to CI/CD pipeline (deploys to GitHub Pages after release)
- Added case study documentation for issue #25

### Fixed
- Fixed unsupported look-ahead regex in `create-github-release.rs` that caused a panic when parsing CHANGELOG.md. Replaced with a two-step approach using only features supported by Rust's `regex` crate.

### Changed
- Restructured example application as a simple CLI sum calculator using `lino-arguments`
- Renamed default package to `example-sum-package-name` with Unlicense license
- Reorganized test structure: `tests/unit/sum.rs`, `tests/integration/sum.rs`, `tests/unit/ci-cd/`
- Converted experiment scripts into proper unit tests in `tests/unit/ci-cd/changelog_parsing.rs`
- Added CI/CD skip logic for template default package name `example-sum-package-name`
- Updated README.md badges and documentation

### Fixed

- Change detection script now uses per-commit diff instead of full PR diff, so commits touching only non-code files correctly skip CI jobs even when earlier commits in the same PR changed code files

### Fixed
- Make release scripts resolve the publishable crate manifest when the repository root uses a Cargo workspace manifest.

### Fixed
- Decoupled GitHub Pages documentation deployment from package release publication and fixed release-script warning failures under `RUSTFLAGS=-Dwarnings`.

### Changed
- Added explicit GitHub Actions job timeouts and documented Rust test timeout guidance.

### Fixed
- Added a non-blocking warning threshold to the Rust file-size check so near-limit files are surfaced before concurrent PR merges can exceed the hard limit.

### Fixed
- Made `create-github-release.rs` build GitHub release titles as `[Language] X.Y.Z` instead of reusing the tag prefix.

### Added
- Added optional Docker Hub image publishing tied to Rust crate releases, including crates.io visibility waiting, version/latest image tags, and Docker Hub badges in GitHub release notes.

### Changed
- Release completeness checks now self-heal when crates.io exists but configured Docker Hub or GitHub release artifacts are missing.

### Fixed
- Switched documentation deployment to the official GitHub Pages artifact workflow so repositories using GitHub Actions as their Pages source do not get false-positive branch-push deploys.

### Added
- Documented the one-time `Settings → Pages → Source = GitHub Actions` prerequisite for the `deploy-docs` job in `README.md` and as a comment above the `deploy-docs` job in `release.yml`, so downstream template users hit a documented setup step instead of a `Get Pages site failed` error on the first deploy.

### Added
- Tracking case study at `docs/case-studies/issue-52/` registering the `browser-commander` + Playwright preview-regeneration pattern from [`konard/vk-bot-desktop#52`](https://github.com/konard/vk-bot-desktop/pull/52), with an activation checklist for when an example-app surface lands in this template. Documentation only — no workflow, script, or runtime code changes. Primary upstream tracking issue: [`link-foundation/js-ai-driven-development-pipeline-template#62`](https://github.com/link-foundation/js-ai-driven-development-pipeline-template/issues/62).

### Fixed
- Release automation now keeps the workspace package entry in `Cargo.lock` synchronized when `scripts/version-and-commit.rs` bumps `Cargo.toml`, preventing stale lock-file version diffs in later pull requests.

### Fixed
- `scripts/publish-crate.rs` now treats crates.io HTTP 429 throttle responses ("You have published too many versions of this crate in the last 24 hours") as a deferred `publish_result=rate_limited` outcome (it writes the output, prints an explanatory banner and exits successfully) instead of a hard CI failure reported as a generic `failed` ("Failed to publish for unknown reason"). Authentication, already-published and unknown failures still exit non-zero. Failed-publish classification is consolidated through a single `classify_failure` function and `FailureKind` enum (with an `is_deferred` predicate), covered by unit tests runnable via `rust-script --test scripts/publish-crate.rs`.
- The release workflow (`.github/workflows/release.yml`) now gates crate-availability waiting, Docker Hub publishing and GitHub release creation on either an already-published crate or `publish_result=success`, so a deferred (rate-limited) crate upload no longer produces partial downstream release artifacts and the same version is retried automatically on the next push to `main`.

### Fixed
- Fixed reversed `cancel-in-progress` concurrency condition in `release.yml` that cancelled in-flight releases on `main` and never superseded older PR runs. The condition now uses `!=` so `main` releases run to completion while newer PR pushes cancel stale runs.

### Added
- Added a `scripts/check-crate-size.rs` guard that builds the `.crate` archive and fails the release before publishing when it exceeds the crates.io 10 MiB upload limit. The check runs in the build job and before publishing in both the auto-release and manual-release jobs.

### Changed
- Added a narrow `include` allowlist to `Cargo.toml` so docs, case studies, generated CI artifacts, changelog fragments, scripts, and experiments no longer inflate the published release archive.

### Fixed
- Prevented GitHub release creation from treating generic API validation failures as existing releases, and capped oversized release notes with a link to the full tagged changelog.

### Added
- Added the initial `meta-language` links-network core with self-description roots,
  verification, trivia attachment configuration, and a minimal CLI.
- Added the default lossless `parse` entry point, projection views, and a
  tested parity roadmap for competitor features, grammar embedding, and language
  coverage targets.
- Added exact reconstruction, mixed-region detection, query matching,
  substitution rules, concept reconstruction, object identity helpers,
  many-valued truth values, and executable parity fixtures for every tracked
  competitor target.
- Added executable language fixtures and tests for every requested Markdown,
  HTML, top-ten programming-language, and top-ten natural-language target.

### Fixed
- Added a CI guard that rejects Rust test modules and test attributes under `src/`, keeping tests in the `tests/` tree.

## Minor Changes

- Add immutable and mutable network snapshots with provenance and forward
  version commits for roadmap snapshot/versioning coverage.

### Added
- Added `txt` as a first-class markup/container target with a UTF-8 lossless
  fixture and content-sniffing fallback regions.

### Fixed
- Reconciled the natural-language target ordering with the Ethnologue 2025
  total-speaker order cited by the parity roadmap.

### Added
- Added a grammar-backed parser front end for Python, C, Java, C++, C#, JavaScript, and R using official tree-sitter grammar crates.

### Added
- Parse supported mixed-mode embedded regions into the host links network with grammar-backed syntax links.

### Added
- Added natural-language segmentation, identification, normalization, and bidi annotation links over lossless text parses.

### Added
- Added grammar-backed Visual Basic parsing with byte-exact reconstruction and recovery flag coverage.

bump: minor

- Add a grammar-backed `sql-ansi` SQL-family dialect fixture using
  `tree-sitter-sequel`.

### Added
- Added grammar-backed Delphi/Object Pascal parsing through `tree-sitter-pascal`.

### Added
- Enriched link queries with S-expression structural matching, captures, by-type construction, host predicate hooks, and link-cli-style variable substitution bindings.

### Added
- Added persistent snapshot structural sharing with interned metadata text storage.

### Added
- Added a query-transform surface for selecting captured links and replacing
  their source text while preserving unchanged bytes.

### Added
- Materialized self-description roots as controlled links with complete root-definition closure and round-trippable `describe` output.

### Added
- Added common concept ontology seeding from meta-expression's semantic lexicon, with shared concept links, syntax mappings, and structural programming-language concepts.

## [0.31.0] - 2026-06-07

### Added
- Changeset-style fragment format with frontmatter for specifying version bump type
- New `get-bump-type.mjs` script to automatically determine version bump from fragments
- Automatic version bumping on merge to main based on changelog fragments
- Detailed documentation for the changelog fragment system in `changelog.d/README.md`

### Changed
- Updated `collect-changelog.mjs` to strip frontmatter when collecting fragments
- Updated `version-and-commit.mjs` to handle frontmatter in fragments
- Enhanced release workflow to automatically determine bump type from changesets

### Changed
- Add `detect-changes` job with cross-platform `detect-code-changes.mjs` script
- Make lint job independent of changelog check (runs based on file changes only)
- Allow docs-only PRs without changelog fragment requirement
- Handle changelog check 'skipped' state in dependent jobs
- Exclude `changelog.d/`, `docs/`, `experiments/`, `examples/` folders and markdown files from code changes detection

### Fixed
- Fixed README.md to correctly reference Node.js scripts (`.mjs`) instead of Python scripts (`.py`)
- Updated project structure in README.md to match actual script files in `scripts/` directory
- Fixed example code in README.md that had invalid Rust with two `main` functions

### Added

- Added crates.io publishing support to CI/CD workflow
- Added `release_mode` input with "instant" and "changelog-pr" options for manual releases
- Added `--tag-prefix` and `--crates-io-url` options to create-github-release.mjs script
- Added comprehensive case study documentation for Issue #11 in docs/case-studies/issue-11/

### Changed

- Changed changelog fragment check from warning to error (exit 1) to enforce changelog requirements
- Updated job conditions with `always() && !cancelled()` to fix workflow_dispatch job skipping issue
- Renamed manual-release job to "Instant Release" for clarity

### Fixed

- Fixed deprecated `::set-output` GitHub Actions command in version-and-commit.mjs
- Fixed workflow_dispatch triggering issues where lint/build/release jobs were incorrectly skipped

### Fixed

- Fixed changelog fragment check to validate that a fragment is **added in the PR diff** rather than just checking if any fragments exist in the directory. This prevents the check from incorrectly passing when there are leftover fragments from previous PRs that haven't been released yet.

### Changed

- Converted shell scripts in `release.yml` to cross-platform `.mjs` scripts for improved portability and performance:
  - `check-changelog-fragment.mjs` - validates changelog fragment is added in PR diff
  - `git-config.mjs` - configures git user for CI/CD
  - `check-release-needed.mjs` - checks if release is needed
  - `publish-crate.mjs` - publishes package to crates.io
  - `create-changelog-fragment.mjs` - creates changelog fragments for manual releases
  - `get-version.mjs` - gets current version from Cargo.toml

### Added

- Added `check-version-modification.mjs` script to detect manual version changes in Cargo.toml
- Added `version-check` job to CI/CD workflow that runs on pull requests
- Added skip logic for automated release branches (changelog-manual-release-*, changeset-release/*, release/*, automated-release/*)

### Changed

- Version modifications in Cargo.toml are now blocked in pull requests to enforce automated release pipeline

### Added

- Added support for `CARGO_REGISTRY_TOKEN` as alternative to `CARGO_TOKEN` for crates.io publishing
- Added case study documentation for Issue #17 (yargs reserved word and dual token support)

### Changed

- Updated workflow to use fallback logic: `${{ secrets.CARGO_REGISTRY_TOKEN || secrets.CARGO_TOKEN }}`
- Improved publish-crate.mjs to check both `CARGO_REGISTRY_TOKEN` and `CARGO_TOKEN` environment variables
- Added warning message when neither token is set

### Added
- New `scripts/rust-paths.mjs` utility for automatic Rust package root detection
- Support for both single-language and multi-language repository structures in all CI/CD scripts
- Configuration options via `--rust-root` CLI argument and `RUST_ROOT` environment variable
- Comprehensive case study documentation in `docs/case-studies/issue-19/`

### Changed
- Updated all release scripts to use the new path detection utility:
  - `scripts/bump-version.mjs`
  - `scripts/check-release-needed.mjs`
  - `scripts/collect-changelog.mjs`
  - `scripts/get-bump-type.mjs`
  - `scripts/get-version.mjs`
  - `scripts/publish-crate.mjs`
  - `scripts/version-and-commit.mjs`

### Changed

- **check-release-needed.mjs**: Now checks crates.io API directly instead of git tags to determine if a version is already released. This prevents false positives where git tags exist but the package was never actually published to crates.io.

### Added

- **CI/CD Troubleshooting Guide**: New documentation at `docs/ci-cd/troubleshooting.md` covering common issues like skipped jobs, false positive version checks, publishing failures, and secret configuration.

- **Enhanced Error Handling in publish-crate.mjs**: Added specific detection and helpful error messages for authentication failures, including guidance on secret configuration and workflow setup.

- **Case Study Documentation**: Added comprehensive case study at `docs/case-studies/issue-21/` analyzing CI/CD failures from browser-commander repository (issues #27, #29, #31, #33) with timeline, root causes, and lessons learned.

### Fixed

- **Prevent False Positive Version Checks**: The release workflow now correctly identifies unpublished versions by checking crates.io instead of relying on git tags, which can exist without the package being published.

### Changed

- Translated all CI/CD scripts from JavaScript (.mjs) to Rust (.rs) using rust-script
- Scripts now use native Rust with rust-script for execution in shell
- Removed Node.js dependency from CI/CD pipeline
- Updated GitHub Actions workflow to use rust-script instead of node
- Updated README and CONTRIBUTING documentation with new script references

### Added

- Cache `restore-keys` for partial cache hits across all workflow jobs
- Explicit `token` parameter in checkout for release jobs
- Code coverage job with `cargo-llvm-cov` and Codecov integration
- Codecov badge in README.md
- Pre-release version support (e.g., `0.1.0-beta.1`) in version parsing
- `--release-label` parameter for multi-language release disambiguation
- `ensure_version_exceeds_published()` logic to prevent publishing duplicate versions
- `get_max_published_version()` to query highest non-yanked version from crates.io
- `max_published_version` output from check-release-needed for downstream use
- Version fallback logic in auto-release Create GitHub Release step

### Changed

- Updated `actions/checkout` from v4 to v6
- Updated `actions/cache` from v4 to v5
- Updated `peter-evans/create-pull-request` from v7 to v8
- Made `publish-crate.rs` fail (exit 1) when version already exists on crates.io
- Improved `create-github-release.rs` to check combined stdout+stderr and detect "Validation Failed"

### Fixed

- Fix publish steps overriding workflow-level CARGO_TOKEN fallback, breaking CARGO_REGISTRY_TOKEN-only configurations (#32)
- Fix non-fast-forward push failures in multi-workflow repos by adding fetch/rebase and push retry logic (#31)
- Add mono-repo path support to check-changelog-fragment.rs, check-version-modification.rs, and create-changelog-fragment.rs
- Add `!cancelled()` guard to test job condition to respect workflow cancellation

### Fixed

- Fixed `version-and-commit.rs` to check crates.io instead of git tags for determining if a version is already released
- This prevents the release pipeline from getting stuck when git tags exist without corresponding crates.io publication

### Added

- Added `--tag-prefix` support to `version-and-commit.rs` for multi-language repository compatibility
- Added crates.io and docs.rs badges to README.md
- Added automatic crates.io and docs.rs badge injection in GitHub release notes
- Added documentation deployment job to CI/CD pipeline (deploys to GitHub Pages after release)
- Added case study documentation for issue #25

### Fixed
- Fixed unsupported look-ahead regex in `create-github-release.rs` that caused a panic when parsing CHANGELOG.md. Replaced with a two-step approach using only features supported by Rust's `regex` crate.

### Changed
- Restructured example application as a simple CLI sum calculator using `lino-arguments`
- Renamed default package to `example-sum-package-name` with Unlicense license
- Reorganized test structure: `tests/unit/sum.rs`, `tests/integration/sum.rs`, `tests/unit/ci-cd/`
- Converted experiment scripts into proper unit tests in `tests/unit/ci-cd/changelog_parsing.rs`
- Added CI/CD skip logic for template default package name `example-sum-package-name`
- Updated README.md badges and documentation

### Fixed

- Change detection script now uses per-commit diff instead of full PR diff, so commits touching only non-code files correctly skip CI jobs even when earlier commits in the same PR changed code files

### Fixed
- Make release scripts resolve the publishable crate manifest when the repository root uses a Cargo workspace manifest.

### Fixed
- Decoupled GitHub Pages documentation deployment from package release publication and fixed release-script warning failures under `RUSTFLAGS=-Dwarnings`.

### Changed
- Added explicit GitHub Actions job timeouts and documented Rust test timeout guidance.

### Fixed
- Added a non-blocking warning threshold to the Rust file-size check so near-limit files are surfaced before concurrent PR merges can exceed the hard limit.

### Fixed
- Made `create-github-release.rs` build GitHub release titles as `[Language] X.Y.Z` instead of reusing the tag prefix.

### Added
- Added optional Docker Hub image publishing tied to Rust crate releases, including crates.io visibility waiting, version/latest image tags, and Docker Hub badges in GitHub release notes.

### Changed
- Release completeness checks now self-heal when crates.io exists but configured Docker Hub or GitHub release artifacts are missing.

### Fixed
- Switched documentation deployment to the official GitHub Pages artifact workflow so repositories using GitHub Actions as their Pages source do not get false-positive branch-push deploys.

### Added
- Documented the one-time `Settings → Pages → Source = GitHub Actions` prerequisite for the `deploy-docs` job in `README.md` and as a comment above the `deploy-docs` job in `release.yml`, so downstream template users hit a documented setup step instead of a `Get Pages site failed` error on the first deploy.

### Added
- Tracking case study at `docs/case-studies/issue-52/` registering the `browser-commander` + Playwright preview-regeneration pattern from [`konard/vk-bot-desktop#52`](https://github.com/konard/vk-bot-desktop/pull/52), with an activation checklist for when an example-app surface lands in this template. Documentation only — no workflow, script, or runtime code changes. Primary upstream tracking issue: [`link-foundation/js-ai-driven-development-pipeline-template#62`](https://github.com/link-foundation/js-ai-driven-development-pipeline-template/issues/62).

### Fixed
- Release automation now keeps the workspace package entry in `Cargo.lock` synchronized when `scripts/version-and-commit.rs` bumps `Cargo.toml`, preventing stale lock-file version diffs in later pull requests.

### Fixed
- `scripts/publish-crate.rs` now treats crates.io HTTP 429 throttle responses ("You have published too many versions of this crate in the last 24 hours") as a deferred `publish_result=rate_limited` outcome (it writes the output, prints an explanatory banner and exits successfully) instead of a hard CI failure reported as a generic `failed` ("Failed to publish for unknown reason"). Authentication, already-published and unknown failures still exit non-zero. Failed-publish classification is consolidated through a single `classify_failure` function and `FailureKind` enum (with an `is_deferred` predicate), covered by unit tests runnable via `rust-script --test scripts/publish-crate.rs`.
- The release workflow (`.github/workflows/release.yml`) now gates crate-availability waiting, Docker Hub publishing and GitHub release creation on either an already-published crate or `publish_result=success`, so a deferred (rate-limited) crate upload no longer produces partial downstream release artifacts and the same version is retried automatically on the next push to `main`.

### Fixed
- Fixed reversed `cancel-in-progress` concurrency condition in `release.yml` that cancelled in-flight releases on `main` and never superseded older PR runs. The condition now uses `!=` so `main` releases run to completion while newer PR pushes cancel stale runs.

### Added
- Added a `scripts/check-crate-size.rs` guard that builds the `.crate` archive and fails the release before publishing when it exceeds the crates.io 10 MiB upload limit. The check runs in the build job and before publishing in both the auto-release and manual-release jobs.

### Changed
- Added a narrow `include` allowlist to `Cargo.toml` so docs, case studies, generated CI artifacts, changelog fragments, scripts, and experiments no longer inflate the published release archive.

### Fixed
- Prevented GitHub release creation from treating generic API validation failures as existing releases, and capped oversized release notes with a link to the full tagged changelog.

### Added
- Added the initial `meta-language` links-network core with self-description roots,
  verification, trivia attachment configuration, and a minimal CLI.
- Added the default lossless `parse` entry point, projection views, and a
  tested parity roadmap for competitor features, grammar embedding, and language
  coverage targets.
- Added exact reconstruction, mixed-region detection, query matching,
  substitution rules, concept reconstruction, object identity helpers,
  many-valued truth values, and executable parity fixtures for every tracked
  competitor target.
- Added executable language fixtures and tests for every requested Markdown,
  HTML, top-ten programming-language, and top-ten natural-language target.

### Fixed
- Added a CI guard that rejects Rust test modules and test attributes under `src/`, keeping tests in the `tests/` tree.

## Minor Changes

- Add immutable and mutable network snapshots with provenance and forward
  version commits for roadmap snapshot/versioning coverage.

### Added
- Added `txt` as a first-class markup/container target with a UTF-8 lossless
  fixture and content-sniffing fallback regions.

### Fixed
- Reconciled the natural-language target ordering with the Ethnologue 2025
  total-speaker order cited by the parity roadmap.

### Added
- Added a grammar-backed parser front end for Python, C, Java, C++, C#, JavaScript, and R using official tree-sitter grammar crates.

### Added
- Parse supported mixed-mode embedded regions into the host links network with grammar-backed syntax links.

### Added
- Added natural-language segmentation, identification, normalization, and bidi annotation links over lossless text parses.

### Added
- Added grammar-backed Visual Basic parsing with byte-exact reconstruction and recovery flag coverage.

bump: minor

- Add a grammar-backed `sql-ansi` SQL-family dialect fixture using
  `tree-sitter-sequel`.

### Added
- Added grammar-backed Delphi/Object Pascal parsing through `tree-sitter-pascal`.

### Added
- Enriched link queries with S-expression structural matching, captures, by-type construction, host predicate hooks, and link-cli-style variable substitution bindings.

### Added
- Added persistent snapshot structural sharing with interned metadata text storage.

### Added
- Added a query-transform surface for selecting captured links and replacing
  their source text while preserving unchanged bytes.

### Added
- Materialized self-description roots as controlled links with complete root-definition closure and round-trippable `describe` output.

## [0.30.0] - 2026-06-07

### Added
- Changeset-style fragment format with frontmatter for specifying version bump type
- New `get-bump-type.mjs` script to automatically determine version bump from fragments
- Automatic version bumping on merge to main based on changelog fragments
- Detailed documentation for the changelog fragment system in `changelog.d/README.md`

### Changed
- Updated `collect-changelog.mjs` to strip frontmatter when collecting fragments
- Updated `version-and-commit.mjs` to handle frontmatter in fragments
- Enhanced release workflow to automatically determine bump type from changesets

### Changed
- Add `detect-changes` job with cross-platform `detect-code-changes.mjs` script
- Make lint job independent of changelog check (runs based on file changes only)
- Allow docs-only PRs without changelog fragment requirement
- Handle changelog check 'skipped' state in dependent jobs
- Exclude `changelog.d/`, `docs/`, `experiments/`, `examples/` folders and markdown files from code changes detection

### Fixed
- Fixed README.md to correctly reference Node.js scripts (`.mjs`) instead of Python scripts (`.py`)
- Updated project structure in README.md to match actual script files in `scripts/` directory
- Fixed example code in README.md that had invalid Rust with two `main` functions

### Added

- Added crates.io publishing support to CI/CD workflow
- Added `release_mode` input with "instant" and "changelog-pr" options for manual releases
- Added `--tag-prefix` and `--crates-io-url` options to create-github-release.mjs script
- Added comprehensive case study documentation for Issue #11 in docs/case-studies/issue-11/

### Changed

- Changed changelog fragment check from warning to error (exit 1) to enforce changelog requirements
- Updated job conditions with `always() && !cancelled()` to fix workflow_dispatch job skipping issue
- Renamed manual-release job to "Instant Release" for clarity

### Fixed

- Fixed deprecated `::set-output` GitHub Actions command in version-and-commit.mjs
- Fixed workflow_dispatch triggering issues where lint/build/release jobs were incorrectly skipped

### Fixed

- Fixed changelog fragment check to validate that a fragment is **added in the PR diff** rather than just checking if any fragments exist in the directory. This prevents the check from incorrectly passing when there are leftover fragments from previous PRs that haven't been released yet.

### Changed

- Converted shell scripts in `release.yml` to cross-platform `.mjs` scripts for improved portability and performance:
  - `check-changelog-fragment.mjs` - validates changelog fragment is added in PR diff
  - `git-config.mjs` - configures git user for CI/CD
  - `check-release-needed.mjs` - checks if release is needed
  - `publish-crate.mjs` - publishes package to crates.io
  - `create-changelog-fragment.mjs` - creates changelog fragments for manual releases
  - `get-version.mjs` - gets current version from Cargo.toml

### Added

- Added `check-version-modification.mjs` script to detect manual version changes in Cargo.toml
- Added `version-check` job to CI/CD workflow that runs on pull requests
- Added skip logic for automated release branches (changelog-manual-release-*, changeset-release/*, release/*, automated-release/*)

### Changed

- Version modifications in Cargo.toml are now blocked in pull requests to enforce automated release pipeline

### Added

- Added support for `CARGO_REGISTRY_TOKEN` as alternative to `CARGO_TOKEN` for crates.io publishing
- Added case study documentation for Issue #17 (yargs reserved word and dual token support)

### Changed

- Updated workflow to use fallback logic: `${{ secrets.CARGO_REGISTRY_TOKEN || secrets.CARGO_TOKEN }}`
- Improved publish-crate.mjs to check both `CARGO_REGISTRY_TOKEN` and `CARGO_TOKEN` environment variables
- Added warning message when neither token is set

### Added
- New `scripts/rust-paths.mjs` utility for automatic Rust package root detection
- Support for both single-language and multi-language repository structures in all CI/CD scripts
- Configuration options via `--rust-root` CLI argument and `RUST_ROOT` environment variable
- Comprehensive case study documentation in `docs/case-studies/issue-19/`

### Changed
- Updated all release scripts to use the new path detection utility:
  - `scripts/bump-version.mjs`
  - `scripts/check-release-needed.mjs`
  - `scripts/collect-changelog.mjs`
  - `scripts/get-bump-type.mjs`
  - `scripts/get-version.mjs`
  - `scripts/publish-crate.mjs`
  - `scripts/version-and-commit.mjs`

### Changed

- **check-release-needed.mjs**: Now checks crates.io API directly instead of git tags to determine if a version is already released. This prevents false positives where git tags exist but the package was never actually published to crates.io.

### Added

- **CI/CD Troubleshooting Guide**: New documentation at `docs/ci-cd/troubleshooting.md` covering common issues like skipped jobs, false positive version checks, publishing failures, and secret configuration.

- **Enhanced Error Handling in publish-crate.mjs**: Added specific detection and helpful error messages for authentication failures, including guidance on secret configuration and workflow setup.

- **Case Study Documentation**: Added comprehensive case study at `docs/case-studies/issue-21/` analyzing CI/CD failures from browser-commander repository (issues #27, #29, #31, #33) with timeline, root causes, and lessons learned.

### Fixed

- **Prevent False Positive Version Checks**: The release workflow now correctly identifies unpublished versions by checking crates.io instead of relying on git tags, which can exist without the package being published.

### Changed

- Translated all CI/CD scripts from JavaScript (.mjs) to Rust (.rs) using rust-script
- Scripts now use native Rust with rust-script for execution in shell
- Removed Node.js dependency from CI/CD pipeline
- Updated GitHub Actions workflow to use rust-script instead of node
- Updated README and CONTRIBUTING documentation with new script references

### Added

- Cache `restore-keys` for partial cache hits across all workflow jobs
- Explicit `token` parameter in checkout for release jobs
- Code coverage job with `cargo-llvm-cov` and Codecov integration
- Codecov badge in README.md
- Pre-release version support (e.g., `0.1.0-beta.1`) in version parsing
- `--release-label` parameter for multi-language release disambiguation
- `ensure_version_exceeds_published()` logic to prevent publishing duplicate versions
- `get_max_published_version()` to query highest non-yanked version from crates.io
- `max_published_version` output from check-release-needed for downstream use
- Version fallback logic in auto-release Create GitHub Release step

### Changed

- Updated `actions/checkout` from v4 to v6
- Updated `actions/cache` from v4 to v5
- Updated `peter-evans/create-pull-request` from v7 to v8
- Made `publish-crate.rs` fail (exit 1) when version already exists on crates.io
- Improved `create-github-release.rs` to check combined stdout+stderr and detect "Validation Failed"

### Fixed

- Fix publish steps overriding workflow-level CARGO_TOKEN fallback, breaking CARGO_REGISTRY_TOKEN-only configurations (#32)
- Fix non-fast-forward push failures in multi-workflow repos by adding fetch/rebase and push retry logic (#31)
- Add mono-repo path support to check-changelog-fragment.rs, check-version-modification.rs, and create-changelog-fragment.rs
- Add `!cancelled()` guard to test job condition to respect workflow cancellation

### Fixed

- Fixed `version-and-commit.rs` to check crates.io instead of git tags for determining if a version is already released
- This prevents the release pipeline from getting stuck when git tags exist without corresponding crates.io publication

### Added

- Added `--tag-prefix` support to `version-and-commit.rs` for multi-language repository compatibility
- Added crates.io and docs.rs badges to README.md
- Added automatic crates.io and docs.rs badge injection in GitHub release notes
- Added documentation deployment job to CI/CD pipeline (deploys to GitHub Pages after release)
- Added case study documentation for issue #25

### Fixed
- Fixed unsupported look-ahead regex in `create-github-release.rs` that caused a panic when parsing CHANGELOG.md. Replaced with a two-step approach using only features supported by Rust's `regex` crate.

### Changed
- Restructured example application as a simple CLI sum calculator using `lino-arguments`
- Renamed default package to `example-sum-package-name` with Unlicense license
- Reorganized test structure: `tests/unit/sum.rs`, `tests/integration/sum.rs`, `tests/unit/ci-cd/`
- Converted experiment scripts into proper unit tests in `tests/unit/ci-cd/changelog_parsing.rs`
- Added CI/CD skip logic for template default package name `example-sum-package-name`
- Updated README.md badges and documentation

### Fixed

- Change detection script now uses per-commit diff instead of full PR diff, so commits touching only non-code files correctly skip CI jobs even when earlier commits in the same PR changed code files

### Fixed
- Make release scripts resolve the publishable crate manifest when the repository root uses a Cargo workspace manifest.

### Fixed
- Decoupled GitHub Pages documentation deployment from package release publication and fixed release-script warning failures under `RUSTFLAGS=-Dwarnings`.

### Changed
- Added explicit GitHub Actions job timeouts and documented Rust test timeout guidance.

### Fixed
- Added a non-blocking warning threshold to the Rust file-size check so near-limit files are surfaced before concurrent PR merges can exceed the hard limit.

### Fixed
- Made `create-github-release.rs` build GitHub release titles as `[Language] X.Y.Z` instead of reusing the tag prefix.

### Added
- Added optional Docker Hub image publishing tied to Rust crate releases, including crates.io visibility waiting, version/latest image tags, and Docker Hub badges in GitHub release notes.

### Changed
- Release completeness checks now self-heal when crates.io exists but configured Docker Hub or GitHub release artifacts are missing.

### Fixed
- Switched documentation deployment to the official GitHub Pages artifact workflow so repositories using GitHub Actions as their Pages source do not get false-positive branch-push deploys.

### Added
- Documented the one-time `Settings → Pages → Source = GitHub Actions` prerequisite for the `deploy-docs` job in `README.md` and as a comment above the `deploy-docs` job in `release.yml`, so downstream template users hit a documented setup step instead of a `Get Pages site failed` error on the first deploy.

### Added
- Tracking case study at `docs/case-studies/issue-52/` registering the `browser-commander` + Playwright preview-regeneration pattern from [`konard/vk-bot-desktop#52`](https://github.com/konard/vk-bot-desktop/pull/52), with an activation checklist for when an example-app surface lands in this template. Documentation only — no workflow, script, or runtime code changes. Primary upstream tracking issue: [`link-foundation/js-ai-driven-development-pipeline-template#62`](https://github.com/link-foundation/js-ai-driven-development-pipeline-template/issues/62).

### Fixed
- Release automation now keeps the workspace package entry in `Cargo.lock` synchronized when `scripts/version-and-commit.rs` bumps `Cargo.toml`, preventing stale lock-file version diffs in later pull requests.

### Fixed
- `scripts/publish-crate.rs` now treats crates.io HTTP 429 throttle responses ("You have published too many versions of this crate in the last 24 hours") as a deferred `publish_result=rate_limited` outcome (it writes the output, prints an explanatory banner and exits successfully) instead of a hard CI failure reported as a generic `failed` ("Failed to publish for unknown reason"). Authentication, already-published and unknown failures still exit non-zero. Failed-publish classification is consolidated through a single `classify_failure` function and `FailureKind` enum (with an `is_deferred` predicate), covered by unit tests runnable via `rust-script --test scripts/publish-crate.rs`.
- The release workflow (`.github/workflows/release.yml`) now gates crate-availability waiting, Docker Hub publishing and GitHub release creation on either an already-published crate or `publish_result=success`, so a deferred (rate-limited) crate upload no longer produces partial downstream release artifacts and the same version is retried automatically on the next push to `main`.

### Fixed
- Fixed reversed `cancel-in-progress` concurrency condition in `release.yml` that cancelled in-flight releases on `main` and never superseded older PR runs. The condition now uses `!=` so `main` releases run to completion while newer PR pushes cancel stale runs.

### Added
- Added a `scripts/check-crate-size.rs` guard that builds the `.crate` archive and fails the release before publishing when it exceeds the crates.io 10 MiB upload limit. The check runs in the build job and before publishing in both the auto-release and manual-release jobs.

### Changed
- Added a narrow `include` allowlist to `Cargo.toml` so docs, case studies, generated CI artifacts, changelog fragments, scripts, and experiments no longer inflate the published release archive.

### Fixed
- Prevented GitHub release creation from treating generic API validation failures as existing releases, and capped oversized release notes with a link to the full tagged changelog.

### Added
- Added the initial `meta-language` links-network core with self-description roots,
  verification, trivia attachment configuration, and a minimal CLI.
- Added the default lossless `parse` entry point, projection views, and a
  tested parity roadmap for competitor features, grammar embedding, and language
  coverage targets.
- Added exact reconstruction, mixed-region detection, query matching,
  substitution rules, concept reconstruction, object identity helpers,
  many-valued truth values, and executable parity fixtures for every tracked
  competitor target.
- Added executable language fixtures and tests for every requested Markdown,
  HTML, top-ten programming-language, and top-ten natural-language target.

### Fixed
- Added a CI guard that rejects Rust test modules and test attributes under `src/`, keeping tests in the `tests/` tree.

## Minor Changes

- Add immutable and mutable network snapshots with provenance and forward
  version commits for roadmap snapshot/versioning coverage.

### Added
- Added `txt` as a first-class markup/container target with a UTF-8 lossless
  fixture and content-sniffing fallback regions.

### Fixed
- Reconciled the natural-language target ordering with the Ethnologue 2025
  total-speaker order cited by the parity roadmap.

### Added
- Added a grammar-backed parser front end for Python, C, Java, C++, C#, JavaScript, and R using official tree-sitter grammar crates.

### Added
- Parse supported mixed-mode embedded regions into the host links network with grammar-backed syntax links.

### Added
- Added natural-language segmentation, identification, normalization, and bidi annotation links over lossless text parses.

### Added
- Added grammar-backed Visual Basic parsing with byte-exact reconstruction and recovery flag coverage.

bump: minor

- Add a grammar-backed `sql-ansi` SQL-family dialect fixture using
  `tree-sitter-sequel`.

### Added
- Added grammar-backed Delphi/Object Pascal parsing through `tree-sitter-pascal`.

### Added
- Enriched link queries with S-expression structural matching, captures, by-type construction, host predicate hooks, and link-cli-style variable substitution bindings.

### Added
- Added persistent snapshot structural sharing with interned metadata text storage.

### Added
- Added a query-transform surface for selecting captured links and replacing
  their source text while preserving unchanged bytes.

## [0.29.0] - 2026-06-07

### Added
- Changeset-style fragment format with frontmatter for specifying version bump type
- New `get-bump-type.mjs` script to automatically determine version bump from fragments
- Automatic version bumping on merge to main based on changelog fragments
- Detailed documentation for the changelog fragment system in `changelog.d/README.md`

### Changed
- Updated `collect-changelog.mjs` to strip frontmatter when collecting fragments
- Updated `version-and-commit.mjs` to handle frontmatter in fragments
- Enhanced release workflow to automatically determine bump type from changesets

### Changed
- Add `detect-changes` job with cross-platform `detect-code-changes.mjs` script
- Make lint job independent of changelog check (runs based on file changes only)
- Allow docs-only PRs without changelog fragment requirement
- Handle changelog check 'skipped' state in dependent jobs
- Exclude `changelog.d/`, `docs/`, `experiments/`, `examples/` folders and markdown files from code changes detection

### Fixed
- Fixed README.md to correctly reference Node.js scripts (`.mjs`) instead of Python scripts (`.py`)
- Updated project structure in README.md to match actual script files in `scripts/` directory
- Fixed example code in README.md that had invalid Rust with two `main` functions

### Added

- Added crates.io publishing support to CI/CD workflow
- Added `release_mode` input with "instant" and "changelog-pr" options for manual releases
- Added `--tag-prefix` and `--crates-io-url` options to create-github-release.mjs script
- Added comprehensive case study documentation for Issue #11 in docs/case-studies/issue-11/

### Changed

- Changed changelog fragment check from warning to error (exit 1) to enforce changelog requirements
- Updated job conditions with `always() && !cancelled()` to fix workflow_dispatch job skipping issue
- Renamed manual-release job to "Instant Release" for clarity

### Fixed

- Fixed deprecated `::set-output` GitHub Actions command in version-and-commit.mjs
- Fixed workflow_dispatch triggering issues where lint/build/release jobs were incorrectly skipped

### Fixed

- Fixed changelog fragment check to validate that a fragment is **added in the PR diff** rather than just checking if any fragments exist in the directory. This prevents the check from incorrectly passing when there are leftover fragments from previous PRs that haven't been released yet.

### Changed

- Converted shell scripts in `release.yml` to cross-platform `.mjs` scripts for improved portability and performance:
  - `check-changelog-fragment.mjs` - validates changelog fragment is added in PR diff
  - `git-config.mjs` - configures git user for CI/CD
  - `check-release-needed.mjs` - checks if release is needed
  - `publish-crate.mjs` - publishes package to crates.io
  - `create-changelog-fragment.mjs` - creates changelog fragments for manual releases
  - `get-version.mjs` - gets current version from Cargo.toml

### Added

- Added `check-version-modification.mjs` script to detect manual version changes in Cargo.toml
- Added `version-check` job to CI/CD workflow that runs on pull requests
- Added skip logic for automated release branches (changelog-manual-release-*, changeset-release/*, release/*, automated-release/*)

### Changed

- Version modifications in Cargo.toml are now blocked in pull requests to enforce automated release pipeline

### Added

- Added support for `CARGO_REGISTRY_TOKEN` as alternative to `CARGO_TOKEN` for crates.io publishing
- Added case study documentation for Issue #17 (yargs reserved word and dual token support)

### Changed

- Updated workflow to use fallback logic: `${{ secrets.CARGO_REGISTRY_TOKEN || secrets.CARGO_TOKEN }}`
- Improved publish-crate.mjs to check both `CARGO_REGISTRY_TOKEN` and `CARGO_TOKEN` environment variables
- Added warning message when neither token is set

### Added
- New `scripts/rust-paths.mjs` utility for automatic Rust package root detection
- Support for both single-language and multi-language repository structures in all CI/CD scripts
- Configuration options via `--rust-root` CLI argument and `RUST_ROOT` environment variable
- Comprehensive case study documentation in `docs/case-studies/issue-19/`

### Changed
- Updated all release scripts to use the new path detection utility:
  - `scripts/bump-version.mjs`
  - `scripts/check-release-needed.mjs`
  - `scripts/collect-changelog.mjs`
  - `scripts/get-bump-type.mjs`
  - `scripts/get-version.mjs`
  - `scripts/publish-crate.mjs`
  - `scripts/version-and-commit.mjs`

### Changed

- **check-release-needed.mjs**: Now checks crates.io API directly instead of git tags to determine if a version is already released. This prevents false positives where git tags exist but the package was never actually published to crates.io.

### Added

- **CI/CD Troubleshooting Guide**: New documentation at `docs/ci-cd/troubleshooting.md` covering common issues like skipped jobs, false positive version checks, publishing failures, and secret configuration.

- **Enhanced Error Handling in publish-crate.mjs**: Added specific detection and helpful error messages for authentication failures, including guidance on secret configuration and workflow setup.

- **Case Study Documentation**: Added comprehensive case study at `docs/case-studies/issue-21/` analyzing CI/CD failures from browser-commander repository (issues #27, #29, #31, #33) with timeline, root causes, and lessons learned.

### Fixed

- **Prevent False Positive Version Checks**: The release workflow now correctly identifies unpublished versions by checking crates.io instead of relying on git tags, which can exist without the package being published.

### Changed

- Translated all CI/CD scripts from JavaScript (.mjs) to Rust (.rs) using rust-script
- Scripts now use native Rust with rust-script for execution in shell
- Removed Node.js dependency from CI/CD pipeline
- Updated GitHub Actions workflow to use rust-script instead of node
- Updated README and CONTRIBUTING documentation with new script references

### Added

- Cache `restore-keys` for partial cache hits across all workflow jobs
- Explicit `token` parameter in checkout for release jobs
- Code coverage job with `cargo-llvm-cov` and Codecov integration
- Codecov badge in README.md
- Pre-release version support (e.g., `0.1.0-beta.1`) in version parsing
- `--release-label` parameter for multi-language release disambiguation
- `ensure_version_exceeds_published()` logic to prevent publishing duplicate versions
- `get_max_published_version()` to query highest non-yanked version from crates.io
- `max_published_version` output from check-release-needed for downstream use
- Version fallback logic in auto-release Create GitHub Release step

### Changed

- Updated `actions/checkout` from v4 to v6
- Updated `actions/cache` from v4 to v5
- Updated `peter-evans/create-pull-request` from v7 to v8
- Made `publish-crate.rs` fail (exit 1) when version already exists on crates.io
- Improved `create-github-release.rs` to check combined stdout+stderr and detect "Validation Failed"

### Fixed

- Fix publish steps overriding workflow-level CARGO_TOKEN fallback, breaking CARGO_REGISTRY_TOKEN-only configurations (#32)
- Fix non-fast-forward push failures in multi-workflow repos by adding fetch/rebase and push retry logic (#31)
- Add mono-repo path support to check-changelog-fragment.rs, check-version-modification.rs, and create-changelog-fragment.rs
- Add `!cancelled()` guard to test job condition to respect workflow cancellation

### Fixed

- Fixed `version-and-commit.rs` to check crates.io instead of git tags for determining if a version is already released
- This prevents the release pipeline from getting stuck when git tags exist without corresponding crates.io publication

### Added

- Added `--tag-prefix` support to `version-and-commit.rs` for multi-language repository compatibility
- Added crates.io and docs.rs badges to README.md
- Added automatic crates.io and docs.rs badge injection in GitHub release notes
- Added documentation deployment job to CI/CD pipeline (deploys to GitHub Pages after release)
- Added case study documentation for issue #25

### Fixed
- Fixed unsupported look-ahead regex in `create-github-release.rs` that caused a panic when parsing CHANGELOG.md. Replaced with a two-step approach using only features supported by Rust's `regex` crate.

### Changed
- Restructured example application as a simple CLI sum calculator using `lino-arguments`
- Renamed default package to `example-sum-package-name` with Unlicense license
- Reorganized test structure: `tests/unit/sum.rs`, `tests/integration/sum.rs`, `tests/unit/ci-cd/`
- Converted experiment scripts into proper unit tests in `tests/unit/ci-cd/changelog_parsing.rs`
- Added CI/CD skip logic for template default package name `example-sum-package-name`
- Updated README.md badges and documentation

### Fixed

- Change detection script now uses per-commit diff instead of full PR diff, so commits touching only non-code files correctly skip CI jobs even when earlier commits in the same PR changed code files

### Fixed
- Make release scripts resolve the publishable crate manifest when the repository root uses a Cargo workspace manifest.

### Fixed
- Decoupled GitHub Pages documentation deployment from package release publication and fixed release-script warning failures under `RUSTFLAGS=-Dwarnings`.

### Changed
- Added explicit GitHub Actions job timeouts and documented Rust test timeout guidance.

### Fixed
- Added a non-blocking warning threshold to the Rust file-size check so near-limit files are surfaced before concurrent PR merges can exceed the hard limit.

### Fixed
- Made `create-github-release.rs` build GitHub release titles as `[Language] X.Y.Z` instead of reusing the tag prefix.

### Added
- Added optional Docker Hub image publishing tied to Rust crate releases, including crates.io visibility waiting, version/latest image tags, and Docker Hub badges in GitHub release notes.

### Changed
- Release completeness checks now self-heal when crates.io exists but configured Docker Hub or GitHub release artifacts are missing.

### Fixed
- Switched documentation deployment to the official GitHub Pages artifact workflow so repositories using GitHub Actions as their Pages source do not get false-positive branch-push deploys.

### Added
- Documented the one-time `Settings → Pages → Source = GitHub Actions` prerequisite for the `deploy-docs` job in `README.md` and as a comment above the `deploy-docs` job in `release.yml`, so downstream template users hit a documented setup step instead of a `Get Pages site failed` error on the first deploy.

### Added
- Tracking case study at `docs/case-studies/issue-52/` registering the `browser-commander` + Playwright preview-regeneration pattern from [`konard/vk-bot-desktop#52`](https://github.com/konard/vk-bot-desktop/pull/52), with an activation checklist for when an example-app surface lands in this template. Documentation only — no workflow, script, or runtime code changes. Primary upstream tracking issue: [`link-foundation/js-ai-driven-development-pipeline-template#62`](https://github.com/link-foundation/js-ai-driven-development-pipeline-template/issues/62).

### Fixed
- Release automation now keeps the workspace package entry in `Cargo.lock` synchronized when `scripts/version-and-commit.rs` bumps `Cargo.toml`, preventing stale lock-file version diffs in later pull requests.

### Fixed
- `scripts/publish-crate.rs` now treats crates.io HTTP 429 throttle responses ("You have published too many versions of this crate in the last 24 hours") as a deferred `publish_result=rate_limited` outcome (it writes the output, prints an explanatory banner and exits successfully) instead of a hard CI failure reported as a generic `failed` ("Failed to publish for unknown reason"). Authentication, already-published and unknown failures still exit non-zero. Failed-publish classification is consolidated through a single `classify_failure` function and `FailureKind` enum (with an `is_deferred` predicate), covered by unit tests runnable via `rust-script --test scripts/publish-crate.rs`.
- The release workflow (`.github/workflows/release.yml`) now gates crate-availability waiting, Docker Hub publishing and GitHub release creation on either an already-published crate or `publish_result=success`, so a deferred (rate-limited) crate upload no longer produces partial downstream release artifacts and the same version is retried automatically on the next push to `main`.

### Fixed
- Fixed reversed `cancel-in-progress` concurrency condition in `release.yml` that cancelled in-flight releases on `main` and never superseded older PR runs. The condition now uses `!=` so `main` releases run to completion while newer PR pushes cancel stale runs.

### Added
- Added a `scripts/check-crate-size.rs` guard that builds the `.crate` archive and fails the release before publishing when it exceeds the crates.io 10 MiB upload limit. The check runs in the build job and before publishing in both the auto-release and manual-release jobs.

### Changed
- Added a narrow `include` allowlist to `Cargo.toml` so docs, case studies, generated CI artifacts, changelog fragments, scripts, and experiments no longer inflate the published release archive.

### Fixed
- Prevented GitHub release creation from treating generic API validation failures as existing releases, and capped oversized release notes with a link to the full tagged changelog.

### Added
- Added the initial `meta-language` links-network core with self-description roots,
  verification, trivia attachment configuration, and a minimal CLI.
- Added the default lossless `parse` entry point, projection views, and a
  tested parity roadmap for competitor features, grammar embedding, and language
  coverage targets.
- Added exact reconstruction, mixed-region detection, query matching,
  substitution rules, concept reconstruction, object identity helpers,
  many-valued truth values, and executable parity fixtures for every tracked
  competitor target.
- Added executable language fixtures and tests for every requested Markdown,
  HTML, top-ten programming-language, and top-ten natural-language target.

### Fixed
- Added a CI guard that rejects Rust test modules and test attributes under `src/`, keeping tests in the `tests/` tree.

## Minor Changes

- Add immutable and mutable network snapshots with provenance and forward
  version commits for roadmap snapshot/versioning coverage.

### Added
- Added `txt` as a first-class markup/container target with a UTF-8 lossless
  fixture and content-sniffing fallback regions.

### Fixed
- Reconciled the natural-language target ordering with the Ethnologue 2025
  total-speaker order cited by the parity roadmap.

### Added
- Added a grammar-backed parser front end for Python, C, Java, C++, C#, JavaScript, and R using official tree-sitter grammar crates.

### Added
- Parse supported mixed-mode embedded regions into the host links network with grammar-backed syntax links.

### Added
- Added natural-language segmentation, identification, normalization, and bidi annotation links over lossless text parses.

### Added
- Added grammar-backed Visual Basic parsing with byte-exact reconstruction and recovery flag coverage.

bump: minor

- Add a grammar-backed `sql-ansi` SQL-family dialect fixture using
  `tree-sitter-sequel`.

### Added
- Added grammar-backed Delphi/Object Pascal parsing through `tree-sitter-pascal`.

### Added
- Enriched link queries with S-expression structural matching, captures, by-type construction, host predicate hooks, and link-cli-style variable substitution bindings.

### Added
- Added persistent snapshot structural sharing with interned metadata text storage.

## [0.28.0] - 2026-06-07

### Added
- Changeset-style fragment format with frontmatter for specifying version bump type
- New `get-bump-type.mjs` script to automatically determine version bump from fragments
- Automatic version bumping on merge to main based on changelog fragments
- Detailed documentation for the changelog fragment system in `changelog.d/README.md`

### Changed
- Updated `collect-changelog.mjs` to strip frontmatter when collecting fragments
- Updated `version-and-commit.mjs` to handle frontmatter in fragments
- Enhanced release workflow to automatically determine bump type from changesets

### Changed
- Add `detect-changes` job with cross-platform `detect-code-changes.mjs` script
- Make lint job independent of changelog check (runs based on file changes only)
- Allow docs-only PRs without changelog fragment requirement
- Handle changelog check 'skipped' state in dependent jobs
- Exclude `changelog.d/`, `docs/`, `experiments/`, `examples/` folders and markdown files from code changes detection

### Fixed
- Fixed README.md to correctly reference Node.js scripts (`.mjs`) instead of Python scripts (`.py`)
- Updated project structure in README.md to match actual script files in `scripts/` directory
- Fixed example code in README.md that had invalid Rust with two `main` functions

### Added

- Added crates.io publishing support to CI/CD workflow
- Added `release_mode` input with "instant" and "changelog-pr" options for manual releases
- Added `--tag-prefix` and `--crates-io-url` options to create-github-release.mjs script
- Added comprehensive case study documentation for Issue #11 in docs/case-studies/issue-11/

### Changed

- Changed changelog fragment check from warning to error (exit 1) to enforce changelog requirements
- Updated job conditions with `always() && !cancelled()` to fix workflow_dispatch job skipping issue
- Renamed manual-release job to "Instant Release" for clarity

### Fixed

- Fixed deprecated `::set-output` GitHub Actions command in version-and-commit.mjs
- Fixed workflow_dispatch triggering issues where lint/build/release jobs were incorrectly skipped

### Fixed

- Fixed changelog fragment check to validate that a fragment is **added in the PR diff** rather than just checking if any fragments exist in the directory. This prevents the check from incorrectly passing when there are leftover fragments from previous PRs that haven't been released yet.

### Changed

- Converted shell scripts in `release.yml` to cross-platform `.mjs` scripts for improved portability and performance:
  - `check-changelog-fragment.mjs` - validates changelog fragment is added in PR diff
  - `git-config.mjs` - configures git user for CI/CD
  - `check-release-needed.mjs` - checks if release is needed
  - `publish-crate.mjs` - publishes package to crates.io
  - `create-changelog-fragment.mjs` - creates changelog fragments for manual releases
  - `get-version.mjs` - gets current version from Cargo.toml

### Added

- Added `check-version-modification.mjs` script to detect manual version changes in Cargo.toml
- Added `version-check` job to CI/CD workflow that runs on pull requests
- Added skip logic for automated release branches (changelog-manual-release-*, changeset-release/*, release/*, automated-release/*)

### Changed

- Version modifications in Cargo.toml are now blocked in pull requests to enforce automated release pipeline

### Added

- Added support for `CARGO_REGISTRY_TOKEN` as alternative to `CARGO_TOKEN` for crates.io publishing
- Added case study documentation for Issue #17 (yargs reserved word and dual token support)

### Changed

- Updated workflow to use fallback logic: `${{ secrets.CARGO_REGISTRY_TOKEN || secrets.CARGO_TOKEN }}`
- Improved publish-crate.mjs to check both `CARGO_REGISTRY_TOKEN` and `CARGO_TOKEN` environment variables
- Added warning message when neither token is set

### Added
- New `scripts/rust-paths.mjs` utility for automatic Rust package root detection
- Support for both single-language and multi-language repository structures in all CI/CD scripts
- Configuration options via `--rust-root` CLI argument and `RUST_ROOT` environment variable
- Comprehensive case study documentation in `docs/case-studies/issue-19/`

### Changed
- Updated all release scripts to use the new path detection utility:
  - `scripts/bump-version.mjs`
  - `scripts/check-release-needed.mjs`
  - `scripts/collect-changelog.mjs`
  - `scripts/get-bump-type.mjs`
  - `scripts/get-version.mjs`
  - `scripts/publish-crate.mjs`
  - `scripts/version-and-commit.mjs`

### Changed

- **check-release-needed.mjs**: Now checks crates.io API directly instead of git tags to determine if a version is already released. This prevents false positives where git tags exist but the package was never actually published to crates.io.

### Added

- **CI/CD Troubleshooting Guide**: New documentation at `docs/ci-cd/troubleshooting.md` covering common issues like skipped jobs, false positive version checks, publishing failures, and secret configuration.

- **Enhanced Error Handling in publish-crate.mjs**: Added specific detection and helpful error messages for authentication failures, including guidance on secret configuration and workflow setup.

- **Case Study Documentation**: Added comprehensive case study at `docs/case-studies/issue-21/` analyzing CI/CD failures from browser-commander repository (issues #27, #29, #31, #33) with timeline, root causes, and lessons learned.

### Fixed

- **Prevent False Positive Version Checks**: The release workflow now correctly identifies unpublished versions by checking crates.io instead of relying on git tags, which can exist without the package being published.

### Changed

- Translated all CI/CD scripts from JavaScript (.mjs) to Rust (.rs) using rust-script
- Scripts now use native Rust with rust-script for execution in shell
- Removed Node.js dependency from CI/CD pipeline
- Updated GitHub Actions workflow to use rust-script instead of node
- Updated README and CONTRIBUTING documentation with new script references

### Added

- Cache `restore-keys` for partial cache hits across all workflow jobs
- Explicit `token` parameter in checkout for release jobs
- Code coverage job with `cargo-llvm-cov` and Codecov integration
- Codecov badge in README.md
- Pre-release version support (e.g., `0.1.0-beta.1`) in version parsing
- `--release-label` parameter for multi-language release disambiguation
- `ensure_version_exceeds_published()` logic to prevent publishing duplicate versions
- `get_max_published_version()` to query highest non-yanked version from crates.io
- `max_published_version` output from check-release-needed for downstream use
- Version fallback logic in auto-release Create GitHub Release step

### Changed

- Updated `actions/checkout` from v4 to v6
- Updated `actions/cache` from v4 to v5
- Updated `peter-evans/create-pull-request` from v7 to v8
- Made `publish-crate.rs` fail (exit 1) when version already exists on crates.io
- Improved `create-github-release.rs` to check combined stdout+stderr and detect "Validation Failed"

### Fixed

- Fix publish steps overriding workflow-level CARGO_TOKEN fallback, breaking CARGO_REGISTRY_TOKEN-only configurations (#32)
- Fix non-fast-forward push failures in multi-workflow repos by adding fetch/rebase and push retry logic (#31)
- Add mono-repo path support to check-changelog-fragment.rs, check-version-modification.rs, and create-changelog-fragment.rs
- Add `!cancelled()` guard to test job condition to respect workflow cancellation

### Fixed

- Fixed `version-and-commit.rs` to check crates.io instead of git tags for determining if a version is already released
- This prevents the release pipeline from getting stuck when git tags exist without corresponding crates.io publication

### Added

- Added `--tag-prefix` support to `version-and-commit.rs` for multi-language repository compatibility
- Added crates.io and docs.rs badges to README.md
- Added automatic crates.io and docs.rs badge injection in GitHub release notes
- Added documentation deployment job to CI/CD pipeline (deploys to GitHub Pages after release)
- Added case study documentation for issue #25

### Fixed
- Fixed unsupported look-ahead regex in `create-github-release.rs` that caused a panic when parsing CHANGELOG.md. Replaced with a two-step approach using only features supported by Rust's `regex` crate.

### Changed
- Restructured example application as a simple CLI sum calculator using `lino-arguments`
- Renamed default package to `example-sum-package-name` with Unlicense license
- Reorganized test structure: `tests/unit/sum.rs`, `tests/integration/sum.rs`, `tests/unit/ci-cd/`
- Converted experiment scripts into proper unit tests in `tests/unit/ci-cd/changelog_parsing.rs`
- Added CI/CD skip logic for template default package name `example-sum-package-name`
- Updated README.md badges and documentation

### Fixed

- Change detection script now uses per-commit diff instead of full PR diff, so commits touching only non-code files correctly skip CI jobs even when earlier commits in the same PR changed code files

### Fixed
- Make release scripts resolve the publishable crate manifest when the repository root uses a Cargo workspace manifest.

### Fixed
- Decoupled GitHub Pages documentation deployment from package release publication and fixed release-script warning failures under `RUSTFLAGS=-Dwarnings`.

### Changed
- Added explicit GitHub Actions job timeouts and documented Rust test timeout guidance.

### Fixed
- Added a non-blocking warning threshold to the Rust file-size check so near-limit files are surfaced before concurrent PR merges can exceed the hard limit.

### Fixed
- Made `create-github-release.rs` build GitHub release titles as `[Language] X.Y.Z` instead of reusing the tag prefix.

### Added
- Added optional Docker Hub image publishing tied to Rust crate releases, including crates.io visibility waiting, version/latest image tags, and Docker Hub badges in GitHub release notes.

### Changed
- Release completeness checks now self-heal when crates.io exists but configured Docker Hub or GitHub release artifacts are missing.

### Fixed
- Switched documentation deployment to the official GitHub Pages artifact workflow so repositories using GitHub Actions as their Pages source do not get false-positive branch-push deploys.

### Added
- Documented the one-time `Settings → Pages → Source = GitHub Actions` prerequisite for the `deploy-docs` job in `README.md` and as a comment above the `deploy-docs` job in `release.yml`, so downstream template users hit a documented setup step instead of a `Get Pages site failed` error on the first deploy.

### Added
- Tracking case study at `docs/case-studies/issue-52/` registering the `browser-commander` + Playwright preview-regeneration pattern from [`konard/vk-bot-desktop#52`](https://github.com/konard/vk-bot-desktop/pull/52), with an activation checklist for when an example-app surface lands in this template. Documentation only — no workflow, script, or runtime code changes. Primary upstream tracking issue: [`link-foundation/js-ai-driven-development-pipeline-template#62`](https://github.com/link-foundation/js-ai-driven-development-pipeline-template/issues/62).

### Fixed
- Release automation now keeps the workspace package entry in `Cargo.lock` synchronized when `scripts/version-and-commit.rs` bumps `Cargo.toml`, preventing stale lock-file version diffs in later pull requests.

### Fixed
- `scripts/publish-crate.rs` now treats crates.io HTTP 429 throttle responses ("You have published too many versions of this crate in the last 24 hours") as a deferred `publish_result=rate_limited` outcome (it writes the output, prints an explanatory banner and exits successfully) instead of a hard CI failure reported as a generic `failed` ("Failed to publish for unknown reason"). Authentication, already-published and unknown failures still exit non-zero. Failed-publish classification is consolidated through a single `classify_failure` function and `FailureKind` enum (with an `is_deferred` predicate), covered by unit tests runnable via `rust-script --test scripts/publish-crate.rs`.
- The release workflow (`.github/workflows/release.yml`) now gates crate-availability waiting, Docker Hub publishing and GitHub release creation on either an already-published crate or `publish_result=success`, so a deferred (rate-limited) crate upload no longer produces partial downstream release artifacts and the same version is retried automatically on the next push to `main`.

### Fixed
- Fixed reversed `cancel-in-progress` concurrency condition in `release.yml` that cancelled in-flight releases on `main` and never superseded older PR runs. The condition now uses `!=` so `main` releases run to completion while newer PR pushes cancel stale runs.

### Added
- Added a `scripts/check-crate-size.rs` guard that builds the `.crate` archive and fails the release before publishing when it exceeds the crates.io 10 MiB upload limit. The check runs in the build job and before publishing in both the auto-release and manual-release jobs.

### Changed
- Added a narrow `include` allowlist to `Cargo.toml` so docs, case studies, generated CI artifacts, changelog fragments, scripts, and experiments no longer inflate the published release archive.

### Fixed
- Prevented GitHub release creation from treating generic API validation failures as existing releases, and capped oversized release notes with a link to the full tagged changelog.

### Added
- Added the initial `meta-language` links-network core with self-description roots,
  verification, trivia attachment configuration, and a minimal CLI.
- Added the default lossless `parse` entry point, projection views, and a
  tested parity roadmap for competitor features, grammar embedding, and language
  coverage targets.
- Added exact reconstruction, mixed-region detection, query matching,
  substitution rules, concept reconstruction, object identity helpers,
  many-valued truth values, and executable parity fixtures for every tracked
  competitor target.
- Added executable language fixtures and tests for every requested Markdown,
  HTML, top-ten programming-language, and top-ten natural-language target.

### Fixed
- Added a CI guard that rejects Rust test modules and test attributes under `src/`, keeping tests in the `tests/` tree.

## Minor Changes

- Add immutable and mutable network snapshots with provenance and forward
  version commits for roadmap snapshot/versioning coverage.

### Added
- Added `txt` as a first-class markup/container target with a UTF-8 lossless
  fixture and content-sniffing fallback regions.

### Fixed
- Reconciled the natural-language target ordering with the Ethnologue 2025
  total-speaker order cited by the parity roadmap.

### Added
- Added a grammar-backed parser front end for Python, C, Java, C++, C#, JavaScript, and R using official tree-sitter grammar crates.

### Added
- Parse supported mixed-mode embedded regions into the host links network with grammar-backed syntax links.

### Added
- Added natural-language segmentation, identification, normalization, and bidi annotation links over lossless text parses.

### Added
- Added grammar-backed Visual Basic parsing with byte-exact reconstruction and recovery flag coverage.

bump: minor

- Add a grammar-backed `sql-ansi` SQL-family dialect fixture using
  `tree-sitter-sequel`.

### Added
- Added grammar-backed Delphi/Object Pascal parsing through `tree-sitter-pascal`.

### Added
- Enriched link queries with S-expression structural matching, captures, by-type construction, host predicate hooks, and link-cli-style variable substitution bindings.

## [0.27.0] - 2026-06-07

### Added
- Changeset-style fragment format with frontmatter for specifying version bump type
- New `get-bump-type.mjs` script to automatically determine version bump from fragments
- Automatic version bumping on merge to main based on changelog fragments
- Detailed documentation for the changelog fragment system in `changelog.d/README.md`

### Changed
- Updated `collect-changelog.mjs` to strip frontmatter when collecting fragments
- Updated `version-and-commit.mjs` to handle frontmatter in fragments
- Enhanced release workflow to automatically determine bump type from changesets

### Changed
- Add `detect-changes` job with cross-platform `detect-code-changes.mjs` script
- Make lint job independent of changelog check (runs based on file changes only)
- Allow docs-only PRs without changelog fragment requirement
- Handle changelog check 'skipped' state in dependent jobs
- Exclude `changelog.d/`, `docs/`, `experiments/`, `examples/` folders and markdown files from code changes detection

### Fixed
- Fixed README.md to correctly reference Node.js scripts (`.mjs`) instead of Python scripts (`.py`)
- Updated project structure in README.md to match actual script files in `scripts/` directory
- Fixed example code in README.md that had invalid Rust with two `main` functions

### Added

- Added crates.io publishing support to CI/CD workflow
- Added `release_mode` input with "instant" and "changelog-pr" options for manual releases
- Added `--tag-prefix` and `--crates-io-url` options to create-github-release.mjs script
- Added comprehensive case study documentation for Issue #11 in docs/case-studies/issue-11/

### Changed

- Changed changelog fragment check from warning to error (exit 1) to enforce changelog requirements
- Updated job conditions with `always() && !cancelled()` to fix workflow_dispatch job skipping issue
- Renamed manual-release job to "Instant Release" for clarity

### Fixed

- Fixed deprecated `::set-output` GitHub Actions command in version-and-commit.mjs
- Fixed workflow_dispatch triggering issues where lint/build/release jobs were incorrectly skipped

### Fixed

- Fixed changelog fragment check to validate that a fragment is **added in the PR diff** rather than just checking if any fragments exist in the directory. This prevents the check from incorrectly passing when there are leftover fragments from previous PRs that haven't been released yet.

### Changed

- Converted shell scripts in `release.yml` to cross-platform `.mjs` scripts for improved portability and performance:
  - `check-changelog-fragment.mjs` - validates changelog fragment is added in PR diff
  - `git-config.mjs` - configures git user for CI/CD
  - `check-release-needed.mjs` - checks if release is needed
  - `publish-crate.mjs` - publishes package to crates.io
  - `create-changelog-fragment.mjs` - creates changelog fragments for manual releases
  - `get-version.mjs` - gets current version from Cargo.toml

### Added

- Added `check-version-modification.mjs` script to detect manual version changes in Cargo.toml
- Added `version-check` job to CI/CD workflow that runs on pull requests
- Added skip logic for automated release branches (changelog-manual-release-*, changeset-release/*, release/*, automated-release/*)

### Changed

- Version modifications in Cargo.toml are now blocked in pull requests to enforce automated release pipeline

### Added

- Added support for `CARGO_REGISTRY_TOKEN` as alternative to `CARGO_TOKEN` for crates.io publishing
- Added case study documentation for Issue #17 (yargs reserved word and dual token support)

### Changed

- Updated workflow to use fallback logic: `${{ secrets.CARGO_REGISTRY_TOKEN || secrets.CARGO_TOKEN }}`
- Improved publish-crate.mjs to check both `CARGO_REGISTRY_TOKEN` and `CARGO_TOKEN` environment variables
- Added warning message when neither token is set

### Added
- New `scripts/rust-paths.mjs` utility for automatic Rust package root detection
- Support for both single-language and multi-language repository structures in all CI/CD scripts
- Configuration options via `--rust-root` CLI argument and `RUST_ROOT` environment variable
- Comprehensive case study documentation in `docs/case-studies/issue-19/`

### Changed
- Updated all release scripts to use the new path detection utility:
  - `scripts/bump-version.mjs`
  - `scripts/check-release-needed.mjs`
  - `scripts/collect-changelog.mjs`
  - `scripts/get-bump-type.mjs`
  - `scripts/get-version.mjs`
  - `scripts/publish-crate.mjs`
  - `scripts/version-and-commit.mjs`

### Changed

- **check-release-needed.mjs**: Now checks crates.io API directly instead of git tags to determine if a version is already released. This prevents false positives where git tags exist but the package was never actually published to crates.io.

### Added

- **CI/CD Troubleshooting Guide**: New documentation at `docs/ci-cd/troubleshooting.md` covering common issues like skipped jobs, false positive version checks, publishing failures, and secret configuration.

- **Enhanced Error Handling in publish-crate.mjs**: Added specific detection and helpful error messages for authentication failures, including guidance on secret configuration and workflow setup.

- **Case Study Documentation**: Added comprehensive case study at `docs/case-studies/issue-21/` analyzing CI/CD failures from browser-commander repository (issues #27, #29, #31, #33) with timeline, root causes, and lessons learned.

### Fixed

- **Prevent False Positive Version Checks**: The release workflow now correctly identifies unpublished versions by checking crates.io instead of relying on git tags, which can exist without the package being published.

### Changed

- Translated all CI/CD scripts from JavaScript (.mjs) to Rust (.rs) using rust-script
- Scripts now use native Rust with rust-script for execution in shell
- Removed Node.js dependency from CI/CD pipeline
- Updated GitHub Actions workflow to use rust-script instead of node
- Updated README and CONTRIBUTING documentation with new script references

### Added

- Cache `restore-keys` for partial cache hits across all workflow jobs
- Explicit `token` parameter in checkout for release jobs
- Code coverage job with `cargo-llvm-cov` and Codecov integration
- Codecov badge in README.md
- Pre-release version support (e.g., `0.1.0-beta.1`) in version parsing
- `--release-label` parameter for multi-language release disambiguation
- `ensure_version_exceeds_published()` logic to prevent publishing duplicate versions
- `get_max_published_version()` to query highest non-yanked version from crates.io
- `max_published_version` output from check-release-needed for downstream use
- Version fallback logic in auto-release Create GitHub Release step

### Changed

- Updated `actions/checkout` from v4 to v6
- Updated `actions/cache` from v4 to v5
- Updated `peter-evans/create-pull-request` from v7 to v8
- Made `publish-crate.rs` fail (exit 1) when version already exists on crates.io
- Improved `create-github-release.rs` to check combined stdout+stderr and detect "Validation Failed"

### Fixed

- Fix publish steps overriding workflow-level CARGO_TOKEN fallback, breaking CARGO_REGISTRY_TOKEN-only configurations (#32)
- Fix non-fast-forward push failures in multi-workflow repos by adding fetch/rebase and push retry logic (#31)
- Add mono-repo path support to check-changelog-fragment.rs, check-version-modification.rs, and create-changelog-fragment.rs
- Add `!cancelled()` guard to test job condition to respect workflow cancellation

### Fixed

- Fixed `version-and-commit.rs` to check crates.io instead of git tags for determining if a version is already released
- This prevents the release pipeline from getting stuck when git tags exist without corresponding crates.io publication

### Added

- Added `--tag-prefix` support to `version-and-commit.rs` for multi-language repository compatibility
- Added crates.io and docs.rs badges to README.md
- Added automatic crates.io and docs.rs badge injection in GitHub release notes
- Added documentation deployment job to CI/CD pipeline (deploys to GitHub Pages after release)
- Added case study documentation for issue #25

### Fixed
- Fixed unsupported look-ahead regex in `create-github-release.rs` that caused a panic when parsing CHANGELOG.md. Replaced with a two-step approach using only features supported by Rust's `regex` crate.

### Changed
- Restructured example application as a simple CLI sum calculator using `lino-arguments`
- Renamed default package to `example-sum-package-name` with Unlicense license
- Reorganized test structure: `tests/unit/sum.rs`, `tests/integration/sum.rs`, `tests/unit/ci-cd/`
- Converted experiment scripts into proper unit tests in `tests/unit/ci-cd/changelog_parsing.rs`
- Added CI/CD skip logic for template default package name `example-sum-package-name`
- Updated README.md badges and documentation

### Fixed

- Change detection script now uses per-commit diff instead of full PR diff, so commits touching only non-code files correctly skip CI jobs even when earlier commits in the same PR changed code files

### Fixed
- Make release scripts resolve the publishable crate manifest when the repository root uses a Cargo workspace manifest.

### Fixed
- Decoupled GitHub Pages documentation deployment from package release publication and fixed release-script warning failures under `RUSTFLAGS=-Dwarnings`.

### Changed
- Added explicit GitHub Actions job timeouts and documented Rust test timeout guidance.

### Fixed
- Added a non-blocking warning threshold to the Rust file-size check so near-limit files are surfaced before concurrent PR merges can exceed the hard limit.

### Fixed
- Made `create-github-release.rs` build GitHub release titles as `[Language] X.Y.Z` instead of reusing the tag prefix.

### Added
- Added optional Docker Hub image publishing tied to Rust crate releases, including crates.io visibility waiting, version/latest image tags, and Docker Hub badges in GitHub release notes.

### Changed
- Release completeness checks now self-heal when crates.io exists but configured Docker Hub or GitHub release artifacts are missing.

### Fixed
- Switched documentation deployment to the official GitHub Pages artifact workflow so repositories using GitHub Actions as their Pages source do not get false-positive branch-push deploys.

### Added
- Documented the one-time `Settings → Pages → Source = GitHub Actions` prerequisite for the `deploy-docs` job in `README.md` and as a comment above the `deploy-docs` job in `release.yml`, so downstream template users hit a documented setup step instead of a `Get Pages site failed` error on the first deploy.

### Added
- Tracking case study at `docs/case-studies/issue-52/` registering the `browser-commander` + Playwright preview-regeneration pattern from [`konard/vk-bot-desktop#52`](https://github.com/konard/vk-bot-desktop/pull/52), with an activation checklist for when an example-app surface lands in this template. Documentation only — no workflow, script, or runtime code changes. Primary upstream tracking issue: [`link-foundation/js-ai-driven-development-pipeline-template#62`](https://github.com/link-foundation/js-ai-driven-development-pipeline-template/issues/62).

### Fixed
- Release automation now keeps the workspace package entry in `Cargo.lock` synchronized when `scripts/version-and-commit.rs` bumps `Cargo.toml`, preventing stale lock-file version diffs in later pull requests.

### Fixed
- `scripts/publish-crate.rs` now treats crates.io HTTP 429 throttle responses ("You have published too many versions of this crate in the last 24 hours") as a deferred `publish_result=rate_limited` outcome (it writes the output, prints an explanatory banner and exits successfully) instead of a hard CI failure reported as a generic `failed` ("Failed to publish for unknown reason"). Authentication, already-published and unknown failures still exit non-zero. Failed-publish classification is consolidated through a single `classify_failure` function and `FailureKind` enum (with an `is_deferred` predicate), covered by unit tests runnable via `rust-script --test scripts/publish-crate.rs`.
- The release workflow (`.github/workflows/release.yml`) now gates crate-availability waiting, Docker Hub publishing and GitHub release creation on either an already-published crate or `publish_result=success`, so a deferred (rate-limited) crate upload no longer produces partial downstream release artifacts and the same version is retried automatically on the next push to `main`.

### Fixed
- Fixed reversed `cancel-in-progress` concurrency condition in `release.yml` that cancelled in-flight releases on `main` and never superseded older PR runs. The condition now uses `!=` so `main` releases run to completion while newer PR pushes cancel stale runs.

### Added
- Added a `scripts/check-crate-size.rs` guard that builds the `.crate` archive and fails the release before publishing when it exceeds the crates.io 10 MiB upload limit. The check runs in the build job and before publishing in both the auto-release and manual-release jobs.

### Changed
- Added a narrow `include` allowlist to `Cargo.toml` so docs, case studies, generated CI artifacts, changelog fragments, scripts, and experiments no longer inflate the published release archive.

### Fixed
- Prevented GitHub release creation from treating generic API validation failures as existing releases, and capped oversized release notes with a link to the full tagged changelog.

### Added
- Added the initial `meta-language` links-network core with self-description roots,
  verification, trivia attachment configuration, and a minimal CLI.
- Added the default lossless `parse` entry point, projection views, and a
  tested parity roadmap for competitor features, grammar embedding, and language
  coverage targets.
- Added exact reconstruction, mixed-region detection, query matching,
  substitution rules, concept reconstruction, object identity helpers,
  many-valued truth values, and executable parity fixtures for every tracked
  competitor target.
- Added executable language fixtures and tests for every requested Markdown,
  HTML, top-ten programming-language, and top-ten natural-language target.

### Fixed
- Added a CI guard that rejects Rust test modules and test attributes under `src/`, keeping tests in the `tests/` tree.

## Minor Changes

- Add immutable and mutable network snapshots with provenance and forward
  version commits for roadmap snapshot/versioning coverage.

### Added
- Added `txt` as a first-class markup/container target with a UTF-8 lossless
  fixture and content-sniffing fallback regions.

### Fixed
- Reconciled the natural-language target ordering with the Ethnologue 2025
  total-speaker order cited by the parity roadmap.

### Added
- Added a grammar-backed parser front end for Python, C, Java, C++, C#, JavaScript, and R using official tree-sitter grammar crates.

### Added
- Parse supported mixed-mode embedded regions into the host links network with grammar-backed syntax links.

### Added
- Added natural-language segmentation, identification, normalization, and bidi annotation links over lossless text parses.

### Added
- Added grammar-backed Visual Basic parsing with byte-exact reconstruction and recovery flag coverage.

bump: minor

- Add a grammar-backed `sql-ansi` SQL-family dialect fixture using
  `tree-sitter-sequel`.

### Added
- Added grammar-backed Delphi/Object Pascal parsing through `tree-sitter-pascal`.

## [0.26.0] - 2026-06-07

### Added
- Changeset-style fragment format with frontmatter for specifying version bump type
- New `get-bump-type.mjs` script to automatically determine version bump from fragments
- Automatic version bumping on merge to main based on changelog fragments
- Detailed documentation for the changelog fragment system in `changelog.d/README.md`

### Changed
- Updated `collect-changelog.mjs` to strip frontmatter when collecting fragments
- Updated `version-and-commit.mjs` to handle frontmatter in fragments
- Enhanced release workflow to automatically determine bump type from changesets

### Changed
- Add `detect-changes` job with cross-platform `detect-code-changes.mjs` script
- Make lint job independent of changelog check (runs based on file changes only)
- Allow docs-only PRs without changelog fragment requirement
- Handle changelog check 'skipped' state in dependent jobs
- Exclude `changelog.d/`, `docs/`, `experiments/`, `examples/` folders and markdown files from code changes detection

### Fixed
- Fixed README.md to correctly reference Node.js scripts (`.mjs`) instead of Python scripts (`.py`)
- Updated project structure in README.md to match actual script files in `scripts/` directory
- Fixed example code in README.md that had invalid Rust with two `main` functions

### Added

- Added crates.io publishing support to CI/CD workflow
- Added `release_mode` input with "instant" and "changelog-pr" options for manual releases
- Added `--tag-prefix` and `--crates-io-url` options to create-github-release.mjs script
- Added comprehensive case study documentation for Issue #11 in docs/case-studies/issue-11/

### Changed

- Changed changelog fragment check from warning to error (exit 1) to enforce changelog requirements
- Updated job conditions with `always() && !cancelled()` to fix workflow_dispatch job skipping issue
- Renamed manual-release job to "Instant Release" for clarity

### Fixed

- Fixed deprecated `::set-output` GitHub Actions command in version-and-commit.mjs
- Fixed workflow_dispatch triggering issues where lint/build/release jobs were incorrectly skipped

### Fixed

- Fixed changelog fragment check to validate that a fragment is **added in the PR diff** rather than just checking if any fragments exist in the directory. This prevents the check from incorrectly passing when there are leftover fragments from previous PRs that haven't been released yet.

### Changed

- Converted shell scripts in `release.yml` to cross-platform `.mjs` scripts for improved portability and performance:
  - `check-changelog-fragment.mjs` - validates changelog fragment is added in PR diff
  - `git-config.mjs` - configures git user for CI/CD
  - `check-release-needed.mjs` - checks if release is needed
  - `publish-crate.mjs` - publishes package to crates.io
  - `create-changelog-fragment.mjs` - creates changelog fragments for manual releases
  - `get-version.mjs` - gets current version from Cargo.toml

### Added

- Added `check-version-modification.mjs` script to detect manual version changes in Cargo.toml
- Added `version-check` job to CI/CD workflow that runs on pull requests
- Added skip logic for automated release branches (changelog-manual-release-*, changeset-release/*, release/*, automated-release/*)

### Changed

- Version modifications in Cargo.toml are now blocked in pull requests to enforce automated release pipeline

### Added

- Added support for `CARGO_REGISTRY_TOKEN` as alternative to `CARGO_TOKEN` for crates.io publishing
- Added case study documentation for Issue #17 (yargs reserved word and dual token support)

### Changed

- Updated workflow to use fallback logic: `${{ secrets.CARGO_REGISTRY_TOKEN || secrets.CARGO_TOKEN }}`
- Improved publish-crate.mjs to check both `CARGO_REGISTRY_TOKEN` and `CARGO_TOKEN` environment variables
- Added warning message when neither token is set

### Added
- New `scripts/rust-paths.mjs` utility for automatic Rust package root detection
- Support for both single-language and multi-language repository structures in all CI/CD scripts
- Configuration options via `--rust-root` CLI argument and `RUST_ROOT` environment variable
- Comprehensive case study documentation in `docs/case-studies/issue-19/`

### Changed
- Updated all release scripts to use the new path detection utility:
  - `scripts/bump-version.mjs`
  - `scripts/check-release-needed.mjs`
  - `scripts/collect-changelog.mjs`
  - `scripts/get-bump-type.mjs`
  - `scripts/get-version.mjs`
  - `scripts/publish-crate.mjs`
  - `scripts/version-and-commit.mjs`

### Changed

- **check-release-needed.mjs**: Now checks crates.io API directly instead of git tags to determine if a version is already released. This prevents false positives where git tags exist but the package was never actually published to crates.io.

### Added

- **CI/CD Troubleshooting Guide**: New documentation at `docs/ci-cd/troubleshooting.md` covering common issues like skipped jobs, false positive version checks, publishing failures, and secret configuration.

- **Enhanced Error Handling in publish-crate.mjs**: Added specific detection and helpful error messages for authentication failures, including guidance on secret configuration and workflow setup.

- **Case Study Documentation**: Added comprehensive case study at `docs/case-studies/issue-21/` analyzing CI/CD failures from browser-commander repository (issues #27, #29, #31, #33) with timeline, root causes, and lessons learned.

### Fixed

- **Prevent False Positive Version Checks**: The release workflow now correctly identifies unpublished versions by checking crates.io instead of relying on git tags, which can exist without the package being published.

### Changed

- Translated all CI/CD scripts from JavaScript (.mjs) to Rust (.rs) using rust-script
- Scripts now use native Rust with rust-script for execution in shell
- Removed Node.js dependency from CI/CD pipeline
- Updated GitHub Actions workflow to use rust-script instead of node
- Updated README and CONTRIBUTING documentation with new script references

### Added

- Cache `restore-keys` for partial cache hits across all workflow jobs
- Explicit `token` parameter in checkout for release jobs
- Code coverage job with `cargo-llvm-cov` and Codecov integration
- Codecov badge in README.md
- Pre-release version support (e.g., `0.1.0-beta.1`) in version parsing
- `--release-label` parameter for multi-language release disambiguation
- `ensure_version_exceeds_published()` logic to prevent publishing duplicate versions
- `get_max_published_version()` to query highest non-yanked version from crates.io
- `max_published_version` output from check-release-needed for downstream use
- Version fallback logic in auto-release Create GitHub Release step

### Changed

- Updated `actions/checkout` from v4 to v6
- Updated `actions/cache` from v4 to v5
- Updated `peter-evans/create-pull-request` from v7 to v8
- Made `publish-crate.rs` fail (exit 1) when version already exists on crates.io
- Improved `create-github-release.rs` to check combined stdout+stderr and detect "Validation Failed"

### Fixed

- Fix publish steps overriding workflow-level CARGO_TOKEN fallback, breaking CARGO_REGISTRY_TOKEN-only configurations (#32)
- Fix non-fast-forward push failures in multi-workflow repos by adding fetch/rebase and push retry logic (#31)
- Add mono-repo path support to check-changelog-fragment.rs, check-version-modification.rs, and create-changelog-fragment.rs
- Add `!cancelled()` guard to test job condition to respect workflow cancellation

### Fixed

- Fixed `version-and-commit.rs` to check crates.io instead of git tags for determining if a version is already released
- This prevents the release pipeline from getting stuck when git tags exist without corresponding crates.io publication

### Added

- Added `--tag-prefix` support to `version-and-commit.rs` for multi-language repository compatibility
- Added crates.io and docs.rs badges to README.md
- Added automatic crates.io and docs.rs badge injection in GitHub release notes
- Added documentation deployment job to CI/CD pipeline (deploys to GitHub Pages after release)
- Added case study documentation for issue #25

### Fixed
- Fixed unsupported look-ahead regex in `create-github-release.rs` that caused a panic when parsing CHANGELOG.md. Replaced with a two-step approach using only features supported by Rust's `regex` crate.

### Changed
- Restructured example application as a simple CLI sum calculator using `lino-arguments`
- Renamed default package to `example-sum-package-name` with Unlicense license
- Reorganized test structure: `tests/unit/sum.rs`, `tests/integration/sum.rs`, `tests/unit/ci-cd/`
- Converted experiment scripts into proper unit tests in `tests/unit/ci-cd/changelog_parsing.rs`
- Added CI/CD skip logic for template default package name `example-sum-package-name`
- Updated README.md badges and documentation

### Fixed

- Change detection script now uses per-commit diff instead of full PR diff, so commits touching only non-code files correctly skip CI jobs even when earlier commits in the same PR changed code files

### Fixed
- Make release scripts resolve the publishable crate manifest when the repository root uses a Cargo workspace manifest.

### Fixed
- Decoupled GitHub Pages documentation deployment from package release publication and fixed release-script warning failures under `RUSTFLAGS=-Dwarnings`.

### Changed
- Added explicit GitHub Actions job timeouts and documented Rust test timeout guidance.

### Fixed
- Added a non-blocking warning threshold to the Rust file-size check so near-limit files are surfaced before concurrent PR merges can exceed the hard limit.

### Fixed
- Made `create-github-release.rs` build GitHub release titles as `[Language] X.Y.Z` instead of reusing the tag prefix.

### Added
- Added optional Docker Hub image publishing tied to Rust crate releases, including crates.io visibility waiting, version/latest image tags, and Docker Hub badges in GitHub release notes.

### Changed
- Release completeness checks now self-heal when crates.io exists but configured Docker Hub or GitHub release artifacts are missing.

### Fixed
- Switched documentation deployment to the official GitHub Pages artifact workflow so repositories using GitHub Actions as their Pages source do not get false-positive branch-push deploys.

### Added
- Documented the one-time `Settings → Pages → Source = GitHub Actions` prerequisite for the `deploy-docs` job in `README.md` and as a comment above the `deploy-docs` job in `release.yml`, so downstream template users hit a documented setup step instead of a `Get Pages site failed` error on the first deploy.

### Added
- Tracking case study at `docs/case-studies/issue-52/` registering the `browser-commander` + Playwright preview-regeneration pattern from [`konard/vk-bot-desktop#52`](https://github.com/konard/vk-bot-desktop/pull/52), with an activation checklist for when an example-app surface lands in this template. Documentation only — no workflow, script, or runtime code changes. Primary upstream tracking issue: [`link-foundation/js-ai-driven-development-pipeline-template#62`](https://github.com/link-foundation/js-ai-driven-development-pipeline-template/issues/62).

### Fixed
- Release automation now keeps the workspace package entry in `Cargo.lock` synchronized when `scripts/version-and-commit.rs` bumps `Cargo.toml`, preventing stale lock-file version diffs in later pull requests.

### Fixed
- `scripts/publish-crate.rs` now treats crates.io HTTP 429 throttle responses ("You have published too many versions of this crate in the last 24 hours") as a deferred `publish_result=rate_limited` outcome (it writes the output, prints an explanatory banner and exits successfully) instead of a hard CI failure reported as a generic `failed` ("Failed to publish for unknown reason"). Authentication, already-published and unknown failures still exit non-zero. Failed-publish classification is consolidated through a single `classify_failure` function and `FailureKind` enum (with an `is_deferred` predicate), covered by unit tests runnable via `rust-script --test scripts/publish-crate.rs`.
- The release workflow (`.github/workflows/release.yml`) now gates crate-availability waiting, Docker Hub publishing and GitHub release creation on either an already-published crate or `publish_result=success`, so a deferred (rate-limited) crate upload no longer produces partial downstream release artifacts and the same version is retried automatically on the next push to `main`.

### Fixed
- Fixed reversed `cancel-in-progress` concurrency condition in `release.yml` that cancelled in-flight releases on `main` and never superseded older PR runs. The condition now uses `!=` so `main` releases run to completion while newer PR pushes cancel stale runs.

### Added
- Added a `scripts/check-crate-size.rs` guard that builds the `.crate` archive and fails the release before publishing when it exceeds the crates.io 10 MiB upload limit. The check runs in the build job and before publishing in both the auto-release and manual-release jobs.

### Changed
- Added a narrow `include` allowlist to `Cargo.toml` so docs, case studies, generated CI artifacts, changelog fragments, scripts, and experiments no longer inflate the published release archive.

### Fixed
- Prevented GitHub release creation from treating generic API validation failures as existing releases, and capped oversized release notes with a link to the full tagged changelog.

### Added
- Added the initial `meta-language` links-network core with self-description roots,
  verification, trivia attachment configuration, and a minimal CLI.
- Added the default lossless `parse` entry point, projection views, and a
  tested parity roadmap for competitor features, grammar embedding, and language
  coverage targets.
- Added exact reconstruction, mixed-region detection, query matching,
  substitution rules, concept reconstruction, object identity helpers,
  many-valued truth values, and executable parity fixtures for every tracked
  competitor target.
- Added executable language fixtures and tests for every requested Markdown,
  HTML, top-ten programming-language, and top-ten natural-language target.

### Fixed
- Added a CI guard that rejects Rust test modules and test attributes under `src/`, keeping tests in the `tests/` tree.

## Minor Changes

- Add immutable and mutable network snapshots with provenance and forward
  version commits for roadmap snapshot/versioning coverage.

### Added
- Added `txt` as a first-class markup/container target with a UTF-8 lossless
  fixture and content-sniffing fallback regions.

### Fixed
- Reconciled the natural-language target ordering with the Ethnologue 2025
  total-speaker order cited by the parity roadmap.

### Added
- Added a grammar-backed parser front end for Python, C, Java, C++, C#, JavaScript, and R using official tree-sitter grammar crates.

### Added
- Parse supported mixed-mode embedded regions into the host links network with grammar-backed syntax links.

### Added
- Added natural-language segmentation, identification, normalization, and bidi annotation links over lossless text parses.

### Added
- Added grammar-backed Visual Basic parsing with byte-exact reconstruction and recovery flag coverage.

bump: minor

- Add a grammar-backed `sql-ansi` SQL-family dialect fixture using
  `tree-sitter-sequel`.

## [0.25.0] - 2026-06-07

### Added
- Changeset-style fragment format with frontmatter for specifying version bump type
- New `get-bump-type.mjs` script to automatically determine version bump from fragments
- Automatic version bumping on merge to main based on changelog fragments
- Detailed documentation for the changelog fragment system in `changelog.d/README.md`

### Changed
- Updated `collect-changelog.mjs` to strip frontmatter when collecting fragments
- Updated `version-and-commit.mjs` to handle frontmatter in fragments
- Enhanced release workflow to automatically determine bump type from changesets

### Changed
- Add `detect-changes` job with cross-platform `detect-code-changes.mjs` script
- Make lint job independent of changelog check (runs based on file changes only)
- Allow docs-only PRs without changelog fragment requirement
- Handle changelog check 'skipped' state in dependent jobs
- Exclude `changelog.d/`, `docs/`, `experiments/`, `examples/` folders and markdown files from code changes detection

### Fixed
- Fixed README.md to correctly reference Node.js scripts (`.mjs`) instead of Python scripts (`.py`)
- Updated project structure in README.md to match actual script files in `scripts/` directory
- Fixed example code in README.md that had invalid Rust with two `main` functions

### Added

- Added crates.io publishing support to CI/CD workflow
- Added `release_mode` input with "instant" and "changelog-pr" options for manual releases
- Added `--tag-prefix` and `--crates-io-url` options to create-github-release.mjs script
- Added comprehensive case study documentation for Issue #11 in docs/case-studies/issue-11/

### Changed

- Changed changelog fragment check from warning to error (exit 1) to enforce changelog requirements
- Updated job conditions with `always() && !cancelled()` to fix workflow_dispatch job skipping issue
- Renamed manual-release job to "Instant Release" for clarity

### Fixed

- Fixed deprecated `::set-output` GitHub Actions command in version-and-commit.mjs
- Fixed workflow_dispatch triggering issues where lint/build/release jobs were incorrectly skipped

### Fixed

- Fixed changelog fragment check to validate that a fragment is **added in the PR diff** rather than just checking if any fragments exist in the directory. This prevents the check from incorrectly passing when there are leftover fragments from previous PRs that haven't been released yet.

### Changed

- Converted shell scripts in `release.yml` to cross-platform `.mjs` scripts for improved portability and performance:
  - `check-changelog-fragment.mjs` - validates changelog fragment is added in PR diff
  - `git-config.mjs` - configures git user for CI/CD
  - `check-release-needed.mjs` - checks if release is needed
  - `publish-crate.mjs` - publishes package to crates.io
  - `create-changelog-fragment.mjs` - creates changelog fragments for manual releases
  - `get-version.mjs` - gets current version from Cargo.toml

### Added

- Added `check-version-modification.mjs` script to detect manual version changes in Cargo.toml
- Added `version-check` job to CI/CD workflow that runs on pull requests
- Added skip logic for automated release branches (changelog-manual-release-*, changeset-release/*, release/*, automated-release/*)

### Changed

- Version modifications in Cargo.toml are now blocked in pull requests to enforce automated release pipeline

### Added

- Added support for `CARGO_REGISTRY_TOKEN` as alternative to `CARGO_TOKEN` for crates.io publishing
- Added case study documentation for Issue #17 (yargs reserved word and dual token support)

### Changed

- Updated workflow to use fallback logic: `${{ secrets.CARGO_REGISTRY_TOKEN || secrets.CARGO_TOKEN }}`
- Improved publish-crate.mjs to check both `CARGO_REGISTRY_TOKEN` and `CARGO_TOKEN` environment variables
- Added warning message when neither token is set

### Added
- New `scripts/rust-paths.mjs` utility for automatic Rust package root detection
- Support for both single-language and multi-language repository structures in all CI/CD scripts
- Configuration options via `--rust-root` CLI argument and `RUST_ROOT` environment variable
- Comprehensive case study documentation in `docs/case-studies/issue-19/`

### Changed
- Updated all release scripts to use the new path detection utility:
  - `scripts/bump-version.mjs`
  - `scripts/check-release-needed.mjs`
  - `scripts/collect-changelog.mjs`
  - `scripts/get-bump-type.mjs`
  - `scripts/get-version.mjs`
  - `scripts/publish-crate.mjs`
  - `scripts/version-and-commit.mjs`

### Changed

- **check-release-needed.mjs**: Now checks crates.io API directly instead of git tags to determine if a version is already released. This prevents false positives where git tags exist but the package was never actually published to crates.io.

### Added

- **CI/CD Troubleshooting Guide**: New documentation at `docs/ci-cd/troubleshooting.md` covering common issues like skipped jobs, false positive version checks, publishing failures, and secret configuration.

- **Enhanced Error Handling in publish-crate.mjs**: Added specific detection and helpful error messages for authentication failures, including guidance on secret configuration and workflow setup.

- **Case Study Documentation**: Added comprehensive case study at `docs/case-studies/issue-21/` analyzing CI/CD failures from browser-commander repository (issues #27, #29, #31, #33) with timeline, root causes, and lessons learned.

### Fixed

- **Prevent False Positive Version Checks**: The release workflow now correctly identifies unpublished versions by checking crates.io instead of relying on git tags, which can exist without the package being published.

### Changed

- Translated all CI/CD scripts from JavaScript (.mjs) to Rust (.rs) using rust-script
- Scripts now use native Rust with rust-script for execution in shell
- Removed Node.js dependency from CI/CD pipeline
- Updated GitHub Actions workflow to use rust-script instead of node
- Updated README and CONTRIBUTING documentation with new script references

### Added

- Cache `restore-keys` for partial cache hits across all workflow jobs
- Explicit `token` parameter in checkout for release jobs
- Code coverage job with `cargo-llvm-cov` and Codecov integration
- Codecov badge in README.md
- Pre-release version support (e.g., `0.1.0-beta.1`) in version parsing
- `--release-label` parameter for multi-language release disambiguation
- `ensure_version_exceeds_published()` logic to prevent publishing duplicate versions
- `get_max_published_version()` to query highest non-yanked version from crates.io
- `max_published_version` output from check-release-needed for downstream use
- Version fallback logic in auto-release Create GitHub Release step

### Changed

- Updated `actions/checkout` from v4 to v6
- Updated `actions/cache` from v4 to v5
- Updated `peter-evans/create-pull-request` from v7 to v8
- Made `publish-crate.rs` fail (exit 1) when version already exists on crates.io
- Improved `create-github-release.rs` to check combined stdout+stderr and detect "Validation Failed"

### Fixed

- Fix publish steps overriding workflow-level CARGO_TOKEN fallback, breaking CARGO_REGISTRY_TOKEN-only configurations (#32)
- Fix non-fast-forward push failures in multi-workflow repos by adding fetch/rebase and push retry logic (#31)
- Add mono-repo path support to check-changelog-fragment.rs, check-version-modification.rs, and create-changelog-fragment.rs
- Add `!cancelled()` guard to test job condition to respect workflow cancellation

### Fixed

- Fixed `version-and-commit.rs` to check crates.io instead of git tags for determining if a version is already released
- This prevents the release pipeline from getting stuck when git tags exist without corresponding crates.io publication

### Added

- Added `--tag-prefix` support to `version-and-commit.rs` for multi-language repository compatibility
- Added crates.io and docs.rs badges to README.md
- Added automatic crates.io and docs.rs badge injection in GitHub release notes
- Added documentation deployment job to CI/CD pipeline (deploys to GitHub Pages after release)
- Added case study documentation for issue #25

### Fixed
- Fixed unsupported look-ahead regex in `create-github-release.rs` that caused a panic when parsing CHANGELOG.md. Replaced with a two-step approach using only features supported by Rust's `regex` crate.

### Changed
- Restructured example application as a simple CLI sum calculator using `lino-arguments`
- Renamed default package to `example-sum-package-name` with Unlicense license
- Reorganized test structure: `tests/unit/sum.rs`, `tests/integration/sum.rs`, `tests/unit/ci-cd/`
- Converted experiment scripts into proper unit tests in `tests/unit/ci-cd/changelog_parsing.rs`
- Added CI/CD skip logic for template default package name `example-sum-package-name`
- Updated README.md badges and documentation

### Fixed

- Change detection script now uses per-commit diff instead of full PR diff, so commits touching only non-code files correctly skip CI jobs even when earlier commits in the same PR changed code files

### Fixed
- Make release scripts resolve the publishable crate manifest when the repository root uses a Cargo workspace manifest.

### Fixed
- Decoupled GitHub Pages documentation deployment from package release publication and fixed release-script warning failures under `RUSTFLAGS=-Dwarnings`.

### Changed
- Added explicit GitHub Actions job timeouts and documented Rust test timeout guidance.

### Fixed
- Added a non-blocking warning threshold to the Rust file-size check so near-limit files are surfaced before concurrent PR merges can exceed the hard limit.

### Fixed
- Made `create-github-release.rs` build GitHub release titles as `[Language] X.Y.Z` instead of reusing the tag prefix.

### Added
- Added optional Docker Hub image publishing tied to Rust crate releases, including crates.io visibility waiting, version/latest image tags, and Docker Hub badges in GitHub release notes.

### Changed
- Release completeness checks now self-heal when crates.io exists but configured Docker Hub or GitHub release artifacts are missing.

### Fixed
- Switched documentation deployment to the official GitHub Pages artifact workflow so repositories using GitHub Actions as their Pages source do not get false-positive branch-push deploys.

### Added
- Documented the one-time `Settings → Pages → Source = GitHub Actions` prerequisite for the `deploy-docs` job in `README.md` and as a comment above the `deploy-docs` job in `release.yml`, so downstream template users hit a documented setup step instead of a `Get Pages site failed` error on the first deploy.

### Added
- Tracking case study at `docs/case-studies/issue-52/` registering the `browser-commander` + Playwright preview-regeneration pattern from [`konard/vk-bot-desktop#52`](https://github.com/konard/vk-bot-desktop/pull/52), with an activation checklist for when an example-app surface lands in this template. Documentation only — no workflow, script, or runtime code changes. Primary upstream tracking issue: [`link-foundation/js-ai-driven-development-pipeline-template#62`](https://github.com/link-foundation/js-ai-driven-development-pipeline-template/issues/62).

### Fixed
- Release automation now keeps the workspace package entry in `Cargo.lock` synchronized when `scripts/version-and-commit.rs` bumps `Cargo.toml`, preventing stale lock-file version diffs in later pull requests.

### Fixed
- `scripts/publish-crate.rs` now treats crates.io HTTP 429 throttle responses ("You have published too many versions of this crate in the last 24 hours") as a deferred `publish_result=rate_limited` outcome (it writes the output, prints an explanatory banner and exits successfully) instead of a hard CI failure reported as a generic `failed` ("Failed to publish for unknown reason"). Authentication, already-published and unknown failures still exit non-zero. Failed-publish classification is consolidated through a single `classify_failure` function and `FailureKind` enum (with an `is_deferred` predicate), covered by unit tests runnable via `rust-script --test scripts/publish-crate.rs`.
- The release workflow (`.github/workflows/release.yml`) now gates crate-availability waiting, Docker Hub publishing and GitHub release creation on either an already-published crate or `publish_result=success`, so a deferred (rate-limited) crate upload no longer produces partial downstream release artifacts and the same version is retried automatically on the next push to `main`.

### Fixed
- Fixed reversed `cancel-in-progress` concurrency condition in `release.yml` that cancelled in-flight releases on `main` and never superseded older PR runs. The condition now uses `!=` so `main` releases run to completion while newer PR pushes cancel stale runs.

### Added
- Added a `scripts/check-crate-size.rs` guard that builds the `.crate` archive and fails the release before publishing when it exceeds the crates.io 10 MiB upload limit. The check runs in the build job and before publishing in both the auto-release and manual-release jobs.

### Changed
- Added a narrow `include` allowlist to `Cargo.toml` so docs, case studies, generated CI artifacts, changelog fragments, scripts, and experiments no longer inflate the published release archive.

### Fixed
- Prevented GitHub release creation from treating generic API validation failures as existing releases, and capped oversized release notes with a link to the full tagged changelog.

### Added
- Added the initial `meta-language` links-network core with self-description roots,
  verification, trivia attachment configuration, and a minimal CLI.
- Added the default lossless `parse` entry point, projection views, and a
  tested parity roadmap for competitor features, grammar embedding, and language
  coverage targets.
- Added exact reconstruction, mixed-region detection, query matching,
  substitution rules, concept reconstruction, object identity helpers,
  many-valued truth values, and executable parity fixtures for every tracked
  competitor target.
- Added executable language fixtures and tests for every requested Markdown,
  HTML, top-ten programming-language, and top-ten natural-language target.

### Fixed
- Added a CI guard that rejects Rust test modules and test attributes under `src/`, keeping tests in the `tests/` tree.

## Minor Changes

- Add immutable and mutable network snapshots with provenance and forward
  version commits for roadmap snapshot/versioning coverage.

### Added
- Added `txt` as a first-class markup/container target with a UTF-8 lossless
  fixture and content-sniffing fallback regions.

### Fixed
- Reconciled the natural-language target ordering with the Ethnologue 2025
  total-speaker order cited by the parity roadmap.

### Added
- Added a grammar-backed parser front end for Python, C, Java, C++, C#, JavaScript, and R using official tree-sitter grammar crates.

### Added
- Parse supported mixed-mode embedded regions into the host links network with grammar-backed syntax links.

### Added
- Added natural-language segmentation, identification, normalization, and bidi annotation links over lossless text parses.

### Added
- Added grammar-backed Visual Basic parsing with byte-exact reconstruction and recovery flag coverage.

## [0.24.0] - 2026-06-07

### Added
- Changeset-style fragment format with frontmatter for specifying version bump type
- New `get-bump-type.mjs` script to automatically determine version bump from fragments
- Automatic version bumping on merge to main based on changelog fragments
- Detailed documentation for the changelog fragment system in `changelog.d/README.md`

### Changed
- Updated `collect-changelog.mjs` to strip frontmatter when collecting fragments
- Updated `version-and-commit.mjs` to handle frontmatter in fragments
- Enhanced release workflow to automatically determine bump type from changesets

### Changed
- Add `detect-changes` job with cross-platform `detect-code-changes.mjs` script
- Make lint job independent of changelog check (runs based on file changes only)
- Allow docs-only PRs without changelog fragment requirement
- Handle changelog check 'skipped' state in dependent jobs
- Exclude `changelog.d/`, `docs/`, `experiments/`, `examples/` folders and markdown files from code changes detection

### Fixed
- Fixed README.md to correctly reference Node.js scripts (`.mjs`) instead of Python scripts (`.py`)
- Updated project structure in README.md to match actual script files in `scripts/` directory
- Fixed example code in README.md that had invalid Rust with two `main` functions

### Added

- Added crates.io publishing support to CI/CD workflow
- Added `release_mode` input with "instant" and "changelog-pr" options for manual releases
- Added `--tag-prefix` and `--crates-io-url` options to create-github-release.mjs script
- Added comprehensive case study documentation for Issue #11 in docs/case-studies/issue-11/

### Changed

- Changed changelog fragment check from warning to error (exit 1) to enforce changelog requirements
- Updated job conditions with `always() && !cancelled()` to fix workflow_dispatch job skipping issue
- Renamed manual-release job to "Instant Release" for clarity

### Fixed

- Fixed deprecated `::set-output` GitHub Actions command in version-and-commit.mjs
- Fixed workflow_dispatch triggering issues where lint/build/release jobs were incorrectly skipped

### Fixed

- Fixed changelog fragment check to validate that a fragment is **added in the PR diff** rather than just checking if any fragments exist in the directory. This prevents the check from incorrectly passing when there are leftover fragments from previous PRs that haven't been released yet.

### Changed

- Converted shell scripts in `release.yml` to cross-platform `.mjs` scripts for improved portability and performance:
  - `check-changelog-fragment.mjs` - validates changelog fragment is added in PR diff
  - `git-config.mjs` - configures git user for CI/CD
  - `check-release-needed.mjs` - checks if release is needed
  - `publish-crate.mjs` - publishes package to crates.io
  - `create-changelog-fragment.mjs` - creates changelog fragments for manual releases
  - `get-version.mjs` - gets current version from Cargo.toml

### Added

- Added `check-version-modification.mjs` script to detect manual version changes in Cargo.toml
- Added `version-check` job to CI/CD workflow that runs on pull requests
- Added skip logic for automated release branches (changelog-manual-release-*, changeset-release/*, release/*, automated-release/*)

### Changed

- Version modifications in Cargo.toml are now blocked in pull requests to enforce automated release pipeline

### Added

- Added support for `CARGO_REGISTRY_TOKEN` as alternative to `CARGO_TOKEN` for crates.io publishing
- Added case study documentation for Issue #17 (yargs reserved word and dual token support)

### Changed

- Updated workflow to use fallback logic: `${{ secrets.CARGO_REGISTRY_TOKEN || secrets.CARGO_TOKEN }}`
- Improved publish-crate.mjs to check both `CARGO_REGISTRY_TOKEN` and `CARGO_TOKEN` environment variables
- Added warning message when neither token is set

### Added
- New `scripts/rust-paths.mjs` utility for automatic Rust package root detection
- Support for both single-language and multi-language repository structures in all CI/CD scripts
- Configuration options via `--rust-root` CLI argument and `RUST_ROOT` environment variable
- Comprehensive case study documentation in `docs/case-studies/issue-19/`

### Changed
- Updated all release scripts to use the new path detection utility:
  - `scripts/bump-version.mjs`
  - `scripts/check-release-needed.mjs`
  - `scripts/collect-changelog.mjs`
  - `scripts/get-bump-type.mjs`
  - `scripts/get-version.mjs`
  - `scripts/publish-crate.mjs`
  - `scripts/version-and-commit.mjs`

### Changed

- **check-release-needed.mjs**: Now checks crates.io API directly instead of git tags to determine if a version is already released. This prevents false positives where git tags exist but the package was never actually published to crates.io.

### Added

- **CI/CD Troubleshooting Guide**: New documentation at `docs/ci-cd/troubleshooting.md` covering common issues like skipped jobs, false positive version checks, publishing failures, and secret configuration.

- **Enhanced Error Handling in publish-crate.mjs**: Added specific detection and helpful error messages for authentication failures, including guidance on secret configuration and workflow setup.

- **Case Study Documentation**: Added comprehensive case study at `docs/case-studies/issue-21/` analyzing CI/CD failures from browser-commander repository (issues #27, #29, #31, #33) with timeline, root causes, and lessons learned.

### Fixed

- **Prevent False Positive Version Checks**: The release workflow now correctly identifies unpublished versions by checking crates.io instead of relying on git tags, which can exist without the package being published.

### Changed

- Translated all CI/CD scripts from JavaScript (.mjs) to Rust (.rs) using rust-script
- Scripts now use native Rust with rust-script for execution in shell
- Removed Node.js dependency from CI/CD pipeline
- Updated GitHub Actions workflow to use rust-script instead of node
- Updated README and CONTRIBUTING documentation with new script references

### Added

- Cache `restore-keys` for partial cache hits across all workflow jobs
- Explicit `token` parameter in checkout for release jobs
- Code coverage job with `cargo-llvm-cov` and Codecov integration
- Codecov badge in README.md
- Pre-release version support (e.g., `0.1.0-beta.1`) in version parsing
- `--release-label` parameter for multi-language release disambiguation
- `ensure_version_exceeds_published()` logic to prevent publishing duplicate versions
- `get_max_published_version()` to query highest non-yanked version from crates.io
- `max_published_version` output from check-release-needed for downstream use
- Version fallback logic in auto-release Create GitHub Release step

### Changed

- Updated `actions/checkout` from v4 to v6
- Updated `actions/cache` from v4 to v5
- Updated `peter-evans/create-pull-request` from v7 to v8
- Made `publish-crate.rs` fail (exit 1) when version already exists on crates.io
- Improved `create-github-release.rs` to check combined stdout+stderr and detect "Validation Failed"

### Fixed

- Fix publish steps overriding workflow-level CARGO_TOKEN fallback, breaking CARGO_REGISTRY_TOKEN-only configurations (#32)
- Fix non-fast-forward push failures in multi-workflow repos by adding fetch/rebase and push retry logic (#31)
- Add mono-repo path support to check-changelog-fragment.rs, check-version-modification.rs, and create-changelog-fragment.rs
- Add `!cancelled()` guard to test job condition to respect workflow cancellation

### Fixed

- Fixed `version-and-commit.rs` to check crates.io instead of git tags for determining if a version is already released
- This prevents the release pipeline from getting stuck when git tags exist without corresponding crates.io publication

### Added

- Added `--tag-prefix` support to `version-and-commit.rs` for multi-language repository compatibility
- Added crates.io and docs.rs badges to README.md
- Added automatic crates.io and docs.rs badge injection in GitHub release notes
- Added documentation deployment job to CI/CD pipeline (deploys to GitHub Pages after release)
- Added case study documentation for issue #25

### Fixed
- Fixed unsupported look-ahead regex in `create-github-release.rs` that caused a panic when parsing CHANGELOG.md. Replaced with a two-step approach using only features supported by Rust's `regex` crate.

### Changed
- Restructured example application as a simple CLI sum calculator using `lino-arguments`
- Renamed default package to `example-sum-package-name` with Unlicense license
- Reorganized test structure: `tests/unit/sum.rs`, `tests/integration/sum.rs`, `tests/unit/ci-cd/`
- Converted experiment scripts into proper unit tests in `tests/unit/ci-cd/changelog_parsing.rs`
- Added CI/CD skip logic for template default package name `example-sum-package-name`
- Updated README.md badges and documentation

### Fixed

- Change detection script now uses per-commit diff instead of full PR diff, so commits touching only non-code files correctly skip CI jobs even when earlier commits in the same PR changed code files

### Fixed
- Make release scripts resolve the publishable crate manifest when the repository root uses a Cargo workspace manifest.

### Fixed
- Decoupled GitHub Pages documentation deployment from package release publication and fixed release-script warning failures under `RUSTFLAGS=-Dwarnings`.

### Changed
- Added explicit GitHub Actions job timeouts and documented Rust test timeout guidance.

### Fixed
- Added a non-blocking warning threshold to the Rust file-size check so near-limit files are surfaced before concurrent PR merges can exceed the hard limit.

### Fixed
- Made `create-github-release.rs` build GitHub release titles as `[Language] X.Y.Z` instead of reusing the tag prefix.

### Added
- Added optional Docker Hub image publishing tied to Rust crate releases, including crates.io visibility waiting, version/latest image tags, and Docker Hub badges in GitHub release notes.

### Changed
- Release completeness checks now self-heal when crates.io exists but configured Docker Hub or GitHub release artifacts are missing.

### Fixed
- Switched documentation deployment to the official GitHub Pages artifact workflow so repositories using GitHub Actions as their Pages source do not get false-positive branch-push deploys.

### Added
- Documented the one-time `Settings → Pages → Source = GitHub Actions` prerequisite for the `deploy-docs` job in `README.md` and as a comment above the `deploy-docs` job in `release.yml`, so downstream template users hit a documented setup step instead of a `Get Pages site failed` error on the first deploy.

### Added
- Tracking case study at `docs/case-studies/issue-52/` registering the `browser-commander` + Playwright preview-regeneration pattern from [`konard/vk-bot-desktop#52`](https://github.com/konard/vk-bot-desktop/pull/52), with an activation checklist for when an example-app surface lands in this template. Documentation only — no workflow, script, or runtime code changes. Primary upstream tracking issue: [`link-foundation/js-ai-driven-development-pipeline-template#62`](https://github.com/link-foundation/js-ai-driven-development-pipeline-template/issues/62).

### Fixed
- Release automation now keeps the workspace package entry in `Cargo.lock` synchronized when `scripts/version-and-commit.rs` bumps `Cargo.toml`, preventing stale lock-file version diffs in later pull requests.

### Fixed
- `scripts/publish-crate.rs` now treats crates.io HTTP 429 throttle responses ("You have published too many versions of this crate in the last 24 hours") as a deferred `publish_result=rate_limited` outcome (it writes the output, prints an explanatory banner and exits successfully) instead of a hard CI failure reported as a generic `failed` ("Failed to publish for unknown reason"). Authentication, already-published and unknown failures still exit non-zero. Failed-publish classification is consolidated through a single `classify_failure` function and `FailureKind` enum (with an `is_deferred` predicate), covered by unit tests runnable via `rust-script --test scripts/publish-crate.rs`.
- The release workflow (`.github/workflows/release.yml`) now gates crate-availability waiting, Docker Hub publishing and GitHub release creation on either an already-published crate or `publish_result=success`, so a deferred (rate-limited) crate upload no longer produces partial downstream release artifacts and the same version is retried automatically on the next push to `main`.

### Fixed
- Fixed reversed `cancel-in-progress` concurrency condition in `release.yml` that cancelled in-flight releases on `main` and never superseded older PR runs. The condition now uses `!=` so `main` releases run to completion while newer PR pushes cancel stale runs.

### Added
- Added a `scripts/check-crate-size.rs` guard that builds the `.crate` archive and fails the release before publishing when it exceeds the crates.io 10 MiB upload limit. The check runs in the build job and before publishing in both the auto-release and manual-release jobs.

### Changed
- Added a narrow `include` allowlist to `Cargo.toml` so docs, case studies, generated CI artifacts, changelog fragments, scripts, and experiments no longer inflate the published release archive.

### Fixed
- Prevented GitHub release creation from treating generic API validation failures as existing releases, and capped oversized release notes with a link to the full tagged changelog.

### Added
- Added the initial `meta-language` links-network core with self-description roots,
  verification, trivia attachment configuration, and a minimal CLI.
- Added the default lossless `parse` entry point, projection views, and a
  tested parity roadmap for competitor features, grammar embedding, and language
  coverage targets.
- Added exact reconstruction, mixed-region detection, query matching,
  substitution rules, concept reconstruction, object identity helpers,
  many-valued truth values, and executable parity fixtures for every tracked
  competitor target.
- Added executable language fixtures and tests for every requested Markdown,
  HTML, top-ten programming-language, and top-ten natural-language target.

### Fixed
- Added a CI guard that rejects Rust test modules and test attributes under `src/`, keeping tests in the `tests/` tree.

## Minor Changes

- Add immutable and mutable network snapshots with provenance and forward
  version commits for roadmap snapshot/versioning coverage.

### Added
- Added `txt` as a first-class markup/container target with a UTF-8 lossless
  fixture and content-sniffing fallback regions.

### Fixed
- Reconciled the natural-language target ordering with the Ethnologue 2025
  total-speaker order cited by the parity roadmap.

### Added
- Added a grammar-backed parser front end for Python, C, Java, C++, C#, JavaScript, and R using official tree-sitter grammar crates.

### Added
- Parse supported mixed-mode embedded regions into the host links network with grammar-backed syntax links.

### Added
- Added natural-language segmentation, identification, normalization, and bidi annotation links over lossless text parses.

## [0.23.0] - 2026-06-06

### Added
- Changeset-style fragment format with frontmatter for specifying version bump type
- New `get-bump-type.mjs` script to automatically determine version bump from fragments
- Automatic version bumping on merge to main based on changelog fragments
- Detailed documentation for the changelog fragment system in `changelog.d/README.md`

### Changed
- Updated `collect-changelog.mjs` to strip frontmatter when collecting fragments
- Updated `version-and-commit.mjs` to handle frontmatter in fragments
- Enhanced release workflow to automatically determine bump type from changesets

### Changed
- Add `detect-changes` job with cross-platform `detect-code-changes.mjs` script
- Make lint job independent of changelog check (runs based on file changes only)
- Allow docs-only PRs without changelog fragment requirement
- Handle changelog check 'skipped' state in dependent jobs
- Exclude `changelog.d/`, `docs/`, `experiments/`, `examples/` folders and markdown files from code changes detection

### Fixed
- Fixed README.md to correctly reference Node.js scripts (`.mjs`) instead of Python scripts (`.py`)
- Updated project structure in README.md to match actual script files in `scripts/` directory
- Fixed example code in README.md that had invalid Rust with two `main` functions

### Added

- Added crates.io publishing support to CI/CD workflow
- Added `release_mode` input with "instant" and "changelog-pr" options for manual releases
- Added `--tag-prefix` and `--crates-io-url` options to create-github-release.mjs script
- Added comprehensive case study documentation for Issue #11 in docs/case-studies/issue-11/

### Changed

- Changed changelog fragment check from warning to error (exit 1) to enforce changelog requirements
- Updated job conditions with `always() && !cancelled()` to fix workflow_dispatch job skipping issue
- Renamed manual-release job to "Instant Release" for clarity

### Fixed

- Fixed deprecated `::set-output` GitHub Actions command in version-and-commit.mjs
- Fixed workflow_dispatch triggering issues where lint/build/release jobs were incorrectly skipped

### Fixed

- Fixed changelog fragment check to validate that a fragment is **added in the PR diff** rather than just checking if any fragments exist in the directory. This prevents the check from incorrectly passing when there are leftover fragments from previous PRs that haven't been released yet.

### Changed

- Converted shell scripts in `release.yml` to cross-platform `.mjs` scripts for improved portability and performance:
  - `check-changelog-fragment.mjs` - validates changelog fragment is added in PR diff
  - `git-config.mjs` - configures git user for CI/CD
  - `check-release-needed.mjs` - checks if release is needed
  - `publish-crate.mjs` - publishes package to crates.io
  - `create-changelog-fragment.mjs` - creates changelog fragments for manual releases
  - `get-version.mjs` - gets current version from Cargo.toml

### Added

- Added `check-version-modification.mjs` script to detect manual version changes in Cargo.toml
- Added `version-check` job to CI/CD workflow that runs on pull requests
- Added skip logic for automated release branches (changelog-manual-release-*, changeset-release/*, release/*, automated-release/*)

### Changed

- Version modifications in Cargo.toml are now blocked in pull requests to enforce automated release pipeline

### Added

- Added support for `CARGO_REGISTRY_TOKEN` as alternative to `CARGO_TOKEN` for crates.io publishing
- Added case study documentation for Issue #17 (yargs reserved word and dual token support)

### Changed

- Updated workflow to use fallback logic: `${{ secrets.CARGO_REGISTRY_TOKEN || secrets.CARGO_TOKEN }}`
- Improved publish-crate.mjs to check both `CARGO_REGISTRY_TOKEN` and `CARGO_TOKEN` environment variables
- Added warning message when neither token is set

### Added
- New `scripts/rust-paths.mjs` utility for automatic Rust package root detection
- Support for both single-language and multi-language repository structures in all CI/CD scripts
- Configuration options via `--rust-root` CLI argument and `RUST_ROOT` environment variable
- Comprehensive case study documentation in `docs/case-studies/issue-19/`

### Changed
- Updated all release scripts to use the new path detection utility:
  - `scripts/bump-version.mjs`
  - `scripts/check-release-needed.mjs`
  - `scripts/collect-changelog.mjs`
  - `scripts/get-bump-type.mjs`
  - `scripts/get-version.mjs`
  - `scripts/publish-crate.mjs`
  - `scripts/version-and-commit.mjs`

### Changed

- **check-release-needed.mjs**: Now checks crates.io API directly instead of git tags to determine if a version is already released. This prevents false positives where git tags exist but the package was never actually published to crates.io.

### Added

- **CI/CD Troubleshooting Guide**: New documentation at `docs/ci-cd/troubleshooting.md` covering common issues like skipped jobs, false positive version checks, publishing failures, and secret configuration.

- **Enhanced Error Handling in publish-crate.mjs**: Added specific detection and helpful error messages for authentication failures, including guidance on secret configuration and workflow setup.

- **Case Study Documentation**: Added comprehensive case study at `docs/case-studies/issue-21/` analyzing CI/CD failures from browser-commander repository (issues #27, #29, #31, #33) with timeline, root causes, and lessons learned.

### Fixed

- **Prevent False Positive Version Checks**: The release workflow now correctly identifies unpublished versions by checking crates.io instead of relying on git tags, which can exist without the package being published.

### Changed

- Translated all CI/CD scripts from JavaScript (.mjs) to Rust (.rs) using rust-script
- Scripts now use native Rust with rust-script for execution in shell
- Removed Node.js dependency from CI/CD pipeline
- Updated GitHub Actions workflow to use rust-script instead of node
- Updated README and CONTRIBUTING documentation with new script references

### Added

- Cache `restore-keys` for partial cache hits across all workflow jobs
- Explicit `token` parameter in checkout for release jobs
- Code coverage job with `cargo-llvm-cov` and Codecov integration
- Codecov badge in README.md
- Pre-release version support (e.g., `0.1.0-beta.1`) in version parsing
- `--release-label` parameter for multi-language release disambiguation
- `ensure_version_exceeds_published()` logic to prevent publishing duplicate versions
- `get_max_published_version()` to query highest non-yanked version from crates.io
- `max_published_version` output from check-release-needed for downstream use
- Version fallback logic in auto-release Create GitHub Release step

### Changed

- Updated `actions/checkout` from v4 to v6
- Updated `actions/cache` from v4 to v5
- Updated `peter-evans/create-pull-request` from v7 to v8
- Made `publish-crate.rs` fail (exit 1) when version already exists on crates.io
- Improved `create-github-release.rs` to check combined stdout+stderr and detect "Validation Failed"

### Fixed

- Fix publish steps overriding workflow-level CARGO_TOKEN fallback, breaking CARGO_REGISTRY_TOKEN-only configurations (#32)
- Fix non-fast-forward push failures in multi-workflow repos by adding fetch/rebase and push retry logic (#31)
- Add mono-repo path support to check-changelog-fragment.rs, check-version-modification.rs, and create-changelog-fragment.rs
- Add `!cancelled()` guard to test job condition to respect workflow cancellation

### Fixed

- Fixed `version-and-commit.rs` to check crates.io instead of git tags for determining if a version is already released
- This prevents the release pipeline from getting stuck when git tags exist without corresponding crates.io publication

### Added

- Added `--tag-prefix` support to `version-and-commit.rs` for multi-language repository compatibility
- Added crates.io and docs.rs badges to README.md
- Added automatic crates.io and docs.rs badge injection in GitHub release notes
- Added documentation deployment job to CI/CD pipeline (deploys to GitHub Pages after release)
- Added case study documentation for issue #25

### Fixed
- Fixed unsupported look-ahead regex in `create-github-release.rs` that caused a panic when parsing CHANGELOG.md. Replaced with a two-step approach using only features supported by Rust's `regex` crate.

### Changed
- Restructured example application as a simple CLI sum calculator using `lino-arguments`
- Renamed default package to `example-sum-package-name` with Unlicense license
- Reorganized test structure: `tests/unit/sum.rs`, `tests/integration/sum.rs`, `tests/unit/ci-cd/`
- Converted experiment scripts into proper unit tests in `tests/unit/ci-cd/changelog_parsing.rs`
- Added CI/CD skip logic for template default package name `example-sum-package-name`
- Updated README.md badges and documentation

### Fixed

- Change detection script now uses per-commit diff instead of full PR diff, so commits touching only non-code files correctly skip CI jobs even when earlier commits in the same PR changed code files

### Fixed
- Make release scripts resolve the publishable crate manifest when the repository root uses a Cargo workspace manifest.

### Fixed
- Decoupled GitHub Pages documentation deployment from package release publication and fixed release-script warning failures under `RUSTFLAGS=-Dwarnings`.

### Changed
- Added explicit GitHub Actions job timeouts and documented Rust test timeout guidance.

### Fixed
- Added a non-blocking warning threshold to the Rust file-size check so near-limit files are surfaced before concurrent PR merges can exceed the hard limit.

### Fixed
- Made `create-github-release.rs` build GitHub release titles as `[Language] X.Y.Z` instead of reusing the tag prefix.

### Added
- Added optional Docker Hub image publishing tied to Rust crate releases, including crates.io visibility waiting, version/latest image tags, and Docker Hub badges in GitHub release notes.

### Changed
- Release completeness checks now self-heal when crates.io exists but configured Docker Hub or GitHub release artifacts are missing.

### Fixed
- Switched documentation deployment to the official GitHub Pages artifact workflow so repositories using GitHub Actions as their Pages source do not get false-positive branch-push deploys.

### Added
- Documented the one-time `Settings → Pages → Source = GitHub Actions` prerequisite for the `deploy-docs` job in `README.md` and as a comment above the `deploy-docs` job in `release.yml`, so downstream template users hit a documented setup step instead of a `Get Pages site failed` error on the first deploy.

### Added
- Tracking case study at `docs/case-studies/issue-52/` registering the `browser-commander` + Playwright preview-regeneration pattern from [`konard/vk-bot-desktop#52`](https://github.com/konard/vk-bot-desktop/pull/52), with an activation checklist for when an example-app surface lands in this template. Documentation only — no workflow, script, or runtime code changes. Primary upstream tracking issue: [`link-foundation/js-ai-driven-development-pipeline-template#62`](https://github.com/link-foundation/js-ai-driven-development-pipeline-template/issues/62).

### Fixed
- Release automation now keeps the workspace package entry in `Cargo.lock` synchronized when `scripts/version-and-commit.rs` bumps `Cargo.toml`, preventing stale lock-file version diffs in later pull requests.

### Fixed
- `scripts/publish-crate.rs` now treats crates.io HTTP 429 throttle responses ("You have published too many versions of this crate in the last 24 hours") as a deferred `publish_result=rate_limited` outcome (it writes the output, prints an explanatory banner and exits successfully) instead of a hard CI failure reported as a generic `failed` ("Failed to publish for unknown reason"). Authentication, already-published and unknown failures still exit non-zero. Failed-publish classification is consolidated through a single `classify_failure` function and `FailureKind` enum (with an `is_deferred` predicate), covered by unit tests runnable via `rust-script --test scripts/publish-crate.rs`.
- The release workflow (`.github/workflows/release.yml`) now gates crate-availability waiting, Docker Hub publishing and GitHub release creation on either an already-published crate or `publish_result=success`, so a deferred (rate-limited) crate upload no longer produces partial downstream release artifacts and the same version is retried automatically on the next push to `main`.

### Fixed
- Fixed reversed `cancel-in-progress` concurrency condition in `release.yml` that cancelled in-flight releases on `main` and never superseded older PR runs. The condition now uses `!=` so `main` releases run to completion while newer PR pushes cancel stale runs.

### Added
- Added a `scripts/check-crate-size.rs` guard that builds the `.crate` archive and fails the release before publishing when it exceeds the crates.io 10 MiB upload limit. The check runs in the build job and before publishing in both the auto-release and manual-release jobs.

### Changed
- Added a narrow `include` allowlist to `Cargo.toml` so docs, case studies, generated CI artifacts, changelog fragments, scripts, and experiments no longer inflate the published release archive.

### Fixed
- Prevented GitHub release creation from treating generic API validation failures as existing releases, and capped oversized release notes with a link to the full tagged changelog.

### Added
- Added the initial `meta-language` links-network core with self-description roots,
  verification, trivia attachment configuration, and a minimal CLI.
- Added the default lossless `parse` entry point, projection views, and a
  tested parity roadmap for competitor features, grammar embedding, and language
  coverage targets.
- Added exact reconstruction, mixed-region detection, query matching,
  substitution rules, concept reconstruction, object identity helpers,
  many-valued truth values, and executable parity fixtures for every tracked
  competitor target.
- Added executable language fixtures and tests for every requested Markdown,
  HTML, top-ten programming-language, and top-ten natural-language target.

### Fixed
- Added a CI guard that rejects Rust test modules and test attributes under `src/`, keeping tests in the `tests/` tree.

## Minor Changes

- Add immutable and mutable network snapshots with provenance and forward
  version commits for roadmap snapshot/versioning coverage.

### Added
- Added `txt` as a first-class markup/container target with a UTF-8 lossless
  fixture and content-sniffing fallback regions.

### Fixed
- Reconciled the natural-language target ordering with the Ethnologue 2025
  total-speaker order cited by the parity roadmap.

### Added
- Added a grammar-backed parser front end for Python, C, Java, C++, C#, JavaScript, and R using official tree-sitter grammar crates.

### Added
- Parse supported mixed-mode embedded regions into the host links network with grammar-backed syntax links.

## [0.22.0] - 2026-06-06

### Added
- Changeset-style fragment format with frontmatter for specifying version bump type
- New `get-bump-type.mjs` script to automatically determine version bump from fragments
- Automatic version bumping on merge to main based on changelog fragments
- Detailed documentation for the changelog fragment system in `changelog.d/README.md`

### Changed
- Updated `collect-changelog.mjs` to strip frontmatter when collecting fragments
- Updated `version-and-commit.mjs` to handle frontmatter in fragments
- Enhanced release workflow to automatically determine bump type from changesets

### Changed
- Add `detect-changes` job with cross-platform `detect-code-changes.mjs` script
- Make lint job independent of changelog check (runs based on file changes only)
- Allow docs-only PRs without changelog fragment requirement
- Handle changelog check 'skipped' state in dependent jobs
- Exclude `changelog.d/`, `docs/`, `experiments/`, `examples/` folders and markdown files from code changes detection

### Fixed
- Fixed README.md to correctly reference Node.js scripts (`.mjs`) instead of Python scripts (`.py`)
- Updated project structure in README.md to match actual script files in `scripts/` directory
- Fixed example code in README.md that had invalid Rust with two `main` functions

### Added

- Added crates.io publishing support to CI/CD workflow
- Added `release_mode` input with "instant" and "changelog-pr" options for manual releases
- Added `--tag-prefix` and `--crates-io-url` options to create-github-release.mjs script
- Added comprehensive case study documentation for Issue #11 in docs/case-studies/issue-11/

### Changed

- Changed changelog fragment check from warning to error (exit 1) to enforce changelog requirements
- Updated job conditions with `always() && !cancelled()` to fix workflow_dispatch job skipping issue
- Renamed manual-release job to "Instant Release" for clarity

### Fixed

- Fixed deprecated `::set-output` GitHub Actions command in version-and-commit.mjs
- Fixed workflow_dispatch triggering issues where lint/build/release jobs were incorrectly skipped

### Fixed

- Fixed changelog fragment check to validate that a fragment is **added in the PR diff** rather than just checking if any fragments exist in the directory. This prevents the check from incorrectly passing when there are leftover fragments from previous PRs that haven't been released yet.

### Changed

- Converted shell scripts in `release.yml` to cross-platform `.mjs` scripts for improved portability and performance:
  - `check-changelog-fragment.mjs` - validates changelog fragment is added in PR diff
  - `git-config.mjs` - configures git user for CI/CD
  - `check-release-needed.mjs` - checks if release is needed
  - `publish-crate.mjs` - publishes package to crates.io
  - `create-changelog-fragment.mjs` - creates changelog fragments for manual releases
  - `get-version.mjs` - gets current version from Cargo.toml

### Added

- Added `check-version-modification.mjs` script to detect manual version changes in Cargo.toml
- Added `version-check` job to CI/CD workflow that runs on pull requests
- Added skip logic for automated release branches (changelog-manual-release-*, changeset-release/*, release/*, automated-release/*)

### Changed

- Version modifications in Cargo.toml are now blocked in pull requests to enforce automated release pipeline

### Added

- Added support for `CARGO_REGISTRY_TOKEN` as alternative to `CARGO_TOKEN` for crates.io publishing
- Added case study documentation for Issue #17 (yargs reserved word and dual token support)

### Changed

- Updated workflow to use fallback logic: `${{ secrets.CARGO_REGISTRY_TOKEN || secrets.CARGO_TOKEN }}`
- Improved publish-crate.mjs to check both `CARGO_REGISTRY_TOKEN` and `CARGO_TOKEN` environment variables
- Added warning message when neither token is set

### Added
- New `scripts/rust-paths.mjs` utility for automatic Rust package root detection
- Support for both single-language and multi-language repository structures in all CI/CD scripts
- Configuration options via `--rust-root` CLI argument and `RUST_ROOT` environment variable
- Comprehensive case study documentation in `docs/case-studies/issue-19/`

### Changed
- Updated all release scripts to use the new path detection utility:
  - `scripts/bump-version.mjs`
  - `scripts/check-release-needed.mjs`
  - `scripts/collect-changelog.mjs`
  - `scripts/get-bump-type.mjs`
  - `scripts/get-version.mjs`
  - `scripts/publish-crate.mjs`
  - `scripts/version-and-commit.mjs`

### Changed

- **check-release-needed.mjs**: Now checks crates.io API directly instead of git tags to determine if a version is already released. This prevents false positives where git tags exist but the package was never actually published to crates.io.

### Added

- **CI/CD Troubleshooting Guide**: New documentation at `docs/ci-cd/troubleshooting.md` covering common issues like skipped jobs, false positive version checks, publishing failures, and secret configuration.

- **Enhanced Error Handling in publish-crate.mjs**: Added specific detection and helpful error messages for authentication failures, including guidance on secret configuration and workflow setup.

- **Case Study Documentation**: Added comprehensive case study at `docs/case-studies/issue-21/` analyzing CI/CD failures from browser-commander repository (issues #27, #29, #31, #33) with timeline, root causes, and lessons learned.

### Fixed

- **Prevent False Positive Version Checks**: The release workflow now correctly identifies unpublished versions by checking crates.io instead of relying on git tags, which can exist without the package being published.

### Changed

- Translated all CI/CD scripts from JavaScript (.mjs) to Rust (.rs) using rust-script
- Scripts now use native Rust with rust-script for execution in shell
- Removed Node.js dependency from CI/CD pipeline
- Updated GitHub Actions workflow to use rust-script instead of node
- Updated README and CONTRIBUTING documentation with new script references

### Added

- Cache `restore-keys` for partial cache hits across all workflow jobs
- Explicit `token` parameter in checkout for release jobs
- Code coverage job with `cargo-llvm-cov` and Codecov integration
- Codecov badge in README.md
- Pre-release version support (e.g., `0.1.0-beta.1`) in version parsing
- `--release-label` parameter for multi-language release disambiguation
- `ensure_version_exceeds_published()` logic to prevent publishing duplicate versions
- `get_max_published_version()` to query highest non-yanked version from crates.io
- `max_published_version` output from check-release-needed for downstream use
- Version fallback logic in auto-release Create GitHub Release step

### Changed

- Updated `actions/checkout` from v4 to v6
- Updated `actions/cache` from v4 to v5
- Updated `peter-evans/create-pull-request` from v7 to v8
- Made `publish-crate.rs` fail (exit 1) when version already exists on crates.io
- Improved `create-github-release.rs` to check combined stdout+stderr and detect "Validation Failed"

### Fixed

- Fix publish steps overriding workflow-level CARGO_TOKEN fallback, breaking CARGO_REGISTRY_TOKEN-only configurations (#32)
- Fix non-fast-forward push failures in multi-workflow repos by adding fetch/rebase and push retry logic (#31)
- Add mono-repo path support to check-changelog-fragment.rs, check-version-modification.rs, and create-changelog-fragment.rs
- Add `!cancelled()` guard to test job condition to respect workflow cancellation

### Fixed

- Fixed `version-and-commit.rs` to check crates.io instead of git tags for determining if a version is already released
- This prevents the release pipeline from getting stuck when git tags exist without corresponding crates.io publication

### Added

- Added `--tag-prefix` support to `version-and-commit.rs` for multi-language repository compatibility
- Added crates.io and docs.rs badges to README.md
- Added automatic crates.io and docs.rs badge injection in GitHub release notes
- Added documentation deployment job to CI/CD pipeline (deploys to GitHub Pages after release)
- Added case study documentation for issue #25

### Fixed
- Fixed unsupported look-ahead regex in `create-github-release.rs` that caused a panic when parsing CHANGELOG.md. Replaced with a two-step approach using only features supported by Rust's `regex` crate.

### Changed
- Restructured example application as a simple CLI sum calculator using `lino-arguments`
- Renamed default package to `example-sum-package-name` with Unlicense license
- Reorganized test structure: `tests/unit/sum.rs`, `tests/integration/sum.rs`, `tests/unit/ci-cd/`
- Converted experiment scripts into proper unit tests in `tests/unit/ci-cd/changelog_parsing.rs`
- Added CI/CD skip logic for template default package name `example-sum-package-name`
- Updated README.md badges and documentation

### Fixed

- Change detection script now uses per-commit diff instead of full PR diff, so commits touching only non-code files correctly skip CI jobs even when earlier commits in the same PR changed code files

### Fixed
- Make release scripts resolve the publishable crate manifest when the repository root uses a Cargo workspace manifest.

### Fixed
- Decoupled GitHub Pages documentation deployment from package release publication and fixed release-script warning failures under `RUSTFLAGS=-Dwarnings`.

### Changed
- Added explicit GitHub Actions job timeouts and documented Rust test timeout guidance.

### Fixed
- Added a non-blocking warning threshold to the Rust file-size check so near-limit files are surfaced before concurrent PR merges can exceed the hard limit.

### Fixed
- Made `create-github-release.rs` build GitHub release titles as `[Language] X.Y.Z` instead of reusing the tag prefix.

### Added
- Added optional Docker Hub image publishing tied to Rust crate releases, including crates.io visibility waiting, version/latest image tags, and Docker Hub badges in GitHub release notes.

### Changed
- Release completeness checks now self-heal when crates.io exists but configured Docker Hub or GitHub release artifacts are missing.

### Fixed
- Switched documentation deployment to the official GitHub Pages artifact workflow so repositories using GitHub Actions as their Pages source do not get false-positive branch-push deploys.

### Added
- Documented the one-time `Settings → Pages → Source = GitHub Actions` prerequisite for the `deploy-docs` job in `README.md` and as a comment above the `deploy-docs` job in `release.yml`, so downstream template users hit a documented setup step instead of a `Get Pages site failed` error on the first deploy.

### Added
- Tracking case study at `docs/case-studies/issue-52/` registering the `browser-commander` + Playwright preview-regeneration pattern from [`konard/vk-bot-desktop#52`](https://github.com/konard/vk-bot-desktop/pull/52), with an activation checklist for when an example-app surface lands in this template. Documentation only — no workflow, script, or runtime code changes. Primary upstream tracking issue: [`link-foundation/js-ai-driven-development-pipeline-template#62`](https://github.com/link-foundation/js-ai-driven-development-pipeline-template/issues/62).

### Fixed
- Release automation now keeps the workspace package entry in `Cargo.lock` synchronized when `scripts/version-and-commit.rs` bumps `Cargo.toml`, preventing stale lock-file version diffs in later pull requests.

### Fixed
- `scripts/publish-crate.rs` now treats crates.io HTTP 429 throttle responses ("You have published too many versions of this crate in the last 24 hours") as a deferred `publish_result=rate_limited` outcome (it writes the output, prints an explanatory banner and exits successfully) instead of a hard CI failure reported as a generic `failed` ("Failed to publish for unknown reason"). Authentication, already-published and unknown failures still exit non-zero. Failed-publish classification is consolidated through a single `classify_failure` function and `FailureKind` enum (with an `is_deferred` predicate), covered by unit tests runnable via `rust-script --test scripts/publish-crate.rs`.
- The release workflow (`.github/workflows/release.yml`) now gates crate-availability waiting, Docker Hub publishing and GitHub release creation on either an already-published crate or `publish_result=success`, so a deferred (rate-limited) crate upload no longer produces partial downstream release artifacts and the same version is retried automatically on the next push to `main`.

### Fixed
- Fixed reversed `cancel-in-progress` concurrency condition in `release.yml` that cancelled in-flight releases on `main` and never superseded older PR runs. The condition now uses `!=` so `main` releases run to completion while newer PR pushes cancel stale runs.

### Added
- Added a `scripts/check-crate-size.rs` guard that builds the `.crate` archive and fails the release before publishing when it exceeds the crates.io 10 MiB upload limit. The check runs in the build job and before publishing in both the auto-release and manual-release jobs.

### Changed
- Added a narrow `include` allowlist to `Cargo.toml` so docs, case studies, generated CI artifacts, changelog fragments, scripts, and experiments no longer inflate the published release archive.

### Fixed
- Prevented GitHub release creation from treating generic API validation failures as existing releases, and capped oversized release notes with a link to the full tagged changelog.

### Added
- Added the initial `meta-language` links-network core with self-description roots,
  verification, trivia attachment configuration, and a minimal CLI.
- Added the default lossless `parse` entry point, projection views, and a
  tested parity roadmap for competitor features, grammar embedding, and language
  coverage targets.
- Added exact reconstruction, mixed-region detection, query matching,
  substitution rules, concept reconstruction, object identity helpers,
  many-valued truth values, and executable parity fixtures for every tracked
  competitor target.
- Added executable language fixtures and tests for every requested Markdown,
  HTML, top-ten programming-language, and top-ten natural-language target.

### Fixed
- Added a CI guard that rejects Rust test modules and test attributes under `src/`, keeping tests in the `tests/` tree.

## Minor Changes

- Add immutable and mutable network snapshots with provenance and forward
  version commits for roadmap snapshot/versioning coverage.

### Added
- Added `txt` as a first-class markup/container target with a UTF-8 lossless
  fixture and content-sniffing fallback regions.

### Fixed
- Reconciled the natural-language target ordering with the Ethnologue 2025
  total-speaker order cited by the parity roadmap.

### Added
- Added a grammar-backed parser front end for Python, C, Java, C++, C#, JavaScript, and R using official tree-sitter grammar crates.

## [0.21.0] - 2026-06-06

### Added
- Changeset-style fragment format with frontmatter for specifying version bump type
- New `get-bump-type.mjs` script to automatically determine version bump from fragments
- Automatic version bumping on merge to main based on changelog fragments
- Detailed documentation for the changelog fragment system in `changelog.d/README.md`

### Changed
- Updated `collect-changelog.mjs` to strip frontmatter when collecting fragments
- Updated `version-and-commit.mjs` to handle frontmatter in fragments
- Enhanced release workflow to automatically determine bump type from changesets

### Changed
- Add `detect-changes` job with cross-platform `detect-code-changes.mjs` script
- Make lint job independent of changelog check (runs based on file changes only)
- Allow docs-only PRs without changelog fragment requirement
- Handle changelog check 'skipped' state in dependent jobs
- Exclude `changelog.d/`, `docs/`, `experiments/`, `examples/` folders and markdown files from code changes detection

### Fixed
- Fixed README.md to correctly reference Node.js scripts (`.mjs`) instead of Python scripts (`.py`)
- Updated project structure in README.md to match actual script files in `scripts/` directory
- Fixed example code in README.md that had invalid Rust with two `main` functions

### Added

- Added crates.io publishing support to CI/CD workflow
- Added `release_mode` input with "instant" and "changelog-pr" options for manual releases
- Added `--tag-prefix` and `--crates-io-url` options to create-github-release.mjs script
- Added comprehensive case study documentation for Issue #11 in docs/case-studies/issue-11/

### Changed

- Changed changelog fragment check from warning to error (exit 1) to enforce changelog requirements
- Updated job conditions with `always() && !cancelled()` to fix workflow_dispatch job skipping issue
- Renamed manual-release job to "Instant Release" for clarity

### Fixed

- Fixed deprecated `::set-output` GitHub Actions command in version-and-commit.mjs
- Fixed workflow_dispatch triggering issues where lint/build/release jobs were incorrectly skipped

### Fixed

- Fixed changelog fragment check to validate that a fragment is **added in the PR diff** rather than just checking if any fragments exist in the directory. This prevents the check from incorrectly passing when there are leftover fragments from previous PRs that haven't been released yet.

### Changed

- Converted shell scripts in `release.yml` to cross-platform `.mjs` scripts for improved portability and performance:
  - `check-changelog-fragment.mjs` - validates changelog fragment is added in PR diff
  - `git-config.mjs` - configures git user for CI/CD
  - `check-release-needed.mjs` - checks if release is needed
  - `publish-crate.mjs` - publishes package to crates.io
  - `create-changelog-fragment.mjs` - creates changelog fragments for manual releases
  - `get-version.mjs` - gets current version from Cargo.toml

### Added

- Added `check-version-modification.mjs` script to detect manual version changes in Cargo.toml
- Added `version-check` job to CI/CD workflow that runs on pull requests
- Added skip logic for automated release branches (changelog-manual-release-*, changeset-release/*, release/*, automated-release/*)

### Changed

- Version modifications in Cargo.toml are now blocked in pull requests to enforce automated release pipeline

### Added

- Added support for `CARGO_REGISTRY_TOKEN` as alternative to `CARGO_TOKEN` for crates.io publishing
- Added case study documentation for Issue #17 (yargs reserved word and dual token support)

### Changed

- Updated workflow to use fallback logic: `${{ secrets.CARGO_REGISTRY_TOKEN || secrets.CARGO_TOKEN }}`
- Improved publish-crate.mjs to check both `CARGO_REGISTRY_TOKEN` and `CARGO_TOKEN` environment variables
- Added warning message when neither token is set

### Added
- New `scripts/rust-paths.mjs` utility for automatic Rust package root detection
- Support for both single-language and multi-language repository structures in all CI/CD scripts
- Configuration options via `--rust-root` CLI argument and `RUST_ROOT` environment variable
- Comprehensive case study documentation in `docs/case-studies/issue-19/`

### Changed
- Updated all release scripts to use the new path detection utility:
  - `scripts/bump-version.mjs`
  - `scripts/check-release-needed.mjs`
  - `scripts/collect-changelog.mjs`
  - `scripts/get-bump-type.mjs`
  - `scripts/get-version.mjs`
  - `scripts/publish-crate.mjs`
  - `scripts/version-and-commit.mjs`

### Changed

- **check-release-needed.mjs**: Now checks crates.io API directly instead of git tags to determine if a version is already released. This prevents false positives where git tags exist but the package was never actually published to crates.io.

### Added

- **CI/CD Troubleshooting Guide**: New documentation at `docs/ci-cd/troubleshooting.md` covering common issues like skipped jobs, false positive version checks, publishing failures, and secret configuration.

- **Enhanced Error Handling in publish-crate.mjs**: Added specific detection and helpful error messages for authentication failures, including guidance on secret configuration and workflow setup.

- **Case Study Documentation**: Added comprehensive case study at `docs/case-studies/issue-21/` analyzing CI/CD failures from browser-commander repository (issues #27, #29, #31, #33) with timeline, root causes, and lessons learned.

### Fixed

- **Prevent False Positive Version Checks**: The release workflow now correctly identifies unpublished versions by checking crates.io instead of relying on git tags, which can exist without the package being published.

### Changed

- Translated all CI/CD scripts from JavaScript (.mjs) to Rust (.rs) using rust-script
- Scripts now use native Rust with rust-script for execution in shell
- Removed Node.js dependency from CI/CD pipeline
- Updated GitHub Actions workflow to use rust-script instead of node
- Updated README and CONTRIBUTING documentation with new script references

### Added

- Cache `restore-keys` for partial cache hits across all workflow jobs
- Explicit `token` parameter in checkout for release jobs
- Code coverage job with `cargo-llvm-cov` and Codecov integration
- Codecov badge in README.md
- Pre-release version support (e.g., `0.1.0-beta.1`) in version parsing
- `--release-label` parameter for multi-language release disambiguation
- `ensure_version_exceeds_published()` logic to prevent publishing duplicate versions
- `get_max_published_version()` to query highest non-yanked version from crates.io
- `max_published_version` output from check-release-needed for downstream use
- Version fallback logic in auto-release Create GitHub Release step

### Changed

- Updated `actions/checkout` from v4 to v6
- Updated `actions/cache` from v4 to v5
- Updated `peter-evans/create-pull-request` from v7 to v8
- Made `publish-crate.rs` fail (exit 1) when version already exists on crates.io
- Improved `create-github-release.rs` to check combined stdout+stderr and detect "Validation Failed"

### Fixed

- Fix publish steps overriding workflow-level CARGO_TOKEN fallback, breaking CARGO_REGISTRY_TOKEN-only configurations (#32)
- Fix non-fast-forward push failures in multi-workflow repos by adding fetch/rebase and push retry logic (#31)
- Add mono-repo path support to check-changelog-fragment.rs, check-version-modification.rs, and create-changelog-fragment.rs
- Add `!cancelled()` guard to test job condition to respect workflow cancellation

### Fixed

- Fixed `version-and-commit.rs` to check crates.io instead of git tags for determining if a version is already released
- This prevents the release pipeline from getting stuck when git tags exist without corresponding crates.io publication

### Added

- Added `--tag-prefix` support to `version-and-commit.rs` for multi-language repository compatibility
- Added crates.io and docs.rs badges to README.md
- Added automatic crates.io and docs.rs badge injection in GitHub release notes
- Added documentation deployment job to CI/CD pipeline (deploys to GitHub Pages after release)
- Added case study documentation for issue #25

### Fixed
- Fixed unsupported look-ahead regex in `create-github-release.rs` that caused a panic when parsing CHANGELOG.md. Replaced with a two-step approach using only features supported by Rust's `regex` crate.

### Changed
- Restructured example application as a simple CLI sum calculator using `lino-arguments`
- Renamed default package to `example-sum-package-name` with Unlicense license
- Reorganized test structure: `tests/unit/sum.rs`, `tests/integration/sum.rs`, `tests/unit/ci-cd/`
- Converted experiment scripts into proper unit tests in `tests/unit/ci-cd/changelog_parsing.rs`
- Added CI/CD skip logic for template default package name `example-sum-package-name`
- Updated README.md badges and documentation

### Fixed

- Change detection script now uses per-commit diff instead of full PR diff, so commits touching only non-code files correctly skip CI jobs even when earlier commits in the same PR changed code files

### Fixed
- Make release scripts resolve the publishable crate manifest when the repository root uses a Cargo workspace manifest.

### Fixed
- Decoupled GitHub Pages documentation deployment from package release publication and fixed release-script warning failures under `RUSTFLAGS=-Dwarnings`.

### Changed
- Added explicit GitHub Actions job timeouts and documented Rust test timeout guidance.

### Fixed
- Added a non-blocking warning threshold to the Rust file-size check so near-limit files are surfaced before concurrent PR merges can exceed the hard limit.

### Fixed
- Made `create-github-release.rs` build GitHub release titles as `[Language] X.Y.Z` instead of reusing the tag prefix.

### Added
- Added optional Docker Hub image publishing tied to Rust crate releases, including crates.io visibility waiting, version/latest image tags, and Docker Hub badges in GitHub release notes.

### Changed
- Release completeness checks now self-heal when crates.io exists but configured Docker Hub or GitHub release artifacts are missing.

### Fixed
- Switched documentation deployment to the official GitHub Pages artifact workflow so repositories using GitHub Actions as their Pages source do not get false-positive branch-push deploys.

### Added
- Documented the one-time `Settings → Pages → Source = GitHub Actions` prerequisite for the `deploy-docs` job in `README.md` and as a comment above the `deploy-docs` job in `release.yml`, so downstream template users hit a documented setup step instead of a `Get Pages site failed` error on the first deploy.

### Added
- Tracking case study at `docs/case-studies/issue-52/` registering the `browser-commander` + Playwright preview-regeneration pattern from [`konard/vk-bot-desktop#52`](https://github.com/konard/vk-bot-desktop/pull/52), with an activation checklist for when an example-app surface lands in this template. Documentation only — no workflow, script, or runtime code changes. Primary upstream tracking issue: [`link-foundation/js-ai-driven-development-pipeline-template#62`](https://github.com/link-foundation/js-ai-driven-development-pipeline-template/issues/62).

### Fixed
- Release automation now keeps the workspace package entry in `Cargo.lock` synchronized when `scripts/version-and-commit.rs` bumps `Cargo.toml`, preventing stale lock-file version diffs in later pull requests.

### Fixed
- `scripts/publish-crate.rs` now treats crates.io HTTP 429 throttle responses ("You have published too many versions of this crate in the last 24 hours") as a deferred `publish_result=rate_limited` outcome (it writes the output, prints an explanatory banner and exits successfully) instead of a hard CI failure reported as a generic `failed` ("Failed to publish for unknown reason"). Authentication, already-published and unknown failures still exit non-zero. Failed-publish classification is consolidated through a single `classify_failure` function and `FailureKind` enum (with an `is_deferred` predicate), covered by unit tests runnable via `rust-script --test scripts/publish-crate.rs`.
- The release workflow (`.github/workflows/release.yml`) now gates crate-availability waiting, Docker Hub publishing and GitHub release creation on either an already-published crate or `publish_result=success`, so a deferred (rate-limited) crate upload no longer produces partial downstream release artifacts and the same version is retried automatically on the next push to `main`.

### Fixed
- Fixed reversed `cancel-in-progress` concurrency condition in `release.yml` that cancelled in-flight releases on `main` and never superseded older PR runs. The condition now uses `!=` so `main` releases run to completion while newer PR pushes cancel stale runs.

### Added
- Added a `scripts/check-crate-size.rs` guard that builds the `.crate` archive and fails the release before publishing when it exceeds the crates.io 10 MiB upload limit. The check runs in the build job and before publishing in both the auto-release and manual-release jobs.

### Changed
- Added a narrow `include` allowlist to `Cargo.toml` so docs, case studies, generated CI artifacts, changelog fragments, scripts, and experiments no longer inflate the published release archive.

### Fixed
- Prevented GitHub release creation from treating generic API validation failures as existing releases, and capped oversized release notes with a link to the full tagged changelog.

### Added
- Added the initial `meta-language` links-network core with self-description roots,
  verification, trivia attachment configuration, and a minimal CLI.
- Added the default lossless `parse` entry point, projection views, and a
  tested parity roadmap for competitor features, grammar embedding, and language
  coverage targets.
- Added exact reconstruction, mixed-region detection, query matching,
  substitution rules, concept reconstruction, object identity helpers,
  many-valued truth values, and executable parity fixtures for every tracked
  competitor target.
- Added executable language fixtures and tests for every requested Markdown,
  HTML, top-ten programming-language, and top-ten natural-language target.

### Fixed
- Added a CI guard that rejects Rust test modules and test attributes under `src/`, keeping tests in the `tests/` tree.

## Minor Changes

- Add immutable and mutable network snapshots with provenance and forward
  version commits for roadmap snapshot/versioning coverage.

### Added
- Added `txt` as a first-class markup/container target with a UTF-8 lossless
  fixture and content-sniffing fallback regions.

### Fixed
- Reconciled the natural-language target ordering with the Ethnologue 2025
  total-speaker order cited by the parity roadmap.

## [0.20.0] - 2026-06-06

### Added
- Changeset-style fragment format with frontmatter for specifying version bump type
- New `get-bump-type.mjs` script to automatically determine version bump from fragments
- Automatic version bumping on merge to main based on changelog fragments
- Detailed documentation for the changelog fragment system in `changelog.d/README.md`

### Changed
- Updated `collect-changelog.mjs` to strip frontmatter when collecting fragments
- Updated `version-and-commit.mjs` to handle frontmatter in fragments
- Enhanced release workflow to automatically determine bump type from changesets

### Changed
- Add `detect-changes` job with cross-platform `detect-code-changes.mjs` script
- Make lint job independent of changelog check (runs based on file changes only)
- Allow docs-only PRs without changelog fragment requirement
- Handle changelog check 'skipped' state in dependent jobs
- Exclude `changelog.d/`, `docs/`, `experiments/`, `examples/` folders and markdown files from code changes detection

### Fixed
- Fixed README.md to correctly reference Node.js scripts (`.mjs`) instead of Python scripts (`.py`)
- Updated project structure in README.md to match actual script files in `scripts/` directory
- Fixed example code in README.md that had invalid Rust with two `main` functions

### Added

- Added crates.io publishing support to CI/CD workflow
- Added `release_mode` input with "instant" and "changelog-pr" options for manual releases
- Added `--tag-prefix` and `--crates-io-url` options to create-github-release.mjs script
- Added comprehensive case study documentation for Issue #11 in docs/case-studies/issue-11/

### Changed

- Changed changelog fragment check from warning to error (exit 1) to enforce changelog requirements
- Updated job conditions with `always() && !cancelled()` to fix workflow_dispatch job skipping issue
- Renamed manual-release job to "Instant Release" for clarity

### Fixed

- Fixed deprecated `::set-output` GitHub Actions command in version-and-commit.mjs
- Fixed workflow_dispatch triggering issues where lint/build/release jobs were incorrectly skipped

### Fixed

- Fixed changelog fragment check to validate that a fragment is **added in the PR diff** rather than just checking if any fragments exist in the directory. This prevents the check from incorrectly passing when there are leftover fragments from previous PRs that haven't been released yet.

### Changed

- Converted shell scripts in `release.yml` to cross-platform `.mjs` scripts for improved portability and performance:
  - `check-changelog-fragment.mjs` - validates changelog fragment is added in PR diff
  - `git-config.mjs` - configures git user for CI/CD
  - `check-release-needed.mjs` - checks if release is needed
  - `publish-crate.mjs` - publishes package to crates.io
  - `create-changelog-fragment.mjs` - creates changelog fragments for manual releases
  - `get-version.mjs` - gets current version from Cargo.toml

### Added

- Added `check-version-modification.mjs` script to detect manual version changes in Cargo.toml
- Added `version-check` job to CI/CD workflow that runs on pull requests
- Added skip logic for automated release branches (changelog-manual-release-*, changeset-release/*, release/*, automated-release/*)

### Changed

- Version modifications in Cargo.toml are now blocked in pull requests to enforce automated release pipeline

### Added

- Added support for `CARGO_REGISTRY_TOKEN` as alternative to `CARGO_TOKEN` for crates.io publishing
- Added case study documentation for Issue #17 (yargs reserved word and dual token support)

### Changed

- Updated workflow to use fallback logic: `${{ secrets.CARGO_REGISTRY_TOKEN || secrets.CARGO_TOKEN }}`
- Improved publish-crate.mjs to check both `CARGO_REGISTRY_TOKEN` and `CARGO_TOKEN` environment variables
- Added warning message when neither token is set

### Added
- New `scripts/rust-paths.mjs` utility for automatic Rust package root detection
- Support for both single-language and multi-language repository structures in all CI/CD scripts
- Configuration options via `--rust-root` CLI argument and `RUST_ROOT` environment variable
- Comprehensive case study documentation in `docs/case-studies/issue-19/`

### Changed
- Updated all release scripts to use the new path detection utility:
  - `scripts/bump-version.mjs`
  - `scripts/check-release-needed.mjs`
  - `scripts/collect-changelog.mjs`
  - `scripts/get-bump-type.mjs`
  - `scripts/get-version.mjs`
  - `scripts/publish-crate.mjs`
  - `scripts/version-and-commit.mjs`

### Changed

- **check-release-needed.mjs**: Now checks crates.io API directly instead of git tags to determine if a version is already released. This prevents false positives where git tags exist but the package was never actually published to crates.io.

### Added

- **CI/CD Troubleshooting Guide**: New documentation at `docs/ci-cd/troubleshooting.md` covering common issues like skipped jobs, false positive version checks, publishing failures, and secret configuration.

- **Enhanced Error Handling in publish-crate.mjs**: Added specific detection and helpful error messages for authentication failures, including guidance on secret configuration and workflow setup.

- **Case Study Documentation**: Added comprehensive case study at `docs/case-studies/issue-21/` analyzing CI/CD failures from browser-commander repository (issues #27, #29, #31, #33) with timeline, root causes, and lessons learned.

### Fixed

- **Prevent False Positive Version Checks**: The release workflow now correctly identifies unpublished versions by checking crates.io instead of relying on git tags, which can exist without the package being published.

### Changed

- Translated all CI/CD scripts from JavaScript (.mjs) to Rust (.rs) using rust-script
- Scripts now use native Rust with rust-script for execution in shell
- Removed Node.js dependency from CI/CD pipeline
- Updated GitHub Actions workflow to use rust-script instead of node
- Updated README and CONTRIBUTING documentation with new script references

### Added

- Cache `restore-keys` for partial cache hits across all workflow jobs
- Explicit `token` parameter in checkout for release jobs
- Code coverage job with `cargo-llvm-cov` and Codecov integration
- Codecov badge in README.md
- Pre-release version support (e.g., `0.1.0-beta.1`) in version parsing
- `--release-label` parameter for multi-language release disambiguation
- `ensure_version_exceeds_published()` logic to prevent publishing duplicate versions
- `get_max_published_version()` to query highest non-yanked version from crates.io
- `max_published_version` output from check-release-needed for downstream use
- Version fallback logic in auto-release Create GitHub Release step

### Changed

- Updated `actions/checkout` from v4 to v6
- Updated `actions/cache` from v4 to v5
- Updated `peter-evans/create-pull-request` from v7 to v8
- Made `publish-crate.rs` fail (exit 1) when version already exists on crates.io
- Improved `create-github-release.rs` to check combined stdout+stderr and detect "Validation Failed"

### Fixed

- Fix publish steps overriding workflow-level CARGO_TOKEN fallback, breaking CARGO_REGISTRY_TOKEN-only configurations (#32)
- Fix non-fast-forward push failures in multi-workflow repos by adding fetch/rebase and push retry logic (#31)
- Add mono-repo path support to check-changelog-fragment.rs, check-version-modification.rs, and create-changelog-fragment.rs
- Add `!cancelled()` guard to test job condition to respect workflow cancellation

### Fixed

- Fixed `version-and-commit.rs` to check crates.io instead of git tags for determining if a version is already released
- This prevents the release pipeline from getting stuck when git tags exist without corresponding crates.io publication

### Added

- Added `--tag-prefix` support to `version-and-commit.rs` for multi-language repository compatibility
- Added crates.io and docs.rs badges to README.md
- Added automatic crates.io and docs.rs badge injection in GitHub release notes
- Added documentation deployment job to CI/CD pipeline (deploys to GitHub Pages after release)
- Added case study documentation for issue #25

### Fixed
- Fixed unsupported look-ahead regex in `create-github-release.rs` that caused a panic when parsing CHANGELOG.md. Replaced with a two-step approach using only features supported by Rust's `regex` crate.

### Changed
- Restructured example application as a simple CLI sum calculator using `lino-arguments`
- Renamed default package to `example-sum-package-name` with Unlicense license
- Reorganized test structure: `tests/unit/sum.rs`, `tests/integration/sum.rs`, `tests/unit/ci-cd/`
- Converted experiment scripts into proper unit tests in `tests/unit/ci-cd/changelog_parsing.rs`
- Added CI/CD skip logic for template default package name `example-sum-package-name`
- Updated README.md badges and documentation

### Fixed

- Change detection script now uses per-commit diff instead of full PR diff, so commits touching only non-code files correctly skip CI jobs even when earlier commits in the same PR changed code files

### Fixed
- Make release scripts resolve the publishable crate manifest when the repository root uses a Cargo workspace manifest.

### Fixed
- Decoupled GitHub Pages documentation deployment from package release publication and fixed release-script warning failures under `RUSTFLAGS=-Dwarnings`.

### Changed
- Added explicit GitHub Actions job timeouts and documented Rust test timeout guidance.

### Fixed
- Added a non-blocking warning threshold to the Rust file-size check so near-limit files are surfaced before concurrent PR merges can exceed the hard limit.

### Fixed
- Made `create-github-release.rs` build GitHub release titles as `[Language] X.Y.Z` instead of reusing the tag prefix.

### Added
- Added optional Docker Hub image publishing tied to Rust crate releases, including crates.io visibility waiting, version/latest image tags, and Docker Hub badges in GitHub release notes.

### Changed
- Release completeness checks now self-heal when crates.io exists but configured Docker Hub or GitHub release artifacts are missing.

### Fixed
- Switched documentation deployment to the official GitHub Pages artifact workflow so repositories using GitHub Actions as their Pages source do not get false-positive branch-push deploys.

### Added
- Documented the one-time `Settings → Pages → Source = GitHub Actions` prerequisite for the `deploy-docs` job in `README.md` and as a comment above the `deploy-docs` job in `release.yml`, so downstream template users hit a documented setup step instead of a `Get Pages site failed` error on the first deploy.

### Added
- Tracking case study at `docs/case-studies/issue-52/` registering the `browser-commander` + Playwright preview-regeneration pattern from [`konard/vk-bot-desktop#52`](https://github.com/konard/vk-bot-desktop/pull/52), with an activation checklist for when an example-app surface lands in this template. Documentation only — no workflow, script, or runtime code changes. Primary upstream tracking issue: [`link-foundation/js-ai-driven-development-pipeline-template#62`](https://github.com/link-foundation/js-ai-driven-development-pipeline-template/issues/62).

### Fixed
- Release automation now keeps the workspace package entry in `Cargo.lock` synchronized when `scripts/version-and-commit.rs` bumps `Cargo.toml`, preventing stale lock-file version diffs in later pull requests.

### Fixed
- `scripts/publish-crate.rs` now treats crates.io HTTP 429 throttle responses ("You have published too many versions of this crate in the last 24 hours") as a deferred `publish_result=rate_limited` outcome (it writes the output, prints an explanatory banner and exits successfully) instead of a hard CI failure reported as a generic `failed` ("Failed to publish for unknown reason"). Authentication, already-published and unknown failures still exit non-zero. Failed-publish classification is consolidated through a single `classify_failure` function and `FailureKind` enum (with an `is_deferred` predicate), covered by unit tests runnable via `rust-script --test scripts/publish-crate.rs`.
- The release workflow (`.github/workflows/release.yml`) now gates crate-availability waiting, Docker Hub publishing and GitHub release creation on either an already-published crate or `publish_result=success`, so a deferred (rate-limited) crate upload no longer produces partial downstream release artifacts and the same version is retried automatically on the next push to `main`.

### Fixed
- Fixed reversed `cancel-in-progress` concurrency condition in `release.yml` that cancelled in-flight releases on `main` and never superseded older PR runs. The condition now uses `!=` so `main` releases run to completion while newer PR pushes cancel stale runs.

### Added
- Added a `scripts/check-crate-size.rs` guard that builds the `.crate` archive and fails the release before publishing when it exceeds the crates.io 10 MiB upload limit. The check runs in the build job and before publishing in both the auto-release and manual-release jobs.

### Changed
- Added a narrow `include` allowlist to `Cargo.toml` so docs, case studies, generated CI artifacts, changelog fragments, scripts, and experiments no longer inflate the published release archive.

### Fixed
- Prevented GitHub release creation from treating generic API validation failures as existing releases, and capped oversized release notes with a link to the full tagged changelog.

### Added
- Added the initial `meta-language` links-network core with self-description roots,
  verification, trivia attachment configuration, and a minimal CLI.
- Added the default lossless `parse` entry point, projection views, and a
  tested parity roadmap for competitor features, grammar embedding, and language
  coverage targets.
- Added exact reconstruction, mixed-region detection, query matching,
  substitution rules, concept reconstruction, object identity helpers,
  many-valued truth values, and executable parity fixtures for every tracked
  competitor target.
- Added executable language fixtures and tests for every requested Markdown,
  HTML, top-ten programming-language, and top-ten natural-language target.

### Fixed
- Added a CI guard that rejects Rust test modules and test attributes under `src/`, keeping tests in the `tests/` tree.

## Minor Changes

- Add immutable and mutable network snapshots with provenance and forward
  version commits for roadmap snapshot/versioning coverage.

### Added
- Added `txt` as a first-class markup/container target with a UTF-8 lossless
  fixture and content-sniffing fallback regions.

## [0.19.0] - 2026-06-05

### Added
- Changeset-style fragment format with frontmatter for specifying version bump type
- New `get-bump-type.mjs` script to automatically determine version bump from fragments
- Automatic version bumping on merge to main based on changelog fragments
- Detailed documentation for the changelog fragment system in `changelog.d/README.md`

### Changed
- Updated `collect-changelog.mjs` to strip frontmatter when collecting fragments
- Updated `version-and-commit.mjs` to handle frontmatter in fragments
- Enhanced release workflow to automatically determine bump type from changesets

### Changed
- Add `detect-changes` job with cross-platform `detect-code-changes.mjs` script
- Make lint job independent of changelog check (runs based on file changes only)
- Allow docs-only PRs without changelog fragment requirement
- Handle changelog check 'skipped' state in dependent jobs
- Exclude `changelog.d/`, `docs/`, `experiments/`, `examples/` folders and markdown files from code changes detection

### Fixed
- Fixed README.md to correctly reference Node.js scripts (`.mjs`) instead of Python scripts (`.py`)
- Updated project structure in README.md to match actual script files in `scripts/` directory
- Fixed example code in README.md that had invalid Rust with two `main` functions

### Added

- Added crates.io publishing support to CI/CD workflow
- Added `release_mode` input with "instant" and "changelog-pr" options for manual releases
- Added `--tag-prefix` and `--crates-io-url` options to create-github-release.mjs script
- Added comprehensive case study documentation for Issue #11 in docs/case-studies/issue-11/

### Changed

- Changed changelog fragment check from warning to error (exit 1) to enforce changelog requirements
- Updated job conditions with `always() && !cancelled()` to fix workflow_dispatch job skipping issue
- Renamed manual-release job to "Instant Release" for clarity

### Fixed

- Fixed deprecated `::set-output` GitHub Actions command in version-and-commit.mjs
- Fixed workflow_dispatch triggering issues where lint/build/release jobs were incorrectly skipped

### Fixed

- Fixed changelog fragment check to validate that a fragment is **added in the PR diff** rather than just checking if any fragments exist in the directory. This prevents the check from incorrectly passing when there are leftover fragments from previous PRs that haven't been released yet.

### Changed

- Converted shell scripts in `release.yml` to cross-platform `.mjs` scripts for improved portability and performance:
  - `check-changelog-fragment.mjs` - validates changelog fragment is added in PR diff
  - `git-config.mjs` - configures git user for CI/CD
  - `check-release-needed.mjs` - checks if release is needed
  - `publish-crate.mjs` - publishes package to crates.io
  - `create-changelog-fragment.mjs` - creates changelog fragments for manual releases
  - `get-version.mjs` - gets current version from Cargo.toml

### Added

- Added `check-version-modification.mjs` script to detect manual version changes in Cargo.toml
- Added `version-check` job to CI/CD workflow that runs on pull requests
- Added skip logic for automated release branches (changelog-manual-release-*, changeset-release/*, release/*, automated-release/*)

### Changed

- Version modifications in Cargo.toml are now blocked in pull requests to enforce automated release pipeline

### Added

- Added support for `CARGO_REGISTRY_TOKEN` as alternative to `CARGO_TOKEN` for crates.io publishing
- Added case study documentation for Issue #17 (yargs reserved word and dual token support)

### Changed

- Updated workflow to use fallback logic: `${{ secrets.CARGO_REGISTRY_TOKEN || secrets.CARGO_TOKEN }}`
- Improved publish-crate.mjs to check both `CARGO_REGISTRY_TOKEN` and `CARGO_TOKEN` environment variables
- Added warning message when neither token is set

### Added
- New `scripts/rust-paths.mjs` utility for automatic Rust package root detection
- Support for both single-language and multi-language repository structures in all CI/CD scripts
- Configuration options via `--rust-root` CLI argument and `RUST_ROOT` environment variable
- Comprehensive case study documentation in `docs/case-studies/issue-19/`

### Changed
- Updated all release scripts to use the new path detection utility:
  - `scripts/bump-version.mjs`
  - `scripts/check-release-needed.mjs`
  - `scripts/collect-changelog.mjs`
  - `scripts/get-bump-type.mjs`
  - `scripts/get-version.mjs`
  - `scripts/publish-crate.mjs`
  - `scripts/version-and-commit.mjs`

### Changed

- **check-release-needed.mjs**: Now checks crates.io API directly instead of git tags to determine if a version is already released. This prevents false positives where git tags exist but the package was never actually published to crates.io.

### Added

- **CI/CD Troubleshooting Guide**: New documentation at `docs/ci-cd/troubleshooting.md` covering common issues like skipped jobs, false positive version checks, publishing failures, and secret configuration.

- **Enhanced Error Handling in publish-crate.mjs**: Added specific detection and helpful error messages for authentication failures, including guidance on secret configuration and workflow setup.

- **Case Study Documentation**: Added comprehensive case study at `docs/case-studies/issue-21/` analyzing CI/CD failures from browser-commander repository (issues #27, #29, #31, #33) with timeline, root causes, and lessons learned.

### Fixed

- **Prevent False Positive Version Checks**: The release workflow now correctly identifies unpublished versions by checking crates.io instead of relying on git tags, which can exist without the package being published.

### Changed

- Translated all CI/CD scripts from JavaScript (.mjs) to Rust (.rs) using rust-script
- Scripts now use native Rust with rust-script for execution in shell
- Removed Node.js dependency from CI/CD pipeline
- Updated GitHub Actions workflow to use rust-script instead of node
- Updated README and CONTRIBUTING documentation with new script references

### Added

- Cache `restore-keys` for partial cache hits across all workflow jobs
- Explicit `token` parameter in checkout for release jobs
- Code coverage job with `cargo-llvm-cov` and Codecov integration
- Codecov badge in README.md
- Pre-release version support (e.g., `0.1.0-beta.1`) in version parsing
- `--release-label` parameter for multi-language release disambiguation
- `ensure_version_exceeds_published()` logic to prevent publishing duplicate versions
- `get_max_published_version()` to query highest non-yanked version from crates.io
- `max_published_version` output from check-release-needed for downstream use
- Version fallback logic in auto-release Create GitHub Release step

### Changed

- Updated `actions/checkout` from v4 to v6
- Updated `actions/cache` from v4 to v5
- Updated `peter-evans/create-pull-request` from v7 to v8
- Made `publish-crate.rs` fail (exit 1) when version already exists on crates.io
- Improved `create-github-release.rs` to check combined stdout+stderr and detect "Validation Failed"

### Fixed

- Fix publish steps overriding workflow-level CARGO_TOKEN fallback, breaking CARGO_REGISTRY_TOKEN-only configurations (#32)
- Fix non-fast-forward push failures in multi-workflow repos by adding fetch/rebase and push retry logic (#31)
- Add mono-repo path support to check-changelog-fragment.rs, check-version-modification.rs, and create-changelog-fragment.rs
- Add `!cancelled()` guard to test job condition to respect workflow cancellation

### Fixed

- Fixed `version-and-commit.rs` to check crates.io instead of git tags for determining if a version is already released
- This prevents the release pipeline from getting stuck when git tags exist without corresponding crates.io publication

### Added

- Added `--tag-prefix` support to `version-and-commit.rs` for multi-language repository compatibility
- Added crates.io and docs.rs badges to README.md
- Added automatic crates.io and docs.rs badge injection in GitHub release notes
- Added documentation deployment job to CI/CD pipeline (deploys to GitHub Pages after release)
- Added case study documentation for issue #25

### Fixed
- Fixed unsupported look-ahead regex in `create-github-release.rs` that caused a panic when parsing CHANGELOG.md. Replaced with a two-step approach using only features supported by Rust's `regex` crate.

### Changed
- Restructured example application as a simple CLI sum calculator using `lino-arguments`
- Renamed default package to `example-sum-package-name` with Unlicense license
- Reorganized test structure: `tests/unit/sum.rs`, `tests/integration/sum.rs`, `tests/unit/ci-cd/`
- Converted experiment scripts into proper unit tests in `tests/unit/ci-cd/changelog_parsing.rs`
- Added CI/CD skip logic for template default package name `example-sum-package-name`
- Updated README.md badges and documentation

### Fixed

- Change detection script now uses per-commit diff instead of full PR diff, so commits touching only non-code files correctly skip CI jobs even when earlier commits in the same PR changed code files

### Fixed
- Make release scripts resolve the publishable crate manifest when the repository root uses a Cargo workspace manifest.

### Fixed
- Decoupled GitHub Pages documentation deployment from package release publication and fixed release-script warning failures under `RUSTFLAGS=-Dwarnings`.

### Changed
- Added explicit GitHub Actions job timeouts and documented Rust test timeout guidance.

### Fixed
- Added a non-blocking warning threshold to the Rust file-size check so near-limit files are surfaced before concurrent PR merges can exceed the hard limit.

### Fixed
- Made `create-github-release.rs` build GitHub release titles as `[Language] X.Y.Z` instead of reusing the tag prefix.

### Added
- Added optional Docker Hub image publishing tied to Rust crate releases, including crates.io visibility waiting, version/latest image tags, and Docker Hub badges in GitHub release notes.

### Changed
- Release completeness checks now self-heal when crates.io exists but configured Docker Hub or GitHub release artifacts are missing.

### Fixed
- Switched documentation deployment to the official GitHub Pages artifact workflow so repositories using GitHub Actions as their Pages source do not get false-positive branch-push deploys.

### Added
- Documented the one-time `Settings → Pages → Source = GitHub Actions` prerequisite for the `deploy-docs` job in `README.md` and as a comment above the `deploy-docs` job in `release.yml`, so downstream template users hit a documented setup step instead of a `Get Pages site failed` error on the first deploy.

### Added
- Tracking case study at `docs/case-studies/issue-52/` registering the `browser-commander` + Playwright preview-regeneration pattern from [`konard/vk-bot-desktop#52`](https://github.com/konard/vk-bot-desktop/pull/52), with an activation checklist for when an example-app surface lands in this template. Documentation only — no workflow, script, or runtime code changes. Primary upstream tracking issue: [`link-foundation/js-ai-driven-development-pipeline-template#62`](https://github.com/link-foundation/js-ai-driven-development-pipeline-template/issues/62).

### Fixed
- Release automation now keeps the workspace package entry in `Cargo.lock` synchronized when `scripts/version-and-commit.rs` bumps `Cargo.toml`, preventing stale lock-file version diffs in later pull requests.

### Fixed
- `scripts/publish-crate.rs` now treats crates.io HTTP 429 throttle responses ("You have published too many versions of this crate in the last 24 hours") as a deferred `publish_result=rate_limited` outcome (it writes the output, prints an explanatory banner and exits successfully) instead of a hard CI failure reported as a generic `failed` ("Failed to publish for unknown reason"). Authentication, already-published and unknown failures still exit non-zero. Failed-publish classification is consolidated through a single `classify_failure` function and `FailureKind` enum (with an `is_deferred` predicate), covered by unit tests runnable via `rust-script --test scripts/publish-crate.rs`.
- The release workflow (`.github/workflows/release.yml`) now gates crate-availability waiting, Docker Hub publishing and GitHub release creation on either an already-published crate or `publish_result=success`, so a deferred (rate-limited) crate upload no longer produces partial downstream release artifacts and the same version is retried automatically on the next push to `main`.

### Fixed
- Fixed reversed `cancel-in-progress` concurrency condition in `release.yml` that cancelled in-flight releases on `main` and never superseded older PR runs. The condition now uses `!=` so `main` releases run to completion while newer PR pushes cancel stale runs.

### Added
- Added a `scripts/check-crate-size.rs` guard that builds the `.crate` archive and fails the release before publishing when it exceeds the crates.io 10 MiB upload limit. The check runs in the build job and before publishing in both the auto-release and manual-release jobs.

### Changed
- Added a narrow `include` allowlist to `Cargo.toml` so docs, case studies, generated CI artifacts, changelog fragments, scripts, and experiments no longer inflate the published release archive.

### Fixed
- Prevented GitHub release creation from treating generic API validation failures as existing releases, and capped oversized release notes with a link to the full tagged changelog.

### Added
- Added the initial `meta-language` links-network core with self-description roots,
  verification, trivia attachment configuration, and a minimal CLI.
- Added the default lossless `parse` entry point, projection views, and a
  tested parity roadmap for competitor features, grammar embedding, and language
  coverage targets.
- Added exact reconstruction, mixed-region detection, query matching,
  substitution rules, concept reconstruction, object identity helpers,
  many-valued truth values, and executable parity fixtures for every tracked
  competitor target.
- Added executable language fixtures and tests for every requested Markdown,
  HTML, top-ten programming-language, and top-ten natural-language target.

### Fixed
- Added a CI guard that rejects Rust test modules and test attributes under `src/`, keeping tests in the `tests/` tree.

## Minor Changes

- Add immutable and mutable network snapshots with provenance and forward
  version commits for roadmap snapshot/versioning coverage.

## [0.18.0] - 2026-06-05

### Added
- Changeset-style fragment format with frontmatter for specifying version bump type
- New `get-bump-type.mjs` script to automatically determine version bump from fragments
- Automatic version bumping on merge to main based on changelog fragments
- Detailed documentation for the changelog fragment system in `changelog.d/README.md`

### Changed
- Updated `collect-changelog.mjs` to strip frontmatter when collecting fragments
- Updated `version-and-commit.mjs` to handle frontmatter in fragments
- Enhanced release workflow to automatically determine bump type from changesets

### Changed
- Add `detect-changes` job with cross-platform `detect-code-changes.mjs` script
- Make lint job independent of changelog check (runs based on file changes only)
- Allow docs-only PRs without changelog fragment requirement
- Handle changelog check 'skipped' state in dependent jobs
- Exclude `changelog.d/`, `docs/`, `experiments/`, `examples/` folders and markdown files from code changes detection

### Fixed
- Fixed README.md to correctly reference Node.js scripts (`.mjs`) instead of Python scripts (`.py`)
- Updated project structure in README.md to match actual script files in `scripts/` directory
- Fixed example code in README.md that had invalid Rust with two `main` functions

### Added

- Added crates.io publishing support to CI/CD workflow
- Added `release_mode` input with "instant" and "changelog-pr" options for manual releases
- Added `--tag-prefix` and `--crates-io-url` options to create-github-release.mjs script
- Added comprehensive case study documentation for Issue #11 in docs/case-studies/issue-11/

### Changed

- Changed changelog fragment check from warning to error (exit 1) to enforce changelog requirements
- Updated job conditions with `always() && !cancelled()` to fix workflow_dispatch job skipping issue
- Renamed manual-release job to "Instant Release" for clarity

### Fixed

- Fixed deprecated `::set-output` GitHub Actions command in version-and-commit.mjs
- Fixed workflow_dispatch triggering issues where lint/build/release jobs were incorrectly skipped

### Fixed

- Fixed changelog fragment check to validate that a fragment is **added in the PR diff** rather than just checking if any fragments exist in the directory. This prevents the check from incorrectly passing when there are leftover fragments from previous PRs that haven't been released yet.

### Changed

- Converted shell scripts in `release.yml` to cross-platform `.mjs` scripts for improved portability and performance:
  - `check-changelog-fragment.mjs` - validates changelog fragment is added in PR diff
  - `git-config.mjs` - configures git user for CI/CD
  - `check-release-needed.mjs` - checks if release is needed
  - `publish-crate.mjs` - publishes package to crates.io
  - `create-changelog-fragment.mjs` - creates changelog fragments for manual releases
  - `get-version.mjs` - gets current version from Cargo.toml

### Added

- Added `check-version-modification.mjs` script to detect manual version changes in Cargo.toml
- Added `version-check` job to CI/CD workflow that runs on pull requests
- Added skip logic for automated release branches (changelog-manual-release-*, changeset-release/*, release/*, automated-release/*)

### Changed

- Version modifications in Cargo.toml are now blocked in pull requests to enforce automated release pipeline

### Added

- Added support for `CARGO_REGISTRY_TOKEN` as alternative to `CARGO_TOKEN` for crates.io publishing
- Added case study documentation for Issue #17 (yargs reserved word and dual token support)

### Changed

- Updated workflow to use fallback logic: `${{ secrets.CARGO_REGISTRY_TOKEN || secrets.CARGO_TOKEN }}`
- Improved publish-crate.mjs to check both `CARGO_REGISTRY_TOKEN` and `CARGO_TOKEN` environment variables
- Added warning message when neither token is set

### Added
- New `scripts/rust-paths.mjs` utility for automatic Rust package root detection
- Support for both single-language and multi-language repository structures in all CI/CD scripts
- Configuration options via `--rust-root` CLI argument and `RUST_ROOT` environment variable
- Comprehensive case study documentation in `docs/case-studies/issue-19/`

### Changed
- Updated all release scripts to use the new path detection utility:
  - `scripts/bump-version.mjs`
  - `scripts/check-release-needed.mjs`
  - `scripts/collect-changelog.mjs`
  - `scripts/get-bump-type.mjs`
  - `scripts/get-version.mjs`
  - `scripts/publish-crate.mjs`
  - `scripts/version-and-commit.mjs`

### Changed

- **check-release-needed.mjs**: Now checks crates.io API directly instead of git tags to determine if a version is already released. This prevents false positives where git tags exist but the package was never actually published to crates.io.

### Added

- **CI/CD Troubleshooting Guide**: New documentation at `docs/ci-cd/troubleshooting.md` covering common issues like skipped jobs, false positive version checks, publishing failures, and secret configuration.

- **Enhanced Error Handling in publish-crate.mjs**: Added specific detection and helpful error messages for authentication failures, including guidance on secret configuration and workflow setup.

- **Case Study Documentation**: Added comprehensive case study at `docs/case-studies/issue-21/` analyzing CI/CD failures from browser-commander repository (issues #27, #29, #31, #33) with timeline, root causes, and lessons learned.

### Fixed

- **Prevent False Positive Version Checks**: The release workflow now correctly identifies unpublished versions by checking crates.io instead of relying on git tags, which can exist without the package being published.

### Changed

- Translated all CI/CD scripts from JavaScript (.mjs) to Rust (.rs) using rust-script
- Scripts now use native Rust with rust-script for execution in shell
- Removed Node.js dependency from CI/CD pipeline
- Updated GitHub Actions workflow to use rust-script instead of node
- Updated README and CONTRIBUTING documentation with new script references

### Added

- Cache `restore-keys` for partial cache hits across all workflow jobs
- Explicit `token` parameter in checkout for release jobs
- Code coverage job with `cargo-llvm-cov` and Codecov integration
- Codecov badge in README.md
- Pre-release version support (e.g., `0.1.0-beta.1`) in version parsing
- `--release-label` parameter for multi-language release disambiguation
- `ensure_version_exceeds_published()` logic to prevent publishing duplicate versions
- `get_max_published_version()` to query highest non-yanked version from crates.io
- `max_published_version` output from check-release-needed for downstream use
- Version fallback logic in auto-release Create GitHub Release step

### Changed

- Updated `actions/checkout` from v4 to v6
- Updated `actions/cache` from v4 to v5
- Updated `peter-evans/create-pull-request` from v7 to v8
- Made `publish-crate.rs` fail (exit 1) when version already exists on crates.io
- Improved `create-github-release.rs` to check combined stdout+stderr and detect "Validation Failed"

### Fixed

- Fix publish steps overriding workflow-level CARGO_TOKEN fallback, breaking CARGO_REGISTRY_TOKEN-only configurations (#32)
- Fix non-fast-forward push failures in multi-workflow repos by adding fetch/rebase and push retry logic (#31)
- Add mono-repo path support to check-changelog-fragment.rs, check-version-modification.rs, and create-changelog-fragment.rs
- Add `!cancelled()` guard to test job condition to respect workflow cancellation

### Fixed

- Fixed `version-and-commit.rs` to check crates.io instead of git tags for determining if a version is already released
- This prevents the release pipeline from getting stuck when git tags exist without corresponding crates.io publication

### Added

- Added `--tag-prefix` support to `version-and-commit.rs` for multi-language repository compatibility
- Added crates.io and docs.rs badges to README.md
- Added automatic crates.io and docs.rs badge injection in GitHub release notes
- Added documentation deployment job to CI/CD pipeline (deploys to GitHub Pages after release)
- Added case study documentation for issue #25

### Fixed
- Fixed unsupported look-ahead regex in `create-github-release.rs` that caused a panic when parsing CHANGELOG.md. Replaced with a two-step approach using only features supported by Rust's `regex` crate.

### Changed
- Restructured example application as a simple CLI sum calculator using `lino-arguments`
- Renamed default package to `example-sum-package-name` with Unlicense license
- Reorganized test structure: `tests/unit/sum.rs`, `tests/integration/sum.rs`, `tests/unit/ci-cd/`
- Converted experiment scripts into proper unit tests in `tests/unit/ci-cd/changelog_parsing.rs`
- Added CI/CD skip logic for template default package name `example-sum-package-name`
- Updated README.md badges and documentation

### Fixed

- Change detection script now uses per-commit diff instead of full PR diff, so commits touching only non-code files correctly skip CI jobs even when earlier commits in the same PR changed code files

### Fixed
- Make release scripts resolve the publishable crate manifest when the repository root uses a Cargo workspace manifest.

### Fixed
- Decoupled GitHub Pages documentation deployment from package release publication and fixed release-script warning failures under `RUSTFLAGS=-Dwarnings`.

### Changed
- Added explicit GitHub Actions job timeouts and documented Rust test timeout guidance.

### Fixed
- Added a non-blocking warning threshold to the Rust file-size check so near-limit files are surfaced before concurrent PR merges can exceed the hard limit.

### Fixed
- Made `create-github-release.rs` build GitHub release titles as `[Language] X.Y.Z` instead of reusing the tag prefix.

### Added
- Added optional Docker Hub image publishing tied to Rust crate releases, including crates.io visibility waiting, version/latest image tags, and Docker Hub badges in GitHub release notes.

### Changed
- Release completeness checks now self-heal when crates.io exists but configured Docker Hub or GitHub release artifacts are missing.

### Fixed
- Switched documentation deployment to the official GitHub Pages artifact workflow so repositories using GitHub Actions as their Pages source do not get false-positive branch-push deploys.

### Added
- Documented the one-time `Settings → Pages → Source = GitHub Actions` prerequisite for the `deploy-docs` job in `README.md` and as a comment above the `deploy-docs` job in `release.yml`, so downstream template users hit a documented setup step instead of a `Get Pages site failed` error on the first deploy.

### Added
- Tracking case study at `docs/case-studies/issue-52/` registering the `browser-commander` + Playwright preview-regeneration pattern from [`konard/vk-bot-desktop#52`](https://github.com/konard/vk-bot-desktop/pull/52), with an activation checklist for when an example-app surface lands in this template. Documentation only — no workflow, script, or runtime code changes. Primary upstream tracking issue: [`link-foundation/js-ai-driven-development-pipeline-template#62`](https://github.com/link-foundation/js-ai-driven-development-pipeline-template/issues/62).

### Fixed
- Release automation now keeps the workspace package entry in `Cargo.lock` synchronized when `scripts/version-and-commit.rs` bumps `Cargo.toml`, preventing stale lock-file version diffs in later pull requests.

### Fixed
- `scripts/publish-crate.rs` now treats crates.io HTTP 429 throttle responses ("You have published too many versions of this crate in the last 24 hours") as a deferred `publish_result=rate_limited` outcome (it writes the output, prints an explanatory banner and exits successfully) instead of a hard CI failure reported as a generic `failed` ("Failed to publish for unknown reason"). Authentication, already-published and unknown failures still exit non-zero. Failed-publish classification is consolidated through a single `classify_failure` function and `FailureKind` enum (with an `is_deferred` predicate), covered by unit tests runnable via `rust-script --test scripts/publish-crate.rs`.
- The release workflow (`.github/workflows/release.yml`) now gates crate-availability waiting, Docker Hub publishing and GitHub release creation on either an already-published crate or `publish_result=success`, so a deferred (rate-limited) crate upload no longer produces partial downstream release artifacts and the same version is retried automatically on the next push to `main`.

### Fixed
- Fixed reversed `cancel-in-progress` concurrency condition in `release.yml` that cancelled in-flight releases on `main` and never superseded older PR runs. The condition now uses `!=` so `main` releases run to completion while newer PR pushes cancel stale runs.

### Added
- Added a `scripts/check-crate-size.rs` guard that builds the `.crate` archive and fails the release before publishing when it exceeds the crates.io 10 MiB upload limit. The check runs in the build job and before publishing in both the auto-release and manual-release jobs.

### Changed
- Added a narrow `include` allowlist to `Cargo.toml` so docs, case studies, generated CI artifacts, changelog fragments, scripts, and experiments no longer inflate the published release archive.

### Fixed
- Prevented GitHub release creation from treating generic API validation failures as existing releases, and capped oversized release notes with a link to the full tagged changelog.

## [0.17.0] - 2026-06-04

### Added
- Changeset-style fragment format with frontmatter for specifying version bump type
- New `get-bump-type.mjs` script to automatically determine version bump from fragments
- Automatic version bumping on merge to main based on changelog fragments
- Detailed documentation for the changelog fragment system in `changelog.d/README.md`

### Changed
- Updated `collect-changelog.mjs` to strip frontmatter when collecting fragments
- Updated `version-and-commit.mjs` to handle frontmatter in fragments
- Enhanced release workflow to automatically determine bump type from changesets

### Changed
- Add `detect-changes` job with cross-platform `detect-code-changes.mjs` script
- Make lint job independent of changelog check (runs based on file changes only)
- Allow docs-only PRs without changelog fragment requirement
- Handle changelog check 'skipped' state in dependent jobs
- Exclude `changelog.d/`, `docs/`, `experiments/`, `examples/` folders and markdown files from code changes detection

### Fixed
- Fixed README.md to correctly reference Node.js scripts (`.mjs`) instead of Python scripts (`.py`)
- Updated project structure in README.md to match actual script files in `scripts/` directory
- Fixed example code in README.md that had invalid Rust with two `main` functions

### Added

- Added crates.io publishing support to CI/CD workflow
- Added `release_mode` input with "instant" and "changelog-pr" options for manual releases
- Added `--tag-prefix` and `--crates-io-url` options to create-github-release.mjs script
- Added comprehensive case study documentation for Issue #11 in docs/case-studies/issue-11/

### Changed

- Changed changelog fragment check from warning to error (exit 1) to enforce changelog requirements
- Updated job conditions with `always() && !cancelled()` to fix workflow_dispatch job skipping issue
- Renamed manual-release job to "Instant Release" for clarity

### Fixed

- Fixed deprecated `::set-output` GitHub Actions command in version-and-commit.mjs
- Fixed workflow_dispatch triggering issues where lint/build/release jobs were incorrectly skipped

### Fixed

- Fixed changelog fragment check to validate that a fragment is **added in the PR diff** rather than just checking if any fragments exist in the directory. This prevents the check from incorrectly passing when there are leftover fragments from previous PRs that haven't been released yet.

### Changed

- Converted shell scripts in `release.yml` to cross-platform `.mjs` scripts for improved portability and performance:
  - `check-changelog-fragment.mjs` - validates changelog fragment is added in PR diff
  - `git-config.mjs` - configures git user for CI/CD
  - `check-release-needed.mjs` - checks if release is needed
  - `publish-crate.mjs` - publishes package to crates.io
  - `create-changelog-fragment.mjs` - creates changelog fragments for manual releases
  - `get-version.mjs` - gets current version from Cargo.toml

### Added

- Added `check-version-modification.mjs` script to detect manual version changes in Cargo.toml
- Added `version-check` job to CI/CD workflow that runs on pull requests
- Added skip logic for automated release branches (changelog-manual-release-*, changeset-release/*, release/*, automated-release/*)

### Changed

- Version modifications in Cargo.toml are now blocked in pull requests to enforce automated release pipeline

### Added

- Added support for `CARGO_REGISTRY_TOKEN` as alternative to `CARGO_TOKEN` for crates.io publishing
- Added case study documentation for Issue #17 (yargs reserved word and dual token support)

### Changed

- Updated workflow to use fallback logic: `${{ secrets.CARGO_REGISTRY_TOKEN || secrets.CARGO_TOKEN }}`
- Improved publish-crate.mjs to check both `CARGO_REGISTRY_TOKEN` and `CARGO_TOKEN` environment variables
- Added warning message when neither token is set

### Added
- New `scripts/rust-paths.mjs` utility for automatic Rust package root detection
- Support for both single-language and multi-language repository structures in all CI/CD scripts
- Configuration options via `--rust-root` CLI argument and `RUST_ROOT` environment variable
- Comprehensive case study documentation in `docs/case-studies/issue-19/`

### Changed
- Updated all release scripts to use the new path detection utility:
  - `scripts/bump-version.mjs`
  - `scripts/check-release-needed.mjs`
  - `scripts/collect-changelog.mjs`
  - `scripts/get-bump-type.mjs`
  - `scripts/get-version.mjs`
  - `scripts/publish-crate.mjs`
  - `scripts/version-and-commit.mjs`

### Changed

- **check-release-needed.mjs**: Now checks crates.io API directly instead of git tags to determine if a version is already released. This prevents false positives where git tags exist but the package was never actually published to crates.io.

### Added

- **CI/CD Troubleshooting Guide**: New documentation at `docs/ci-cd/troubleshooting.md` covering common issues like skipped jobs, false positive version checks, publishing failures, and secret configuration.

- **Enhanced Error Handling in publish-crate.mjs**: Added specific detection and helpful error messages for authentication failures, including guidance on secret configuration and workflow setup.

- **Case Study Documentation**: Added comprehensive case study at `docs/case-studies/issue-21/` analyzing CI/CD failures from browser-commander repository (issues #27, #29, #31, #33) with timeline, root causes, and lessons learned.

### Fixed

- **Prevent False Positive Version Checks**: The release workflow now correctly identifies unpublished versions by checking crates.io instead of relying on git tags, which can exist without the package being published.

### Changed

- Translated all CI/CD scripts from JavaScript (.mjs) to Rust (.rs) using rust-script
- Scripts now use native Rust with rust-script for execution in shell
- Removed Node.js dependency from CI/CD pipeline
- Updated GitHub Actions workflow to use rust-script instead of node
- Updated README and CONTRIBUTING documentation with new script references

### Added

- Cache `restore-keys` for partial cache hits across all workflow jobs
- Explicit `token` parameter in checkout for release jobs
- Code coverage job with `cargo-llvm-cov` and Codecov integration
- Codecov badge in README.md
- Pre-release version support (e.g., `0.1.0-beta.1`) in version parsing
- `--release-label` parameter for multi-language release disambiguation
- `ensure_version_exceeds_published()` logic to prevent publishing duplicate versions
- `get_max_published_version()` to query highest non-yanked version from crates.io
- `max_published_version` output from check-release-needed for downstream use
- Version fallback logic in auto-release Create GitHub Release step

### Changed

- Updated `actions/checkout` from v4 to v6
- Updated `actions/cache` from v4 to v5
- Updated `peter-evans/create-pull-request` from v7 to v8
- Made `publish-crate.rs` fail (exit 1) when version already exists on crates.io
- Improved `create-github-release.rs` to check combined stdout+stderr and detect "Validation Failed"

### Fixed

- Fix publish steps overriding workflow-level CARGO_TOKEN fallback, breaking CARGO_REGISTRY_TOKEN-only configurations (#32)
- Fix non-fast-forward push failures in multi-workflow repos by adding fetch/rebase and push retry logic (#31)
- Add mono-repo path support to check-changelog-fragment.rs, check-version-modification.rs, and create-changelog-fragment.rs
- Add `!cancelled()` guard to test job condition to respect workflow cancellation

### Fixed

- Fixed `version-and-commit.rs` to check crates.io instead of git tags for determining if a version is already released
- This prevents the release pipeline from getting stuck when git tags exist without corresponding crates.io publication

### Added

- Added `--tag-prefix` support to `version-and-commit.rs` for multi-language repository compatibility
- Added crates.io and docs.rs badges to README.md
- Added automatic crates.io and docs.rs badge injection in GitHub release notes
- Added documentation deployment job to CI/CD pipeline (deploys to GitHub Pages after release)
- Added case study documentation for issue #25

### Fixed
- Fixed unsupported look-ahead regex in `create-github-release.rs` that caused a panic when parsing CHANGELOG.md. Replaced with a two-step approach using only features supported by Rust's `regex` crate.

### Changed
- Restructured example application as a simple CLI sum calculator using `lino-arguments`
- Renamed default package to `example-sum-package-name` with Unlicense license
- Reorganized test structure: `tests/unit/sum.rs`, `tests/integration/sum.rs`, `tests/unit/ci-cd/`
- Converted experiment scripts into proper unit tests in `tests/unit/ci-cd/changelog_parsing.rs`
- Added CI/CD skip logic for template default package name `example-sum-package-name`
- Updated README.md badges and documentation

### Fixed

- Change detection script now uses per-commit diff instead of full PR diff, so commits touching only non-code files correctly skip CI jobs even when earlier commits in the same PR changed code files

### Fixed
- Make release scripts resolve the publishable crate manifest when the repository root uses a Cargo workspace manifest.

### Fixed
- Decoupled GitHub Pages documentation deployment from package release publication and fixed release-script warning failures under `RUSTFLAGS=-Dwarnings`.

### Changed
- Added explicit GitHub Actions job timeouts and documented Rust test timeout guidance.

### Fixed
- Added a non-blocking warning threshold to the Rust file-size check so near-limit files are surfaced before concurrent PR merges can exceed the hard limit.

### Fixed
- Made `create-github-release.rs` build GitHub release titles as `[Language] X.Y.Z` instead of reusing the tag prefix.

### Added
- Added optional Docker Hub image publishing tied to Rust crate releases, including crates.io visibility waiting, version/latest image tags, and Docker Hub badges in GitHub release notes.

### Changed
- Release completeness checks now self-heal when crates.io exists but configured Docker Hub or GitHub release artifacts are missing.

### Fixed
- Switched documentation deployment to the official GitHub Pages artifact workflow so repositories using GitHub Actions as their Pages source do not get false-positive branch-push deploys.

### Added
- Documented the one-time `Settings → Pages → Source = GitHub Actions` prerequisite for the `deploy-docs` job in `README.md` and as a comment above the `deploy-docs` job in `release.yml`, so downstream template users hit a documented setup step instead of a `Get Pages site failed` error on the first deploy.

### Added
- Tracking case study at `docs/case-studies/issue-52/` registering the `browser-commander` + Playwright preview-regeneration pattern from [`konard/vk-bot-desktop#52`](https://github.com/konard/vk-bot-desktop/pull/52), with an activation checklist for when an example-app surface lands in this template. Documentation only — no workflow, script, or runtime code changes. Primary upstream tracking issue: [`link-foundation/js-ai-driven-development-pipeline-template#62`](https://github.com/link-foundation/js-ai-driven-development-pipeline-template/issues/62).

### Fixed
- Release automation now keeps the workspace package entry in `Cargo.lock` synchronized when `scripts/version-and-commit.rs` bumps `Cargo.toml`, preventing stale lock-file version diffs in later pull requests.

### Fixed
- `scripts/publish-crate.rs` now treats crates.io HTTP 429 throttle responses ("You have published too many versions of this crate in the last 24 hours") as a deferred `publish_result=rate_limited` outcome (it writes the output, prints an explanatory banner and exits successfully) instead of a hard CI failure reported as a generic `failed` ("Failed to publish for unknown reason"). Authentication, already-published and unknown failures still exit non-zero. Failed-publish classification is consolidated through a single `classify_failure` function and `FailureKind` enum (with an `is_deferred` predicate), covered by unit tests runnable via `rust-script --test scripts/publish-crate.rs`.
- The release workflow (`.github/workflows/release.yml`) now gates crate-availability waiting, Docker Hub publishing and GitHub release creation on either an already-published crate or `publish_result=success`, so a deferred (rate-limited) crate upload no longer produces partial downstream release artifacts and the same version is retried automatically on the next push to `main`.

### Fixed
- Fixed reversed `cancel-in-progress` concurrency condition in `release.yml` that cancelled in-flight releases on `main` and never superseded older PR runs. The condition now uses `!=` so `main` releases run to completion while newer PR pushes cancel stale runs.

### Added
- Added a `scripts/check-crate-size.rs` guard that builds the `.crate` archive and fails the release before publishing when it exceeds the crates.io 10 MiB upload limit. The check runs in the build job and before publishing in both the auto-release and manual-release jobs.

### Changed
- Added a narrow `include` allowlist to `Cargo.toml` so docs, case studies, generated CI artifacts, changelog fragments, scripts, and experiments no longer inflate the published release archive.

### Fixed
- Prevented GitHub release creation from treating generic API validation failures as existing releases, and capped oversized release notes with a link to the full tagged changelog.

## [0.16.0] - 2026-05-29

### Added
- Changeset-style fragment format with frontmatter for specifying version bump type
- New `get-bump-type.mjs` script to automatically determine version bump from fragments
- Automatic version bumping on merge to main based on changelog fragments
- Detailed documentation for the changelog fragment system in `changelog.d/README.md`

### Changed
- Updated `collect-changelog.mjs` to strip frontmatter when collecting fragments
- Updated `version-and-commit.mjs` to handle frontmatter in fragments
- Enhanced release workflow to automatically determine bump type from changesets

### Changed
- Add `detect-changes` job with cross-platform `detect-code-changes.mjs` script
- Make lint job independent of changelog check (runs based on file changes only)
- Allow docs-only PRs without changelog fragment requirement
- Handle changelog check 'skipped' state in dependent jobs
- Exclude `changelog.d/`, `docs/`, `experiments/`, `examples/` folders and markdown files from code changes detection

### Fixed
- Fixed README.md to correctly reference Node.js scripts (`.mjs`) instead of Python scripts (`.py`)
- Updated project structure in README.md to match actual script files in `scripts/` directory
- Fixed example code in README.md that had invalid Rust with two `main` functions

### Added

- Added crates.io publishing support to CI/CD workflow
- Added `release_mode` input with "instant" and "changelog-pr" options for manual releases
- Added `--tag-prefix` and `--crates-io-url` options to create-github-release.mjs script
- Added comprehensive case study documentation for Issue #11 in docs/case-studies/issue-11/

### Changed

- Changed changelog fragment check from warning to error (exit 1) to enforce changelog requirements
- Updated job conditions with `always() && !cancelled()` to fix workflow_dispatch job skipping issue
- Renamed manual-release job to "Instant Release" for clarity

### Fixed

- Fixed deprecated `::set-output` GitHub Actions command in version-and-commit.mjs
- Fixed workflow_dispatch triggering issues where lint/build/release jobs were incorrectly skipped

### Fixed

- Fixed changelog fragment check to validate that a fragment is **added in the PR diff** rather than just checking if any fragments exist in the directory. This prevents the check from incorrectly passing when there are leftover fragments from previous PRs that haven't been released yet.

### Changed

- Converted shell scripts in `release.yml` to cross-platform `.mjs` scripts for improved portability and performance:
  - `check-changelog-fragment.mjs` - validates changelog fragment is added in PR diff
  - `git-config.mjs` - configures git user for CI/CD
  - `check-release-needed.mjs` - checks if release is needed
  - `publish-crate.mjs` - publishes package to crates.io
  - `create-changelog-fragment.mjs` - creates changelog fragments for manual releases
  - `get-version.mjs` - gets current version from Cargo.toml

### Added

- Added `check-version-modification.mjs` script to detect manual version changes in Cargo.toml
- Added `version-check` job to CI/CD workflow that runs on pull requests
- Added skip logic for automated release branches (changelog-manual-release-*, changeset-release/*, release/*, automated-release/*)

### Changed

- Version modifications in Cargo.toml are now blocked in pull requests to enforce automated release pipeline

### Added

- Added support for `CARGO_REGISTRY_TOKEN` as alternative to `CARGO_TOKEN` for crates.io publishing
- Added case study documentation for Issue #17 (yargs reserved word and dual token support)

### Changed

- Updated workflow to use fallback logic: `${{ secrets.CARGO_REGISTRY_TOKEN || secrets.CARGO_TOKEN }}`
- Improved publish-crate.mjs to check both `CARGO_REGISTRY_TOKEN` and `CARGO_TOKEN` environment variables
- Added warning message when neither token is set

### Added
- New `scripts/rust-paths.mjs` utility for automatic Rust package root detection
- Support for both single-language and multi-language repository structures in all CI/CD scripts
- Configuration options via `--rust-root` CLI argument and `RUST_ROOT` environment variable
- Comprehensive case study documentation in `docs/case-studies/issue-19/`

### Changed
- Updated all release scripts to use the new path detection utility:
  - `scripts/bump-version.mjs`
  - `scripts/check-release-needed.mjs`
  - `scripts/collect-changelog.mjs`
  - `scripts/get-bump-type.mjs`
  - `scripts/get-version.mjs`
  - `scripts/publish-crate.mjs`
  - `scripts/version-and-commit.mjs`

### Changed

- **check-release-needed.mjs**: Now checks crates.io API directly instead of git tags to determine if a version is already released. This prevents false positives where git tags exist but the package was never actually published to crates.io.

### Added

- **CI/CD Troubleshooting Guide**: New documentation at `docs/ci-cd/troubleshooting.md` covering common issues like skipped jobs, false positive version checks, publishing failures, and secret configuration.

- **Enhanced Error Handling in publish-crate.mjs**: Added specific detection and helpful error messages for authentication failures, including guidance on secret configuration and workflow setup.

- **Case Study Documentation**: Added comprehensive case study at `docs/case-studies/issue-21/` analyzing CI/CD failures from browser-commander repository (issues #27, #29, #31, #33) with timeline, root causes, and lessons learned.

### Fixed

- **Prevent False Positive Version Checks**: The release workflow now correctly identifies unpublished versions by checking crates.io instead of relying on git tags, which can exist without the package being published.

### Changed

- Translated all CI/CD scripts from JavaScript (.mjs) to Rust (.rs) using rust-script
- Scripts now use native Rust with rust-script for execution in shell
- Removed Node.js dependency from CI/CD pipeline
- Updated GitHub Actions workflow to use rust-script instead of node
- Updated README and CONTRIBUTING documentation with new script references

### Added

- Cache `restore-keys` for partial cache hits across all workflow jobs
- Explicit `token` parameter in checkout for release jobs
- Code coverage job with `cargo-llvm-cov` and Codecov integration
- Codecov badge in README.md
- Pre-release version support (e.g., `0.1.0-beta.1`) in version parsing
- `--release-label` parameter for multi-language release disambiguation
- `ensure_version_exceeds_published()` logic to prevent publishing duplicate versions
- `get_max_published_version()` to query highest non-yanked version from crates.io
- `max_published_version` output from check-release-needed for downstream use
- Version fallback logic in auto-release Create GitHub Release step

### Changed

- Updated `actions/checkout` from v4 to v6
- Updated `actions/cache` from v4 to v5
- Updated `peter-evans/create-pull-request` from v7 to v8
- Made `publish-crate.rs` fail (exit 1) when version already exists on crates.io
- Improved `create-github-release.rs` to check combined stdout+stderr and detect "Validation Failed"

### Fixed

- Fix publish steps overriding workflow-level CARGO_TOKEN fallback, breaking CARGO_REGISTRY_TOKEN-only configurations (#32)
- Fix non-fast-forward push failures in multi-workflow repos by adding fetch/rebase and push retry logic (#31)
- Add mono-repo path support to check-changelog-fragment.rs, check-version-modification.rs, and create-changelog-fragment.rs
- Add `!cancelled()` guard to test job condition to respect workflow cancellation

### Fixed

- Fixed `version-and-commit.rs` to check crates.io instead of git tags for determining if a version is already released
- This prevents the release pipeline from getting stuck when git tags exist without corresponding crates.io publication

### Added

- Added `--tag-prefix` support to `version-and-commit.rs` for multi-language repository compatibility
- Added crates.io and docs.rs badges to README.md
- Added automatic crates.io and docs.rs badge injection in GitHub release notes
- Added documentation deployment job to CI/CD pipeline (deploys to GitHub Pages after release)
- Added case study documentation for issue #25

### Fixed
- Fixed unsupported look-ahead regex in `create-github-release.rs` that caused a panic when parsing CHANGELOG.md. Replaced with a two-step approach using only features supported by Rust's `regex` crate.

### Changed
- Restructured example application as a simple CLI sum calculator using `lino-arguments`
- Renamed default package to `example-sum-package-name` with Unlicense license
- Reorganized test structure: `tests/unit/sum.rs`, `tests/integration/sum.rs`, `tests/unit/ci-cd/`
- Converted experiment scripts into proper unit tests in `tests/unit/ci-cd/changelog_parsing.rs`
- Added CI/CD skip logic for template default package name `example-sum-package-name`
- Updated README.md badges and documentation

### Fixed

- Change detection script now uses per-commit diff instead of full PR diff, so commits touching only non-code files correctly skip CI jobs even when earlier commits in the same PR changed code files

### Fixed
- Make release scripts resolve the publishable crate manifest when the repository root uses a Cargo workspace manifest.

### Fixed
- Decoupled GitHub Pages documentation deployment from package release publication and fixed release-script warning failures under `RUSTFLAGS=-Dwarnings`.

### Changed
- Added explicit GitHub Actions job timeouts and documented Rust test timeout guidance.

### Fixed
- Added a non-blocking warning threshold to the Rust file-size check so near-limit files are surfaced before concurrent PR merges can exceed the hard limit.

### Fixed
- Made `create-github-release.rs` build GitHub release titles as `[Language] X.Y.Z` instead of reusing the tag prefix.

### Added
- Added optional Docker Hub image publishing tied to Rust crate releases, including crates.io visibility waiting, version/latest image tags, and Docker Hub badges in GitHub release notes.

### Changed
- Release completeness checks now self-heal when crates.io exists but configured Docker Hub or GitHub release artifacts are missing.

### Fixed
- Switched documentation deployment to the official GitHub Pages artifact workflow so repositories using GitHub Actions as their Pages source do not get false-positive branch-push deploys.

### Added
- Documented the one-time `Settings → Pages → Source = GitHub Actions` prerequisite for the `deploy-docs` job in `README.md` and as a comment above the `deploy-docs` job in `release.yml`, so downstream template users hit a documented setup step instead of a `Get Pages site failed` error on the first deploy.

### Added
- Tracking case study at `docs/case-studies/issue-52/` registering the `browser-commander` + Playwright preview-regeneration pattern from [`konard/vk-bot-desktop#52`](https://github.com/konard/vk-bot-desktop/pull/52), with an activation checklist for when an example-app surface lands in this template. Documentation only — no workflow, script, or runtime code changes. Primary upstream tracking issue: [`link-foundation/js-ai-driven-development-pipeline-template#62`](https://github.com/link-foundation/js-ai-driven-development-pipeline-template/issues/62).

### Fixed
- Release automation now keeps the workspace package entry in `Cargo.lock` synchronized when `scripts/version-and-commit.rs` bumps `Cargo.toml`, preventing stale lock-file version diffs in later pull requests.

### Fixed
- `scripts/publish-crate.rs` now treats crates.io HTTP 429 throttle responses ("You have published too many versions of this crate in the last 24 hours") as a deferred `publish_result=rate_limited` outcome (it writes the output, prints an explanatory banner and exits successfully) instead of a hard CI failure reported as a generic `failed` ("Failed to publish for unknown reason"). Authentication, already-published and unknown failures still exit non-zero. Failed-publish classification is consolidated through a single `classify_failure` function and `FailureKind` enum (with an `is_deferred` predicate), covered by unit tests runnable via `rust-script --test scripts/publish-crate.rs`.
- The release workflow (`.github/workflows/release.yml`) now gates crate-availability waiting, Docker Hub publishing and GitHub release creation on either an already-published crate or `publish_result=success`, so a deferred (rate-limited) crate upload no longer produces partial downstream release artifacts and the same version is retried automatically on the next push to `main`.

### Fixed
- Fixed reversed `cancel-in-progress` concurrency condition in `release.yml` that cancelled in-flight releases on `main` and never superseded older PR runs. The condition now uses `!=` so `main` releases run to completion while newer PR pushes cancel stale runs.

### Added
- Added a `scripts/check-crate-size.rs` guard that builds the `.crate` archive and fails the release before publishing when it exceeds the crates.io 10 MiB upload limit. The check runs in the build job and before publishing in both the auto-release and manual-release jobs.

### Changed
- Added a narrow `include` allowlist to `Cargo.toml` so docs, case studies, generated CI artifacts, changelog fragments, scripts, and experiments no longer inflate the published release archive.

## [0.15.0] - 2026-05-16

### Added
- Changeset-style fragment format with frontmatter for specifying version bump type
- New `get-bump-type.mjs` script to automatically determine version bump from fragments
- Automatic version bumping on merge to main based on changelog fragments
- Detailed documentation for the changelog fragment system in `changelog.d/README.md`

### Changed
- Updated `collect-changelog.mjs` to strip frontmatter when collecting fragments
- Updated `version-and-commit.mjs` to handle frontmatter in fragments
- Enhanced release workflow to automatically determine bump type from changesets

### Changed
- Add `detect-changes` job with cross-platform `detect-code-changes.mjs` script
- Make lint job independent of changelog check (runs based on file changes only)
- Allow docs-only PRs without changelog fragment requirement
- Handle changelog check 'skipped' state in dependent jobs
- Exclude `changelog.d/`, `docs/`, `experiments/`, `examples/` folders and markdown files from code changes detection

### Fixed
- Fixed README.md to correctly reference Node.js scripts (`.mjs`) instead of Python scripts (`.py`)
- Updated project structure in README.md to match actual script files in `scripts/` directory
- Fixed example code in README.md that had invalid Rust with two `main` functions

### Added

- Added crates.io publishing support to CI/CD workflow
- Added `release_mode` input with "instant" and "changelog-pr" options for manual releases
- Added `--tag-prefix` and `--crates-io-url` options to create-github-release.mjs script
- Added comprehensive case study documentation for Issue #11 in docs/case-studies/issue-11/

### Changed

- Changed changelog fragment check from warning to error (exit 1) to enforce changelog requirements
- Updated job conditions with `always() && !cancelled()` to fix workflow_dispatch job skipping issue
- Renamed manual-release job to "Instant Release" for clarity

### Fixed

- Fixed deprecated `::set-output` GitHub Actions command in version-and-commit.mjs
- Fixed workflow_dispatch triggering issues where lint/build/release jobs were incorrectly skipped

### Fixed

- Fixed changelog fragment check to validate that a fragment is **added in the PR diff** rather than just checking if any fragments exist in the directory. This prevents the check from incorrectly passing when there are leftover fragments from previous PRs that haven't been released yet.

### Changed

- Converted shell scripts in `release.yml` to cross-platform `.mjs` scripts for improved portability and performance:
  - `check-changelog-fragment.mjs` - validates changelog fragment is added in PR diff
  - `git-config.mjs` - configures git user for CI/CD
  - `check-release-needed.mjs` - checks if release is needed
  - `publish-crate.mjs` - publishes package to crates.io
  - `create-changelog-fragment.mjs` - creates changelog fragments for manual releases
  - `get-version.mjs` - gets current version from Cargo.toml

### Added

- Added `check-version-modification.mjs` script to detect manual version changes in Cargo.toml
- Added `version-check` job to CI/CD workflow that runs on pull requests
- Added skip logic for automated release branches (changelog-manual-release-*, changeset-release/*, release/*, automated-release/*)

### Changed

- Version modifications in Cargo.toml are now blocked in pull requests to enforce automated release pipeline

### Added

- Added support for `CARGO_REGISTRY_TOKEN` as alternative to `CARGO_TOKEN` for crates.io publishing
- Added case study documentation for Issue #17 (yargs reserved word and dual token support)

### Changed

- Updated workflow to use fallback logic: `${{ secrets.CARGO_REGISTRY_TOKEN || secrets.CARGO_TOKEN }}`
- Improved publish-crate.mjs to check both `CARGO_REGISTRY_TOKEN` and `CARGO_TOKEN` environment variables
- Added warning message when neither token is set

### Added
- New `scripts/rust-paths.mjs` utility for automatic Rust package root detection
- Support for both single-language and multi-language repository structures in all CI/CD scripts
- Configuration options via `--rust-root` CLI argument and `RUST_ROOT` environment variable
- Comprehensive case study documentation in `docs/case-studies/issue-19/`

### Changed
- Updated all release scripts to use the new path detection utility:
  - `scripts/bump-version.mjs`
  - `scripts/check-release-needed.mjs`
  - `scripts/collect-changelog.mjs`
  - `scripts/get-bump-type.mjs`
  - `scripts/get-version.mjs`
  - `scripts/publish-crate.mjs`
  - `scripts/version-and-commit.mjs`

### Changed

- **check-release-needed.mjs**: Now checks crates.io API directly instead of git tags to determine if a version is already released. This prevents false positives where git tags exist but the package was never actually published to crates.io.

### Added

- **CI/CD Troubleshooting Guide**: New documentation at `docs/ci-cd/troubleshooting.md` covering common issues like skipped jobs, false positive version checks, publishing failures, and secret configuration.

- **Enhanced Error Handling in publish-crate.mjs**: Added specific detection and helpful error messages for authentication failures, including guidance on secret configuration and workflow setup.

- **Case Study Documentation**: Added comprehensive case study at `docs/case-studies/issue-21/` analyzing CI/CD failures from browser-commander repository (issues #27, #29, #31, #33) with timeline, root causes, and lessons learned.

### Fixed

- **Prevent False Positive Version Checks**: The release workflow now correctly identifies unpublished versions by checking crates.io instead of relying on git tags, which can exist without the package being published.

### Changed

- Translated all CI/CD scripts from JavaScript (.mjs) to Rust (.rs) using rust-script
- Scripts now use native Rust with rust-script for execution in shell
- Removed Node.js dependency from CI/CD pipeline
- Updated GitHub Actions workflow to use rust-script instead of node
- Updated README and CONTRIBUTING documentation with new script references

### Added

- Cache `restore-keys` for partial cache hits across all workflow jobs
- Explicit `token` parameter in checkout for release jobs
- Code coverage job with `cargo-llvm-cov` and Codecov integration
- Codecov badge in README.md
- Pre-release version support (e.g., `0.1.0-beta.1`) in version parsing
- `--release-label` parameter for multi-language release disambiguation
- `ensure_version_exceeds_published()` logic to prevent publishing duplicate versions
- `get_max_published_version()` to query highest non-yanked version from crates.io
- `max_published_version` output from check-release-needed for downstream use
- Version fallback logic in auto-release Create GitHub Release step

### Changed

- Updated `actions/checkout` from v4 to v6
- Updated `actions/cache` from v4 to v5
- Updated `peter-evans/create-pull-request` from v7 to v8
- Made `publish-crate.rs` fail (exit 1) when version already exists on crates.io
- Improved `create-github-release.rs` to check combined stdout+stderr and detect "Validation Failed"

### Fixed

- Fix publish steps overriding workflow-level CARGO_TOKEN fallback, breaking CARGO_REGISTRY_TOKEN-only configurations (#32)
- Fix non-fast-forward push failures in multi-workflow repos by adding fetch/rebase and push retry logic (#31)
- Add mono-repo path support to check-changelog-fragment.rs, check-version-modification.rs, and create-changelog-fragment.rs
- Add `!cancelled()` guard to test job condition to respect workflow cancellation

### Fixed

- Fixed `version-and-commit.rs` to check crates.io instead of git tags for determining if a version is already released
- This prevents the release pipeline from getting stuck when git tags exist without corresponding crates.io publication

### Added

- Added `--tag-prefix` support to `version-and-commit.rs` for multi-language repository compatibility
- Added crates.io and docs.rs badges to README.md
- Added automatic crates.io and docs.rs badge injection in GitHub release notes
- Added documentation deployment job to CI/CD pipeline (deploys to GitHub Pages after release)
- Added case study documentation for issue #25

### Fixed
- Fixed unsupported look-ahead regex in `create-github-release.rs` that caused a panic when parsing CHANGELOG.md. Replaced with a two-step approach using only features supported by Rust's `regex` crate.

### Changed
- Restructured example application as a simple CLI sum calculator using `lino-arguments`
- Renamed default package to `example-sum-package-name` with Unlicense license
- Reorganized test structure: `tests/unit/sum.rs`, `tests/integration/sum.rs`, `tests/unit/ci-cd/`
- Converted experiment scripts into proper unit tests in `tests/unit/ci-cd/changelog_parsing.rs`
- Added CI/CD skip logic for template default package name `example-sum-package-name`
- Updated README.md badges and documentation

### Fixed

- Change detection script now uses per-commit diff instead of full PR diff, so commits touching only non-code files correctly skip CI jobs even when earlier commits in the same PR changed code files

### Fixed
- Make release scripts resolve the publishable crate manifest when the repository root uses a Cargo workspace manifest.

### Fixed
- Decoupled GitHub Pages documentation deployment from package release publication and fixed release-script warning failures under `RUSTFLAGS=-Dwarnings`.

### Changed
- Added explicit GitHub Actions job timeouts and documented Rust test timeout guidance.

### Fixed
- Added a non-blocking warning threshold to the Rust file-size check so near-limit files are surfaced before concurrent PR merges can exceed the hard limit.

### Fixed
- Made `create-github-release.rs` build GitHub release titles as `[Language] X.Y.Z` instead of reusing the tag prefix.

### Added
- Added optional Docker Hub image publishing tied to Rust crate releases, including crates.io visibility waiting, version/latest image tags, and Docker Hub badges in GitHub release notes.

### Changed
- Release completeness checks now self-heal when crates.io exists but configured Docker Hub or GitHub release artifacts are missing.

### Fixed
- Switched documentation deployment to the official GitHub Pages artifact workflow so repositories using GitHub Actions as their Pages source do not get false-positive branch-push deploys.

### Added
- Documented the one-time `Settings → Pages → Source = GitHub Actions` prerequisite for the `deploy-docs` job in `README.md` and as a comment above the `deploy-docs` job in `release.yml`, so downstream template users hit a documented setup step instead of a `Get Pages site failed` error on the first deploy.

### Added
- Tracking case study at `docs/case-studies/issue-52/` registering the `browser-commander` + Playwright preview-regeneration pattern from [`konard/vk-bot-desktop#52`](https://github.com/konard/vk-bot-desktop/pull/52), with an activation checklist for when an example-app surface lands in this template. Documentation only — no workflow, script, or runtime code changes. Primary upstream tracking issue: [`link-foundation/js-ai-driven-development-pipeline-template#62`](https://github.com/link-foundation/js-ai-driven-development-pipeline-template/issues/62).

### Fixed
- Release automation now keeps the workspace package entry in `Cargo.lock` synchronized when `scripts/version-and-commit.rs` bumps `Cargo.toml`, preventing stale lock-file version diffs in later pull requests.

## [0.14.0] - 2026-05-15

### Added
- Changeset-style fragment format with frontmatter for specifying version bump type
- New `get-bump-type.mjs` script to automatically determine version bump from fragments
- Automatic version bumping on merge to main based on changelog fragments
- Detailed documentation for the changelog fragment system in `changelog.d/README.md`

### Changed
- Updated `collect-changelog.mjs` to strip frontmatter when collecting fragments
- Updated `version-and-commit.mjs` to handle frontmatter in fragments
- Enhanced release workflow to automatically determine bump type from changesets

### Changed
- Add `detect-changes` job with cross-platform `detect-code-changes.mjs` script
- Make lint job independent of changelog check (runs based on file changes only)
- Allow docs-only PRs without changelog fragment requirement
- Handle changelog check 'skipped' state in dependent jobs
- Exclude `changelog.d/`, `docs/`, `experiments/`, `examples/` folders and markdown files from code changes detection

### Fixed
- Fixed README.md to correctly reference Node.js scripts (`.mjs`) instead of Python scripts (`.py`)
- Updated project structure in README.md to match actual script files in `scripts/` directory
- Fixed example code in README.md that had invalid Rust with two `main` functions

### Added

- Added crates.io publishing support to CI/CD workflow
- Added `release_mode` input with "instant" and "changelog-pr" options for manual releases
- Added `--tag-prefix` and `--crates-io-url` options to create-github-release.mjs script
- Added comprehensive case study documentation for Issue #11 in docs/case-studies/issue-11/

### Changed

- Changed changelog fragment check from warning to error (exit 1) to enforce changelog requirements
- Updated job conditions with `always() && !cancelled()` to fix workflow_dispatch job skipping issue
- Renamed manual-release job to "Instant Release" for clarity

### Fixed

- Fixed deprecated `::set-output` GitHub Actions command in version-and-commit.mjs
- Fixed workflow_dispatch triggering issues where lint/build/release jobs were incorrectly skipped

### Fixed

- Fixed changelog fragment check to validate that a fragment is **added in the PR diff** rather than just checking if any fragments exist in the directory. This prevents the check from incorrectly passing when there are leftover fragments from previous PRs that haven't been released yet.

### Changed

- Converted shell scripts in `release.yml` to cross-platform `.mjs` scripts for improved portability and performance:
  - `check-changelog-fragment.mjs` - validates changelog fragment is added in PR diff
  - `git-config.mjs` - configures git user for CI/CD
  - `check-release-needed.mjs` - checks if release is needed
  - `publish-crate.mjs` - publishes package to crates.io
  - `create-changelog-fragment.mjs` - creates changelog fragments for manual releases
  - `get-version.mjs` - gets current version from Cargo.toml

### Added

- Added `check-version-modification.mjs` script to detect manual version changes in Cargo.toml
- Added `version-check` job to CI/CD workflow that runs on pull requests
- Added skip logic for automated release branches (changelog-manual-release-*, changeset-release/*, release/*, automated-release/*)

### Changed

- Version modifications in Cargo.toml are now blocked in pull requests to enforce automated release pipeline

### Added

- Added support for `CARGO_REGISTRY_TOKEN` as alternative to `CARGO_TOKEN` for crates.io publishing
- Added case study documentation for Issue #17 (yargs reserved word and dual token support)

### Changed

- Updated workflow to use fallback logic: `${{ secrets.CARGO_REGISTRY_TOKEN || secrets.CARGO_TOKEN }}`
- Improved publish-crate.mjs to check both `CARGO_REGISTRY_TOKEN` and `CARGO_TOKEN` environment variables
- Added warning message when neither token is set

### Added
- New `scripts/rust-paths.mjs` utility for automatic Rust package root detection
- Support for both single-language and multi-language repository structures in all CI/CD scripts
- Configuration options via `--rust-root` CLI argument and `RUST_ROOT` environment variable
- Comprehensive case study documentation in `docs/case-studies/issue-19/`

### Changed
- Updated all release scripts to use the new path detection utility:
  - `scripts/bump-version.mjs`
  - `scripts/check-release-needed.mjs`
  - `scripts/collect-changelog.mjs`
  - `scripts/get-bump-type.mjs`
  - `scripts/get-version.mjs`
  - `scripts/publish-crate.mjs`
  - `scripts/version-and-commit.mjs`

### Changed

- **check-release-needed.mjs**: Now checks crates.io API directly instead of git tags to determine if a version is already released. This prevents false positives where git tags exist but the package was never actually published to crates.io.

### Added

- **CI/CD Troubleshooting Guide**: New documentation at `docs/ci-cd/troubleshooting.md` covering common issues like skipped jobs, false positive version checks, publishing failures, and secret configuration.

- **Enhanced Error Handling in publish-crate.mjs**: Added specific detection and helpful error messages for authentication failures, including guidance on secret configuration and workflow setup.

- **Case Study Documentation**: Added comprehensive case study at `docs/case-studies/issue-21/` analyzing CI/CD failures from browser-commander repository (issues #27, #29, #31, #33) with timeline, root causes, and lessons learned.

### Fixed

- **Prevent False Positive Version Checks**: The release workflow now correctly identifies unpublished versions by checking crates.io instead of relying on git tags, which can exist without the package being published.

### Changed

- Translated all CI/CD scripts from JavaScript (.mjs) to Rust (.rs) using rust-script
- Scripts now use native Rust with rust-script for execution in shell
- Removed Node.js dependency from CI/CD pipeline
- Updated GitHub Actions workflow to use rust-script instead of node
- Updated README and CONTRIBUTING documentation with new script references

### Added

- Cache `restore-keys` for partial cache hits across all workflow jobs
- Explicit `token` parameter in checkout for release jobs
- Code coverage job with `cargo-llvm-cov` and Codecov integration
- Codecov badge in README.md
- Pre-release version support (e.g., `0.1.0-beta.1`) in version parsing
- `--release-label` parameter for multi-language release disambiguation
- `ensure_version_exceeds_published()` logic to prevent publishing duplicate versions
- `get_max_published_version()` to query highest non-yanked version from crates.io
- `max_published_version` output from check-release-needed for downstream use
- Version fallback logic in auto-release Create GitHub Release step

### Changed

- Updated `actions/checkout` from v4 to v6
- Updated `actions/cache` from v4 to v5
- Updated `peter-evans/create-pull-request` from v7 to v8
- Made `publish-crate.rs` fail (exit 1) when version already exists on crates.io
- Improved `create-github-release.rs` to check combined stdout+stderr and detect "Validation Failed"

### Fixed

- Fix publish steps overriding workflow-level CARGO_TOKEN fallback, breaking CARGO_REGISTRY_TOKEN-only configurations (#32)
- Fix non-fast-forward push failures in multi-workflow repos by adding fetch/rebase and push retry logic (#31)
- Add mono-repo path support to check-changelog-fragment.rs, check-version-modification.rs, and create-changelog-fragment.rs
- Add `!cancelled()` guard to test job condition to respect workflow cancellation

### Fixed

- Fixed `version-and-commit.rs` to check crates.io instead of git tags for determining if a version is already released
- This prevents the release pipeline from getting stuck when git tags exist without corresponding crates.io publication

### Added

- Added `--tag-prefix` support to `version-and-commit.rs` for multi-language repository compatibility
- Added crates.io and docs.rs badges to README.md
- Added automatic crates.io and docs.rs badge injection in GitHub release notes
- Added documentation deployment job to CI/CD pipeline (deploys to GitHub Pages after release)
- Added case study documentation for issue #25

### Fixed
- Fixed unsupported look-ahead regex in `create-github-release.rs` that caused a panic when parsing CHANGELOG.md. Replaced with a two-step approach using only features supported by Rust's `regex` crate.

### Changed
- Restructured example application as a simple CLI sum calculator using `lino-arguments`
- Renamed default package to `example-sum-package-name` with Unlicense license
- Reorganized test structure: `tests/unit/sum.rs`, `tests/integration/sum.rs`, `tests/unit/ci-cd/`
- Converted experiment scripts into proper unit tests in `tests/unit/ci-cd/changelog_parsing.rs`
- Added CI/CD skip logic for template default package name `example-sum-package-name`
- Updated README.md badges and documentation

### Fixed

- Change detection script now uses per-commit diff instead of full PR diff, so commits touching only non-code files correctly skip CI jobs even when earlier commits in the same PR changed code files

### Fixed
- Make release scripts resolve the publishable crate manifest when the repository root uses a Cargo workspace manifest.

### Fixed
- Decoupled GitHub Pages documentation deployment from package release publication and fixed release-script warning failures under `RUSTFLAGS=-Dwarnings`.

### Changed
- Added explicit GitHub Actions job timeouts and documented Rust test timeout guidance.

### Fixed
- Added a non-blocking warning threshold to the Rust file-size check so near-limit files are surfaced before concurrent PR merges can exceed the hard limit.

### Fixed
- Made `create-github-release.rs` build GitHub release titles as `[Language] X.Y.Z` instead of reusing the tag prefix.

### Added
- Added optional Docker Hub image publishing tied to Rust crate releases, including crates.io visibility waiting, version/latest image tags, and Docker Hub badges in GitHub release notes.

### Changed
- Release completeness checks now self-heal when crates.io exists but configured Docker Hub or GitHub release artifacts are missing.

### Fixed
- Switched documentation deployment to the official GitHub Pages artifact workflow so repositories using GitHub Actions as their Pages source do not get false-positive branch-push deploys.

### Added
- Documented the one-time `Settings → Pages → Source = GitHub Actions` prerequisite for the `deploy-docs` job in `README.md` and as a comment above the `deploy-docs` job in `release.yml`, so downstream template users hit a documented setup step instead of a `Get Pages site failed` error on the first deploy.

### Added
- Tracking case study at `docs/case-studies/issue-52/` registering the `browser-commander` + Playwright preview-regeneration pattern from [`konard/vk-bot-desktop#52`](https://github.com/konard/vk-bot-desktop/pull/52), with an activation checklist for when an example-app surface lands in this template. Documentation only — no workflow, script, or runtime code changes. Primary upstream tracking issue: [`link-foundation/js-ai-driven-development-pipeline-template#62`](https://github.com/link-foundation/js-ai-driven-development-pipeline-template/issues/62).

## [0.13.0] - 2026-05-12

### Added
- Changeset-style fragment format with frontmatter for specifying version bump type
- New `get-bump-type.mjs` script to automatically determine version bump from fragments
- Automatic version bumping on merge to main based on changelog fragments
- Detailed documentation for the changelog fragment system in `changelog.d/README.md`

### Changed
- Updated `collect-changelog.mjs` to strip frontmatter when collecting fragments
- Updated `version-and-commit.mjs` to handle frontmatter in fragments
- Enhanced release workflow to automatically determine bump type from changesets

### Changed
- Add `detect-changes` job with cross-platform `detect-code-changes.mjs` script
- Make lint job independent of changelog check (runs based on file changes only)
- Allow docs-only PRs without changelog fragment requirement
- Handle changelog check 'skipped' state in dependent jobs
- Exclude `changelog.d/`, `docs/`, `experiments/`, `examples/` folders and markdown files from code changes detection

### Fixed
- Fixed README.md to correctly reference Node.js scripts (`.mjs`) instead of Python scripts (`.py`)
- Updated project structure in README.md to match actual script files in `scripts/` directory
- Fixed example code in README.md that had invalid Rust with two `main` functions

### Added

- Added crates.io publishing support to CI/CD workflow
- Added `release_mode` input with "instant" and "changelog-pr" options for manual releases
- Added `--tag-prefix` and `--crates-io-url` options to create-github-release.mjs script
- Added comprehensive case study documentation for Issue #11 in docs/case-studies/issue-11/

### Changed

- Changed changelog fragment check from warning to error (exit 1) to enforce changelog requirements
- Updated job conditions with `always() && !cancelled()` to fix workflow_dispatch job skipping issue
- Renamed manual-release job to "Instant Release" for clarity

### Fixed

- Fixed deprecated `::set-output` GitHub Actions command in version-and-commit.mjs
- Fixed workflow_dispatch triggering issues where lint/build/release jobs were incorrectly skipped

### Fixed

- Fixed changelog fragment check to validate that a fragment is **added in the PR diff** rather than just checking if any fragments exist in the directory. This prevents the check from incorrectly passing when there are leftover fragments from previous PRs that haven't been released yet.

### Changed

- Converted shell scripts in `release.yml` to cross-platform `.mjs` scripts for improved portability and performance:
  - `check-changelog-fragment.mjs` - validates changelog fragment is added in PR diff
  - `git-config.mjs` - configures git user for CI/CD
  - `check-release-needed.mjs` - checks if release is needed
  - `publish-crate.mjs` - publishes package to crates.io
  - `create-changelog-fragment.mjs` - creates changelog fragments for manual releases
  - `get-version.mjs` - gets current version from Cargo.toml

### Added

- Added `check-version-modification.mjs` script to detect manual version changes in Cargo.toml
- Added `version-check` job to CI/CD workflow that runs on pull requests
- Added skip logic for automated release branches (changelog-manual-release-*, changeset-release/*, release/*, automated-release/*)

### Changed

- Version modifications in Cargo.toml are now blocked in pull requests to enforce automated release pipeline

### Added

- Added support for `CARGO_REGISTRY_TOKEN` as alternative to `CARGO_TOKEN` for crates.io publishing
- Added case study documentation for Issue #17 (yargs reserved word and dual token support)

### Changed

- Updated workflow to use fallback logic: `${{ secrets.CARGO_REGISTRY_TOKEN || secrets.CARGO_TOKEN }}`
- Improved publish-crate.mjs to check both `CARGO_REGISTRY_TOKEN` and `CARGO_TOKEN` environment variables
- Added warning message when neither token is set

### Added
- New `scripts/rust-paths.mjs` utility for automatic Rust package root detection
- Support for both single-language and multi-language repository structures in all CI/CD scripts
- Configuration options via `--rust-root` CLI argument and `RUST_ROOT` environment variable
- Comprehensive case study documentation in `docs/case-studies/issue-19/`

### Changed
- Updated all release scripts to use the new path detection utility:
  - `scripts/bump-version.mjs`
  - `scripts/check-release-needed.mjs`
  - `scripts/collect-changelog.mjs`
  - `scripts/get-bump-type.mjs`
  - `scripts/get-version.mjs`
  - `scripts/publish-crate.mjs`
  - `scripts/version-and-commit.mjs`

### Changed

- **check-release-needed.mjs**: Now checks crates.io API directly instead of git tags to determine if a version is already released. This prevents false positives where git tags exist but the package was never actually published to crates.io.

### Added

- **CI/CD Troubleshooting Guide**: New documentation at `docs/ci-cd/troubleshooting.md` covering common issues like skipped jobs, false positive version checks, publishing failures, and secret configuration.

- **Enhanced Error Handling in publish-crate.mjs**: Added specific detection and helpful error messages for authentication failures, including guidance on secret configuration and workflow setup.

- **Case Study Documentation**: Added comprehensive case study at `docs/case-studies/issue-21/` analyzing CI/CD failures from browser-commander repository (issues #27, #29, #31, #33) with timeline, root causes, and lessons learned.

### Fixed

- **Prevent False Positive Version Checks**: The release workflow now correctly identifies unpublished versions by checking crates.io instead of relying on git tags, which can exist without the package being published.

### Changed

- Translated all CI/CD scripts from JavaScript (.mjs) to Rust (.rs) using rust-script
- Scripts now use native Rust with rust-script for execution in shell
- Removed Node.js dependency from CI/CD pipeline
- Updated GitHub Actions workflow to use rust-script instead of node
- Updated README and CONTRIBUTING documentation with new script references

### Added

- Cache `restore-keys` for partial cache hits across all workflow jobs
- Explicit `token` parameter in checkout for release jobs
- Code coverage job with `cargo-llvm-cov` and Codecov integration
- Codecov badge in README.md
- Pre-release version support (e.g., `0.1.0-beta.1`) in version parsing
- `--release-label` parameter for multi-language release disambiguation
- `ensure_version_exceeds_published()` logic to prevent publishing duplicate versions
- `get_max_published_version()` to query highest non-yanked version from crates.io
- `max_published_version` output from check-release-needed for downstream use
- Version fallback logic in auto-release Create GitHub Release step

### Changed

- Updated `actions/checkout` from v4 to v6
- Updated `actions/cache` from v4 to v5
- Updated `peter-evans/create-pull-request` from v7 to v8
- Made `publish-crate.rs` fail (exit 1) when version already exists on crates.io
- Improved `create-github-release.rs` to check combined stdout+stderr and detect "Validation Failed"

### Fixed

- Fix publish steps overriding workflow-level CARGO_TOKEN fallback, breaking CARGO_REGISTRY_TOKEN-only configurations (#32)
- Fix non-fast-forward push failures in multi-workflow repos by adding fetch/rebase and push retry logic (#31)
- Add mono-repo path support to check-changelog-fragment.rs, check-version-modification.rs, and create-changelog-fragment.rs
- Add `!cancelled()` guard to test job condition to respect workflow cancellation

### Fixed

- Fixed `version-and-commit.rs` to check crates.io instead of git tags for determining if a version is already released
- This prevents the release pipeline from getting stuck when git tags exist without corresponding crates.io publication

### Added

- Added `--tag-prefix` support to `version-and-commit.rs` for multi-language repository compatibility
- Added crates.io and docs.rs badges to README.md
- Added automatic crates.io and docs.rs badge injection in GitHub release notes
- Added documentation deployment job to CI/CD pipeline (deploys to GitHub Pages after release)
- Added case study documentation for issue #25

### Fixed
- Fixed unsupported look-ahead regex in `create-github-release.rs` that caused a panic when parsing CHANGELOG.md. Replaced with a two-step approach using only features supported by Rust's `regex` crate.

### Changed
- Restructured example application as a simple CLI sum calculator using `lino-arguments`
- Renamed default package to `example-sum-package-name` with Unlicense license
- Reorganized test structure: `tests/unit/sum.rs`, `tests/integration/sum.rs`, `tests/unit/ci-cd/`
- Converted experiment scripts into proper unit tests in `tests/unit/ci-cd/changelog_parsing.rs`
- Added CI/CD skip logic for template default package name `example-sum-package-name`
- Updated README.md badges and documentation

### Fixed

- Change detection script now uses per-commit diff instead of full PR diff, so commits touching only non-code files correctly skip CI jobs even when earlier commits in the same PR changed code files

### Fixed
- Make release scripts resolve the publishable crate manifest when the repository root uses a Cargo workspace manifest.

### Fixed
- Decoupled GitHub Pages documentation deployment from package release publication and fixed release-script warning failures under `RUSTFLAGS=-Dwarnings`.

### Changed
- Added explicit GitHub Actions job timeouts and documented Rust test timeout guidance.

### Fixed
- Added a non-blocking warning threshold to the Rust file-size check so near-limit files are surfaced before concurrent PR merges can exceed the hard limit.

### Fixed
- Made `create-github-release.rs` build GitHub release titles as `[Language] X.Y.Z` instead of reusing the tag prefix.

### Added
- Added optional Docker Hub image publishing tied to Rust crate releases, including crates.io visibility waiting, version/latest image tags, and Docker Hub badges in GitHub release notes.

### Changed
- Release completeness checks now self-heal when crates.io exists but configured Docker Hub or GitHub release artifacts are missing.

### Fixed
- Switched documentation deployment to the official GitHub Pages artifact workflow so repositories using GitHub Actions as their Pages source do not get false-positive branch-push deploys.

### Added
- Documented the one-time `Settings → Pages → Source = GitHub Actions` prerequisite for the `deploy-docs` job in `README.md` and as a comment above the `deploy-docs` job in `release.yml`, so downstream template users hit a documented setup step instead of a `Get Pages site failed` error on the first deploy.

## [0.12.0] - 2026-05-12

### Added
- Changeset-style fragment format with frontmatter for specifying version bump type
- New `get-bump-type.mjs` script to automatically determine version bump from fragments
- Automatic version bumping on merge to main based on changelog fragments
- Detailed documentation for the changelog fragment system in `changelog.d/README.md`

### Changed
- Updated `collect-changelog.mjs` to strip frontmatter when collecting fragments
- Updated `version-and-commit.mjs` to handle frontmatter in fragments
- Enhanced release workflow to automatically determine bump type from changesets

### Changed
- Add `detect-changes` job with cross-platform `detect-code-changes.mjs` script
- Make lint job independent of changelog check (runs based on file changes only)
- Allow docs-only PRs without changelog fragment requirement
- Handle changelog check 'skipped' state in dependent jobs
- Exclude `changelog.d/`, `docs/`, `experiments/`, `examples/` folders and markdown files from code changes detection

### Fixed
- Fixed README.md to correctly reference Node.js scripts (`.mjs`) instead of Python scripts (`.py`)
- Updated project structure in README.md to match actual script files in `scripts/` directory
- Fixed example code in README.md that had invalid Rust with two `main` functions

### Added

- Added crates.io publishing support to CI/CD workflow
- Added `release_mode` input with "instant" and "changelog-pr" options for manual releases
- Added `--tag-prefix` and `--crates-io-url` options to create-github-release.mjs script
- Added comprehensive case study documentation for Issue #11 in docs/case-studies/issue-11/

### Changed

- Changed changelog fragment check from warning to error (exit 1) to enforce changelog requirements
- Updated job conditions with `always() && !cancelled()` to fix workflow_dispatch job skipping issue
- Renamed manual-release job to "Instant Release" for clarity

### Fixed

- Fixed deprecated `::set-output` GitHub Actions command in version-and-commit.mjs
- Fixed workflow_dispatch triggering issues where lint/build/release jobs were incorrectly skipped

### Fixed

- Fixed changelog fragment check to validate that a fragment is **added in the PR diff** rather than just checking if any fragments exist in the directory. This prevents the check from incorrectly passing when there are leftover fragments from previous PRs that haven't been released yet.

### Changed

- Converted shell scripts in `release.yml` to cross-platform `.mjs` scripts for improved portability and performance:
  - `check-changelog-fragment.mjs` - validates changelog fragment is added in PR diff
  - `git-config.mjs` - configures git user for CI/CD
  - `check-release-needed.mjs` - checks if release is needed
  - `publish-crate.mjs` - publishes package to crates.io
  - `create-changelog-fragment.mjs` - creates changelog fragments for manual releases
  - `get-version.mjs` - gets current version from Cargo.toml

### Added

- Added `check-version-modification.mjs` script to detect manual version changes in Cargo.toml
- Added `version-check` job to CI/CD workflow that runs on pull requests
- Added skip logic for automated release branches (changelog-manual-release-*, changeset-release/*, release/*, automated-release/*)

### Changed

- Version modifications in Cargo.toml are now blocked in pull requests to enforce automated release pipeline

### Added

- Added support for `CARGO_REGISTRY_TOKEN` as alternative to `CARGO_TOKEN` for crates.io publishing
- Added case study documentation for Issue #17 (yargs reserved word and dual token support)

### Changed

- Updated workflow to use fallback logic: `${{ secrets.CARGO_REGISTRY_TOKEN || secrets.CARGO_TOKEN }}`
- Improved publish-crate.mjs to check both `CARGO_REGISTRY_TOKEN` and `CARGO_TOKEN` environment variables
- Added warning message when neither token is set

### Added
- New `scripts/rust-paths.mjs` utility for automatic Rust package root detection
- Support for both single-language and multi-language repository structures in all CI/CD scripts
- Configuration options via `--rust-root` CLI argument and `RUST_ROOT` environment variable
- Comprehensive case study documentation in `docs/case-studies/issue-19/`

### Changed
- Updated all release scripts to use the new path detection utility:
  - `scripts/bump-version.mjs`
  - `scripts/check-release-needed.mjs`
  - `scripts/collect-changelog.mjs`
  - `scripts/get-bump-type.mjs`
  - `scripts/get-version.mjs`
  - `scripts/publish-crate.mjs`
  - `scripts/version-and-commit.mjs`

### Changed

- **check-release-needed.mjs**: Now checks crates.io API directly instead of git tags to determine if a version is already released. This prevents false positives where git tags exist but the package was never actually published to crates.io.

### Added

- **CI/CD Troubleshooting Guide**: New documentation at `docs/ci-cd/troubleshooting.md` covering common issues like skipped jobs, false positive version checks, publishing failures, and secret configuration.

- **Enhanced Error Handling in publish-crate.mjs**: Added specific detection and helpful error messages for authentication failures, including guidance on secret configuration and workflow setup.

- **Case Study Documentation**: Added comprehensive case study at `docs/case-studies/issue-21/` analyzing CI/CD failures from browser-commander repository (issues #27, #29, #31, #33) with timeline, root causes, and lessons learned.

### Fixed

- **Prevent False Positive Version Checks**: The release workflow now correctly identifies unpublished versions by checking crates.io instead of relying on git tags, which can exist without the package being published.

### Changed

- Translated all CI/CD scripts from JavaScript (.mjs) to Rust (.rs) using rust-script
- Scripts now use native Rust with rust-script for execution in shell
- Removed Node.js dependency from CI/CD pipeline
- Updated GitHub Actions workflow to use rust-script instead of node
- Updated README and CONTRIBUTING documentation with new script references

### Added

- Cache `restore-keys` for partial cache hits across all workflow jobs
- Explicit `token` parameter in checkout for release jobs
- Code coverage job with `cargo-llvm-cov` and Codecov integration
- Codecov badge in README.md
- Pre-release version support (e.g., `0.1.0-beta.1`) in version parsing
- `--release-label` parameter for multi-language release disambiguation
- `ensure_version_exceeds_published()` logic to prevent publishing duplicate versions
- `get_max_published_version()` to query highest non-yanked version from crates.io
- `max_published_version` output from check-release-needed for downstream use
- Version fallback logic in auto-release Create GitHub Release step

### Changed

- Updated `actions/checkout` from v4 to v6
- Updated `actions/cache` from v4 to v5
- Updated `peter-evans/create-pull-request` from v7 to v8
- Made `publish-crate.rs` fail (exit 1) when version already exists on crates.io
- Improved `create-github-release.rs` to check combined stdout+stderr and detect "Validation Failed"

### Fixed

- Fix publish steps overriding workflow-level CARGO_TOKEN fallback, breaking CARGO_REGISTRY_TOKEN-only configurations (#32)
- Fix non-fast-forward push failures in multi-workflow repos by adding fetch/rebase and push retry logic (#31)
- Add mono-repo path support to check-changelog-fragment.rs, check-version-modification.rs, and create-changelog-fragment.rs
- Add `!cancelled()` guard to test job condition to respect workflow cancellation

### Fixed

- Fixed `version-and-commit.rs` to check crates.io instead of git tags for determining if a version is already released
- This prevents the release pipeline from getting stuck when git tags exist without corresponding crates.io publication

### Added

- Added `--tag-prefix` support to `version-and-commit.rs` for multi-language repository compatibility
- Added crates.io and docs.rs badges to README.md
- Added automatic crates.io and docs.rs badge injection in GitHub release notes
- Added documentation deployment job to CI/CD pipeline (deploys to GitHub Pages after release)
- Added case study documentation for issue #25

### Fixed
- Fixed unsupported look-ahead regex in `create-github-release.rs` that caused a panic when parsing CHANGELOG.md. Replaced with a two-step approach using only features supported by Rust's `regex` crate.

### Changed
- Restructured example application as a simple CLI sum calculator using `lino-arguments`
- Renamed default package to `example-sum-package-name` with Unlicense license
- Reorganized test structure: `tests/unit/sum.rs`, `tests/integration/sum.rs`, `tests/unit/ci-cd/`
- Converted experiment scripts into proper unit tests in `tests/unit/ci-cd/changelog_parsing.rs`
- Added CI/CD skip logic for template default package name `example-sum-package-name`
- Updated README.md badges and documentation

### Fixed

- Change detection script now uses per-commit diff instead of full PR diff, so commits touching only non-code files correctly skip CI jobs even when earlier commits in the same PR changed code files

### Fixed
- Make release scripts resolve the publishable crate manifest when the repository root uses a Cargo workspace manifest.

### Fixed
- Decoupled GitHub Pages documentation deployment from package release publication and fixed release-script warning failures under `RUSTFLAGS=-Dwarnings`.

### Changed
- Added explicit GitHub Actions job timeouts and documented Rust test timeout guidance.

### Fixed
- Added a non-blocking warning threshold to the Rust file-size check so near-limit files are surfaced before concurrent PR merges can exceed the hard limit.

### Fixed
- Made `create-github-release.rs` build GitHub release titles as `[Language] X.Y.Z` instead of reusing the tag prefix.

### Added
- Added optional Docker Hub image publishing tied to Rust crate releases, including crates.io visibility waiting, version/latest image tags, and Docker Hub badges in GitHub release notes.

### Changed
- Release completeness checks now self-heal when crates.io exists but configured Docker Hub or GitHub release artifacts are missing.

### Fixed
- Switched documentation deployment to the official GitHub Pages artifact workflow so repositories using GitHub Actions as their Pages source do not get false-positive branch-push deploys.

## [0.11.0] - 2026-05-09

### Added
- Changeset-style fragment format with frontmatter for specifying version bump type
- New `get-bump-type.mjs` script to automatically determine version bump from fragments
- Automatic version bumping on merge to main based on changelog fragments
- Detailed documentation for the changelog fragment system in `changelog.d/README.md`

### Changed
- Updated `collect-changelog.mjs` to strip frontmatter when collecting fragments
- Updated `version-and-commit.mjs` to handle frontmatter in fragments
- Enhanced release workflow to automatically determine bump type from changesets

### Changed
- Add `detect-changes` job with cross-platform `detect-code-changes.mjs` script
- Make lint job independent of changelog check (runs based on file changes only)
- Allow docs-only PRs without changelog fragment requirement
- Handle changelog check 'skipped' state in dependent jobs
- Exclude `changelog.d/`, `docs/`, `experiments/`, `examples/` folders and markdown files from code changes detection

### Fixed
- Fixed README.md to correctly reference Node.js scripts (`.mjs`) instead of Python scripts (`.py`)
- Updated project structure in README.md to match actual script files in `scripts/` directory
- Fixed example code in README.md that had invalid Rust with two `main` functions

### Added

- Added crates.io publishing support to CI/CD workflow
- Added `release_mode` input with "instant" and "changelog-pr" options for manual releases
- Added `--tag-prefix` and `--crates-io-url` options to create-github-release.mjs script
- Added comprehensive case study documentation for Issue #11 in docs/case-studies/issue-11/

### Changed

- Changed changelog fragment check from warning to error (exit 1) to enforce changelog requirements
- Updated job conditions with `always() && !cancelled()` to fix workflow_dispatch job skipping issue
- Renamed manual-release job to "Instant Release" for clarity

### Fixed

- Fixed deprecated `::set-output` GitHub Actions command in version-and-commit.mjs
- Fixed workflow_dispatch triggering issues where lint/build/release jobs were incorrectly skipped

### Fixed

- Fixed changelog fragment check to validate that a fragment is **added in the PR diff** rather than just checking if any fragments exist in the directory. This prevents the check from incorrectly passing when there are leftover fragments from previous PRs that haven't been released yet.

### Changed

- Converted shell scripts in `release.yml` to cross-platform `.mjs` scripts for improved portability and performance:
  - `check-changelog-fragment.mjs` - validates changelog fragment is added in PR diff
  - `git-config.mjs` - configures git user for CI/CD
  - `check-release-needed.mjs` - checks if release is needed
  - `publish-crate.mjs` - publishes package to crates.io
  - `create-changelog-fragment.mjs` - creates changelog fragments for manual releases
  - `get-version.mjs` - gets current version from Cargo.toml

### Added

- Added `check-version-modification.mjs` script to detect manual version changes in Cargo.toml
- Added `version-check` job to CI/CD workflow that runs on pull requests
- Added skip logic for automated release branches (changelog-manual-release-*, changeset-release/*, release/*, automated-release/*)

### Changed

- Version modifications in Cargo.toml are now blocked in pull requests to enforce automated release pipeline

### Added

- Added support for `CARGO_REGISTRY_TOKEN` as alternative to `CARGO_TOKEN` for crates.io publishing
- Added case study documentation for Issue #17 (yargs reserved word and dual token support)

### Changed

- Updated workflow to use fallback logic: `${{ secrets.CARGO_REGISTRY_TOKEN || secrets.CARGO_TOKEN }}`
- Improved publish-crate.mjs to check both `CARGO_REGISTRY_TOKEN` and `CARGO_TOKEN` environment variables
- Added warning message when neither token is set

### Added
- New `scripts/rust-paths.mjs` utility for automatic Rust package root detection
- Support for both single-language and multi-language repository structures in all CI/CD scripts
- Configuration options via `--rust-root` CLI argument and `RUST_ROOT` environment variable
- Comprehensive case study documentation in `docs/case-studies/issue-19/`

### Changed
- Updated all release scripts to use the new path detection utility:
  - `scripts/bump-version.mjs`
  - `scripts/check-release-needed.mjs`
  - `scripts/collect-changelog.mjs`
  - `scripts/get-bump-type.mjs`
  - `scripts/get-version.mjs`
  - `scripts/publish-crate.mjs`
  - `scripts/version-and-commit.mjs`

### Changed

- **check-release-needed.mjs**: Now checks crates.io API directly instead of git tags to determine if a version is already released. This prevents false positives where git tags exist but the package was never actually published to crates.io.

### Added

- **CI/CD Troubleshooting Guide**: New documentation at `docs/ci-cd/troubleshooting.md` covering common issues like skipped jobs, false positive version checks, publishing failures, and secret configuration.

- **Enhanced Error Handling in publish-crate.mjs**: Added specific detection and helpful error messages for authentication failures, including guidance on secret configuration and workflow setup.

- **Case Study Documentation**: Added comprehensive case study at `docs/case-studies/issue-21/` analyzing CI/CD failures from browser-commander repository (issues #27, #29, #31, #33) with timeline, root causes, and lessons learned.

### Fixed

- **Prevent False Positive Version Checks**: The release workflow now correctly identifies unpublished versions by checking crates.io instead of relying on git tags, which can exist without the package being published.

### Changed

- Translated all CI/CD scripts from JavaScript (.mjs) to Rust (.rs) using rust-script
- Scripts now use native Rust with rust-script for execution in shell
- Removed Node.js dependency from CI/CD pipeline
- Updated GitHub Actions workflow to use rust-script instead of node
- Updated README and CONTRIBUTING documentation with new script references

### Added

- Cache `restore-keys` for partial cache hits across all workflow jobs
- Explicit `token` parameter in checkout for release jobs
- Code coverage job with `cargo-llvm-cov` and Codecov integration
- Codecov badge in README.md
- Pre-release version support (e.g., `0.1.0-beta.1`) in version parsing
- `--release-label` parameter for multi-language release disambiguation
- `ensure_version_exceeds_published()` logic to prevent publishing duplicate versions
- `get_max_published_version()` to query highest non-yanked version from crates.io
- `max_published_version` output from check-release-needed for downstream use
- Version fallback logic in auto-release Create GitHub Release step

### Changed

- Updated `actions/checkout` from v4 to v6
- Updated `actions/cache` from v4 to v5
- Updated `peter-evans/create-pull-request` from v7 to v8
- Made `publish-crate.rs` fail (exit 1) when version already exists on crates.io
- Improved `create-github-release.rs` to check combined stdout+stderr and detect "Validation Failed"

### Fixed

- Fix publish steps overriding workflow-level CARGO_TOKEN fallback, breaking CARGO_REGISTRY_TOKEN-only configurations (#32)
- Fix non-fast-forward push failures in multi-workflow repos by adding fetch/rebase and push retry logic (#31)
- Add mono-repo path support to check-changelog-fragment.rs, check-version-modification.rs, and create-changelog-fragment.rs
- Add `!cancelled()` guard to test job condition to respect workflow cancellation

### Fixed

- Fixed `version-and-commit.rs` to check crates.io instead of git tags for determining if a version is already released
- This prevents the release pipeline from getting stuck when git tags exist without corresponding crates.io publication

### Added

- Added `--tag-prefix` support to `version-and-commit.rs` for multi-language repository compatibility
- Added crates.io and docs.rs badges to README.md
- Added automatic crates.io and docs.rs badge injection in GitHub release notes
- Added documentation deployment job to CI/CD pipeline (deploys to GitHub Pages after release)
- Added case study documentation for issue #25

### Fixed
- Fixed unsupported look-ahead regex in `create-github-release.rs` that caused a panic when parsing CHANGELOG.md. Replaced with a two-step approach using only features supported by Rust's `regex` crate.

### Changed
- Restructured example application as a simple CLI sum calculator using `lino-arguments`
- Renamed default package to `example-sum-package-name` with Unlicense license
- Reorganized test structure: `tests/unit/sum.rs`, `tests/integration/sum.rs`, `tests/unit/ci-cd/`
- Converted experiment scripts into proper unit tests in `tests/unit/ci-cd/changelog_parsing.rs`
- Added CI/CD skip logic for template default package name `example-sum-package-name`
- Updated README.md badges and documentation

### Fixed

- Change detection script now uses per-commit diff instead of full PR diff, so commits touching only non-code files correctly skip CI jobs even when earlier commits in the same PR changed code files

### Fixed
- Make release scripts resolve the publishable crate manifest when the repository root uses a Cargo workspace manifest.

### Fixed
- Decoupled GitHub Pages documentation deployment from package release publication and fixed release-script warning failures under `RUSTFLAGS=-Dwarnings`.

### Changed
- Added explicit GitHub Actions job timeouts and documented Rust test timeout guidance.

### Fixed
- Added a non-blocking warning threshold to the Rust file-size check so near-limit files are surfaced before concurrent PR merges can exceed the hard limit.

### Fixed
- Made `create-github-release.rs` build GitHub release titles as `[Language] X.Y.Z` instead of reusing the tag prefix.

### Added
- Added optional Docker Hub image publishing tied to Rust crate releases, including crates.io visibility waiting, version/latest image tags, and Docker Hub badges in GitHub release notes.

### Changed
- Release completeness checks now self-heal when crates.io exists but configured Docker Hub or GitHub release artifacts are missing.

## [0.10.0] - 2026-05-09

### Added
- Changeset-style fragment format with frontmatter for specifying version bump type
- New `get-bump-type.mjs` script to automatically determine version bump from fragments
- Automatic version bumping on merge to main based on changelog fragments
- Detailed documentation for the changelog fragment system in `changelog.d/README.md`

### Changed
- Updated `collect-changelog.mjs` to strip frontmatter when collecting fragments
- Updated `version-and-commit.mjs` to handle frontmatter in fragments
- Enhanced release workflow to automatically determine bump type from changesets

### Changed
- Add `detect-changes` job with cross-platform `detect-code-changes.mjs` script
- Make lint job independent of changelog check (runs based on file changes only)
- Allow docs-only PRs without changelog fragment requirement
- Handle changelog check 'skipped' state in dependent jobs
- Exclude `changelog.d/`, `docs/`, `experiments/`, `examples/` folders and markdown files from code changes detection

### Fixed
- Fixed README.md to correctly reference Node.js scripts (`.mjs`) instead of Python scripts (`.py`)
- Updated project structure in README.md to match actual script files in `scripts/` directory
- Fixed example code in README.md that had invalid Rust with two `main` functions

### Added

- Added crates.io publishing support to CI/CD workflow
- Added `release_mode` input with "instant" and "changelog-pr" options for manual releases
- Added `--tag-prefix` and `--crates-io-url` options to create-github-release.mjs script
- Added comprehensive case study documentation for Issue #11 in docs/case-studies/issue-11/

### Changed

- Changed changelog fragment check from warning to error (exit 1) to enforce changelog requirements
- Updated job conditions with `always() && !cancelled()` to fix workflow_dispatch job skipping issue
- Renamed manual-release job to "Instant Release" for clarity

### Fixed

- Fixed deprecated `::set-output` GitHub Actions command in version-and-commit.mjs
- Fixed workflow_dispatch triggering issues where lint/build/release jobs were incorrectly skipped

### Fixed

- Fixed changelog fragment check to validate that a fragment is **added in the PR diff** rather than just checking if any fragments exist in the directory. This prevents the check from incorrectly passing when there are leftover fragments from previous PRs that haven't been released yet.

### Changed

- Converted shell scripts in `release.yml` to cross-platform `.mjs` scripts for improved portability and performance:
  - `check-changelog-fragment.mjs` - validates changelog fragment is added in PR diff
  - `git-config.mjs` - configures git user for CI/CD
  - `check-release-needed.mjs` - checks if release is needed
  - `publish-crate.mjs` - publishes package to crates.io
  - `create-changelog-fragment.mjs` - creates changelog fragments for manual releases
  - `get-version.mjs` - gets current version from Cargo.toml

### Added

- Added `check-version-modification.mjs` script to detect manual version changes in Cargo.toml
- Added `version-check` job to CI/CD workflow that runs on pull requests
- Added skip logic for automated release branches (changelog-manual-release-*, changeset-release/*, release/*, automated-release/*)

### Changed

- Version modifications in Cargo.toml are now blocked in pull requests to enforce automated release pipeline

### Added

- Added support for `CARGO_REGISTRY_TOKEN` as alternative to `CARGO_TOKEN` for crates.io publishing
- Added case study documentation for Issue #17 (yargs reserved word and dual token support)

### Changed

- Updated workflow to use fallback logic: `${{ secrets.CARGO_REGISTRY_TOKEN || secrets.CARGO_TOKEN }}`
- Improved publish-crate.mjs to check both `CARGO_REGISTRY_TOKEN` and `CARGO_TOKEN` environment variables
- Added warning message when neither token is set

### Added
- New `scripts/rust-paths.mjs` utility for automatic Rust package root detection
- Support for both single-language and multi-language repository structures in all CI/CD scripts
- Configuration options via `--rust-root` CLI argument and `RUST_ROOT` environment variable
- Comprehensive case study documentation in `docs/case-studies/issue-19/`

### Changed
- Updated all release scripts to use the new path detection utility:
  - `scripts/bump-version.mjs`
  - `scripts/check-release-needed.mjs`
  - `scripts/collect-changelog.mjs`
  - `scripts/get-bump-type.mjs`
  - `scripts/get-version.mjs`
  - `scripts/publish-crate.mjs`
  - `scripts/version-and-commit.mjs`

### Changed

- **check-release-needed.mjs**: Now checks crates.io API directly instead of git tags to determine if a version is already released. This prevents false positives where git tags exist but the package was never actually published to crates.io.

### Added

- **CI/CD Troubleshooting Guide**: New documentation at `docs/ci-cd/troubleshooting.md` covering common issues like skipped jobs, false positive version checks, publishing failures, and secret configuration.

- **Enhanced Error Handling in publish-crate.mjs**: Added specific detection and helpful error messages for authentication failures, including guidance on secret configuration and workflow setup.

- **Case Study Documentation**: Added comprehensive case study at `docs/case-studies/issue-21/` analyzing CI/CD failures from browser-commander repository (issues #27, #29, #31, #33) with timeline, root causes, and lessons learned.

### Fixed

- **Prevent False Positive Version Checks**: The release workflow now correctly identifies unpublished versions by checking crates.io instead of relying on git tags, which can exist without the package being published.

### Changed

- Translated all CI/CD scripts from JavaScript (.mjs) to Rust (.rs) using rust-script
- Scripts now use native Rust with rust-script for execution in shell
- Removed Node.js dependency from CI/CD pipeline
- Updated GitHub Actions workflow to use rust-script instead of node
- Updated README and CONTRIBUTING documentation with new script references

### Added

- Cache `restore-keys` for partial cache hits across all workflow jobs
- Explicit `token` parameter in checkout for release jobs
- Code coverage job with `cargo-llvm-cov` and Codecov integration
- Codecov badge in README.md
- Pre-release version support (e.g., `0.1.0-beta.1`) in version parsing
- `--release-label` parameter for multi-language release disambiguation
- `ensure_version_exceeds_published()` logic to prevent publishing duplicate versions
- `get_max_published_version()` to query highest non-yanked version from crates.io
- `max_published_version` output from check-release-needed for downstream use
- Version fallback logic in auto-release Create GitHub Release step

### Changed

- Updated `actions/checkout` from v4 to v6
- Updated `actions/cache` from v4 to v5
- Updated `peter-evans/create-pull-request` from v7 to v8
- Made `publish-crate.rs` fail (exit 1) when version already exists on crates.io
- Improved `create-github-release.rs` to check combined stdout+stderr and detect "Validation Failed"

### Fixed

- Fix publish steps overriding workflow-level CARGO_TOKEN fallback, breaking CARGO_REGISTRY_TOKEN-only configurations (#32)
- Fix non-fast-forward push failures in multi-workflow repos by adding fetch/rebase and push retry logic (#31)
- Add mono-repo path support to check-changelog-fragment.rs, check-version-modification.rs, and create-changelog-fragment.rs
- Add `!cancelled()` guard to test job condition to respect workflow cancellation

### Fixed

- Fixed `version-and-commit.rs` to check crates.io instead of git tags for determining if a version is already released
- This prevents the release pipeline from getting stuck when git tags exist without corresponding crates.io publication

### Added

- Added `--tag-prefix` support to `version-and-commit.rs` for multi-language repository compatibility
- Added crates.io and docs.rs badges to README.md
- Added automatic crates.io and docs.rs badge injection in GitHub release notes
- Added documentation deployment job to CI/CD pipeline (deploys to GitHub Pages after release)
- Added case study documentation for issue #25

### Fixed
- Fixed unsupported look-ahead regex in `create-github-release.rs` that caused a panic when parsing CHANGELOG.md. Replaced with a two-step approach using only features supported by Rust's `regex` crate.

### Changed
- Restructured example application as a simple CLI sum calculator using `lino-arguments`
- Renamed default package to `example-sum-package-name` with Unlicense license
- Reorganized test structure: `tests/unit/sum.rs`, `tests/integration/sum.rs`, `tests/unit/ci-cd/`
- Converted experiment scripts into proper unit tests in `tests/unit/ci-cd/changelog_parsing.rs`
- Added CI/CD skip logic for template default package name `example-sum-package-name`
- Updated README.md badges and documentation

### Fixed

- Change detection script now uses per-commit diff instead of full PR diff, so commits touching only non-code files correctly skip CI jobs even when earlier commits in the same PR changed code files

### Fixed
- Make release scripts resolve the publishable crate manifest when the repository root uses a Cargo workspace manifest.

### Fixed
- Decoupled GitHub Pages documentation deployment from package release publication and fixed release-script warning failures under `RUSTFLAGS=-Dwarnings`.

### Changed
- Added explicit GitHub Actions job timeouts and documented Rust test timeout guidance.

### Fixed
- Added a non-blocking warning threshold to the Rust file-size check so near-limit files are surfaced before concurrent PR merges can exceed the hard limit.

### Fixed
- Made `create-github-release.rs` build GitHub release titles as `[Language] X.Y.Z` instead of reusing the tag prefix.

## [0.9.0] - 2026-05-03

### Added
- Changeset-style fragment format with frontmatter for specifying version bump type
- New `get-bump-type.mjs` script to automatically determine version bump from fragments
- Automatic version bumping on merge to main based on changelog fragments
- Detailed documentation for the changelog fragment system in `changelog.d/README.md`

### Changed
- Updated `collect-changelog.mjs` to strip frontmatter when collecting fragments
- Updated `version-and-commit.mjs` to handle frontmatter in fragments
- Enhanced release workflow to automatically determine bump type from changesets

### Changed
- Add `detect-changes` job with cross-platform `detect-code-changes.mjs` script
- Make lint job independent of changelog check (runs based on file changes only)
- Allow docs-only PRs without changelog fragment requirement
- Handle changelog check 'skipped' state in dependent jobs
- Exclude `changelog.d/`, `docs/`, `experiments/`, `examples/` folders and markdown files from code changes detection

### Fixed
- Fixed README.md to correctly reference Node.js scripts (`.mjs`) instead of Python scripts (`.py`)
- Updated project structure in README.md to match actual script files in `scripts/` directory
- Fixed example code in README.md that had invalid Rust with two `main` functions

### Added

- Added crates.io publishing support to CI/CD workflow
- Added `release_mode` input with "instant" and "changelog-pr" options for manual releases
- Added `--tag-prefix` and `--crates-io-url` options to create-github-release.mjs script
- Added comprehensive case study documentation for Issue #11 in docs/case-studies/issue-11/

### Changed

- Changed changelog fragment check from warning to error (exit 1) to enforce changelog requirements
- Updated job conditions with `always() && !cancelled()` to fix workflow_dispatch job skipping issue
- Renamed manual-release job to "Instant Release" for clarity

### Fixed

- Fixed deprecated `::set-output` GitHub Actions command in version-and-commit.mjs
- Fixed workflow_dispatch triggering issues where lint/build/release jobs were incorrectly skipped

### Fixed

- Fixed changelog fragment check to validate that a fragment is **added in the PR diff** rather than just checking if any fragments exist in the directory. This prevents the check from incorrectly passing when there are leftover fragments from previous PRs that haven't been released yet.

### Changed

- Converted shell scripts in `release.yml` to cross-platform `.mjs` scripts for improved portability and performance:
  - `check-changelog-fragment.mjs` - validates changelog fragment is added in PR diff
  - `git-config.mjs` - configures git user for CI/CD
  - `check-release-needed.mjs` - checks if release is needed
  - `publish-crate.mjs` - publishes package to crates.io
  - `create-changelog-fragment.mjs` - creates changelog fragments for manual releases
  - `get-version.mjs` - gets current version from Cargo.toml

### Added

- Added `check-version-modification.mjs` script to detect manual version changes in Cargo.toml
- Added `version-check` job to CI/CD workflow that runs on pull requests
- Added skip logic for automated release branches (changelog-manual-release-*, changeset-release/*, release/*, automated-release/*)

### Changed

- Version modifications in Cargo.toml are now blocked in pull requests to enforce automated release pipeline

### Added

- Added support for `CARGO_REGISTRY_TOKEN` as alternative to `CARGO_TOKEN` for crates.io publishing
- Added case study documentation for Issue #17 (yargs reserved word and dual token support)

### Changed

- Updated workflow to use fallback logic: `${{ secrets.CARGO_REGISTRY_TOKEN || secrets.CARGO_TOKEN }}`
- Improved publish-crate.mjs to check both `CARGO_REGISTRY_TOKEN` and `CARGO_TOKEN` environment variables
- Added warning message when neither token is set

### Added
- New `scripts/rust-paths.mjs` utility for automatic Rust package root detection
- Support for both single-language and multi-language repository structures in all CI/CD scripts
- Configuration options via `--rust-root` CLI argument and `RUST_ROOT` environment variable
- Comprehensive case study documentation in `docs/case-studies/issue-19/`

### Changed
- Updated all release scripts to use the new path detection utility:
  - `scripts/bump-version.mjs`
  - `scripts/check-release-needed.mjs`
  - `scripts/collect-changelog.mjs`
  - `scripts/get-bump-type.mjs`
  - `scripts/get-version.mjs`
  - `scripts/publish-crate.mjs`
  - `scripts/version-and-commit.mjs`

### Changed

- **check-release-needed.mjs**: Now checks crates.io API directly instead of git tags to determine if a version is already released. This prevents false positives where git tags exist but the package was never actually published to crates.io.

### Added

- **CI/CD Troubleshooting Guide**: New documentation at `docs/ci-cd/troubleshooting.md` covering common issues like skipped jobs, false positive version checks, publishing failures, and secret configuration.

- **Enhanced Error Handling in publish-crate.mjs**: Added specific detection and helpful error messages for authentication failures, including guidance on secret configuration and workflow setup.

- **Case Study Documentation**: Added comprehensive case study at `docs/case-studies/issue-21/` analyzing CI/CD failures from browser-commander repository (issues #27, #29, #31, #33) with timeline, root causes, and lessons learned.

### Fixed

- **Prevent False Positive Version Checks**: The release workflow now correctly identifies unpublished versions by checking crates.io instead of relying on git tags, which can exist without the package being published.

### Changed

- Translated all CI/CD scripts from JavaScript (.mjs) to Rust (.rs) using rust-script
- Scripts now use native Rust with rust-script for execution in shell
- Removed Node.js dependency from CI/CD pipeline
- Updated GitHub Actions workflow to use rust-script instead of node
- Updated README and CONTRIBUTING documentation with new script references

### Added

- Cache `restore-keys` for partial cache hits across all workflow jobs
- Explicit `token` parameter in checkout for release jobs
- Code coverage job with `cargo-llvm-cov` and Codecov integration
- Codecov badge in README.md
- Pre-release version support (e.g., `0.1.0-beta.1`) in version parsing
- `--release-label` parameter for multi-language release disambiguation
- `ensure_version_exceeds_published()` logic to prevent publishing duplicate versions
- `get_max_published_version()` to query highest non-yanked version from crates.io
- `max_published_version` output from check-release-needed for downstream use
- Version fallback logic in auto-release Create GitHub Release step

### Changed

- Updated `actions/checkout` from v4 to v6
- Updated `actions/cache` from v4 to v5
- Updated `peter-evans/create-pull-request` from v7 to v8
- Made `publish-crate.rs` fail (exit 1) when version already exists on crates.io
- Improved `create-github-release.rs` to check combined stdout+stderr and detect "Validation Failed"

### Fixed

- Fix publish steps overriding workflow-level CARGO_TOKEN fallback, breaking CARGO_REGISTRY_TOKEN-only configurations (#32)
- Fix non-fast-forward push failures in multi-workflow repos by adding fetch/rebase and push retry logic (#31)
- Add mono-repo path support to check-changelog-fragment.rs, check-version-modification.rs, and create-changelog-fragment.rs
- Add `!cancelled()` guard to test job condition to respect workflow cancellation

### Fixed

- Fixed `version-and-commit.rs` to check crates.io instead of git tags for determining if a version is already released
- This prevents the release pipeline from getting stuck when git tags exist without corresponding crates.io publication

### Added

- Added `--tag-prefix` support to `version-and-commit.rs` for multi-language repository compatibility
- Added crates.io and docs.rs badges to README.md
- Added automatic crates.io and docs.rs badge injection in GitHub release notes
- Added documentation deployment job to CI/CD pipeline (deploys to GitHub Pages after release)
- Added case study documentation for issue #25

### Fixed
- Fixed unsupported look-ahead regex in `create-github-release.rs` that caused a panic when parsing CHANGELOG.md. Replaced with a two-step approach using only features supported by Rust's `regex` crate.

### Changed
- Restructured example application as a simple CLI sum calculator using `lino-arguments`
- Renamed default package to `example-sum-package-name` with Unlicense license
- Reorganized test structure: `tests/unit/sum.rs`, `tests/integration/sum.rs`, `tests/unit/ci-cd/`
- Converted experiment scripts into proper unit tests in `tests/unit/ci-cd/changelog_parsing.rs`
- Added CI/CD skip logic for template default package name `example-sum-package-name`
- Updated README.md badges and documentation

### Fixed

- Change detection script now uses per-commit diff instead of full PR diff, so commits touching only non-code files correctly skip CI jobs even when earlier commits in the same PR changed code files

### Fixed
- Make release scripts resolve the publishable crate manifest when the repository root uses a Cargo workspace manifest.

### Fixed
- Decoupled GitHub Pages documentation deployment from package release publication and fixed release-script warning failures under `RUSTFLAGS=-Dwarnings`.

### Changed
- Added explicit GitHub Actions job timeouts and documented Rust test timeout guidance.

### Fixed
- Added a non-blocking warning threshold to the Rust file-size check so near-limit files are surfaced before concurrent PR merges can exceed the hard limit.

## [0.8.0] - 2026-05-01

### Added
- Changeset-style fragment format with frontmatter for specifying version bump type
- New `get-bump-type.mjs` script to automatically determine version bump from fragments
- Automatic version bumping on merge to main based on changelog fragments
- Detailed documentation for the changelog fragment system in `changelog.d/README.md`

### Changed
- Updated `collect-changelog.mjs` to strip frontmatter when collecting fragments
- Updated `version-and-commit.mjs` to handle frontmatter in fragments
- Enhanced release workflow to automatically determine bump type from changesets

### Changed
- Add `detect-changes` job with cross-platform `detect-code-changes.mjs` script
- Make lint job independent of changelog check (runs based on file changes only)
- Allow docs-only PRs without changelog fragment requirement
- Handle changelog check 'skipped' state in dependent jobs
- Exclude `changelog.d/`, `docs/`, `experiments/`, `examples/` folders and markdown files from code changes detection

### Fixed
- Fixed README.md to correctly reference Node.js scripts (`.mjs`) instead of Python scripts (`.py`)
- Updated project structure in README.md to match actual script files in `scripts/` directory
- Fixed example code in README.md that had invalid Rust with two `main` functions

### Added

- Added crates.io publishing support to CI/CD workflow
- Added `release_mode` input with "instant" and "changelog-pr" options for manual releases
- Added `--tag-prefix` and `--crates-io-url` options to create-github-release.mjs script
- Added comprehensive case study documentation for Issue #11 in docs/case-studies/issue-11/

### Changed

- Changed changelog fragment check from warning to error (exit 1) to enforce changelog requirements
- Updated job conditions with `always() && !cancelled()` to fix workflow_dispatch job skipping issue
- Renamed manual-release job to "Instant Release" for clarity

### Fixed

- Fixed deprecated `::set-output` GitHub Actions command in version-and-commit.mjs
- Fixed workflow_dispatch triggering issues where lint/build/release jobs were incorrectly skipped

### Fixed

- Fixed changelog fragment check to validate that a fragment is **added in the PR diff** rather than just checking if any fragments exist in the directory. This prevents the check from incorrectly passing when there are leftover fragments from previous PRs that haven't been released yet.

### Changed

- Converted shell scripts in `release.yml` to cross-platform `.mjs` scripts for improved portability and performance:
  - `check-changelog-fragment.mjs` - validates changelog fragment is added in PR diff
  - `git-config.mjs` - configures git user for CI/CD
  - `check-release-needed.mjs` - checks if release is needed
  - `publish-crate.mjs` - publishes package to crates.io
  - `create-changelog-fragment.mjs` - creates changelog fragments for manual releases
  - `get-version.mjs` - gets current version from Cargo.toml

### Added

- Added `check-version-modification.mjs` script to detect manual version changes in Cargo.toml
- Added `version-check` job to CI/CD workflow that runs on pull requests
- Added skip logic for automated release branches (changelog-manual-release-*, changeset-release/*, release/*, automated-release/*)

### Changed

- Version modifications in Cargo.toml are now blocked in pull requests to enforce automated release pipeline

### Added

- Added support for `CARGO_REGISTRY_TOKEN` as alternative to `CARGO_TOKEN` for crates.io publishing
- Added case study documentation for Issue #17 (yargs reserved word and dual token support)

### Changed

- Updated workflow to use fallback logic: `${{ secrets.CARGO_REGISTRY_TOKEN || secrets.CARGO_TOKEN }}`
- Improved publish-crate.mjs to check both `CARGO_REGISTRY_TOKEN` and `CARGO_TOKEN` environment variables
- Added warning message when neither token is set

### Added
- New `scripts/rust-paths.mjs` utility for automatic Rust package root detection
- Support for both single-language and multi-language repository structures in all CI/CD scripts
- Configuration options via `--rust-root` CLI argument and `RUST_ROOT` environment variable
- Comprehensive case study documentation in `docs/case-studies/issue-19/`

### Changed
- Updated all release scripts to use the new path detection utility:
  - `scripts/bump-version.mjs`
  - `scripts/check-release-needed.mjs`
  - `scripts/collect-changelog.mjs`
  - `scripts/get-bump-type.mjs`
  - `scripts/get-version.mjs`
  - `scripts/publish-crate.mjs`
  - `scripts/version-and-commit.mjs`

### Changed

- **check-release-needed.mjs**: Now checks crates.io API directly instead of git tags to determine if a version is already released. This prevents false positives where git tags exist but the package was never actually published to crates.io.

### Added

- **CI/CD Troubleshooting Guide**: New documentation at `docs/ci-cd/troubleshooting.md` covering common issues like skipped jobs, false positive version checks, publishing failures, and secret configuration.

- **Enhanced Error Handling in publish-crate.mjs**: Added specific detection and helpful error messages for authentication failures, including guidance on secret configuration and workflow setup.

- **Case Study Documentation**: Added comprehensive case study at `docs/case-studies/issue-21/` analyzing CI/CD failures from browser-commander repository (issues #27, #29, #31, #33) with timeline, root causes, and lessons learned.

### Fixed

- **Prevent False Positive Version Checks**: The release workflow now correctly identifies unpublished versions by checking crates.io instead of relying on git tags, which can exist without the package being published.

### Changed

- Translated all CI/CD scripts from JavaScript (.mjs) to Rust (.rs) using rust-script
- Scripts now use native Rust with rust-script for execution in shell
- Removed Node.js dependency from CI/CD pipeline
- Updated GitHub Actions workflow to use rust-script instead of node
- Updated README and CONTRIBUTING documentation with new script references

### Added

- Cache `restore-keys` for partial cache hits across all workflow jobs
- Explicit `token` parameter in checkout for release jobs
- Code coverage job with `cargo-llvm-cov` and Codecov integration
- Codecov badge in README.md
- Pre-release version support (e.g., `0.1.0-beta.1`) in version parsing
- `--release-label` parameter for multi-language release disambiguation
- `ensure_version_exceeds_published()` logic to prevent publishing duplicate versions
- `get_max_published_version()` to query highest non-yanked version from crates.io
- `max_published_version` output from check-release-needed for downstream use
- Version fallback logic in auto-release Create GitHub Release step

### Changed

- Updated `actions/checkout` from v4 to v6
- Updated `actions/cache` from v4 to v5
- Updated `peter-evans/create-pull-request` from v7 to v8
- Made `publish-crate.rs` fail (exit 1) when version already exists on crates.io
- Improved `create-github-release.rs` to check combined stdout+stderr and detect "Validation Failed"

### Fixed

- Fix publish steps overriding workflow-level CARGO_TOKEN fallback, breaking CARGO_REGISTRY_TOKEN-only configurations (#32)
- Fix non-fast-forward push failures in multi-workflow repos by adding fetch/rebase and push retry logic (#31)
- Add mono-repo path support to check-changelog-fragment.rs, check-version-modification.rs, and create-changelog-fragment.rs
- Add `!cancelled()` guard to test job condition to respect workflow cancellation

### Fixed

- Fixed `version-and-commit.rs` to check crates.io instead of git tags for determining if a version is already released
- This prevents the release pipeline from getting stuck when git tags exist without corresponding crates.io publication

### Added

- Added `--tag-prefix` support to `version-and-commit.rs` for multi-language repository compatibility
- Added crates.io and docs.rs badges to README.md
- Added automatic crates.io and docs.rs badge injection in GitHub release notes
- Added documentation deployment job to CI/CD pipeline (deploys to GitHub Pages after release)
- Added case study documentation for issue #25

### Fixed
- Fixed unsupported look-ahead regex in `create-github-release.rs` that caused a panic when parsing CHANGELOG.md. Replaced with a two-step approach using only features supported by Rust's `regex` crate.

### Changed
- Restructured example application as a simple CLI sum calculator using `lino-arguments`
- Renamed default package to `example-sum-package-name` with Unlicense license
- Reorganized test structure: `tests/unit/sum.rs`, `tests/integration/sum.rs`, `tests/unit/ci-cd/`
- Converted experiment scripts into proper unit tests in `tests/unit/ci-cd/changelog_parsing.rs`
- Added CI/CD skip logic for template default package name `example-sum-package-name`
- Updated README.md badges and documentation

### Fixed

- Change detection script now uses per-commit diff instead of full PR diff, so commits touching only non-code files correctly skip CI jobs even when earlier commits in the same PR changed code files

### Fixed
- Make release scripts resolve the publishable crate manifest when the repository root uses a Cargo workspace manifest.

### Fixed
- Decoupled GitHub Pages documentation deployment from package release publication and fixed release-script warning failures under `RUSTFLAGS=-Dwarnings`.

## [0.7.0] - 2026-04-14

### Added
- Changeset-style fragment format with frontmatter for specifying version bump type
- New `get-bump-type.mjs` script to automatically determine version bump from fragments
- Automatic version bumping on merge to main based on changelog fragments
- Detailed documentation for the changelog fragment system in `changelog.d/README.md`

### Changed
- Updated `collect-changelog.mjs` to strip frontmatter when collecting fragments
- Updated `version-and-commit.mjs` to handle frontmatter in fragments
- Enhanced release workflow to automatically determine bump type from changesets

### Changed
- Add `detect-changes` job with cross-platform `detect-code-changes.mjs` script
- Make lint job independent of changelog check (runs based on file changes only)
- Allow docs-only PRs without changelog fragment requirement
- Handle changelog check 'skipped' state in dependent jobs
- Exclude `changelog.d/`, `docs/`, `experiments/`, `examples/` folders and markdown files from code changes detection

### Fixed
- Fixed README.md to correctly reference Node.js scripts (`.mjs`) instead of Python scripts (`.py`)
- Updated project structure in README.md to match actual script files in `scripts/` directory
- Fixed example code in README.md that had invalid Rust with two `main` functions

### Added

- Added crates.io publishing support to CI/CD workflow
- Added `release_mode` input with "instant" and "changelog-pr" options for manual releases
- Added `--tag-prefix` and `--crates-io-url` options to create-github-release.mjs script
- Added comprehensive case study documentation for Issue #11 in docs/case-studies/issue-11/

### Changed

- Changed changelog fragment check from warning to error (exit 1) to enforce changelog requirements
- Updated job conditions with `always() && !cancelled()` to fix workflow_dispatch job skipping issue
- Renamed manual-release job to "Instant Release" for clarity

### Fixed

- Fixed deprecated `::set-output` GitHub Actions command in version-and-commit.mjs
- Fixed workflow_dispatch triggering issues where lint/build/release jobs were incorrectly skipped

### Fixed

- Fixed changelog fragment check to validate that a fragment is **added in the PR diff** rather than just checking if any fragments exist in the directory. This prevents the check from incorrectly passing when there are leftover fragments from previous PRs that haven't been released yet.

### Changed

- Converted shell scripts in `release.yml` to cross-platform `.mjs` scripts for improved portability and performance:
  - `check-changelog-fragment.mjs` - validates changelog fragment is added in PR diff
  - `git-config.mjs` - configures git user for CI/CD
  - `check-release-needed.mjs` - checks if release is needed
  - `publish-crate.mjs` - publishes package to crates.io
  - `create-changelog-fragment.mjs` - creates changelog fragments for manual releases
  - `get-version.mjs` - gets current version from Cargo.toml

### Added

- Added `check-version-modification.mjs` script to detect manual version changes in Cargo.toml
- Added `version-check` job to CI/CD workflow that runs on pull requests
- Added skip logic for automated release branches (changelog-manual-release-*, changeset-release/*, release/*, automated-release/*)

### Changed

- Version modifications in Cargo.toml are now blocked in pull requests to enforce automated release pipeline

### Added

- Added support for `CARGO_REGISTRY_TOKEN` as alternative to `CARGO_TOKEN` for crates.io publishing
- Added case study documentation for Issue #17 (yargs reserved word and dual token support)

### Changed

- Updated workflow to use fallback logic: `${{ secrets.CARGO_REGISTRY_TOKEN || secrets.CARGO_TOKEN }}`
- Improved publish-crate.mjs to check both `CARGO_REGISTRY_TOKEN` and `CARGO_TOKEN` environment variables
- Added warning message when neither token is set

### Added
- New `scripts/rust-paths.mjs` utility for automatic Rust package root detection
- Support for both single-language and multi-language repository structures in all CI/CD scripts
- Configuration options via `--rust-root` CLI argument and `RUST_ROOT` environment variable
- Comprehensive case study documentation in `docs/case-studies/issue-19/`

### Changed
- Updated all release scripts to use the new path detection utility:
  - `scripts/bump-version.mjs`
  - `scripts/check-release-needed.mjs`
  - `scripts/collect-changelog.mjs`
  - `scripts/get-bump-type.mjs`
  - `scripts/get-version.mjs`
  - `scripts/publish-crate.mjs`
  - `scripts/version-and-commit.mjs`

### Changed

- **check-release-needed.mjs**: Now checks crates.io API directly instead of git tags to determine if a version is already released. This prevents false positives where git tags exist but the package was never actually published to crates.io.

### Added

- **CI/CD Troubleshooting Guide**: New documentation at `docs/ci-cd/troubleshooting.md` covering common issues like skipped jobs, false positive version checks, publishing failures, and secret configuration.

- **Enhanced Error Handling in publish-crate.mjs**: Added specific detection and helpful error messages for authentication failures, including guidance on secret configuration and workflow setup.

- **Case Study Documentation**: Added comprehensive case study at `docs/case-studies/issue-21/` analyzing CI/CD failures from browser-commander repository (issues #27, #29, #31, #33) with timeline, root causes, and lessons learned.

### Fixed

- **Prevent False Positive Version Checks**: The release workflow now correctly identifies unpublished versions by checking crates.io instead of relying on git tags, which can exist without the package being published.

### Changed

- Translated all CI/CD scripts from JavaScript (.mjs) to Rust (.rs) using rust-script
- Scripts now use native Rust with rust-script for execution in shell
- Removed Node.js dependency from CI/CD pipeline
- Updated GitHub Actions workflow to use rust-script instead of node
- Updated README and CONTRIBUTING documentation with new script references

### Added

- Cache `restore-keys` for partial cache hits across all workflow jobs
- Explicit `token` parameter in checkout for release jobs
- Code coverage job with `cargo-llvm-cov` and Codecov integration
- Codecov badge in README.md
- Pre-release version support (e.g., `0.1.0-beta.1`) in version parsing
- `--release-label` parameter for multi-language release disambiguation
- `ensure_version_exceeds_published()` logic to prevent publishing duplicate versions
- `get_max_published_version()` to query highest non-yanked version from crates.io
- `max_published_version` output from check-release-needed for downstream use
- Version fallback logic in auto-release Create GitHub Release step

### Changed

- Updated `actions/checkout` from v4 to v6
- Updated `actions/cache` from v4 to v5
- Updated `peter-evans/create-pull-request` from v7 to v8
- Made `publish-crate.rs` fail (exit 1) when version already exists on crates.io
- Improved `create-github-release.rs` to check combined stdout+stderr and detect "Validation Failed"

### Fixed

- Fix publish steps overriding workflow-level CARGO_TOKEN fallback, breaking CARGO_REGISTRY_TOKEN-only configurations (#32)
- Fix non-fast-forward push failures in multi-workflow repos by adding fetch/rebase and push retry logic (#31)
- Add mono-repo path support to check-changelog-fragment.rs, check-version-modification.rs, and create-changelog-fragment.rs
- Add `!cancelled()` guard to test job condition to respect workflow cancellation

### Fixed

- Fixed `version-and-commit.rs` to check crates.io instead of git tags for determining if a version is already released
- This prevents the release pipeline from getting stuck when git tags exist without corresponding crates.io publication

### Added

- Added `--tag-prefix` support to `version-and-commit.rs` for multi-language repository compatibility
- Added crates.io and docs.rs badges to README.md
- Added automatic crates.io and docs.rs badge injection in GitHub release notes
- Added documentation deployment job to CI/CD pipeline (deploys to GitHub Pages after release)
- Added case study documentation for issue #25

### Fixed
- Fixed unsupported look-ahead regex in `create-github-release.rs` that caused a panic when parsing CHANGELOG.md. Replaced with a two-step approach using only features supported by Rust's `regex` crate.

### Changed
- Restructured example application as a simple CLI sum calculator using `lino-arguments`
- Renamed default package to `example-sum-package-name` with Unlicense license
- Reorganized test structure: `tests/unit/sum.rs`, `tests/integration/sum.rs`, `tests/unit/ci-cd/`
- Converted experiment scripts into proper unit tests in `tests/unit/ci-cd/changelog_parsing.rs`
- Added CI/CD skip logic for template default package name `example-sum-package-name`
- Updated README.md badges and documentation

### Fixed

- Change detection script now uses per-commit diff instead of full PR diff, so commits touching only non-code files correctly skip CI jobs even when earlier commits in the same PR changed code files

## [0.6.0] - 2026-04-13

### Added
- Changeset-style fragment format with frontmatter for specifying version bump type
- New `get-bump-type.mjs` script to automatically determine version bump from fragments
- Automatic version bumping on merge to main based on changelog fragments
- Detailed documentation for the changelog fragment system in `changelog.d/README.md`

### Changed
- Updated `collect-changelog.mjs` to strip frontmatter when collecting fragments
- Updated `version-and-commit.mjs` to handle frontmatter in fragments
- Enhanced release workflow to automatically determine bump type from changesets

### Changed
- Add `detect-changes` job with cross-platform `detect-code-changes.mjs` script
- Make lint job independent of changelog check (runs based on file changes only)
- Allow docs-only PRs without changelog fragment requirement
- Handle changelog check 'skipped' state in dependent jobs
- Exclude `changelog.d/`, `docs/`, `experiments/`, `examples/` folders and markdown files from code changes detection

### Fixed
- Fixed README.md to correctly reference Node.js scripts (`.mjs`) instead of Python scripts (`.py`)
- Updated project structure in README.md to match actual script files in `scripts/` directory
- Fixed example code in README.md that had invalid Rust with two `main` functions

### Added

- Added crates.io publishing support to CI/CD workflow
- Added `release_mode` input with "instant" and "changelog-pr" options for manual releases
- Added `--tag-prefix` and `--crates-io-url` options to create-github-release.mjs script
- Added comprehensive case study documentation for Issue #11 in docs/case-studies/issue-11/

### Changed

- Changed changelog fragment check from warning to error (exit 1) to enforce changelog requirements
- Updated job conditions with `always() && !cancelled()` to fix workflow_dispatch job skipping issue
- Renamed manual-release job to "Instant Release" for clarity

### Fixed

- Fixed deprecated `::set-output` GitHub Actions command in version-and-commit.mjs
- Fixed workflow_dispatch triggering issues where lint/build/release jobs were incorrectly skipped

### Fixed

- Fixed changelog fragment check to validate that a fragment is **added in the PR diff** rather than just checking if any fragments exist in the directory. This prevents the check from incorrectly passing when there are leftover fragments from previous PRs that haven't been released yet.

### Changed

- Converted shell scripts in `release.yml` to cross-platform `.mjs` scripts for improved portability and performance:
  - `check-changelog-fragment.mjs` - validates changelog fragment is added in PR diff
  - `git-config.mjs` - configures git user for CI/CD
  - `check-release-needed.mjs` - checks if release is needed
  - `publish-crate.mjs` - publishes package to crates.io
  - `create-changelog-fragment.mjs` - creates changelog fragments for manual releases
  - `get-version.mjs` - gets current version from Cargo.toml

### Added

- Added `check-version-modification.mjs` script to detect manual version changes in Cargo.toml
- Added `version-check` job to CI/CD workflow that runs on pull requests
- Added skip logic for automated release branches (changelog-manual-release-*, changeset-release/*, release/*, automated-release/*)

### Changed

- Version modifications in Cargo.toml are now blocked in pull requests to enforce automated release pipeline

### Added

- Added support for `CARGO_REGISTRY_TOKEN` as alternative to `CARGO_TOKEN` for crates.io publishing
- Added case study documentation for Issue #17 (yargs reserved word and dual token support)

### Changed

- Updated workflow to use fallback logic: `${{ secrets.CARGO_REGISTRY_TOKEN || secrets.CARGO_TOKEN }}`
- Improved publish-crate.mjs to check both `CARGO_REGISTRY_TOKEN` and `CARGO_TOKEN` environment variables
- Added warning message when neither token is set

### Added
- New `scripts/rust-paths.mjs` utility for automatic Rust package root detection
- Support for both single-language and multi-language repository structures in all CI/CD scripts
- Configuration options via `--rust-root` CLI argument and `RUST_ROOT` environment variable
- Comprehensive case study documentation in `docs/case-studies/issue-19/`

### Changed
- Updated all release scripts to use the new path detection utility:
  - `scripts/bump-version.mjs`
  - `scripts/check-release-needed.mjs`
  - `scripts/collect-changelog.mjs`
  - `scripts/get-bump-type.mjs`
  - `scripts/get-version.mjs`
  - `scripts/publish-crate.mjs`
  - `scripts/version-and-commit.mjs`

### Changed

- **check-release-needed.mjs**: Now checks crates.io API directly instead of git tags to determine if a version is already released. This prevents false positives where git tags exist but the package was never actually published to crates.io.

### Added

- **CI/CD Troubleshooting Guide**: New documentation at `docs/ci-cd/troubleshooting.md` covering common issues like skipped jobs, false positive version checks, publishing failures, and secret configuration.

- **Enhanced Error Handling in publish-crate.mjs**: Added specific detection and helpful error messages for authentication failures, including guidance on secret configuration and workflow setup.

- **Case Study Documentation**: Added comprehensive case study at `docs/case-studies/issue-21/` analyzing CI/CD failures from browser-commander repository (issues #27, #29, #31, #33) with timeline, root causes, and lessons learned.

### Fixed

- **Prevent False Positive Version Checks**: The release workflow now correctly identifies unpublished versions by checking crates.io instead of relying on git tags, which can exist without the package being published.

### Changed

- Translated all CI/CD scripts from JavaScript (.mjs) to Rust (.rs) using rust-script
- Scripts now use native Rust with rust-script for execution in shell
- Removed Node.js dependency from CI/CD pipeline
- Updated GitHub Actions workflow to use rust-script instead of node
- Updated README and CONTRIBUTING documentation with new script references

### Added

- Cache `restore-keys` for partial cache hits across all workflow jobs
- Explicit `token` parameter in checkout for release jobs
- Code coverage job with `cargo-llvm-cov` and Codecov integration
- Codecov badge in README.md
- Pre-release version support (e.g., `0.1.0-beta.1`) in version parsing
- `--release-label` parameter for multi-language release disambiguation
- `ensure_version_exceeds_published()` logic to prevent publishing duplicate versions
- `get_max_published_version()` to query highest non-yanked version from crates.io
- `max_published_version` output from check-release-needed for downstream use
- Version fallback logic in auto-release Create GitHub Release step

### Changed

- Updated `actions/checkout` from v4 to v6
- Updated `actions/cache` from v4 to v5
- Updated `peter-evans/create-pull-request` from v7 to v8
- Made `publish-crate.rs` fail (exit 1) when version already exists on crates.io
- Improved `create-github-release.rs` to check combined stdout+stderr and detect "Validation Failed"

### Fixed

- Fix publish steps overriding workflow-level CARGO_TOKEN fallback, breaking CARGO_REGISTRY_TOKEN-only configurations (#32)
- Fix non-fast-forward push failures in multi-workflow repos by adding fetch/rebase and push retry logic (#31)
- Add mono-repo path support to check-changelog-fragment.rs, check-version-modification.rs, and create-changelog-fragment.rs
- Add `!cancelled()` guard to test job condition to respect workflow cancellation

### Fixed

- Fixed `version-and-commit.rs` to check crates.io instead of git tags for determining if a version is already released
- This prevents the release pipeline from getting stuck when git tags exist without corresponding crates.io publication

### Added

- Added `--tag-prefix` support to `version-and-commit.rs` for multi-language repository compatibility
- Added crates.io and docs.rs badges to README.md
- Added automatic crates.io and docs.rs badge injection in GitHub release notes
- Added documentation deployment job to CI/CD pipeline (deploys to GitHub Pages after release)
- Added case study documentation for issue #25

### Fixed
- Fixed unsupported look-ahead regex in `create-github-release.rs` that caused a panic when parsing CHANGELOG.md. Replaced with a two-step approach using only features supported by Rust's `regex` crate.

### Changed
- Restructured example application as a simple CLI sum calculator using `lino-arguments`
- Renamed default package to `example-sum-package-name` with Unlicense license
- Reorganized test structure: `tests/unit/sum.rs`, `tests/integration/sum.rs`, `tests/unit/ci-cd/`
- Converted experiment scripts into proper unit tests in `tests/unit/ci-cd/changelog_parsing.rs`
- Added CI/CD skip logic for template default package name `example-sum-package-name`
- Updated README.md badges and documentation

## [0.5.0] - 2026-04-13

### Added
- Changeset-style fragment format with frontmatter for specifying version bump type
- New `get-bump-type.mjs` script to automatically determine version bump from fragments
- Automatic version bumping on merge to main based on changelog fragments
- Detailed documentation for the changelog fragment system in `changelog.d/README.md`

### Changed
- Updated `collect-changelog.mjs` to strip frontmatter when collecting fragments
- Updated `version-and-commit.mjs` to handle frontmatter in fragments
- Enhanced release workflow to automatically determine bump type from changesets

### Changed
- Add `detect-changes` job with cross-platform `detect-code-changes.mjs` script
- Make lint job independent of changelog check (runs based on file changes only)
- Allow docs-only PRs without changelog fragment requirement
- Handle changelog check 'skipped' state in dependent jobs
- Exclude `changelog.d/`, `docs/`, `experiments/`, `examples/` folders and markdown files from code changes detection

### Fixed
- Fixed README.md to correctly reference Node.js scripts (`.mjs`) instead of Python scripts (`.py`)
- Updated project structure in README.md to match actual script files in `scripts/` directory
- Fixed example code in README.md that had invalid Rust with two `main` functions

### Added

- Added crates.io publishing support to CI/CD workflow
- Added `release_mode` input with "instant" and "changelog-pr" options for manual releases
- Added `--tag-prefix` and `--crates-io-url` options to create-github-release.mjs script
- Added comprehensive case study documentation for Issue #11 in docs/case-studies/issue-11/

### Changed

- Changed changelog fragment check from warning to error (exit 1) to enforce changelog requirements
- Updated job conditions with `always() && !cancelled()` to fix workflow_dispatch job skipping issue
- Renamed manual-release job to "Instant Release" for clarity

### Fixed

- Fixed deprecated `::set-output` GitHub Actions command in version-and-commit.mjs
- Fixed workflow_dispatch triggering issues where lint/build/release jobs were incorrectly skipped

### Fixed

- Fixed changelog fragment check to validate that a fragment is **added in the PR diff** rather than just checking if any fragments exist in the directory. This prevents the check from incorrectly passing when there are leftover fragments from previous PRs that haven't been released yet.

### Changed

- Converted shell scripts in `release.yml` to cross-platform `.mjs` scripts for improved portability and performance:
  - `check-changelog-fragment.mjs` - validates changelog fragment is added in PR diff
  - `git-config.mjs` - configures git user for CI/CD
  - `check-release-needed.mjs` - checks if release is needed
  - `publish-crate.mjs` - publishes package to crates.io
  - `create-changelog-fragment.mjs` - creates changelog fragments for manual releases
  - `get-version.mjs` - gets current version from Cargo.toml

### Added

- Added `check-version-modification.mjs` script to detect manual version changes in Cargo.toml
- Added `version-check` job to CI/CD workflow that runs on pull requests
- Added skip logic for automated release branches (changelog-manual-release-*, changeset-release/*, release/*, automated-release/*)

### Changed

- Version modifications in Cargo.toml are now blocked in pull requests to enforce automated release pipeline

### Added

- Added support for `CARGO_REGISTRY_TOKEN` as alternative to `CARGO_TOKEN` for crates.io publishing
- Added case study documentation for Issue #17 (yargs reserved word and dual token support)

### Changed

- Updated workflow to use fallback logic: `${{ secrets.CARGO_REGISTRY_TOKEN || secrets.CARGO_TOKEN }}`
- Improved publish-crate.mjs to check both `CARGO_REGISTRY_TOKEN` and `CARGO_TOKEN` environment variables
- Added warning message when neither token is set

### Added
- New `scripts/rust-paths.mjs` utility for automatic Rust package root detection
- Support for both single-language and multi-language repository structures in all CI/CD scripts
- Configuration options via `--rust-root` CLI argument and `RUST_ROOT` environment variable
- Comprehensive case study documentation in `docs/case-studies/issue-19/`

### Changed
- Updated all release scripts to use the new path detection utility:
  - `scripts/bump-version.mjs`
  - `scripts/check-release-needed.mjs`
  - `scripts/collect-changelog.mjs`
  - `scripts/get-bump-type.mjs`
  - `scripts/get-version.mjs`
  - `scripts/publish-crate.mjs`
  - `scripts/version-and-commit.mjs`

### Changed

- **check-release-needed.mjs**: Now checks crates.io API directly instead of git tags to determine if a version is already released. This prevents false positives where git tags exist but the package was never actually published to crates.io.

### Added

- **CI/CD Troubleshooting Guide**: New documentation at `docs/ci-cd/troubleshooting.md` covering common issues like skipped jobs, false positive version checks, publishing failures, and secret configuration.

- **Enhanced Error Handling in publish-crate.mjs**: Added specific detection and helpful error messages for authentication failures, including guidance on secret configuration and workflow setup.

- **Case Study Documentation**: Added comprehensive case study at `docs/case-studies/issue-21/` analyzing CI/CD failures from browser-commander repository (issues #27, #29, #31, #33) with timeline, root causes, and lessons learned.

### Fixed

- **Prevent False Positive Version Checks**: The release workflow now correctly identifies unpublished versions by checking crates.io instead of relying on git tags, which can exist without the package being published.

### Changed

- Translated all CI/CD scripts from JavaScript (.mjs) to Rust (.rs) using rust-script
- Scripts now use native Rust with rust-script for execution in shell
- Removed Node.js dependency from CI/CD pipeline
- Updated GitHub Actions workflow to use rust-script instead of node
- Updated README and CONTRIBUTING documentation with new script references

### Added

- Cache `restore-keys` for partial cache hits across all workflow jobs
- Explicit `token` parameter in checkout for release jobs
- Code coverage job with `cargo-llvm-cov` and Codecov integration
- Codecov badge in README.md
- Pre-release version support (e.g., `0.1.0-beta.1`) in version parsing
- `--release-label` parameter for multi-language release disambiguation
- `ensure_version_exceeds_published()` logic to prevent publishing duplicate versions
- `get_max_published_version()` to query highest non-yanked version from crates.io
- `max_published_version` output from check-release-needed for downstream use
- Version fallback logic in auto-release Create GitHub Release step

### Changed

- Updated `actions/checkout` from v4 to v6
- Updated `actions/cache` from v4 to v5
- Updated `peter-evans/create-pull-request` from v7 to v8
- Made `publish-crate.rs` fail (exit 1) when version already exists on crates.io
- Improved `create-github-release.rs` to check combined stdout+stderr and detect "Validation Failed"

### Fixed

- Fixed `version-and-commit.rs` to check crates.io instead of git tags for determining if a version is already released
- This prevents the release pipeline from getting stuck when git tags exist without corresponding crates.io publication

### Added

- Added `--tag-prefix` support to `version-and-commit.rs` for multi-language repository compatibility
- Added crates.io and docs.rs badges to README.md
- Added automatic crates.io and docs.rs badge injection in GitHub release notes
- Added documentation deployment job to CI/CD pipeline (deploys to GitHub Pages after release)
- Added case study documentation for issue #25

### Fixed
- Fixed unsupported look-ahead regex in `create-github-release.rs` that caused a panic when parsing CHANGELOG.md. Replaced with a two-step approach using only features supported by Rust's `regex` crate.

### Changed
- Restructured example application as a simple CLI sum calculator using `lino-arguments`
- Renamed default package to `example-sum-package-name` with Unlicense license
- Reorganized test structure: `tests/unit/sum.rs`, `tests/integration/sum.rs`, `tests/unit/ci-cd/`
- Converted experiment scripts into proper unit tests in `tests/unit/ci-cd/changelog_parsing.rs`
- Added CI/CD skip logic for template default package name `example-sum-package-name`
- Updated README.md badges and documentation

## [0.4.0] - 2026-04-13

### Added
- Changeset-style fragment format with frontmatter for specifying version bump type
- New `get-bump-type.mjs` script to automatically determine version bump from fragments
- Automatic version bumping on merge to main based on changelog fragments
- Detailed documentation for the changelog fragment system in `changelog.d/README.md`

### Changed
- Updated `collect-changelog.mjs` to strip frontmatter when collecting fragments
- Updated `version-and-commit.mjs` to handle frontmatter in fragments
- Enhanced release workflow to automatically determine bump type from changesets

### Changed
- Add `detect-changes` job with cross-platform `detect-code-changes.mjs` script
- Make lint job independent of changelog check (runs based on file changes only)
- Allow docs-only PRs without changelog fragment requirement
- Handle changelog check 'skipped' state in dependent jobs
- Exclude `changelog.d/`, `docs/`, `experiments/`, `examples/` folders and markdown files from code changes detection

### Fixed
- Fixed README.md to correctly reference Node.js scripts (`.mjs`) instead of Python scripts (`.py`)
- Updated project structure in README.md to match actual script files in `scripts/` directory
- Fixed example code in README.md that had invalid Rust with two `main` functions

### Added

- Added crates.io publishing support to CI/CD workflow
- Added `release_mode` input with "instant" and "changelog-pr" options for manual releases
- Added `--tag-prefix` and `--crates-io-url` options to create-github-release.mjs script
- Added comprehensive case study documentation for Issue #11 in docs/case-studies/issue-11/

### Changed

- Changed changelog fragment check from warning to error (exit 1) to enforce changelog requirements
- Updated job conditions with `always() && !cancelled()` to fix workflow_dispatch job skipping issue
- Renamed manual-release job to "Instant Release" for clarity

### Fixed

- Fixed deprecated `::set-output` GitHub Actions command in version-and-commit.mjs
- Fixed workflow_dispatch triggering issues where lint/build/release jobs were incorrectly skipped

### Fixed

- Fixed changelog fragment check to validate that a fragment is **added in the PR diff** rather than just checking if any fragments exist in the directory. This prevents the check from incorrectly passing when there are leftover fragments from previous PRs that haven't been released yet.

### Changed

- Converted shell scripts in `release.yml` to cross-platform `.mjs` scripts for improved portability and performance:
  - `check-changelog-fragment.mjs` - validates changelog fragment is added in PR diff
  - `git-config.mjs` - configures git user for CI/CD
  - `check-release-needed.mjs` - checks if release is needed
  - `publish-crate.mjs` - publishes package to crates.io
  - `create-changelog-fragment.mjs` - creates changelog fragments for manual releases
  - `get-version.mjs` - gets current version from Cargo.toml

### Added

- Added `check-version-modification.mjs` script to detect manual version changes in Cargo.toml
- Added `version-check` job to CI/CD workflow that runs on pull requests
- Added skip logic for automated release branches (changelog-manual-release-*, changeset-release/*, release/*, automated-release/*)

### Changed

- Version modifications in Cargo.toml are now blocked in pull requests to enforce automated release pipeline

### Added

- Added support for `CARGO_REGISTRY_TOKEN` as alternative to `CARGO_TOKEN` for crates.io publishing
- Added case study documentation for Issue #17 (yargs reserved word and dual token support)

### Changed

- Updated workflow to use fallback logic: `${{ secrets.CARGO_REGISTRY_TOKEN || secrets.CARGO_TOKEN }}`
- Improved publish-crate.mjs to check both `CARGO_REGISTRY_TOKEN` and `CARGO_TOKEN` environment variables
- Added warning message when neither token is set

### Added
- New `scripts/rust-paths.mjs` utility for automatic Rust package root detection
- Support for both single-language and multi-language repository structures in all CI/CD scripts
- Configuration options via `--rust-root` CLI argument and `RUST_ROOT` environment variable
- Comprehensive case study documentation in `docs/case-studies/issue-19/`

### Changed
- Updated all release scripts to use the new path detection utility:
  - `scripts/bump-version.mjs`
  - `scripts/check-release-needed.mjs`
  - `scripts/collect-changelog.mjs`
  - `scripts/get-bump-type.mjs`
  - `scripts/get-version.mjs`
  - `scripts/publish-crate.mjs`
  - `scripts/version-and-commit.mjs`

### Changed

- **check-release-needed.mjs**: Now checks crates.io API directly instead of git tags to determine if a version is already released. This prevents false positives where git tags exist but the package was never actually published to crates.io.

### Added

- **CI/CD Troubleshooting Guide**: New documentation at `docs/ci-cd/troubleshooting.md` covering common issues like skipped jobs, false positive version checks, publishing failures, and secret configuration.

- **Enhanced Error Handling in publish-crate.mjs**: Added specific detection and helpful error messages for authentication failures, including guidance on secret configuration and workflow setup.

- **Case Study Documentation**: Added comprehensive case study at `docs/case-studies/issue-21/` analyzing CI/CD failures from browser-commander repository (issues #27, #29, #31, #33) with timeline, root causes, and lessons learned.

### Fixed

- **Prevent False Positive Version Checks**: The release workflow now correctly identifies unpublished versions by checking crates.io instead of relying on git tags, which can exist without the package being published.

### Changed

- Translated all CI/CD scripts from JavaScript (.mjs) to Rust (.rs) using rust-script
- Scripts now use native Rust with rust-script for execution in shell
- Removed Node.js dependency from CI/CD pipeline
- Updated GitHub Actions workflow to use rust-script instead of node
- Updated README and CONTRIBUTING documentation with new script references

### Added

- Cache `restore-keys` for partial cache hits across all workflow jobs
- Explicit `token` parameter in checkout for release jobs
- Code coverage job with `cargo-llvm-cov` and Codecov integration
- Codecov badge in README.md
- Pre-release version support (e.g., `0.1.0-beta.1`) in version parsing
- `--release-label` parameter for multi-language release disambiguation
- `ensure_version_exceeds_published()` logic to prevent publishing duplicate versions
- `get_max_published_version()` to query highest non-yanked version from crates.io
- `max_published_version` output from check-release-needed for downstream use
- Version fallback logic in auto-release Create GitHub Release step

### Changed

- Updated `actions/checkout` from v4 to v6
- Updated `actions/cache` from v4 to v5
- Updated `peter-evans/create-pull-request` from v7 to v8
- Made `publish-crate.rs` fail (exit 1) when version already exists on crates.io
- Improved `create-github-release.rs` to check combined stdout+stderr and detect "Validation Failed"

### Fixed

- Fixed `version-and-commit.rs` to check crates.io instead of git tags for determining if a version is already released
- This prevents the release pipeline from getting stuck when git tags exist without corresponding crates.io publication

### Added

- Added `--tag-prefix` support to `version-and-commit.rs` for multi-language repository compatibility
- Added crates.io and docs.rs badges to README.md
- Added automatic crates.io and docs.rs badge injection in GitHub release notes
- Added documentation deployment job to CI/CD pipeline (deploys to GitHub Pages after release)
- Added case study documentation for issue #25

## [0.3.0] - 2026-04-13

### Added
- Changeset-style fragment format with frontmatter for specifying version bump type
- New `get-bump-type.mjs` script to automatically determine version bump from fragments
- Automatic version bumping on merge to main based on changelog fragments
- Detailed documentation for the changelog fragment system in `changelog.d/README.md`

### Changed
- Updated `collect-changelog.mjs` to strip frontmatter when collecting fragments
- Updated `version-and-commit.mjs` to handle frontmatter in fragments
- Enhanced release workflow to automatically determine bump type from changesets

### Changed
- Add `detect-changes` job with cross-platform `detect-code-changes.mjs` script
- Make lint job independent of changelog check (runs based on file changes only)
- Allow docs-only PRs without changelog fragment requirement
- Handle changelog check 'skipped' state in dependent jobs
- Exclude `changelog.d/`, `docs/`, `experiments/`, `examples/` folders and markdown files from code changes detection

### Fixed
- Fixed README.md to correctly reference Node.js scripts (`.mjs`) instead of Python scripts (`.py`)
- Updated project structure in README.md to match actual script files in `scripts/` directory
- Fixed example code in README.md that had invalid Rust with two `main` functions

### Added

- Added crates.io publishing support to CI/CD workflow
- Added `release_mode` input with "instant" and "changelog-pr" options for manual releases
- Added `--tag-prefix` and `--crates-io-url` options to create-github-release.mjs script
- Added comprehensive case study documentation for Issue #11 in docs/case-studies/issue-11/

### Changed

- Changed changelog fragment check from warning to error (exit 1) to enforce changelog requirements
- Updated job conditions with `always() && !cancelled()` to fix workflow_dispatch job skipping issue
- Renamed manual-release job to "Instant Release" for clarity

### Fixed

- Fixed deprecated `::set-output` GitHub Actions command in version-and-commit.mjs
- Fixed workflow_dispatch triggering issues where lint/build/release jobs were incorrectly skipped

### Fixed

- Fixed changelog fragment check to validate that a fragment is **added in the PR diff** rather than just checking if any fragments exist in the directory. This prevents the check from incorrectly passing when there are leftover fragments from previous PRs that haven't been released yet.

### Changed

- Converted shell scripts in `release.yml` to cross-platform `.mjs` scripts for improved portability and performance:
  - `check-changelog-fragment.mjs` - validates changelog fragment is added in PR diff
  - `git-config.mjs` - configures git user for CI/CD
  - `check-release-needed.mjs` - checks if release is needed
  - `publish-crate.mjs` - publishes package to crates.io
  - `create-changelog-fragment.mjs` - creates changelog fragments for manual releases
  - `get-version.mjs` - gets current version from Cargo.toml

### Added

- Added `check-version-modification.mjs` script to detect manual version changes in Cargo.toml
- Added `version-check` job to CI/CD workflow that runs on pull requests
- Added skip logic for automated release branches (changelog-manual-release-*, changeset-release/*, release/*, automated-release/*)

### Changed

- Version modifications in Cargo.toml are now blocked in pull requests to enforce automated release pipeline

### Added

- Added support for `CARGO_REGISTRY_TOKEN` as alternative to `CARGO_TOKEN` for crates.io publishing
- Added case study documentation for Issue #17 (yargs reserved word and dual token support)

### Changed

- Updated workflow to use fallback logic: `${{ secrets.CARGO_REGISTRY_TOKEN || secrets.CARGO_TOKEN }}`
- Improved publish-crate.mjs to check both `CARGO_REGISTRY_TOKEN` and `CARGO_TOKEN` environment variables
- Added warning message when neither token is set

### Added
- New `scripts/rust-paths.mjs` utility for automatic Rust package root detection
- Support for both single-language and multi-language repository structures in all CI/CD scripts
- Configuration options via `--rust-root` CLI argument and `RUST_ROOT` environment variable
- Comprehensive case study documentation in `docs/case-studies/issue-19/`

### Changed
- Updated all release scripts to use the new path detection utility:
  - `scripts/bump-version.mjs`
  - `scripts/check-release-needed.mjs`
  - `scripts/collect-changelog.mjs`
  - `scripts/get-bump-type.mjs`
  - `scripts/get-version.mjs`
  - `scripts/publish-crate.mjs`
  - `scripts/version-and-commit.mjs`

### Changed

- **check-release-needed.mjs**: Now checks crates.io API directly instead of git tags to determine if a version is already released. This prevents false positives where git tags exist but the package was never actually published to crates.io.

### Added

- **CI/CD Troubleshooting Guide**: New documentation at `docs/ci-cd/troubleshooting.md` covering common issues like skipped jobs, false positive version checks, publishing failures, and secret configuration.

- **Enhanced Error Handling in publish-crate.mjs**: Added specific detection and helpful error messages for authentication failures, including guidance on secret configuration and workflow setup.

- **Case Study Documentation**: Added comprehensive case study at `docs/case-studies/issue-21/` analyzing CI/CD failures from browser-commander repository (issues #27, #29, #31, #33) with timeline, root causes, and lessons learned.

### Fixed

- **Prevent False Positive Version Checks**: The release workflow now correctly identifies unpublished versions by checking crates.io instead of relying on git tags, which can exist without the package being published.

### Changed

- Translated all CI/CD scripts from JavaScript (.mjs) to Rust (.rs) using rust-script
- Scripts now use native Rust with rust-script for execution in shell
- Removed Node.js dependency from CI/CD pipeline
- Updated GitHub Actions workflow to use rust-script instead of node
- Updated README and CONTRIBUTING documentation with new script references

### Fixed

- Fixed `version-and-commit.rs` to check crates.io instead of git tags for determining if a version is already released
- This prevents the release pipeline from getting stuck when git tags exist without corresponding crates.io publication

### Added

- Added `--tag-prefix` support to `version-and-commit.rs` for multi-language repository compatibility
- Added crates.io and docs.rs badges to README.md
- Added automatic crates.io and docs.rs badge injection in GitHub release notes
- Added documentation deployment job to CI/CD pipeline (deploys to GitHub Pages after release)
- Added case study documentation for issue #25

## [0.2.0] - 2026-03-11

### Added
- Changeset-style fragment format with frontmatter for specifying version bump type
- New `get-bump-type.mjs` script to automatically determine version bump from fragments
- Automatic version bumping on merge to main based on changelog fragments
- Detailed documentation for the changelog fragment system in `changelog.d/README.md`

### Changed
- Updated `collect-changelog.mjs` to strip frontmatter when collecting fragments
- Updated `version-and-commit.mjs` to handle frontmatter in fragments
- Enhanced release workflow to automatically determine bump type from changesets

### Changed
- Add `detect-changes` job with cross-platform `detect-code-changes.mjs` script
- Make lint job independent of changelog check (runs based on file changes only)
- Allow docs-only PRs without changelog fragment requirement
- Handle changelog check 'skipped' state in dependent jobs
- Exclude `changelog.d/`, `docs/`, `experiments/`, `examples/` folders and markdown files from code changes detection

### Fixed
- Fixed README.md to correctly reference Node.js scripts (`.mjs`) instead of Python scripts (`.py`)
- Updated project structure in README.md to match actual script files in `scripts/` directory
- Fixed example code in README.md that had invalid Rust with two `main` functions

### Added

- Added crates.io publishing support to CI/CD workflow
- Added `release_mode` input with "instant" and "changelog-pr" options for manual releases
- Added `--tag-prefix` and `--crates-io-url` options to create-github-release.mjs script
- Added comprehensive case study documentation for Issue #11 in docs/case-studies/issue-11/

### Changed

- Changed changelog fragment check from warning to error (exit 1) to enforce changelog requirements
- Updated job conditions with `always() && !cancelled()` to fix workflow_dispatch job skipping issue
- Renamed manual-release job to "Instant Release" for clarity

### Fixed

- Fixed deprecated `::set-output` GitHub Actions command in version-and-commit.mjs
- Fixed workflow_dispatch triggering issues where lint/build/release jobs were incorrectly skipped

### Fixed

- Fixed changelog fragment check to validate that a fragment is **added in the PR diff** rather than just checking if any fragments exist in the directory. This prevents the check from incorrectly passing when there are leftover fragments from previous PRs that haven't been released yet.

### Changed

- Converted shell scripts in `release.yml` to cross-platform `.mjs` scripts for improved portability and performance:
  - `check-changelog-fragment.mjs` - validates changelog fragment is added in PR diff
  - `git-config.mjs` - configures git user for CI/CD
  - `check-release-needed.mjs` - checks if release is needed
  - `publish-crate.mjs` - publishes package to crates.io
  - `create-changelog-fragment.mjs` - creates changelog fragments for manual releases
  - `get-version.mjs` - gets current version from Cargo.toml

### Added

- Added `check-version-modification.mjs` script to detect manual version changes in Cargo.toml
- Added `version-check` job to CI/CD workflow that runs on pull requests
- Added skip logic for automated release branches (changelog-manual-release-*, changeset-release/*, release/*, automated-release/*)

### Changed

- Version modifications in Cargo.toml are now blocked in pull requests to enforce automated release pipeline

### Added

- Added support for `CARGO_REGISTRY_TOKEN` as alternative to `CARGO_TOKEN` for crates.io publishing
- Added case study documentation for Issue #17 (yargs reserved word and dual token support)

### Changed

- Updated workflow to use fallback logic: `${{ secrets.CARGO_REGISTRY_TOKEN || secrets.CARGO_TOKEN }}`
- Improved publish-crate.mjs to check both `CARGO_REGISTRY_TOKEN` and `CARGO_TOKEN` environment variables
- Added warning message when neither token is set

### Added
- New `scripts/rust-paths.mjs` utility for automatic Rust package root detection
- Support for both single-language and multi-language repository structures in all CI/CD scripts
- Configuration options via `--rust-root` CLI argument and `RUST_ROOT` environment variable
- Comprehensive case study documentation in `docs/case-studies/issue-19/`

### Changed
- Updated all release scripts to use the new path detection utility:
  - `scripts/bump-version.mjs`
  - `scripts/check-release-needed.mjs`
  - `scripts/collect-changelog.mjs`
  - `scripts/get-bump-type.mjs`
  - `scripts/get-version.mjs`
  - `scripts/publish-crate.mjs`
  - `scripts/version-and-commit.mjs`

### Changed

- **check-release-needed.mjs**: Now checks crates.io API directly instead of git tags to determine if a version is already released. This prevents false positives where git tags exist but the package was never actually published to crates.io.

### Added

- **CI/CD Troubleshooting Guide**: New documentation at `docs/ci-cd/troubleshooting.md` covering common issues like skipped jobs, false positive version checks, publishing failures, and secret configuration.

- **Enhanced Error Handling in publish-crate.mjs**: Added specific detection and helpful error messages for authentication failures, including guidance on secret configuration and workflow setup.

- **Case Study Documentation**: Added comprehensive case study at `docs/case-studies/issue-21/` analyzing CI/CD failures from browser-commander repository (issues #27, #29, #31, #33) with timeline, root causes, and lessons learned.

### Fixed

- **Prevent False Positive Version Checks**: The release workflow now correctly identifies unpublished versions by checking crates.io instead of relying on git tags, which can exist without the package being published.

### Changed

- Translated all CI/CD scripts from JavaScript (.mjs) to Rust (.rs) using rust-script
- Scripts now use native Rust with rust-script for execution in shell
- Removed Node.js dependency from CI/CD pipeline
- Updated GitHub Actions workflow to use rust-script instead of node
- Updated README and CONTRIBUTING documentation with new script references

## [0.1.0] - 2025-01-XX

### Added

- Initial project structure
- Basic example functions (add, multiply, delay)
- Comprehensive test suite
- Code quality tools (rustfmt, clippy)
- Pre-commit hooks configuration
- GitHub Actions CI/CD pipeline
- Changelog fragment system (similar to Changesets/Scriv)
- Release automation (GitHub releases)
- Template structure for AI-driven Rust development