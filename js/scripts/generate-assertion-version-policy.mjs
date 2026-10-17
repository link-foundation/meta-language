import { fileURLToPath } from 'node:url';
import { checkProjectData, writeProjectData } from './project-data.mjs';

export function assertionVersionPolicy() {
  return {
    schemaVersion: 1,
    assertions: [{
      id: 'unicode-identifier-property-generation',
      subject: 'Unicode',
      comparison: { kind: 'release', precision: 2 },
      expectations: [{
        id: 'unicode-17-identifier-properties',
        minimum: '17.0', maximum: '17.1', includeMinimum: true, includeMaximum: false,
        expected: '17.0',
        evidence: 'bbb13433fbdd3db3df8a5831eed8b9a6e5b83be1:js/scripts/build-unicode-identifier-properties.mjs requires process.versions.unicode === 17.0; original source sha256 58bb5fe590aed45a4b6d5e68e34e1120d8d7b7fcb7b74907e09f9d3ebde56d78; existing js/src/data/unicode-identifier-properties.lino records Unicode 17.0',
      }],
    }, {
      id: 'antlr-lexer-reference-semantics',
      subject: 'ANTLR reference semantics',
      comparison: { kind: 'release', precision: 3 },
      expectations: [{
        id: 'antlr-4-13-2-lexical-semantics',
        minimum: '4.13.2', maximum: '4.13.2', includeMinimum: true, includeMaximum: true,
        expected: 'ANTLR 4.13.2',
        sourceReferences: {
          runtime: 'https://raw.githubusercontent.com/antlr/antlr4/4.13.2/runtime/Java/src/org/antlr/v4/runtime/Lexer.java',
          actions: 'https://raw.githubusercontent.com/antlr/antlr4/4.13.2/tool/src/org/antlr/v4/automata/LexerATNFactory.java',
          diagnostics: 'https://raw.githubusercontent.com/antlr/antlr4/4.13.2/tool/src/org/antlr/v4/tool/ErrorType.java',
        },
        evidence: 'The shared grammar-lexer-rules.lino upstreamSemantics names ANTLR 4.13.2 Lexer.java, LexerATNFactory.java and ErrorType.java; existing ordered lexer command and inherited parser option assertions retain these exact reference semantics.',
      }],
    }, {
      id: 'peggy-optimized-character-set-shape',
      subject: 'Peggy optimizer',
      comparison: { kind: 'release', precision: 3 },
      expectations: [{
        id: 'peggy-5-1-0-character-set-shape',
        minimum: '5.1.0', maximum: '5.1.0', includeMinimum: true, includeMaximum: true,
        expected: 'Peggy 5.1.0',
        evidence: 'js/package.json and js/package-lock.json pin Peggy 5.1.0. Its actual generate(..., { output: ast }) optimizer merges the retained BMP, ASCII and astral literal alternatives into class nodes; js/tests/grammar-interchange-tools.test.js executes the optimizer and retains every source lexer, offset and tampering assertion.',
      }],
    }, {
      id: 'typescript-const-model-rejection-diagnostic',
      subject: 'TypeScript public declaration compiler',
      comparison: { kind: 'release', precision: 3 },
      expectations: [{
        id: 'typescript-5-9-2-const-model-rejection',
        minimum: '5.9.2', maximum: '5.9.2', includeMinimum: true, includeMaximum: true,
        expected: ['TS2820'],
        evidence: 'The existing TypeScript 5.9.2 compiler produced TS2820 for the unchanged closed pure Rust const predicates 99 consumer after adding the Boolean const model to the public union. The actual diagnostic is retained in /private/tmp/meta-language-bulk51-typescript-contract-probe.log (sha256 5ac1b0263186ac10bf1e0befcd32c40112f9bb8b3f2b3db120a8f1370fabb28c); this singleton records that local observation only.',
      }, {
        id: 'typescript-7-0-2-const-model-rejection',
        minimum: '7.0.2', maximum: '7.0.2', includeMinimum: true, includeMaximum: true,
        expected: ['TS2820'],
        evidence: 'js/package.json and js/package-lock.json pin TypeScript 7.0.2. This singleton specifies the required CI diagnostic for the same unchanged closed pure Rust const predicates 99 consumer; it is not a local TypeScript 7 execution observation.',
      }],
    }],
    observationBindings: [{
      testId: 'i195-assertion-explicit-version-ranges-tooling-antlr-command-order',
      assertionId: 'antlrCommandOrderHasApplicableVersionContext',
      fixtureId: 'assertion-version-antlr-command-order-source',
      fixtureFile: 'js/tests/grammar-antlr-lexer-election.test.js',
      assertionPolicy: 'antlr-lexer-reference-semantics',
    }, {
      testId: 'i195-assertion-explicit-version-ranges-tooling-antlr-inherited-options',
      assertionId: 'antlrInheritedOptionsHaveApplicableVersionContext',
      fixtureId: 'assertion-version-antlr-inherited-options-source',
      fixtureFile: 'js/tests/grammar-antlr.test.js',
      assertionPolicy: 'antlr-lexer-reference-semantics',
    }, {
      testId: 'i195-assertion-explicit-version-ranges-tooling-peggy-character-set-shape',
      assertionId: 'peggyOptimizedCharacterSetsHaveApplicableVersionContext',
      fixtureId: 'assertion-version-peggy-character-set-shape-source',
      fixtureFile: 'js/tests/grammar-interchange-tools.test.js',
      assertionPolicy: 'peggy-optimized-character-set-shape',
    }, {
      testId: 'i195-assertion-explicit-version-ranges-tooling-peggy-character-set-shape',
      assertionId: 'peggyCharacterSetRefusalsHaveApplicableVersionContext',
      fixtureId: 'assertion-version-peggy-character-set-shape-source',
      fixtureFile: 'js/tests/grammar-interchange-tools.test.js',
      assertionPolicy: 'peggy-optimized-character-set-shape',
    }],
  };
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  const file = new URL('../../parity/assertion-version-ranges.lino', import.meta.url);
  if (process.argv.includes('--check')) checkProjectData(file, assertionVersionPolicy());
  else writeProjectData(file, assertionVersionPolicy());
  console.log('Assertion version ranges: explicit active-version expectations recorded as Links');
}
