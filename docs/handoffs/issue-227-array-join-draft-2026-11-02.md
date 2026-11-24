# Issue 227 Array join source draft, 2026-11-02

Source draft only. No imports, tests, generation, validation, build or runtime
execution performed. This handoff describes integration work, not evidence.
The authoritative architecture remains docs/vision.md.

The new compiler exports `testJavaScriptArrayJoinOperation(operation)` for
`Array.join` and `compileJavaScriptArrayJoinFunction(identity, realm, roots)`.
`roots` has the existing `JavaScriptPrimitiveWrapperRoots` shape from
javascript-object-call-source.js: objectPrototype, stringPrototype,
numberPrototype, booleanPrototype, integerPrototype, symbolPrototype. All are
actual noncallable Object values. Missing/invalid roots refuse compilation.

The body uses root-owned appendJavaScriptObjectCoercion with register operands
(code, 0, 1, 7, 14, 8, 13, roots). This requires coercibility, preserves existing
Object receivers and selects
actual primitive wrapper roots before `primitiveWrapper` allocation. Length is
Get once, implicit ToNumber and shared ToLength. Separator conversion follows
that snapshot even for length zero. Undefined alone chooses comma; all other
separators and nonnullish elements use implicit `convert primitive text`, which
retains Symbol rejection and ordinary object conversion callbacks. A Number
index ascends through every saved coordinate with ordinaryGet, including holes
and inherited getters. No HasProperty or iteration protocol substitutes for Get.
Text concatenation retains existing shared output resource bounds.

Root integration needs two new instructions:

- `{$:'arrayJoinEnter', destination, receiver}` returns a Boolean. It compares
  the selected Object identity with active join receivers; true acquires a
  frame-owned token, false acquires nothing. All Object receivers participate.
- `{$:'arrayJoinLeave', receiver}` releases only this frame's acquired token.
  Return/throw/frame unwinding must release an owned token too, while a false
  enter must never release the enclosing owner's token. Resource suspension
  retains it and resumes without acquiring a second token.

The body converts separator before checking empty length, returns empty for
zero length before acquiring a token, returns empty on duplicate entry, and
releases successful entry on normal completion. Nested joins during separator
conversion occur before this call's token acquisition.

Version qualification: the recursion behavior is a V8 compatibility operation,
not an invented ECMAScript abstract operation. Root should gate it by its actual
Node/V8 profile. The requested V8 tag 12.8.216 source could not be retrieved in
this environment (web fetch unavailable; shell DNS unavailable). The readable
upstream source at commit dc5908fd731c73e02c329b9336af6940ca70ae3d, and separately
c73144da2b1ab7d385bbfe79cb5f95395ad632e0, uses JSReceiver identities, len > 0 before
JoinStackPushInline, and pop on both success and exception. These source commits
are read-only design references, not observations for any Node version. Exact
Node22/24 coverage and their corresponding V8 versions remain unverified.

References:
https://chromium.googlesource.com/v8/v8/+/dc5908fd731c73e02c329b9336af6940ca70ae3d/src/builtins/array-join.tq
https://chromium.googlesource.com/v8/v8/+/c73144da2b1ab7d385bbfe79cb5f95395ad632e0/src/builtins/array-join.tq

Deferred direct controls cover root refusal, coercibility/boxing order,
length-before-separator and empty conversion ordering, ordered Get without Has,
nullish element branches, guard branch ownership and bounded destinations.
Deferred shared-machine controls cover inherited holes, primitive UTF16 String
boxing, separator mutation after length snapshot, recursive generic receivers
and fresh subsequent calls. They have not executed and do not establish parity.
Root owns catalog linkage, source author registration, VM schemas/validation/
codec and translation support, original/native fixture producers, version gates,
architecture update and all later verification. Additional abrupt cleanup,
getter/effect and cyclic Array original/native coverage belongs in that batch.
