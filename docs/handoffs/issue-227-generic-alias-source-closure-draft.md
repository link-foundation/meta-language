# Generic alias source closure draft

This handoff records an unintegrated source draft, not an architecture
specification, observation record or completion claim. `docs/vision.md` remains
the architecture contract. No requirement ledger was read: its online source
audit prerequisite cannot run in this source-only assignment. No tests, source
generation, validators, represented programs, native compilation, network
operations, commits or pushes ran.

## Authored sources

`js/src/translation/rust-generic-type-alias-rules.js` contains three reusable
decision policies using the existing dense primitive-column representation:

| Operation | Actual input premises | Draft output |
| --- | --- | --- |
| `readRustGenericTypeAliasParameterPlan` | Aligned actual token kinds, values and raw spellings; index of the declaration's `<` | Explicit unconstrained type parameter names, closing token coordinate and unconsumed `=` punctuation; otherwise a refusal |
| `readRustGenericTypeAliasSubstitutionPlan` | Distinct actual parameter binding identities, one checked complete argument-origin premise per parameter, actual resolved binding identity for every template occurrence | Argument index for a parameter occurrence; `-1` for a definition-bound occurrence; otherwise a refusal |
| `readRustGenericTypeAliasExpansionError` | Actual declaration owner, active declaration-owner stack, expansion count | Existing recursion/depth/expansion refusal, including recursion that changes its arguments |

The declaration policy admits a trailing comma, retains raw keyword spellings
such as `r#gen`, and rejects bare edition-2024 reserved keywords, duplicate
parameters, empty lists, bounds/defaults and lifetime/const declarations. Its
proposed parameter bound is 64. The closing `>=` is returned as one consumed
angle plus an unconsumed `=`; adapters must partition the original token and
preserve source spans with the existing type-closing rule. It never edits tokens.

Substitution is a selection plan over binding identities. It does not replace
text or construct a type. An adapter must obtain identities from actual binding
resolution, retain definition-bound owners, copy each selected argument's
complete checked origin, and charge materialization before copying. A supplied
Boolean or matching name alone cannot establish these premises. The proposed
occurrence bound reuses the existing 262144-node materialization scale but does
not replace actual clone accounting. No persistent instantiation-cache identity
or cross-language concept identity is introduced.

`js/tests/fixtures/rust-generic-type-alias-draft.mjs` contains only literal future
policy inputs/expected decisions and original Rust source programs. Its sources
cover independent instantiations, reordered parameters, caller namespace
shadowing, nested closing punctuation, invalid unused definitions, changing-
argument recursion, bounds and absent explicit arguments. These are deferred
controls; no fixture is imported by a runner or represented as executed evidence.
The literal version boundary records the existing proposed Rust fixture profile
(`>=1.90.0 <1.100.0`, edition 2024, 64-bit pointer width), not a live version audit.

## Inspected production constraints

`readRustTypeAliasDeclarationPlan` in
`js/src/translation/rust-type-alias-rules.js` requires `=` immediately after an
actual alias identifier and otherwise returns the complete-parameter refusal.
Its consumers use five fields per declaration row. Altering that layout in
place would affect both frontends and existing independent plan controls.

`initializeRustTypeAliases` and `expandAlias` in
`js/src/translation/rust-type-aliases.js` retain definition source coordinates
and module paths. Expansion restores parser cursor/module/lifetime/inherent
state; the cache and recursion stack currently use the declaration key alone.
`readRustTypeAliasValue` receives path segments after `RustParser.typeValue`
reads the path, before generic arguments are consumed. Simply relaxing discovery
would therefore leave argument punctuation unconsumed and reuse an unrelated
instantiation from a declaration-only cache.

`finishRustTypeAliases` eagerly checks every alias and verifies actual named
datatype owners before publication. A generic parameter cannot become a nominal
datatype by falling through that path. Skipping generic declarations during this
pass would lose the existing unused-invalid-alias protection.

`qualifyType`, `copyOrigin` and source-owner reconstruction preserve type,
wrapper, tuple-field and callable origins. A parameter substitution must carry
all of them, including primitive owner positions, library arguments, Copy facts,
text/vector models and source lifetimes. A same-spelled caller datatype must
never be rebound as an alias's definition owner or concrete argument owner.

Native transport in `rust/src/translation/rust/type_aliases.rs` has the matching
five-field declaration table, declaration-only cache/stack, eager finishing and
source-origin traversal. `rust/src/translation/rust/items/types.rs` has the
matching type-parser entry point and source-token partition transport. These
files were read but not edited. No handwritten native replacement semantics
were authored.

## Remaining integration closure

1. Add a separate shared declaration extension directory or explicitly version
   the existing five-field plan, preserving the old row contract and all current
   macro/function-body opacity. Resolve every discovered parameter to an actual
   binding identity scoped to its complete alias declaration. Do not treat a
   parameter name as a datatype owner or discard actual raw source spelling.
2. Represent a checked symbolic alias template as inspectable shared Links data,
   retaining every definition occurrence and its actual binding identity. Check
   unused templates against the complete namespace before publication. Admitted
   symbolic shapes must prove their constraints; for example `Vec<T>` cannot
   assume the ordinary scalar element restrictions hold for arbitrary `T`.
   The first safe profile can retain refusal for such shapes while admitting
   checked identity/permutation shapes.
3. At `readRustTypeAliasValue`, consume explicit concrete argument syntax with
   the existing source parser and actual closing-token rules, retain complete
   origins, and call the shared arity/substitution decisions. Bind parameter
   identities to those complete origins without reparsing caller spellings in
   the alias module. Restore all parser context on failure and success.
4. Materialize the shared substitution plan over actual typed/source-origin
   Links. Reuse `readRustTypeAliasCloneError` before allocation. Cache only
   fully checked instantiations keyed by declaration identity and complete
   argument-origin identity; a type kind, width or textual spelling is
   insufficient. Keep active declaration identity separately for changing-
   argument recursion. Until a safe full key exists, omit instantiation caching
   rather than accepting a declaration-only hit.
5. Compose concrete generic aliases into the existing nominal, primitive,
   String/vector and function-value member rules only after complete actual
   owner proof. Do not broaden their current owner predicates to accept
   uninstantiated templates or inferred arguments. Generic nominal datatypes,
   trait bounds/defaults, lifetimes/const arguments, local/associated aliases,
   qualified paths and general inference remain independent obligations.
6. Generate the entire new normative JavaScript dependency closure into Rust
   and typed Links through the existing source-library workflow. The existing
   `js/scripts/draft-rust-type-alias-rules.mjs` currently selects the old source
   file; the new source needs an explicit dependency/product authoring path.
   Implement both runtime adapters as transport over those generated decisions.
   If translation rejects an authored operation, improve the translator.
7. Extend literal controls with exact parameter/occurrence and expansion-bound
   endpoints, malformed token columns, both punctuation forms, source offsets
   before/after reference splitting, cache alternation between same-width
   distinct primitive owners, independent definition/argument name shadowing,
   missing/extra arguments, concrete fresh owned values and unused invalid
   templates. Preserve the existing generic refusal input and predecessor bytes;
   only replace its assertion once the same original input has a complete
   admitted interpretation and stronger independent controls.
8. After full source drafting and authorization of generation/verification,
   reconcile source/naming/feature inventories and sources before ledger use.
   Future checks must exercise actual shipped rule products with independent
   lexers, actual JavaScript and native frontends, original Rust programs,
   signature publication, both codecs, four emissions and source-free reload.
   Record observations only after execution at the exact recorded commit.

All current frontend generic-alias refusals remain active. This batch provides
source decisions and deferred inputs for later integration; it does not resolve
full generic substitution or establish JavaScript/Rust parity.
