// Atomic requirements of the repository cache cleanup requested in
// https://github.com/link-foundation/meta-language/pull/196#issuecomment-5856764580
// and restated in
// https://github.com/link-foundation/meta-language/pull/196#issuecomment-5870004788.
//
// The cleanup is repository tooling, so its cells run under the `tooling`
// runtime: the behavioral cells are recorded by js/tests/cache-cleanup*.test.js
// in temporary fixtures, and the measured cell by the evidence runner after it
// cleaned the real worktree.

export const CACHE_CLEANUP_FIXTURE = 'scripts/lib/cache-classes.mjs';

export const CACHE_CLEANUP_REQUIREMENTS = Object.freeze([
  {
    id: 'I195-CACHE-CLEANUP-ENTRY-POINT',
    construct: 'single documented entry point for every regenerable cache class',
    expectedBehavior:
      'node scripts/clean-caches.mjs discovers and removes Rust targets, JavaScript caches and temporary package consumers, generated parser and compiler intermediates, Lean and Rocq build output, acceptance and benchmark scratch, marked temporary clones with their nested Rust targets, and project-labeled container and BuildKit caches, reporting bytes before, after and reclaimed, and a repeated run is a no-op.',
    assertions: [
      'singleDocumentedEntryPoint',
      'everyRequiredCategoryRegistered',
      'everyClassRemovedFromFixture',
      'repeatedRunIsIdempotent',
      'reportsBeforeAfterReclaimed',
    ],
  },
  {
    id: 'I195-CACHE-CLEANUP-SAFETY',
    construct: 'repository-scoped, validated and concurrency-safe deletion',
    expectedBehavior:
      'The cleanup deletes only git-ignored, untracked paths inside its own worktree (or scratch it marked, or an explicit Cargo target directory), rejects traversal and symlink escapes, keeps source, tracked vendored grammars, fixtures, proofs, uncommitted work, evidence, other worktrees and outputs of active builds, tolerates missing tools and paths with spaces, touches only project-labeled container resources, and serializes concurrent runs.',
    assertions: [
      'sourceAndTrackedFilesPreserved',
      'ignoredOnlyInsideRepository',
      'symlinkAndTraversalRejected',
      'otherWorktreesPreserved',
      'activeBuildOutputsPreserved',
      'evidencePreserved',
      'customTargetDirectoriesHandled',
      'pathsWithSpacesHandled',
      'missingToolsTolerated',
      'onlyLabeledContainerResources',
      'concurrentCleanupsSerialized',
    ],
  },
  {
    id: 'I195-CACHE-CLEANUP-HOOK',
    construct: 'every-commit hook installed by the developer bootstrap',
    expectedBehavior:
      'node scripts/install-dev-hooks.mjs on a fresh clone installs a pre-commit hook that runs the cleanup on every commit, including documentation-only commits, chains an existing hook without losing its status, and is never installed into a downstream consumer by a build or package script.',
    assertions: [
      'freshCloneBootstrapInstallsHook',
      'nonCodeCommitRunsCleanup',
      'existingHooksComposed',
      'downstreamGitConfigUntouched',
    ],
  },
  {
    id: 'I195-CACHE-CLEANUP-EVENTS',
    construct: 'cleanup after build, test, coverage, benchmark, package, acceptance and CI teardown',
    expectedBehavior:
      'node scripts/with-cache-cleanup.mjs runs the command with bounded parallelism, no incremental compilation and a bounded compiler cache, waits for it on success, failure and interruption, then cleans the caches and exits with the command status.',
    assertions: [
      'cleanupAfterSuccess',
      'cleanupAfterFailure',
      'cleanupAfterInterruption',
      'exitStatusPreserved',
      'childrenAwaited',
      'boundedGrowthEnvironment',
    ],
  },
  {
    id: 'I195-CACHE-CLEANUP-BUDGET',
    construct: 'aggregate disk budget, warm cache, full clean and low-disk preflight',
    expectedBehavior:
      'Pruning removes transient caches and then warm caches in priority order until the aggregate fits the configured budget, keeps a warm cache that fits, removes every class in full mode, and escalates to a full clean when free disk space is below the preflight floor.',
    assertions: [
      'budgetEnforced',
      'warmCacheKeptWithinBudget',
      'fullModeRemovesEveryClass',
      'lowDiskPreflightEscalates',
    ],
  },
  {
    id: 'I195-CACHE-CLEANUP-POLICY',
    construct: 'CI policy check for hooks, wrappers, teardown, categories and build profiles',
    expectedBehavior:
      'node scripts/check-cache-policy.mjs passes on the repository and fails when the hook or its pre-commit entry is disabled, a cargo or npm step bypasses the wrapper, a job lacks the teardown that runs after failures, a required category has no class, or the dev and test profiles are not lean.',
    assertions: [
      'policyPassesOnRepository',
      'disabledHookDetected',
      'unwrappedCommandDetected',
      'missingTeardownDetected',
      'omittedCategoryDetected',
      'unboundedProfileDetected',
    ],
  },
  {
    id: 'I195-CACHE-CLEANUP-MEASURED',
    construct: 'measured disk use of the acceptance run before and after cleanup',
    expectedBehavior:
      'After the full pre-merge evidence run, the cleanup of the real worktree reports measured bytes before, after and reclaimed, ends within the budget, leaves the tracked state unchanged, and keeps every execution record, log and artifact the gate reads.',
    assertions: [
      'beforeAfterReclaimedMeasured',
      'withinBudgetAfterCleanup',
      'trackedStateUnchanged',
      'evidencePreservedAfterCleanup',
    ],
  },
]);

/** Builds the ledger entries with the ledger's own constructors. */
export function buildCacheCleanupRequirements(fixtureCatalog, { requirement, verification, pinnedFixture, source }) {
  return CACHE_CLEANUP_REQUIREMENTS.map(({ id, construct, expectedBehavior, assertions }) => {
    const fixtureId = `planned:cache-cleanup:${id.toLowerCase()}`;
    fixtureCatalog[fixtureId] = pinnedFixture(CACHE_CLEANUP_FIXTURE, construct);
    return requirement({
      id,
      source,
      area: 'cache-cleanup',
      scope: {
        language: 'repository tooling (Node.js scripts, git hooks and CI workflows)',
        version: 'Node.js 20 or later',
        edition: 'every checkout, worktree and CI job of this repository',
        construct,
        aliases: [],
        extensions: [],
      },
      expectedBehavior,
      requiredRuntimes: ['tooling'],
      implementationEntryPoints: {
        tooling: [
          'scripts/clean-caches.mjs',
          'scripts/lib/cache-cleanup.mjs',
          'scripts/lib/cache-classes.mjs',
          'scripts/with-cache-cleanup.mjs',
          'scripts/install-dev-hooks.mjs',
          'scripts/check-cache-policy.mjs',
          '.githooks/pre-commit',
        ],
      },
      verifications: [
        verification({
          requirementId: id,
          runtime: 'tooling',
          suffix: id === 'I195-CACHE-CLEANUP-MEASURED' ? 'measured' : 'behavior',
          fixtureIds: [fixtureId],
          assertions,
        }),
      ],
    });
  });
}
