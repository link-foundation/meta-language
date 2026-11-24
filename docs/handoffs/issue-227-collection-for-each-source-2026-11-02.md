# Map and Set forEach source batch — 2026-11-02

This handoff supports the authoritative [vision](../vision.md). It is a source
draft and integration recipe, not execution evidence or a competing specification.

## Authored source

`js/src/translation/javascript-collection-for-each.js` supplies a direct cursor
state machine over existing MapData/SetData insertion-history entries. It reuses
shared heap validation, branded receiver validation and SameValue. No iterator
object, iterator-result object, entry Array, Symbol.iterator property, size Get,
entries/values method or user next is involved. The compiler emits the dedicated
`collectionForEach` instruction under a protected actual standard function owner.

The receiver's actual collection slot is checked before callback callability,
including empty collections. The callback receives `(value, key, collection)`
for Map and `(value, value, collection)` for Set. thisArg passes unchanged into
shared Call, which retains lexical/strict/native callback conventions. Every
callback result is ignored. The original thrown value and all prior heap effects
escape; there is no IteratorClose operation for this direct data traversal.

One transition handles one insertion ordinal, including deleted entries, so the
VM's existing bounded stepping can suspend without eager whole-history scans.
The cursor advances before the callback starts. Every later step reads the
latest shared collection row, retaining appended entries, deleted/reinserted
ordinals and replaced future values. clear retains existing empty ordinals, and
new additions remain visible. Overwriting an existing visited entry does not
visit it again. Nested calls have separate cursor states. Selected callback
values/key are captured before Call and remain valid when that callback deletes,
changes or clears the source entry. Revalidating held values against the latest
entry would incorrectly reject legal callbacks and is deliberately avoided.

The transition union is `collectionForEachContinue`, `collectionForEachCall`,
`collectionForEachCompleted`, `collectionForEachThrow`, `collectionForEachError`
and `collectionForEachInvalid`. No fresh identity is needed. Shared VM stack,
argument, heap-history and total-step resource budgets remain responsible for
bounded work; resource suspension before starting a selected callback retains
its exact selected call for retry.

## Required shared integration

The root agent owns these shared files to avoid concurrent edits. Until all
entries below are connected, the new module alone does not satisfy public
forEach support.

1. `javascript-call-code.js`: add
   `{$:'collectionForEach',destination:number,operation:string}` to the instruction
   union; include it in the destination-only register coordinate group.
2. `javascript-call-machine.js`: import the kernel's operation test, begin, step,
   resume and state validator. Add
   `{$:'waitCollectionForEach',destination:number,realm:bigint,state:JavaScriptCollectionForEachState}`
   to pending states. Program validation requires a primitive intrinsic frame and
   a registered Map.forEach/Set.forEach operation.
3. Include `waitCollectionForEach` in `readJavaScriptCallPendingDestination` and
   selected-child frame verification. Its `state.call` supplies the exact child
   callee/receiver/arguments. A suspended state must have an actual selected call.
   Pending validation requires the preceding instruction to be collectionForEach,
   matching destination, realm, operation, frame receiver, callback argument zero
   and thisArg argument one. Run `readJavaScriptCollectionForEachStateError` on the
   latest heap. No exotic coercion Symbol is needed for this operation.
4. Instruction execution calls `beginJavaScriptCollectionForEach(heap, operation,
   registers[0] ?? undefined, frame.arguments[0] ?? undefined,
   frame.arguments[1] ?? undefined)`. Store the updated parent position as with
   arrayCallback. Idle pending stepping invokes `stepJavaScriptCollectionForEach`;
   normal child return invokes `resumeJavaScriptCollectionForEach` with its actual
   normal completion. Callback throws use the existing ordinary source unwinder:
   there are no traversal getters, finally regions or iterator cleanup to resume.
5. Add `acceptJavaScriptCallCollectionForEach` mirroring the Array callback
   acceptor. Always adopt `transition.heap`. Continue stores the pending state;
   Call stores it then uses shared `pushJavaScriptCall`; Completed clears pending
   through `writeJavaScriptCallRegister`; Throw uses ordinary throw unwinding;
   Error uses actual source realm Error allocation; Invalid reports Invalid.
   Shared Call resource failures leave the selected call pending, not completed.
6. `javascript-collection-call-source.js`: allow the operation test and use
   `compileJavaScriptCollectionForEachFunction` for its two registered owners.
   `javascript-collection-realm.js`: capture actual Map.prototype.forEach and
   Set.prototype.forEach descriptor values, authenticate them before publication
   and verification, and register their function identities as nonconstructible
   primitive method owners. Do not insert them in existing Map/Set data-operation
   lists: these are callback algorithms, not single-step collectionOperation.
7. Register the new module with deferred source policy authoring and the new
   control file/fixture in the complete native/source replay author. Automatically
   translate the complete JavaScript body to Rust when that phase resumes. Add
   relevant architecture prose to the vision and the user-visible changelog.
   Neither generated native semantics nor binary policy snapshots were edited.

## Deferred controls

`js/tests/fixtures/javascript-collection-for-each-source.mjs` records fifteen
independent original source programs. Its explicit Node ranges are
`>=22.15.0 <23.0.0` and `>=24.0.0 <25.0.0`; the dedicated runner selects against
`process.versions.node` and errors on uncovered versions rather than skipping.
The exact Error message formatter is not asserted by these fixtures: invalid
callback/receiver examples retain their inputs and assert the TypeError class.
The source kernel follows the existing generic collection Error-reason boundary;
complete native diagnostic spelling remains governed by the project's separate
version-sensitive diagnostic requirements.

`js/tests/translation-javascript-collection-for-each.test.js` adds seven kernel/
compiler controls and all fifteen original-versus-product oracles. They cover
live replacement, deletion/reinsertion, clear/append, held callback arguments,
three operands and thisArg, nested cursors, ignored callback results, normalized
zero, original throws/effects, receiver-before-callability, empty callbacks,
ordinal resource suspension and selected-call substitution refusal. Native
original execution is confined to deferred test callbacks; represented product
execution uses the captured source and portable VM.

No tests, imports executing represented code, generation, compilation, validators
or CI feedback were run/read for this batch. No execution observation is claimed.
The ledger/register audit, naming and runtime feature mappings remain deferred to
the mandated source-audit/generation phase. Unrelated journals were preserved.
