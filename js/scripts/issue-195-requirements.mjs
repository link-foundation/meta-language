import { createHash } from 'node:crypto';
import { readFile } from 'node:fs/promises';
import path from 'node:path';

import { buildCacheCleanupRequirements } from './issue-195-cache-requirements.mjs';
import { CONSUMER_MATRIX, downstreamByRow, parseConsumerMatrix } from './issue-195-downstream.mjs';
import { evidenceGroupFor } from './issue-195-evidence-plan.mjs';
import { VISION_REQUIREMENTS, buildVisionRequirements, traceabilityFor } from './issue-195-vision-requirements.mjs';
import { readProjectData, readProjectFileBytes } from './project-data.mjs';

export const ISSUE_195_MANIFEST_SCHEMA_VERSION = 1;

export const ISSUE_195_SOURCES = Object.freeze({
  issue: 'https://github.com/link-foundation/meta-language/issues/195',
  sentenceAbbreviations: 'https://github.com/link-foundation/meta-language/issues/232',
  larkRegexTerminals: 'https://github.com/link-foundation/meta-language/issues/233',
  pestBuiltInRules: 'https://github.com/link-foundation/meta-language/issues/234',
  grammarAssetAvailability: 'https://github.com/link-foundation/meta-language/issues/244',
  pdfStreamNullCharacters: 'https://github.com/link-foundation/meta-language/issues/239',
  visibleDocumentText: 'https://github.com/link-foundation/meta-language/issues/238',
  rankedLanguageIdentification: 'https://github.com/link-foundation/meta-language/issues/237',
  grammarSearch: 'https://github.com/link-foundation/meta-language/issues/235',
  jsonSchema: 'https://github.com/link-foundation/meta-language/issues/240',
  jsonRecovery: 'https://github.com/link-foundation/meta-language/issues/241',
  naturalLanguageValues: 'https://github.com/link-foundation/meta-language/issues/242',
  openVocabularyMorphology: 'https://github.com/link-foundation/meta-language/issues/236',
  normalizedTextLocation: 'https://github.com/link-foundation/meta-language/issues/243',
  remainingScope: 'https://github.com/link-foundation/meta-language/issues/199',
  clarification:
    'https://github.com/link-foundation/meta-language/pull/196#issuecomment-5795832509',
  acceptanceGate:
    'https://github.com/link-foundation/meta-language/pull/196#issuecomment-5798732287',
  completeScope:
    'https://github.com/link-foundation/meta-language/pull/196#issuecomment-5802299918',
  deliveryCorrection:
    'https://github.com/link-foundation/meta-language/pull/196#issuecomment-5819788893',
  fullDelivery:
    'https://github.com/link-foundation/meta-language/pull/196#issuecomment-5836875663',
  cacheCleanup:
    'https://github.com/link-foundation/meta-language/pull/196#issuecomment-5856764580',
  remainingAudit:
    'https://github.com/link-foundation/meta-language/pull/196#issuecomment-5870004788',
  repositoryDirective:
    'https://github.com/link-foundation/meta-language/pull/196#issuecomment-5885245090',
  consolidatedChecklist:
    'https://github.com/link-foundation/meta-language/pull/196#issuecomment-5895896874',
  currentHeadAudit:
    'https://github.com/link-foundation/meta-language/pull/196#issuecomment-5906611497',
  resourceAudit:
    'https://github.com/link-foundation/meta-language/pull/196#issuecomment-5956284229',
  greenPathAudit:
    'https://github.com/link-foundation/meta-language/pull/196#issuecomment-5965533606',
  nextPhaseAudit:
    'https://github.com/link-foundation/meta-language/pull/196#issuecomment-5971613170',
  remainingWorkAudit:
    'https://github.com/link-foundation/meta-language/pull/196#issuecomment-5979473773',
  recoveryAudit:
    'https://github.com/link-foundation/meta-language/pull/196#issuecomment-5985077141',
  finishTheWorkAudit:
    'https://github.com/link-foundation/meta-language/pull/196#issuecomment-6023909737',
  bulkDeliveryAudit:
    'https://github.com/link-foundation/meta-language/pull/196#issuecomment-6024489303',
  bulkDeliveryRepost:
    'https://github.com/link-foundation/meta-language/pull/196#issuecomment-6025209380',
  bulkDeliveryFinalRepost:
    'https://github.com/link-foundation/meta-language/pull/196#issuecomment-6025369792',
  followupDeliveryAudit:
    'https://github.com/link-foundation/meta-language/pull/201#issuecomment-6059812347',
  latestDeliveryAudit:
    'https://github.com/link-foundation/meta-language/pull/201#issuecomment-6116039616',
  formalAiSelfTranslation:
    'https://github.com/link-foundation/meta-language/issues/199#issuecomment-6055834620',
  reverseStringSemantics:
    'https://github.com/link-foundation/meta-language/issues/199#issuecomment-6101425254',
  globalConceptSpace:
    'parity/sources/global-concept-space-2026-10-12.txt',
  recursiveDefinitionsRequest:
    'parity/sources/recursive-formal-definitions-2026-10-12.txt',
  linksFoundationRequest:
    'parity/sources/links-foundation-and-fractal-motivation-2026-10-12.txt',
  recordConversationRequest:
    'parity/sources/record-conversation-requirements-2026-10-12.txt',
  associativeFoundationRequest:
    'parity/sources/associative-stack-and-theory-foundation-2026-10-12.txt',
  dynamicKnowledgeRequest:
    'parity/sources/dynamic-knowledge-and-grammar-inference-2026-10-12.txt',
  translationPriorityRequest:
    'parity/sources/bidirectional-translation-priority-2026-10-12.txt',
  multiProjectTranslationRequest:
    'parity/sources/multi-project-translation-reuse-2026-10-12.txt',
  externalGrammarStorageRequest:
    'parity/sources/temporary-external-grammar-inputs-2026-10-12.txt',
  externalGrammarIgnoreRequest:
    'parity/sources/ignored-external-grammar-folders-2026-10-12.txt',
  associativeDataStorageRequest:
    'parity/sources/associative-data-storage-2026-10-12.txt',
  agentReductionRequest:
    'parity/sources/gradual-agent-reduction-2026-10-12.txt',
  ciFollowThroughRequest:
    'parity/sources/ci-follow-through-2026-10-13.txt',
  expandedDelegationRequest:
    'parity/sources/expanded-agent-delegation-2026-10-13.txt',
  localResourceScopeRequest:
    'parity/sources/meta-language-local-resource-scope-2026-10-16.txt',
  ciCancellationInvestigationRequest:
    'parity/sources/ci-cancellation-investigation-2026-10-16.txt',
  recursiveConcreteSyntaxRequest:
    'parity/sources/recursive-concrete-syntax-and-any-grammar-2026-10-16.txt',
  ownLanguageGrammarPriorityRequest:
    'parity/sources/own-language-complete-grammar-priority-2026-10-16.txt',
  routerPracticeRecordingRequest:
    'parity/sources/router-practice-reuse-and-current-recording-2026-10-16.txt',
  modernJavaScriptParityRequest:
    'parity/sources/modern-javascript-parity-expectations-2026-10-16.txt',
  compactContributorInstructionsRequest:
    'parity/sources/compact-contributor-instructions-and-assertion-corrections-2026-10-16.txt',
  versionBoundedAssertionsRequest:
    'parity/sources/version-bounded-assertions-2026-10-16.txt',
  grammarInferenceIssue:
    'https://github.com/link-foundation/meta-language/issues/224',
  localResourceReminderRequest:
    'parity/sources/local-resource-reminder-2026-10-17.txt',
  minimalDiskContinuationRequest:
    'parity/sources/minimal-disk-continuation-2026-10-18.txt',
  twoSubagentContinuationLimitRequest:
    'parity/sources/two-subagent-continuation-limit-2026-10-18.txt',
  soloContinuationRequest:
    'parity/sources/solo-continuation-2026-10-19.txt',
  continuousBulkCompletionRequest:
    'parity/sources/continuous-bulk-completion-2026-10-19.txt',
  continueUnmetRequirementsDuringCiRequest:
    'parity/sources/continue-unmet-requirements-during-ci-2026-10-19.txt',
  automationFirstOwnCodebaseBaselineRequest:
    'parity/sources/automation-first-own-codebase-baseline-2026-10-21.txt',
  latestSoloContinuationRequest:
    'parity/sources/solo-continuation-2026-10-21.txt',
});

// The rows the audit of c39470d0 lists as open work.
const FINISH_THE_WORK_ROWS = Object.freeze([
  'I195-SELF-TRANSLATION-TOOL', 'I195-SELF-TRANSLATION-CARRIED-ZERO', 'I195-SELF-TRANSLATION-SHARED-CORPUS',
  'I195-GRAMMAR-NATIVE-MERGED', 'I195-GRAMMAR-LANGUAGE-CATALOG', 'I195-GRAMMAR-LOSSLESS-TREES',
  'I195-GRAMMAR-DEPENDENCY-BOUNDARY', 'I195-DEPENDENCY-PRODUCTION-PARSERS-REMOVED',
  'I195-MERGE-AUTOMATIC-PIPELINE', 'I195-MERGE-SHIPPED-GRAMMARS-ARE-MERGED',
  'I195-ACCEPTANCE-INDEPENDENT-ORACLES', 'I195-INTERCHANGE-FORMAT-FEATURES',
  'I195-INTERCHANGE-CROSS-FORMAT-TOOLS', 'I195-SEMANTICS-CONSTRUCT-INVENTORY',
  'I195-DOWNSTREAM-TYPESCRIPT-TRANSLATIONS',
]);

const FOUR_LANGUAGE_DETAILS = Object.freeze({
  JavaScript: { version: 'ECMAScript 2026', edition: 'ECMA-262, 17th edition' },
  Rust: { version: '1.99.0', edition: '2024' },
  Lean: { version: '4.34.1', edition: 'Lean 4' },
  Rocq: { version: '9.3', edition: 'Vernacular' },
});

export const ASSERTION_PROFILES = Object.freeze({
  cstPositive: [
    'ordinaryPublicParseApi',
    'realGrammarNodes',
    'independentExpectedStructure',
    'grammarVersionRecorded',
    'hierarchy',
    'namedFields',
    'childOrder',
    'tokens',
    'commentsAndTrivia',
    'exactUtf8Spans',
    'errorAndMissingNodes',
    'embeddedLanguageBoundaries',
    'exactReconstruction',
    'allAliases',
    'extensionDispatch',
  ],
  cstNegative: [
    'malformedInputRetained',
    'diagnosticReported',
    'notReportedAsClean',
    'exactReconstruction',
  ],
  semanticPositive: [
    'projectContextLoaded',
    'languageSpecificStructurePreserved',
    'sourceMappingPreserved',
    'missingContextDiagnosed',
    'validContextEnablesBehavior',
  ],
  transformPositive: [
    'publicApiUsed',
    'editSequence',
    'serializeReload',
    'constructionWithoutOriginalSource',
    'treeIntegrity',
    'spansUpdated',
    'sourceMappingsUpdated',
    'diagnosticsUpdated',
    'originalBufferDiscarded',
    'emittedSourceReparses',
    'intendedStructureObserved',
  ],
  bindingRename: [
    'symbolIdentity',
    'shadowing',
    'nestedScopes',
    'qualifiedNames',
    'unicodeIdentifiers',
    'macroOrProofBinders',
    'captureAvoidance',
    'commentsStringsAndLiteralsUnaffected',
  ],
  translationPositive: [
    'publicTranslatorUsed',
    'realTargetArtifact',
    'targetParses',
    'nativeTargetValidation',
    'observationContractChecked',
    'semanticPreservationChecked',
    'observationModelRecorded',
    'encodingAndRuntimeRecorded',
    'assumptionsRecorded',
    'formalObligationsDischarged',
    'sourceMappingsPreserved',
    'provenanceRecorded',
    'noSourceRelabelling',
    'noUnsupportedDescriptor',
    'noSilentWeakening',
  ],
  parityPositive: [
    'sameFixtureRevision',
    'completeStructureCompared',
    'fieldsFlagsAndSpansCompared',
    'triviaAndDiagnosticsCompared',
    'bindingsCompared',
    'transformationsCompared',
    'translationsCompared',
    'normalizationPreservesDistinctions',
    'noTextLevelEqualityShortcut',
  ],
  importerPositive: [
    'ordinaryPublicImportApi',
    'grammarExpressionsPreserved',
    'generatedParserExecuted',
    'generatedEmitterExecuted',
    'independentCorpusParses',
    'roundTripPreservesGrammar',
  ],
  conformancePositive: [
    'claimedConstructInventoryMapped',
    'upstreamCorpusExecuted',
    'representativeRealProjectExecuted',
    'externalCorpusProvenanceRecorded',
    'malformedRecoveryCasesExecuted',
    'unicodeCasesExecuted',
    'mixedLanguageCasesExecuted',
  ],
  generativePositive: [
    'propertyBasedCasesExecuted',
    'fuzzCasesExecuted',
    'metamorphicCasesExecuted',
    'roundTripPropertiesChecked',
    'malformedInputPropertiesChecked',
    'unicodeSpanPropertiesChecked',
    'editSequencePropertiesChecked',
    'independentOracleUsed',
  ],
  validationPositive: [
    'declaredToolchainPresent',
    'artifactValidated',
    'toolchainVersionRecorded',
    'reproducibleCommandRecorded',
    'failureLogRecorded',
  ],
  deliveryPositive: [
    'cleanEnvironment',
    'exactArtifactChecksum',
    'publicEntryPoints',
    'offlineFirstParse',
    'supportedPlatforms',
    'downstreamRmlIntegration',
  ],
  gateFaultInjection: [
    'faultActivated',
    'expectedRequirementFailed',
    'aggregateFailed',
    'failureReasonRecorded',
  ],
});

function slug(value) {
  return value
    .normalize('NFKD')
    .toLowerCase()
    .replace(/\+/g, '-plus')
    .replace(/#/g, '-sharp')
    .replace(/[^a-z0-9]+/g, '-')
    .replace(/^-|-$/g, '');
}

function sha256(text) {
  return createHash('sha256').update(text).digest('hex');
}

function fixtureEntry(pathname, selector, digest) {
  return { path: pathname, selector, sha256: digest };
}

function pinnedFixture(pathname, selector) {
  return fixtureEntry(pathname, selector, null);
}

function verification({
  requirementId,
  runtime,
  suffix,
  fixtureIds,
  assertions,
  checkpoint = 'pre-merge',
  kind = 'positive',
  evidenceArtifact = `issue-195-results/${requirementId}-${runtime}-${suffix}.json`.toLowerCase(),
}) {
  const testId = `${requirementId}-${runtime}-${suffix}`.toLowerCase();
  return {
    testId,
    runtime,
    kind,
    checkpoint,
    fixtureIds,
    assertions,
    evidenceArtifact,
  };
}

function requirement({
  id,
  source,
  area,
  scope,
  expectedBehavior,
  requiredRuntimes,
  implementationEntryPoints,
  verifications,
}) {
  return {
    id,
    critical: true,
    fullScopeReleaseBlocker: true,
    source,
    area,
    scope,
    expectedBehavior,
    requiredRuntimes,
    implementationEntryPoints,
    verifications,
  };
}

function buildLanguageTaskRequirements(fixtureCatalog) {
  const entries = [
    {
      id: 'I195-GRAMMAR-SENTENCE-ABBREVIATIONS',
      source: ISSUE_195_SOURCES.sentenceAbbreviations,
      language: 'English and Russian',
      version: 'shared native text grammar selected by the active catalog',
      construct: 'context-dependent periods inside lossless sentence spans',
      expectedBehavior: 'The default parser keeps common abbreviations, initials and decimal periods within a sentence when source context indicates continuation. It preserves true sentence boundaries and every source byte without adding recovery errors.',
      fixture: 'parity/fixtures/natural-language-sentence-abbreviations.lino',
      selector: 'all original reproductions and positive and negative boundary examples',
      assertions: ['sentenceSpansMatchExpectedText', 'sourceReconstructsExactly', 'parseHasNoRecoveryErrors'],
      implementationEntryPoints: {
        javascript: ['js/src/data/native-text-grammar.lino', 'js/tests/native-text-grammars.test.js'],
        rust: ['rust/src/data/native-text-grammar.lino', 'rust/tests/unit/native_text_grammars.rs'],
      },
    },
    {
      id: 'I195-GRAMMAR-PARSER-AVAILABILITY',
      source: ISSUE_195_SOURCES.grammarAssetAvailability,
      language: 'all standalone default catalog parsers',
      version: 'active catalog and grammar lock, Node.js 22 or later',
      construct: 'inspect standalone default parser availability before parsing',
      expectedBehavior: 'The default parser API lists every canonical language in catalog order. JavaScript reports the required missing WebAssembly asset paths without loading a grammar. Rust reports its compiled-in parsers as available.',
      fixture: 'parity/language-grammar-inventory.json',
      selector: 'canonical catalog order and the standalone default parser profiles',
      assertions: ['languagesListedInCatalogOrder', 'standaloneAvailabilityMatchesAssets'],
      implementationEntryPoints: {
        javascript: ['js/src/network.js', 'js/tests/grammar-parser-availability.test.js'],
        rust: ['rust/src/link_network.rs', 'rust/tests/unit/language_catalog.rs'],
      },
    },
    {
      id: 'I195-GRAMMAR-PDF-STREAM-NULL-CHARACTERS',
      source: ISSUE_195_SOURCES.pdfStreamNullCharacters,
      language: 'PDF',
      version: 'PDF 1.7 fixture parsed by the active default grammar',
      construct: 'NUL inside stream data',
      expectedBehavior: 'The parser preserves NUL bytes inside stream data and verifies the source cleanly. NUL after the end-of-file marker remains invalid and is identified by a source token error. Both cases reconstruct exactly.',
      fixture: 'parity/fixtures/pdf-stream-null-characters.lino',
      selector: 'binary stream reproduction and the trailing NUL negative case',
      assertions: ['streamBytesRetainedInDataSpan', 'streamBytesDoNotCauseRecovery', 'invalidNullCharacterHasErrorToken', 'sourcesReconstructExactly'],
      implementationEntryPoints: {
        javascript: ['js/src/pdf-grammar.js', 'js/tests/pdf-grammar.test.js'],
        rust: ['rust/src/pdf_grammar.rs', 'rust/tests/unit/pdf_grammar.rs'],
      },
    },
    {
      id: 'I195-NATURAL-LANGUAGE-IDENTIFICATION-RANKING',
      source: ISSUE_195_SOURCES.rankedLanguageIdentification,
      language: 'natural language identification',
      version: 'franc-min and franc 6.2.0, Unicode CLDR 48 alphabet data',
      construct: 'ranked language candidates and script span evidence',
      expectedBehavior: 'The package APIs rank evidence from the requested Cyrillic language models, give an unknown result below a configurable confidence threshold, keep sparse input explicit, and return mixed-script half-open UTF-8 spans.',
      fixture: 'parity/fixtures/ranked-language-identification.lino',
      selector: 'the reported Cyrillic, mixed-script and short inputs with additional Cyrillic models and Unicode boundaries',
      assertions: ['rankedCyrillicModels', 'explicitUnknownThreshold', 'shortTextEvidence', 'utf8ScriptSpans'],
      implementationEntryPoints: {
        javascript: ['js/src/language-identification.js', 'js/tests/natural-language.test.js'],
        rust: ['rust/src/language_identification.rs', 'rust/tests/unit/language_identification.rs'],
      },
    },
    ...[
      {
        id: 'I195-I239-PDF-CONTENT-STREAMS',
        construct: 'linked decoded page content and located stream errors',
        expectedBehavior: 'Both PDF parse entry points preserve the original file and expose Flate- and LZW-decoded content operators in linked documents with the encoded range. LZW supports both standard code-width transitions and chained ASCIIHex decoding. An option disables stream decoding and corrupt encoded data identifies the affected range.',
        selector: 'Flate and LZW page content, both binary input modes, disabled decoding, both LZW EarlyChange values, a chained filter and corrupt data',
        assertions: ['pdfDerivedOperatorsKeepEncodedRange', 'pdfFiltersCanBeDisabled', 'corruptPdfFilterKeepsLocatedError', 'pdfLzwCodeWidthsAndFilterChainsKeepEncodedRange', 'corruptPdfLzwKeepsLocatedError'],
      },
      {
        id: 'I195-I239-PDF-FONT-UNICODE',
        construct: 'font Unicode mappings and decoded visible text in page order',
        expectedBehavior: 'The visible PDF projection follows the page tree and show-text operators, maps glyph codes through ToUnicode CMaps and simple-font encodings, and returns the covering original stream or literal source range. Explicitly positioned horizontal lines follow their page coordinates while retaining their original source ranges.',
        selector: 'reverse physical page order, an inherited Identity-H font with bfchar and bfrange, a WinAnsi Difference, and horizontal text lines drawn out of reading order',
        assertions: ['pdfTextUsesFontMappingsAndPageOrder', 'pdfVisibleTextKeepsEncodedAndLiteralRanges', 'pdfPositionedLinesKeepReadingOrderAndRanges'],
      },
    ].map(entry => ({
      ...entry,
      source: ISSUE_195_SOURCES.pdfStreamNullCharacters,
      language: 'PDF',
      version: 'PDF 1.7, ISO 32000-1:2008 stream and text operators',
      fixture: 'parity/fixtures/pdf-document.lino',
      implementationEntryPoints: {
        javascript: ['js/src/pdf-document.js', 'js/src/pdf-text-fonts.js', 'js/tests/pdf-document.test.js'],
        rust: ['rust/src/link_network/visible_document_text/pdf_document.rs', 'rust/src/link_network/visible_document_text/pdf_text_fonts.rs', 'rust/tests/unit/document_containers.rs'],
      },
    })),
    {
      id: 'I195-I239-WORDPROCESSING-PACKAGE',
      source: ISSUE_195_SOURCES.pdfStreamNullCharacters,
      language: 'WordprocessingML',
      version: 'ECMA-376 transitional and strict namespaces with PKWARE APPNOTE 6.3.10 stored or deflated ZIP entries',
      construct: 'linked Open Packaging Conventions parts and visible text ranges',
      expectedBehavior: 'Both binary entry points open stored and deflated WordprocessingML packages and link the document, headers, footers and notes. The visible projection preserves text, tabs and breaks with original source ranges, and corrupt parts have located diagnostics without loss of original bytes.',
      fixture: 'parity/fixtures/wordprocessing-package.lino',
      selector: 'stored and deflated packages with the original part XML, exact or covering ranges, and a damaged stored part',
      assertions: ['opcPackageKeepsStoredAndDeflatedParts', 'opcVisibleTextKeepsEncodedAndStoredRanges', 'corruptOpcPartKeepsLocatedError'],
      implementationEntryPoints: {
        javascript: ['js/src/open-packaging-conventions.js', 'js/src/visible-document-text.js', 'js/tests/wordprocessing-package.test.js'],
        rust: ['rust/src/link_network/visible_document_text/document_package.rs', 'rust/src/link_network/visible_document_text/mod.rs', 'rust/tests/unit/document_containers.rs'],
      },
    },
    {
      id: 'I195-I239-PACKAGE-RELATIONSHIPS',
      source: ISSUE_195_SOURCES.pdfStreamNullCharacters,
      language: 'WordprocessingML',
      version: 'ECMA-376 Open Packaging Conventions and transitional or strict WordprocessingML relationship types',
      construct: 'WordprocessingML package relationship targets',
      expectedBehavior: 'Package and main-part relationships select the internal WordprocessingML parts, including custom locations, document-relative and package-absolute references. Both input modes retain the selected parts and original coordinates. External, unrelated and foreign-namespace relationships contribute no text. A missing or invalid internal target reports the original relationship entry range.',
      fixture: 'parity/fixtures/wordprocessing-relationships.lino',
      selector: 'stored and deflated custom part locations with relative, absolute, external and wrong-namespace relationships, plus an invalid internal target',
      assertions: ['opcRelationshipsResolveInternalParts', 'opcRelationshipsKeepSourceRanges', 'invalidOpcTargetKeepsRelationshipRange'],
      implementationEntryPoints: {
        javascript: ['js/src/open-packaging-conventions.js', 'js/src/visible-document-text.js', 'js/tests/wordprocessing-package.test.js'],
        rust: ['rust/src/link_network/visible_document_text/document_package.rs', 'rust/tests/unit/document_containers.rs'],
      },
    },
    ...[
      {
        id: 'I195-I235-GRAMMAR-SEARCH-SPANS',
        construct: 'grammar search and original-text coordinates',
        expectedBehavior: 'A compiled grammar finds the reported price phrases and returns their half-open locations and concrete trees. JavaScript uses UTF-16 offsets for match ranges and Rust uses byte offsets. Whole-input parsing keeps its existing acceptance behavior.',
        selector: 'all three price phrases and the original whole-input parse',
        assertions: ['searchFindsOriginalSpans', 'wholeInputParseIsPreserved'],
      },
      {
        id: 'I195-I235-GRAMMAR-SEARCH-CAPTURES',
        construct: 'named grammar captures',
        expectedBehavior: 'A caller can read all occurrences of a named rule in the match, including both amounts in the range, with the original spelling and span for each.',
        selector: 'amount and currency captures from all three price phrases',
        assertions: ['repeatedNamedCapturesRetainSourceSpans'],
      },
      {
        id: 'I195-I235-GRAMMAR-SEARCH-POLICY',
        construct: 'Unicode word boundaries, match selection and overlap',
        expectedBehavior: 'Search exposes first and longest match selection at each start, controls whether matches overlap, ignores zero-width results, and can require Unicode word boundaries.',
        selector: 'word-boundary, overlapping, match policy and empty-result controls',
        assertions: ['wordBoundariesUseUnicode', 'matchPoliciesChooseAtEachStart', 'overlapAndEmptyMatchesAreControlled'],
      },
      {
        id: 'I195-I235-GRAMMAR-SEARCH-QUERY',
        construct: 'grammar search in source text predicates',
        expectedBehavior: 'A grammar predicate composes with existing source text predicates in a LinkQuery over captured text.',
        selector: 'the positive and negative HTML text links with combined predicates',
        assertions: ['sourceTextQueryPredicateComposes'],
      },
    ].map(entry => ({
      ...entry,
      source: ISSUE_195_SOURCES.grammarSearch,
      language: 'native grammar execution',
      version: 'active shared grammar executors',
      fixture: 'parity/fixtures/grammar-search.json',
      implementationEntryPoints: {
        javascript: ['js/src/grammar-runtime.js', 'js/tests/grammar-search.test.js'],
        rust: ['rust/src/grammar/feature_runtime/search.rs', 'rust/tests/unit/issue_195_grammar_feature_union.rs'],
      },
    })),
    ...[
      {
        id: 'I195-I233-LARK-REGEX-TERMINALS',
        construct: 'portable regular-expression terminal lowering',
        expectedBehavior: 'The reported Lark terminals recognize their expected strings using native expressions, including Unicode digit and whitespace shorthands. Greedy regex consumption is atomic to the enclosing grammar.',
        selector: 'cases, all accepted and rejected strings on both sides of Lark emission',
        assertions: ['regexTerminalsMatchTheirLanguage'],
      },
      {
        id: 'I195-I233-LARK-REGEX-FLAGS',
        construct: 'case-insensitive and dot-all regex flags',
        expectedBehavior: 'The Lark importer recognizes i and s flags and applies the documented matching behavior of the portable subset.',
        selector: 'flags, all accepted and rejected strings on both sides of Lark emission',
        assertions: ['supportedRegexFlagsChangeMatching'],
      },
      {
        id: 'I195-I233-LARK-REGEX-DIAGNOSTICS',
        construct: 'unsupported or malformed Lark regular expressions',
        expectedBehavior: 'Unsupported flags and regex syntax fail at import with an exact UTF-8 byte diagnostic.',
        selector: 'diagnostics, exact messages including a UTF-8 input prefix',
        assertions: ['unsupportedRegexReportsSourceByte'],
      },
    ].map(entry => ({
      ...entry,
      source: ISSUE_195_SOURCES.larkRegexTerminals,
      language: 'Lark regular-expression terminals',
      version: 'shared portable importer for Python re 3.11–3.14 syntax in the covered subset',
      fixture: 'parity/fixtures/lark-regex-terminals.json',
      implementationEntryPoints: {
        javascript: ['js/src/grammar-importers/lark-regex.js', 'js/tests/grammar-lark.test.js'],
        rust: ['rust/src/grammar/import/lark/regex.rs', 'rust/tests/unit/grammar_import_lark.rs'],
      },
    })),
    ...[
      {
        id: 'I195-I234-PEST-BUILT-IN-RULES',
        construct: 'built-in ASCII and Unicode classes and input anchors',
        expectedBehavior: 'Imported Pest grammars execute predefined character rules and positive input anchors. Searching at a later offset cannot satisfy the beginning-of-input anchor.',
        selector: 'all built-in acceptance and rejection cases, and absolute anchor search spans',
        assertions: ['builtInRulesExecute', 'anchorsUseAbsolutePositions'],
      },
      {
        id: 'I195-I234-PEST-IMPLICIT-TRIVIA',
        construct: 'implicit whitespace and comment insertion',
        expectedBehavior: 'A declared whitespace or comment rule matches implicitly at sequence and repeat boundaries whenever the current Pest rule call allows implicit trivia.',
        selector: 'all trivia acceptance and rejection cases across the rule modifiers',
        assertions: ['implicitTriviaTracksAtomicContext'],
      },
      {
        id: 'I195-I234-PEST-RULE-DIAGNOSTICS',
        construct: 'undefined rules and unsupported stacks',
        expectedBehavior: 'Undefined rule references and unsupported stack rules produce consistent explicit grammar import errors.',
        selector: 'all grammar sources and exact diagnostic messages',
        assertions: ['pestRuleDiagnosticsUseExactMessages'],
      },
    ].map(entry => ({
      ...entry,
      source: ISSUE_195_SOURCES.pestBuiltInRules,
      language: 'Pest PEG grammars',
      version: 'Pest 2.9.2 built-in character tables and grammar semantics',
      fixture: 'parity/fixtures/pest-built-ins.lino',
      implementationEntryPoints: {
        javascript: ['js/src/grammar-importers/pest.js', 'js/tests/grammar-importers.test.js'],
        rust: ['rust/src/grammar/import/pest.rs', 'rust/tests/unit/grammar_import_pest.rs'],
      },
    })),
    {
      id: 'I195-I236-OPEN-VOCABULARY-MORPHOLOGY',
      source: ISSUE_195_SOURCES.openVocabularyMorphology,
      language: 'Natural-language word morphology with an OpenCorpora Russian vocabulary',
      version: 'Universal Dependencies v2 tags, OpenCorpora source revision 417150 and corpus revision 4580142',
      construct: 'scored morphology readings, inverse inflection and cardinal agreement',
      expectedBehavior: 'Each segmented word has lemma, part of speech and feature links at its UTF-8 span. Scored alternatives preserve ambiguous readings. Suffix prediction covers unseen words. Inflection selects vocabulary forms and cardinal agreement uses the requested number.',
      fixture: 'parity/fixtures/natural-language-morphology.lino',
      selector: 'the reported Russian sentence, dictionary ambiguity, an invented instrumental plural, and Russian and English inflections and number agreement',
      assertions: ['everyWordHasLinkedReading', 'ambiguousReadingsAreScored', 'unseenWordsHaveSuffixReadings', 'inflectionAndNumberAgreement'],
      implementationEntryPoints: {
        javascript: ['js/src/morphology.js', 'js/tests/natural-language-morphology.test.js'],
        rust: ['rust/src/link_network/morphology/mod.rs', 'rust/tests/unit/natural_language_morphology.rs'],
      },
    },
    {
      id: 'I195-I240-JSON-SCHEMA',
      source: ISSUE_195_SOURCES.jsonSchema,
      language: 'JSON with draft 2020-12 schemas',
      version: 'RFC 8259 JSON, RFC 6901 pointer and JSON Schema draft 2020-12 semantics',
      construct: 'schema import, decoder grammar export, located validation and reverse inference',
      expectedBehavior: 'Import named and referenced schema constraints and emit interoperable decoder grammars. Report unsupported constraints. Validation preserves syntax links, UTF-8 spans and pointers at schema violations and malformed JSON. Emit the original schema or infer a schema from bounded finite productions.',
      fixture: 'parity/fixtures/json-schema-and-recovery.lino',
      selector: 'answer schema, closed records, tuple bounds, local references, formats, integer ranges, finite exclusive alternatives, unsupported keywords, malformed answers and inferred constants',
      assertions: ['decoderGrammarsConstrainSchema', 'schemaDiagnosticsRetainSyntaxLinks', 'grammarAndSchemaRoundTrip'],
      implementationEntryPoints: {
        javascript: ['js/src/grammar-importers/json-schema.js', 'js/src/json-schema-validation.js', 'js/tests/grammar-json-document.test.js'],
        rust: ['rust/src/grammar/json_document/import.rs', 'rust/src/grammar/json_document/validation.rs', 'rust/tests/unit/json_schema_and_recovery.rs'],
      },
    },
    {
      id: 'I195-I241-JSON-RECOVERY',
      source: ISSUE_195_SOURCES.jsonRecovery,
      language: 'JSON document recovery',
      version: 'RFC 8259 JSON and RFC 6901 pointers',
      construct: 'concrete-syntax-anchored edit policy and repetition diagnostics',
      expectedBehavior: 'Each enabled recovery action retains an edit anchored to original recovery syntax. EOF closure preserves incomplete status and validation reports required fields without supplying values. Degenerate repetition has a precise original span.',
      fixture: 'parity/fixtures/json-schema-and-recovery.lino',
      selector: 'all six actions, combined comma and EOF closure, no-op defaults, missing required fields and Unicode repetition spans',
      assertions: ['permittedRepairsRetainEditAnchors', 'closingRetainsTruncatedStatusAndMissingFields', 'repetitionsRetainSourceSpans'],
      implementationEntryPoints: {
        javascript: ['js/src/json-recovery.js', 'js/tests/grammar-json-document.test.js'],
        rust: ['rust/src/grammar/json_document/recovery.rs', 'rust/tests/unit/json_schema_and_recovery.rs'],
      },
    },
    {
      id: 'I195-I242-NATURAL-LANGUAGE-VALUES',
      source: ISSUE_195_SOURCES.naturalLanguageValues,
      language: 'English, Russian and shared digit-based value syntax',
      version: 'ISO 4217 List One 2026-09-17, exact decimal arithmetic, proleptic Gregorian dates',
      construct: 'typed value links from grammar searches',
      expectedBehavior: 'Grammar search emits localized number, numeric range, currency, duration, calendar date and calendar range annotations over original UTF-8 spans. Inflected number words and open date endpoints are recognized. A numeric range and a date can coexist over the same spelling.',
      fixture: 'parity/fixtures/natural-language-values.lino',
      selector: 'the reported Russian and English examples, related inflected forms, punctuation, shared Spanish digit and currency syntax, and invalid dates and currency codes',
      assertions: ['localizedExactNumbers', 'currencyRanges', 'inflectedDurations', 'dateAlternatives', 'utf8SourceSpans'],
      implementationEntryPoints: {
        javascript: ['js/src/natural-language-values.js', 'js/tests/natural-language-values.test.js'],
        rust: ['rust/src/link_network/natural_language_values/mod.rs', 'rust/tests/unit/natural_language_values.rs'],
      },
    },
    ...[
      {
        id: 'I195-I243-NORMALIZED-TEXT-LOCATION',
        construct: 'normalization-tolerant quote location',
        expectedBehavior: 'Every occurrence of the normalized fragment has an original UTF-16 or UTF-8 source span and the transformations needed for equality. An occurrence spans whole normalization expansions.',
        selector: 'exact cases, all matched original spans and needed normalizations',
        assertions: ['allNormalizedMatchesKeepSourceRanges', 'neededNormalizationsAreReported'],
      },
      {
        id: 'I195-I243-APPROXIMATE-TEXT-LOCATION',
        construct: 'bounded grapheme and word edit scripts',
        expectedBehavior: 'An explicit edit budget switches on approximate matching. Each returned edit describes the source and fragment unit, with native original-text offsets on each applicable side.',
        selector: 'approximate cases, substitutions, insertions and deletions in both edit units',
        assertions: ['editBudgetAndUnitControlMatches', 'reportedEditScriptsKeepSourceRanges'],
      },
      {
        id: 'I195-I243-WORD-NGRAM-OVERLAP',
        construct: 'word sequence overlap and source range pairs',
        expectedBehavior: 'A shared word n-gram is reported once for every left and right occurrence pair, with exact source spans in both original texts.',
        selector: 'overlap cases, duplicate word sequence pairs and the character-only negative input',
        assertions: ['wordNgramsReportEverySharedSourcePair'],
      },
    ].map(entry => ({
      ...entry,
      source: ISSUE_195_SOURCES.normalizedTextLocation,
      language: 'Unicode text search',
      version: 'Unicode 13.0 through 17.0 fixture characters and boundaries',
      fixture: 'parity/fixtures/text-location.lino',
      implementationEntryPoints: {
        javascript: ['js/src/text-location.js', 'js/tests/language-text-location.test.js'],
        rust: ['rust/src/link_network/text_location.rs', 'rust/tests/unit/text_location.rs'],
      },
    })),
    ...[
      {
        id: 'I195-I238-HTML-VISIBLE-TEXT',
        language: 'HTML',
        version: 'HTML Living Standard character reference table pinned in build-document-character-references.mjs',
        construct: 'visible HTML text and character reference projection',
        expectedBehavior: 'Visible HTML omits hidden elements, comments and attributes, applies the configured entity and block options, and maps each visible range to its original bytes.',
        selector: 'HTML cases, exact text, projection options, monotonic ranges and selected source spans',
        assertions: ['htmlVisibleTextAndOptions', 'htmlVisibleTextKeepsSourceRanges'],
      },
      {
        id: 'I195-I238-MARKDOWN-VISIBLE-TEXT',
        language: 'Markdown',
        version: 'CommonMark 0.31.2 inline text and code span notation',
        construct: 'visible Markdown text and source ranges',
        expectedBehavior: 'Visible Markdown retains link labels, list bodies and literal code, removes syntax markers, and maps visible ranges to their original bytes.',
        selector: 'Markdown cases, exact text, monotonic ranges and selected source spans',
        assertions: ['markdownVisibleText', 'markdownVisibleTextKeepsSourceRanges'],
      },
      {
        id: 'I195-I238-WORDPROCESSINGML-VISIBLE-TEXT',
        language: 'WordprocessingML',
        version: 'ECMA-376 WordprocessingML transitional namespace for paragraph, text and break markup',
        construct: 'visible WordprocessingML text and source ranges',
        expectedBehavior: 'Visible XML document text preserves runs, tabs and line breaks in document order and maps visible ranges to their original bytes.',
        selector: 'WordprocessingML cases, exact text, monotonic ranges and selected source spans',
        assertions: ['wordprocessingmlVisibleText', 'wordprocessingmlVisibleTextKeepsSourceRanges'],
      },
    ].map(entry => ({
      ...entry,
      source: ISSUE_195_SOURCES.visibleDocumentText,
      fixture: 'parity/fixtures/visible-document-text.lino',
      implementationEntryPoints: {
        javascript: ['js/src/visible-document-text.js', 'js/tests/visible-document-text.test.js'],
        rust: ['rust/src/link_network/visible_document_text/mod.rs', 'rust/tests/unit/visible_document_text.rs'],
      },
    })),
  ];
  return entries.map(({ id, source, language, version, construct, expectedBehavior, fixture, selector, assertions, implementationEntryPoints }) => {
    const fixtureId = id.toLowerCase();
    fixtureCatalog[fixtureId] = pinnedFixture(fixture, selector);
    const requiredRuntimes = ['javascript', 'rust'];
    return requirement({
      id, source, area: 'default-cst', scope: { language, version, edition: 'current default', construct, aliases: [], extensions: [] },
      expectedBehavior, requiredRuntimes, implementationEntryPoints,
      verifications: requiredRuntimes.map((runtime) => verification({
        requirementId: id, runtime, suffix: 'behavior', fixtureIds: [fixtureId], assertions,
      })),
    });
  });
}

function languageDetails(language) {
  return (
    FOUR_LANGUAGE_DETAILS[language] ?? {
      version: 'the version pinned by the runtime lockfiles',
      edition: 'every advertised alias, dialect, and registered profile',
    }
  );
}

function buildCstRequirements(inventory, fixtureCatalog) {
  return inventory.languages.map((language) => {
    const languageSlug = slug(language.name);
    const id = `I195-CST-${languageSlug}`;
    const fixtureId = `inventory:${language.name}`;
    const details = languageDetails(language.name);
    fixtureCatalog[fixtureId] = fixtureEntry(
      'parity/language-grammar-inventory.json',
      `languages[name=${JSON.stringify(language.name)}]`,
      null,
    );
    const negativeFixtureId = `planned:cst-negative:${language.name}`;
    fixtureCatalog[negativeFixtureId] = fixtureEntry(
      'parity/language-grammar-inventory.json',
      `languages[name=${JSON.stringify(language.name)}].source + negativeMutation`,
      null,
    );

    const verifications = [];
    for (const runtime of ['javascript', 'rust']) {
      verifications.push(
        verification({
          requirementId: id,
          runtime,
          suffix: 'positive',
          fixtureIds: [fixtureId],
          assertions: ASSERTION_PROFILES.cstPositive,
        }),
        verification({
          requirementId: id,
          runtime,
          suffix: 'negative',
          fixtureIds: [negativeFixtureId],
          assertions: ASSERTION_PROFILES.cstNegative,
          kind: 'negative',
        }),
      );
    }

    return requirement({
      id,
      source: ISSUE_195_SOURCES.acceptanceGate,
      area: 'default-cst',
      scope: {
        language: language.name,
        version: details.version,
        edition: details.edition,
        construct: 'complete lossless concrete syntax tree through the ordinary parse API',
        aliases: language.aliases,
        extensions: inventory.extensionDispatch?.[language.name] ?? [],
        family: language.family,
      },
      expectedBehavior:
        'Both runtimes select a real grammar by default and independently verify complete CST structure, recovery, and byte-exact reconstruction for every alias and extension path.',
      requiredRuntimes: ['javascript', 'rust'],
      implementationEntryPoints: {
        javascript: ['js/src/network.js', 'js/src/programming-language-parser.js'],
        rust: ['rust/src/language_parser.rs', 'rust/src/tree_sitter_adapter.rs'],
      },
      verifications,
    });
  });
}

function buildEmbeddedRequirements(inventory, fixtureCatalog) {
  const requirements = [];
  for (const pathEntry of inventory.embeddedLanguagePaths) {
    for (const target of pathEntry.targets) {
      const id = `I195-EMBED-${slug(pathEntry.host)}-${slug(target)}`;
      const fixtureId = `planned:embedded:${pathEntry.host}:${target}`;
      fixtureCatalog[fixtureId] = pinnedFixture(
        'parity/fixtures/issue-195-evidence.json',
        `${pathEntry.host} host with an independently asserted ${target} embedded boundary`,
      );
      requirements.push(
        requirement({
          id,
          source: ISSUE_195_SOURCES.acceptanceGate,
          area: 'embedded-language',
          scope: {
            language: pathEntry.host,
            version: 'the host and target versions pinned by runtime lockfiles',
            edition: 'all advertised embedded paths',
            construct: `embedded ${target}`,
            aliases: [],
            extensions: [],
          },
          expectedBehavior:
            'The host CST retains an explicit embedded-language boundary and the embedded source is parsed by the target grammar in both runtimes.',
          requiredRuntimes: ['javascript', 'rust'],
          implementationEntryPoints: {
            javascript: ['js/src/network.js', 'js/src/regions.js', 'js/src/programming-language-parser.js'],
            rust: ['rust/src/link_network.rs', 'rust/src/embedded_region_parser.rs'],
          },
          verifications: ['javascript', 'rust'].map((runtime) =>
            verification({
              requirementId: id,
              runtime,
              suffix: 'positive',
              fixtureIds: [fixtureId],
              assertions: ASSERTION_PROFILES.cstPositive,
            }),
          ),
        }),
      );
    }
  }
  return requirements;
}

function buildImporterRequirements(inventory, fixtureCatalog) {
  return inventory.grammarImporters.map((importer) => {
    const id = `I195-IMPORT-${slug(importer)}`;
    const fixtureId = `planned:grammar-importer:${importer}`;
    fixtureCatalog[fixtureId] = pinnedFixture(
      'parity/fixtures/grammar-importers.json',
      `${importer} grammar importing, parsing, emission, and round-trip fixture`,
    );
    return requirement({
      id,
      source: ISSUE_195_SOURCES.acceptanceGate,
      area: 'grammar-importer',
      scope: {
        language: importer,
        version: 'declared importer version',
        edition: 'all advertised importer constructs',
        construct: 'grammar import, generated parser, and generated emitter',
        aliases: [],
        extensions: [],
      },
      expectedBehavior:
        'Both runtimes import the grammar, parse its corpus, emit source, and retain the imported grammar constructs without fallback.',
      requiredRuntimes: ['javascript', 'rust'],
      implementationEntryPoints: {
        javascript: ['js/src/grammar-importers.js', 'js/src/grammar-importers'],
        rust: ['rust/src/grammar'],
      },
      verifications: ['javascript', 'rust'].map((runtime) =>
        verification({
          requirementId: id,
          runtime,
          suffix: 'positive',
          fixtureIds: [fixtureId],
          assertions: ASSERTION_PROFILES.importerPositive,
        }),
      ),
    });
  });
}

function buildFourLanguageCorpusRequirements(fixtureCatalog, extensions) {
  const requirements = [];
  for (const [language, details] of Object.entries(FOUR_LANGUAGE_DETAILS)) {
    for (const [suffix, construct, profile] of [
      [
        'CONFORMANCE',
        'mapped upstream conformance suites, grammar corpora, and representative real projects',
        ASSERTION_PROFILES.conformancePositive,
      ],
      [
        'GENERATIVE',
        'property-based, fuzz, and metamorphic coverage for round trips, recovery, Unicode spans, and edit sequences',
        ASSERTION_PROFILES.generativePositive,
      ],
    ]) {
      const id = `I195-${suffix}-${slug(language)}`;
      const fixtureId = `planned:${suffix.toLowerCase()}:${language}`;
      fixtureCatalog[fixtureId] = pinnedFixture(
        suffix === 'CONFORMANCE'
          ? 'parity/fixtures/issue-195-conformance/manifest.json'
          : 'parity/fixtures/issue-195-generative/manifest.json',
        `${language} ${construct}`,
      );
      requirements.push(
        requirement({
          id,
          source: ISSUE_195_SOURCES.acceptanceGate,
          area: suffix === 'CONFORMANCE' ? 'external-conformance' : 'generative-testing',
          scope: {
            language,
            version: details.version,
            edition: details.edition,
            construct,
            aliases: [],
            extensions: extensions[language],
          },
          expectedBehavior:
            suffix === 'CONFORMANCE'
              ? 'Both runtimes execute mapped external corpora and real projects with provenance and independently asserted construct coverage, including malformed, Unicode, and mixed-language cases.'
              : 'Both runtimes execute reproducible generative tests against an independent oracle; self-consistency between the parser and its own emitter is insufficient.',
          requiredRuntimes: ['javascript', 'rust'],
          implementationEntryPoints: {
            javascript: ['js/src/network.js', 'js/src/program-representation.js'],
            rust: ['rust/src/link_network.rs', 'rust/src/program_representation.rs'],
          },
          verifications: ['javascript', 'rust'].map((runtime) =>
            verification({
              requirementId: id,
              runtime,
              suffix: 'positive-and-negative',
              fixtureIds: [fixtureId],
              assertions: profile,
            }),
          ),
        }),
      );
    }
  }
  return requirements;
}

function buildSemanticRequirements(fixtureCatalog, extensions) {
  const constructs = [
    'modules-and-imports',
    'scopes-and-bindings',
    'recursive-definitions',
    'types-and-universes',
    'effects',
    'attributes',
    'macros-and-notation',
    'proof-terms-and-tactics',
    'surface-expansion-elaboration-traces',
    'project-context-and-dependencies',
  ];
  const requirements = [];
  for (const [language, details] of Object.entries(FOUR_LANGUAGE_DETAILS)) {
    for (const construct of constructs) {
      const id = `I195-SEM-${slug(language)}-${construct}`;
      const fixtureId = `planned:semantic:${language}:${construct}`;
      fixtureCatalog[fixtureId] = pinnedFixture(
        'parity/fixtures/four-language-conformance.json',
        `${language} real-project positive and missing-context cases for ${construct}`,
      );
      requirements.push(
        requirement({
          id,
          source: ISSUE_195_SOURCES.acceptanceGate,
          area: 'four-language-semantics',
          scope: {
            language,
            version: details.version,
            edition: details.edition,
            construct,
            aliases: [],
            extensions: extensions[language],
          },
          expectedBehavior:
            'Both runtimes represent this construct with project-aware symbol identity and language-specific distinctions; missing context is diagnosed and valid context enables the behavior.',
          requiredRuntimes: ['javascript', 'rust'],
          implementationEntryPoints: {
            javascript: ['js/src/program-representation.js', 'js/src/language-support.js'],
            rust: ['rust/src/program_representation.rs', 'rust/src/language_support.rs'],
          },
          verifications: ['javascript', 'rust'].map((runtime) =>
            verification({
              requirementId: id,
              runtime,
              suffix: 'positive-and-negative',
              fixtureIds: [fixtureId],
              assertions: ASSERTION_PROFILES.semanticPositive,
            }),
          ),
        }),
      );
    }
  }
  return requirements;
}

function buildTransformationRequirements(fixtureCatalog, extensions) {
  const operations = ['query', 'insert', 'delete', 'replace', 'move', 'clone', 'construct', 'emit'];
  const requirements = [];
  for (const [language, details] of Object.entries(FOUR_LANGUAGE_DETAILS)) {
    for (const operation of operations) {
      const id = `I195-XFORM-${slug(language)}-${operation}`;
      const fixtureId = `planned:transformation:${language}:${operation}`;
      fixtureCatalog[fixtureId] = pinnedFixture(
        'parity/fixtures/four-language-conformance.json',
        `${language} ${operation} sequences, serialize/reload, and reparsing corpus`,
      );
      requirements.push(
        requirement({
          id,
          source: ISSUE_195_SOURCES.acceptanceGate,
          area: 'structured-transformation',
          scope: {
            language,
            version: details.version,
            edition: details.edition,
            construct: operation,
            aliases: [],
            extensions: extensions[language],
          },
          expectedBehavior:
            'The public structured transformation operates after the original source buffer is discarded, updates derived metadata, emits valid source, and reparses to the intended structure.',
          requiredRuntimes: ['javascript', 'rust'],
          implementationEntryPoints: {
            javascript: ['js/src/program-representation.js', 'js/src/program-snapshot.js'],
            rust: [
              'rust/src/program_representation.rs',
              'rust/src/program_representation/edit.rs',
              'rust/src/program_representation/snapshot.rs',
            ],
          },
          verifications: ['javascript', 'rust'].map((runtime) =>
            verification({
              requirementId: id,
              runtime,
              suffix: 'positive',
              fixtureIds: [fixtureId],
              assertions: ASSERTION_PROFILES.transformPositive,
            }),
          ),
        }),
      );
    }

    const renameId = `I195-RENAME-${slug(language)}`;
    const renameFixtureId = `planned:binding-rename:${language}`;
    fixtureCatalog[renameFixtureId] = pinnedFixture(
      'parity/fixtures/four-language-conformance.json',
      `${language} binding-aware rename and capture-avoidance corpus`,
    );
    requirements.push(
      requirement({
        id: renameId,
        source: ISSUE_195_SOURCES.acceptanceGate,
        area: 'binding-safe-transformation',
        scope: {
          language,
          version: details.version,
          edition: details.edition,
          construct: 'binding-aware rename with capture avoidance',
          aliases: [],
          extensions: extensions[language],
        },
        expectedBehavior:
          'Rename follows symbol identity across scopes and modules, avoids capture, handles Unicode and generated binders, and leaves comments, strings, regexes, and unrelated names unchanged.',
        requiredRuntimes: ['javascript', 'rust'],
        implementationEntryPoints: {
          javascript: ['js/src/program-representation.js'],
          rust: ['rust/src/program_representation.rs', 'rust/src/program_representation/edit.rs'],
        },
        verifications: ['javascript', 'rust'].map((runtime) =>
          verification({
            requirementId: renameId,
            runtime,
            suffix: 'positive-and-negative',
            fixtureIds: [renameFixtureId],
            assertions: ASSERTION_PROFILES.bindingRename,
          }),
        ),
      }),
    );
  }
  return requirements;
}

function buildTranslationRequirements(fixtureCatalog, extensions) {
  const languages = Object.keys(FOUR_LANGUAGE_DETAILS);
  const requirements = [];
  for (const sourceLanguage of languages) {
    for (const targetLanguage of languages) {
      if (sourceLanguage === targetLanguage) continue;
      const id = `I195-TRANSLATE-${slug(sourceLanguage)}-to-${slug(targetLanguage)}`;
      const fixtureId = `planned:translation:${sourceLanguage}:${targetLanguage}`;
      fixtureCatalog[fixtureId] = pinnedFixture(
        'parity/fixtures/four-language-conformance.json',
        `${sourceLanguage} to ${targetLanguage} construct/project corpus and preservation oracle`,
      );
      requirements.push(
        requirement({
          id,
          source: ISSUE_195_SOURCES.acceptanceGate,
          area: 'directed-translation',
          scope: {
            language: `${sourceLanguage} -> ${targetLanguage}`,
            version: `${FOUR_LANGUAGE_DETAILS[sourceLanguage].version} -> ${FOUR_LANGUAGE_DETAILS[targetLanguage].version}`,
            edition: `${FOUR_LANGUAGE_DETAILS[sourceLanguage].edition} -> ${FOUR_LANGUAGE_DETAILS[targetLanguage].edition}`,
            construct: 'control flow, recursion, data/types, modules, effects, and proof constructs',
            aliases: [],
            extensions: extensions[targetLanguage],
          },
          expectedBehavior:
            'The public translator emits a real validated target artifact and satisfies the declared observation and preservation contract without unsupported descriptors, relabelled input, erased effects, added axioms, or weakened theorems.',
          requiredRuntimes: ['javascript', 'rust'],
          implementationEntryPoints: {
            javascript: ['js/src/program-translation.js', 'js/src/translation-renderer.js'],
            rust: ['rust/src/program_translation.rs', 'rust/src/translation_rules/renderer.rs'],
          },
          verifications: ['javascript', 'rust'].map((runtime) =>
            verification({
              requirementId: id,
              runtime,
              suffix: 'positive',
              fixtureIds: [fixtureId],
              assertions: ASSERTION_PROFILES.translationPositive,
            }),
          ),
        }),
      );
    }
  }
  return requirements;
}

function buildCrossCuttingRequirements(fixtureCatalog, extensions) {
  const rows = [
    ['I195-SHARED-SCHEMA', 'shared-concepts', 'schema versioning and extension registration'],
    ['I195-SHARED-PROVENANCE', 'shared-concepts', 'provenance and source mappings'],
    ['I195-SHARED-CONCEPTS', 'shared-concepts', 'binding, typing, module, and proof concepts without semantic collapse'],
    ['I195-PARITY-CST', 'runtime-parity', 'normalized complete CSTs'],
    ['I195-PARITY-DIAGNOSTICS', 'runtime-parity', 'diagnostics and recovery'],
    ['I195-PARITY-SEMANTICS', 'runtime-parity', 'bindings, types, modules, and proof structure'],
    ['I195-PARITY-TRANSFORMS', 'runtime-parity', 'structured transformations'],
    ['I195-PARITY-TRANSLATIONS', 'runtime-parity', 'all directed translations'],
  ];
  return rows.map(([id, area, construct]) => {
    const fixtureId = `planned:cross-cutting:${id}`;
    fixtureCatalog[fixtureId] = pinnedFixture(
      'parity/fixtures/four-language-conformance.json',
      `${construct} shared corpus and expected normalization`,
    );
    return requirement({
      id,
      source: ISSUE_195_SOURCES.acceptanceGate,
      area,
      scope: {
        language: 'JavaScript, Rust, Lean, Rocq, and the complete registered inventory',
        version: 'manifest schema revision 1 and pinned language versions',
        edition: 'all advertised profiles',
        construct,
        aliases: [],
        extensions: [],
      },
      expectedBehavior:
        'The same fixture revision produces faithfully normalized, fully compared observable behavior in both packages without erasing language distinctions.',
      requiredRuntimes: ['javascript', 'rust'],
      implementationEntryPoints: {
        javascript: ['js/src/program-representation.js', 'js/scripts/check-js-rust-parity.mjs'],
        rust: ['rust/src/program_representation.rs', 'rust/src/program_translation.rs'],
      },
      verifications: ['javascript', 'rust'].map((runtime) =>
        verification({
          requirementId: id,
          runtime,
          suffix: 'positive',
          fixtureIds: [fixtureId],
          assertions: ASSERTION_PROFILES.parityPositive,
        }),
      ),
    });
  });
}

function buildValidationAndDeliveryRequirements(fixtureCatalog, extensions) {
  const requirements = [];
  for (const language of Object.keys(FOUR_LANGUAGE_DETAILS)) {
    const id = `I195-NATIVE-${slug(language)}`;
    const fixtureId = `planned:native-validation:${language}`;
    fixtureCatalog[fixtureId] = pinnedFixture(
      'parity/fixtures/issue-195-evidence.json',
      `${language} emitted-artifact validation corpus`,
    );
    requirements.push(
      requirement({
        id,
        source: ISSUE_195_SOURCES.acceptanceGate,
        area: 'native-validation',
        scope: {
          language,
          version: FOUR_LANGUAGE_DETAILS[language].version,
          edition: FOUR_LANGUAGE_DETAILS[language].edition,
          construct: 'emitted artifact validation',
          aliases: [],
          extensions: extensions[language],
        },
        expectedBehavior:
          'The declared native compiler, host, or prover validates emitted artifacts and records reproducible versions and logs without being treated as RML proof authority.',
        requiredRuntimes: ['javascript', 'rust'],
        implementationEntryPoints: {
          javascript: ['js/src/program-translation.js', 'js/scripts/run-issue-195-evidence.mjs'],
          rust: ['rust/src/program_translation.rs'],
        },
        verifications: ['javascript', 'rust'].map((runtime) =>
          verification({
            requirementId: id,
            runtime,
            suffix: 'positive',
            fixtureIds: [fixtureId],
            assertions: ASSERTION_PROFILES.validationPositive,
          }),
        ),
      }),
    );
  }

  const deliveryRows = [
    ['I195-DELIVERY-NPM-CANDIDATE', 'npm candidate package', 'pre-merge'],
    ['I195-DELIVERY-CRATE-CANDIDATE', 'crate candidate package', 'pre-merge'],
    ['I195-DELIVERY-RML-CANDIDATE', 'downstream RML candidate-artifact integration', 'pre-merge'],
    ['I195-DELIVERY-NPM-PUBLISHED', 'published npm package', 'release-delivery'],
    ['I195-DELIVERY-CRATE-PUBLISHED', 'published crates.io package', 'release-delivery'],
    ['I195-DELIVERY-RML-PUBLISHED', 'downstream RML published-artifact integration', 'release-delivery'],
  ];
  for (const [id, construct, checkpoint] of deliveryRows) {
    const fixtureId = `planned:delivery:${id}`;
    fixtureCatalog[fixtureId] = pinnedFixture(
      'parity/fixtures/issue-195-evidence.json',
      `${construct} clean-consumer matrix`,
    );
    requirements.push(
      requirement({
        id,
        source: ISSUE_195_SOURCES.acceptanceGate,
        area: 'package-delivery',
        scope: {
          language: 'JavaScript and Rust packages',
          version: 'matching release version',
          edition: 'all supported OS/runtime combinations',
          construct,
          aliases: [],
          extensions: [],
        },
        expectedBehavior:
          'A clean consumer installs the exact artifact, verifies its checksum, uses public entry points, parses offline on first use, and passes downstream RML integration.',
        requiredRuntimes: ['javascript', 'rust'],
        implementationEntryPoints: {
          javascript: ['js/package.json', 'js/scripts/run-issue-195-evidence.mjs'],
          rust: ['rust/Cargo.toml', 'js/scripts/run-issue-195-evidence.mjs'],
        },
        verifications: ['javascript', 'rust'].map((runtime) =>
          verification({
            requirementId: id,
            runtime,
            suffix: checkpoint,
            fixtureIds: [fixtureId],
            assertions: ASSERTION_PROFILES.deliveryPositive,
            checkpoint,
          }),
        ),
      }),
    );
  }
  return requirements;
}

function buildGateRequirements(fixtureCatalog) {
  const faults = [
    'plain-text-parser-fallback',
    'lexical-parser-fallback',
    'removed-inventory-row-or-alias',
    'dropped-fields-trivia-or-spans',
    'stubbed-binding-resolution',
    'rename-capture-bug',
    'stale-structure-after-edit',
    'unsupported-translation-descriptor',
    'relabeled-source-translation',
    'skipped-test-or-job',
    'falsified-capability-declaration',
  ];
  return faults.map((fault) => {
    const id = `I195-GATE-FAULT-${fault}`;
    const fixtureId = `fault:${fault}`;
    fixtureCatalog[fixtureId] = fixtureEntry(
      'js/tests/issue-195-acceptance.test.js',
      `fault injection: ${fault}`,
      null,
    );
    return requirement({
      id,
      source: ISSUE_195_SOURCES.acceptanceGate,
      area: 'acceptance-gate-fault-injection',
      scope: {
        language: 'aggregate acceptance gate',
        version: 'manifest schema revision 1',
        edition: 'pre-merge acceptance checkpoint',
        construct: fault,
        aliases: [],
        extensions: [],
      },
      expectedBehavior:
        'Activating this known shortcut makes the aggregate acceptance result fail with the affected stable requirement ID and a reproducible reason.',
      requiredRuntimes: ['aggregate'],
      implementationEntryPoints: {
        aggregate: [
          'js/scripts/issue-195-acceptance-lib.mjs',
          'js/scripts/check-issue-195-acceptance.mjs',
        ],
      },
      verifications: [
        verification({
          requirementId: id,
          runtime: 'aggregate',
          suffix: 'mutation',
          fixtureIds: [fixtureId],
          assertions: ASSERTION_PROFILES.gateFaultInjection,
          kind: 'fault-injection',
          evidenceArtifact: `issue-195-results/gate-faults.json#${id.toLowerCase()}-aggregate-mutation`,
        }),
      ],
    });
  });
}

export async function buildIssue195Manifest(root) {
  const inventoryPath = path.join(root, 'parity', 'language-grammar-inventory.json');
  const corpusPath = path.join(root, 'parity', 'fixtures', 'four-language-conformance.json');
  const registerPath = path.join(root, 'parity', 'issue-195-sources.json');
  const [inventoryText, corpusText, register, consumerMatrixText] = await Promise.all([
    readProjectFileBytes(inventoryPath),
    readProjectFileBytes(corpusPath),
    readProjectData(registerPath),
    readFile(path.join(root, CONSUMER_MATRIX), 'utf8'),
  ]);
  const inventory = readProjectData(inventoryPath);
  const fixtureCatalog = {};
  const requirements = [
    ...buildCstRequirements(inventory, fixtureCatalog),
    ...buildEmbeddedRequirements(inventory, fixtureCatalog),
    ...buildImporterRequirements(inventory, fixtureCatalog),
    ...buildFourLanguageCorpusRequirements(fixtureCatalog, inventory.extensionDispatch),
    ...buildSemanticRequirements(fixtureCatalog, inventory.extensionDispatch),
    ...buildTransformationRequirements(fixtureCatalog, inventory.extensionDispatch),
    ...buildTranslationRequirements(fixtureCatalog, inventory.extensionDispatch),
    ...buildCrossCuttingRequirements(fixtureCatalog, inventory.extensionDispatch),
    ...buildValidationAndDeliveryRequirements(fixtureCatalog, inventory.extensionDispatch),
    ...buildGateRequirements(fixtureCatalog),
    ...buildLanguageTaskRequirements(fixtureCatalog),
    ...buildCacheCleanupRequirements(fixtureCatalog, {
      requirement, verification, pinnedFixture, source: ISSUE_195_SOURCES.cacheCleanup,
    }),
    ...buildVisionRequirements(fixtureCatalog, {
      requirement, verification, pinnedFixture, source: ISSUE_195_SOURCES.repositoryDirective,
      sourceByRequirement: {
        ...Object.fromEntries(VISION_REQUIREMENTS.filter(({ sourceKey }) => sourceKey)
          .map(({ id, sourceKey }) => [id, ISSUE_195_SOURCES[sourceKey]])),
        ...Object.fromEntries(VISION_REQUIREMENTS.filter(({ id }) => id.startsWith('I195-GLOBAL-CONCEPT-'))
          .map(({ id }) => [id, ISSUE_195_SOURCES.globalConceptSpace])),
        'I195-GRAMMAR-NATURAL-LANGUAGE-REPRESENTATION': ISSUE_195_SOURCES.latestDeliveryAudit,
        'I195-STRING-TRIM-SEMANTICS': ISSUE_195_SOURCES.reverseStringSemantics,
        ...Object.fromEntries(['HERE-DOCUMENTS', 'INTERPOLATED-QUOTES', 'OFFSIDE-LAYOUT', 'FENCED-BLOCKS']
          .map((suffix) => [`I195-GRAMMAR-SCANNER-${suffix}`, ISSUE_195_SOURCES.latestDeliveryAudit])),
        'I195-DEPENDENCY-CURRENT-STABLE-DELIVERY': ISSUE_195_SOURCES.consolidatedChecklist,
        'I195-DOWNSTREAM-RML-PR184-AUDIT': ISSUE_195_SOURCES.consolidatedChecklist,
        'I195-DOWNSTREAM-TYPESCRIPT-TRANSLATIONS': ISSUE_195_SOURCES.consolidatedChecklist,
        // Re-classified to the post-merge checkpoint by the maintainer's CI decision.
        'I195-ACCEPTANCE-REQUIRED-MERGE-CHECK': ISSUE_195_SOURCES.greenPathAudit,
        'I195-ACCEPTANCE-PR-CHECKS-PASSABLE': ISSUE_195_SOURCES.greenPathAudit,
        // The resource audit asks for the native grammars one language at a time.
        'I195-GRAMMAR-NATIVE-JSON': ISSUE_195_SOURCES.resourceAudit,
        'I195-GRAMMAR-NATIVE-INI': ISSUE_195_SOURCES.resourceAudit,
        'I195-GRAMMAR-NATIVE-DIFF': ISSUE_195_SOURCES.resourceAudit,
        'I195-GRAMMAR-NATIVE-CSV': ISSUE_195_SOURCES.resourceAudit,
        'I195-GRAMMAR-NATIVE-JSON5': ISSUE_195_SOURCES.resourceAudit,
        'I195-GRAMMAR-NATIVE-SCHEME': ISSUE_195_SOURCES.resourceAudit,
        'I195-GRAMMAR-NATIVE-RACKET': ISSUE_195_SOURCES.resourceAudit,
        // The green-path audit asks for recovery once, in the executor.
        'I195-GRAMMAR-NATIVE-RECOVERY': ISSUE_195_SOURCES.greenPathAudit,
        'I195-GRAMMAR-NATIVE-C': ISSUE_195_SOURCES.greenPathAudit,
        'I195-GRAMMAR-NATIVE-RUST': ISSUE_195_SOURCES.greenPathAudit,
        'I195-GRAMMAR-NATIVE-JAVASCRIPT': ISSUE_195_SOURCES.greenPathAudit,
        'I195-GRAMMAR-NATIVE-TYPESCRIPT': ISSUE_195_SOURCES.greenPathAudit,
        'I195-GRAMMAR-NATIVE-TSX': ISSUE_195_SOURCES.greenPathAudit,
        'I195-GRAMMAR-NATIVE-LEAN': ISSUE_195_SOURCES.greenPathAudit,
        'I195-GRAMMAR-NATIVE-ROCQ': ISSUE_195_SOURCES.greenPathAudit,
        'I195-GRAMMAR-NATIVE-JAVA': ISSUE_195_SOURCES.greenPathAudit,
        'I195-GRAMMAR-NATIVE-GO': ISSUE_195_SOURCES.greenPathAudit,
        'I195-GRAMMAR-NATIVE-REGEX': ISSUE_195_SOURCES.greenPathAudit,
        'I195-GRAMMAR-NATIVE-GRAPHQL': ISSUE_195_SOURCES.greenPathAudit,
        'I195-GRAMMAR-NATIVE-PROTO': ISSUE_195_SOURCES.greenPathAudit,
        'I195-GRAMMAR-NATIVE-MAKE': ISSUE_195_SOURCES.greenPathAudit,
        'I195-GRAMMAR-NATIVE-SOLIDITY': ISSUE_195_SOURCES.greenPathAudit,
        'I195-GRAMMAR-SHARED-CONCEPTS': ISSUE_195_SOURCES.greenPathAudit,
        'I195-GRAMMAR-CONCEPT-REUSE-REPORT': ISSUE_195_SOURCES.greenPathAudit,
        'I195-GRAMMAR-CONCEPT-TRANSLATION': ISSUE_195_SOURCES.greenPathAudit,
        'I195-NAMING-NATIVE-GRAMMARS': ISSUE_195_SOURCES.greenPathAudit,
        ...Object.fromEntries(VISION_REQUIREMENTS.filter(({ area }) => area === 'resource-limits')
          .map(({ id }) => [id, ISSUE_195_SOURCES.resourceAudit])),
        'I195-RESOURCE-BOUNDED-MEMORY-REGRESSIONS': ISSUE_195_SOURCES.remainingScope,
        'I195-GRAMMAR-SCANNER-DELIMITER-FAMILIES': ISSUE_195_SOURCES.remainingScope,
        'I195-GRAMMAR-SCANNER-COUNTED-DELIMITERS': ISSUE_195_SOURCES.remainingScope,
        'I195-GRAMMAR-NATIVE-LUA': ISSUE_195_SOURCES.remainingScope,
        'I195-GRAMMAR-NATIVE-HCL': ISSUE_195_SOURCES.remainingScope,
        'I195-GRAMMAR-NATIVE-R': ISSUE_195_SOURCES.remainingScope,
        'I195-GRAMMAR-NATIVE-ODIN': ISSUE_195_SOURCES.remainingScope,
        'I195-GRAMMAR-NATIVE-AGDA': ISSUE_195_SOURCES.remainingScope,
        'I195-GRAMMAR-NATIVE-HTML': ISSUE_195_SOURCES.remainingScope,
        'I195-GRAMMAR-SCANNER-STATE-SEMANTICS': ISSUE_195_SOURCES.remainingScope,
        'I195-GRAMMAR-NATIVE-XML': ISSUE_195_SOURCES.remainingScope,
        'I195-GRAMMAR-NATIVE-DTD': ISSUE_195_SOURCES.remainingScope,
        'I195-GRAMMAR-NATIVE-PYTHON': ISSUE_195_SOURCES.remainingScope,
        // The next-phase audit asks for JavaScript first, self-translation, decorators and bulk
        // grammar work; the audit of fe9b6ff1 makes the CI order and declared settling binding.
        ...Object.fromEntries(VISION_REQUIREMENTS.filter(({ area }) => area === 'javascript-first')
          .map(({ id }) => [id, ISSUE_195_SOURCES.nextPhaseAudit])),
        ...Object.fromEntries(VISION_REQUIREMENTS.filter(({ area }) => area === 'continuous-integration')
          .map(({ id }) => [id, ISSUE_195_SOURCES.remainingWorkAudit])),
        'I195-GRAMMAR-BULK-PIPELINE': ISSUE_195_SOURCES.nextPhaseAudit,
        'I195-GRAMMAR-DECLARED-SETTLING': ISSUE_195_SOURCES.remainingWorkAudit,
        // The audit of 6676c132 asks for the parse memory budget, the acceptance stages after
        // JavaScript, one gate error per skipped stage, real merging and carried = 0.
        ...Object.fromEntries(['I195-RESOURCE-PARSE-MEMORY-BUDGET', 'I195-CI-ACCEPTANCE-AFTER-JAVASCRIPT',
          'I195-CI-SKIPPED-STAGE-ONE-GATE-ERROR', 'I195-MERGE-REAL-RECONCILIATION', 'I195-SELF-TRANSLATION-CARRIED-ZERO']
          .map((id) => [id, ISSUE_195_SOURCES.recoveryAudit])),
        // The audit of c39470d0, re-posted to deliver it in bulk, makes every row that is still open work this PR delivers.
        ...Object.fromEntries(FINISH_THE_WORK_ROWS.map((id) => [id, ISSUE_195_SOURCES.bulkDeliveryRepost])),
      },
    }),
  ];
  const consumersByRow = downstreamByRow(parseConsumerMatrix(consumerMatrixText));
  for (const entry of requirements) {
    entry.traceability = traceabilityFor(entry, { fixtureCatalog, register, evidenceGroupFor, consumersByRow });
  }

  const digests = {
    'parity/language-grammar-inventory.json': sha256(inventoryText),
    'parity/fixtures/four-language-conformance.json': sha256(corpusText),
  };
  for (const fixture of Object.values(fixtureCatalog)) {
    if (fixture.path && !digests[fixture.path]) {
      try {
        digests[fixture.path] = sha256(readProjectFileBytes(path.join(root, fixture.path)));
      } catch {
        // The manifest keeps a null digest so the aggregate gate fails closed.
      }
    }
    if (fixture.path && digests[fixture.path]) fixture.sha256 = digests[fixture.path];
  }

  return {
    schemaVersion: ISSUE_195_MANIFEST_SCHEMA_VERSION,
    issue: 195,
    generatedBy: 'js/scripts/check-issue-195-acceptance.mjs --refresh-manifest',
    sources: ISSUE_195_SOURCES,
    checkpoints: {
      'pre-merge':
        'All implementation and candidate-artifact requirements must pass on the exact merge candidate. Every PR check must pass. A red check is a defect to fix now; unfinished requirements are work to implement, not a reason for red CI.',
      'post-merge':
        'Verification of state that only exists after merge (live default-branch rules) runs on main as a non-blocking report; a failure is fixed in a follow-up commit and never blocks a pull request.',
      'release-delivery':
        'Only verification that inherently requires published artifacts may run here; delivery remains incomplete until it passes.',
    },
    resultPolicy: {
      acceptedOutcome: 'passed',
      forbiddenOutcomes: ['missing', 'open', 'partial', 'unsupported', 'skipped', 'todo', 'xfail'],
      exactCommitRequired: true,
      positiveEvidenceRequired: true,
      capabilityDeclarationsAreNotEvidence: true,
    },
    fixtureCatalog,
    atomicRequirements: requirements.sort((left, right) => left.id.localeCompare(right.id)),
  };
}
