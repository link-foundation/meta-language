// Atomic requirements of the repository-wide delivery directive
// https://github.com/link-foundation/meta-language/pull/196#issuecomment-5885245090,
// specified in docs/vision.md.
//
// Each row pins its fixture to the section of docs/vision.md (or the inventory)
// that states the obligation. A runtime whose implementation does not exist yet
// has a null entry point, and its cells stay failing until tests executed on
// the exact commit record every assertion; nothing here is marked complete.

export const VISION_SPECIFICATION = 'docs/vision.md';
export const SOURCE_REGISTER = 'parity/issue-195-sources.json';
export const LINO_COMPATIBILITY_MATRIX = 'parity/fixtures/lino-compatibility-matrix.json';
export const LANGUAGE_INVENTORY = 'parity/language-grammar-inventory.json';
export const GRAMMAR_IMPORTER_CORPUS = 'parity/fixtures/grammar-importers.json';
export const FOUR_LANGUAGE_CORPUS = 'parity/fixtures/four-language-conformance.json';

const REPOSITORY_TOOLING = {
  language: 'repository tooling (Node.js scripts, documentation and CI workflows)',
  version: 'Node.js 22 or later',
  edition: 'every checkout and CI job of this repository',
};
const BOTH_PACKAGES = {
  language: 'meta-language runtime packages',
  version: 'the npm package and the Rust crate of the same release',
  edition: 'every catalog language, grammar format and edition',
};

// Entry points of the capabilities that already exist in some form; a runtime
// missing here has none yet.
const JAVASCRIPT_LINO = ['js/src/lino-grammar.js', 'js/src/lino-semantics.js', 'js/package.json'];
const RUST_LINO = ['rust/src/lino_grammar.rs', 'rust/src/lino_parser.rs', 'rust/Cargo.toml'];

export const VISION_REQUIREMENTS = Object.freeze([
  {
    id: 'I195-VISION-SPECIFICATION',
    area: 'vision-and-traceability',
    specification: 'sources-of-truth',
    construct: 'one authoritative vision and architecture specification',
    expectedBehavior:
      'docs/vision.md states the whole target of the repository-wide directive, section by section. The README, the contributor instructions and the agent instructions link to it, and the subordinate grammar and parity documents name it as the contract instead of competing with it.',
    assertions: [
      'specificationCommitted',
      'everyDirectiveSectionSpecified',
      'linkedFromReadme',
      'linkedFromContributorInstructions',
      'linkedFromAgentInstructions',
      'subordinateDocumentsDeferToSpecification',
    ],
    tooling: ['docs/vision.md', 'README.md', 'CONTRIBUTING.md', 'AGENTS.md'],
  },
  {
    id: 'I195-VISION-SOURCE-REGISTER',
    area: 'vision-and-traceability',
    specification: 'sources-of-truth',
    fixture: SOURCE_REGISTER,
    construct: 'register of every requirement source with its revision and content hash',
    expectedBehavior:
      'parity/issue-195-sources.json registers the issue and every requirement-bearing comment with its revision time, byte count and sha256 content hash. The check fails when a registered source was edited or deleted, or when a comment that is not automation output is unregistered. It also fails when a ledger row cites an unregistered source or is not covered by its source.',
    assertions: [
      'everySourceRegisteredWithHash',
      'editedSourceDetected',
      'deletedSourceDetected',
      'unregisteredCommentDetected',
      'automationCommentsIgnored',
      'everyRowSourceRegistered',
      'everySourceCoversLedgerRows',
    ],
    tooling: ['js/scripts/issue-195-sources.mjs', 'js/scripts/check-issue-195-sources.mjs', SOURCE_REGISTER],
  },
  {
    id: 'I195-VISION-REQUIREMENT-TRACEABILITY',
    area: 'vision-and-traceability',
    specification: 'sources-of-truth',
    construct: 'traceability from each atomic requirement to its evidence chain',
    expectedBehavior:
      'Every ledger row carries a traceability record. The record links the row to its section of docs/vision.md, its feature inventory, every registered source that covers it, the CI workflow and evidence group that evaluate it, the packages that ship it and the downstream evidence it needs, and every link resolves to an existing file or anchor.',
    assertions: [
      'everyRowHasTraceability',
      'specificationAnchorsResolve',
      'inventoriesExist',
      'coveringSourcesRegistered',
      'workflowsAndEvidenceGroupsResolve',
      'packagesMatchRuntimes',
      'downstreamEvidenceLinked',
    ],
    tooling: ['js/scripts/issue-195-vision-requirements.mjs', 'js/scripts/issue-195-requirements.mjs'],
  },
  {
    id: 'I195-VISION-CONTRADICTION-AUDIT',
    area: 'vision-and-traceability',
    specification: 'documentation-consistency',
    fixture: 'parity/documentation/claim-fixtures.json',
    construct: 'repository audit for statements that contradict the specification',
    expectedBehavior:
      'A check reads every documentation file, package README, changelog fragment and case study. It fails when one of them presents future emitters, approximate round trips, external production parsers or an incomplete scope as the finished contract, and it fails when a case study lacks the historical label.',
    assertions: [
      'everyDocumentAudited',
      'futureWorkNotPresentedAsDelivered',
      'caseStudiesLabeledHistorical',
      'contradictionFixturesRejected',
    ],
    tooling: ['js/scripts/issue-195-documentation.mjs', 'js/scripts/check-documentation.mjs'],
  },
  {
    id: 'I195-VISION-FOUNDATION-NEUTRAL',
    area: 'vision-and-traceability',
    specification: 'foundation-neutrality',
    fixture: 'parity/foundation-models.json',
    construct: 'foundation-neutral shared concepts',
    expectedBehavior:
      'The shared concepts represent integer and overflow models, universes, effects and proof systems as distinct data with explicit correspondences. No RML-specific syntax, foundation or proof authority is built into meta-language, and a check rejects a hard-coded universal logic.',
    assertions: [
      'foundationsRepresentedAsData',
      'noRelativeMetaLogicSpecificSyntax',
      'distinctModelsKeptDistinct',
      'hardCodedFoundationRejected',
    ],
    javascript: ['js/src/foundation-models.js', 'js/scripts/build-foundation-models.mjs'],
    rust: ['rust/src/foundation_models.rs'],
  },
  {
    id: 'I195-DEPENDENCY-INVENTORY',
    area: 'dependency-freshness',
    specification: 'dependencies',
    fixture: 'parity/dependency-inventory.json',
    construct: 'inventory of every dependency, toolchain, action and image at its current stable release',
    expectedBehavior:
      'docs/dependency-audit.md, dated, lists every runtime, development, build and optional dependency, lockfile resolution, vendored grammar asset, generator, toolchain, workflow action and pinned build image with its current stable release. A check fails when a retained item is behind that release without a recorded compatibility reason.',
    assertions: [
      'everyDependencyInventoried',
      'auditDateRecorded',
      'retainedItemsAtCurrentStableRelease',
      'staleDependencyRejected',
    ],
    tooling: ['js/scripts/dependency-inventory.mjs', 'js/scripts/check-dependencies.mjs'],
  },
  {
    id: 'I195-DEPENDENCY-PRODUCTION-PARSERS-REMOVED',
    area: 'dependency-freshness',
    specification: 'dependencies',
    construct: 'external parser engines removed from the production packages',
    expectedBehavior:
      'Neither published package declares tree-sitter, web-tree-sitter, peggy or another external grammar engine as a production dependency or ships vendored grammar binaries. Independent oracles remain only as test and development dependencies.',
    assertions: [
      'noExternalParserProductionDependency',
      'noVendoredGrammarBinariesShipped',
      'oraclesOnlyInTestScope',
    ],
    tooling: null,
  },
  {
    id: 'I195-LINO-UPGRADE',
    area: 'links-notation-fitness',
    specification: 'links-notation-fitness',
    fixture: LINO_COMPATIBILITY_MATRIX,
    construct: 'current links-notation release in both runtimes',
    expectedBehavior:
      'Both packages depend on the links-notation release recorded in the compatibility matrix, which was verified separately on npm and crates.io. The installed package reports that version, and the version-coupled grammar fixtures were regenerated against it.',
    assertions: ['declaredReleaseMatchesMatrix', 'installedReleaseMatchesMatrix', 'fixturesRegeneratedAgainstRelease'],
    javascript: JAVASCRIPT_LINO,
    rust: RUST_LINO,
  },
  {
    id: 'I195-LINO-COMPATIBILITY-MATRIX',
    area: 'links-notation-fitness',
    specification: 'links-notation-fitness',
    fixture: LINO_COMPATIBILITY_MATRIX,
    construct: 'feature-by-feature Links Notation compatibility matrix',
    expectedBehavior:
      'For every feature in the compatibility matrix (named and anonymous links, arbitrary arity, shared, recursive and forward references, identity, ordering, indentation, nested multiline groups, quoting, escaping, comments and trivia, Unicode and source mappings), both runtimes encode, decode, edit and reconstruct the feature exactly through links-notation plus the meta-language representation layer.',
    assertions: ['everyFeatureDecoded', 'everyFeatureReconstructedExactly', 'everyFeatureEditable', 'everyFeatureEncoded'],
    javascript: JAVASCRIPT_LINO,
    rust: RUST_LINO,
  },
  {
    id: 'I195-LINO-UPSTREAM-REGRESSIONS',
    area: 'links-notation-fitness',
    specification: 'links-notation-fitness',
    fixture: LINO_COMPATIBILITY_MATRIX,
    construct: 'relative-meta-logic upstream findings as regression tests',
    expectedBehavior:
      'The Links Notation findings of relative-meta-logic (whitespace and runtime differences, indented identifiers, deep nesting, long quote runs, stack safety and nonlinear parsing) are regression inputs in both runtimes. Every input parses losslessly in linear time, and nesting beyond the official 64-level limit becomes an ERROR node, not a stack overflow.',
    assertions: ['everyRegressionInputLossless', 'everyRegressionInputLinear', 'excessiveNestingIsErrorNode'],
    javascript: ['js/src/lino-grammar.js', 'js/src/lino-semantics.js'],
    rust: ['rust/src/lino_grammar.rs', 'rust/src/lino_parser.rs'],
  },
  {
    id: 'I195-GRAMMAR-LANGUAGE-CATALOG',
    area: 'native-grammar',
    specification: 'language-catalog-and-default-parsing',
    fixture: LANGUAGE_INVENTORY,
    construct: 'versioned language catalog with the strongest grammar sources per language',
    expectedBehavior:
      'The versioned catalog lists every popular language with its versions or editions, dialects, aliases, extensions and embedded languages. For each language it inventories the official, ANTLR grammars-v4, tree-sitter and other mature grammar sources with pinned revisions, licenses, provenance and independent test corpora, reconciled with every dispatcher and downstream requirement.',
    assertions: [
      'everyLanguageVersioned',
      'everyApplicableGrammarSourceInventoried',
      'sourcesPinnedWithLicenseAndProvenance',
      'independentCorporaPinned',
      'reconciledWithDispatchersAndConsumers',
    ],
    javascript: null,
    rust: null,
  },
  {
    id: 'I195-GRAMMAR-NATIVE-MERGED',
    area: 'native-grammar',
    specification: 'native-merged-grammars',
    fixture: LANGUAGE_INVENTORY,
    construct: 'native Links Notation grammar as the shipped canonical grammar of every language',
    expectedBehavior:
      'The shipped grammar of every catalog language is a native Links Notation grammar merged from all of its inventoried sources. Default parsing executes it, not a wrapped external engine, a copied generated parser or a stored foreign grammar string.',
    assertions: [
      'everyLanguageHasNativeGrammar',
      'nativeGrammarMergedFromAllSources',
      'defaultParseExecutesNativeGrammar',
      'noForeignGrammarStoredAsString',
    ],
    javascript: null,
    rust: null,
  },
  {
    id: 'I195-GRAMMAR-FEATURE-UNION',
    area: 'native-grammar',
    specification: 'grammar-feature-union',
    construct: 'grammar representation covering the union of all source grammar features',
    expectedBehavior:
      'The native grammar representation and its executor cover every feature listed in the grammar feature union, from ordered and unordered alternatives, left recursion and precedence to lexer modes, external scanners, semantic actions, embedded languages and error recovery. External scanners and actions are executable link definitions, and each feature has positive and negative executable tests.',
    assertions: [
      'everyUnionFeatureRepresented',
      'everyUnionFeatureExecuted',
      'scannersAndActionsExecutableAsLinks',
      'negativeCasesRejected',
    ],
    javascript: null,
    rust: null,
  },
  {
    id: 'I195-GRAMMAR-CONCEPT-DISTINCTIONS',
    area: 'native-grammar',
    specification: 'grammar-feature-union',
    construct: 'preserved semantic distinctions between grammar concepts',
    expectedBehavior:
      'Concepts are shared only under a justified one-to-one correspondence of meaning. Ordered and unordered choice, lexical and syntactic precedence, integer and overflow models, binding rules, effects and proof universes stay distinct, and fixtures with the same spelling but different meanings are not merged.',
    assertions: ['sharedOnlyWithJustifiedCorrespondence', 'requiredDistinctionsPreserved', 'lookalikeConceptsKeptDistinct'],
    javascript: null,
    rust: null,
  },
  {
    id: 'I195-GRAMMAR-LOSSLESS-TREES',
    area: 'native-grammar',
    specification: 'concrete-and-abstract-syntax-trees',
    fixture: LANGUAGE_INVENTORY,
    construct: 'full lossless concrete syntax trees from native grammars, with optional abstract trees',
    expectedBehavior:
      'Native grammar parsing of every catalog language yields a lossless concrete syntax tree with node kinds, fields, hierarchy, exact tokens, trivia, spans, error and missing nodes and embedded-language boundaries. The optional abstract tree keeps provenance to the concrete tree, and both trees can be serialized, reloaded without the source, edited, generated back to source and reparsed.',
    assertions: [
      'nativeTreesLossless',
      'fieldsAndSpansExact',
      'errorAndMissingNodesRecovered',
      'abstractTreeProvenanceKept',
      'treesReloadedWithoutSource',
      'editedTreesGeneratedAndReparsed',
    ],
    javascript: null,
    rust: null,
  },
  {
    id: 'I195-GRAMMAR-DEPENDENCY-BOUNDARY',
    area: 'native-grammar',
    specification: 'native-merged-grammars',
    construct: 'native dependency boundary of the installed packages',
    expectedBehavior:
      'The installed production packages parse every catalog language with every external grammar engine, subprocess, foreign generated parser, download and oracle asset absent. Changing one native linked grammar rule changes the output of the public parse path without a host-code patch.',
    assertions: [
      'parsesWithExternalEnginesAbsent',
      'noSubprocessOrDownloadAtParseTime',
      'nativeRuleChangeChangesParse',
      'generatedAccelerationDerivedFromNativeGrammar',
    ],
    javascript: null,
    rust: null,
  },
  {
    id: 'I195-INTERCHANGE-API-CLI',
    area: 'grammar-interchange',
    specification: 'grammar-import-conversion-and-reverse-conversion',
    fixture: GRAMMAR_IMPORTER_CORPUS,
    construct: 'public API and command-line tools for grammar interchange',
    expectedBehavior:
      'Both packages expose public APIs and command-line tools that import, validate, convert, merge, rename, export and round-trip grammars in every catalog format, with the same observable results in both runtimes.',
    assertions: ['importCommand', 'validateCommand', 'convertCommand', 'mergeCommand', 'renameCommand', 'exportCommand', 'roundTripCommand'],
    javascript: null,
    rust: null,
  },
  {
    id: 'I195-INTERCHANGE-FORMAT-FEATURES',
    area: 'grammar-interchange',
    specification: 'grammar-import-conversion-and-reverse-conversion',
    fixture: GRAMMAR_IMPORTER_CORPUS,
    construct: 'every feature of every grammar format imported with its supporting definitions',
    expectedBehavior:
      'Every feature of BNF, EBNF, ABNF, PEG, Peggy, Pest, ANTLR, tree-sitter source and JSON, Lark, GBNF, Yacc/Bison, lexer specifications and the other reconciled formats has an explicit native representation and operational meaning. Imports follow included grammars, lexer and scanner definitions and action code, and keep comments, locations, original names and formatting provenance.',
    assertions: [
      'everyFormatFeatureRepresented',
      'includedGrammarsFollowed',
      'lexerScannerAndActionsImported',
      'provenancePreserved',
    ],
    javascript: null,
    rust: null,
  },
  {
    id: 'I195-INTERCHANGE-REVERSE-CONVERSION',
    area: 'grammar-interchange',
    specification: 'grammar-import-conversion-and-reverse-conversion',
    fixture: GRAMMAR_IMPORTER_CORPUS,
    construct: 'conversion back to every source grammar format',
    expectedBehavior:
      'For every format, source grammar -> native links -> exported grammar -> native links is structurally and semantically equivalent with the original source unavailable to the emitter, and the lossless mode reconstructs the source exactly.',
    assertions: ['exportWithoutOriginalSource', 'reimportEquivalent', 'losslessModeExact'],
    javascript: null,
    rust: null,
  },
  {
    id: 'I195-INTERCHANGE-CROSS-FORMAT-TOOLS',
    area: 'grammar-interchange',
    specification: 'grammar-import-conversion-and-reverse-conversion',
    fixture: GRAMMAR_IMPORTER_CORPUS,
    construct: 'cross-format conversion checked by the real target tools',
    expectedBehavior:
      'Cross-format conversions and native -> external -> native cycles run under the actual independent target tools. The tools compare accepted and rejected programs, parse structure, lexical decisions, precedence, actions and diagnostics with the source grammar.',
    assertions: ['crossFormatCyclesTested', 'targetToolsExecuted', 'acceptanceAndStructureCompared', 'diagnosticsCompared'],
    javascript: null,
    rust: null,
  },
  {
    id: 'I195-INTERCHANGE-FAITHFUL-LOWERING',
    area: 'grammar-interchange',
    specification: 'grammar-import-conversion-and-reverse-conversion',
    fixture: GRAMMAR_IMPORTER_CORPUS,
    construct: 'faithful lowering into less expressive grammar notations',
    expectedBehavior:
      'Exporting to a less expressive notation produces a faithful lowering or runtime encoding with explicit reconstruction metadata. An executable target grammar is distinguished from a lossless interchange package, and no feature is silently dropped or hidden in an opaque sidecar.',
    assertions: ['loweringExecutable', 'reconstructionMetadataExplicit', 'noSilentlyDroppedFeature'],
    javascript: null,
    rust: null,
  },
  {
    id: 'I195-INTERCHANGE-MUTATION-GUARDS',
    area: 'grammar-interchange',
    specification: 'grammar-import-conversion-and-reverse-conversion',
    fixture: GRAMMAR_IMPORTER_CORPUS,
    construct: 'round trips that cannot pass by returning the saved input',
    expectedBehavior:
      'Round-trip tests change the grammar before export and require the change in the exported grammar. Malformed and negative cases are rejected, so two mutually wrong importers and exporters cannot validate each other.',
    assertions: ['mutationVisibleAfterRoundTrip', 'malformedGrammarsRejected', 'mutuallyWrongPairDetected'],
    javascript: null,
    rust: null,
  },
  {
    id: 'I195-MERGE-AUTOMATIC-PIPELINE',
    area: 'automatic-merge',
    specification: 'automatic-merging-concept-recognition-deduplication-and-renaming',
    fixture: LANGUAGE_INVENTORY,
    construct: 'non-interactive multi-source grammar merge pipeline',
    expectedBehavior:
      'The default pipeline discovers and imports the inventoried sources, recognizes corresponding concepts under different names, reconciles and normalizes them, deduplicates equivalent concepts, preserves unique ones, and emits the canonical native grammar with a reproducible provenance and decision report, without user interaction.',
    assertions: [
      'sourcesDiscoveredAndImported',
      'correspondingConceptsRecognized',
      'uniqueConceptsPreserved',
      'decisionReportReproducible',
      'noInteractionRequired',
    ],
    javascript: null,
    rust: null,
  },
  {
    id: 'I195-MERGE-BINDING-AWARE-RENAME',
    area: 'automatic-merge',
    specification: 'automatic-merging-concept-recognition-deduplication-and-renaming',
    construct: 'binding-aware grammar renaming',
    expectedBehavior:
      'Renaming runs through references, imports, captures, semantic actions, mappings and generated artifacts, keeps stable identities and source aliases for export, and is correct for collisions, shadowing, recursive and mutually recursive rules, qualified references and references after serialization and reload.',
    assertions: [
      'collisionsHandled',
      'shadowingHandled',
      'recursiveRulesRenamed',
      'qualifiedReferencesRenamed',
      'referencesAfterReloadRenamed',
      'sourceAliasesKept',
    ],
    javascript: null,
    rust: null,
  },
  {
    id: 'I195-MERGE-MEANING-AWARE-DEDUPLICATION',
    area: 'automatic-merge',
    specification: 'automatic-merging-concept-recognition-deduplication-and-renaming',
    construct: 'deduplication by meaning',
    expectedBehavior:
      'Duplicates are recognized by meaning-aware normalization, recursive graph and alpha-equivalence, lexical and parser semantics and justified mappings. Name similarity and generated samples only nominate candidates, differently named equivalent concepts are merged, and identically named non-equivalent concepts are not.',
    assertions: ['equivalentConceptsMerged', 'homonymsKeptDistinct', 'samplesOnlyNominate', 'equivalenceJustified'],
    javascript: null,
    rust: null,
  },
  {
    id: 'I195-MERGE-UNCERTAINTY-PRESERVED',
    area: 'automatic-merge',
    specification: 'automatic-merging-concept-recognition-deduplication-and-renaming',
    construct: 'uncertain matches kept explicit',
    expectedBehavior:
      'Uncertain matches, distinct meanings and version, dialect or ambiguity alternatives stay explicit instead of being conflated. An unresolved required equivalence is reported as failing work.',
    assertions: ['uncertainMatchesNotConflated', 'alternativesExplicit', 'unresolvedEquivalenceFails'],
    javascript: null,
    rust: null,
  },
  {
    id: 'I195-MERGE-SHIPPED-GRAMMARS-ARE-MERGED',
    area: 'automatic-merge',
    specification: 'automatic-merging-concept-recognition-deduplication-and-renaming',
    fixture: LANGUAGE_INVENTORY,
    construct: 'shipped and test grammars produced by the multi-source merge',
    expectedBehavior:
      'The production grammars and the native grammars used in tests are outputs of the multi-source merge. Each compatible source keeps its unique valid coverage, dialects are not united indiscriminately, and upstream oracles stay independent of the merged output.',
    assertions: ['shippedGrammarsAreMergeOutputs', 'testGrammarsAreMergeOutputs', 'uniqueSourceCoverageKept', 'oraclesIndependent'],
    javascript: null,
    rust: null,
  },
  {
    id: 'I195-MERGE-DETERMINISM',
    area: 'automatic-merge',
    specification: 'automatic-merging-concept-recognition-deduplication-and-renaming',
    construct: 'deterministic, idempotent and incremental merge',
    expectedBehavior:
      'Merging is reproducible, idempotent and stable under input reordering. It re-merges incrementally after upstream changes and resolves conflicts by recorded precedence rules. It never creates duplicate canonical identities for established equivalences, and it preserves language and edition boundaries.',
    assertions: [
      'reproducibleOutput',
      'idempotent',
      'stableUnderReordering',
      'incrementalRemerge',
      'noDuplicateCanonicalIdentities',
      'editionBoundariesPreserved',
    ],
    javascript: null,
    rust: null,
  },
  {
    id: 'I195-MERGE-QUALITY-EVIDENCE',
    area: 'automatic-merge',
    specification: 'automatic-merging-concept-recognition-deduplication-and-renaming',
    construct: 'published merge quality comparison against the source grammars',
    expectedBehavior:
      'A published comparison against every inventoried source grammar measures construct coverage and correctness, preserved unique features and shared reuse, recovery quality, execution time and memory on independent corpora, never on self-generated fixtures alone.',
    assertions: ['coverageMeasured', 'correctnessMeasured', 'recoveryMeasured', 'timeAndMemoryMeasured', 'comparisonPublished'],
    javascript: null,
    rust: null,
  },
  {
    id: 'I195-NAMING-CONVENTION',
    area: 'readable-naming',
    specification: 'readable-english-names',
    fixture: 'parity/naming/canonical-concepts.json',
    construct: 'self-explanatory English noun and verb phrases for canonical names',
    expectedBehavior:
      'Every canonical concept name is an unabbreviated English noun phrase and every operation or relation name an unabbreviated English verb phrase. Original tokens and external identifiers remain lossless provenance and aliases.',
    assertions: ['conceptsAreNounPhrases', 'operationsAreVerbPhrases', 'noAbbreviations', 'originalNamesKeptAsAliases'],
    javascript: ['js/src/concept-records.js', 'js/src/network.js'],
    rust: ['rust/src/concept_records.rs', 'rust/src/concept_ontology.rs'],
  },
  {
    id: 'I195-NAMING-CONCEPT-RECORDS',
    area: 'readable-naming',
    specification: 'readable-english-names',
    fixture: 'parity/naming/canonical-concepts.json',
    construct: 'concept records with identity, phrase, meaning, constraints and aliases',
    expectedBehavior:
      'Every canonical concept has a stable identity, a readable phrase, a definition, its constraints and its source aliases. Import and merge assign and rename them, and round trips through the source formats are unaffected.',
    assertions: ['stableIdentity', 'readablePhrase', 'definitionRecorded', 'constraintsRecorded', 'sourceAliasesRecorded', 'roundTripsUnaffected'],
    javascript: ['js/src/concept-records.js', 'js/src/network.js'],
    rust: ['rust/src/concept_records.rs', 'rust/src/concept_ontology.rs'],
  },
  {
    id: 'I195-NAMING-CI-ENFORCEMENT',
    area: 'readable-naming',
    specification: 'readable-english-names',
    fixture: 'parity/naming/naming-fixtures.json',
    construct: 'CI check of the naming convention',
    expectedBehavior:
      'CI checks the vocabulary, phrase role, abbreviation expansion, collisions, duplicates and full inventory coverage of every canonical name. Positive and negative fixtures with deliberately abbreviated, ambiguous and duplicate names are rejected.',
    assertions: [
      'vocabularyChecked',
      'phraseRoleChecked',
      'abbreviationsRejected',
      'ambiguousNamesRejected',
      'duplicatesRejected',
      'fullInventoryCovered',
    ],
    tooling: [
      'js/scripts/english-vocabulary.mjs',
      'js/scripts/issue-195-naming.mjs',
      'js/scripts/check-naming.mjs',
      'parity/naming/canonical-concepts.json',
      'parity/naming/technical-vocabulary.json',
      'parity/naming/abbreviations.json',
      'parity/naming/naming-fixtures.json',
      '.github/workflows/js.yml',
    ],
  },
  {
    id: 'I195-SEMANTICS-CONSTRUCT-INVENTORY',
    area: 'full-semantics',
    specification: 'four-language-semantics-and-translation',
    fixture: FOUR_LANGUAGE_CORPUS,
    construct: 'full JavaScript, Rust, Lean and Rocq construct and project inventory',
    expectedBehavior:
      'The four-language inventory covers classes and objects, arrays and mutation, standard-library behavior, imports, higher-order functions, asynchronous behavior, ownership, macros and notation, types, universes, effects, proofs and tactics. Every entry is represented and transformed through the common links representation, and normal JavaScript is accepted without annotations.',
    assertions: ['everyConstructRepresented', 'everyConstructTransformed', 'projectsTransformed', 'normalJavaScriptAccepted'],
    javascript: null,
    rust: null,
  },
  {
    id: 'I195-SEMANTICS-FAITHFUL-BEHAVIOR',
    area: 'full-semantics',
    specification: 'four-language-semantics-and-translation',
    fixture: FOUR_LANGUAGE_CORPUS,
    construct: 'faithful errors, aborts, overflow and effects in all 12 translations',
    expectedBehavior:
      'All 12 directed translations preserve errors, aborts, overflow and effects, using a faithful operational encoding and runtime where the target has no native concept. An unsupported-construct diagnostic, source pass-through or a non-aborting-only claim does not count.',
    assertions: ['errorsPreserved', 'abortsPreserved', 'overflowPreserved', 'effectsPreserved', 'noPassThrough'],
    javascript: null,
    rust: null,
  },
  {
    id: 'I195-SEMANTICS-PROOF-PRESERVATION',
    area: 'full-semantics',
    specification: 'four-language-semantics-and-translation',
    fixture: FOUR_LANGUAGE_CORPUS,
    construct: 'theorems carried as target proof obligations',
    expectedBehavior:
      'A theorem is carried into Lean and Rocq as a faithful proof obligation and discharged by the target kernel. Bounded property checks are reported separately and never counted as general proofs, and no external kernel authority is imported into RML.',
    assertions: ['theoremsCarriedAsObligations', 'obligationsDischargedByTarget', 'boundedChecksReportedSeparately'],
    javascript: null,
    rust: null,
  },
  {
    id: 'I195-DOWNSTREAM-CONSUMER-MATRIX',
    area: 'downstream-consumers',
    specification: 'downstream-consumers',
    fixture: 'docs/downstream-consumers.md',
    construct: 'consumer matrix for relative-meta-logic and formal-ai',
    expectedBehavior:
      'docs/downstream-consumers.md maps every requirement and actual usage of link-foundation/relative-meta-logic and link-assistant/formal-ai to meta-language capabilities, ledger rows and tests, and a check fails when a mapped row or test does not exist.',
    assertions: ['relativeMetaLogicUsageMapped', 'formalAiUsageMapped', 'everyMappingResolves'],
    tooling: ['docs/downstream-consumers.md', 'js/scripts/issue-195-downstream.mjs', 'js/scripts/check-downstream-consumers.mjs'],
  },
  {
    id: 'I195-DOWNSTREAM-RML-WORKLOADS',
    area: 'downstream-consumers',
    specification: 'downstream-consumers',
    construct: 'relative-meta-logic workloads on clean installed artifacts',
    expectedBehavior:
      'The relative-meta-logic workloads (parse -> common links -> semantic transformation or translation -> emission) run against clean installed artifacts of both packages, reuse shared concepts and preserve language-specific distinctions, without meta-language taking over RML foundations or proof authority.',
    assertions: ['workloadsRunOnInstalledArtifacts', 'sharedConceptsReused', 'distinctionsPreserved', 'foundationAuthorityStaysInRml'],
    javascript: null,
    rust: null,
  },
  {
    id: 'I195-DOWNSTREAM-FORMAL-AI-WORKLOADS',
    area: 'downstream-consumers',
    specification: 'downstream-consumers',
    construct: 'formal-ai workloads on clean installed artifacts',
    expectedBehavior:
      'The link-assistant/formal-ai workloads identified by the consumer matrix run against clean installed artifacts of both packages through the common links representation.',
    assertions: ['workloadsRunOnInstalledArtifacts', 'sharedConceptsReused', 'distinctionsPreserved'],
    javascript: null,
    rust: null,
  },
  {
    id: 'I195-DOWNSTREAM-FORMAL-AI-PUBLISHED',
    area: 'downstream-consumers',
    specification: 'packages-and-publication',
    checkpoint: 'release-delivery',
    group: 'delivery:formal-ai',
    construct: 'formal-ai consuming the published packages',
    expectedBehavior:
      'link-assistant/formal-ai installs the published npm and crates.io releases that carry this work and runs its workloads against them in a clean consumer.',
    assertions: ['publishedReleasesInstalled', 'workloadsRunOnPublishedReleases'],
    javascript: null,
    rust: null,
  },
  {
    id: 'I195-ACCEPTANCE-INDEPENDENT-ORACLES',
    area: 'independent-acceptance',
    specification: 'acceptance-and-evidence',
    fixture: LANGUAGE_INVENTORY,
    construct: 'native merged grammars checked against independent oracles and real projects',
    expectedBehavior:
      'Every native merged grammar runs against pinned independent ANTLR, tree-sitter and official parser, compiler and prover corpora and real projects, with multiple oracles where available. Every construct and source discrepancy traces to a test and a justified resolution.',
    assertions: ['independentCorporaExecuted', 'multipleOraclesCompared', 'discrepanciesTracedAndResolved', 'realProjectsParsed'],
    javascript: null,
    rust: null,
  },
  {
    id: 'I195-ACCEPTANCE-GATE-MUTATIONS',
    area: 'independent-acceptance',
    specification: 'acceptance-and-evidence',
    construct: 'gate-mutation tests for the directive obligations',
    expectedBehavior:
      'Removing a grammar feature, leaving an old dependency, bypassing native execution, breaking reverse conversion, introducing a semantic duplicate or an abbreviation, dropping a source requirement, weakening an oracle mapping or replacing proof obligations with bounded checks each fails the relevant gate.',
    assertions: [
      'removedGrammarFeatureFails',
      'staleDependencyFails',
      'nativeBypassFails',
      'brokenReverseConversionFails',
      'semanticDuplicateFails',
      'abbreviationFails',
      'droppedSourceRequirementFails',
      'weakenedOracleMappingFails',
      'boundedProofSubstituteFails',
    ],
    tooling: null,
  },
  {
    id: 'I195-ACCEPTANCE-FINITE-CLAIMS-DOCUMENTED',
    area: 'independent-acceptance',
    specification: 'what-finite-tests-establish',
    construct: 'documented limits of finite test evidence',
    expectedBehavior:
      'docs/vision.md states what finite tests establish and what needs proofs or decision procedures, and the generated ledger and PR report present universal claims only with such proofs.',
    assertions: ['finiteEvidenceLimitsDocumented', 'universalClaimsRequireProofs'],
    tooling: ['docs/vision.md', 'js/scripts/issue-195-acceptance-lib.mjs'],
  },
  {
    id: 'I195-CACHE-CLEANUP-GRAMMAR-CACHES',
    area: 'cache-cleanup',
    specification: 'cache-cleanup',
    fixture: 'scripts/lib/cache-classes.mjs',
    construct: 'cleanup of imported grammar corpora, oracle builds and merged-grammar caches',
    expectedBehavior:
      'The cache cleanup registers imported grammar corpora, generator and oracle builds and merged-grammar caches as regenerable classes, removes them under the same safety rules, and never deletes canonical native grammar sources, fixtures, licenses or verification evidence.',
    assertions: [
      'grammarCorporaClassRegistered',
      'oracleBuildsClassRegistered',
      'mergedGrammarCachesClassRegistered',
      'canonicalGrammarsPreserved',
    ],
    tooling: ['scripts/lib/cache-classes.mjs', 'scripts/lib/cache-cleanup.mjs'],
  },
  {
    id: 'I195-DOCUMENTATION-RECONCILED',
    area: 'documentation-consistency',
    specification: 'documentation-consistency',
    fixture: 'parity/documentation/claim-fixtures.json',
    construct: 'README, vision, register, ledger, PR body and releases agree with the evidence',
    expectedBehavior:
      'A check compares the README, docs/vision.md, the source register, the generated ledger, the pull request description and the release reports. It fails when any of them states or implies completion while a ledger row fails, or carries an issue-closing directive before every requirement is verified and delivered.',
    assertions: [
      'readmeConsistentWithLedger',
      'pullRequestBodyConsistentWithLedger',
      'releaseReportsConsistentWithLedger',
      'noClosingDirectiveBeforeDelivery',
    ],
    tooling: ['js/scripts/issue-195-documentation.mjs', 'js/scripts/check-documentation.mjs'],
  },
]);

const RUNTIMES = ['javascript', 'rust', 'tooling'];

/** Builds the ledger entries with the ledger's own constructors. */
export function buildVisionRequirements(fixtureCatalog, { requirement, verification, pinnedFixture, source }) {
  return VISION_REQUIREMENTS.map((row) => {
    const fixtureId = `planned:repository-directive:${row.id.toLowerCase()}`;
    fixtureCatalog[fixtureId] = row.fixture
      ? pinnedFixture(row.fixture, `${VISION_SPECIFICATION}#${row.specification}: ${row.construct}`)
      : pinnedFixture(VISION_SPECIFICATION, `#${row.specification}: ${row.construct}`);
    const requiredRuntimes = RUNTIMES.filter((runtime) => runtime in row);
    return requirement({
      id: row.id,
      source,
      area: row.area,
      scope: {
        ...(requiredRuntimes.includes('tooling') ? REPOSITORY_TOOLING : BOTH_PACKAGES),
        construct: row.construct,
        aliases: [],
        extensions: [],
      },
      expectedBehavior: row.expectedBehavior,
      requiredRuntimes,
      implementationEntryPoints: Object.fromEntries(requiredRuntimes.map((runtime) => [runtime, row[runtime]])),
      verifications: requiredRuntimes.map((runtime) =>
        verification({
          requirementId: row.id,
          runtime,
          suffix: row.checkpoint === 'release-delivery' ? 'published' : 'behavior',
          fixtureIds: [fixtureId],
          assertions: row.assertions,
          checkpoint: row.checkpoint ?? 'pre-merge',
        })),
    });
  });
}

/** The evidence group of a directive row whose cells do not run in their runtime's suite. */
export function visionEvidenceGroup(requirementId) {
  const row = VISION_REQUIREMENTS.find(({ id }) => id === requirementId);
  if (!row) return null;
  if (row.group) return row.group;
  return 'tooling' in row ? 'suite:javascript' : null;
}

// Sections of docs/vision.md that specify each ledger area.
export const AREA_SPECIFICATIONS = Object.freeze({
  'default-cst': 'concrete-and-abstract-syntax-trees',
  'embedded-language': 'language-catalog-and-default-parsing',
  'grammar-importer': 'grammar-import-conversion-and-reverse-conversion',
  'four-language-semantics': 'four-language-semantics-and-translation',
  'directed-translation': 'four-language-semantics-and-translation',
  'native-validation': 'four-language-semantics-and-translation',
  'structured-transformation': 'structured-transformation-and-binding-safety',
  'binding-safe-transformation': 'structured-transformation-and-binding-safety',
  'shared-concepts': 'one-common-language-of-links',
  'runtime-parity': 'one-common-language-of-links',
  'external-conformance': 'acceptance-and-evidence',
  'generative-testing': 'acceptance-and-evidence',
  'acceptance-gate-fault-injection': 'acceptance-and-evidence',
  'package-delivery': 'packages-and-publication',
  'cache-cleanup': 'cache-cleanup',
});

const PACKAGES = Object.freeze({
  javascript: 'npm:meta-language',
  rust: 'crates.io:meta-language',
  tooling: 'repository tooling, not packaged',
  aggregate: 'acceptance gate, not packaged',
});

// Downstream consumers whose evidence a row needs by its definition; the
// consumer matrix (I195-DOWNSTREAM-CONSUMER-MATRIX) adds the consumers whose
// usage each row covers.
const DOWNSTREAM = Object.freeze({
  'I195-DELIVERY-RML-PUBLISHED': ['link-foundation/relative-meta-logic'],
  'I195-DELIVERY-RML-CANDIDATE': ['link-foundation/relative-meta-logic'],
  'I195-DOWNSTREAM-RML-WORKLOADS': ['link-foundation/relative-meta-logic'],
  'I195-DOWNSTREAM-FORMAL-AI-WORKLOADS': ['link-assistant/formal-ai'],
  'I195-DOWNSTREAM-FORMAL-AI-PUBLISHED': ['link-assistant/formal-ai'],
  'I195-LINO-UPSTREAM-REGRESSIONS': ['link-foundation/relative-meta-logic'],
});

/**
 * The traceability record of a ledger row: its specification section, the
 * inventories its fixtures pin, the registered sources that cover it, the
 * workflow and evidence groups that evaluate it, its packages and the
 * downstream consumers whose evidence it needs: those of its definition and
 * those of `consumersByRow`, the consumer matrix, or null without a matrix.
 */
export function traceabilityFor(entry, { fixtureCatalog, register, evidenceGroupFor, consumersByRow = null }) {
  const directive = VISION_REQUIREMENTS.find(({ id }) => id === entry.id);
  const anchor = directive?.specification ?? AREA_SPECIFICATIONS[entry.area] ?? null;
  const inventories = [...new Set(entry.verifications.flatMap((cell) =>
    cell.fixtureIds.map((fixtureId) => fixtureCatalog[fixtureId]?.path).filter(Boolean)))].sort();
  const sources = (register.requirementSources ?? [])
    .filter(({ ledgerCoverage }) => ledgerCoverage?.some((prefix) => entry.id.startsWith(prefix)))
    .map(({ key }) => key);
  return {
    specification: anchor ? `${VISION_SPECIFICATION}#${anchor}` : null,
    inventories,
    sources,
    workflow: '.github/workflows/issue-195-acceptance.yml',
    evidenceGroups: [...new Set(entry.verifications.map((cell) => evidenceGroupFor(entry, cell)))].sort(),
    packages: entry.requiredRuntimes.map((runtime) => PACKAGES[runtime] ?? runtime),
    downstream: consumersByRow === null
      ? DOWNSTREAM[entry.id] ?? null
      : [...new Set([...(DOWNSTREAM[entry.id] ?? []), ...(consumersByRow.get(entry.id) ?? [])])].sort(),
  };
}
