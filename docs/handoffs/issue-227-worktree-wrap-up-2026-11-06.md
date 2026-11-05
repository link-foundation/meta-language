# Worktree and publication wrap-up — 2026-11-06

This is a Git/workspace audit, not implementation or behavioral delivery evidence.
The source head inspected was `7ebb25851b0cd2a12b1738bc9df2cf3aa7ec4bda`.
The [vision](../vision.md) remains the architecture and earlier code/control
handoffs retain all unfinished requirements. No tests, compilation, represented
programs or validation gates ran; no CI jobs, failure logs or artifacts were read.

## Local findings and preservation

- All listed subagents have completed their work.
- Thirty worktree registrations existed: the current task checkout, a clean
  checkout for issue #217, and 28 missing temporary directories with stale
  registrations. Every stale registration's retained index was compared with its
  own HEAD; all had no staged changes. Missing directories cannot be inspected
  for files that no longer exist; this audit does not claim to recover deleted,
  never-committed data.
- The two existing checkouts have no tracked modifications. There are no stashes.
  Issue #217's checkout is four commits behind its published branch, with no
  unpublished local work; it is retained as a separate task's checkout.
- Two inactive, untracked Codex session transcripts total 305,106,998 bytes. They
  are preserved with atomic same-filesystem renames into
  `.git/local-session-archives/2026-11-06/`, outside the working tree. Their bytes
  are not read, staged or deleted. The local receipt retains original/archived
  paths, sizes, timestamps and inode identities. They are local conversation
  records, not repository source changes or pushed artifacts.
- Only the 28 absent, unlocked, independently checked stale registrations are
  pruned. Existing checkouts, all local branches, committed objects, unrelated
  files and task evidence are preserved.

## Publishing historical committed work

A fresh `git ls-remote --heads origin` returned 107 remote branch heads. Of these,
106 commit objects were already present locally. Reachability against those
published heads left 314 commits across 123 local branch tips without proven
published coverage. The one unavailable remote head is not assumed to establish
ancestry. Some of this old work may have been cherry-picked into published
history under different commit identities; this audit preserves the exact
original objects rather than treating patch similarity as an ancestry proof.

The automatically captured [publication manifest](issue-227-worktree-publication-2026-11-06.lino)
maps each local branch and exact tip to
`archive/worktree-wrap-up-2026-11-06/<original-branch>`. These archive branches
preserve historical snapshots; they do not integrate old experiments or repairs
into the current implementation and do not establish that their code passes.
They are created without force updates or local branch deletion. An atomic push
publishes this audit commit on the current continuation branch and every archive
ref together. Fresh remote equality is checked after the push; the manifest is a
publication declaration, not a fabricated prior observation.

The workflow's push trigger selects `main`; the archive refs do not target that
branch and have no opened PRs. Only the existing continuation PR receives the
new audit commit. The source drafting phase and no-local-Rust rule are retained.

## Registrations inspected

| Original path | Original branch | HEAD | Finding |
| --- | --- | --- | --- |
| /Users/konard/Code/Archive/link-foundation/meta-language | refs/heads/issue-227-translation-continuation-2026-10-22 | `7ebb25851b0cd2a12b1738bc9df2cf3aa7ec4bda` | Existing; retained |
| /private/tmp/meta-language-anchored-owner-tests | refs/heads/bulk-nineteen-anchored-owner-tests | `2c4d9c995e0563cbae16af025e1c69001f634c23` | Missing directory; retained index matches HEAD; stale metadata pruned |
| /private/tmp/meta-language-binary-ci-repair | refs/heads/ci-binary-rule-styles | `1bee863ffc7594ab0af5a4851553130b506d96cc` | Missing directory; retained index matches HEAD; stale metadata pruned |
| /private/tmp/meta-language-bulk-eighteen-naming | refs/heads/bulk-eighteen-empty-option-naming | `3cf5bd8c16578aa81df15cb33de69a7d19cfbca3` | Missing directory; retained index matches HEAD; stale metadata pruned |
| /private/tmp/meta-language-ci-eighteen-product-completion | refs/heads/ci-eighteen-product-completion | `d0860f7a944e2bbf43563b45c3039d2b5473d20a` | Missing directory; retained index matches HEAD; stale metadata pruned |
| /private/tmp/meta-language-ci-eighteen-typed-program-rules | refs/heads/ci-eighteen-typed-program-rules | `b58d9fb88024b06c5b15f401786c891405254185` | Missing directory; retained index matches HEAD; stale metadata pruned |
| /private/tmp/meta-language-ci-pascal-migration-baseline | refs/heads/ci-pascal-migration-baseline | `1fb92395ed2c4b68f5e4a7c39d9717cc80a6303b` | Missing directory; retained index matches HEAD; stale metadata pruned |
| /private/tmp/meta-language-complete-javascript-replay | refs/heads/bulk-eighteen-complete-javascript-replay | `0d6df2b60af3448dc0d4758614accab2e2ecb312` | Missing directory; retained index matches HEAD; stale metadata pruned |
| /private/tmp/meta-language-empty-option-consumers | refs/heads/bulk-nineteen-empty-option-consumers | `7a8976d85ca734f58d6634489c01273bd8763afb` | Missing directory; retained index matches HEAD; stale metadata pruned |
| /private/tmp/meta-language-empty-option-gates | refs/heads/ci-empty-option-gates | `1f21fd4bd8a79dd7f76976b5a568638496d7a372` | Missing directory; retained index matches HEAD; stale metadata pruned |
| /private/tmp/meta-language-empty-option-source | refs/heads/bulk-nineteen-empty-option-source | `a58a153a19d046f7a09effe65d27bfe2a39facf9` | Missing directory; retained index matches HEAD; stale metadata pruned |
| /private/tmp/meta-language-generator-gates | refs/heads/ci-generated-gate-additions | `c23f04ebdcfe9681b7bfe3aca13476916a4106d9` | Missing directory; retained index matches HEAD; stale metadata pruned |
| /private/tmp/meta-language-host-clippy | refs/heads/bulk-twenty-host-clippy | `caaed8084579c6193e650854051f3cdc5db7cd15` | Missing directory; retained index matches HEAD; stale metadata pruned |
| /private/tmp/meta-language-mutable-tuple-binding | refs/heads/bulk-eighteen-mutable-tuple-binding | `0c58f17439ade83ac794a5f662ce91db09370ff6` | Missing directory; retained index matches HEAD; stale metadata pruned |
| /private/tmp/meta-language-namespace-public | refs/heads/bulk-nineteen-namespace-public | `04838a35f54e3af360256044a2c88160850f0b44` | Missing directory; retained index matches HEAD; stale metadata pruned |
| /private/tmp/meta-language-naming-reconciliation | refs/heads/bulk-eighteen-naming-reconciliation | `0b00b78a2f345456f8ffdf5e688223681a3bc011` | Missing directory; retained index matches HEAD; stale metadata pruned |
| /private/tmp/meta-language-native-type-namespace | refs/heads/bulk-nineteen-native-type-namespace | `00fc4f731ac296294cc8c74348150278cedf39d3` | Missing directory; retained index matches HEAD; stale metadata pruned |
| /private/tmp/meta-language-native-unit-api-repairs | refs/heads/ci-native-unit-api-repairs | `c6e4769fcc2dfe3f2defe536bb309c292ea65e62` | Missing directory; retained index matches HEAD; stale metadata pruned |
| /private/tmp/meta-language-numeric-function-model | refs/heads/numeric-function-model-draft | `e4ad67dd51c059610076a32e9ae0073a12aa44f5` | Missing directory; retained index matches HEAD; stale metadata pruned |
| /private/tmp/meta-language-owned-text-public | refs/heads/public-owned-text-consumers | `cdb229255181ccef9be4e81ef8afcc6c225bda07` | Missing directory; retained index matches HEAD; stale metadata pruned |
| /private/tmp/meta-language-package-partitions | refs/heads/ci-package-check-partitions | `35673d550eb4bcb6738d74bdfdce05f28aa11ca4` | Missing directory; retained index matches HEAD; stale metadata pruned |
| /private/tmp/meta-language-pure-unit-calls | refs/heads/bulk-nineteen-pure-unit-calls | `3058c46302ca12552e4cb1871cf3d5de5b6e46a9` | Missing directory; retained index matches HEAD; stale metadata pruned |
| /private/tmp/meta-language-qualified-primitive-owner | refs/heads/bulk-eighteen-qualified-primitive-owner | `8eb388db6e8bacdaf57a6381e070fca6f37f4d14` | Missing directory; retained index matches HEAD; stale metadata pruned |
| /private/tmp/meta-language-resource-description-loop-ownership | refs/heads/ci-resource-description-loop-ownership | `4e9e0db504e86ebdf5ec37f3901482fd60149ba0` | Missing directory; retained index matches HEAD; stale metadata pruned |
| /private/tmp/meta-language-rust-type-namespace-gates | refs/heads/ci-rust-type-namespace-gates | `85fb94f7af259f6c2a038e017294e27749e1180f` | Missing directory; retained index matches HEAD; stale metadata pruned |
| /private/tmp/meta-language-scalar-loop-profile | refs/heads/bulk-seventeen-scalar-loop-targets | `c067ef9c1591a04c753972cd48ac51566486f321` | Missing directory; retained index matches HEAD; stale metadata pruned |
| /private/tmp/meta-language-scoped-field-ownership | refs/heads/bulk-nineteen-scoped-field-ownership | `def09a2e97c71617fda117b2665e6285aac24909` | Missing directory; retained index matches HEAD; stale metadata pruned |
| /private/tmp/meta-language-selected-model-consumers | refs/heads/bulk-eighteen-selected-model-public-consumers | `1b2cf6f3fbdd94bb2b59bd69141ad47cb6296951` | Missing directory; retained index matches HEAD; stale metadata pruned |
| /private/tmp/meta-language-source-header-emission | refs/heads/bulk-nineteen-namespace-constructor-refusal | `fbc2f734d7961165ebbdea90d4a6073e0bbd2093` | Missing directory; retained index matches HEAD; stale metadata pruned |
| /Users/konard/Code/Archive/link-foundation/meta-language-tr | refs/heads/issue-217-wrap-long-rust-lines | `388197ec637365cf8e21557792e9dcded7998edf` | Existing; retained |
