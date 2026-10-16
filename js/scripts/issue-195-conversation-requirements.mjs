// Critical conversation obligations. Captures are requirements, not behavioral
// fixtures or successful execution evidence. Missing implementations stay null.
const capture = (name) => `parity/sources/${name}-2026-10-12.txt`;
const definitionSource = capture('recursive-formal-definitions');
const linksSource = capture('links-foundation-and-fractal-motivation');
const recordingSource = capture('record-conversation-requirements');
const stackSource = capture('associative-stack-and-theory-foundation');
const originalSource = capture('global-concept-space');
const dynamicSource = capture('dynamic-knowledge-and-grammar-inference');
const translationSource = capture('bidirectional-translation-priority');
const projectSource = capture('multi-project-translation-reuse');
const externalGrammarSource = capture('temporary-external-grammar-inputs');
const externalGrammarIgnoreSource = capture('ignored-external-grammar-folders');
const associativeDataSource = capture('associative-data-storage');
const agentReductionSource = capture('gradual-agent-reduction');
const localResourceScopeSource = 'parity/sources/meta-language-local-resource-scope-2026-10-16.txt';
const ciCancellationInvestigationSource = 'parity/sources/ci-cancellation-investigation-2026-10-16.txt';
const modernJavaScriptParitySource = 'parity/sources/modern-javascript-parity-expectations-2026-10-16.txt';
const compactContributorInstructionsSource = 'parity/sources/compact-contributor-instructions-and-assertion-corrections-2026-10-16.txt';
const versionBoundedAssertionsSource = 'parity/sources/version-bounded-assertions-2026-10-16.txt';
const inferenceSource = 'parity/sources/grammar-inference-issue-224.txt';

const sourceTooling = {
  'I195-WORKFLOW-CANCELLED-CI-INVESTIGATION': [
    'js/scripts/ci-cancellation-investigation.mjs', 'js/tests/ci-cancellation-investigation.test.js',
  ],
  'I195-THEORY-FOUNDATION-PINNED-SOURCE-AUDIT': [
    'js/scripts/meta-theory-foundation-sources.mjs', 'js/tests/issue-195-meta-theory-sources.test.js',
  ],
  'I195-ASSOCIATIVE-STACK-PINNED-SOURCE-AUDIT': [
    'js/scripts/associative-stack-sources.mjs', 'js/tests/issue-195-associative-stack-sources.test.js',
  ],
};

const sourceToolingFixtures = {
  ...Object.fromEntries([
    'I195-ASSERTION-EXPLICIT-VERSION-RANGES', 'I195-ASSERTION-VERSION-COVERAGE', 'I195-ASSERTION-LANGUAGE-EDITION-ORDER',
  ].map(id => [id, [
    { id: 'assertion-version-ranges', path: 'parity/assertion-version-ranges.lino' },
    { id: 'unicode-identifier-properties', path: 'js/src/data/unicode-identifier-properties.lino' },
  ]])),
  'I195-WORKFLOW-CANCELLED-CI-INVESTIGATION': [
    { id: 'ci-cancellation-investigation', path: 'parity/fixtures/ci-cancellation-investigation.lino' },
  ],
};

// Entry points identify executable implementations. Their presence is not an
// observation: the tooling tests emit each cell only after its whole behavior
// succeeds, with the original conversation capture retained as a fixture.
const dataTooling = {
  'I195-ASSOCIATIVE-DATA-STORAGE': {
    tooling: ['js/src/structured-data-links.js', 'rust/src/structured_data_links.rs',
      'js/scripts/project-data-storage.mjs', 'js/scripts/project-data-tooling-evidence.mjs'],
    fixtures: ['structured-data-migration-baselines', 'project-data-json-exceptions'],
  },
  'I195-ASSOCIATIVE-DATA-AUTOMATED-CONVERSION': {
    tooling: ['js/scripts/convert-structured-data.mjs', 'js/scripts/validate-structured-data-migration.mjs',
      'js/scripts/fixture-source-bytes.mjs', 'js/src/project-data-files.js', 'rust/src/project_data.rs'],
    fixtures: ['structured-data-migration-baselines'],
  },
  'I195-ASSOCIATIVE-DATA-JSON-EXCEPTIONS': {
    tooling: ['js/scripts/project-data-storage.mjs', 'js/scripts/check-project-data-storage.mjs',
      'js/tests/project-data-tooling-evidence.test.js'],
    fixtures: ['project-data-json-exceptions'],
  },
  'I195-EXTERNAL-GRAMMAR-UNTRACKED': {
    tooling: ['js/scripts/external-grammar-cache.mjs', 'js/scripts/project-data-tooling-evidence.mjs'],
    fixtures: ['external-grammar-assets', 'language-grammar-inventory'],
  },
  'I195-EXTERNAL-GRAMMAR-TEMPORARY-DOWNLOAD': {
    tooling: ['js/scripts/external-grammar-cache.mjs', 'js/scripts/project-data-tooling-evidence.mjs',
      'js/scripts/check-prepared-native-snapshots.mjs'],
    fixtures: ['external-grammar-assets', 'language-grammar-inventory'],
  },
  'I195-EXTERNAL-GRAMMAR-IGNORED-DIRECTORY': {
    tooling: ['.gitignore', 'js/scripts/project-data-tooling-evidence.mjs',
      'js/tests/project-data-tooling-evidence.test.js'],
    fixtures: ['external-grammar-assets', 'language-grammar-inventory'],
  },
};

const dataToolingFixtures = {
  'structured-data-migration-baselines': 'parity/fixtures/structured-data-migration-baselines.lino',
  'project-data-json-exceptions': 'parity/project-data-json-exceptions.lino',
  'external-grammar-assets': 'parity/grammars/external-assets.lino',
  'language-grammar-inventory': 'parity/language-grammar-inventory.lino',
};

const runtimeEntryPoints = {
  'I195-GRAMMAR-INFERENCE-FORMAL-CONCEPT-INTEGRATION': {
    javascript: ['js/src/discovered-concept-links.js'],
    rust: ['rust/src/discovered_concept_links.rs'],
  },
  ...Object.fromEntries([
    'I195-ONTOLOGY-LIBRARY-ENTRYPOINT', 'I195-TAXONOMY-LIBRARY-ENTRYPOINT',
    'I195-ONTOLOGY-TAXONOMY-MEMBERSHIP', 'I195-ONTOLOGY-TAXONOMY-RELATION-DISTINCTION',
    'I195-ONTOLOGY-MULTIPLE-LINK-ROOTS', 'I195-TAXONOMY-MULTIPLE-LINK-ROOTS',
  ].map(id => [id, {
    javascript: ['js/src/concept-views.js'], rust: ['rust/src/concept_ontology/views.rs'],
  }])),
};

function rows(sourceKey, fixture, specification, area, declarations, tooling = false) {
  return declarations.map(([id, construct, expectedBehavior, assertions]) => ({
    id, construct, expectedBehavior, assertions, sourceKey, fixture,
    fixtureId: `conversation-contract:${sourceKey}`,
    ...(area === 'formal-concept-definitions' ? {
      additionalFixtures: [{ id: 'formal-concept-definitions', path: 'parity/fixtures/formal-concept-definitions.json' }],
    } : {}),
    specification, area,
    ...(tooling ? { tooling: dataTooling[id]?.tooling ?? sourceTooling[id] ?? null } : (runtimeEntryPoints[id] ?? { javascript: null, rust: null })),
    ...(dataTooling[id] ? {
      additionalFixtures: [
        { id: 'project-data-tooling-contract', path: 'parity/fixtures/project-data-tooling-contract.lino' },
        ...dataTooling[id].fixtures.map(id => ({ id, path: dataToolingFixtures[id] })),
      ],
    } : {}),
    ...(sourceToolingFixtures[id] ? { additionalFixtures: sourceToolingFixtures[id] } : {}),
  }));
}

const ASSERTION_POLICY_CONVERSATION_REQUIREMENTS = Object.freeze([
  ...rows('modernJavaScriptParityRequest', modernJavaScriptParitySource, 'critical-conversation-requirements', 'modern-javascript-parity', [
    ['I195-PARITY-MODERN-JAVASCRIPT-ARRAY-BEHAVIOR', 'modern JavaScript string length and observable out of bounds array behavior',
      'For the retained conflicting array and string inputs, use the newer JavaScript behavior in both runtime packages: actual UTF-16 code unit string lengths and observable out of bounds array reads remain part of the selected semantics. Preserve every original source input, including the complete ARRAYS corpus and the original string length source, and add genuine cross-runtime controls for supplementary Unicode and out of bounds reads rather than preserving obsolete contradictory native expectations.',
      ['originalArrayAndStringInputsRetained', 'utf16StringLengthParityExecuted', 'observableOutOfBoundsParityExecuted']],
    ['I195-PARITY-OBSOLETE-NATIVE-EXPECTATIONS', 'evidence based correction of obsolete native array expectations',
      'Correct the obsolete Rust expectations identified in the captured question and answer to the newer JavaScript behavior while retaining all original test inputs and intended coverage. Record the actual contradictory before and after expectations and the active language or dependency context; genuine unsupported mutations and unrelated refusal boundaries remain required. Updating a wrong expectation is not permission to drop a fixture, hide a failed observation or claim unexecuted native parity.',
      ['obsoleteExpectationConflictRecorded', 'correctedExpectationsFollowModernJavaScript', 'originalCoverageAndGenuineRefusalsRetained']],
  ]),
  ...rows('compactContributorInstructionsRequest', compactContributorInstructionsSource, 'critical-conversation-requirements', 'contributor-instructions', [
    ['I195-CONTRIBUTOR-INSTRUCTIONS-CONCISE', 'minimal contributor instruction wording with unchanged obligations',
      'Reduce AGENTS.md and CLAUDE.md to the minimum practical wording while retaining the same substantive contributor obligations and the authoritative vision reference. Shared instructions may be referenced to avoid duplication; brevity does not remove implementation, evidence, resource, JavaScript-first, native CI or review obligations.',
      ['contributorInstructionsReducedWithoutLostObligations', 'sharedInstructionsRemainAuthoritative']],
    ['I195-ASSERTION-EVIDENCE-BASED-CORRECTION', 'demonstrably wrong assertions remain correctable without losing coverage',
      'Allow demonstrably wrong assertions and obsolete expected values to be corrected using recorded source, language or dependency evidence while preserving the original test inputs, genuine requirements, intended coverage and honest failure reporting. Record the reason and replacement expectation, update affected tests consistently and execute the corrected applicable assertions; a correction cannot excuse a real defect, weaken an acceptance obligation or stand in for execution evidence.',
      ['wrongAssertionCorrectionSupportedByEvidence', 'originalInputsAndCoverageRetained', 'correctedApplicableAssertionsExecuted']],
  ], true),
  ...rows('versionBoundedAssertionsRequest', versionBoundedAssertionsSource, 'critical-conversation-requirements', 'assertion-version-ranges', [
    ['I195-ASSERTION-EXPLICIT-VERSION-RANGES', 'explicit applicable version ranges for version dependent assertions',
      'Every assertion whose expected behavior depends on a language or dependency version declares its version subject, explicit minimum and maximum bounds, endpoint inclusion and evidence for the expectation. Select and execute the expectation applicable to the actual active version instead of treating one historical result as valid for all versions; retain the original inputs and the recorded historical expectation.',
      ['versionDependentExpectationsHaveExplicitBounds', 'activeVersionSelectsApplicableExpectation', 'historicalInputsAndExpectationsRetained']],
    ['I195-ASSERTION-VERSION-COVERAGE', 'missing and conflicting active version expectation coverage refuses',
      'Require exactly one applicable expectation for an active version and validate every declared range, including inactive alternatives. Missing coverage, overlapping or conflicting ranges, malformed versions and ambiguous expectation identities fail explicitly rather than skipping the assertion or choosing an arbitrary alternative.',
      ['missingActiveVersionCoverageRefused', 'overlappingAndConflictingRangesRefused', 'allDeclaredRangesValidated']],
    ['I195-ASSERTION-LANGUAGE-EDITION-ORDER', 'declared language edition order distinct from release coordinates',
      'Compare dependency release coordinates with their explicit precision, and compare language editions using their declared order rather than assuming edition names obey semantic version ordering. Unknown active editions and invalid release coordinates refuse before expectation selection.',
      ['releasePrecisionExplicit', 'languageEditionOrderExplicit', 'unknownEditionsAndInvalidCoordinatesRefused']],
  ], true),
]);

export const CRITICAL_CONVERSATION_REQUIREMENTS = Object.freeze([
  ...rows('ciCancellationInvestigationRequest', ciCancellationInvestigationSource, 'critical-conversation-requirements', 'workflow-contract', [
    ['I195-WORKFLOW-CANCELLED-CI-INVESTIGATION', 'evidence-based investigation of the reported CI run cancellation',
      'The reported cancellation of CI run 38431880453 is investigated using actual workflow, job or check-run evidence and the relevant workflow configuration. Its cause and supporting evidence are recorded rather than assuming the user cancelled it. The investigation preserves the existing CI follow-through and repair requirements.',
      ['cancelledWorkflowInvestigated', 'cancellationCauseSupportedByActualWorkflowEvidence']],
  ], true),
  ...rows('localResourceScopeRequest', localResourceScopeSource, 'critical-conversation-requirements', 'workflow-contract', [
    ['I195-LOCAL-RESOURCE-PROJECT-OWNED-CLEANUP', 'cleanup limited to disk space owned by this meta-language task',
      'Cleanup performed for this task removes only disk space demonstrably owned by meta-language work. Unrelated user files, other projects and system data remain untouched; recovering disk space does not authorize broader deletion.',
      ['metaLanguageOwnershipEstablishedBeforeCleanup', 'unrelatedDiskSpacePreserved']],
    ['I195-LOCAL-RESOURCE-CAREFUL-BOUNDED-USE', 'careful local resource use within available capacity',
      'Contributors check available system resources and keep local disk, memory and process use bounded while performing this task, preventing meta-language work from exhausting the host resources. Resource care does not remove inherited project requirements or replace required CI verification.',
      ['localResourceUseChecked', 'localWorkBoundedToAvailableResources']],
    ['I195-LOCAL-RESOURCE-RUST-COMPILATION-CI-ONLY', 'Rust compilation exclusively in CI/CD for this task',
      'No contributor compiles Rust locally for any purpose during this task. Rust compilation and compiled validation run only in CI/CD; local authoring and bounded JavaScript checks preserve all required native verification obligations.',
      ['noLocalRustCompilation', 'rustCompilationRunsOnlyInCi']],
  ], true),
  ...rows('agentReductionRequest', agentReductionSource, 'critical-conversation-requirements', 'workflow-contract', [
    ['I195-WORKFLOW-GRADUAL-AGENT-REDUCTION', 'gradual conclusion of agent work followed by coordinator-only implementation',
      'After the later user steering, active contributors finish their existing scoped work and agent activity gradually concludes. Once all active contributors finish, the coordinator continues implementation and CI follow-up alone. The earlier parallel drafting phase remains recorded, and no inherited project scope is removed.',
      ['existingAgentWorkConcludesGradually', 'coordinatorWorksAloneAfterAgentCompletion', 'inheritedProjectScopeRetained']],
  ], true),
  ...rows('associativeDataStorageRequest', associativeDataSource, 'associative-data-storage', 'associative-data-storage', [
    ['I195-ASSOCIATIVE-DATA-STORAGE', 'project-owned structured data stored through the associative Links stack',
      'Project-owned structured records, including native grammar corpus fixtures such as the Agda corpus, use inspectable Links Notation through the relevant associative stack by default. Generated native grammars, fixture records and shared package data remain reconstructable and usable through the existing public algorithms without hidden authoritative JSON side tables.',
      ['projectStructuredDataUsesLinksNotation', 'nativeGrammarCorpusStoredAsLinkedRecords', 'associativeStackDataAlgorithmsReused']],
    ['I195-ASSOCIATIVE-DATA-AUTOMATED-CONVERSION', 'automated lossless conversion of structured data and fixture metadata',
      'Structured-data migration is automated rather than manually rewriting records. The converter losslessly reconstructs the original values and preserves identities, source order, exact numbers, strings, aliases, required fixtures, acceptance assertions and cross-record references. Both package readers and repository tooling retain their observable behavior, and conversion does not reduce protected requirement scope.',
      ['structuredDataConversionAutomated', 'convertedValuesLosslesslyReconstructed', 'identitiesOrderNumbersStringsAliasesAndReferencesPreserved', 'fixtureIdsAndAcceptanceAssertionsPreserved', 'packageReadersAndToolingBehaviorPreserved', 'protectedScopeNotReduced']],
    ['I195-ASSOCIATIVE-DATA-JSON-EXCEPTIONS', 'JSON retained only for necessary declared external interfaces',
      'JSON remains only where a genuine external standard or source-language JSON interface requires it, with an explicit recorded necessity and boundary. Convenience, an existing JSON file or an opaque payload does not establish an exception; the project-owned authoritative representation remains linked data wherever the interface permits it.',
      ['jsonExceptionsHaveExplicitNecessity', 'externalJsonInterfaceBoundariesRecorded', 'noConvenienceOrOpaquePayloadException']],
  ], true),
  ...rows('externalGrammarStorageRequest', externalGrammarSource, 'temporary-external-grammar-inputs', 'external-grammar-storage', [
    ['I195-EXTERNAL-GRAMMAR-UNTRACKED', 'upstream external grammar and parser assets excluded from version control',
      'Upstream external raw grammars, parser sources, compiled parsers and WebAssembly assets are not tracked in the repository. The project-generated native or meta-language Links Notation grammars and prepared snapshots remain permitted and preserve all inherited grammar, provenance and offline production requirements.',
      ['externalUpstreamAssetsUntracked', 'generatedNativeSnapshotsRetained', 'inheritedOfflineAndProvenanceRequirementsPreserved']],
    ['I195-EXTERNAL-GRAMMAR-TEMPORARY-DOWNLOAD', 'temporary pinned external grammar inputs with checked provenance',
      'A reproducible tool downloads upstream external grammar and oracle assets temporarily when import, regeneration or independent tests require them, verifies their pinned checksums and records revision, origin and license provenance. Downloaded inputs remain outside tracked files, temporary assets are cleaned after use, and generated native snapshots remain independently usable without those assets.',
      ['temporaryDownloaderExecuted', 'pinnedChecksumsVerified', 'downloadProvenanceRetained', 'downloadedAssetsUntracked', 'temporaryAssetsCleaned', 'generatedSnapshotsUsableWithoutDownloads']],
  ], true),
  ...rows('externalGrammarIgnoreRequest', externalGrammarIgnoreSource, 'temporary-external-grammar-inputs', 'external-grammar-storage', [
    ['I195-EXTERNAL-GRAMMAR-IGNORED-DIRECTORY', 'external grammar download folders explicitly ignored by Git',
      'Every folder used for external grammar, parser or oracle downloads is explicitly covered by Git ignore rules, so ordinary staging cannot add downloaded inputs. Ignore rules preserve the project-generated native Links Notation grammars, source manifests, checksums and provenance records that must remain tracked.',
      ['externalGrammarDirectoriesIgnored', 'ordinaryStagingExcludesDownloads', 'generatedSnapshotsAndProvenanceRemainTracked']],
  ], true),
  ...rows('grammarInferenceIssue', inferenceSource, 'dynamic-knowledge-and-language-extension', 'grammar-inference', [
    ['I195-GRAMMAR-INFERENCE-EXISTING-WORK-AUDIT', 'existing inference foundations and JavaScript parity gaps',
      'Audit and reuse the lexical, state-merging, context-free, structural-prior, active-learning and minimization work associated with issues93,114,117 through123 and the existing Rust inference modules. Closed issues do not prove complete capability or remove JavaScript parity obligations.', ['existingInferenceModulesAudited', 'closedIssueEvidenceRechecked', 'javascriptParityGapsRecorded']],
    ['I195-GRAMMAR-INFERENCE-RESEARCH-SELECTION', 'pinned modern grammar discovery research and selection rationale',
      'Compare the pinned XVada, Crucio, NatGI and attributed grammar-inference methods with recorded revisions, implementations, licenses, independent oracles, datasets and configurations. Paper-reported results remain hypotheses for local reproduction, and method selection retains its evidence rationale.', ['researchRevisionsAndLicensesPinned', 'fourResearchMethodsEvaluated', 'selectionRationaleAndReproductionRecorded']],
    ['I195-GRAMMAR-INFERENCE-INDEPENDENT-EVALUATION', 'independent held-out grammar discovery evaluation',
      'Separate discovery training from holdout evaluation and test adversarial rejections and real projects under several independent oracles. Record precision, recall, F1, CST fields, trivia, spans, attributed behavior, reconstruction, grammar size, determinism, query counts, elapsed time and peak memory.', ['trainingAndHoldoutSeparated', 'multipleIndependentOraclesAndProjectsUsed', 'allRequiredQualityAndResourceMetricsRecorded']],
    ['I195-GRAMMAR-INFERENCE-COMPARATIVE-BENCHMARKS', 'reproducible inference comparison within common budgets',
      'Benchmark existing inference and selected modern methods with the same pinned workloads and budgets. Retain all failures and disagreement traces, select reproducible improvements without weakening acceptance, and keep statistical coverage distinct from semantic proof.', ['commonBudgetsAndWorkloadsUsed', 'failuresAndDisagreementsRetained', 'reproducibleImprovementMeasured', 'statisticsNotPresentedAsProof']],
    ['I195-GRAMMAR-INFERENCE-CI-RESOURCE-POLICY', 'bounded JavaScript-first inference development and CI artifacts',
      'Use targeted JavaScript authoring locally; run Rust compilation, broad discovery experiments, whole corpora and downstream workloads in CI with bounded caches and per-run artifacts. Record critical scope, API parity and exact executed observations without completion claims for missing behavior.', ['javascriptFirstInferenceDevelopment', 'expensiveInferenceAndRustRunInCI', 'boundedCachesAndRunArtifacts', 'exactEvidenceAndApiParityRecorded']],
  ], true),
  ...rows('grammarInferenceIssue', inferenceSource, 'dynamic-knowledge-and-language-extension', 'grammar-inference', [
    ['I195-GRAMMAR-INFERENCE-PUBLIC-API-CLI', 'matching public grammar discovery library and CLI interfaces',
      'JavaScript and Rust library and CLI interfaces accept pinned positive and negative examples, independent membership or parser oracles, optional execution traces and existing grammars. Diagnostics identify unresolved context sensitivity, scanners and action semantics rather than silently approximating them.', ['matchingDiscoveryApiAndCliExecuted', 'examplesOraclesTracesAndSourcesAccepted', 'unsupportedSemanticsPreciselyDiagnosed']],
    ['I195-GRAMMAR-INFERENCE-NATIVE-OUTPUT', 'fully native executable discovered grammars and attributed behavior',
      'Discovery emits executable Links grammars with lossless CST structure, lexer and scanner state, precedence, ambiguity, recovery and attributed semantic actions. Foreign parser blobs, opaque sources and host callbacks cannot stand in for native operational definitions.', ['discoveredGrammarNativeAndExecutable', 'discoveredCstAndScannerSemanticsPreserved', 'attributedActionsAndRecoveryExplicit']],
    ['I195-GRAMMAR-INFERENCE-GENERIC-UNION', 'generic structural and semantic discovery through the source-union pipeline',
      'Discover lexical classes, recursion and semantic relations generically, preserving every source production and provenance. Merge discovered definitions with the applicable official and mature grammars through the existing automatic native pipeline.', ['lexicalRecursiveAndSemanticDiscoveryExecuted', 'allSourceProductionsAndProvenanceRetained', 'discoveredAndOfficialGrammarsMerged']],
    ['I195-GRAMMAR-INFERENCE-FORMAL-CONCEPT-INTEGRATION', 'discovered concepts integrated with recursive formal definitions',
      'Copy discovered concepts into the single global space with formal definitions that recursively reference other linked concepts. Deduplicate only checked context-preserving one-to-one meaning certificates; equal strings, names, glosses or finite samples do not establish equivalence, and unresolved alternatives survive.', ['discoveredConceptsAndFormalDefinitionsLinked', 'contextPreservingMeaningCertificatesChecked', 'unresolvedDiscoveryAlternativesPreserved']],
    ['I195-GRAMMAR-INFERENCE-DETERMINISTIC-REFINEMENT', 'deterministic counterexample-guided incremental discovery',
      'Seeded discovery is deterministic and supports counterexample-guided refinement, incremental updates and checkpoint resume without losing the examples, decisions, assumptions or rejected alternatives used to obtain the grammar.', ['seededDiscoveryDeterministic', 'counterexampleAndIncrementalRefinementExecuted', 'checkpointResumeRetainsEvidence']],
    ['I195-GRAMMAR-INFERENCE-SOURCE-FREE-SNAPSHOTS', 'source-free discovered grammar snapshots in text and versioned binary links',
      'Discovered snapshots restore and extend through link-cli, text Links Notation and a versioned binary links representation without their original source buffers, retaining identities, cycles, roots, full CSTs and proofs.', ['discoverySnapshotsReloadWithoutSource', 'linkCliTextAndVersionedBinaryAgree', 'snapshotIdentitiesRootsTreesAndProofsRetained']],
    ['I195-GRAMMAR-INFERENCE-DYNAMIC-TRANSLATION', 'discovered languages execute and translate in the generic runtime',
      'After installing a discovered language dynamically, ordinary parsing and Rust/JavaScript translation execute through the common algorithms. Changing a linked definition changes execution without changing the generic host runtime; stale certificates and incompatible definitions are rejected or revalidated explicitly.', ['discoveredLanguageParsesAndTranslates', 'linkedMutationChangesGenericExecution', 'staleCertificatesRejectedOrRevalidated']],
  ]),
  ...rows('dynamicKnowledgeRequest', dynamicSource, 'dynamic-knowledge-and-language-extension', 'dynamic-knowledge', [
    ['I195-KNOWLEDGE-RDF-SUPPORT', 'full Resource Description Framework representation and operations',
      'Both packages preserve the complete required RDF constructs, identities, relationships, literals, provenance and selected semantics as inspectable links. A finite triple example or opaque payload does not establish full RDF support.', ['rdfConstructsPreserved', 'rdfSemanticsAndProvenanceExplicit']],
    ['I195-KNOWLEDGE-OWL-SUPPORT', 'full Web Ontology Language representation and operations',
      'Both packages represent the required OWL constructs, formal axioms and chosen semantic profiles as links, preserving distinctions and obligations without silently replacing the consumer foundation.', ['owlConstructsAndAxiomsPreserved', 'owlSemanticProfilesExplicit', 'consumerFoundationRetained']],
    ['I195-KNOWLEDGE-RDF-OWL-IMPORT', 'complete RDF and OWL importers',
      'Public importers in both packages import required RDF and OWL representations into the common concept and links space with their full structure, provenance and explicit diagnostics for unresolved obligations.', ['rdfAndOwlPublicImportersExecuted', 'importedKnowledgeStructureAndProvenanceRetained']],
    ['I195-KNOWLEDGE-RDF-OWL-CONVERSION', 'faithful RDF and OWL conversion cycles',
      'Public converters and exporters in both packages execute faithful knowledge-format conversion cycles from the shared representation without returning a saved input or silently weakening axioms and identities.', ['knowledgeConversionCyclesExecuted', 'knowledgeAxiomsAndIdentitiesPreserved', 'noSavedInputConversionShortcut']],
    ['I195-KNOWLEDGE-RDF-OWL-MERGE', 'meaning-preserving RDF and OWL merges',
      'Both packages merge imported RDF and OWL knowledge through the common links space, deduplicating only proved correspondences while retaining unique concepts, conflicting axioms, uncertainty and source-qualified provenance.', ['knowledgeMergesExecuted', 'knowledgeDeduplicationProved', 'conflictsAndUncertaintyPreserved']],
    ['I195-DYNAMIC-SNAPSHOT-RECONSTRUCTION', 'static snapshot becomes a complete dynamic representation on demand',
      'A prepared static snapshot can be reconstructed on demand into the complete CST and concept space, retaining rules, formal definitions, identities, recursive links, source bytes and provenance instead of reconstructing a token list or descriptor.', ['snapshotReconstructsFullCst', 'snapshotReconstructsFullConceptSpace', 'dynamicReconstructionRetainsProvenance']],
    ['I195-DYNAMIC-RECONSTRUCTION-RESOURCE-MEASUREMENT', 'measured latency and resources for complete dynamic reconstruction',
      'Both packages measure actual complete CST and concept-space reconstruction through the public link-cli, text Links Notation and binary Links Notation paths. Measurements record reconstruction latency and peak resources together with pinned input identity, representation, size, algorithm and execution environment provenance. The reconstructed graphs preserve the complete trees, rules, recursive formal definitions, identities, references, selected models, source bytes and provenance. Lazy loading, token-only reconstruction and descriptor creation cannot count as a measured full rebuild. These measurements retain the source qualitative speed requirement without inventing a numeric deadline.',
      ['completeDynamicReconstructionMeasured', 'reconstructionLatencyAndResourcesRecorded', 'measurementInputAndEnvironmentProvenanceRetained', 'fullRepresentationPreservationRetained', 'lazyOrTokenOnlyReconstructionNotCounted']],
    ['I195-DYNAMIC-LINK-CLI-RECONSTRUCTION', 'dynamic reconstruction through link-cli',
      'The dynamic representation can be recreated and processed through link-cli with a reproducible public path, preserving the complete trees, concept definitions and selected model.', ['linkCliReconstructionExecuted', 'linkCliPreservesFullTreesAndConcepts']],
    ['I195-DYNAMIC-TEXT-LINKS-RECONSTRUCTION', 'dynamic reconstruction from text Links Notation',
      'Text Links Notation can serialize, reload and extend the complete dynamic CST and concept space without the original grammar or source buffers.', ['textLinksDynamicRoundTripExecuted', 'textLinksPreservesCompleteRepresentation']],
    ['I195-DYNAMIC-BINARY-LINKS-RECONSTRUCTION', 'dynamic reconstruction from binary Links Notation',
      'Binary Links Notation can serialize, reload and extend the same complete dynamic representation with parity to its text form and without hidden authoritative side tables.', ['binaryLinksDynamicRoundTripExecuted', 'textAndBinaryDynamicRepresentationsAgree']],
    ['I195-DYNAMIC-LANGUAGE-EXTENSION', 'algorithmic runtime extension with other languages',
      'Users can apply the shared algorithms to extend the dynamic meta-language with new languages, grammars and formal concepts. Extensions retain the global concept space, validation, source provenance and default behavior of existing catalog languages.', ['dynamicLanguageExtensionExecuted', 'extensionUsesSharedAlgorithms', 'existingLanguageBehaviorPreserved']],
  ]),
  ...rows('dynamicKnowledgeRequest', dynamicSource, 'dynamic-knowledge-and-language-extension', 'grammar-inference', [
    ['I195-GRAMMAR-INFERENCE-SEPARATE-ISSUE', 'separate tracked automatic grammar inference issue',
      'A separate repository issue records automatic grammar inference and discovery with modern primary research, concrete algorithm obligations, source provenance, behavioral acceptance and links back to this critical scope. Creating the issue does not satisfy inference implementation.', ['separateInferenceIssueCreated', 'modernResearchAndAcceptanceRecorded', 'issueDoesNotSubstituteImplementation']],
  ], true),
  ...rows('dynamicKnowledgeRequest', dynamicSource, 'dynamic-knowledge-and-language-extension', 'grammar-inference', [
    ['I195-GRAMMAR-INFERENCE-AUTOMATIC-DISCOVERY', 'automatic grammar inference through research-backed algorithms',
      'Both packages discover or infer grammar definitions using recorded modern research and explicit assumptions, then validate and integrate the inferred definitions through the native pipeline and concept space with independent accepted and rejected observations.', ['automaticGrammarDiscoveryExecuted', 'researchAndAssumptionsPinned', 'inferredGrammarsIndependentlyValidated']],
  ]),
  ...rows('translationPriorityRequest', translationSource, 'critical-conversation-requirements', 'translation-priority', [
    ['I195-TRANSLATION-PRIORITY-RUST', 'immediate bidirectional Rust and meta-language translation priority',
      'Current implementation priority maximizes Rust to editable meta-language and meta-language to Rust translation, including complete source and semantic preservation obligations. Earlier languages, knowledge-format support and inherited requirements remain in scope.', ['rustToMetaLanguageExecuted', 'metaLanguageToRustExecuted', 'rustPriorityPreservesInheritedScope']],
    ['I195-TRANSLATION-PRIORITY-JAVASCRIPT', 'immediate bidirectional JavaScript and meta-language translation priority',
      'Current implementation priority maximizes JavaScript to editable meta-language and meta-language to JavaScript translation, retaining complete source, structures and semantics while preserving every earlier requirement.', ['javascriptToMetaLanguageExecuted', 'metaLanguageToJavascriptExecuted', 'javascriptPriorityPreservesInheritedScope']],
  ]),
  ...rows('multiProjectTranslationRequest', projectSource, 'critical-conversation-requirements', 'translation-priority', [
    ['I195-TRANSLATION-PRIORITY-MULTI-PROJECT', 'reusable source translation across multiple JavaScript and Rust projects',
      'The same automated translation and common concept algorithms operate on multiple real JavaScript and Rust projects with pinned inputs and independent behavioral validation. Project-specific source rewrites or carried originals do not establish successful reusable executable translation.', ['multipleRealProjectsTranslated', 'sharedTranslationAlgorithmsReused', 'projectBehaviorsIndependentlyValidated']],
  ]),
  ...rows('recursiveDefinitionsRequest', definitionSource, 'recursive-formal-concept-definitions', 'formal-concept-definitions', [
    ['I195-FORMAL-CONCEPT-DEFINITIONS', 'deep formal definition of every concept',
      'Every concept has a recursively inspectable formal definition represented as links, with explicit axioms, operations, bindings and declared foundational primitives. An English description or opaque semantic signature is insufficient as its formal definition.',
      ['everyConceptFormallyDefined', 'definitionsAreInspectableLinks', 'foundationalPrimitivesExplicit']],
    ['I195-FORMAL-CONCEPT-REFERENCES', 'mutual references between formal concept definitions',
      'Concept definitions can use other concepts recursively while retaining stable identities, qualified references and the selected foundation. Shared and cyclic references serialize and reload without flattening definitions or discarding their assumptions.',
      ['recursiveConceptReferencesPreserved', 'sharedAndCyclicDefinitionsReloaded', 'definitionAssumptionsPreserved']],
    ['I195-FORMAL-CONCEPT-MEANING-COMPARISON', 'formal definitions as evidence of equal concept meaning',
      'Meaning comparison uses the complete recursive formal definitions and checked obligations under their explicit foundations. Equal names, source text, labels or declared meaning signatures alone cannot certify semantic equivalence; unresolved obligations preserve separate concepts.',
      ['formalDefinitionsCompared', 'semanticObligationsChecked', 'unprovedMeaningEquivalenceRejected']],
  ]),
  ...rows('recursiveDefinitionsRequest', definitionSource, 'ontology-and-taxonomy-entry-points', 'ontology-and-taxonomy', [
    ['I195-ONTOLOGY-LIBRARY-ENTRYPOINT', 'separate ontology library entry point',
      'Both packages expose a separate ontology entry point for concepts, relationships and formal axioms in the global space, including concepts unused by any language grammar. Their definitions and links remain available without caller grammar registration.',
      ['ontologyEntryPointAvailable', 'grammarUnusedConceptsAvailable', 'ontologyDefinitionsAndRelationsInspectable']],
    ['I195-TAXONOMY-LIBRARY-ENTRYPOINT', 'separate taxonomy library entry point',
      'Both packages expose a separate taxonomy entry point for classification and explicitly declared subsumption links over the global concept space, including concepts unused by grammars.',
      ['taxonomyEntryPointAvailable', 'classificationAndSubsumptionInspectable', 'grammarUnusedTaxonomyConceptsAvailable']],
    ['I195-ONTOLOGY-TAXONOMY-MEMBERSHIP', 'concept membership in ontology, taxonomy or both',
      'A concept can occur in the ontology, taxonomy or both through the same global identity. Grammar reachability never removes a concept, and both entry points retain its recursive formal definition and provenance.',
      ['sharedOntologyTaxonomyIdentity', 'membershipIndependentOfGrammarReachability', 'definitionsAndProvenanceRetained']],
    ['I195-ONTOLOGY-TAXONOMY-RELATION-DISTINCTION', 'distinction between general relations and classification',
      'The ontology represents concepts, relationships and formal axioms; the taxonomy represents classification or declared subsumption. An arbitrary associative relation does not become a classification edge, and the chosen hierarchy semantics remain explicit links.',
      ['ontologyAndTaxonomyRolesDistinct', 'arbitraryRelationsDoNotImplyClassification', 'hierarchySemanticsExplicit']],
    ['I195-ONTOLOGY-MULTIPLE-LINK-ROOTS', 'multiple explicit ontology root links',
      'An ontology can have several independently selectable roots, each represented as an explicit link in the global space. A root may also be a type only when its model declares that role; root selection and traversal retain identity and provenance.',
      ['multipleOntologyRootsSelectable', 'ontologyRootsAreLinks', 'ontologyRootTypeRoleExplicit']],
    ['I195-TAXONOMY-MULTIPLE-LINK-ROOTS', 'multiple explicit taxonomy root links',
      'A taxonomy can have several explicitly represented root links and multiple classification entry points. A root type role is declared by the selected model rather than inferred from being a root, and classification traversal preserves its declared semantics.',
      ['multipleTaxonomyRootsSelectable', 'taxonomyRootsAreLinks', 'taxonomyRootTypeRoleExplicit']],
  ]),
  ...rows('linksFoundationRequest', linksSource, 'links-as-fundamental-entities-and-relationships', 'links-foundation', [
    ['I195-LINK-FOUNDATION-UNIVERSAL-REPRESENTATION', 'links as the fundamental representation of concepts, entities and relationships',
      'Every represented concept, entity, relationship, root and formal definition is fundamentally a link or composition of links in the same network. Independent host objects or hidden side tables cannot substitute for the common representation.',
      ['conceptsEntitiesRelationshipsAreLinks', 'rootsAndDefinitionsAreLinks', 'noHiddenEntityRepresentation']],
    ['I195-LINK-FOUNDATION-SELF-LOOP-VERTEX', 'vertex represented by a self-referencing link',
      'The selected links model can represent a vertex as a link whose beginning and end reference that same link. Its identity, self-reference and selected model are explicit and survive serialization and reload.',
      ['selfLoopVertexRepresentable', 'selfReferencingEndpointsPreserved', 'vertexModelAndIdentityRetained']],
    ['I195-LINK-FOUNDATION-MULTIPLE-ROLES', 'one link as relationship, concept and entity',
      'The same link can carry relationship, concept and entity roles without replacing its identity with disconnected host representations. Role declarations and multiple root selections remain inspectable links.',
      ['linkIdentitySharedAcrossRoles', 'rolesAreExplicitLinks', 'multipleRootRolesPreserved']],
    ['I195-LINK-FOUNDATION-RECURSIVE-COMPOSITION', 'recursive compositional links across representation levels',
      'Links can reference and compose other links recursively across representation levels, preserving the reusable structural motif, shared references and explicit cycles. This compositional model does not itself establish a mathematical fractal dimension or eliminate logical paradoxes.',
      ['recursiveLinkCompositionPreserved', 'sharedMotifAndCyclesExplicit', 'noUnprovedConsistencyInference']],
    ['I195-LINK-FOUNDATION-OPTIONAL-TYPE-ROOT', 'declared type roots without a universal self-typing axiom',
      'A model may declare a root link to be a type. The representation does not infer a universal Type:Type axiom or a universal proof logic from links, self-loops, recursion or root membership; foundation premises remain explicit and selectable.',
      ['rootTypeRolesDeclared', 'noImplicitUniversalSelfTyping', 'selectedFoundationPremisesPreserved']],
  ]),
  ...rows('linksFoundationRequest', linksSource, 'links-as-fundamental-entities-and-relationships', 'links-foundation', [
    ['I195-LINK-FOUNDATION-PHYSICAL-ANALOGY', 'recorded wave and fractal motivation with an explicit epistemic boundary',
      'The user wave, particle and fractal analogy is preserved verbatim as motivation and a hypothesis. Documentation and formal models do not report those physical propositions, universal self-similarity or freedom from contradictions as established facts or consistency proofs.',
      ['physicalAnalogyCapturedVerbatim', 'physicalPropositionsMarkedAsHypotheses', 'noAnalogyPresentedAsConsistencyProof']],
  ], true),
  ...rows('recordConversationRequest', recordingSource, 'critical-conversation-requirements', 'conversation-contract', [
    ['I195-CONVERSATION-VERBATIM-REGISTER', 'immutable hashed capture of every requirement-bearing conversation message',
      'Every requirement-bearing conversation message is captured verbatim with its content hash and explicit capture provenance, registered as a source and linked to atomic obligations. New messages append captures rather than overwriting earlier requests.',
      ['conversationMessagesCapturedVerbatim', 'captureHashesVerified', 'allConversationObligationsMapped']],
    ['I195-CONVERSATION-CRITICAL-DELIVERY', 'critical classification of all conversation and inherited obligations',
      'Every conversation requirement and every inherited ledger obligation is critical and blocks a full-scope delivery or release-completion claim while it lacks required exact-commit evidence. Existing source, scope, assertions, fixtures, runtimes and checkpoint execution order remain intact.',
      ['allRequirementsCritical', 'unverifiedRowsBlockFullScopeDelivery', 'inheritedScopeAndCheckpointsPreserved']],
  ], true),
  ...rows('associativeFoundationRequest', stackSource, 'associative-stack-and-theoretical-foundation', 'associative-foundation', [
    ['I195-ASSOCIATIVE-STACK-PINNED-SOURCE-AUDIT', 'pinned audit of the referenced associative technology stack',
      'The project audit reads the referenced formal-ai associative technology stack at a recorded immutable revision and records its content hash, providers and dependency-role boundaries. A moving main URL or a provider name alone does not establish adoption.',
      ['associativeStackSourceReadAndPinned', 'stackContentHashRecorded', 'providerRolesClassified']],
    ['I195-ASSOCIATIVE-STACK-PROJECT-WIDE-USE', 'associative stack adoption across the entire project',
      'An executable project-wide audit covers both packages, code, grammars, translation, tools, documentation and dependencies against the relevant associative stack obligations. Direct runtime providers, compatibility or theoretical alignment and development tools are distinguished; unrelated formal-ai dependencies are not blindly introduced.',
      ['entireProjectStackUsageAudited', 'actualCodeAndDependenciesTraced', 'runtimeCompatibilityAndDevelopmentRolesDistinct']],
    ['I195-THEORY-FOUNDATION-PINNED-SOURCE-AUDIT', 'pinned philosophical and theoretical meta-theory foundation',
      'The meta-theory source is read at an immutable revision with archived releases and drafts identified separately. Adopted definitions, assumptions and provenance are recorded explicitly; a repository citation alone is insufficient as a theoretical foundation.',
      ['metaTheorySourceReadAndPinned', 'archivesAndDraftsDistinguished', 'adoptedDefinitionsAndAssumptionsTraced']],
  ], true),
  ...rows('associativeFoundationRequest', stackSource, 'associative-stack-and-theoretical-foundation', 'associative-foundation', [
    ['I195-THEORY-FOUNDATION-PROJECT-WIDE-APPLICATION', 'meta-theory foundation applied to the common links model',
      'The common concept, grammar, ontology, taxonomy, entity and relationship representation applies the adopted links meta-theory definitions throughout both packages. Model choices, theoretical premises and operational encodings remain explicit inspectable links with source provenance.',
      ['metaTheoryDefinitionsAppliedProjectWide', 'modelPremisesAreInspectableLinks', 'operationalEncodingsRetainProvenance']],
    ['I195-THEORY-FOUNDATION-CONSUMER-BOUNDARIES', 'theoretical foundation separated from consumer proof authority',
      'The shared philosophical and theoretical links foundation does not silently choose a universal proof logic, Type:Type axiom or consumer execution policy. Consumer-owned logical semantics and proof authority remain separate explicit model choices with checked correspondences.',
      ['theoreticalAndConsumerFoundationsDistinct', 'noSilentUniversalLogicOrSelfTyping', 'consumerProofAuthorityPreserved']],
  ]),
  ...rows('globalConceptSpace', originalSource, 'critical-conversation-requirements', 'workflow-contract', [
    ['I195-WORKFLOW-FAST-CONTINUOUS-PROGRESS', 'fast continuous implementation between bulk feedback cycles',
      'Deliver the complete authorized scope as quickly as available resources and correct verification permit, using useful parallel work and coherent batches. Continue independent drafting and implementation while CI runs instead of waiting idle; speed never substitutes for required behavior or evidence.', ['independentWorkContinuesDuringCi', 'deliveryLatencyMinimizedWithoutScopeReduction']],
    ['I195-WORKFLOW-LATEST-REPOSITORY', 'latest repository version in the shared local checkout',
      'Before drafting the batch, the coordinator obtains the latest authorized repository and pull-request refs, reconciles required main changes and records the checkout revision used for implementation and CI feedback.', ['latestAuthorizedRefsObtained', 'checkoutRevisionRecorded']],
    ['I195-WORKFLOW-BULK-EXACT-SCOPE', 'one bulk draft of exactly all issue, pull-request and conversation obligations',
      'Bulk work covers every registered issue, pull-request and conversation obligation, preserving all inherited scope. Drafting, implementation and reporting do not substitute narrower capabilities or omit unresolved requirements.', ['entireRegisteredScopeIncluded', 'noScopeSubstitutionOrOmission']],
    ['I195-WORKFLOW-MINIMAL-LOCAL-RESOURCES', 'minimal local resources and targeted JavaScript validation',
      'Local work uses the minimum resources needed for targeted JavaScript validation and authoring. Full suites, whole-registry experiments, downstream workload clones and native toolchain matrices remain CI work.', ['targetedJavaScriptOnlyLocally', 'expensiveChecksDelegatedToCI']],
    ['I195-WORKFLOW-PARALLEL-DELEGATION', 'maximum useful independent agent delegation',
      'Independent bulk tasks are split among as many useful authorized agents as the available concurrency permits, with clear ownership and integration coordination over the shared checkout.', ['independentTasksDelegated', 'availableConcurrencyUsedWithOwnership']],
    ['I195-WORKFLOW-CONTRIBUTOR-COMMITS', 'every contributor commits its reviewable work',
      'Every contributor with implementation work commits its explicit owned paths, preserving other agents staged changes and retaining reviewable commit provenance for the bulk batch.', ['everyContributorWorkCommitted', 'ownedPathsAndCommitProvenanceRetained']],
    ['I195-WORKFLOW-SINGLE-PUSH-COORDINATOR', 'one coordinator performs batch pushes',
      'Exactly one authorized coordinator pushes the integrated batch after reviewing all committed work. Other contributors commit their changes without pushing.', ['singlePushCoordinatorEnforced', 'integratedBatchReviewedBeforePush']],
    ['I195-WORKFLOW-PREVIOUS-CI-TERMINAL', 'previous CI fully finished before a new batch push',
      'Before pushing the next integrated batch, the coordinator verifies the prior relevant CI/CD runs are fully finished and inspects their complete failure feedback. The batch has new concrete requirements or fixes to deliver.', ['previousCiRunsTerminal', 'completeFeedbackInspected', 'newConcreteBatchChangesPresent']],
    ['I195-WORKFLOW-PREVIOUS-CI-ERRORS-ADDRESSED', 'all previous CI errors addressed before the feedback push',
      'Every prior CI/CD error is addressed with concrete implementation and the applicable targeted verification before the integrated feedback push. No failing check is described as intended and no acceptance gate is weakened.', ['allPreviousCiErrorsAddressed', 'targetedFixVerificationRecorded', 'gatesRemainIntact']],
    ['I195-WORKFLOW-NO-LOCAL-RUST-COMPILATION', 'no local Rust compilation during the requested workflow',
      'No contributor compiles Rust locally for testing in this workflow. JavaScript validation develops the system behavior, and compiled Rust parity, platform and native execution checks run in CI.', ['noRustCompiledLocally', 'rustExecutionChecksRunInCI']],
    ['I195-WORKFLOW-AUTOMATED-TRANSLATION', 'JavaScript-first automated translation and referenced best practices',
      'Bulk work develops JavaScript behavior first, ports through automated translation where supported and applies the relevant pinned practices from Router PR761, formal-ai PR1188 and Relative Meta Logic PR184. Unsupported translation constructs remain implementation gaps rather than hand-written substitutes or success declarations.', ['javascriptFirstBulkDevelopment', 'automatedTranslationUsed', 'referencedPracticesAudited', 'unsupportedTranslationGapsRetained']],
  ], true),
  ...rows("recursiveConcreteSyntaxRequest", "parity/sources/recursive-concrete-syntax-and-any-grammar-2026-10-16.txt", "concrete-and-abstract-syntax-trees", "recursive-concrete-syntax", [
  [
    "I195-CST-DEFAULT-EVERY-LANGUAGE",
    "full concrete syntax as the default grammar output for every supported language",
    "Every supported language grammar generates the complete lossless concrete syntax tree by default, including tokens, trivia, fields, hierarchy, order, spans, recovery information and embedded boundaries. A token list, retained source envelope or abstract projection cannot substitute for that default.",
    [
      "everySupportedLanguageDefaultsToCompleteConcreteSyntax",
      "allConcreteStructureAndSourceBytesPreserved"
    ]
  ],
  [
    "I195-CST-EXPLICIT-AST-SELECTION",
    "optional abstract syntax with concrete syntax remaining the default",
    "Both runtime packages expose an explicit optional switch or projection from concrete to abstract syntax for supported languages. Abstract output retains inspectable provenance to the complete concrete tree, and omitting the option always selects concrete syntax.",
    [
      "abstractSyntaxRequiresExplicitSelection",
      "concreteSyntaxRemainsDefault",
      "abstractProjectionRetainsConcreteProvenance"
    ]
  ],
  [
    "I195-CST-EMBEDDED-GLOBAL-CONCEPT-SPACE",
    "one global concept space across all recursively embedded grammars",
    "All host and embedded grammars use the same global concept space at every nesting level. Repeated or same-language embeddings preserve stable concept identities, source-qualified grammar references and the existing requirement that deduplication requires a proved one-to-one correspondence.",
    [
      "hostAndEmbeddedGrammarsShareGlobalConceptSpace",
      "recursiveConceptIdentityAndProvenancePreserved",
      "unprovedMeaningsRemainDistinct"
    ]
  ],
  [
    "I195-CST-ANY-GRAMMAR-RECURSIVE-RULE",
    "generic any grammar or all grammar selection rule",
    "Provide an inspectable linked any-grammar or all-grammar rule that can select and invoke supported grammar definitions recursively through the common algorithms. Selection, embedded boundaries, ambiguities and failures remain explicit; adding a supported grammar does not require special cases for every possible host and child language pair.",
    [
      "genericGrammarSelectionRepresentedAsLinks",
      "supportedGrammarsRecursivelyInvoked",
      "selectionAmbiguityAndFailuresExplicit"
    ]
  ],
  [
    "I195-CST-DEEPLY-RECURSIVE-MIXED-LANGUAGES",
    "deep same-language and mixed-language recursive concrete parsing",
    "Both packages support deeply recursive parsing, including Markdown within Markdown and Markdown containing JavaScript containing JSON whose content contains Markdown and Python, as well as mixed logs and documents. Recursion follows linked embedding definitions and can return to an earlier language; it is not restricted to one nesting level or to Markdown and HTML hosts. Non-consuming cycles and resource exhaustion report diagnostics rather than silently omitting nested trees.",
    [
      "sameLanguageRecursionSupported",
      "markdownJavaScriptJsonMarkdownPythonNestingSupported",
      "mixedLogsAndDocumentsSupported",
      "cyclesAndResourceLimitsReportedWithoutSilentTruncation"
    ]
  ],
  [
    "I195-CST-RECURSIVE-BOUNDARIES-LOSSLESS",
    "lossless recursive boundaries and nested translation provenance",
    "Deeply embedded concrete trees preserve their full enclosing hierarchy, exact original source bytes, delimiters, escapes, trivia and mappings to host spans. Serialization, reload and translation retain those boundaries and provenance so nested edits and translations do not discard or reinterpret surrounding content.",
    [
      "completeEmbeddedHierarchyAndBoundariesPreserved",
      "nestedEscapesAndHostSpansRemainExact",
      "nestedSerializationReloadAndTranslationPreserveSource"
    ]
  ]
]),
  ...rows("ownLanguageGrammarPriorityRequest", "parity/sources/own-language-complete-grammar-priority-2026-10-16.txt", "language-catalog-and-default-parsing", "own-language-grammar-priority", [
  [
    "I195-GRAMMAR-PRIORITY-JAVASCRIPT-COMPLETE",
    "complete JavaScript grammar as an initial delivery priority",
    "Prioritize a complete JavaScript grammar that produces full lossless concrete syntax by default and supports the language constructs used by this project and its own real source files, with independent accepted and rejected observations. Grammar registration, a partial syntax subset or opaque carried source does not establish complete grammar support; all inherited catalog obligations remain.",
    [
      "javascriptGrammarPrioritized",
      "javascriptCompleteConcreteSyntaxValidated",
      "javascriptOwnSourceFilesExercised"
    ]
  ],
  [
    "I195-GRAMMAR-PRIORITY-MARKDOWN-COMPLETE",
    "complete Markdown grammar as an initial delivery priority",
    "Prioritize a complete Markdown grammar that produces full lossless concrete syntax by default and supports the language constructs used by this project and its own real source files, with independent accepted and rejected observations. Grammar registration, a partial syntax subset or opaque carried source does not establish complete grammar support; all inherited catalog obligations remain.",
    [
      "markdownGrammarPrioritized",
      "markdownCompleteConcreteSyntaxValidated",
      "markdownOwnSourceFilesExercised"
    ]
  ],
  [
    "I195-GRAMMAR-PRIORITY-RUST-COMPLETE",
    "complete Rust grammar as an initial delivery priority",
    "Prioritize a complete Rust grammar that produces full lossless concrete syntax by default and supports the language constructs used by this project and its own real source files, with independent accepted and rejected observations. Grammar registration, a partial syntax subset or opaque carried source does not establish complete grammar support; all inherited catalog obligations remain.",
    [
      "rustGrammarPrioritized",
      "rustCompleteConcreteSyntaxValidated",
      "rustOwnSourceFilesExercised"
    ]
  ],
  [
    "I195-GRAMMAR-PRIORITY-TYPESCRIPT-COMPLETE",
    "complete TypeScript grammar as an initial delivery priority",
    "Prioritize a complete TypeScript grammar that produces full lossless concrete syntax by default and supports the language constructs used by this project and its own real source files, with independent accepted and rejected observations. Grammar registration, a partial syntax subset or opaque carried source does not establish complete grammar support; all inherited catalog obligations remain.",
    [
      "typescriptGrammarPrioritized",
      "typescriptCompleteConcreteSyntaxValidated",
      "typescriptOwnSourceFilesExercised"
    ]
  ],
  [
    "I195-GRAMMAR-PRIORITY-OWN-SOURCE-TRANSLATION",
    "complete project language grammars for translation of our own code",
    "Deliver full grammars first for JavaScript, Markdown, Rust, TypeScript and the other languages used by these projects to enable the best available reusable tools and automated translation of their own code through the common editable links representation. This priority preserves the full scope for other languages and does not equate a concrete parse or carried original with executable semantic translation.",
    [
      "ownProjectLanguagesReceiveInitialGrammarPriority",
      "ownCodeUsesCommonEditableTranslationRepresentation",
      "otherLanguageAndSemanticObligationsRetained"
    ]
  ]
]),
  ...rows("routerPracticeRecordingRequest", "parity/sources/router-practice-reuse-and-current-recording-2026-10-16.txt", "javascript-first-and-self-translation", "conversation-practice-reuse", [
  [
    "I195-PRACTICE-ROUTER-761-REUSE",
    "review and reuse applicable best practices from Router pull request 761",
    "Review Router PR761 at a recorded revision and reuse as many applicable ideas as possible for complete original-source inventories, checked semantic links, serialization and reparsing before emission, deterministic regeneration, independent body and proof review, explicit unsupported gaps, same-commit JavaScript-first CI and bounded resources. Record applicability and actual validation; do not blindly import project-specific formats, versions or workflow policies that conflict with this project.",
    [
      "routerPracticeRevisionAndSourcesRecorded",
      "applicablePracticesReusedWithTraceability",
      "unsupportedAndUnverifiedWorkRemainsExplicit",
      "incompatibleProjectPoliciesNotBlindlyCopied"
    ]
  ],
  [
    "I195-CONVERSATION-CURRENT-VISION-REQUIREMENTS",
    "complete recording of current requests in the authoritative vision and atomic requirements",
    "Every requirement-bearing request in the conversation through this instruction is preserved verbatim with its capture provenance and hash, reflected in docs/vision.md and mapped to critical atomic ledger obligations. This includes default full concrete syntax, optional abstract syntax, recursive any-grammar embedding in the global concept space, initial complete JavaScript Markdown Rust and TypeScript grammars, and Router best-practice reuse. All earlier obligations and source bytes remain intact.",
    [
      "allCurrentMessagesCapturedVerbatimAndHashed",
      "currentRequestsReflectedInVisionAndAtomicLedger",
      "earlierSourcesRequirementsAndScopesPreserved"
    ]
  ]
], true),
  ...ASSERTION_POLICY_CONVERSATION_REQUIREMENTS,
]);
