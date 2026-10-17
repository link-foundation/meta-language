// Authored finite inputs and independent expectations only. This fixture does
// not execute a parser, source-mapping decision or CST validator.
export function grammarSelectionMappingObservationSections() {
  return Object.fromEntries([
    ['selection', 'I195-CST-ANY-GRAMMAR-RECURSIVE-RULE', [
      'jsonStringAnyAllCandidateWitnessesPreserved',
      'jsonStringUnsupportedAndNonScalarProfilesRefused',
      'mappedProfileMemoKeysKeepExactGrammarNames',
      'jsonStringLexicalAndMalformedInputsRefused',
      'invalidUtf8RootCannotClaimUtf8SourcePlane',
    ]],
    ['boundaries', 'I195-CST-RECURSIVE-BOUNDARIES-LOSSLESS', [
      'shippedJsonPythonConcreteTreesRemainExact',
      'jsonStringUtf8IntervalsAndQuoteOwnershipExact',
      'mappedSourceRegistryAndGrammarReplayTextAndBinary',
      'serializableMappingForgeriesRefusedAfterReload',
    ]],
    ['recursion', 'I195-CST-DEEPLY-RECURSIVE-MIXED-LANGUAGES', [
      'nestedMappedSourcesRetainActualDerivationOrder',
      'mappedParsersRetainExistingLimitRefusals',
      'mappedCycleAndSyntaxFailuresRetainSourceRoutes',
      'failedMappedParseCannotAuthenticateMissingRoot',
    ]],
  ].map(([section, requirementId, assertions]) => [section, {
    requirementId, suffix: 'json-string-profile', assertions,
    fixtureIds: Object.fromEntries(['javascript', 'rust'].map(runtime =>
      [runtime, `grammar-selection-json-string-${section}-${runtime}`])),
  }]));
}

export function grammarSelectionMappingFixture({ captures, python }) {
  return {
    schemaVersion: 1, id: 'grammar-selection-json-string',
    boundary: 'Only the listed Unicode-scalar json-string inputs and source-qualified tests. Physical and decoded CSTs, exact UTF-8 intervals, two nested derivations, any/all witnesses, original refusal tables and existing limits are finite controls. The runtime mutation tables differ and are preserved separately. Local source-plane references are not Links identifiers, global concept equivalence, arbitrary mapping support or full language coverage. No observation is stored in this fixture.',
    captures, inputs: {
      python,
      escape: {
        raw: '"a\\n\\u03C0\\uD83D\\uDE00"', decoded: 'a\nπ😀',
        physicalBytes: [34, 97, 92, 110, 92, 117, 48, 51, 67, 48, 92, 117, 68, 56, 51, 68, 92, 117, 68, 69, 48, 48, 34],
        decodedBytes: [97, 10, 207, 128, 240, 159, 152, 128], prefixes: ['', 'λ<'],
        intervals: [[0, 1, 0, 0], [1, 2, 0, 1], [2, 4, 1, 2],
          [4, 10, 2, 4], [10, 22, 4, 8], [22, 23, 8, 8]],
      },
      nesting: { smallDecoded: 'x', oneLevel: '"x"', twoLevels: '"\\"x\\""' },
      selection: { modes: ['any-grammar', 'all-grammar'],
        siblings: { firstProgram: 'x=1\n', secondProgram: 'x=2\n' } },
      refusals: {
        invalidBytes: [255, 34, 120, 34],
        loneSurrogateEscapes: ['"\\uD800"', '"\\uDC00"'],
        invalidEscapes: ['"\\q"', '"\\u{41}"', '"\\u12"', '"line\nbreak"'],
        unsupportedMapping: 'quoted-escape', mixedMappings: ['json-string', 'identity'],
        limitOptions: [
          { options: { stepLimit: 0 }, reason: 'stepLimit' },
          { options: { memoryLimit: 0 }, reason: 'memoryBudget' },
          { options: { maxDepth: 0 }, reason: 'nestingDepth' },
        ],
      },
      memo: { decoded: 'x', identityName: 'json-string|decoded', warmMappings: ['identity', 'json-string'] },
    },
    runtimeCases: {
      javascript: { serializableForgeries: [
        'changed physical container start', 'changed physical container end',
        'changed decoded container start', 'changed decoded container end',
        'changed mapped embed start', 'changed mapped embed end',
        'absent registry', 'empty registry', 'null root', 'unknown source kind',
        'foreign encoding', 'duplicate source identity', 'fractional source identity',
        'changed root length', 'changed derived length', 'reordered definitions',
        'missing derivation', 'forward derivation', 'unknown derivation', 'different occurrence',
        'unknown definition field', 'unknown derivation field', 'unknown mapping profile',
        'missing physical CST', 'wrong input plane', 'foreign output plane',
        'unknown mapping operation', 'unknown mapping field', 'interval gap', 'interval overlap',
        'interval reversal', 'missing closing quote', 'fractional interval', 'negative interval',
        'fifth interval column', 'surrogate width', 'edited physical bytes', 'edited decoded bytes',
      ], invalidUtf8Prefix: true },
      rust: { serializableForgeries: {
        replacements: [
          { path: '/tree/start', value: 1 }, { path: '/tree/end', value: 22 },
          { path: '/tree/children/0/start', value: 1 }, { path: '/tree/children/0/end', value: 22 },
          { path: '/tree/children/0/root/start', value: 1 }, { path: '/tree/children/0/root/end', value: 7 },
          { path: '/sourcePlanes', value: null },
          { path: '/sourcePlanes/0/kind', value: 'unowned-source' },
          { path: '/sourcePlanes/0/encoding', value: 'UTF-16' },
          { path: '/sourcePlanes/0/identity', value: 1 },
          { path: '/sourcePlanes/0/byteLength', value: 24 },
          { path: '/sourcePlanes/1/identity', value: 0 },
          { path: '/sourcePlanes/1/identity', value: 0.5 },
          { path: '/sourcePlanes/1/byteLength', value: 9 },
          { path: '/sourcePlanes/1/derivation', value: null },
          { path: '/sourcePlanes/1/derivation/input', value: 1 },
          { path: '/sourcePlanes/1/derivation/operation', value: 'identity' },
          { path: '/sourcePlanes/1/derivation/start', value: 1 },
          { path: '/tree/children/0/selection/mapping', value: 'guessed-string' },
          { path: '/tree/children/0/sourceChildren', value: null },
          { path: '/tree/children/0/selection/sourceMapping/input', value: 1 },
          { path: '/tree/children/0/selection/sourceMapping/output', value: 2 },
          { path: '/tree/children/0/selection/sourceMapping/operation', value: 'identity' },
          { path: '/tree/children/0/selection/sourceMapping/intervals/1/0', value: 2 },
          { path: '/tree/children/0/selection/sourceMapping/intervals/1/1', value: 3 },
          { path: '/tree/children/0/selection/sourceMapping/intervals/1/2', value: 0.5 },
          { path: '/tree/children/0/selection/sourceMapping/intervals/0/0', value: -1 },
          { path: '/tree/children/0/selection/sourceMapping/intervals/4/3', value: 7 },
          { path: '/tree/children/0/root/children/0/text', value: 'b\nπ😀' },
        ],
        extraFields: ['/sourcePlanes/0', '/sourcePlanes/1/derivation',
          '/tree/children/0/selection/sourceMapping'], missingRegistry: true,
      } },
    },
    observations: grammarSelectionMappingObservationSections(),
  };
}
