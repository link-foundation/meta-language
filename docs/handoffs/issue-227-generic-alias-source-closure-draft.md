# Generic alias frontend source draft

This handoff records authored sources, not an architecture specification,
observation record or completion claim. `docs/vision.md` remains the architecture
contract. No ledger was read because its online audit prerequisite cannot run
in this source-only assignment. No tests, generation, validators, represented
programs, native compilation, network operations, commits or pushes ran.

## Authored integration

`js/src/translation/rust-generic-type-alias-rules.js` now supplies shared source
decisions consumed by the ordinary JavaScript alias frontend:

| Operation | Input | Decision |
| --- | --- | --- |
| `readRustGenericTypeAliasDeclarationPlan` | Actual aligned module token kinds, values and raw spellings | Additive declaration directory with complete RHS source coordinates and parameter names |
| `readRustGenericTypeAliasParameterPlan` | Actual declaration parameter-list tokens | Explicit unconstrained type parameters, closing-token coordinate and remaining punctuation |
| `readRustGenericTypeAliasBindingPlan` | Declaration-scoped parameter identities, actual symbolic argument count, bound template occurrences | Whole-template substitution indices, without claiming concrete origin proof |
| `readRustGenericTypeAliasSubstitutionPlan` | The same bound occurrences and complete checked concrete argument-origin premises | Concrete-origin substitution indices |
| `readRustGenericTypeAliasExpansionError` | Actual active declaration owners and expansion count | Existing recursion/depth/expansion decisions, including argument-changing recursion |
| Template depth, unused-parameter and concrete-leaf policies | Actual template facts | Bounded construction, unused-parameter refusal and direct scalar/Unit leaf admission |

The original nongeneric five-field planning API remains unchanged. The new
directory retains the original module/body/macro opacity and adds variable rows:
qualified name, keyword index, RHS start/end, after-item index, parameter count,
and parameter names. Its bounds are the existing 10000 alias declarations,
existing 128 expansion depth and 4096 expansions, proposed 64 explicit
parameters, and the existing 262144 materialization scale. Trailing parameter
commas and raw spellings such as `r#gen` are retained. The subsequent additive
parameter directory also retains trailing type-default spans, including nested
compound delimiters. Bounds, lifetime/const parameters and where clauses still
refuse. A compound `>=` retains the exact remaining `=` rather than consuming
source syntax.

`js/src/translation/rust-type-aliases.js` actually consumes this directory.
Discovery stores declaration-scoped binding identities with original source
positions. `readRustTypeAliasValue` now routes generic rows through actual
argument parsing and concrete instantiation. `finishRustTypeAliases` clears
both ordinary and generic template caches against the complete namespace and
checks every generic template before publication. Ordinary nongeneric expansion
continues through its existing interpretation. Saved cursors now restore by
actual token object identity, so earlier declaration token splitting cannot
silently shift a caller's resume index.

`js/src/translation/rust-generic-type-aliases.js` transports actual tokens and
complete source origins around the shared policies. It supports direct type
parameters, scalar/Unit, safe concrete fn-pointer and actual named datatype
leaves, grouping, tuple templates and explicit
applications of other supported generic aliases. For example, `Swap<T,U>=(U,T)`
and `Pair<T>=Swap<T,i64>` compose actual declaration-scoped bindings. It prepares
a target declaration before descending into symbolic arguments, retaining the
recursion refusal for `Recursive<T>=Recursive<Box<T>>`. Unused parameters and
invalid unused definitions refuse before public interfaces can be exposed.

Canonical standard `Option<T>` and `Result<T,E>` now have symbolic template
nodes in both adapters. Actual namespace resolution precedes the shared
owner/constructor/arity selection; local aliases, datatypes and std modules
cannot impersonate the library. Every symbolic argument is checked, but an
unknown parameter is never declared Copy. Concrete materialization delegates
complete origins to the existing standard-sum instantiator and enforces its
actual Copy or subsequently drafted owned admission and shared active nesting
depth. Copy evidence remains limited to actual all-Copy children. Thus
`Maybe<T>=Option<T>` and `Result<T>=std::result::Result<T,Error>` reuse the actual
sum datatype/constructor representation. Nominal leaves preserve actual source
datatype identities; their current real nominal-Copy proof is consulted only
when an instantiation needs it. An unused nominal sum template can be well
formed without claiming that every future instantiation belongs to this profile.

Canonical standard `Vec<T>` now has a symbolic vector template node in both
adapters. Source/import resolution uses the existing vector-owner policy and
checks actual catalog and type-alias roots for shadows. The shared explicit
arity rule admits one symbolic element without asserting Copy. At concrete
materialization, the shared element rule reuses the existing scalar/Unit/owned
String domain and checks its complete actual primitive/library owner. It builds
the existing owned array type with its actual element owner. Existing typed
origin construction derives scalar element Copy separately; owned String stays
non-Copy. No annotation gains fresh-constructor or fresh-result evidence.

Ordinary alias-typed scalar vector parameters, fresh scalar results, forwarding
and concrete specializations reuse the existing ownership/fresh-tail checks.
For example `Values<T>=std::vec::Vec<T>` supports existing vec/typed constructor
bodies through `Values<i64>` signatures, while `Integers=Values<i64>` reuses the
preceding `Integers::new()` member admission. Tuple/vector and sum/vector
templates compose symbolically; actual instantiation still enforces every
existing product/element/direct-Copy restriction. Vec allocator forms, nested or
nominal element domains, borrowed elements, local vector moves, String-vector
results and other generic alias-associated operations remain obligations.

Explicit alias-qualified construction now has a consumed source draft in both
frontends: `Values::<u8>::new()`, `Maybe::<u8>::Some(7)`, and
`Maybe::<u8>::None` first instantiate the actual generic declaration. Shared
constructor selection checks the complete resulting owner and actual stored
sum declaration before the transport reads its registered constructor path.
Vector construction reuses the existing typed fresh empty-array node. Sum
arguments use the existing actual-constructor contextual argument and ownership
adapters; nullary variants remain values without parentheses. Wrong owners,
members, argument counts and unproved source origins refuse. No type-name
spelling supplies an element, variant or source owner. Alias-qualified constructor
function values, inferred alias arguments and other associated operations retain
their independent obligations.

Static source review also corrected generic parameter raw-token handling.
Both actual lexers normalize identifier values to NFC while retaining original
raw spelling. The parameter policy now reads the actual raw-prefix distinction
without requiring normalized values to equal original Unicode source bytes.
Identifier validity still comes from the actual lexer; this policy adds no
competing Unicode normalizer. Review of native origin construction found both
new private header/sum fields initialized in the two complete constructors;
remaining source-origin literals use those constructors through struct updates.
Pending rule ABI projections follow the existing source-generated alias/pointer
policy interfaces, not handwritten replacement modules. No compiler or validator
was invoked by this review.

Every concrete application parses its explicit arguments in the caller context,
uses the shared binding plan, and copies complete argument origins. Primitive
owners, wrappers, callable fields, vector/text models, Copy facts, source
lifetimes and definition provenance remain on those origins. Tuple construction
uses existing common tuple types and source product admission. No generic
instantiation uses the declaration-only concrete-origin cache; alternating
concrete arguments cannot share a stale origin. Allocation accounting precedes
copies and bounds private construction-tree traversal. No source text is
substituted or reparsed under a caller-invented owner.

`rust/src/translation/rust/type_aliases.rs` and its new sibling
`generic_type_aliases.rs` now draft the matching native discovery, eager checking,
symbolic composition, explicit argument parsing and complete origin transport.
Their decisions call the pending source-generated `rust_generic_type_alias_rules`
module. Private immutable `Rc` template sharing prevents composition from
implicitly cloning argument trees before bounded traversal. Native cursor
restoration retains actual original source coordinates across token partitioning.
Logical template/origin accounting follows the JavaScript adapter. Both ordinary
alias copy paths now traverse complete nested standard-sum argument origins,
preserving their actual child types and owners before copying.

`js/scripts/draft-rust-generic-type-alias-rules.mjs` authors automatic native and
typed Links production for the complete new normative JavaScript policy closure.
It was authored only, never invoked. No generated Rust policy was handwritten.

The private template tree is construction state, not a serializable trust flag
or an executable opaque source payload. Only existing checked concrete types and
origins leave this adapter for the semantic graph. Other parameter-dependent wrapper,
library and callable template semantics retain refusal. Identity arguments can
retain already admitted complete origins; concrete tuple instantiations remain
subject to the existing product restriction.

## Deferred controls and unchanged inputs

`js/tests/fixtures/rust-generic-type-alias-draft.mjs` contains literal future
policy inputs and original Rust programs. `js/tests/translation-rust-generic-type-aliases.test.js`
authors actual policy consumers, ordinary checked source parsing, public
signature discovery, both codecs, four target emissions and future emitted
JavaScript result checks. Inputs cover independent instantiations, permutation
and composition, caller namespace shadowing, nested compound closings, Unit,
well-formed unused definitions and unsupported/invalid definitions. These files
were only authored and reread, never imported or executed.
`rust/tests/unit/translation_rust_generic_type_aliases.rs` drafts the actual
existing original-product pipeline with the new shared rule program, plus
independent native binding/arity/recursion and complete scalar-owner controls.
Its pending generated module and package asset remain explicit dependencies.

The original `type Identity<T>=T;fn main(){}` input and preceding refusal
assertion are preserved together in the new literal controls. Its proposed new
assertion checks the complete valid declaration and absence of runtime exports.
The fixture AUTHOR SOURCE `js/scripts/draft-rust-type-alias-fixtures.mjs` now
moves that unchanged original from current refusals into `genericOriginalControls`,
retaining its exact preceding kind/reason and an explicit correction rationale.
It adds concrete generic identity/permutation/composition programs and a public
concrete-signature publication. The where-clause refusal retains its original
input and now names the actual new discovery refusal. Parameter-bound,
unused-parameter and argument-changing recursion refusals add coverage. Its
assertions and version-qualified boundary describe the integrated JavaScript
profile and deferred native products. The new test source consumes the authored
correction directory without skipping missing controls.

The generator was not run and generated LiNo/native controls remain unchanged.
Full predecessor-byte capture and independent original/native evidence remain
required before any recorded correction evidence. This static source change
preserves the original input and oracle rather than deleting their coverage.

The fixture version boundary records the existing proposed Rust source profile
(`>=1.90.0 <1.100.0`, edition 2024, 64-bit pointer width), not a live audit.
Additional full endpoint, malformed-column, source-coordinate, owner, native
and original-program controls remain required. This draft is not evidence.

## Remaining closure

Both frontend adapters are authored; generated policy products remain deferred.
Generate the complete normative generic policy
dependency closure into Rust and typed Links with the existing source-library
workflow, improving the translator for unsupported source constructs. The
existing `draft-rust-type-alias-rules.mjs` selects the preceding policy source;
the new sibling author selects the generic closure explicitly. Parent module,
test and package/catalog registration must retain the new native adapter,
generated module and shared program asset together. Do not handwrite replacement
native semantics or broaden owner predicates to accept uninstantiated templates.

The additive native signature batch now mirrors the parent JavaScript sum
signature integration. `source_signatures.rs` retains the standalone validator
and adds `validate_source_signature_with_data`, whose borrowed actual datatype
callback supplies constructor layouts and existing retained field facets. The
pending generated `rust_standard_sum_signature_rules` selector alone maps
canonical standard-library arguments to exact actual payload constructors.
Complete constructor inventories, retained field identities, direct payload
positions and recursively identical source owners are checked under one finite
visit/depth budget. Same-width primitive spellings cannot replace retained
owners. The standalone wrapper still refuses sums without actual data context.

`source_data_field_signatures.rs` forwards actual registry/type callbacks.
`check/items.rs` installs every resolved datatype layout and normalizes retained
field positions in their original declaration scope before any nested facet
validation, then validates function signatures against the complete registry.
`typed_program_links/checked.rs` uses the actual saved declaration registry for
both field and function signatures before runtime-profile early returns, and
refuses duplicate datatype registry identities. No new metadata schema or trust
flag is introduced. Focused deferred native source controls retain no-context,
actual-layout, omitted-facet, reordered-constructor and same-erased-owner cases.

`rust/src/translation/rust/standard_sum_methods.rs` drafts actual native method
transport for the sibling JavaScript method policy. The postfix hook runs before
argument punctuation is consumed. It obtains the actual registered receiver
origin, captures the receiver and each eager default/callable factory once, and
constructs the existing typed let/match/constructor nodes using source-generated
branch decisions. Callback parameters/results retain complete actual source
identities. An unknown mapper result receives parameter-only context for a real
lambda; named callable expressions retain their actual result. `map` delegates
result construction to the existing instantiator, while `and_then` requires the
same actual sum family and unchanged complete Result error identity. Callback
invocation remains inside the selected constructor branch. Module registration
is recorded by the parent batch; shared policy generation and execution remain
deferred.

Before any ledger use, run the required online source audit, reread changed
sources, reconcile source/naming/feature inventories, preserve exact prior
fixtures and append obligations. Reconcile all preceding generic refusal
assertions with the retained original source and stronger independent controls.
After the complete draft, required CI must execute actual original Rust,
JavaScript/native frontends, shipped rule products, complete source owners,
publications, codecs, four emissions and source-free reload. Record observations
only afterward at the exact executed commit.

The subsequent resource draft extracts shared semantic cost roles into
`js/src/translation/rust-standard-sum-source-metadata-rules.js`, with borrowed
host traversals in `rust-standard-sum-source-metadata.js` and native
`rust/standard_sum_metadata.rs`. The pending generated module is
`rust_standard_sum_source_metadata_rules`; source generation remains deferred.
Both walks count one node for each record, ordered array, string, boolean,
number or null, and count UTF16 units for canonical record keys and string
values. Numeric array indexes do not contribute keys. The origin projection
retains type, path, wrappers, both complete source-owner trees, fields,
callable parameters/result, float/text/array models, source lifetimes,
namespace spelling and the sum's actual owner/argument origins. Missing
optional records normalize to null; missing lifetime/namespace columns
normalize to empty arrays. Every portable Type form has an explicit shared
record role. Effective identity projection additionally follows the actual
catalog sum argument directory and qualifies nominal type paths identically
to identity construction. Neither walk includes source coordinates, cached
identity encodings, generated constructor paths, ordinals, private phase flags
or incidental binding bookkeeping. Repeated references count per occurrence;
structural/registry cycles and projection depth beyond 256 refuse early.

JS initialization and concrete instantiation reserve sixteen projected copies
before origin cloning or JSON encoding; instantiation reserves both raw and
effective identity projections. Native `SourceOrigins::new` reserves each
distinct incoming catalog sum before its first origin clone, retaining the
cumulative budget/error latch for parser initialization and instantiation.
Initialization's later owned row copies use that reservation. Borrowed native
catalog lookup avoids cloning a constructor/function declaration while proving
the projection bound. Identity equality remains the existing complete
structured identity; no hash or generated-name equivalence substitutes for it.
The shared limits remain 1,048,576 nodes and 8,388,608 UTF16 units per parser.
This is a common semantic metadata domain, not a measured host allocation
byte count. Independent literal unit/Unicode/expanded-owner costs, reservation
edges, depth edges and JavaScript sharing/cycle controls are authored in
`translation-rust-standard-sum-source-metadata.test.js`, its draft fixture,
and the native helper's deferred control module. None were executed.

The next namespace source draft adds `rust-source-namespace-rules.js`. Actual
module/datatype/external-crate catalog entries inhabit the type namespace;
function/constant/constructor entries inhabit the value namespace. Separate
indexes retain overlapping entries across parser passes and source order.
The historical primary catalog remains available to mixed callers, while
actual origin/value/call/pattern consumers and type-owner consumers select
their real namespace. A filtered type lookup stops at a nearer wrong-kind
type item, and a qualified lookup commits to its nearest actual type/module
prefix rather than falling through to an outer child. The shared policy also
retains external-crate conflicts with actual type items while allowing value
homographs. Native indexes and lexical cursors transport the shared decisions;
the borrowed sum identity lookup uses the same type index and prefix rule.
Record constructor renaming and nominal metadata refresh update the indexes.
Parent-owned parser pass/constant-reader and sum owner/constructor seams use
the added registration and role-aware lookup methods. A module and function
with the same name retain independent entries; data/function core declaration
name collisions still require separate core representation and remain a gap.

Deferred namespace controls preserve literal originals for function/local
`Option`, function `std`/`core`, both module/function declaration orders and
imported type aliases with value homographs. Refusals preserve actual user
`std`/`core` modules, forward type shadows and bare `Some` function/local
shadows. Source assertions are explicitly bounded to Rust >=1.90.0 <1.100.0.
`translation-rust-source-namespaces.test.js` drafts actual frontend/checker,
source-free typed text/binary reload and four-emitter comparison controls;
the native type-owner module drafts independent lookup/index controls.
These controls have not run, and no parity or admission result is claimed.

Trailing type defaults now have paired source consumers in the generic alias
adapters. The additive shared parameter directory retains actual token spans,
including the interior offsets of compound `>>=` delimiters; the preceding
explicit-parameter planning API and no-default declaration row shapes remain
unchanged. Every default is parsed as a complete symbolic template in the
alias's definition scope before caching the RHS. Defaults may reference only
earlier actual parameter binding identities. Own/later references, alias
dependency cycles, required parameters after defaults, unresolved default
owners and unsupported type forms refuse even when an argument overrides the
default or the declaration is unused. The existing unused-parameter check on
the RHS remains unchanged. A qualified nominal path tail with the same spelling
as a parameter remains its actual definition owner rather than becoming a
parameter occurrence.

Both concrete and symbolic alias applications fill omitted trailing arguments
from the checked default templates, substituting the earlier complete actual
origins/templates under existing recursion/depth/materialization charges.
Bare aliases with complete defaults, partial arguments and explicit empty
argument lists compose with the admitted tuple, scalar, nominal, canonical
Option/Result and Vec forms. Concrete library admission still checks actual
Copy/element/source-owner facts; symbolic defaults create no Copy evidence.
`Values::<>::new()` and bare all-defaulted `Values::new()`, `Maybe::Some`/`None`
and Result constructor paths reach the existing checked constructor transport.
The existing primary path hook supplies the actual alias prefix and member
token; no root parser rewrite is required. Prefix resolution proves an actual
generic alias with complete authored defaults before instantiation, then the
ordinary complete-owner constructor policy selects its real registered path.
Required arguments are never inferred, local value homographs do not replace
the type prefix, and ordinary module/value paths retain their existing reader.

The expression hook also lazily retains a declaration's complete all-default
origin in the existing alias-access cache, then forwards non-Vec/non-sum members
to the preceding primitive, owned String or nominal member reader. Explicit
specializations never enter this declaration-default cache. Default preparation
disables argument-token consumption so a following comparison `<` cannot be
mistaken for an argument list. Existing copy/materialization accounting applies
both when populating and when reading this scoped origin. Unused declarations
remain symbolic: the accessor never eagerly materializes every default during
declaration validation. Independent source controls retain associated scalar
constants/default/conversion/callable values, alternating explicit widths,
comparison punctuation, String construction/forwarding and nominal variants.
The subsequent source pass adds a bounded actual path-head preparation hook
before the primary's early primitive readers, and calls the same accessor
from the existing primitive-constant transport after it collects its actual
segments. This common transport also reads direct and range constant patterns.
The peek walk follows the existing sixteen-segment primitive-reader window;
it consumes no occurrence tokens, interprets no member spelling and changes
no caller punctuation. Only an actual generic alias prefix with complete
authored defaults can populate the scoped cache. Definition template parsing
still uses the preceding actual token/type parser and restores its caller.
Cached preparation avoids repeat copies. Pattern-only nominal/sum aliases
therefore also reach the existing actual variant-path resolver. New literal
controls cover patterns without an earlier alias expression, both range ends,
actual nominal/sum variants and same-width pointer/fixed owner refusal.
These are source consumers and unexecuted assertions, not observed behavior.
Parent-owned direct primitive-function-value entries invoke the same bounded
head preparation for callbacks that bypass the primary reader. Literal
controls preserve `Option<u8>.map(Word::from)` with defaulted `Word=u16`, a
direct `Item::count_ones` function pointer, and refusals for same-width
`usize`/`u64` callback parameter substitution. The preceding `Maybe<String>`
default refusal now resides in explicit previous-boundary source history,
including the exact input, fixture row, accepted error kinds and version range.
That same input is retained once in the new owned-profile positive originals.
This extends the source interpreter profile; it changes no claim about Rust
validity and records no observed result. Old Copy-only false controls remain.

An additional nominal-owner consumer now prepares the complete actual
`impl Alias` type path before the existing inherent implementation resolver.
`prepareRustTypeAliasDefaultOwner` and native
`prepare_default_type_alias_owner` share the same checked all-default accessor;
they do not infer required parameters or reinterpret the implementation body.
The shared `readRustGenericTypeAliasDefaultOwnerPhase` defers this preparation
only during collecting inherent discovery, allowing a later nominal datatype
to enter the actual catalog. Complete `finishRustTypeAliases` checks every
default afterward, and subsequent parser passes require the real nominal owner
and existing implementation-location, receiver, Copy and borrow proofs. No
parse errors are swallowed and no generic implementation/trait/where syntax
is erased. Literal controls retain a forward nominal default, `Self` record
construction, module scope, a value homograph, invalid unused defaults and
non-nominal/generic/predicate implementation refusals. Source execution and
original-Rust admission remain unobserved.

Generic standard-sum constructor transport now delegates ownership capture to
the same source helper as direct constructors, after real field/call checking
and before return. Native argument facts borrow the completed App's actual
argument slice rather than cloning the expression tree; no later origin mark
overwrites the helper's ownership/freshness facts. The generic shared policy
retains its preceding Copy-only operations and adds separate owned-sum leaf
and constructor operations. These require the exact `owned` profile from the
independently resolved stored datatype origin, while retaining canonical owner,
arity, wrappers and proof checks. No false Copy premise is passed into the old
gate. New direct policy controls preserve the old non-Copy refusal and reject
unknown/copy/borrowed/custom-Drop profile substitutions in the owned gate.
The ownership helper/actual stored-origin validation is the parent's
coordinated source dependency; its matching source API is authored. This draft
records no executed owned admission.

Factory review also exposed private-header consumers that previously required
concrete proofs before the final admission pass. Separate shared header-only
substitution, standard-sum-origin and constructor plans now require actual
header discovery plus the independently resolved stored provisional datatype,
canonical complete source owner, type arguments, arity and direct wrappers.
They transport the existing syntax origin with `proven`/Copy still false.
Concrete/owned gates retain their preceding proof requirements. Actual sum
leaves reached through aliases receive the same bounded header transport;
argument-changing recursion, missing owners, reference payloads and final
profile admission still refuse through existing finalization/later passes.
The common ownership capture helper marks a provisional node and skips runtime
consumption/freshness transitions in header mode. Exact originals cover
`Maybe::<u8>::Some`, `Identity<Option<u8>>`, concrete sum leaves in tuple
templates, owned generic constructors and actual moved String payloads;
independent policy controls reject missing header flags and mismatched stored
provisional facts. This stage distinction is source policy, not admission or
execution evidence.

Concrete callable defaults now have their own shared leaf operation instead
of changing the preceding scalar/nominal leaf domain. Both adapters retain the
actual parsed function type, complete function-owner argument tree, parameter
and result origins, exact widths/source owners and actual Copy/proven facts.
The operation delegates its source-domain check to the existing fn-pointer
policy, checks complete owner/parameter/result shape, and admits no wrapper or
closure capture witness. A separate private-header callable leaf accepts only
the actual parser's `sourceFunctionPointerHeader` flag during header discovery,
using the existing header fn-pointer rule while preserving false proofs.
This also transports concrete callable arguments whose standard-sum results
await final admission; later pointer-domain checks remain mandatory.

Parameter-dependent safe function templates now have private parameter/result
subtrees in both adapters. Actual `fn` source spelling selects that shape;
`r#fn` remains a binding. Parameter names are syntax hints rather than type
uses. Binding discovery/substitution traverses every actual parameter and
result subtree, and omitted results retain the actual implicit Unit type.
Symbolic preparation checks types, bindings, recursion and the shared pointer
parameter limit without assigning any unknown parameter Copy. Materialization
substitutes complete actual origins, invokes the preceding concrete pointer
domain/header checkers, retains exact function-owner/parameter/result facts
and fn source spans, and sets proven/Copy only from actual admission.

The exact old `type Callback<T>=fn(T)->u8;fn main(){}` refusal and unused
`fn(String)` default input are retained as symbolic declaration positives with
their preceding fixture rows/error-kind assertions in previous-boundary
history. The preceding Copy-only checker and its non-Copy rejection controls
remain unchanged; actual owned value domains use the separate gate described
below. Declaration wellformedness is not value-domain admission.
Named function references also use the existing private header separation so
actual provisional nominal/sum results can reach full later admission; their
proven/Copy flags remain false while deferred. Literal controls retain
Mapper aliases, nested Option results, composed aliases/defaults, parameter
name opacity, raw identifiers, fully concrete named/noncapturing
callback defaults, implicit Unit return, private Option results, exact pointer
parameter/result distinctions, captured closures, layers/lifetimes, owned
parameter profile limits and unsafe qualifiers. The preceding scalar leaf's
false function-kind assertion remains in the independent controls. No control
or original Rust program has executed.

The additive owned function-pointer source profile uses actual ownership roles
from the admitted source-origin catalog, independently of the pointer's own
Copy evidence. `readRustFunctionPointerOwnedDomainError` accepts safe ordinary
pointer domains consisting only of actual direct `copy`/`owned` roles, with at
least one owned role, at Rust >=1.90.0 <1.100.0. The combined domain/header
operations retain the old Copy checker first and preserve private provisional
headers until actual later admission. The two old Copy-only operations and
their negative assertions are unchanged. Generic function materialization,
concrete callable leaves and actual declaration/type pointer readers consume
the new operations; unknown owners, borrowed layers, captures, unsupported
ABI/safety syntax and uncovered owned-profile versions retain rejection.
Exact prior used String-domain refusals remain in previous-boundary history
and the same original bytes now appear as source-only positive controls.

`rust-source-type-origin-rules.js` defines the runtime-field clearing plan;
`rust-source-type-origins.js` and native `rust/source_type_origins.rs` transport
that plan over fields, callable children and standard-sum arguments. Their
in-place sanitizer is shared with the existing standard-sum factory. Callable
readers export `cloneRustCallableTypeOrigin` and native
`function_pointers::clone_callable_type_origin`; null JavaScript input returns
null. Stored parameter/result clones remove binding/place identities, loans,
temporary extensions, fresh array/result proofs and sum pattern/borrowed-field
markers while preserving actual types, owners, widths, lifetime/ABI facts,
Copy/proven state, provisional/header flags and capture witnesses. Capture
witnesses must survive until actual callable value admission. No result clone
supplies runtime freshness or ownership transfer; callback consumers must
establish those facts at the real call.

Named and primitive-associated function values now carry a structural root
function owner built from their actual complete parameter/result owners.
Concrete pointer readers also attach an actual Unit owner to an omitted
result before storing its callable child, retaining that distinction through
explicit generic callable arguments with owned parameter domains.
Original Option<Mapper<u8,u16>> controls retain named functions and defaulted
primitive From values, source-free graph round trips and all four emitters.
The subsequent callable-field emission source described below consumes these
controls; they remain obligations without execution evidence.
Independent sanitizer/domain controls are authored in
`translation-rust-source-type-origins.test.js`, native source-type-origin unit
source and the existing generic-alias controls. No source generation, test,
compiler invocation or execution occurred.

The callable-data dependency now has actual paired Rust/Lean emitter consumers.
`callable-data-derivation-rules.js` owns target derivation lists, constructive
node admission, finite traversal limits and observation refusals. Host helpers
`callable-data.js` and native `callable_data.rs` traverse actual child types and
the actual declaration registry. The conservative first profile requires an
acyclic graph and constructive defaults for every referenced datatype and
every callable result. Function parameter values need no default in isolation,
but every parameter datatype is independently emitted with its preceding
Inhabited derivation, so this conservative profile requires its actual default
too. Recursive callable graphs and
empty/defaultless result profiles explicitly refuse instead of introducing
Lean axioms or guessing positivity. Traversal is bounded to depth 256 and
65,536 visits.

Callable data derives only `Clone` in Rust and only `Inhabited` in Lean, using
the existing function-field representations. The old noncallable float/output
and other trait strings are preserved by independent policy controls. The
checker rejects comparison propositions, deep equality, expression comparison,
display/conversion and proof quantification over callable aggregates, including
callable children through wrappers/nominal fields. No function address,
equality, debug or proof representation is fabricated. Safe construction,
storage, pattern extraction and invocation continue through existing nodes.
Independent literal original obligations in `callable-data-originals.mjs`,
`translation-callable-data.test.js` and native `translation_callable_data.rs`
retain Option/Result and acyclic nominal/nested nominal function fields,
source-free text/binary codecs, all four target emissions, exact stdout and
unsupported observation controls. Actual cyclic/constructive-boundary policy
controls remain separate from original compiler acceptance. No generation,
compiler, test, validation or represented program ran.

New owned-sum refusal originals now carry explicit original-compiler
expectations independently of frontend refusal. The first five move/borrow
inputs expect compiler rejection; the exact guarded-field input expects Rust
acceptance. The coordinated guarded-arm extension retains its preceding refusal
row/coordinate/oracle in explicit boundary history and the identical input as a
positive original, with the compiler acceptance contract still attached.
All source/name text remains intact and classifications select both 1.90.0 and
1.99.0, edition 2024. The literal metadata remains an expectation, never a
recorded compiler observation. Classification declaration controls cover the
union of positives/refusals so a later deliberate guard extension retains the
same exact original and compiler oracle.

`produce-rust-standard-sum-integration.mjs` now retains those declarations and
reports classified originals separately from historical unclassified source
obligations, using the small host support module
`rust-original-compiler-classifications.js`. Existing unclassified rows and
frontend assertions remain unchanged; explicit row diagnostic-kind controls
are honored, with the preceding default gate retained for other inputs. A
classified compiler-accepted refusal must have an unsupported-profile frontend
diagnostic, rather than be presented as invalid Rust.

The native integration source compiles every declared classified refusal at
each exact selected compiler endpoint with `--emit=metadata` and JSON
diagnostics, retaining the original bytes and checking accept/reject separately
from JS/native diagnostic parity. It links no refused original and never runs
one. A rejected outcome requires an actual primary diagnostic in that source
file, so setup/operational failure cannot masquerade as the requested source
rejection. Compiler identity, declaration schema, complete endpoint inventory
and declared cell count are mandatory. Historical unclassified compiler
outcomes remain explicitly reported obligations and supply no passing compiler
claim. All compiler calls exist only as future CI runner source; none ran here.
Positive originals with a retained compiler expectation validate its full schema,
version inventory and acceptance outcome before entering the preceding positive
original compiler/runtime matrix, so the moved guard retains both endpoint
obligations without appearing among active refusals.

Independent literal defaults originals/refusals and token-directory controls
are authored in `rust-generic-type-alias-defaults-draft.mjs` and
`translation-rust-generic-type-alias-defaults.test.js`, including complete
pointer-integer owners, dependent/nested defaults, forward alias defaults,
qualified nominal defaults, overrides and defaulted vector freshness.
Native generic alias unit source adds matching shared-policy and actual
frontend controls. The fixture author appends default positives/refusals after
all preceding publication cases, preserving their original module coordinates;
generated LiNo has not changed. Current boundary prose now removes type
defaults from the unresolved list while retaining trait/lifetime/const bounds.
All source assertions remain bounded to Rust >=1.90.0 <1.100.0. No controls,
generation, compilation or validation have run in this batch.

Generic nominal datatypes, other parameter-dependent library/wrapper/callable
templates, trait bounds, lifetime/const arguments, local/associated
aliases, qualified projections, general inference and full namespace/privacy
semantics remain obligations. This source batch establishes no execution,
JavaScript/Rust parity, semantic equivalence or overall requirement completion.
