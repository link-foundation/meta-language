import { createHash } from 'node:crypto';
import { access, readFile } from 'node:fs/promises';
import path from 'node:path';

export const ISSUE_195_MANIFEST_SCHEMA_VERSION = 1;

export const ISSUE_195_SOURCES = Object.freeze({
  issue: 'https://github.com/link-foundation/meta-language/issues/195',
  clarification:
    'https://github.com/link-foundation/meta-language/pull/196#issuecomment-5795832509',
  acceptanceGate:
    'https://github.com/link-foundation/meta-language/pull/196#issuecomment-5798732287',
});

const FOUR_LANGUAGE_DETAILS = Object.freeze({
  JavaScript: { version: 'ECMAScript 2026', edition: 'ECMA-262, 17th edition' },
  Rust: { version: '1.98.1', edition: '2024' },
  Lean: { version: '4.34.0', edition: 'Lean 4' },
  Rocq: { version: '9.3.0', edition: 'Vernacular' },
});

const EXTENSIONS = Object.freeze({
  LiNo: ['.lino'],
  txt: ['.txt', '.text'],
  Markdown: ['.md', '.markdown'],
  PDF: ['.pdf'],
  DOCX: ['.docx'],
  JavaScript: ['.js', '.mjs', '.cjs'],
  Rust: ['.rs'],
  Lean: ['.lean'],
  Rocq: ['.v'],
  Python: ['.py'],
  C: ['.c', '.h'],
  'C++': ['.cc', '.cpp', '.cxx', '.hpp'],
  'C#': ['.cs'],
  Java: ['.java'],
  TypeScript: ['.ts'],
  TSX: ['.tsx'],
  'Visual Basic': ['.vb'],
  'Delphi/Object Pascal': ['.pas', '.dpr'],
  Go: ['.go'],
  R: ['.r'],
  Ruby: ['.rb'],
  PHP: ['.php'],
  Swift: ['.swift'],
  Kotlin: ['.kt', '.kts'],
  Scala: ['.scala'],
  Lua: ['.lua'],
  Perl: ['.pl', '.pm'],
  HTML: ['.html', '.htm'],
  CSS: ['.css'],
  JSON: ['.json'],
  YAML: ['.yaml', '.yml'],
  TOML: ['.toml'],
  XML: ['.xml'],
  DTD: ['.dtd'],
  INI: ['.ini'],
  'Protocol Buffers': ['.proto'],
  GraphQL: ['.graphql', '.gql'],
  CSV: ['.csv'],
  JSON5: ['.json5'],
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

function plannedFixture(selector) {
  return { path: null, selector, sha256: null };
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
    source,
    area,
    scope,
    expectedBehavior,
    requiredRuntimes,
    implementationEntryPoints,
    verifications,
  };
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
    fixtureCatalog[negativeFixtureId] = plannedFixture(
      `malformed and recovery corpus for ${language.name}`,
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
      fixtureCatalog[fixtureId] = plannedFixture(
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
          implementationEntryPoints: { javascript: null, rust: null },
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
    fixtureCatalog[fixtureId] = plannedFixture(
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
      implementationEntryPoints: { javascript: null, rust: ['rust/src/grammar'] },
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

function buildFourLanguageCorpusRequirements(fixtureCatalog) {
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
      fixtureCatalog[fixtureId] = plannedFixture(`${language} ${construct}`);
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
            extensions: EXTENSIONS[language],
          },
          expectedBehavior:
            suffix === 'CONFORMANCE'
              ? 'Both runtimes execute mapped external corpora and real projects with provenance and independently asserted construct coverage, including malformed, Unicode, and mixed-language cases.'
              : 'Both runtimes execute reproducible generative tests against an independent oracle; self-consistency between the parser and its own emitter is insufficient.',
          requiredRuntimes: ['javascript', 'rust'],
          implementationEntryPoints: { javascript: null, rust: null },
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

function buildSemanticRequirements(fixtureCatalog) {
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
      fixtureCatalog[fixtureId] = plannedFixture(
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
            extensions: EXTENSIONS[language],
          },
          expectedBehavior:
            'Both runtimes represent this construct with project-aware symbol identity and language-specific distinctions; missing context is diagnosed and valid context enables the behavior.',
          requiredRuntimes: ['javascript', 'rust'],
          implementationEntryPoints: { javascript: null, rust: null },
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

function buildTransformationRequirements(fixtureCatalog) {
  const operations = ['query', 'insert', 'delete', 'replace', 'move', 'clone', 'construct', 'emit'];
  const requirements = [];
  for (const [language, details] of Object.entries(FOUR_LANGUAGE_DETAILS)) {
    for (const operation of operations) {
      const id = `I195-XFORM-${slug(language)}-${operation}`;
      const fixtureId = `planned:transformation:${language}:${operation}`;
      fixtureCatalog[fixtureId] = plannedFixture(
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
            extensions: EXTENSIONS[language],
          },
          expectedBehavior:
            'The public structured transformation operates after the original source buffer is discarded, updates derived metadata, emits valid source, and reparses to the intended structure.',
          requiredRuntimes: ['javascript', 'rust'],
          implementationEntryPoints: { javascript: null, rust: null },
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
    fixtureCatalog[renameFixtureId] = plannedFixture(
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
          extensions: EXTENSIONS[language],
        },
        expectedBehavior:
          'Rename follows symbol identity across scopes and modules, avoids capture, handles Unicode and generated binders, and leaves comments, strings, regexes, and unrelated names unchanged.',
        requiredRuntimes: ['javascript', 'rust'],
        implementationEntryPoints: { javascript: null, rust: null },
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

function buildTranslationRequirements(fixtureCatalog) {
  const languages = Object.keys(FOUR_LANGUAGE_DETAILS);
  const requirements = [];
  for (const sourceLanguage of languages) {
    for (const targetLanguage of languages) {
      if (sourceLanguage === targetLanguage) continue;
      const id = `I195-TRANSLATE-${slug(sourceLanguage)}-to-${slug(targetLanguage)}`;
      const fixtureId = `planned:translation:${sourceLanguage}:${targetLanguage}`;
      fixtureCatalog[fixtureId] = plannedFixture(
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
            extensions: EXTENSIONS[targetLanguage],
          },
          expectedBehavior:
            'The public translator emits a real validated target artifact and satisfies the declared observation and preservation contract without unsupported descriptors, relabelled input, erased effects, added axioms, or weakened theorems.',
          requiredRuntimes: ['javascript', 'rust'],
          implementationEntryPoints: { javascript: null, rust: null },
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

function buildCrossCuttingRequirements(fixtureCatalog) {
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
    fixtureCatalog[fixtureId] = plannedFixture(`${construct} shared corpus and expected normalization`);
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
      implementationEntryPoints: { javascript: null, rust: null },
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

function buildValidationAndDeliveryRequirements(fixtureCatalog) {
  const requirements = [];
  for (const language of Object.keys(FOUR_LANGUAGE_DETAILS)) {
    const id = `I195-NATIVE-${slug(language)}`;
    const fixtureId = `planned:native-validation:${language}`;
    fixtureCatalog[fixtureId] = plannedFixture(`${language} emitted-artifact validation corpus`);
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
          extensions: EXTENSIONS[language],
        },
        expectedBehavior:
          'The declared native compiler, host, or prover validates emitted artifacts and records reproducible versions and logs without being treated as RML proof authority.',
        requiredRuntimes: ['javascript', 'rust'],
        implementationEntryPoints: { javascript: null, rust: null },
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
    fixtureCatalog[fixtureId] = plannedFixture(`${construct} clean-consumer matrix`);
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
        implementationEntryPoints: { javascript: null, rust: null },
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
  const [inventoryText, corpusText] = await Promise.all([
    readFile(inventoryPath, 'utf8'),
    readFile(corpusPath, 'utf8'),
  ]);
  const inventory = JSON.parse(inventoryText);
  const fixtureCatalog = {};
  const requirements = [
    ...buildCstRequirements(inventory, fixtureCatalog),
    ...buildEmbeddedRequirements(inventory, fixtureCatalog),
    ...buildImporterRequirements(inventory, fixtureCatalog),
    ...buildFourLanguageCorpusRequirements(fixtureCatalog),
    ...buildSemanticRequirements(fixtureCatalog),
    ...buildTransformationRequirements(fixtureCatalog),
    ...buildTranslationRequirements(fixtureCatalog),
    ...buildCrossCuttingRequirements(fixtureCatalog),
    ...buildValidationAndDeliveryRequirements(fixtureCatalog),
    ...buildGateRequirements(fixtureCatalog),
  ];

  const digests = {
    'parity/language-grammar-inventory.json': sha256(inventoryText),
    'parity/fixtures/four-language-conformance.json': sha256(corpusText),
  };
  for (const fixture of Object.values(fixtureCatalog)) {
    if (fixture.path && !digests[fixture.path]) {
      try {
        digests[fixture.path] = sha256(await readFile(path.join(root, fixture.path), 'utf8'));
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
        'All implementation and candidate-artifact requirements must pass on the exact merge candidate.',
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

function allKeys(value, prefix = '') {
  if (!value || typeof value !== 'object') return [];
  const keys = [];
  for (const [key, child] of Object.entries(value)) {
    const full = prefix ? `${prefix}.${key}` : key;
    keys.push(full, ...allKeys(child, full));
  }
  return keys;
}

export async function validateIssue195Manifest(manifest, root) {
  const errors = [];
  if (manifest.schemaVersion !== ISSUE_195_MANIFEST_SCHEMA_VERSION) {
    errors.push(`schemaVersion must be ${ISSUE_195_MANIFEST_SCHEMA_VERSION}`);
  }
  if (manifest.issue !== 195) errors.push('issue must be 195');
  const requirements = manifest.atomicRequirements;
  if (!Array.isArray(requirements) || requirements.length === 0) {
    errors.push('atomicRequirements must be a non-empty array');
    return errors;
  }

  const ids = new Set();
  const testIds = new Set();
  for (const entry of requirements) {
    if (typeof entry.id !== 'string' || !/^I195-[A-Za-z0-9-]+$/.test(entry.id)) {
      errors.push(`invalid stable requirement id: ${JSON.stringify(entry.id)}`);
      continue;
    }
    if (ids.has(entry.id)) errors.push(`duplicate requirement id: ${entry.id}`);
    ids.add(entry.id);
    if (!Object.values(ISSUE_195_SOURCES).includes(entry.source)) {
      errors.push(`${entry.id} has a non-authoritative source permalink`);
    }
    for (const field of ['area', 'scope', 'expectedBehavior', 'requiredRuntimes', 'implementationEntryPoints', 'verifications']) {
      if (entry[field] === undefined || entry[field] === null) {
        errors.push(`${entry.id} is missing ${field}`);
      }
    }
    for (const field of ['language', 'version', 'edition', 'construct', 'aliases', 'extensions']) {
      if (entry.scope?.[field] === undefined || entry.scope?.[field] === null) {
        errors.push(`${entry.id} scope is missing ${field}`);
      }
    }
    if (allKeys(entry).some((key) => /(^|\.)(status|complete|completed)$/i.test(key))) {
      errors.push(`${entry.id} must not contain hand-edited completion/status fields`);
    }
    if (!Array.isArray(entry.requiredRuntimes) || entry.requiredRuntimes.length === 0) {
      errors.push(`${entry.id} must name required runtimes`);
    }
    if (!Array.isArray(entry.verifications) || entry.verifications.length === 0) {
      errors.push(`${entry.id} must name stable verification cells`);
      continue;
    }
    for (const runtime of entry.requiredRuntimes ?? []) {
      if (!(runtime in (entry.implementationEntryPoints ?? {}))) {
        errors.push(`${entry.id} is missing the ${runtime} implementation entry-point cell`);
      }
      if (!entry.verifications.some((cell) => cell.runtime === runtime)) {
        errors.push(`${entry.id} has no verification for required runtime ${runtime}`);
      }
    }
    for (const cell of entry.verifications) {
      if (typeof cell.testId !== 'string' || cell.testId.length === 0) {
        errors.push(`${entry.id} has a verification without a stable testId`);
        continue;
      }
      if (testIds.has(cell.testId)) errors.push(`duplicate testId: ${cell.testId}`);
      testIds.add(cell.testId);
      if (!['positive', 'negative', 'fault-injection'].includes(cell.kind)) {
        errors.push(`${cell.testId} has invalid kind ${cell.kind}`);
      }
      if (!['pre-merge', 'release-delivery'].includes(cell.checkpoint)) {
        errors.push(`${cell.testId} has invalid checkpoint ${cell.checkpoint}`);
      }
      if (!Array.isArray(cell.fixtureIds) || cell.fixtureIds.length === 0) {
        errors.push(`${cell.testId} must name fixtures`);
      }
      for (const fixtureId of cell.fixtureIds ?? []) {
        if (!manifest.fixtureCatalog?.[fixtureId]) {
          errors.push(`${cell.testId} references unknown fixture ${fixtureId}`);
        }
      }
      if (!Array.isArray(cell.assertions) || cell.assertions.length === 0) {
        errors.push(`${cell.testId} must name observable assertions`);
      }
      if (
        typeof cell.evidenceArtifact !== 'string' ||
        !/^issue-195-results\/[a-z0-9._-]+\.json(?:#[a-z0-9-]+)?$/.test(cell.evidenceArtifact)
      ) {
        errors.push(`${cell.testId} must name a stable issue-195-results JSON evidence artifact`);
      }
    }
  }

  const expected = await buildIssue195Manifest(root);
  const expectedIds = new Set(expected.atomicRequirements.map((entry) => entry.id));
  for (const id of expectedIds) {
    if (!ids.has(id)) errors.push(`manifest is missing generated scope requirement ${id}`);
  }
  for (const id of ids) {
    if (!expectedIds.has(id)) errors.push(`manifest has dangling requirement ${id}`);
  }

  for (const entry of requirements) {
    for (const runtime of entry.requiredRuntimes ?? []) {
      const paths = entry.implementationEntryPoints?.[runtime];
      if (paths === null) continue;
      if (!Array.isArray(paths) || paths.length === 0) {
        errors.push(`${entry.id} ${runtime} entry points must be a non-empty path array or null`);
        continue;
      }
      for (const relativePath of paths) {
        try {
          await access(path.join(root, relativePath));
        } catch {
          errors.push(`${entry.id} ${runtime} entry point does not exist: ${relativePath}`);
        }
      }
    }
  }

  const inventory = JSON.parse(
    await readFile(path.join(root, 'parity', 'language-grammar-inventory.json'), 'utf8'),
  );
  const languageNames = new Set(inventory.languages.map(({ name }) => name));
  const extensionNames = new Set(Object.keys(inventory.extensionDispatch ?? {}));
  for (const name of languageNames) {
    if (!extensionNames.has(name)) errors.push(`extension dispatch metadata is missing ${name}`);
  }
  for (const name of extensionNames) {
    if (!languageNames.has(name)) errors.push(`extension dispatch metadata dangles: ${name}`);
  }
  const inventoryAliasOwners = new Map();
  for (const language of inventory.languages) {
    if (!Array.isArray(language.aliases) || language.aliases.length === 0) {
      errors.push(`inventory language ${language.name} has no aliases`);
    }
    for (const alias of language.aliases ?? []) {
      const normalized = alias.toLowerCase();
      const owner = inventoryAliasOwners.get(normalized);
      if (owner && owner !== language.name) {
        errors.push(`inventory alias ${alias} is shared by ${owner} and ${language.name}`);
      }
      inventoryAliasOwners.set(normalized, language.name);
    }
  }

  const jsParserSource = await readFile(
    path.join(root, 'js', 'src', 'programming-language-parser.js'),
    'utf8',
  );
  const aliasBlock = jsParserSource.match(
    /const LANGUAGE_ALIASES = new Map\(\[([\s\S]*?)\]\);/,
  )?.[1];
  if (!aliasBlock) {
    errors.push('could not enumerate JavaScript grammar alias registry');
  } else {
    const actual = new Set(
      [...aliasBlock.matchAll(/\['([^']+)',\s*'[^']+'\]/g)].map((match) =>
        match[1].toLowerCase(),
      ),
    );
    const expectedAliases = new Set(
      inventory.languages
        .filter(({ javascript }) => javascript.status === 'grammar-cst')
        .flatMap(({ aliases }) => aliases.map((alias) => alias.toLowerCase())),
    );
    for (const alias of expectedAliases) {
      if (!actual.has(alias)) errors.push(`JavaScript grammar registry is missing alias ${alias}`);
    }
    for (const alias of actual) {
      if (!expectedAliases.has(alias)) {
        errors.push(`JavaScript grammar registry alias is absent from inventory: ${alias}`);
      }
    }
  }

  const rustParserSource = await readFile(
    path.join(root, 'rust', 'src', 'tree_sitter_adapter.rs'),
    'utf8',
  );
  const rustBuiltInParserSource = await readFile(
    path.join(root, 'rust', 'src', 'language_parser.rs'),
    'utf8',
  );
  const rustAliasBlock = rustParserSource.match(
    /fn grammar_for_language[\s\S]*?\n}\n\nfn convert_node/,
  )?.[0];
  if (!rustAliasBlock) {
    errors.push('could not enumerate Rust grammar alias registry');
  } else {
    const actual = new Set(
      [...rustAliasBlock.matchAll(/"([^"]+)"/g)].map((match) => match[1].toLowerCase()),
    );
    const builtInAliasBlock = rustBuiltInParserSource.match(
      /const BUILT_IN_GRAMMAR_ALIASES[^=]*=\s*&\[([\s\S]*?)\];/,
    )?.[1];
    if (!builtInAliasBlock) {
      errors.push('could not enumerate Rust built-in grammar alias registry');
    } else {
      for (const match of builtInAliasBlock.matchAll(/"([^"]+)"/g)) {
        actual.add(match[1].toLowerCase());
      }
    }
    const expectedAliases = new Set(
      inventory.languages
        .filter(({ rust }) => rust.status === 'grammar-cst')
        .flatMap(({ aliases }) => aliases.map((alias) => alias.toLowerCase())),
    );
    for (const alias of expectedAliases) {
      if (!actual.has(alias)) errors.push(`Rust grammar registry is missing alias ${alias}`);
    }
    for (const alias of actual) {
      if (!expectedAliases.has(alias)) {
        errors.push(`Rust grammar registry alias is absent from inventory: ${alias}`);
      }
    }
  }
  return errors;
}

function resultIndex(resultDocuments) {
  const byTestId = new Map();
  const errors = [];
  for (const document of resultDocuments) {
    if (!document || typeof document !== 'object') {
      errors.push('result document must be an object');
      continue;
    }
    if (document.schemaVersion !== 1) errors.push('result document schemaVersion must be 1');
    if (document.issue !== 195) errors.push('result document issue must be 195');
    if (!Array.isArray(document.results)) {
      errors.push('result document results must be an array');
      continue;
    }
    for (const result of document.results) {
      if (byTestId.has(result.testId)) errors.push(`duplicate result for ${result.testId}`);
      byTestId.set(result.testId, { ...result, document });
    }
  }
  return { byTestId, errors };
}

export function evaluateIssue195Acceptance(
  manifest,
  resultDocuments,
  { checkpoint = 'all', commit = 'WORKTREE' } = {},
) {
  const { byTestId, errors } = resultIndex(resultDocuments);
  const knownTestIds = new Set(
    manifest.atomicRequirements.flatMap((entry) => entry.verifications.map((cell) => cell.testId)),
  );
  for (const resultId of byTestId.keys()) {
    if (!knownTestIds.has(resultId)) errors.push(`unknown or dangling result testId ${resultId}`);
  }

  const requirementResults = [];
  for (const entry of manifest.atomicRequirements) {
    const cells = [];
    for (const cell of entry.verifications) {
      if (checkpoint !== 'all' && cell.checkpoint !== checkpoint) continue;
      const reasons = [];
      const implementation = entry.implementationEntryPoints[cell.runtime];
      if (implementation === null) reasons.push('implementation entry point is not supplied');
      for (const fixtureId of cell.fixtureIds) {
        const fixture = manifest.fixtureCatalog[fixtureId];
        if (!fixture?.path || !fixture?.sha256) {
          reasons.push(`fixture ${fixtureId} is planned but has no pinned corpus artifact`);
        }
      }
      const observed = byTestId.get(cell.testId);
      if (!observed) {
        reasons.push('required result is missing');
      } else {
        if (!observed.document.producer) reasons.push('result producer is missing');
        if (!observed.document.generatedAt) reasons.push('result generation time is missing');
        if (observed.document.commit !== commit) {
          reasons.push(`result commit ${observed.document.commit ?? '<missing>'} does not match ${commit}`);
        }
        if (observed.outcome !== 'passed') reasons.push(`outcome is ${observed.outcome ?? '<missing>'}`);
        if (observed.kind !== cell.kind) {
          reasons.push(`result kind ${observed.kind ?? '<missing>'} does not match ${cell.kind}`);
        }
        if (observed.positiveEvidence !== true) reasons.push('positiveEvidence is not true');
        if (!observed.command) reasons.push('reproducible command is missing');
        if (!observed.toolchainVersions || Object.keys(observed.toolchainVersions).length === 0) {
          reasons.push('toolchain versions are missing');
        }
        if (!observed.grammarVersions || Object.keys(observed.grammarVersions).length === 0) {
          reasons.push('grammar/component versions are missing');
        }
        if (!Array.isArray(observed.evidenceArtifacts) || observed.evidenceArtifacts.length === 0) {
          reasons.push('evidence artifacts are missing');
        }
        if (
          cell.kind === 'fault-injection' &&
          (!Array.isArray(observed.failureLogs) || observed.failureLogs.length === 0)
        ) {
          reasons.push('fault-injection failure log is missing');
        }
        const assertions = new Set(observed.assertionsPassed ?? []);
        for (const assertion of cell.assertions) {
          if (!assertions.has(assertion)) reasons.push(`observable assertion did not pass: ${assertion}`);
        }
        for (const fixtureId of cell.fixtureIds) {
          const expectedDigest = manifest.fixtureCatalog[fixtureId]?.sha256;
          if (observed.fixtureDigests?.[fixtureId] !== expectedDigest) {
            reasons.push(`fixture digest mismatch for ${fixtureId}`);
          }
        }
      }
      cells.push({
        testId: cell.testId,
        runtime: cell.runtime,
        checkpoint: cell.checkpoint,
        passed: reasons.length === 0,
        reasons,
        evidence: observed
          ? {
              producer: observed.document.producer ?? null,
              generatedAt: observed.document.generatedAt ?? null,
              outcome: observed.outcome ?? null,
              kind: observed.kind ?? null,
              positiveEvidence: observed.positiveEvidence === true,
              command: observed.command ?? null,
              toolchainVersions: observed.toolchainVersions ?? null,
              grammarVersions: observed.grammarVersions ?? null,
              evidenceArtifacts: observed.evidenceArtifacts ?? null,
              failureLogs: observed.failureLogs ?? null,
              assertionsPassed: observed.assertionsPassed ?? null,
              fixtureDigests: observed.fixtureDigests ?? null,
            }
          : null,
      });
    }
    if (cells.length > 0) {
      requirementResults.push({
        id: entry.id,
        area: entry.area,
        passed: cells.every((cell) => cell.passed),
        cells,
      });
    }
  }
  const passed = requirementResults.filter((entry) => entry.passed).length;
  return {
    schemaVersion: 1,
    issue: 195,
    commit,
    checkpoint,
    generatedAt: new Date().toISOString(),
    summary: {
      requirements: requirementResults.length,
      passed,
      failed: requirementResults.length - passed,
      verificationCells: requirementResults.reduce((sum, entry) => sum + entry.cells.length, 0),
      gateErrors: errors.length,
    },
    passed: errors.length === 0 && requirementResults.length > 0 && passed === requirementResults.length,
    gateErrors: errors,
    requirements: requirementResults,
  };
}

export function compareScopeBaseline(baseline, candidate) {
  const errors = [];
  const baselineRequirements = new Map(
    baseline.atomicRequirements.map((entry) => [entry.id, entry]),
  );
  const candidateRequirements = new Map(
    candidate.atomicRequirements.map((entry) => [entry.id, entry]),
  );
  for (const [id, oldEntry] of baselineRequirements) {
    const newEntry = candidateRequirements.get(id);
    if (!newEntry) {
      errors.push(`required scope row removed: ${id}`);
      continue;
    }
    for (const field of ['source', 'area', 'expectedBehavior']) {
      if (newEntry[field] !== oldEntry[field]) {
        errors.push(`protected requirement field changed for ${id}: ${field}`);
      }
    }
    for (const field of ['language', 'version', 'edition', 'construct']) {
      if (newEntry.scope?.[field] !== oldEntry.scope?.[field]) {
        errors.push(`protected scope field changed for ${id}: ${field}`);
      }
    }
    for (const field of ['aliases', 'extensions']) {
      const newValues = new Set(newEntry.scope?.[field] ?? []);
      for (const value of oldEntry.scope?.[field] ?? []) {
        if (!newValues.has(value)) {
          const label = field === 'aliases' ? 'alias' : 'extension';
          errors.push(`required ${label} removed from ${id}: ${value}`);
        }
      }
    }
    const newRuntimes = new Set(newEntry.requiredRuntimes ?? []);
    for (const runtime of oldEntry.requiredRuntimes ?? []) {
      if (!newRuntimes.has(runtime)) {
        errors.push(`required runtime removed from ${id}: ${runtime}`);
      }
    }
    const oldCells = new Map(oldEntry.verifications.map((cell) => [cell.testId, cell]));
    const newCells = new Map(newEntry.verifications.map((cell) => [cell.testId, cell]));
    for (const [testId, oldCell] of oldCells) {
      const newCell = newCells.get(testId);
      if (!newCell) {
        errors.push(`required verification removed: ${testId}`);
        continue;
      }
      for (const field of ['runtime', 'kind', 'checkpoint', 'evidenceArtifact']) {
        if (newCell[field] !== oldCell[field]) {
          errors.push(`protected verification field changed for ${testId}: ${field}`);
        }
      }
      for (const assertion of oldCell.assertions) {
        if (!newCell.assertions.includes(assertion)) {
          errors.push(`required assertion removed from ${testId}: ${assertion}`);
        }
      }
      for (const fixtureId of oldCell.fixtureIds) {
        if (!newCell.fixtureIds.includes(fixtureId)) {
          errors.push(`required fixture removed from ${testId}: ${fixtureId}`);
        }
      }
    }
  }
  return errors;
}

function syntheticFaultCase(assertions) {
  const manifest = {
    fixtureCatalog: {
      fixture: { path: 'fixture.json', selector: '$', sha256: 'fixture-digest' },
    },
    atomicRequirements: [
      {
        id: 'I195-FAULT-PROBE',
        area: 'fault-probe',
        scope: { aliases: ['required-alias'], extensions: ['.required'] },
        requiredRuntimes: ['aggregate'],
        implementationEntryPoints: { aggregate: ['gate.js'] },
        verifications: [
          {
            testId: 'i195-fault-probe',
            runtime: 'aggregate',
            kind: 'positive',
            checkpoint: 'pre-merge',
            fixtureIds: ['fixture'],
            assertions,
          },
        ],
      },
    ],
  };
  const document = {
    schemaVersion: 1,
    issue: 195,
    commit: 'fault-candidate',
    producer: 'issue-195-fault-probe',
    generatedAt: new Date(0).toISOString(),
    results: [
      {
        testId: 'i195-fault-probe',
        outcome: 'passed',
        kind: 'positive',
        positiveEvidence: true,
        command: 'fault probe',
        toolchainVersions: { node: process.version },
        grammarVersions: { acceptanceManifest: 'schema-1' },
        evidenceArtifacts: ['fault-result.json'],
        failureLogs: [],
        assertionsPassed: assertions,
        fixtureDigests: { fixture: 'fixture-digest' },
      },
    ],
  };
  return { manifest, document };
}

function faultIsRejected(fault) {
  if (fault === 'removed-inventory-row-or-alias') {
    const { manifest } = syntheticFaultCase(['scopePreserved']);
    const candidate = structuredClone(manifest);
    candidate.atomicRequirements[0].scope.aliases = [];
    const failureLogs = compareScopeBaseline(manifest, candidate);
    return { detected: failureLogs.length > 0, failureLogs };
  }

  const profile =
    fault === 'stubbed-binding-resolution' || fault === 'rename-capture-bug'
      ? ASSERTION_PROFILES.bindingRename
      : fault === 'stale-structure-after-edit'
        ? ASSERTION_PROFILES.transformPositive
        : fault === 'unsupported-translation-descriptor' ||
            fault === 'relabeled-source-translation'
          ? ASSERTION_PROFILES.translationPositive
          : ASSERTION_PROFILES.cstPositive;
  const { manifest, document } = syntheticFaultCase(profile);
  if (fault === 'plain-text-parser-fallback') {
    document.results[0].assertionsPassed = profile.filter(
      (assertion) => assertion !== 'realGrammarNodes',
    );
  } else if (fault === 'lexical-parser-fallback') {
    document.results[0].assertionsPassed = profile.filter(
      (assertion) => assertion !== 'hierarchy',
    );
  } else if (fault === 'dropped-fields-trivia-or-spans') {
    document.results[0].assertionsPassed = profile.filter(
      (assertion) => !['namedFields', 'commentsAndTrivia', 'exactUtf8Spans'].includes(assertion),
    );
  } else if (fault === 'stubbed-binding-resolution') {
    document.results[0].assertionsPassed = profile.filter(
      (assertion) => assertion !== 'symbolIdentity',
    );
  } else if (fault === 'rename-capture-bug') {
    document.results[0].assertionsPassed = profile.filter(
      (assertion) => assertion !== 'captureAvoidance',
    );
  } else if (fault === 'stale-structure-after-edit') {
    document.results[0].assertionsPassed = profile.filter(
      (assertion) => assertion !== 'treeIntegrity',
    );
  } else if (fault === 'unsupported-translation-descriptor') {
    document.results[0].outcome = 'unsupported';
  } else if (fault === 'relabeled-source-translation') {
    document.results[0].assertionsPassed = profile.filter(
      (assertion) => assertion !== 'noSourceRelabelling',
    );
  } else if (fault === 'skipped-test-or-job') {
    document.results = [];
  } else if (fault === 'falsified-capability-declaration') {
    manifest.atomicRequirements[0].status = 'complete';
    document.results = [];
  } else {
    throw new Error(`unknown issue 195 fault injection: ${fault}`);
  }
  const report = evaluateIssue195Acceptance(manifest, [document], {
    checkpoint: 'pre-merge',
    commit: 'fault-candidate',
  });
  const failureLogs = [
    ...report.gateErrors,
    ...report.requirements.flatMap(({ cells }) =>
      cells.flatMap(({ testId, reasons }) => reasons.map((reason) => `${testId}: ${reason}`)),
    ),
  ];
  return { detected: !report.passed, failureLogs };
}

export function runIssue195GateFaultInjections(manifest, commit) {
  const results = manifest.atomicRequirements
    .filter(({ area }) => area === 'acceptance-gate-fault-injection')
    .map((entry) => {
      const fault = entry.scope.construct;
      const cell = entry.verifications[0];
      const { detected, failureLogs } = faultIsRejected(fault);
      return {
        testId: cell.testId,
        outcome: detected ? 'passed' : 'failed',
        kind: cell.kind,
        positiveEvidence: detected,
        command:
          'node js/scripts/check-issue-195-acceptance.mjs --produce-gate-results issue-195-results/gate-faults.json',
        toolchainVersions: { node: process.version },
        grammarVersions: { acceptanceManifest: `schema-${manifest.schemaVersion}` },
        evidenceArtifacts: [
          'js/tests/issue-195-acceptance.test.js',
          'js/scripts/issue-195-acceptance-lib.mjs',
        ],
        failureLogs,
        assertionsPassed: detected ? cell.assertions : [],
        fixtureDigests: Object.fromEntries(
          cell.fixtureIds.map((fixtureId) => [fixtureId, manifest.fixtureCatalog[fixtureId].sha256]),
        ),
      };
    });
  return {
    schemaVersion: 1,
    issue: 195,
    commit,
    producer: 'issue-195-acceptance-gate-fault-injections',
    generatedAt: new Date().toISOString(),
    results,
  };
}

export function renderIssue195Markdown(manifest, report) {
  const lines = [
    '<!-- Generated by js/scripts/check-issue-195-acceptance.mjs; do not edit by hand. -->',
    '# Issue 195 executable requirement ledger',
    '',
    `Commit: \`${report.commit}\``,
    '',
    `Checkpoint: \`${report.checkpoint}\``,
    '',
    `Generated: ${report.generatedAt}`,
    '',
    `Acceptance result: **${report.passed ? 'PASS' : 'FAIL'}** — ${report.summary.passed}/${report.summary.requirements} atomic requirements passed across ${report.summary.verificationCells} verification cells.`,
    '',
    'This report is computed from the atomic manifest and commit-bound test-result artifacts. Parser/capability declarations, unsupported descriptors, planned fixtures, skipped tests, and hand-edited labels are not execution evidence.',
    '',
    '| Requirement | Area | Result | Failed verification |',
    '| --- | --- | --- | --- |',
  ];
  for (const result of report.requirements) {
    const failedCells = result.cells.filter((cell) => !cell.passed);
    const reason = failedCells.length
      ? failedCells
          .map((cell) => `\`${cell.testId}\`: ${cell.reasons.join('; ')}`)
          .join('<br>')
      : 'All required evidence passed.';
    lines.push(`| \`${result.id}\` | ${result.area} | ${result.passed ? 'PASS' : 'FAIL'} | ${reason} |`);
  }
  lines.push(
    '',
    '## Authoritative sources',
    '',
    `- [Issue #195](${manifest.sources.issue})`,
    `- [Full-support clarification](${manifest.sources.clarification})`,
    `- [Executable acceptance-gate clarification](${manifest.sources.acceptanceGate})`,
    '',
    '## Enforcement status',
    '',
    'The dedicated workflow executes this evaluator and fails closed. Repository branch protection is a separate maintainer-controlled setting; the workflow does not claim to be a required check until protection names it.',
  );
  return lines.join('\n');
}
