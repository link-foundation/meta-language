import { createHash } from 'node:crypto';
import { readFile } from 'node:fs/promises';
import path from 'node:path';

import { buildCacheCleanupRequirements } from './issue-195-cache-requirements.mjs';
import { DOCUMENT_TASK_CASES, DOCUMENT_TASK_EXAMPLE } from './issue-195-delivery.mjs';
import { CONSUMER_MATRIX, downstreamByRow, parseConsumerMatrix } from './issue-195-downstream.mjs';
import { evidenceGroupFor } from './issue-195-evidence-plan.mjs';
import { VISION_REQUIREMENTS, buildVisionRequirements, traceabilityFor } from './issue-195-vision-requirements.mjs';
import { readProjectData, readProjectFileBytes } from './project-data.mjs';

export const ISSUE_195_MANIFEST_SCHEMA_VERSION = 1;

export const ISSUE_195_SOURCES = Object.freeze({
  issue: 'https://github.com/link-foundation/meta-language/issues/195',
  htmlPackageDelivery: 'https://github.com/link-foundation/meta-language/issues/230',
  documentTasks: 'https://github.com/link-foundation/meta-language/issues/245',
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
        expectedBehavior: 'Both PDF parse entry points preserve the original file and expose Flate- and LZW-decoded content operators in linked documents with the encoded range. LZW supports both standard code-width transitions and chained ASCIIHex decoding. An option disables stream decoding and corrupt encoded data identifies the affected range. Declared page and form content keeps a decoded tree and encoded range when empty or containing only comments or inline images.',
        selector: 'Flate and LZW page content, both binary input modes, disabled decoding, both LZW EarlyChange values, a chained filter, corrupt data, and declared page and form streams without text operators',
        assertions: ['pdfDerivedOperatorsKeepEncodedRange', 'pdfFiltersCanBeDisabled', 'corruptPdfFilterKeepsLocatedError', 'pdfLzwCodeWidthsAndFilterChainsKeepEncodedRange', 'corruptPdfLzwKeepsLocatedError', 'pdfDeclaredContentWithoutTextKeepsEncodedRanges'],
      },
      {
        id: 'I195-I239-PDF-FONT-UNICODE',
        construct: 'font Unicode mappings and decoded visible text in page order',
        expectedBehavior: 'The visible PDF projection follows the page tree and show-text operators, maps glyph codes through ToUnicode CMaps and simple-font encodings, and returns the covering original stream or literal source range. Explicitly positioned horizontal lines follow their page coordinates while retaining their original source ranges, including after page rotation, scoped graphics transforms, and nested form matrices. Tagged text follows marked-content order from the page structure tree while retaining the original stream or literal ranges.',
        selector: 'reverse physical page order, an inherited Identity-H font with bfchar and bfrange, a WinAnsi Difference, horizontal text lines drawn out of reading order, scoped matrices across content streams and nested forms, inherited page rotation, and a structure tree with tagged page and form text, named properties, nested sequences, and an untagged footer',
        assertions: ['pdfTextUsesFontMappingsAndPageOrder', 'pdfVisibleTextKeepsEncodedAndLiteralRanges', 'pdfPositionedLinesKeepReadingOrderAndRanges', 'pdfTransformedLinesKeepReadingOrderAndRanges', 'pdfTaggedSequencesKeepReadingOrderAndRanges'],
      },
      {
        id: 'I195-I239-PDF-INHERITED-UNICODE-MAPS',
        construct: 'Unicode mappings and code spaces inherited from embedded character maps',
        expectedBehavior: 'Embedded UseCMap streams supply base glyph mappings and code spaces under the stream decoding policy, with nearer mappings taking precedence. The projection checks each byte against its corresponding code-space bounds and preserves the original operand range.',
        selector: 'nested and Flate-compressed ToUnicode base maps, variable-width codes and per-byte ranges, and missing and circular base references',
        assertions: ['pdfUnicodeMapsInheritBaseStreamsAndKeepLiteralRanges'],
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
    {
      id: 'I195-I235-UNICODE-SEARCH-BOUNDARIES',
      source: ISSUE_195_SOURCES.grammarSearch,
      language: 'native grammar execution',
      version: 'Unicode 17.0.0 General_Category',
      construct: 'pinned Unicode word boundaries in grammar search',
      expectedBehavior: 'Both executors use the Unicode 17.0 letters, marks, numbers and connector punctuation to identify word interiors. Whole-word and unrestricted search retain their original source coordinates and return the expected set of matches.',
      fixture: 'parity/fixtures/grammar-search-unicode-boundaries.lino',
      selector: 'whole-word and unrestricted match ranges around a supplementary-plane letter, a combining mark and two connector punctuation characters',
      assertions: ['pinnedWordBoundariesKeepSourceSpans'],
      implementationEntryPoints: {
        javascript: ['js/src/grammar-runtime.js', 'js/src/text-location-character-classes.js', 'js/tests/grammar-search.test.js'],
        rust: ['rust/src/grammar/feature_runtime/search.rs', 'rust/src/link_network/text_location.rs', 'rust/tests/unit/issue_195_grammar_feature_union.rs'],
      },
    },
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
      id: 'I195-I236-OPEN-VOCABULARY-MORPHOLOGY-LAZY-DICTIONARY',
      source: ISSUE_195_SOURCES.openVocabularyMorphology,
      language: 'English and Russian morphology with provisioned OpenCorpora data',
      version: 'OpenCorpora source revision 417150, corpus revision 4580142, compiled dictionary format 2.4',
      construct: 'language-specific dictionary loading',
      expectedBehavior: 'A fresh process annotates English text without opening its configured Russian dictionary. A Russian request in the same process attempts to open the configured dictionary and reports the unusable path.',
      fixture: 'parity/fixtures/natural-language-morphology-lazy-loading.lino',
      selector: 'an English plural, a Russian form and an unusable configured directory',
      assertions: ['configuredDictionaryLoadsOnlyForItsLanguage'],
      implementationEntryPoints: {
        javascript: ['js/src/morphology.js', 'js/tests/natural-language-morphology.test.js'],
        rust: ['rust/src/link_network/morphology/mod.rs', 'rust/tests/unit/natural_language_morphology.rs'],
      },
    },
    {
      id: 'I195-I236-ENGLISH-DICTIONARY-MORPHOLOGY',
      source: ISSUE_195_SOURCES.openVocabularyMorphology,
      language: 'English morphology',
      version: 'Universal Dependencies v2, AGID 2016.01.19 and WordNet 3.1',
      construct: 'dictionary readings and inverse inflection',
      expectedBehavior: 'English words receive scored attested readings, including ambiguous ones, at their exact source spans. Suffix predictions remain available outside the dictionary. Inflection and cardinal agreement select attested forms for the requested features.',
      fixture: 'parity/fixtures/english-morphology.lino',
      selector: 'every English dictionary, function word and invented word reading, every linked span and alternative, and every inverse form and cardinal count',
      assertions: ['englishDictionaryReadingsRetainAlternatives', 'englishMorphologyLinksRetainSourceSpans', 'englishInflectionAndNumberAgreement'],
      implementationEntryPoints: {
        javascript: ['js/src/morphology.js', 'js/src/morphology-english.js', 'js/tests/natural-language-morphology.test.js'],
        rust: ['rust/src/link_network/morphology/mod.rs', 'rust/src/link_network/morphology/english.rs', 'rust/tests/unit/natural_language_morphology.rs'],
      },
    },
    {
      id: 'I195-I240-JSON-SCHEMA',
      source: ISSUE_195_SOURCES.jsonSchema,
      language: 'JSON with draft 2020-12 schemas',
      version: 'RFC 8259 JSON, RFC 6901 pointer and JSON Schema draft 2020-12 semantics',
      construct: 'schema import, decoder grammar export, located validation and reverse inference',
      expectedBehavior: 'Import named and referenced schema constraints and emit interoperable decoder grammars. Lower exclusive alternatives with disjoint instance types exactly and report unsupported constraints. Match supported string patterns against decoded Unicode scalars regardless of their JSON escape spelling. Validation preserves syntax links, UTF-8 spans and pointers at schema violations and malformed JSON. Emit the original schema or infer a schema from bounded finite productions. Date, date-time, UUID and email decoder rules recognize their supported formats across direct and escaped JSON spellings. A pattern surrogate-pair escape matches one Unicode scalar, including in character ranges and repetitions. Object constants and enumerated objects within the decoder member limit accept permutations with the same keys and values, including nested objects. Overlapping integer alternatives admit integer spellings that match exactly one interval and report alternate decimal and exponent spellings. Object alternatives are exact when disjoint allowed strings for a shared required property distinguish each pair, and retain diagnostics when optional or overlapping tags cannot distinguish the branches. Recognized string formats combine with supported regular patterns over decoded strings, with an impossible conjunction producing no generatable grammar. Finite local references and nested alternatives enforce supported sibling assertions and exclusive branch counts. Validation uses the original JSON number spelling for integer types, four numeric bounds, and numbers in constants and enumerations, with precise UTF-8 spans. Sibling type assertions restrict references and alternatives, including recursive references and the integer subset of numbers. Exclusive branch tests apply inside the restricted types. Supported overlapping string alternatives count exactly one match across patterns, formats and finite values, including sibling string constraints. String-only local references and inclusive alternatives intersect supported sibling patterns and formats with referenced and branch languages, with diagnosed fallback at the established work bound. Bounded local reference chains enforce regular string constraints and participate in exact alternatives distinguished by string languages or required object tags. Numeric decoder limits compose through local reference chains and constrain numeric inclusive alternatives and exclusive integer intervals. Object-only reference chains combine required names with a single property layout, including its additional-property policy. Array-only chains take the strongest length bounds around a single item layout. Impossible combinations have no generatable collection and unsupported layout intersections report a validation diagnostic. With a single property or item layout per branch, sibling length bounds and required names apply to every inclusive branch and to exclusive branches with disjoint instance types or required tags. A closed array tail bounds the decoder by the prefix length in the same schema object, including an empty prefix and impossible positive minima across references and alternatives. Object, array, string and numeric sibling assertions also apply within their instance types in mixed-type or untyped local references and inclusive alternatives, and in exclusive alternatives with disjoint referenced instance types. Compatible object layouts combine declared fields and apply each additional-property schema only to names outside its own declarations. Compatible array layouts compose position by position, using each layer\'s own prefix or tail schema. A forbidden item or disjoint declared types end the generatable prefix before the forbidden position. Additional-property defaults do not re-admit a declared property with disjoint types. Supported scalar assertions compose across property and item schemas, including local references to regular string constraints and inclusive numeric bounds. The same conjunction applies to additional property values, and type-scoped assertions permit unrelated instance types. When one property or item layer admits finitely many values, the decoder checks every candidate against all contributing schemas, including exclusive local alternatives, type-scoped numeric bounds and assertions on object members. An empty intersection forbids the property or item. Nested property and item collections combine compatible object and array layouts within a shared construction bound, with reported fallback on recursive intersections that reach the bound. Exclusive array alternatives with disjoint length intervals apply common sibling assertions, including intervals bounded by closed prefixes on local references. Overlapping length intervals with a shared item layout generate only lengths covered by exactly one branch. Unresolved intersections between different item layouts retain the exclusivity diagnostic. Exclusive array alternatives with overlapping lengths apply common sibling assertions when disjoint string sets distinguish a required position across local reference layers. An optional prefix cannot serve as that proof. Overlapping number-only alternatives apply joined limits and accept decimal spellings covered by exactly one branch across inclusive and open endpoints. When the exclusive intervals cover all numbers, exponent spellings remain generatable. Mixed integer and number alternatives count both kinds of branch for integral values, including zero-padded decimal spellings, and count only number branches for non-integral values. Supported string and numeric exclusivity applies independently within their instance types when other JSON instance types have separate alternatives.',
      fixture: 'parity/fixtures/json-schema-and-recovery.lino',
      selector: 'answer schema, closed records, tuple bounds, local references, formats, integer ranges, finite and disjoint exclusive alternatives, unsupported keywords, malformed answers, inferred constants, anchored and unanchored patterns with raw and escaped Unicode, four formats with direct and escaped JSON spellings, regular expression surrogate pairs in ranges and repetitions alongside escaped backslash controls, and reordered objects in constants and enumerations alongside changed-key and changed-value controls, and overlapping inclusive, exclusive and unbounded integer intervals with equivalent numeric spellings, and object alternatives with required and optional discriminating properties, and intersections of calendar, time zone, UUID and email formats with regular patterns and impossible, escaped and nullable-choice controls, alongside finite references with restrictive siblings and nested exclusive alternatives with shared values, and standalone and nested decimal equality, bounds, underflow, negative zero and integer semantics near host rounding boundaries, and differently typed references to the same schema, recursive members, unrestricted and finite branches, exclusive alternatives and impossible type intersections, and overlapping regular string branches with sibling formats, finite values, escaped equivalents and empty exclusive languages, and sibling assertions on local string references and inclusive choices, unrestricted exclusive branches, empty intersections and bounded residual validation, and local reference chains with sibling string constraints and disjoint or overlapping required string tags, plus stricter linked integer and decimal bounds, numeric union and exclusive interval overlap, empty intersections and spellings reserved for validation, and array length and required object properties on local reference chains with closed, typed-open, and conflicting-constraint controls, and common collection bounds or required fields across inclusive and disjoint exclusive alternatives, plus closed tails next to array references and alternatives with incompatible prefix schemas and impossible lower bounds, followed by mixed-type and undeclared-type reference and alternative constraints with accepted unrelated types, plus combined object layouts with duplicate and disjoint declarations, local typed and closed additional properties, impossible required names and diagnosed incompatible constraints, followed by compatible array positions and tails, impossible reachable positions, unreachable conflicting schemas and disjoint property types, followed by shared property and item regular patterns, linked scalar constraints, additional property values, untyped scalar assertions, bounded tails and alternative residuals, followed by finite Boolean, numeric and object values, overlapping exclusive branches, escaped names and values, and a prohibited required property, plus arrays of intersected object layouts, object properties with intersected array layouts, an impossible required nested array, and bounded recursive fallback, then linked exclusive arrays with disjoint lengths and an independent null branch, plus overlapping length intervals with shared items and document validation controls for different item layouts, followed by overlapping lengths with disjoint required prefix strings and an optional-prefix overlap control, then linked overlapping decimal intervals with inclusive and open endpoints, negative zero, singleton and full-range exponent controls, and an impossible duplicate interval, then mixed linked and unbounded numeric alternatives with integral decimal spellings, non-integral values, signed zero and an impossible mixed interval, then projected exclusive alternatives across string, number, Boolean and null instance types',
      assertions: ['decoderGrammarsConstrainSchema', 'schemaDiagnosticsRetainSyntaxLinks', 'grammarAndSchemaRoundTrip', 'decoderDisjointExclusiveBranchesHaveNoResidual', 'decoderPatternsRespectDecodedStrings', 'decoderFormatsRespectDecodedStrings', 'decoderPatternsCombineEscapedSurrogatePairs', 'decoderObjectConstantsAcceptMemberPermutations', 'decoderExclusiveIntegerAlternativesRejectOverlap', 'decoderRequiredStringTagsDistinguishAlternatives', 'decoderFormatsCombineWithRegularPatterns', 'decoderChecksFiniteReferencedAlternatives', 'validationUsesExactNumericSpellings', 'decoderAppliesSiblingTypeAssertions', 'decoderSelectsExclusiveRegularStrings', 'decoderIntersectsSiblingRegularStrings', 'decoderResolvesLinkedAlternatives', 'decoderCombinesLinkedNumericBounds', 'decoderCombinesLinkedCollectionAssertions', 'decoderAppliesSiblingCollectionAssertionsToAlternatives', 'decoderBoundsClosedArrayTailsByTheirOwnPrefix', 'decoderAppliesLinkedAssertionsWithinInstanceTypes', 'decoderCombinesCompatibleObjectPropertyLayouts', 'decoderCombinesCompatibleArrayItemLayouts', 'decoderIntersectsScalarPropertyAndItemAssertions', 'decoderFiltersFinitePropertyAndItemCandidates', 'decoderIntersectsNestedCollectionLayouts', 'decoderDistinguishesExclusiveArrayLengthIntervals', 'decoderDistinguishesRequiredArrayPrefixValues', 'decoderExclusiveDecimalAlternativesRespectEndpoints', 'decoderDistinguishesIntegerValuesWithinNumberAlternatives', 'decoderChecksExclusiveAlternativesWithinInstanceTypes', 'decoderCountsFiniteNumericBranchesInExclusiveAlternatives', 'decoderChecksBooleanAndNullAlternativeValues'],
      implementationEntryPoints: {
        javascript: ['js/src/grammar-importers/json-schema.js', 'js/src/json-schema-validation.js', 'js/tests/grammar-json-document.test.js', 'js/src/grammar-importers/json-schema-string-intersection.js', 'js/src/json-number-spelling.js', 'js/src/data/json-document/schema.lino', 'js/src/grammar-importers/json-schema-numeric-ranges.js'],
        rust: ['rust/src/grammar/json_document/import.rs', 'rust/src/grammar/json_document/validation.rs', 'rust/tests/unit/json_schema_and_recovery.rs', 'rust/src/grammar/json_document/string_intersection.rs', 'rust/src/grammar/json_document/number_spelling.rs', 'rust/src/data/json-document/schema.lino', 'rust/src/grammar/json_document/numeric_ranges.rs'],
      },
    },
    {
      id: 'I195-I240-JSON-SCHEMA-FINITE-COLLECTIONS',
      source: ISSUE_195_SOURCES.jsonSchema,
      language: 'JSON with draft 2020-12 schemas',
      version: 'RFC 8259 JSON, RFC 6901 pointer and JSON Schema draft 2020-12 semantics',
      construct: 'exact exclusive alternatives over bounded finite collections',
      expectedBehavior: 'Within the shared construction limits, enumerate the values of closed object and bounded array branches with finite member schemas. Apply common sibling assertions and require exactly one exclusive alternative to accept each value, preserving object member permutations. Project no generatable production when every value overlaps.',
      fixture: 'parity/fixtures/json-schema-and-recovery.lino',
      selector: 'required and optional object fields, variable array lengths, local references, simultaneous inclusive and exclusive alternatives, nested collections, escaped and reordered object keys, and duplicate impossible alternatives',
      assertions: ['decoderCountsOverlappingFiniteCollections'],
      implementationEntryPoints: {
        javascript: ['js/src/grammar-importers/json-schema.js', 'js/src/data/json-document/schema.lino', 'js/tests/grammar-json-document.test.js'],
        rust: ['rust/src/grammar/json_document/import.rs', 'rust/src/data/json-document/schema.lino', 'rust/tests/unit/json_schema_and_recovery.rs'],
      },
    },
    {
      id: 'I195-I240-JSON-SCHEMA-NUMERIC-CONSTANTS',
      source: ISSUE_195_SOURCES.jsonSchema,
      language: 'JSON with draft 2020-12 schemas',
      version: 'RFC 8259 JSON and JSON Schema draft 2020-12 semantics',
      construct: 'numeric value equality across JSON spellings in constrained decoders',
      expectedBehavior: 'Decode the ordinary decimal representation and bounded exponent shifts of each numeric constant or enumeration member, including numbers inside arrays and objects. Accept all valid zero spellings. Reject different numeric values and invalid JSON number syntax. Report the remaining shifts for original-token validation.',
      fixture: 'parity/fixtures/json-schema-and-recovery.lino',
      selector: 'numeric constants, signed zero, scaled fractions and integers, nested enumerations, invalid numeric syntax and decoded versus validated residual spellings',
      assertions: ['decoderNumericConstantsRespectEquivalentSpellings'],
      implementationEntryPoints: {
        javascript: ['js/src/grammar-importers/json-schema.js', 'js/src/grammar-importers/json-schema-numeric-ranges.js', 'js/tests/grammar-json-document.test.js'],
        rust: ['rust/src/grammar/json_document/import.rs', 'rust/src/grammar/json_document/number_spelling.rs', 'rust/tests/unit/json_schema_and_recovery.rs'],
      },
    },
    {
      id: 'I195-I240-JSON-SCHEMA-EXCLUSIVE-ARRAY-LENGTHS',
      source: ISSUE_195_SOURCES.jsonSchema,
      language: 'JSON with draft 2020-12 schemas',
      version: 'RFC 8259 JSON and JSON Schema draft 2020-12 semantics',
      construct: 'exclusive alternatives over array length intervals with a shared item layout',
      expectedBehavior: 'With a shared item layout, generate arrays whose lengths match exactly one local or referenced branch, subject to common sibling assertions. Apply the item layout to every generated member. Count closed prefixes as upper bounds in their own schema layers. Duplicate alternatives with the same unbounded item language have no generatable production.',
      fixture: 'parity/fixtures/json-schema-exclusive-array-lengths.lino',
      selector: 'two-way and three-way interval overlap, sibling item and length guards, local reference layers, closed prefixes, unbounded tails, impossible intervals, wrong item types, and duplicate alternatives',
      assertions: ['decoderCountsExclusiveArrayLengthIntervals', 'duplicateArrayAlternativesHaveNoGeneratableDocument'],
      implementationEntryPoints: {
        javascript: ['js/src/grammar-importers/json-schema.js', 'js/tests/grammar-json-document.test.js'],
        rust: ['rust/src/grammar/json_document/import.rs', 'rust/tests/unit/json_schema_and_recovery.rs'],
      },
    },
    {
      id: 'I195-I240-JSON-SCHEMA-EXCLUSIVE-TYPED-ARRAY-ITEMS',
      source: ISSUE_195_SOURCES.jsonSchema,
      language: 'JSON with draft 2020-12 schemas',
      version: 'RFC 8259 JSON and JSON Schema draft 2020-12 semantics',
      construct: 'exclusive alternatives over homogeneous arrays with disjoint item types',
      expectedBehavior: 'For prefix-free arrays, distinguish nonempty arrays by disjoint item types and generate positive lengths covered by exactly one branch with the same item layout. Generate the empty array when exactly one branch accepts zero items. Apply a common sibling item schema to otherwise unrestricted branches.',
      fixture: 'parity/fixtures/json-schema-exclusive-typed-array-items.lino',
      selector: 'string, number and Boolean item layouts, local array and item references, overlaps within one item layout, common item constraints, mixed item types and the empty array',
      assertions: ['decoderDistinguishesDisjointArrayItemTypes', 'decoderCountsEmptyArrayBranchMembership'],
      implementationEntryPoints: {
        javascript: ['js/src/grammar-importers/json-schema.js', 'js/tests/grammar-json-document.test.js'],
        rust: ['rust/src/grammar/json_document/import.rs', 'rust/tests/unit/json_schema_and_recovery.rs'],
      },
    },
    {
      id: 'I195-I240-JSON-SCHEMA-EXCLUSIVE-ARRAY-PREFIX-TYPES',
      source: ISSUE_195_SOURCES.jsonSchema,
      language: 'JSON with draft 2020-12 schemas',
      version: 'RFC 8259 JSON and JSON Schema draft 2020-12 semantics',
      construct: 'exclusive alternatives over array layouts with disjoint first-item types',
      expectedBehavior: 'For nonempty arrays, use the disjoint instance types at the first position to separate item layouts, and generate positive lengths covered by exactly one branch within each layout. Count empty array membership across all branches, including a branch with an impossible first item.',
      fixture: 'parity/fixtures/json-schema-exclusive-typed-array-prefixes.lino',
      selector: 'optional typed prefixes, referenced first-item schema, a recurring item constraint, sibling length guards, overlapping branches with a shared prefix and an impossible prefix',
      assertions: ['decoderDistinguishesDisjointArrayPrefixTypes'],
      implementationEntryPoints: {
        javascript: ['js/src/grammar-importers/json-schema.js', 'js/tests/grammar-json-document.test.js'],
        rust: ['rust/src/grammar/json_document/import.rs', 'rust/tests/unit/json_schema_and_recovery.rs'],
      },
    },
    {
      id: 'I195-I240-JSON-SCHEMA-EXCLUSIVE-ARRAY-ITEMS',
      source: ISSUE_195_SOURCES.jsonSchema,
      language: 'JSON with draft 2020-12 schemas',
      version: 'RFC 8259 JSON and JSON Schema draft 2020-12 semantics',
      construct: 'exclusive array alternatives with overlapping item domains',
      expectedBehavior: 'Within the shared construction bound, group candidate item values by the branches that accept them. Apply sibling item assertions and preserve only those branches for subsequent positions. Allow closing the array only when exactly one surviving branch accepts the length. Preserve shared item productions and unbounded tails. Equivalent item languages and lengths have no generatable exclusive document.',
      fixture: 'parity/fixtures/json-schema-exclusive-array-items.lino',
      selector: 'overlapping enums, empty arrays, escaped values, bounded and unbounded lengths, optional prefixes, impossible positions, shared unbounded item schemas, local references, parent item assertions and equivalent branches',
      assertions: ['decoderCountsExclusiveArrayItemMembership', 'equivalentArrayItemLanguagesHaveNoGeneratableDocument'],
      implementationEntryPoints: {
        javascript: ['js/src/grammar-importers/json-schema.js', 'js/src/data/json-document/schema.lino', 'js/tests/grammar-json-document.test.js'],
        rust: ['rust/src/grammar/json_document/import/exclusive_alternatives.rs', 'rust/src/data/json-document/schema.lino', 'rust/tests/unit/json_schema_and_recovery.rs'],
      },
    },
    {
      id: 'I195-I240-JSON-SCHEMA-EXCLUSIVE-MEMBER-TYPES',
      source: ISSUE_195_SOURCES.jsonSchema,
      language: 'JSON with draft 2020-12 schemas',
      version: 'RFC 8259 JSON and JSON Schema draft 2020-12 semantics',
      construct: 'exclusive arrays and objects with overlapping member type declarations',
      expectedBehavior: 'Group base JSON member productions by the branches accepting each type. Intersect type declarations across local references and with siblings. Apply the same membership for every item or declared property and enforce exactly one matching collection branch. Emit no document when the alternatives have equivalent member type languages and collection boundaries.',
      fixture: 'parity/fixtures/json-schema-exclusive-member-types.lino',
      selector: 'overlapping Boolean, null, string, object, array and number domains, exponent notation, local references, sibling types, variable lengths, absent and required properties, and equivalent alternatives',
      assertions: ['decoderCountsExclusiveCollectionMemberTypes', 'equivalentMemberTypesHaveNoGeneratableDocument'],
      implementationEntryPoints: {
        javascript: ['js/src/grammar-importers/json-schema.js', 'js/tests/grammar-json-document.test.js'],
        rust: ['rust/src/grammar/json_document/import/exclusive_alternatives.rs', 'rust/tests/unit/json_schema_and_recovery.rs'],
      },
    },
    {
      id: 'I195-I240-JSON-SCHEMA-EXCLUSIVE-MEMBER-PATTERNS',
      source: ISSUE_195_SOURCES.jsonSchema,
      language: 'JSON with draft 2020-12 schemas',
      version: 'RFC 8259 JSON and JSON Schema draft 2020-12 semantics',
      construct: 'exclusive collection alternatives with overlapping regular member constraints',
      expectedBehavior: 'Within the shared construction budgets, partition string item and property languages by their matching branches. Apply sibling and local reference constraints and combine their membership with unrestricted alternative instance types. Generate a collection only when exactly one branch accepts its members, presence and length. Equivalent regular member languages and collection boundaries have no generatable exclusive document.',
      fixture: 'parity/fixtures/json-schema-exclusive-member-patterns.lino',
      selector: 'overlapping patterns, supported string formats, string constants, sibling and local reference constraints, optional and required properties, mixed and undeclared instance types, escaped strings, variable lengths and equivalent regular languages',
      assertions: ['decoderCountsExclusiveCollectionMemberPatterns', 'equivalentMemberPatternsHaveNoGeneratableDocument'],
      implementationEntryPoints: {
        javascript: ['js/src/grammar-importers/json-schema.js', 'js/src/grammar-importers/json-schema-string-intersection.js', 'js/tests/grammar-json-document.test.js'],
        rust: ['rust/src/grammar/json_document/import.rs', 'rust/src/grammar/json_document/import/exclusive_alternatives.rs', 'rust/src/grammar/json_document/string_intersection.rs', 'rust/tests/unit/json_schema_and_recovery.rs'],
      },
    },
    {
      id: 'I195-I240-JSON-SCHEMA-EXCLUSIVE-MEMBER-BOUNDS',
      source: ISSUE_195_SOURCES.jsonSchema,
      language: 'JSON with draft 2020-12 schemas',
      version: 'RFC 8259 JSON and JSON Schema draft 2020-12 semantics',
      construct: 'exclusive collection alternatives with overlapping numeric member bounds',
      expectedBehavior: 'Within the shared construction budgets, partition decimal number and integer spellings by exact member acceptance. Apply sibling and local reference bounds, preserve unrelated instance types, and close a collection only when one branch accepts its members, presence and length. Validate exponent spellings against the same schema and report the decimal decoder restriction at the constrained member. Equivalent numeric member languages and collection boundaries have no generatable exclusive document.',
      fixture: 'parity/fixtures/json-schema-exclusive-member-bounds.lino',
      selector: 'open and closed endpoints, signed zero, mixed integer and number alternatives, optional and required fields, local references, sibling bounds, repeated items, overlapping lengths, exponent spellings and equivalent numeric languages',
      assertions: ['decoderCountsExclusiveCollectionMemberBounds', 'validationChecksAdditionalNumericSpellings', 'equivalentMemberBoundsHaveNoGeneratableDocument'],
      implementationEntryPoints: {
        javascript: ['js/src/grammar-importers/json-schema.js', 'js/src/grammar-importers/json-schema-numeric-ranges.js', 'js/tests/grammar-json-document.test.js'],
        rust: ['rust/src/grammar/json_document/import.rs', 'rust/src/grammar/json_document/import/exclusive_alternatives.rs', 'rust/src/grammar/json_document/import/numeric_alternatives.rs', 'rust/src/grammar/json_document/numeric_ranges.rs', 'rust/tests/unit/json_schema_and_recovery.rs'],
      },
    },
    {
      id: 'I195-I240-JSON-SCHEMA-EXCLUSIVE-MIXED-MEMBERS',
      source: ISSUE_195_SOURCES.jsonSchema,
      language: 'JSON with draft 2020-12 schemas',
      version: 'RFC 8259 JSON and JSON Schema draft 2020-12 semantics',
      construct: 'exclusive collection alternatives with both numeric bounds and regular string constraints on the same members',
      expectedBehavior: 'Within the shared construction budgets, combine number and string productions that have the same branch membership. Include unrestricted types according to their declarations. Apply sibling and local reference constraints to each partition. Admit a collection only when one branch accepts all its members, presence and length. Locate the decimal decoder restriction and validate equivalent exponent spellings. Emit no document for alternatives with equivalent mixed member languages and collection boundaries.',
      fixture: 'parity/fixtures/json-schema-exclusive-mixed-members.lino',
      selector: 'overlapping decimal intervals and regular patterns, distinct integer and number memberships, supported date format, escaped spellings, independent Boolean and null domains, sibling assertions, local references, repeated items, equivalent exponents and equivalent alternatives',
      assertions: ['decoderCombinesExclusiveCollectionScalarMembership', 'validationChecksMixedMemberExponentSpellings', 'equivalentMixedMemberLanguagesHaveNoGeneratableDocument'],
      implementationEntryPoints: {
        javascript: ['js/src/grammar-importers/json-schema.js', 'js/src/grammar-importers/json-schema-numeric-ranges.js', 'js/src/grammar-importers/json-schema-string-intersection.js', 'js/tests/grammar-json-document.test.js'],
        rust: ['rust/src/grammar/json_document/import/exclusive_alternatives.rs', 'rust/src/grammar/json_document/import/numeric_alternatives.rs', 'rust/src/grammar/json_document/numeric_ranges.rs', 'rust/src/grammar/json_document/string_intersection.rs', 'rust/tests/unit/json_schema_and_recovery.rs'],
      },
    },
    {
      id: 'I195-I240-JSON-SCHEMA-EXCLUSIVE-OBJECT-PROPERTY',
      source: ISSUE_195_SOURCES.jsonSchema,
      language: 'JSON with draft 2020-12 schemas',
      version: 'RFC 8259 JSON and JSON Schema draft 2020-12 semantics',
      construct: 'exclusive alternatives with a shared object layout and one finite property domain per branch',
      expectedBehavior: 'Generate a declared property value only when exactly one branch accepts that value. Require the property even if each branch allows its absence. Apply all common property and additional-property assertions. Report an empty exclusive language when alternatives have identical layouts and constraints or accept the same finite property values.',
      fixture: 'parity/fixtures/json-schema-exclusive-object-property.lino',
      selector: 'overlapping string enumerations with unrestricted siblings, escaped spellings, optional Boolean and null branches, additional-property assertions, referenced object and property schemas, and identical finite domains',
      assertions: ['decoderCountsExclusiveObjectPropertyValues', 'duplicateObjectAlternativesHaveNoGeneratableDocument'],
      implementationEntryPoints: {
        javascript: ['js/src/grammar-importers/json-schema.js', 'js/tests/grammar-json-document.test.js'],
        rust: ['rust/src/grammar/json_document/import.rs', 'rust/tests/unit/json_schema_and_recovery.rs'],
      },
    },
    {
      id: 'I195-I240-JSON-SCHEMA-EXCLUSIVE-OBJECT-SIBLINGS',
      source: ISSUE_195_SOURCES.jsonSchema,
      language: 'JSON with draft 2020-12 schemas',
      version: 'RFC 8259 JSON and JSON Schema draft 2020-12 semantics',
      construct: 'parent object assertions beside exclusive alternatives with a shared layout',
      expectedBehavior: 'Conjoin parent property, required-name and additional-property schemas with the object layout. Generate a differing property value only when exactly one branch accepts it and the parent permits it. Report an empty exclusive language when the parent excludes every such value.',
      fixture: 'parity/fixtures/json-schema-exclusive-object-siblings.lino',
      selector: 'parent properties and defaults, local reference, escaped keys and values, required parent fields, sibling pattern constraints, closed parent objects and exclusion of every exclusive value',
      assertions: ['decoderAppliesSiblingObjectAssertionsToExclusiveAlternatives', 'siblingObjectAssertionsCanExcludeEveryAlternative'],
      implementationEntryPoints: {
        javascript: ['js/src/grammar-importers/json-schema.js', 'js/tests/grammar-json-document.test.js'],
        rust: ['rust/src/grammar/json_document/import.rs', 'rust/tests/unit/json_schema_and_recovery.rs'],
      },
    },
    {
      id: 'I195-I240-JSON-SCHEMA-EXCLUSIVE-OBJECT-FIELDS',
      source: ISSUE_195_SOURCES.jsonSchema,
      language: 'JSON with draft 2020-12 schemas',
      version: 'RFC 8259 JSON and JSON Schema draft 2020-12 semantics',
      construct: 'exclusive alternatives with several finite properties in a shared object layout',
      expectedBehavior: 'Group each differing finite property value by branch membership. Emit only combinations of values and optional field absences accepted by exactly one branch, preserving common and parent object constraints. Report an empty exclusive language when all combinations are forbidden or accepted by multiple branches.',
      fixture: 'parity/fixtures/json-schema-exclusive-object-fields.lino',
      selector: 'two or three branches with overlapping enumerations, optional field absence, a referenced branch, parent field constraints, escaped keys and field permutations',
      assertions: ['decoderCountsExclusiveObjectFieldCombinations', 'incompatibleExclusiveObjectFieldsHaveNoGeneratableDocument'],
      implementationEntryPoints: {
        javascript: ['js/src/grammar-importers/json-schema.js', 'js/tests/grammar-json-document.test.js'],
        rust: ['rust/src/grammar/json_document/import.rs', 'rust/tests/unit/json_schema_and_recovery.rs'],
      },
    },
    {
      id: 'I195-I240-JSON-SCHEMA-EXCLUSIVE-REQUIRED-PROPERTIES',
      source: ISSUE_195_SOURCES.jsonSchema,
      language: 'JSON with draft 2020-12 schemas',
      version: 'RFC 8259 JSON and JSON Schema draft 2020-12 semantics',
      construct: 'exclusive object alternatives distinguished by required property names',
      expectedBehavior: 'Exclude branches requiring an absent property and combine that result with the branch membership of present property values. Generate an object only when exactly one branch accepts it, while preserving common and parent assertions. Report an empty exclusive language when the parent forces every generated object to match multiple branches.',
      fixture: 'parity/fixtures/json-schema-exclusive-required-properties.lino',
      selector: 'contact properties, a referenced branch, escaped and undeclared required keys, additional-property and parent assertions, and interacting value and presence alternatives',
      assertions: ['decoderCountsExclusiveRequiredPropertyNames', 'requiredPropertyNamesCanExcludeEveryAlternative'],
      implementationEntryPoints: {
        javascript: ['js/src/grammar-importers/json-schema.js', 'js/tests/grammar-json-document.test.js'],
        rust: ['rust/src/grammar/json_document/import.rs', 'rust/tests/unit/json_schema_and_recovery.rs'],
      },
    },
    {
      id: 'I195-I240-JSON-SCHEMA-EXCLUSIVE-DECLARED-PROPERTIES',
      source: ISSUE_195_SOURCES.jsonSchema,
      language: 'JSON with draft 2020-12 schemas',
      version: 'RFC 8259 JSON and JSON Schema draft 2020-12 semantics',
      construct: 'exclusive object alternatives with distinct declared properties and a common additional-property rule',
      expectedBehavior: 'Apply each branch default to names it leaves undeclared. Count the branches that permit each property value or its absence, and generate objects only when exactly one branch remains. Preserve parent property and presence constraints and report an empty language when the parent excludes every exclusive layout.',
      fixture: 'parity/fixtures/json-schema-exclusive-declared-properties.lino',
      selector: 'closed alternative layouts with optional contact fields, escaped names, an overlapping enumeration, a common typed default, sibling schemas and incompatible parent requirements',
      assertions: ['decoderCountsExclusiveDeclaredPropertyNames', 'declaredPropertyNamesCanExcludeEveryAlternative'],
      implementationEntryPoints: {
        javascript: ['js/src/grammar-importers/json-schema.js', 'js/tests/grammar-json-document.test.js'],
        rust: ['rust/src/grammar/json_document/import.rs', 'rust/tests/unit/json_schema_and_recovery.rs'],
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
      expectedBehavior: 'Grammar search emits localized number, numeric range, currency, duration, calendar date and calendar range annotations over original UTF-8 spans. Inflected number words and open date endpoints are recognized. A numeric range and a date can coexist over the same spelling. A number word scale assumes one when unqualified and preserves a preceding explicit zero in English and Russian number and duration compounds.',
      fixture: 'parity/fixtures/natural-language-values.lino',
      selector: 'the reported Russian and English examples, related inflected forms, punctuation, shared Spanish digit and currency syntax, invalid dates and currency codes, and explicit zero and bare scales in English and Russian number words and durations',
      assertions: ['localizedExactNumbers', 'currencyRanges', 'inflectedDurations', 'dateAlternatives', 'utf8SourceSpans', 'explicitZeroInNumberWordCompounds'],
      implementationEntryPoints: {
        javascript: ['js/src/natural-language-values.js', 'js/tests/natural-language-values.test.js'],
        rust: ['rust/src/link_network/natural_language_values/mod.rs', 'rust/tests/unit/natural_language_values.rs'],
      },
    },
    {
      id: 'I195-I242-UNICODE-DECIMAL-VALUES',
      source: ISSUE_195_SOURCES.naturalLanguageValues,
      language: 'shared digit-based value syntax across natural-language profiles',
      version: 'Unicode 17.0.0 Decimal_Number, ISO 4217 List One 2026-09-17, proleptic Gregorian dates',
      construct: 'Unicode decimal digits and Arabic number separators in typed values',
      expectedBehavior: 'Shared value grammars recognize Unicode decimal digit fields and Arabic decimal and grouping separators. Value actions preserve exact amounts, ranges, date alternatives, and original UTF-8 spelling. Nondecimal characters and invalid calendar dates do not become corresponding values.',
      fixture: 'parity/fixtures/natural-language-unicode-decimal-values.lino',
      selector: 'Modern Standard Arabic amount and money, Hindi grouping and range, Bengali leap date and alternatives, English supplementary digits and nondecimal superscript, Russian duration and fullwidth dates, and supplementary Tolong Siki digits in common Spanish money syntax',
      assertions: ['unicodeDecimalsAndGrouping', 'unicodeCalendarValidationAndAlternatives', 'unicodeValueSourceSpans'],
      implementationEntryPoints: {
        javascript: ['js/src/natural-language-values.js', 'js/tests/natural-language-values.test.js'],
        rust: ['rust/src/link_network/natural_language_values/mod.rs', 'rust/tests/unit/natural_language_values.rs'],
      },
    },
    {
      id: 'I195-I242-WRITTEN-MULTIPLIERS',
      source: ISSUE_195_SOURCES.naturalLanguageValues,
      language: 'English and Russian typed value syntax',
      version: 'Unicode 17.0.0 Decimal_Number, ISO 4217 List One 2026-09-17, exact decimal arithmetic',
      construct: 'digit amounts with written multipliers',
      expectedBehavior: 'English and Russian written scale words multiply digit amounts exactly. A trailing scale applies to both digit endpoints when the first has no scale. Independently scaled endpoints, explicit zero, and original UTF-8 spans are preserved.',
      fixture: 'parity/fixtures/natural-language-written-multipliers.lino',
      selector: 'Russian inflected scale words, Unicode digits, English decimal and currency ranges, independent endpoint scales, exact durations and zero, unchanged abbreviations and date alternatives, rejected attached scale words and invalid currencies',
      assertions: ['writtenMultipliersNormalizeExactValues', 'writtenMultiplierRangesPreserveEndpointScales', 'writtenMultipliersKeepOriginalSpans'],
      implementationEntryPoints: {
        javascript: ['js/src/natural-language-values.js', 'js/tests/natural-language-values.test.js'],
        rust: ['rust/src/link_network/natural_language_values/mod.rs', 'rust/tests/unit/natural_language_values.rs'],
      },
    },
    {
      id: 'I195-I242-NAMED-CALENDAR-DATES',
      source: ISSUE_195_SOURCES.naturalLanguageValues,
      language: 'English and Russian typed value syntax',
      version: 'Proleptic Gregorian dates and Unicode 17.0.0 Decimal_Number',
      construct: 'full calendar dates with written month names',
      expectedBehavior: 'Recognize day-first dates in English and Russian and month-first dates in English. Use the same Gregorian date validation in standalone matches and bounded or open ranges. Preserve original UTF-8 source spans, including when a numeric field uses Unicode decimal digits.',
      fixture: 'parity/fixtures/natural-language-named-calendar-dates.lino',
      selector: 'day-first and month-first dates, English month abbreviation and optional comma, Russian inflected months, leap days, invalid month days, bounded and open ranges, mixed Unicode decimal digits and surrounding UTF-8 text',
      assertions: ['namedCalendarDatesAndRanges', 'namedCalendarDateSpansAndValidation'],
      implementationEntryPoints: {
        javascript: ['js/scripts/build-natural-language-values.mjs', 'js/src/natural-language-values.js', 'js/tests/natural-language-values.test.js'],
        rust: ['js/scripts/build-natural-language-values.mjs', 'rust/src/link_network/natural_language_values/mod.rs', 'rust/tests/unit/natural_language_values.rs'],
      },
    },
    {
      id: 'I195-I242-CURRENCY-SYMBOL-ALTERNATIVES',
      source: ISSUE_195_SOURCES.naturalLanguageValues,
      language: 'English, Russian and shared currency syntax',
      version: 'CLDR 49 root currency symbols, Unicode 17.0.0 Currency_Symbol, ISO 4217 List One 2026-09-17',
      construct: 'ambiguous currency symbols on typed money amounts and ranges',
      expectedBehavior: 'Return one typed money value per compatible currency at the original UTF-8 span. The established currency stays first for an ambiguous sign. Additional signs and explicit ISO codes constrain the alternatives by intersection. Conflicting markers prevent a money value over their combined span. CLDR currency spellings containing a currency symbol retain their complete sign, including a qualifying prefix.',
      fixture: 'parity/fixtures/natural-language-currency-ambiguity.lino',
      selector: 'the reported dollar range, dollar and yen amounts with explicit codes, qualified and case-insensitive signs, pound and won alternatives, euro amounts, overlapping and incompatible markers, preceding UTF-8 text, and all ten newly recognized CLDR spellings',
      assertions: ['currencySymbolsKeepCompatibleCodes', 'currencyMarkersIntersectAtOriginalSpans', 'qualifiedCurrencySymbolsKeepTheirFullSpans'],
      implementationEntryPoints: {
        javascript: ['js/scripts/build-natural-language-values.mjs', 'js/src/natural-language-values.js', 'js/tests/natural-language-values.test.js'],
        rust: ['js/scripts/build-natural-language-values.mjs', 'rust/src/link_network/natural_language_values/mod.rs', 'rust/tests/unit/natural_language_values.rs'],
      },
    },
    {
      id: 'I195-I242-NUMERIC-SEPARATOR-ALTERNATIVES',
      source: ISSUE_195_SOURCES.naturalLanguageValues,
      language: 'English, Russian and shared digit-based value syntax',
      version: 'Unicode 17.0.0 Decimal_Number and exact decimal arithmetic',
      construct: 'ambiguous decimal and grouping separators in typed values',
      expectedBehavior: 'Return both valid readings of an ambiguous comma or period at the original UTF-8 span and keep the locale preference first. Preserve distinct combinations in range endpoints, money amounts, and durations. Emit a single reading when explicit grouping, Arabic decimal punctuation, a wider leading field, or a leading zero establishes the separator meaning.',
      fixture: 'parity/fixtures/natural-language-separator-ambiguity.lino',
      selector: 'English, Russian and Spanish, signed Unicode digits, explicit and ambiguous grouping, ordered number and money readings, implicit range scales, repeated range readings, two-part durations, and original UTF-8 spans',
      assertions: ['ambiguousSeparatorsPreserveLocaleOrder', 'compoundValuesKeepDistinctReadings', 'unambiguousSeparatorsKeepOneReading', 'separatorReadingsKeepOriginalSpans'],
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
    {
      id: 'I195-I243-UNICODE-CASE-FOLDING',
      source: ISSUE_195_SOURCES.normalizedTextLocation,
      language: 'Unicode text search',
      version: 'Unicode 17.0.0 default full case folding, Unicode 13.0 through 17.0 fixture boundaries',
      construct: 'full case folding for quote and word sequence comparisons',
      expectedBehavior: 'The selected case normalization equates Unicode full case folding variants and preserves every original quote and word sequence span. Exact matches cover complete expansions, and case-dependent matches report the transformation.',
      fixture: 'parity/fixtures/text-location-case-folding.lino',
      selector: 'Greek final sigma, German sharp S, ligatures, dotted and dotless I, combined Cyrillic normalization, original offset controls, and common word sequences with case folding expansions',
      assertions: ['fullCaseFoldingMatchesKeepSourceRanges', 'caseFoldingOverlapKeepsBothSourceRanges'],
      implementationEntryPoints: {
        javascript: ['js/src/text-location.js', 'js/tests/language-text-location.test.js'],
        rust: ['rust/src/link_network/text_location.rs', 'rust/tests/unit/text_location.rs'],
      },
    },
    {
      id: 'I195-I243-UNICODE-GRAPHEME-BOUNDARIES',
      source: ISSUE_195_SOURCES.normalizedTextLocation,
      language: 'Unicode text search',
      version: 'Unicode 17.0.0 default extended grapheme clusters and unicode-segmentation 1.13.3',
      construct: 'extended grapheme boundaries and approximate source ranges',
      expectedBehavior: 'Both runtimes use Unicode 17.0.0 extended grapheme boundaries. Replacing one grapheme reports a single edit over the complete grapheme in each original input.',
      fixture: 'parity/fixtures/text-location-grapheme-conformance.lino',
      selector: 'all Unicode 17.0.0 grapheme break cases and bounded quote edits over every distinct cluster',
      assertions: ['unicode17GraphemeBoundaries', 'graphemeEditsKeepOriginalSourceRanges'],
      implementationEntryPoints: {
        javascript: ['js/src/text-location.js', 'js/src/text-location-graphemes.js', 'js/tests/language-text-location.test.js'],
        rust: ['rust/src/link_network/text_location.rs', 'rust/tests/unit/text_location.rs'],
      },
    },
    {
      id: 'I195-I243-UNICODE-COMPATIBILITY-COMPOSITION',
      source: ISSUE_195_SOURCES.normalizedTextLocation,
      language: 'Unicode text search',
      version: 'Unicode 17.0.0 NFKC and unicode-normalization 0.1.25',
      construct: 'compatibility decomposition, ordering and composition with source ranges',
      expectedBehavior: 'The compatibility projection performs Unicode 17.0.0 NFKC across the full text. A composition or reordering that crosses original grapheme boundaries retains a single ordered source span. Quote location and word sequence overlap retain those original ranges.',
      fixture: 'parity/fixtures/text-location-normalization-conformance.lino',
      selector: 'every Unicode 17.0.0 normalization case, exact source ranges after supplementary characters, and halfwidth Hangul composition across original grapheme boundaries',
      assertions: ['unicode17CompatibilityNormalization', 'compatibilityCompositionsKeepOriginalSourceRanges'],
      implementationEntryPoints: {
        javascript: ['js/src/text-location.js', 'js/src/text-location-normalization.js', 'js/tests/language-text-location.test.js'],
        rust: ['rust/src/link_network/text_location.rs', 'rust/tests/unit/text_location.rs'],
      },
    },
    {
      id: 'I195-I243-UNICODE-WORD-AND-DASH-CLASSES',
      source: ISSUE_195_SOURCES.normalizedTextLocation,
      language: 'Unicode text search',
      version: 'Unicode 17.0.0 General_Category',
      construct: 'shared word and dash character classifications',
      expectedBehavior: 'Word alignment and word sequence overlap use the same Unicode 17.0.0 letters, marks, numbers, and underscore in both runtimes. Dash normalization uses the same Unicode dash punctuation and minus sign. Both retain original spans.',
      fixture: 'parity/fixtures/text-location-character-class-cases.lino',
      selector: 'new Unicode 17.0.0 CJK and Beria Erfe letters, Tolong Siki digits, combining marks, fractions, underscore, punctuation boundaries, word replacements and dash equivalence',
      assertions: ['unicode17WordClassesKeepSourceRanges', 'unicode17DashClassKeepsSourceRanges'],
      implementationEntryPoints: {
        javascript: ['js/src/text-location.js', 'js/src/text-location-character-classes.js', 'js/tests/language-text-location.test.js'],
        rust: ['rust/src/link_network/text_location.rs', 'rust/tests/unit/text_location.rs'],
      },
    },
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
  const releaseProvenanceId = 'I195-I230-PAIRED-RELEASE-PROVENANCE';
  const releaseProvenanceFixture = 'i195-i230-paired-release-provenance';
  fixtureCatalog[releaseProvenanceFixture] = pinnedFixture('js/tests/downstream-release-provenance.test.js', 'registry release provenance and checksum verification');
  requirements.push(requirement({
    id: releaseProvenanceId,
    source: ISSUE_195_SOURCES.htmlPackageDelivery,
    area: 'release-workflow',
    scope: {
      language: 'JavaScript', version: 'Node.js >=22', edition: 'tagged release workflow',
      construct: 'paired npm and crates.io release provenance', aliases: [], extensions: [],
    },
    expectedBehavior: 'The release checks verify the published crate bytes against the registry checksum, require clean Cargo provenance at the release commit, and require matching npm Git metadata whenever the npm version exists.',
    requiredRuntimes: ['javascript'],
    implementationEntryPoints: { javascript: ['js/scripts/release-publication.mjs', '.github/workflows/js.yml'] },
    verifications: [verification({
      requirementId: releaseProvenanceId, runtime: 'javascript', suffix: 'behavior', fixtureIds: [releaseProvenanceFixture],
      assertions: ['crateArchiveMatchesRegistryChecksum', 'crateArchiveIdentifiesCleanReleaseCommit', 'npmMetadataIdentifiesReleaseCommit'],
    })],
  }));
  const exampleFixture = 'i195-installed-document-tasks';
  fixtureCatalog[exampleFixture] = pinnedFixture(DOCUMENT_TASK_EXAMPLE, 'installed public API document task examples');
  for (const [id, source, scenario, assertion, expectedBehavior] of DOCUMENT_TASK_CASES) {
    requirements.push(requirement({
      id,
      source: ISSUE_195_SOURCES[source],
      area: 'package-delivery',
      scope: {
        language: 'JavaScript package',
        version: 'matching release version on Node.js >=22',
        edition: 'clean npm artifact installation',
        construct: scenario,
        aliases: [],
        extensions: [],
      },
      expectedBehavior,
      requiredRuntimes: ['javascript'],
      implementationEntryPoints: {
        javascript: ['js/src/index.js', 'js/scripts/run-issue-195-consumer.mjs'],
      },
      verifications: ['pre-merge', 'release-delivery'].map(checkpoint => verification({
        requirementId: id,
        runtime: 'javascript',
        suffix: checkpoint,
        fixtureIds: [exampleFixture],
        assertions: [assertion],
        checkpoint,
      })),
    }));
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
