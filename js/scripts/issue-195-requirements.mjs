import { createHash } from 'node:crypto';
import { readFile } from 'node:fs/promises';
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
  Lean: { version: '4.33.1', edition: 'Lean 4' },
  Rocq: { version: '9.2', edition: 'Vernacular' },
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
      fixtureCatalog[fixtureId] = pinnedFixture(
        'parity/fixtures/issue-195-evidence.json',
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
            extensions: EXTENSIONS[language],
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
            extensions: EXTENSIONS[language],
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

function buildTransformationRequirements(fixtureCatalog) {
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
            extensions: EXTENSIONS[language],
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
          extensions: EXTENSIONS[language],
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

function buildTranslationRequirements(fixtureCatalog) {
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
            extensions: EXTENSIONS[targetLanguage],
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

function buildValidationAndDeliveryRequirements(fixtureCatalog) {
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
          extensions: EXTENSIONS[language],
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
