# Exclusive source loans and caller writeback draft

This source batch is unexecuted. Tests, represented imports, generation,
validation and compilation did not run. No Rust runtime was handwritten. The
source register and requirement ledger remain unread and unchanged. Existing
immutable and owned parser profiles retain their refusals.

## Kernel and controls

`rust-exclusive-source-loan-rules.js` defines the explicit
`Rust exclusive source loans 1` profile for Rust >=1.90.0 <1.100.0. Its rules
cover place-prefix overlap, exclusive/shared conflicts, closed storage roles,
finite resources and packet indices. `rust-exclusive-source-loans.js` implements
actual lexical scope, root availability, loan acquisition, nested reborrow,
parent suspension/release, access, ordered call activation and completed
writeback state. Original inputs are preserved on refusal; operations return
new inspectable state on success.

The context is a projection of the actual source inventories, not an admission
certificate. It has `$: rustExclusiveSourceContext`, profile, major/minor,
ordered scopes `{identity,parent}`, actual binders
`{identity,scope,name,mutable,mode,position}`, complete records
`{owner,constructor,fields:[{name,position}]}`, actual destructor owner names,
and callables `{identity,path,parameters:[{mode,position}],result}`. Modes are
`owned/shared/exclusive` for binders and `value/shared/exclusive` for formals.
Names must already be alpha-renamed. Positions retain actual `type`, `owner`
and `wrappers`; nominal owners use existing `kind: declaration` and fully
resolved declaration paths, with a leading `crate` allowed in the surface type
path. Exclusive formal positions describe the referent; mode retains the
original exclusive source role independently of erased target types.

All context position graphs are bounded and cycle-checked before recursive
owner comparison. Each position has at most 8192 traversed metadata nodes,
depth 128 and 1048576 text units. Source state bounds include 4096 binders and
loans, lexical depth 256 and 100000 successful operations. Storage proof walks
the actual field inventory and rejects recursive, missing, reference-bearing,
callable, Box or custom-destructor referents. Standard Vec elements must also
be closed owned storage. This first profile is conservative; it does not
reinterpret any earlier unsupported source program as invalid Rust.

The APIs are:

- `readRustExclusiveSourceLoanContextError(context)`
- `readRustExclusiveSourcePositionBudgetError(position)`
- `readRustExclusiveSourceLoanStateError(context,state)`
- `beginRustExclusiveSourceLoans(context)`
- `enterRustExclusiveSourceLoanScope(context,state,scope)`
- `leaveRustExclusiveSourceLoanScope(context,state,scope)`
- `borrowRustExclusiveSourcePlace(context,state,binder,fields,kind,parent=0)`
- `releaseRustExclusiveSourceLoan(context,state,loan)`
- `accessRustExclusiveSourcePlace(context,state,binder,fields,operation,authority=0)`
- `beginRustExclusiveSourceCall(context,state,callee,[{parameter,loan}])`
- `finishRustExclusiveSourceCall(context,state,callIdentity)`
- `createRustExclusiveSourceReturnPacket(context,callee,result,[{parameter,value}])`
- `lowerRustExclusiveSourceCallWriteback(context,state,packetCall,roots,names,continuation)`
- `lowerRustExclusiveSourceVectorPush(context,state,binder,authority,vector,item,itemPosition)`
- `executeRustExclusiveSourceCollectionOperation(context,state,binder,authority,collection,method,args,call)`

State transitions are `rustExclusiveSourceContinue {state}` or
`rustExclusiveSourceInvalid {state,reason}`. Lowering returns
`rustExclusiveSourceLowered {expression,...}` or
`rustExclusiveSourceLoweringInvalid {reason}`. Read/write/move operations check
the actual active loan and selected field path; only whole owned roots may
move. Exclusive parent use is suspended until its child is released. Scope exit
cannot discard surviving loans. Disjoint fields can have separate exclusive
loans; overlapping shared/exclusive argument acquisition refuses before Call.
The current implementation deliberately requires ordered nonescaping reborrows;
it does not implement general nonlexical lifetime inference or branch joins.

## Actual AST packet adapter

The output is **surface AST with `k` discriminators**, suitable for the existing
ordinary `checkProgram` pipeline after the source adapter has consumed its
private operations. It is not a new typed IR or a native mutation engine.
Packets use result slot zero and exclusive referent slots one onward, in formal
order. Caller lowering evaluates its actual adapted function Call once, then
reconstructs each changed nominal field against the latest caller root before
running its continuation. Two disjoint fields of the same caller root therefore
preserve both writes. Calls and root values must already be held lexical names.
The supplied fresh-name allocation is checked for duplicates and actual source
binder collisions. Vec push uses ordinary array accumulation with a held vector
and held actual element; it returns the updated carrier for the caller's state
slot rather than invoking host Vec methods.

`rust-exclusive-source-call-adapter.js` supplies:

`lowerRustExclusiveSourceCallable(parser,context,callee,declaration,modulePath,parameterSlots)`

Here `parameterSlots` is the actual ordered
`[{parameter,binder,lexical}]` source-header projection. Every supplied slot must
already belong to the current parser callable and actual declaration map, with
the exact lexical identity, root binding identity, proven referent origin and
complete source owner. Context binders additionally retain their actual
`lexical` identity. The adapter refuses discovery mode and does not fabricate
slots. It invokes the root-exported
`lowerRustOwnedRecordEffectBody(parser,node,path,declarations,initial,expected,finished)`
over the actual original declaration body. The unchanged existing lowerer owns
expression order, ordinary state reconstruction and completion branches. Its
finished continuation packs every exclusive formal's actual surviving slot.
The adapter returns the transformed declaration and original interface data.

`lowerRustExclusiveSourceInvocation(context,state,callee,evaluatedArguments,borrowedArguments,roots,names,continuation)`

This entry activates the actual ordered argument loans, constructs that selected
Call, performs caller writeback and returns the finished static source state.
Loans remain active afterward until their actual source consumer releases them.

The callable adapter explicitly refuses an existing `sourceSignature` facet;
immutable signature 3 cannot be relabeled with packet types. Main parser
admission still needs a separate mutable source-signature schema, its JS/native
checkers/codecs, authentic exclusive parameter-slot registration and source-free
body/Call rechecking. The entry is therefore an explicit prechecked-body source
adapter, not public end-to-end `&mut` parsing. Existing guards at `rust.js:662`,
`rust.js:1531` and `rust-inherent.js:75` were not changed. Invoke the callable
adapter while its actual parser callable/slots are still active, before
`finishRustOwnedRecordEffects` consumes the whole program. Its generated packet
declaration and caller expression then enter ordinary inference/checking.

## Collection interoperability and remaining work

Standard collection source state composes through an optional explicit registry:
`context.collectionTypes` contains
`{typeIdentity,carrierType,origin,keyDomain,valueDomain,pointerWidth}` entries,
with the actual active `context.pointerWidth`. A collection position retains
`{type:carrierType,owner:origin.sourceOwner,wrappers:[],collectionTypeIdentity}`.
The semantic carrier must be an actual registered nominal type with its complete
constructor/field inventory in `context.records`. Source HashMap/HashSet owners
remain the actual library owners, with exact complete scalar argument origins;
they are never relabeled as Vec or nominal source owners. The implementation
reuses `readRustStandardCollectionSourceOriginError` and checks concrete type
identity, source version, pointer width and complete source owners. The parser
slot must retain its actual `sourceCollectionTypeIdentity`.

Whole-carrier operations reuse the collection kernel only after this loan
engine checks the actual lexical root, availability, covering authority and
overlapping loans. A successful mutation returns both the changed collection
carrier and `loanState`. Existing result/state packet indices apply unchanged.
`get`, clone and new/default refuse in this adapter pending actual borrowed
projection or fresh-result handling. Borrowed get needs an actual
root/revision/key region interpretation; immediate copied/cloned projection
can compose only through the independently checked collection result adapter.
The registry must be projected from the actual checked helper carrier types,
not authored as a trust flag. Public source admission still needs the root's
complete library projection and signature validation.

Deferred controls in `translation-rust-exclusive-source-loans.test.js` cover
context versions/owners, overlapping and disjoint places, nonescaping reborrow,
lexical exit, ordered calls, packet placement, caller field reconstruction,
whole-Vec append, moved roots and forged continuations. The literal fixture
`rust-exclusive-source-loans.mjs` declares ten positive Rust originals and six
compiler-classified refusal originals under the same version/edition scope.
Those originals include future composed Result/error and receiver cases; their
presence does not assert current frontend admission. No native compiler or
source assertion has executed.

Root integration still owns automatic native/source author registration,
complete mutable signature admission, parser slots, free-standing complete
positive parser/checker controls, body/Call proof rechecking, source register
reconciliation and later translation/execution. General returned references,
borrowed fields in stored data, custom destructors/unwinding, branch joins,
arbitrary vector index loans and trait dispatch remain independent boundaries.
