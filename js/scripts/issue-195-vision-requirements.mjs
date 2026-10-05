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
    id: 'I195-DEPENDENCY-CURRENT-STABLE-DELIVERY',
    area: 'dependency-freshness',
    specification: 'dependencies',
    fixture: 'parity/dependency-inventory.json',
    construct: 'current stable retained dependencies and build tools in delivered packages',
    expectedBehavior:
      'Every retained runtime, development, build and optional dependency, transitive resolution, generator, toolchain, workflow action, build image and version-coupled artifact is updated to the current stable compatible release at the recorded audit date. A compatibility reason alone does not count as an upgrade, and the delivery gate rejects stale retained items.',
    assertions: [
      'allRetainedItemsCurrent',
      'transitiveResolutionsCurrent',
      'buildToolsAndImagesCurrent',
      'versionCoupledArtifactsRegenerated',
      'staleDeliveredItemRejected',
    ],
    tooling: ['js/scripts/dependency-inventory.mjs', 'js/scripts/check-dependencies.mjs'],
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
    id: 'I195-GRAMMAR-NATIVE-JSON',
    area: 'native-grammar',
    specification: 'native-merged-grammars',
    fixture: 'parity/fixtures/native-grammars/json.json',
    construct: 'native merged JSON grammar checked against its tree-sitter oracle',
    expectedBehavior:
      'parity/grammars/native/json.lino is a canonical native Links Notation grammar merged from RFC 8259, ECMA-404 and tree-sitter-json 0.24.8. Both executors build, for every corpus source, exactly the concrete syntax tree rows of the tree-sitter-json oracle; they accept what a merged source accepts and the oracle recovers from (an exponent plus sign), reject invalid JSON, and keep every source byte, a leading byte order mark included, in the tree.',
    assertions: [
      'nativeJsonGrammarIsCanonicalLinks',
      'nativeJsonTreesMatchOracle',
      'nativeJsonAcceptsMergedSourceExtensions',
      'nativeJsonRejectsInvalidInput',
      'nativeJsonTreesLossless',
    ],
    javascript: ['js/src/grammar-links.js', 'js/src/grammar-runtime.js', 'js/src/grammar-runtime/text.js'],
    rust: ['rust/src/grammar/interchange/links.rs', 'rust/src/grammar/feature_runtime/mod.rs', 'rust/src/grammar/feature_runtime/tree.rs'],
  },
  {
    id: 'I195-GRAMMAR-NATIVE-INI',
    area: 'native-grammar',
    specification: 'native-merged-grammars',
    fixture: 'parity/fixtures/native-grammars/ini.json',
    construct: 'native merged INI grammar checked against its tree-sitter oracle',
    expectedBehavior:
      'parity/grammars/native/ini.lino is a canonical native Links Notation grammar merged from tree-sitter-ini 1.4.0 and the Python configparser INI file structure. Both executors build, for every corpus source, exactly the concrete syntax tree rows of the tree-sitter-ini oracle with no ambiguity, comments included as extras; they accept what configparser accepts and the oracle recovers from (a last comment line without a line break), reject invalid INI, and keep every source byte, blank lines, line breaks and comment markers included, in the tree.',
    assertions: [
      'nativeIniGrammarIsCanonicalLinks',
      'nativeIniTreesMatchOracle',
      'nativeIniAcceptsMergedSourceExtensions',
      'nativeIniRejectsInvalidInput',
      'nativeIniTreesLossless',
    ],
    javascript: ['js/src/grammar-links.js', 'js/src/grammar-runtime.js', 'js/src/grammar-runtime/text.js'],
    rust: ['rust/src/grammar/interchange/links.rs', 'rust/src/grammar/feature_runtime/mod.rs', 'rust/src/grammar/feature_runtime/tree.rs'],
  },
  {
    id: 'I195-GRAMMAR-NATIVE-DIFF',
    area: 'native-grammar',
    specification: 'native-merged-grammars',
    fixture: 'parity/fixtures/native-grammars/diff.json',
    construct: 'native merged unified diff grammar checked against its tree-sitter oracle',
    expectedBehavior:
      'parity/grammars/native/diff.lino is a canonical native Links Notation grammar merged from tree-sitter-diff 0.1.0, the GNU diffutils unified format and the git patch format. Both executors build, for every corpus source, exactly the concrete syntax tree rows of the tree-sitter-diff oracle with no ambiguity, git blocks, hunks and changes included; they accept what GNU diff and git write and the oracle recovers from (abbreviated object names, context and changed lines that start like a keyword or a file header, a block cut at the end of the input), reject invalid diffs, and keep every source byte, line breaks and blank lines included, in the tree.',
    assertions: [
      'nativeDiffGrammarIsCanonicalLinks',
      'nativeDiffTreesMatchOracle',
      'nativeDiffAcceptsMergedSourceExtensions',
      'nativeDiffRejectsInvalidInput',
      'nativeDiffTreesLossless',
    ],
    javascript: ['js/src/grammar-links.js', 'js/src/grammar-runtime.js', 'js/src/grammar-runtime/text.js'],
    rust: ['rust/src/grammar/interchange/links.rs', 'rust/src/grammar/feature_runtime/mod.rs', 'rust/src/grammar/feature_runtime/tree.rs'],
  },
  {
    id: 'I195-GRAMMAR-NATIVE-CSV',
    area: 'native-grammar',
    specification: 'native-merged-grammars',
    fixture: 'parity/fixtures/native-grammars/csv.json',
    construct: 'native merged CSV grammar checked against its tree-sitter oracle',
    expectedBehavior:
      'parity/grammars/native/csv.lino is a canonical native Links Notation grammar merged from tree-sitter-csv (revision f6bf6e3 with the RFC 4180 quotes patch) and RFC 4180. Both executors build, for every corpus source, exactly the concrete syntax tree rows of the tree-sitter-csv oracle with no ambiguity, typed number, float and boolean fields, quoted fields with doubled quotes and line breaks, and blank lines included; they accept the empty last field RFC 4180 allows and the oracle recovers from, reject invalid quoting, and keep every source byte, line breaks and blank lines included, in the tree.',
    assertions: [
      'nativeCsvGrammarIsCanonicalLinks',
      'nativeCsvTreesMatchOracle',
      'nativeCsvAcceptsMergedSourceExtensions',
      'nativeCsvRejectsInvalidInput',
      'nativeCsvTreesLossless',
    ],
    javascript: ['js/src/grammar-links.js', 'js/src/grammar-runtime.js', 'js/src/grammar-runtime/text.js'],
    rust: ['rust/src/grammar/interchange/links.rs', 'rust/src/grammar/feature_runtime/mod.rs', 'rust/src/grammar/feature_runtime/tree.rs'],
  },
  {
    id: 'I195-GRAMMAR-NATIVE-JSON5',
    area: 'native-grammar',
    specification: 'native-merged-grammars',
    fixture: 'parity/fixtures/native-grammars/json5.json',
    construct: 'native merged JSON5 grammar checked against its tree-sitter oracle',
    expectedBehavior:
      'parity/grammars/native/json5.lino is a canonical native Links Notation grammar merged from tree-sitter-json5-orchard 0.1.0 and the JSON5 1.0.0 specification. Both executors build, for every corpus source, exactly the concrete syntax tree rows of the tree-sitter-json5-orchard oracle with no ambiguity, unquoted and quoted member names, single-quoted and multi-line strings, hexadecimal, signed, Infinity and NaN numbers, trailing commas and line and block comments as extra rows included; they accept the JSON5 white space, identifier names and string escapes the oracle recovers from, reject invalid input, and keep every source byte, comments and white space included, in the tree.',
    assertions: [
      'nativeJson5GrammarIsCanonicalLinks',
      'nativeJson5TreesMatchOracle',
      'nativeJson5AcceptsMergedSourceExtensions',
      'nativeJson5RejectsInvalidInput',
      'nativeJson5TreesLossless',
    ],
    javascript: ['js/src/grammar-links.js', 'js/src/grammar-runtime.js', 'js/src/grammar-runtime/text.js'],
    rust: ['rust/src/grammar/interchange/links.rs', 'rust/src/grammar/feature_runtime/mod.rs', 'rust/src/grammar/feature_runtime/tree.rs'],
  },
  {
    id: 'I195-GRAMMAR-NATIVE-SCHEME',
    area: 'native-grammar',
    specification: 'native-merged-grammars',
    fixture: 'parity/fixtures/native-grammars/scheme.json',
    construct: 'native merged Scheme grammar checked against its tree-sitter oracle',
    expectedBehavior:
      'parity/grammars/native/scheme.lino is a canonical native Links Notation grammar merged from tree-sitter-scheme 0.24.7 and the R7RS small report. Both executors build, for every corpus source, exactly the concrete syntax tree rows of the tree-sitter-scheme oracle with no ambiguity, lists, vectors, byte vectors, quote, quasiquote, unquote and syntax forms, booleans, characters, strings with escape sequences, R5RS, R6RS and R7RS numbers, symbols, keywords, directives and line, datum and nested block comments included; they accept the R7RS bytevectors and datum labels the oracle recovers from, reject invalid input, and keep every source byte, comments and white space included, in the tree.',
    assertions: [
      'nativeSchemeGrammarIsCanonicalLinks',
      'nativeSchemeTreesMatchOracle',
      'nativeSchemeAcceptsMergedSourceExtensions',
      'nativeSchemeRejectsInvalidInput',
      'nativeSchemeTreesLossless',
    ],
    javascript: ['js/src/grammar-links.js', 'js/src/grammar-runtime.js', 'js/src/grammar-runtime/text.js'],
    rust: ['rust/src/grammar/interchange/links.rs', 'rust/src/grammar/feature_runtime/mod.rs', 'rust/src/grammar/feature_runtime/tree.rs'],
  },
  {
    id: 'I195-GRAMMAR-NATIVE-RACKET',
    area: 'native-grammar',
    specification: 'native-merged-grammars',
    fixture: 'parity/fixtures/native-grammars/racket.json',
    construct: 'native merged Racket grammar checked against its tree-sitter oracle',
    expectedBehavior:
      'parity/grammars/native/racket.lino is a canonical native Links Notation grammar merged from tree-sitter-racket 0.25.0 and the reader chapter of the Racket Reference. Both executors build, for every corpus source, exactly the concrete syntax tree rows of the tree-sitter-racket oracle with no ambiguity, lists with dots, vectors, flvectors, fxvectors, structures, hash tables, boxes, graph labels, quote, quasiquote, unquote and syntax forms, booleans, characters, strings, byte strings, here strings, regular expressions, numbers and extflonums, symbols, keywords, #lang and #reader extensions and line, datum and nested block comments included; they accept the line feeds a backslash quotes in characters and symbols, which the oracle recovers from, reject invalid input, and keep every source byte, comments, white space and a leading byte order mark included, in the tree.',
    assertions: [
      'nativeRacketGrammarIsCanonicalLinks',
      'nativeRacketTreesMatchOracle',
      'nativeRacketAcceptsMergedSourceExtensions',
      'nativeRacketRejectsInvalidInput',
      'nativeRacketTreesLossless',
    ],
    javascript: ['js/src/grammar-links.js', 'js/src/grammar-runtime.js', 'js/src/grammar-runtime/text.js'],
    rust: ['rust/src/grammar/interchange/links.rs', 'rust/src/grammar/feature_runtime/mod.rs', 'rust/src/grammar/feature_runtime/tree.rs'],
  },
  {
    id: 'I195-GRAMMAR-NATIVE-C',
    area: 'native-grammar',
    specification: 'native-merged-grammars',
    fixture: 'parity/fixtures/native-grammars/c.json',
    construct: 'native merged C grammar imported from its pinned source and checked against its tree-sitter oracle',
    expectedBehavior:
      'parity/grammars/native/c.lino is a canonical native Links Notation grammar that js/scripts/import-native-grammars.mjs imports from the pinned src/grammar.json of tree-sitter-c 0.24.2, with every rule renamed to its English name and every hand decision recorded in parity/grammars/merge-reports/c.json. Both executors build, for every source of the upstream test/corpus and the added cases, exactly the concrete syntax tree rows of the tree-sitter-c oracle: declarations, function definitions, structures, unions, enumerations, type definitions, preprocessor directives, statements, expressions, string and character literals, attributes, GNU and Microsoft extensions and comments included. A keyword is lexed as the oracle lexes it, an identifier only where the keyword cannot stand; both executors reject invalid input the oracle recovers from, repair it into the recorded tree, and keep every source byte, comments and white space included, in the tree.',
    assertions: [
      'nativeCGrammarIsCanonicalLinks',
      'nativeCTreesMatchOracle',
      'nativeCRejectsInvalidInput',
      'nativeCTreesLossless',
    ],
    javascript: ['js/src/grammar-importers/tree-sitter-native.js', 'js/src/grammar-runtime.js', 'js/src/grammar-runtime/executor.js'],
    rust: ['rust/src/grammar/interchange/links.rs', 'rust/src/grammar/feature_runtime/mod.rs', 'rust/src/grammar/feature_runtime/executor.rs'],
  },
  {
    id: 'I195-GRAMMAR-NATIVE-RUST',
    area: 'native-grammar',
    specification: 'native-merged-grammars',
    fixture: 'parity/fixtures/native-grammars/rust.json',
    construct: 'native merged Rust grammar imported with its native scanner from its pinned source and checked against its tree-sitter oracle',
    expectedBehavior:
      'parity/grammars/native/rust.lino is a canonical native Links Notation grammar that js/scripts/import-native-grammars.mjs imports from the pinned src/grammar.json of tree-sitter-rust 0.24.2 with the meta-language patch of its oracle, with every rule renamed to its English name, its external scanner src/scanner.c ported to the native scanner links of parity/grammars/scanners/rust.lino, and every hand decision recorded in parity/grammars/merge-reports/rust.json. Both executors build, for every source of the upstream test/corpus and the added cases, exactly the concrete syntax tree rows of the tree-sitter-rust oracle with no ambiguity: items, attributes, macros and token trees, patterns, types, generics, closures, expressions, raw strings, nested block comments, doc comments and frontmatter included. A reduce/reduce conflict is settled by precedence as tree-sitter settles it, so m!(x); is an expression statement; both executors reject invalid input the oracle recovers from, repair it into the recorded tree, and keep every source byte, comments and white space included, in the tree.',
    assertions: [
      'nativeRustGrammarIsCanonicalLinks',
      'nativeRustTreesMatchOracle',
      'nativeRustRejectsInvalidInput',
      'nativeRustTreesLossless',
    ],
    javascript: ['js/src/grammar-importers/tree-sitter-native.js', 'js/src/grammar-runtime/operations.js', 'js/src/grammar-runtime/executor.js'],
    rust: ['rust/src/grammar/interchange/links.rs', 'rust/src/grammar/feature_runtime/ordering.rs', 'rust/src/grammar/feature_runtime/precedence.rs'],
  },
  {
    id: 'I195-GRAMMAR-NATIVE-JAVASCRIPT',
    area: 'native-grammar',
    specification: 'native-merged-grammars',
    fixture: 'parity/fixtures/native-grammars/javascript.json',
    construct: 'native merged JavaScript grammar imported with its native scanner from its pinned source and checked against its tree-sitter oracle',
    expectedBehavior:
      'parity/grammars/native/javascript.lino is a canonical native Links Notation grammar that js/scripts/import-native-grammars.mjs imports from the pinned src/grammar.json of tree-sitter-javascript 0.25.0, with every rule renamed to its English name, its external scanner src/scanner.c ported to the five native scanners of parity/grammars/scanners/javascript.lino (automatic semicolons, template characters, the ternary question mark, HTML-like comments and JSX text), and every hand decision recorded in parity/grammars/merge-reports/javascript.json. Both executors build, for every source of the upstream test/corpus the oracle accepts and the added cases, exactly the concrete syntax tree rows of the tree-sitter-javascript oracle with no ambiguity: modules, classes, functions, generators, destructuring, automatic semicolon insertion, template strings with substitutions, regular expressions, JSX and HTML-like comments included. Both executors reject invalid input the oracle recovers from, repair it into the recorded tree, and keep every source byte, comments and white space included, in the tree.',
    assertions: [
      'nativeJavaScriptGrammarIsCanonicalLinks',
      'nativeJavaScriptTreesMatchOracle',
      'nativeJavaScriptRejectsInvalidInput',
      'nativeJavaScriptTreesLossless',
    ],
    javascript: ['js/src/grammar-importers/tree-sitter-native.js', 'js/src/grammar-runtime/operations.js', 'js/src/grammar-runtime/executor.js'],
    rust: ['rust/src/grammar/interchange/links.rs', 'rust/src/grammar/feature_runtime/ordering.rs', 'rust/src/grammar/feature_runtime/rules.rs'],
  },
  {
    id: 'I195-GRAMMAR-NATIVE-TYPESCRIPT',
    area: 'native-grammar',
    specification: 'native-merged-grammars',
    fixture: 'parity/fixtures/native-grammars/typescript.json',
    construct: 'native merged TypeScript grammar imported with its native scanner from its pinned source and checked against its tree-sitter oracle',
    expectedBehavior:
      'parity/grammars/native/typescript.lino is a canonical native Links Notation grammar that js/scripts/import-native-grammars.mjs imports from the pinned typescript/src/grammar.json of tree-sitter-typescript 0.23.2, with every rule renamed to its English name, its external scanner common/scanner.h ported to the seven native scanners of parity/grammars/scanners/typescript.lino (automatic semicolons, template characters, the ternary question mark, HTML-like comments and JSX text, and the automatic semicolon after a function signature and the error recovery sentinel, which fail as their C scanner never returns them), and every hand decision recorded in parity/grammars/merge-reports/typescript.json. Both executors build, for every source of the upstream test/corpus the oracle accepts and the added cases, exactly the concrete syntax tree rows of the tree-sitter-typescript oracle with no ambiguity: interfaces, type aliases, enums, namespaces, ambient declarations, generics, union, intersection, conditional, mapped and template literal types, decorators, as, satisfies and non-null expressions, instantiation expressions and the type assertion `<T>x` included. Both executors reject invalid input the oracle recovers from, repair it into the recorded tree, and keep every source byte, comments and white space included, in the tree.',
    assertions: [
      'nativeTypeScriptGrammarIsCanonicalLinks',
      'nativeTypeScriptTreesMatchOracle',
      'nativeTypeScriptRejectsInvalidInput',
      'nativeTypeScriptTreesLossless',
    ],
    javascript: ['js/src/grammar-importers/tree-sitter-native.js', 'js/src/grammar-runtime/operations.js', 'js/src/grammar-runtime/executor.js'],
    rust: ['rust/src/grammar/interchange/links.rs', 'rust/src/grammar/feature_runtime/ordering.rs', 'rust/src/grammar/feature_runtime/precedence.rs'],
  },
  {
    id: 'I195-GRAMMAR-NATIVE-TSX',
    area: 'native-grammar',
    specification: 'native-merged-grammars',
    fixture: 'parity/fixtures/native-grammars/tsx.json',
    construct: 'native merged TSX grammar imported with its native scanner from its pinned source and checked against its tree-sitter oracle',
    expectedBehavior:
      'parity/grammars/native/tsx.lino is a canonical native Links Notation grammar that js/scripts/import-native-grammars.mjs imports from the pinned tsx/src/grammar.json of tree-sitter-typescript 0.23.2, with every rule renamed to its English name, its external scanner common/scanner.h ported to the seven native scanners of parity/grammars/scanners/typescript.lino (automatic semicolons, template characters, the ternary question mark, HTML-like comments and JSX text, and the automatic semicolon after a function signature and the error recovery sentinel, which fail as their C scanner never returns them), and every hand decision recorded in parity/grammars/merge-reports/tsx.json. Both executors build, for every source of the upstream test/corpus the oracle accepts and the added cases, exactly the concrete syntax tree rows of the tree-sitter-typescript oracle with no ambiguity: interfaces, type aliases, enums, namespaces, ambient declarations, generics, union, intersection, conditional, mapped and template literal types, decorators, as, satisfies and non-null expressions, instantiation expressions and JSX elements in place of the type assertion included. Both executors reject invalid input the oracle recovers from, repair it into the recorded tree, and keep every source byte, comments and white space included, in the tree.',
    assertions: [
      'nativeTsxGrammarIsCanonicalLinks',
      'nativeTsxTreesMatchOracle',
      'nativeTsxRejectsInvalidInput',
      'nativeTsxTreesLossless',
    ],
    javascript: ['js/src/grammar-importers/tree-sitter-native.js', 'js/src/grammar-runtime/operations.js', 'js/src/grammar-runtime/executor.js'],
    rust: ['rust/src/grammar/interchange/links.rs', 'rust/src/grammar/feature_runtime/ordering.rs', 'rust/src/grammar/feature_runtime/precedence.rs'],
  },
  {
    id: 'I195-GRAMMAR-NATIVE-LEAN',
    area: 'native-grammar',
    specification: 'native-merged-grammars',
    fixture: 'parity/fixtures/native-grammars/lean.json',
    construct: 'native merged Lean grammar imported with its native layout scanner from its pinned source and checked against its tree-sitter oracle',
    expectedBehavior:
      'parity/grammars/native/lean.lino is a canonical native Links Notation grammar that js/scripts/import-native-grammars.mjs imports from the pinned src/grammar.json of tree-sitter-lean4 0.3.0, with every rule renamed to its English name and its concept merged into the shared concepts, its external layout scanner src/scanner.c ported to the native layout scanner of parity/grammars/scanners/lean.lino, and every hand decision recorded in parity/grammars/merge-reports/lean.json. Both executors build, for every source of the upstream test/corpus the oracle accepts, exactly the concrete syntax tree rows of the tree-sitter-lean4 oracle with no ambiguity: commands, namespaces and sections, definitions, theorems, instances, structures and inductive types, binders, applications and projections, tactic blocks, calculations, do notation and its layout, match expressions, syntax quotations and string interpolation included. The merged grammar also reads the explicit function `@foo` of Theorem Proving in Lean 4, which the oracle recovers from. Both executors reject invalid input the oracle recovers from and keep every source byte, comments and white space included, in the tree; the default Lean parse keeps the public `file` root over the grammar\'s `module` root.',
    assertions: [
      'nativeLeanGrammarIsCanonicalLinks',
      'nativeLeanTreesMatchOracle',
      'nativeLeanAcceptsMergedSourceExtensions',
      'nativeLeanRejectsInvalidInput',
      'nativeLeanTreesLossless',
    ],
    javascript: ['js/src/grammar-importers/tree-sitter-native.js', 'js/src/grammar-runtime/operations.js', 'js/src/programming-language-parser.js'],
    rust: ['rust/src/grammar/interchange/links.rs', 'rust/src/grammar/feature_runtime/ordering.rs', 'rust/src/tree_sitter_adapter.rs'],
  },
  {
    id: 'I195-GRAMMAR-NATIVE-ROCQ',
    area: 'native-grammar',
    specification: 'native-merged-grammars',
    fixture: 'parity/fixtures/native-grammars/rocq.json',
    construct: 'native merged Rocq grammar imported from its pinned source and checked against its tree-sitter oracle',
    expectedBehavior:
      'parity/grammars/native/rocq.lino is a canonical native Links Notation grammar that js/scripts/import-native-grammars.mjs imports from the pinned src/grammar.json of tree-sitter-rocq 300fe33, with every rule renamed to its English name and its concept merged into the shared concepts, and every hand decision recorded in parity/grammars/merge-reports/rocq.json. Both executors build, for every source of the upstream test/corpus the oracle accepts and for hand-written sources, exactly the concrete syntax tree rows of the tree-sitter-rocq oracle with no ambiguity: attributes, Require and Import, definitions, theorems and their proofs, fixpoints, inductive types and records, sections and modules, notations and scopes, Ltac definitions and tactic sequences, binders, applications, functions, let, match and if expressions, lists, strings and nested comments included. Both executors reject invalid input the oracle recovers from and keep every source byte, comments and white space included, in the tree; the default Rocq parse keeps the semantic `identifier` and `primitive_type` leaves under each `ident` with text.',
    assertions: [
      'nativeRocqGrammarIsCanonicalLinks',
      'nativeRocqTreesMatchOracle',
      'nativeRocqRejectsInvalidInput',
      'nativeRocqTreesLossless',
    ],
    javascript: ['js/src/grammar-importers/tree-sitter-native.js', 'js/src/grammar-runtime/executor.js', 'js/src/programming-language-parser.js'],
    rust: ['rust/src/grammar/interchange/links.rs', 'rust/src/grammar/feature_runtime/ordering.rs', 'rust/src/tree_sitter_adapter.rs'],
  },
  {
    id: 'I195-GRAMMAR-NATIVE-JAVA',
    area: 'native-grammar',
    specification: 'native-merged-grammars',
    fixture: 'parity/fixtures/native-grammars/java.json',
    construct: 'native merged Java grammar imported from its pinned source and checked against its tree-sitter oracle',
    expectedBehavior:
      'parity/grammars/native/java.lino is a canonical native Links Notation grammar that js/scripts/import-native-grammars.mjs imports from the pinned src/grammar.json of tree-sitter-java 0.23.5, with every rule renamed to its English name and its concept merged into the shared concepts, and every hand decision recorded in parity/grammars/merge-reports/java.json. Both executors build, for every source of the upstream test/corpus the oracle accepts and for hand-written sources, exactly the concrete syntax tree rows of the tree-sitter-java oracle with no ambiguity: packages, imports and modules, classes, records, interfaces, enumerations and annotation types, fields, methods and constructors, generic types and type parameters, local variables, statements, switch expressions and yield, lambdas, method references, object creation, binary, instanceof, ternary and cast expressions, annotations and their element values, literals, text blocks and comments included. Both executors reject invalid input the oracle recovers from and keep every source byte, comments and white space included, in the tree.',
    assertions: [
      'nativeJavaGrammarIsCanonicalLinks',
      'nativeJavaTreesMatchOracle',
      'nativeJavaRejectsInvalidInput',
      'nativeJavaTreesLossless',
    ],
    javascript: ['js/src/grammar-importers/tree-sitter-native.js', 'js/src/grammar-runtime/executor.js', 'js/src/programming-language-parser.js'],
    rust: ['rust/src/grammar/interchange/links.rs', 'rust/src/grammar/feature_runtime/ordering.rs', 'rust/src/grammar/feature_runtime/forking.rs', 'rust/src/native_grammar_parser.rs'],
  },
  {
    id: 'I195-GRAMMAR-NATIVE-GO',
    area: 'native-grammar',
    specification: 'native-merged-grammars',
    fixture: 'parity/fixtures/native-grammars/go.json',
    construct: 'native merged Go grammar imported from its pinned source and checked against its tree-sitter oracle',
    expectedBehavior:
      'parity/grammars/native/go.lino is a canonical native Links Notation grammar that js/scripts/import-native-grammars.mjs imports from the pinned src/grammar.json of tree-sitter-go 0.25.0, with every rule renamed to its English name and its concept merged into the shared concepts, and every hand decision recorded in parity/grammars/merge-reports/go.json. Both executors build, for every source of the upstream test/corpus the oracle accepts and for hand-written sources, exactly the concrete syntax tree rows of the tree-sitter-go oracle with no ambiguity: packages and imports, functions, methods and generic type parameters, constants with iota, variables, type declarations and aliases, struct, interface, map, slice, array, pointer, function and channel types, short variable declarations, assignments, increments, if, for, range, switch, type switch and select statements, go, defer, labels, goto and break, channel sends and receives, composite literals, function literals, calls, generic calls, type conversions, index, slice and selector expressions, type assertions, raw and interpreted string literals, rune, integer, float and imaginary literals and comments included. Both executors reject invalid input the oracle recovers from and keep every source byte, comments and white space included, in the tree.',
    assertions: [
      'nativeGoGrammarIsCanonicalLinks',
      'nativeGoTreesMatchOracle',
      'nativeGoRejectsInvalidInput',
      'nativeGoTreesLossless',
    ],
    javascript: ['js/src/grammar-importers/tree-sitter-native.js', 'js/src/grammar-runtime/executor.js', 'js/src/programming-language-parser.js'],
    rust: ['rust/src/grammar/interchange/links.rs', 'rust/src/grammar/feature_runtime/ordering.rs', 'rust/src/grammar/feature_runtime/forking.rs', 'rust/src/native_grammar_parser.rs'],
  },
  {
    id: 'I195-GRAMMAR-NATIVE-RECOVERY',
    area: 'native-grammar',
    specification: 'grammar-feature-union',
    fixture: 'parity/fixtures/native-grammars/json.json',
    construct: 'automatic error recovery of the native executor for every native grammar',
    expectedBehavior:
      'With errorRecovery (error_recovery in Rust) both executors repair a parse the grammar rejects, without recovery rules in the grammar: at the farthest failing element they insert a zero-width MISSING leaf or skip to the next match of the element behind an ERROR leaf, choose the repair of least cost, and cover input no repair reaches with an ERROR leaf. For every rejection of the twelve native grammar fixtures (JSON, INI, diff, CSV, JSON5, Scheme, Racket, C, Rust, JavaScript, TypeScript, TSX) both executors build the same recorded tree, which keeps every source byte and is reported as a recovered rejection; without the option the parse is still rejected, and inputs the grammar accepts parse to the same tree with and without it.',
    assertions: [
      'nativeRecoveryTreesMatchFixtures',
      'nativeRecoveryTreesLossless',
      'nativeRecoveryReportedAsRecovered',
      'nativeRecoveryKeepsAcceptedTrees',
    ],
    javascript: ['js/src/grammar-runtime.js', 'js/src/grammar-runtime/executor.js'],
    rust: ['rust/src/grammar/feature_runtime/mod.rs', 'rust/src/grammar/feature_runtime/executor.rs', 'rust/src/grammar/feature_runtime/results.rs'],
  },
  {
    id: 'I195-GRAMMAR-FEATURE-UNION',
    area: 'native-grammar',
    specification: 'grammar-feature-union',
    fixture: 'parity/fixtures/grammar-feature-union.json',
    construct: 'grammar representation covering the union of all source grammar features',
    expectedBehavior:
      'The native grammar representation and its executor cover every feature listed in the grammar feature union, from ordered and unordered alternatives, left recursion and precedence to lexer modes, external scanners, semantic actions, embedded languages and error recovery. External scanners and actions are executable link definitions, and each feature has positive and negative executable tests.',
    assertions: [
      'everyUnionFeatureRepresented',
      'everyUnionFeatureExecuted',
      'scannersAndActionsExecutableAsLinks',
      'negativeCasesRejected',
    ],
    javascript: [
      'js/src/grammar-feature-forms.js',
      'js/src/grammar-runtime.js',
      'js/src/grammar-runtime/executor.js',
      'js/src/grammar-merge.js',
      'js/src/grammar-lowering.js',
    ],
    rust: [
      'rust/src/grammar/feature.rs',
      'rust/src/grammar/feature_runtime/mod.rs',
      'rust/src/grammar/feature_runtime/executor.rs',
      'rust/src/grammar/merge/declarations.rs',
      'rust/src/grammar/interchange/lowering/metadata.rs',
    ],
  },
  {
    id: 'I195-GRAMMAR-CONCEPT-DISTINCTIONS',
    area: 'native-grammar',
    specification: 'grammar-feature-union',
    fixture: 'parity/fixtures/concept-distinctions.json',
    construct: 'preserved semantic distinctions between grammar concepts',
    expectedBehavior:
      'Concepts are shared only under a justified one-to-one correspondence of meaning. Ordered and unordered choice, lexical and syntactic precedence, integer and overflow models, binding rules, effects and proof universes stay distinct, and fixtures with the same spelling but different meanings are not merged.',
    assertions: ['sharedOnlyWithJustifiedCorrespondence', 'requiredDistinctionsPreserved', 'lookalikeConceptsKeptDistinct'],
    javascript: ['js/src/concept-distinctions.js', 'js/src/concept-records.js', 'js/src/foundation-models.js'],
    rust: ['rust/src/concept_distinctions.rs', 'rust/src/concept_records.rs', 'rust/src/foundation_models.rs'],
  },
  {
    id: 'I195-GRAMMAR-SHARED-CONCEPTS',
    area: 'native-grammar',
    specification: 'grammar-feature-union',
    fixture: 'parity/fixtures/grammar-shared-concepts.json',
    construct: 'a canonical concept record for every rule of every native grammar',
    expectedBehavior:
      'Every rule of every native grammar names a canonical concept record, and the record lists the rule as a native source alias. A construct that corresponds one to one in two or more languages resolves to one shared concept identity, such as the JSON and JSON5 object and pair and the Scheme and Racket list, symbol and boolean. Lookalike constructs stay distinct concepts, and a language-specific concept is named by its own language alone. A rule without a concept record fails the check.',
    assertions: [
      'everyRuleHasConceptRecord',
      'recordAliasesMatchRules',
      'sharedConstructsResolveToOneIdentity',
      'lookalikeConstructsStayDistinct',
      'languageSpecificConceptsStayInTheirLanguage',
    ],
    javascript: ['js/src/grammar-concepts.js', 'js/src/concept-records.js', 'js/src/concept-distinctions.js'],
    rust: ['rust/src/grammar_concepts.rs', 'rust/src/concept_records.rs', 'rust/src/concept_distinctions.rs'],
  },
  {
    id: 'I195-GRAMMAR-CONCEPT-REUSE-REPORT',
    area: 'native-grammar',
    specification: 'grammar-feature-union',
    fixture: 'parity/fixtures/native-grammar-concept-reuse.json',
    construct: 'per-language report of shared and language-specific native grammar rules',
    expectedBehavior:
      'A generated report lists, for every native grammar, the rules that name a shared concept and the rules that name a language-specific concept, with the counts per language. CI fails when the report is stale, both runtimes compute the same report, and it is published with the merge quality evidence.',
    assertions: ['reuseReportCurrent', 'sharedAndSpecificRulesListed', 'reportPublishedWithMergeQualityEvidence', 'runtimesAgreeOnReport'],
    javascript: ['js/src/grammar-concepts.js', 'js/scripts/build-native-grammar-concept-reuse.mjs'],
    rust: ['rust/src/grammar_concepts.rs'],
  },
  {
    id: 'I195-GRAMMAR-CONCEPT-TRANSLATION',
    area: 'native-grammar',
    specification: 'grammar-feature-union',
    fixture: 'parity/fixtures/grammar-shared-concepts.json',
    construct: 'translation of shared constructs between native grammars through their concepts',
    expectedBehavior:
      'A construct of one native grammar translates to another native grammar through its concept record alone, with no rule for the pair of languages. The translated construct tree equals the tree the target grammar parses from the target text, and every construct the target has no rule for is reported as untranslatable instead of guessed.',
    assertions: [
      'constructsTranslateThroughConcepts',
      'noPairwiseRuleUsed',
      'untranslatableConstructsReported',
      'translatedTreesMatchTargetParses',
    ],
    javascript: ['js/src/grammar-concepts.js'],
    rust: ['rust/src/grammar_concepts.rs'],
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
    javascript: ['js/src/grammar-interchange.js', 'js/src/cli.js'],
    rust: ['rust/src/grammar/interchange/mod.rs', 'rust/src/main.rs'],
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
    javascript: ['js/src/grammar-reverse.js', 'js/src/grammar-links.js', 'js/src/grammar-lossless.js'],
    rust: ['rust/src/grammar/reverse.rs', 'rust/src/grammar/interchange/links.rs', 'rust/src/grammar/interchange/lossless.rs'],
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
    javascript: ['js/src/grammar-lowering.js'],
    rust: [
      'rust/src/grammar/interchange/lowering/mod.rs',
      'rust/src/grammar/interchange/lowering/encoding.rs',
      'rust/src/grammar/interchange/lowering/metadata.rs',
    ],
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
    javascript: ['js/src/grammar-round-trip.js'],
    rust: ['rust/src/grammar/round_trip.rs'],
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
    fixture: 'parity/fixtures/grammar-merge.json',
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
    javascript: ['js/src/grammar-merge.js'],
    rust: ['rust/src/grammar/merge/mod.rs'],
  },
  {
    id: 'I195-MERGE-MEANING-AWARE-DEDUPLICATION',
    area: 'automatic-merge',
    specification: 'automatic-merging-concept-recognition-deduplication-and-renaming',
    fixture: 'parity/fixtures/grammar-merge.json',
    construct: 'deduplication by meaning',
    expectedBehavior:
      'Duplicates are recognized by meaning-aware normalization, recursive graph and alpha-equivalence, lexical and parser semantics and justified mappings. Name similarity and generated samples only nominate candidates, differently named equivalent concepts are merged, and identically named non-equivalent concepts are not.',
    assertions: ['equivalentConceptsMerged', 'homonymsKeptDistinct', 'samplesOnlyNominate', 'equivalenceJustified'],
    javascript: ['js/src/grammar-merge.js'],
    rust: ['rust/src/grammar/merge/mod.rs'],
  },
  {
    id: 'I195-MERGE-UNCERTAINTY-PRESERVED',
    area: 'automatic-merge',
    specification: 'automatic-merging-concept-recognition-deduplication-and-renaming',
    fixture: 'parity/fixtures/grammar-merge.json',
    construct: 'uncertain matches kept explicit',
    expectedBehavior:
      'Uncertain matches, distinct meanings and version, dialect or ambiguity alternatives stay explicit instead of being conflated. An unresolved required equivalence is reported as failing work.',
    assertions: ['uncertainMatchesNotConflated', 'alternativesExplicit', 'unresolvedEquivalenceFails'],
    javascript: ['js/src/grammar-merge.js'],
    rust: ['rust/src/grammar/merge/mod.rs'],
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
    fixture: 'parity/fixtures/grammar-merge.json',
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
    javascript: ['js/src/grammar-merge.js'],
    rust: ['rust/src/grammar/merge/mod.rs'],
  },
  {
    id: 'I195-MERGE-QUALITY-EVIDENCE',
    area: 'automatic-merge',
    specification: 'automatic-merging-concept-recognition-deduplication-and-renaming',
    fixture: 'parity/fixtures/merge-quality-evidence.json',
    construct: 'published merge quality comparison against the source grammars',
    expectedBehavior:
      'A published comparison against every inventoried source grammar measures construct coverage and correctness, preserved unique features and shared reuse, recovery quality, execution time and memory on independent corpora, never on self-generated fixtures alone.',
    assertions: ['coverageMeasured', 'correctnessMeasured', 'recoveryMeasured', 'timeAndMemoryMeasured', 'comparisonPublished'],
    javascript: ['js/scripts/build-merge-quality-evidence.mjs', 'js/src/grammar-concepts.js'],
    rust: ['rust/tests/merge_quality.rs', 'rust/src/grammar_concepts.rs'],
  },
  {
    id: 'I195-MERGE-REAL-RECONCILIATION',
    area: 'automatic-merge',
    specification: 'automatic-merging-concept-recognition-deduplication-and-renaming',
    fixture: 'parity/grammars/reconcile/pairs.json',
    construct: 'merging reconciles corresponding rules instead of concatenating sources',
    expectedBehavior:
      'Merging two or more source grammars of one language reconciles their corresponding rules into shared rules, so every merged catalog grammar reports a non-zero sharedRules count, and a test fails when the merge only concatenates the sources side by side, in both runtimes.',
    assertions: ['correspondingRulesReconciled', 'sharedRulesNonZero', 'concatenationRejected'],
    javascript: ['js/src/grammar-merge.js', 'js/src/grammar-reconcile.js'],
    rust: ['rust/src/grammar/merge/mod.rs', 'rust/src/grammar/merge/reconcile.rs'],
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
    id: 'I195-NAMING-NATIVE-GRAMMARS',
    area: 'readable-naming',
    specification: 'readable-english-names',
    fixture: 'parity/naming/canonical-concepts.json',
    construct: 'readable English rule names in the native grammars',
    expectedBehavior:
      'Every rule of every native grammar has a readable English name. A rule renamed from its tree-sitter kind keeps that kind as a source-name alias, and the catalog maps it back for the oracle comparison. The naming check covers every native rule name and every concept reference.',
    assertions: ['nativeRuleNamesAreEnglish', 'sourceNamesKeptAsAliases', 'oracleKindsPreserved', 'namingCheckCoversNativeGrammars'],
    javascript: ['js/src/grammar-concepts.js', 'js/src/grammar-links.js', 'js/scripts/issue-195-naming.mjs'],
    rust: ['rust/src/grammar_concepts.rs', 'rust/src/grammar/interchange/links.rs'],
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
    javascript: [
      'js/src/program-translation.js',
      'js/src/translation/aborts.js',
      'js/src/translation/output.js',
      'js/src/translation/emit-common.js',
      'js/src/translation/emit-lean.js',
      'js/src/translation/emit-rocq.js',
      'js/src/translation/emit-javascript.js',
      'js/src/translation/emit-rust.js',
    ],
    rust: [
      'rust/src/semantic_translation.rs',
      'rust/src/translation/aborts.rs',
      'rust/src/translation/output.rs',
      'rust/src/translation/emit_common.rs',
      'rust/src/translation/emit_lean.rs',
      'rust/src/translation/emit_rocq.rs',
      'rust/src/translation/emit_javascript.rs',
      'rust/src/translation/emit_rust/declarations.rs',
    ],
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
    javascript: [
      'js/src/program-translation.js',
      'js/src/translation/emit-common.js',
      'js/src/translation/emit-lean.js',
      'js/src/translation/emit-rocq.js',
      'js/src/translation/emit-javascript.js',
      'js/src/translation/emit-rust.js',
    ],
    rust: [
      'rust/src/semantic_translation.rs',
      'rust/src/translation/emit_common.rs',
      'rust/src/translation/emit_lean.rs',
      'rust/src/translation/emit_rocq.rs',
      'rust/src/translation/emit_javascript.rs',
      'rust/src/translation/emit_rust/declarations.rs',
    ],
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
    fixture: 'parity/fixtures/rml-pr184-workloads.json',
    construct: 'relative-meta-logic workloads on clean installed artifacts',
    expectedBehavior:
      'The relative-meta-logic workloads (parse -> common links -> semantic transformation or translation -> emission) run against clean installed artifacts of both packages, reuse shared concepts and preserve language-specific distinctions, without meta-language taking over RML foundations or proof authority.',
    assertions: ['workloadsRunOnInstalledArtifacts', 'sharedConceptsReused', 'distinctionsPreserved', 'foundationAuthorityStaysInRml'],
    javascript: [
      'js/scripts/run-rml-pr184-workloads.mjs',
      'js/scripts/issue-195-rml-workloads.mjs',
      'js/scripts/issue-195-rml-workload-probes.mjs',
      'js/tests/issue-195-downstream-rml-workloads.test.js',
    ],
    rust: [
      'js/scripts/run-rml-pr184-workloads.mjs',
      'js/scripts/issue-195-rml-workload-probes.mjs',
      'rust/tests/unit/issue_195_downstream_rml_workloads.rs',
    ],
  },
  {
    id: 'I195-DOWNSTREAM-RML-PR184-AUDIT',
    area: 'downstream-consumers',
    specification: 'downstream-consumers',
    fixture: 'docs/downstream-consumers.md',
    construct: 'current relative-meta-logic pull request 184 workload inventory',
    expectedBehavior:
      'The consumer audit inspects the current relative-meta-logic pull request 184 implementation, records its pinned revision and actual required workloads, and maps every workload to implementation and executable acceptance rows rather than relying only on the older default-branch audit.',
    assertions: ['currentPullRequestRevisionPinned', 'actualWorkloadsInventoried', 'eachWorkloadMappedToExecutableAcceptance'],
    tooling: ['docs/downstream-consumers.md', 'js/scripts/issue-195-rml-pr184.mjs', 'parity/fixtures/rml-pr184-workloads.json'],
  },
  {
    id: 'I195-DOWNSTREAM-FORMAL-AI-WORKLOADS',
    area: 'downstream-consumers',
    specification: 'downstream-consumers',
    fixture: 'parity/fixtures/formal-ai-workloads.json',
    construct: 'formal-ai workloads on clean installed artifacts',
    expectedBehavior:
      'The link-assistant/formal-ai workloads identified by the consumer matrix run against clean installed artifacts of both packages through the common links representation.',
    assertions: ['workloadsRunOnInstalledArtifacts', 'sharedConceptsReused', 'distinctionsPreserved'],
    javascript: [
      'js/scripts/run-formal-ai-workloads.mjs',
      'js/scripts/issue-195-formal-ai-workloads.mjs',
      'js/scripts/issue-195-formal-ai-workload-probes.mjs',
      'js/scripts/issue-195-formal-ai-consumer.mjs',
      'js/tests/issue-195-downstream-formal-ai-workloads.test.js',
    ],
    rust: [
      'js/scripts/run-formal-ai-workloads.mjs',
      'js/scripts/issue-195-formal-ai-rust-probe.mjs',
      'rust/tests/unit/issue_195_downstream_formal_ai_workloads.rs',
    ],
  },
  {
    id: 'I195-DOWNSTREAM-TYPESCRIPT-TRANSLATIONS',
    area: 'downstream-consumers',
    specification: 'downstream-consumers',
    fixture: 'docs/downstream-consumers.md',
    construct: 'Rust, JavaScript and TypeScript semantic projection for formal-ai',
    expectedBehavior:
      'Both runtime packages translate the required Rust, JavaScript and TypeScript projection pairs through the common editable links representation, execute formal-ai projection rules, and preserve the required semantics on clean installed artifacts. A TypeScript concrete syntax tree alone does not count as translation.',
    assertions: ['allRequiredProjectionPairsTranslate', 'consumerRulesExecute', 'semanticOutcomesPreserved', 'cleanInstalledArtifactsUsed'],
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
    tooling: [
      'js/scripts/issue-195-proof-obligations.mjs',
      'js/scripts/issue-195-oracle-mapping.mjs',
      'js/scripts/issue-195-acceptance-lib.mjs',
      'js/scripts/issue-195-sources.mjs',
      'js/scripts/issue-195-naming.mjs',
      'js/scripts/dependency-inventory.mjs',
      'js/src/grammar-lowering.js',
      'js/src/grammar-reverse.js',
    ],
  },
  {
    id: 'I195-ACCEPTANCE-REQUIRED-MERGE-CHECK',
    area: 'independent-acceptance',
    specification: 'acceptance-and-evidence',
    construct: 'enforced full requirements merge check on the default branch',
    expectedBehavior:
      'The default branch has an active repository or organization rule that requires the Full Requirements Aggregate check to pass before a pull request can merge. The rule is live state that exists only outside the pull request, so a non-blocking report on main inspects it after merge with the workflow token: the active non-bypassable rule, the strict required aggregate, an open pull request whose failed aggregate blocks its merge, and the separate release-delivery checkpoint. No pull request check depends on it, and no row requires its own aggregate to fail.',
    assertions: ['activeRuleTargetsDefaultBranch', 'fullAggregateRequired', 'failingCheckBlocksMerge', 'publishedDeliverySeparatelyVerified'],
    tooling: ['js/scripts/issue-195-merge-enforcement.mjs', 'js/scripts/check-issue-195-merge-enforcement.mjs'],
    group: 'merge-enforcement',
    checkpoint: 'post-merge',
  },
  {
    id: 'I195-ACCEPTANCE-PR-CHECKS-PASSABLE',
    area: 'independent-acceptance',
    specification: 'acceptance-and-evidence',
    construct: 'pull request checks that can all pass before merge',
    expectedBehavior:
      'Every PR check must pass. A red check is a defect to fix now; unfinished requirements are work to implement, not a reason for red CI. The pre-merge aggregate is green on a fixture in which every pre-merge row passes, and no pre-merge row needs its own aggregate to fail. No pull request step depends on post-merge state (live rulesets, live registries, live comment edits or published packages): those run on main as non-blocking reports, and a scheduled workflow on main refreshes the dependencies and opens its own pull request.',
    assertions: [
      'everyPreMergeRowPassingGivesGreenAggregate',
      'noPreMergeRowNeedsItsOwnAggregateToFail',
      'pullRequestChecksUseNoPostMergeState',
      'postMergeReportsAreNonBlocking',
      'dependencyRefreshScheduledOnMain',
    ],
    tooling: [
      'js/scripts/issue-195-acceptance-lib.mjs',
      'js/scripts/issue-195-evidence-stages.mjs',
      '.github/workflows/ci.yml',
      '.github/workflows/js.yml',
      '.github/workflows/dependency-refresh.yml',
    ],
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
    id: 'I195-RESOURCE-CI-COMPILE-GATE',
    area: 'resource-limits',
    specification: 'resource-limits',
    construct: 'fast compile gate ahead of every compiling CI job',
    expectedBehavior:
      'A cargo check of all targets and features runs first in the Rust workflow, and the test, minimum-version, coverage, fresh-merge and build jobs need it, so a compile error fails once, within minutes, instead of in every job.',
    assertions: ['compileGateRunsAllTargetsAndFeatures', 'compilingJobsNeedTheCompileGate'],
    tooling: ['.github/workflows/rust.yml'],
  },
  {
    id: 'I195-RESOURCE-CI-TEST-MATRIX',
    area: 'resource-limits',
    specification: 'resource-limits',
    construct: 'test execution split into bounded matrix jobs',
    expectedBehavior:
      'The Rust tests run as matrix jobs by test target and integration filter, and the JavaScript tests as matrix jobs by file group, each with its own timeout. Every JavaScript test file belongs to exactly one group, and the required check names stay valid.',
    assertions: ['rustTestsSplitBySuite', 'javascriptTestsSplitByGroup', 'everyTestFileInExactlyOneGroup', 'matrixJobsHaveTimeouts'],
    tooling: ['.github/workflows/rust.yml', '.github/workflows/js.yml', 'js/scripts/test-groups.mjs'],
  },
  {
    id: 'I195-RESOURCE-CI-EVIDENCE-STAGES',
    area: 'resource-limits',
    specification: 'resource-limits',
    construct: 'checkpoint evidence produced by separate stage jobs',
    expectedBehavior:
      'The JavaScript suite, the Rust suite, the runtime parity check, the native translations (a matrix by target language) and delivery each run as their own CI job and upload their own evidence, and the post-merge merge-enforcement report runs as its own job on main. The Full Requirements Aggregate only merges and evaluates the stage outputs, and a stage that fails or never reports is one gate error naming it, not a failure of every row it feeds.',
    assertions: [
      'stagesRunAsSeparateJobs',
      'nativeTranslationsMatrixByTarget',
      'eachStageUploadsItsEvidence',
      'aggregateOnlyMergesAndEvaluates',
      'failedStageIsOneGateError',
    ],
    tooling: [
      '.github/workflows/ci.yml',
      'js/scripts/issue-195-evidence-stages.mjs',
      'js/scripts/run-issue-195-evidence.mjs',
    ],
  },
  {
    id: 'I195-RESOURCE-CI-CONCURRENCY',
    area: 'resource-limits',
    specification: 'resource-limits',
    construct: 'superseded workflow runs cancelled',
    expectedBehavior:
      'Every workflow declares a concurrency group per workflow and ref, so a new push cancels the runs it supersedes instead of running them all.',
    assertions: ['everyWorkflowHasConcurrencyGroup', 'supersededRunsCancelled'],
    tooling: ['.github/workflows/'],
  },
  {
    id: 'I195-RESOURCE-AGENT-RULES',
    area: 'resource-limits',
    specification: 'resource-limits',
    construct: 'targeted local checks and resource rules for agents and contributors',
    expectedBehavior:
      'AGENTS.md and CONTRIBUTING.md tell agents and contributors to run only the checks that cover a change, with bounded build parallelism, to never run the full acceptance pipeline, the native toolchain matrix, clean-consumer installs, workload clones, coverage or whole-registry experiments locally, to work in batches, and to clean up after each one; the CI workflows run the full verification.',
    assertions: [
      'targetedChecksDocumented',
      'boundedParallelismDocumented',
      'forbiddenLocalRunsListed',
      'batchingAndCleanupDocumented',
      'fullVerificationDelegatedToCi',
    ],
    tooling: ['AGENTS.md', 'CONTRIBUTING.md'],
  },
  {
    id: 'I195-RESOURCE-BOUNDED-SEQUENTIAL-EVIDENCE',
    area: 'resource-limits',
    specification: 'resource-limits',
    construct: 'evidence suites run one at a time with bounded build parallelism',
    expectedBehavior:
      'The evidence runner runs its stages one after another, gives every child process CARGO_BUILD_JOBS=2, RUST_TEST_THREADS=2 and CARGO_INCREMENTAL=0 unless the caller set them, builds only the default Rust features, and runs each JavaScript test group as its own process.',
    assertions: ['stagesRunSequentially', 'boundedBuildEnvironment', 'defaultFeaturesOnly', 'testGroupsRunSeparately'],
    tooling: ['js/scripts/run-issue-195-evidence.mjs', 'js/scripts/issue-195-evidence-stages.mjs'],
  },
  {
    id: 'I195-RESOURCE-LAZY-GRAMMARS',
    area: 'resource-limits',
    specification: 'resource-limits',
    construct: 'WebAssembly grammars loaded on first use',
    expectedBehavior:
      'Importing the JavaScript package loads no grammar; each WebAssembly grammar is decompressed and compiled when a language first needs it and cached by its identifier.',
    assertions: ['noGrammarLoadedOnImport', 'grammarLoadedOnFirstUseAndCached'],
    tooling: ['js/src/programming-language-parser.js'],
  },
  {
    id: 'I195-RESOURCE-GRAMMAR-TIERING-BUDGET',
    area: 'resource-limits',
    specification: 'resource-limits',
    construct: 'grammar code kept off the optimizing compiler',
    expectedBehavior:
      'Grammar modules are compiled with a tiering budget no parse exhausts, so V8 does not recompile their generated lexers with TurboFan after parsing ends, and an idle process does not grow by gigabytes.',
    assertions: ['idleGrowthBoundedAfterParsing'],
    tooling: ['js/src/grammar-tiering.js'],
  },
  {
    id: 'I195-RESOURCE-INLINE-PARSER-REUSE',
    area: 'resource-limits',
    specification: 'resource-limits',
    construct: 'one Markdown inline parser for every inline region',
    expectedBehavior:
      'Markdown inline and table-cell regions are parsed with one reused inline parser instead of a parser per region, and each region tree is still deleted once it is converted.',
    assertions: ['oneInlineParserReused', 'inlineTreesDeleted'],
    tooling: ['js/src/programming-language-parser.js'],
  },
  {
    id: 'I195-RESOURCE-SOURCE-BOUNDARIES',
    area: 'resource-limits',
    specification: 'resource-limits',
    construct: 'source offset index with interval checkpoints',
    expectedBehavior:
      'The map from string offsets to UTF-8 bytes and points keeps one checkpoint per fixed interval of characters and searches it, instead of an entry per character, and agrees with the per-character map at every offset and byte.',
    assertions: ['checkpointIndexMatchesPerCharacterMap', 'oneCheckpointPerInterval'],
    tooling: ['js/src/source-boundaries.js'],
  },
  {
    id: 'I195-RESOURCE-PARITY-DIGESTS',
    area: 'resource-limits',
    specification: 'resource-limits',
    construct: 'runtime parity evidence as entry digests',
    expectedBehavior:
      'The Rust runtime probe streams its observation as NDJSON to a file, read without a large output buffer; the runtimes are compared entry by entry; and the kept evidence is one digest per section entry and runtime plus the full entries only where the runtimes differ.',
    assertions: ['probeStreamsNdjson', 'noLargeOutputBuffer', 'entriesComparedOneAtATime', 'fullEntriesOnlyWhereDifferent'],
    tooling: [
      'js/scripts/check-issue-195-runtime-parity.mjs',
      'js/scripts/issue-195-parity-evidence.mjs',
      'rust/examples/issue_195_runtime_probe.rs',
    ],
  },
  {
    id: 'I195-RESOURCE-CACHE-CLEANUP-WRAPPER',
    area: 'resource-limits',
    specification: 'resource-limits',
    construct: 'cache cleanup wrapper as the documented way to run long local commands',
    expectedBehavior:
      'AGENTS.md and docs/cache-cleanup.md document scripts/with-cache-cleanup.mjs as the way to run local cargo and npm build and test commands and tell sessions to run the cleanup between batches; the wrapper bounds build parallelism and cleans up even when the command fails.',
    assertions: ['wrapperDocumentedForLocalRuns', 'cleanupBetweenBatchesDocumented', 'wrapperBoundsParallelism'],
    tooling: ['scripts/with-cache-cleanup.mjs', 'AGENTS.md', 'docs/cache-cleanup.md'],
  },
  {
    id: 'I195-RESOURCE-NATIVE-OUTPUT-CLEANUP',
    area: 'resource-limits',
    specification: 'resource-limits',
    construct: 'native compiler outputs deleted once each cell is recorded',
    expectedBehavior:
      'After a native validation stage records a target cell it deletes the compiler outputs no cell cites (.rmeta, .olean, .vo and similar) and keeps only the cited translations, reproduction scripts and logs.',
    assertions: ['uncitedOutputsDeleted', 'citedEvidenceKept'],
    tooling: ['js/scripts/run-issue-195-evidence.mjs', 'js/scripts/issue-195-evidence-stages.mjs'],
  },
  {
    id: 'I195-CI-RUST-AFTER-JAVASCRIPT',
    area: 'continuous-integration',
    specification: 'continuous-integration',
    construct: 'Rust jobs gated on every JavaScript job',
    expectedBehavior:
      'One CI workflow calls the JavaScript workflow and then the Rust workflow, which needs it: no Rust job starts before every JavaScript job passed, and the Rust jobs show as skipped when JavaScript fails. The required check names stay valid.',
    assertions: ['rustWorkflowNeedsJavaScript', 'calledWorkflowsHaveNoOwnPushOrPullRequestTriggers', 'skippedWhenJavaScriptFails'],
    tooling: ['.github/workflows/ci.yml', '.github/workflows/js.yml', '.github/workflows/rust.yml'],
  },
  {
    id: 'I195-CI-ACCEPTANCE-STAGE-ORDER',
    area: 'continuous-integration',
    specification: 'continuous-integration',
    construct: 'Rust acceptance stages after the JavaScript stages',
    expectedBehavior:
      'In the CI workflow the Rust acceptance suite stage and the Rust native translations are separate jobs that need the matching JavaScript jobs and run only when those succeeded; no matrix runs a JavaScript and a Rust stage side by side.',
    assertions: ['rustStagesNeedJavaScriptStages'],
    tooling: ['.github/workflows/ci.yml'],
  },
  {
    id: 'I195-CI-REPORT-EVERY-FAILURE',
    area: 'continuous-integration',
    specification: 'continuous-integration',
    construct: 'every failure of a job reported in one run',
    expectedBehavior:
      'Every cargo test run uses --no-fail-fast, every check step after the first of a job runs unless the run was cancelled, and cargo fmt, clippy and doc are independent steps, so one failure does not hide the others.',
    assertions: ['cargoTestNoFailFast', 'checkStepsRunUnlessCancelled', 'lintStepsIndependent'],
    tooling: ['.github/workflows/js.yml', '.github/workflows/rust.yml', '.github/workflows/ci.yml'],
  },
  {
    id: 'I195-CI-ACCEPTANCE-AFTER-JAVASCRIPT',
    area: 'continuous-integration',
    specification: 'continuous-integration',
    construct: 'acceptance stages started only after every JavaScript job passed',
    expectedBehavior:
      'The issue 195 acceptance stages are jobs of the CI workflow that need its JavaScript jobs, so no acceptance stage starts before every JavaScript job passed, and a job that runs despite a failure tests the JavaScript result itself; there is no separate acceptance workflow running beside them.',
    assertions: ['acceptanceJobsNeedJavaScript', 'noSeparateAcceptanceWorkflow'],
    tooling: ['.github/workflows/ci.yml'],
  },
  {
    id: 'I195-CI-SKIPPED-STAGE-ONE-GATE-ERROR',
    area: 'continuous-integration',
    specification: 'continuous-integration',
    construct: 'a stage skipped because JavaScript failed is one gate error',
    expectedBehavior:
      'A stage skipped because its JavaScript stage failed is folded into the one gate error that names the blocking JavaScript failure instead of being reported as a second missing stage, a stage missing for another reason is still reported, and the Full Requirements Aggregate reports a failed JavaScript job as one error naming the failed jobs.',
    assertions: ['skippedStageJoinsBlockingError', 'missingStageStillReported', 'failedJavaScriptIsOneGateError'],
    tooling: ['js/scripts/issue-195-evidence-stages.mjs', '.github/workflows/ci.yml'],
  },
  {
    id: 'I195-RESOURCE-PARSE-MEMORY-BUDGET',
    area: 'resource-limits',
    specification: 'resource-limits',
    construct: 'a hard memory budget for every parse',
    expectedBehavior:
      'Every parse counts the memo cells it keeps, across its runs, repair rounds and embedded grammars, against one budget (2,000,000 cells by default, `memoryLimit` / `memory_limit` to change it); a parse that needs more ends with a `memoryBudget` rejection naming the limit instead of growing until the process runs out of memory, and under a capped heap or address space a large parse ends with that diagnostic, in both runtimes.',
    assertions: ['budgetExceededIsRejection', 'budgetSharedAcrossRepairRounds', 'rejectionNamesLimit', 'cappedHeapParseEndsWithDiagnostic'],
    javascript: ['js/src/grammar-runtime/executor.js', 'js/tests/issue-195-parse-memory-budget.test.js'],
    rust: ['rust/src/grammar/feature_runtime/executor.rs', 'rust/tests/unit/issue_195_parse_memory_budget.rs'],
  },
  {
    id: 'I195-DEVELOPMENT-RULES-DOCUMENTED',
    area: 'javascript-first',
    specification: 'javascript-first-and-self-translation',
    construct: 'JavaScript-first development rules',
    expectedBehavior:
      'docs/vision.md, AGENTS.md and CONTRIBUTING.md state the development rules: JavaScript first and Rust ported in one batch, by self-translation; lossless round trips; the automated grammar pipeline instead of hand work; tree-sitter only as a test oracle; decorators at every level; bulk changes without idle waiting; resource limits; the JavaScript → Rust CI order; and CI that reports every failure.',
    assertions: ['javascriptFirstDocumented', 'selfTranslationDocumented', 'decoratorsDocumented', 'bulkWorkDocumented', 'ciOrderDocumented', 'failLateDocumented'],
    tooling: ['docs/vision.md', 'AGENTS.md', 'CONTRIBUTING.md'],
  },
  {
    id: 'I195-SELF-TRANSLATION-TOOL',
    area: 'javascript-first',
    specification: 'javascript-first-and-self-translation',
    fixture: 'parity/self-translation/cases.lino',
    construct: 'JavaScript/TypeScript ↔ Rust self-translation library API and CLI',
    expectedBehavior:
      'Both packages expose a library API and a CLI command that read JavaScript or TypeScript into meta-language links and write Rust, and read Rust and write JavaScript or TypeScript; translated meta-language modules pass cargo check and clippy.',
    assertions: ['libraryApiInBothPackages', 'cliInBothPackages', 'javascriptToRust', 'rustToJavaScript', 'generatedRustCompiles'],
    javascript: ['js/src/self-translation.js', 'js/src/cli.js', 'js/tests/self-translation.test.js'],
    rust: ['rust/src/self_translation.rs', 'rust/src/main.rs', 'rust/tests/unit/self_translation.rs'],
  },
  {
    id: 'I195-SELF-TRANSLATION-ROUND-TRIP',
    area: 'javascript-first',
    specification: 'javascript-first-and-self-translation',
    fixture: 'parity/self-translation/cases.lino',
    construct: 'lossless self-translation round trips',
    expectedBehavior:
      'JavaScript → links → JavaScript and Rust → links → Rust are byte-identical on meta-language\'s own sources, and JavaScript → Rust → JavaScript and Rust → JavaScript → Rust preserve behavior, keeping comments, layout and names through provenance where the target can express them.',
    assertions: ['sameLanguageByteIdentical', 'crossLanguageBehaviorPreserved', 'provenanceKeepsCommentsAndNames'],
    javascript: ['js/src/self-translation.js', 'js/tests/self-translation.test.js'],
    rust: ['rust/src/self_translation.rs', 'rust/tests/unit/self_translation.rs'],
  },
  {
    id: 'I195-SELF-TRANSLATION-SHARED-CORPUS',
    area: 'javascript-first',
    specification: 'javascript-first-and-self-translation',
    fixture: 'parity/self-translation/cases.lino',
    construct: 'shared Links Notation corpus checked in both runtimes',
    expectedBehavior:
      'As in relative-meta-logic PR #184, every translated module has shared corpus cases whose expected outputs are written in Links Notation and checked in both runtimes, and the translated Rust of each module is measured against the hand-written source with the difference published per module, with and without the shared emitter decorators; a corpus case translated with its decorators matches its hand-written Rust function for function and still restores its source.',
    assertions: ['expectedOutputsInLinksNotation', 'checkedInBothRuntimes', 'differencePerModulePublished', 'decoratorsMatchHandWritten'],
    javascript: ['js/scripts/generate-self-translation-cases.mjs', 'js/scripts/generate-self-translation-report.mjs', 'js/tests/self-translation.test.js', 'parity/self-translation/decorators.lino'],
    rust: ['rust/tests/unit/self_translation.rs'],
  },
  {
    id: 'I195-SELF-TRANSLATION-CARRIED-ZERO',
    area: 'javascript-first',
    specification: 'javascript-first-and-self-translation',
    fixture: 'parity/self-translation/cases.lino',
    construct: 'every self-translated item translated, none carried',
    expectedBehavior:
      'The self-translation report carries no item over unchanged: every function, type and full program of the translated modules is translated, the generated Rust replaces the hand-written Rust it was measured against, the decorators make it match that style, and the round-trip tests run on the translated items.',
    assertions: ['carriedItemsZero', 'generatedRustReplacesHandWritten', 'decoratorsMatchStyle', 'roundTripsUseTranslatedItems', 'fullProgramsTranslate'],
    javascript: ['js/src/self-translation.js', 'js/scripts/generate-self-translation-report.mjs'],
    rust: ['rust/src/self_translation.rs', 'rust/tests/unit/self_translation.rs'],
  },
  {
    id: 'I195-DECORATORS-EVERY-LEVEL',
    area: 'javascript-first',
    specification: 'javascript-first-and-self-translation',
    fixture: 'parity/decorators/cases.json',
    construct: 'one decorator API at every level',
    expectedBehavior:
      'One decorator API extends the importer, grammar rules, merge decisions, concept mappings, the executor and recovery, CST → AST, transformations, emitters and translation rules; decorators compose in a defined order, are stored as links data both runtimes share, and can be removed.',
    assertions: ['decoratorAtEveryLevel', 'compositionOrderDeterministic', 'storedAsLinksData', 'removable'],
    javascript: ['js/src/decorators.js', 'js/src/grammar-decorators.js'],
    rust: ['rust/src/decorators.rs', 'rust/src/grammar/decorators.rs'],
  },
  {
    id: 'I195-PARITY-FEATURE-COMPLETENESS',
    area: 'javascript-first',
    specification: 'javascript-first-and-self-translation',
    construct: 'every public JavaScript feature listed and present in Rust',
    expectedBehavior:
      'parity/language-features.json lists every public JavaScript export, and a test fails when Rust lacks one or when the two runtimes produce different observable output for one.',
    assertions: ['everyPublicExportListed', 'rustHasEveryFeature', 'observableOutputEqual'],
    javascript: ['js/tests/export-parity.test.js', 'js/scripts/generate-export-parity.mjs'],
    rust: ['rust/tests/unit/javascript_export_parity.rs', 'rust/tests/unit/export_parity_corpus.rs'],
  },
  {
    id: 'I195-GRAMMAR-BULK-PIPELINE',
    area: 'native-grammar',
    specification: 'native-merged-grammars',
    fixture: 'docs/grammar/bulk-pipeline.md',
    construct: 'bulk import of every catalog grammar with a published matrix',
    expectedBehavior:
      'One pipeline run imports the grammar.json and ANTLR grammars-v4 sources of every catalog language, merges them, and publishes a matrix of which languages import, execute and match their oracle and which executor or scanner features each one is missing.',
    assertions: ['everyCatalogLanguageImported', 'matrixPublished', 'missingFeaturesListed'],
    tooling: ['js/scripts/run-grammar-bulk-pipeline.mjs', 'parity/grammars-v4-sources.json', '.github/workflows/ci.yml'],
  },
  {
    id: 'I195-GRAMMAR-DECLARED-SETTLING',
    area: 'native-grammar',
    specification: 'native-merged-grammars',
    fixture: 'docs/grammar/feature-union.md',
    construct: 'declared, data-driven conflict and precedence settling',
    expectedBehavior:
      'The LR-style conflict, fork and associativity settling is declared grammar data the tree-sitter, ANTLR and PEG imports share, not executor host code for tree-sitter alone, in both runtimes.',
    assertions: ['settlingDeclaredInGrammar', 'sharedByEveryImporter', 'noHostCodeSettling'],
    javascript: ['js/src/grammar-feature-forms.js', 'js/src/grammar-runtime/load.js', 'js/src/grammar-runtime/executor.js'],
    rust: ['rust/src/grammar/feature.rs', 'rust/src/grammar/feature_runtime/load.rs', 'rust/src/grammar/feature_runtime/ordering.rs'],
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
export function buildVisionRequirements(fixtureCatalog, { requirement, verification, pinnedFixture, source, sourceByRequirement = {} }) {
  return VISION_REQUIREMENTS.map((row) => {
    const fixtureId = `planned:repository-directive:${row.id.toLowerCase()}`;
    fixtureCatalog[fixtureId] = row.fixture
      ? pinnedFixture(row.fixture, `${VISION_SPECIFICATION}#${row.specification}: ${row.construct}`)
      : pinnedFixture(VISION_SPECIFICATION, `#${row.specification}: ${row.construct}`);
    const requiredRuntimes = RUNTIMES.filter((runtime) => runtime in row);
    return requirement({
      id: row.id,
      source: sourceByRequirement[row.id] ?? source,
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
    workflow: '.github/workflows/ci.yml',
    evidenceGroups: [...new Set(entry.verifications.map((cell) => evidenceGroupFor(entry, cell)))].sort(),
    packages: entry.requiredRuntimes.map((runtime) => PACKAGES[runtime] ?? runtime),
    downstream: consumersByRow === null
      ? DOWNSTREAM[entry.id] ?? null
      : [...new Set([...(DOWNSTREAM[entry.id] ?? []), ...(consumersByRow.get(entry.id) ?? [])])].sort(),
  };
}
