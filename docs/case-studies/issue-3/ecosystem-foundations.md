# link-foundation / linksplatform Ecosystem Foundations

> Compiled 2026-06-05 from the link-foundation and linksplatform repositories
> (READMEs, source, and test corpora). Supports the
> [issue #1](https://github.com/link-foundation/meta-language/issues/1) vision and
> the [issue #3](https://github.com/link-foundation/meta-language/issues/3)
> mandate to reuse the ecosystem's own components and tests. These are the
> "internal competitors" — projects whose behavior this crate must subsume into
> one links network.

The crate already names these as `PARITY_TARGETS` (`links-notation`, `link-cli`,
`lino-objects-codec`, `relative-meta-logic`, `formal-ai`, `meta-expression`).
This document records each one's real API, test shape, and the exact behavior the
crate's fixtures must reproduce — plus verified corrections to claims made in
issue #1.

## Meta-theory: the exact definitions (terminology guardrail)

The foundational definitions come from the meta-theory article
(`archive/0.0.2/article.md`). They are the source of truth for the repo's
**"no graph" rule** — the structure is a *links network*, not a graph:

- A **link** is an n-tuple of references to links (line 89).
- A **network** (of links) is a set of links (line 95).
- A **point** is a link that references only itself in every position — the
  self-referential base case (line 101).

Every fixture and every adapter must phrase structure in these terms. External
projects' terminology ("node", "edge", "tree", "AST") is translated to links at
the adapter boundary; it must not leak into the network's own vocabulary.

## links-notation (LiNo) — the textual surface

- **Repo:** `linksplatform/links-notation` (Rust crate `links-notation`).
- **Core type (verified):** a recursive `LiNo<T>` enum with variants for a
  reference (leaf) and a link (sequence of `LiNo<T>`), with optional id. Parses
  the LiNo surface: doublets `(lovesMama: loves mama)`, triplets
  `papa has car`, N-tuples, indented/multiline blocks, and self-referential
  points.
- **Tests:** roughly **~138 tests per language binding** (not "90+" as issue #1
  estimates; the count is higher). The suite asserts **cross-language identity** —
  the same source parses to the same structure across the language bindings.
- **Crate role:** this is the reference grammar for the crate's own LiNo fixtures
  (`links-notation` parity target: doublets, triplets, N-tuples, indentation,
  self-reference). The crate must parse the same surface and reconstruct it
  byte-for-byte.

## link-cli — the single match→substitute operation

- **Repo:** `linksplatform/link-cli` (a.k.a. `l" "links` CLI).
- **Verified operation set** (one operation: match a pattern, substitute a
  pattern):
  - **create:** `() ((1 1))` — empty match, produce a link.
  - **update:** `((1: 1 1)) ((1: 1 2))` — match link id 1, rewrite its target.
  - **delete:** `((1 1)) ()` — match a link, produce nothing.
  - **swap:** `((($index: $source $target)) (($index: $target $source)))` — match
    with variables, substitute with positions swapped.
- **Crate role:** this is the engine model for `SubstitutionRule` /
  `apply_substitution()`. The crate's `link-cli` parity fixtures must cover
  exactly create, update, delete, and swap, with variable binding (`$index`,
  `$source`, `$target`).

## lino-objects-codec — object ⇆ links round-trip

- **Repo:** `linksplatform/lino-objects-codec`.
- **Verified property:** `decode(encode(x)) == x` — objects encode to links and
  decode back identically, including **shared references** (the same sub-object
  referenced twice stays shared) and **circular references** (self-referential
  object structures survive the round-trip).
- **Crate role:** the `lino-objects-codec` parity target's `ObjectRoundTrip`
  capability. Fixtures must include a shared-reference object and a circular
  object, asserting identity after encode→decode.

## relative-meta-logic — dependent types & many-valued evaluation

- **Repo:** `linksplatform/relative-meta-logic`.
- **Verified scope:** dependent types, many-valued and probabilistic evaluation,
  and paradox handling (the **liar paradox** is an explicit corpus case). Shares
  the `.lino` corpus convention with the rest of the ecosystem.
- **Crate role:** the `relative-meta-logic` parity target's `SemanticEvaluation`
  capability and the `TruthValue` type. Fixtures must include a dependent-type
  case and a paradox case yielding a non-binary `TruthValue`.

## formal-ai — formalization corpus

- **Repo:** `link-foundation/formal-ai`.
- **Verified corpus location:** `data/seed/*.lino` and `data/benchmarks/*.lino`.
- **⚠️ Correction to issue #1:** issue #1 references a **"706-case corpus."** That
  exact number could **not be verified** in the repository; the actual corpus is
  the `data/seed/` + `data/benchmarks/` `.lino` files. The case-study and any
  fixture comments should cite the directory, not the unverified "706" figure.
- **Crate role:** the `formal-ai` parity target's `FormalizationRoundTrip`
  capability — formalize natural statements to links and reconstruct the concept.

## meta-expression — formalize / semantic-link / naturalize

- **Repo:** `link-foundation/meta-expression`.
- **Verified facts:**
  - Semantic lexicon: `semantic-lexicon.json` holds **351 concepts** (issue #1
    says "328"; the verified count is **351** — another figure to correct).
  - Confirmed languages: English, Hindi, Russian, Mandarin Chinese (`en`, `hi`,
    `ru`, `zh`).
  - Worked example: **"Hawaii is a state." → "Гавайи это штат."** with Wikidata
    anchors **Q782** (Hawaii) and **Q35657** (U.S. state) — i.e. concepts carry
    cross-language identity via Wikidata QIDs, and naturalization regenerates the
    sentence in another language from the same concept links.
- **Crate role:** the `meta-expression` parity target's `SelfDescription`,
  `CrossLanguageReconstruction`, and span/naturalization capabilities. The crate's
  `verify --language en --text "Hawaii is a state."` path and the
  English/Spanish concept-reconstruction fixtures derive from this model.

## doublets — associative storage substrate

- **Repo:** `linksplatform/doublets-rs` (crate **`doublets`** 0.4.0, Unlicense).
- **Verified API:** the `Doublets` trait — `create`, `update`, `delete`,
  `each`/query iteration, `count` — over united or split, in-memory or file-backed
  link stores.
- **Crate role:** the ecosystem-native persistence option if the network outgrows
  in-memory storage (see the Rust-libraries survey, section E).

## Verified corrections to issue #1 (carry these into the case study)

| Claim in issue #1 | Verified reality | Source |
|---|---|---|
| formal-ai "706-case corpus" | Unverifiable count; corpus is `data/seed/*.lino` + `data/benchmarks/*.lino` | `link-foundation/formal-ai` tree |
| meta-expression "328 concepts" | **351 concepts** in `semantic-lexicon.json` | `link-foundation/meta-expression` |
| links-notation "90+ tests/language" | **~138 tests per language binding** | `linksplatform/links-notation` tests |
| storage crate "doublets-rs" | crate name is **`doublets`**; `doublets-rs` is the **repo**, not a crate | crates.io / `linksplatform/doublets-rs` |

These are minor, good-faith estimation drifts in the founding issue, not errors of
substance — but the case study should cite verified figures so downstream
implementation issues reference reality.

## How the ecosystem maps onto the crate's parity capabilities

| Ecosystem project | Crate capability it grounds | Crate fixture/API surface |
|---|---|---|
| meta-theory | (terminology) links/network/point definitions | all fixtures' vocabulary |
| links-notation | `LosslessParsing`, `SelfDescription` | LiNo doublet/triplet/N-tuple/self-ref fixtures |
| link-cli | `TransformBySubstitution`, `QueryMatching` | `SubstitutionRule`, `apply_substitution()`, `LinkQuery` |
| lino-objects-codec | `ObjectRoundTrip` | shared + circular object fixtures |
| relative-meta-logic | `SemanticEvaluation` | dependent-type + paradox fixtures, `TruthValue` |
| formal-ai | `FormalizationRoundTrip` | formalization source fixture |
| meta-expression | `CrossLanguageReconstruction`, `SelfDescription` | en/es concept-reconstruction, naturalization span |

The crate is, by design, the **union** of these projects expressed as one lossless
links network. Reusing their corpora (all in the link-foundation/linksplatform
orgs, all permissively licensed) is both allowed and the explicit intent of
issue #3.
