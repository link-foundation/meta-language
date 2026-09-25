import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { appendFile } from 'node:fs/promises';
import { test } from 'node:test';
import { runInNewContext } from 'node:vm';

import {
  LANGUAGE_REPRESENTATION_SCHEMA_VERSION,
  LinkNetwork,
  LinkQuery,
  LinkType,
  ParseConfiguration,
  ParserRegistry,
  ProgramRepresentation,
  ReplacementRule,
  TranslationSupport,
  analyzeProgram,
  constructProgram,
  constructProgramFromFragments,
  decodeProgramTranslation,
  fourLanguageSupport,
  languageSupport,
  translationContracts,
  translateProgram,
} from '../src/index.js';

const corpus = JSON.parse(
  readFileSync(new URL('../../parity/fixtures/four-language-conformance.json', import.meta.url)),
);
const grammarInventoryBytes = readFileSync(
  new URL('../../parity/language-grammar-inventory.json', import.meta.url),
);
const grammarInventory = JSON.parse(grammarInventoryBytes);
const grammarInventoryDigest = createHash('sha256').update(grammarInventoryBytes).digest('hex');

async function recordNegativeCstObservation(language, assertionId) {
  if (!process.env.ISSUE_195_OBSERVATION_FILE) return;
  const slug = language.normalize('NFKD').toLowerCase()
    .replaceAll('+', '-plus').replaceAll('#', '-sharp')
    .replace(/[^a-z0-9]+/gu, '-').replace(/^-|-$/gu, '');
  await appendFile(process.env.ISSUE_195_OBSERVATION_FILE, `${JSON.stringify({
    testId: `i195-cst-${slug}-javascript-negative`,
    assertionId,
    fixtureId: `planned:cst-negative:${language}`,
    fixtureDigest: grammarInventoryDigest,
    runtime: 'javascript',
    commit: process.env.ISSUE_195_COMMIT,
    outcome: 'passed',
    testName: 'every JavaScript grammar inventory frontend retains and diagnoses prohibited NUL input',
  })}\n`);
}

test('four-language corpus produces lossless structured syntax', () => {
  for (const fixture of corpus.languages) {
    const network = LinkNetwork.parse(
      fixture.source,
      fixture.name,
      ParseConfiguration.default(),
    );

    assert.equal(network.reconstructText(), fixture.source, `${fixture.name} reconstruction`);
    assert.equal(network.verifyFullMatch().isClean(), true, `${fixture.name} diagnostics`);
    assert.ok(
      network.links().some(
        (link) =>
          link.metadata().linkType === LinkType.Syntax &&
          link.metadata().term === fixture.root &&
          link.metadata().span !== undefined,
      ),
      `${fixture.name} root syntax`,
    );

    const identifiers = network
      .find(LinkQuery.fromSexpression('(identifier) @identifier'))
      .map((match) => network.capturedText(match.captures.get('identifier')));
    assert.deepEqual(identifiers, fixture.identifiers, `${fixture.name} identifiers`);
  }
});

test('all four-language aliases select the same structured frontend', () => {
  for (const fixture of corpus.languages) {
    for (const alias of fixture.aliases) {
      const network = LinkNetwork.parse(fixture.source, alias, ParseConfiguration.default());
      assert.equal(network.reconstructText(), fixture.source, `${alias} reconstruction`);
      assert.ok(
        network.links().some(
          (link) =>
            link.metadata().linkType === LinkType.Syntax &&
            link.metadata().term === fixture.root,
        ),
        `${alias} structured root`,
      );
    }
  }
});

test('formal-language comments and invalid input stay lossless and diagnostic', () => {
  for (const fixture of corpus.negativeCases) {
    const network = LinkNetwork.parse(
      fixture.source,
      fixture.language,
      ParseConfiguration.default(),
    );
    assert.equal(network.reconstructText(), fixture.source, fixture.diagnostic);
    assert.equal(network.verifyFullMatch().isClean(), false, fixture.diagnostic);
  }
});

test('structured edits emit from retained tokens without touching comments or strings', () => {
  for (const fixture of corpus.languages) {
    const network = LinkNetwork.parse(fixture.source, fixture.name, ParseConfiguration.default());
    const query = LinkQuery.fromSexpression(
      `(identifier) @target\n(#eq? @target "${fixture.edit.identifier}")`,
    );
    network.replace(
      network.find(query),
      ReplacementRule.capturedText('target', fixture.edit.replacement),
    );
    assert.equal(network.reconstructText(), fixture.edit.expected, fixture.name);
  }
});

test('JavaScript grammar distinguishes regex text and template interpolation', () => {
  for (const fixture of [
    corpus.javascriptRegressions.regularExpression,
    corpus.javascriptRegressions.templateInterpolation,
  ]) {
    const network = LinkNetwork.parse(fixture.source, 'JavaScript');
    const query = LinkQuery.fromSexpression(
      `(identifier) @target\n(#eq? @target "${fixture.identifier}")`,
    );
    const matches = network.find(query);
    assert.equal(matches.length, fixture.matches);
    network.replace(matches, ReplacementRule.capturedText('target', fixture.replacement));
    assert.equal(network.reconstructText(), fixture.expected);
  }
});

test('JavaScript grammar reports syntactically invalid programs', () => {
  const fixture = corpus.javascriptRegressions.invalidProgram;
  const network = LinkNetwork.parse(fixture.source, 'JavaScript');
  assert.equal(network.reconstructText(), fixture.source);
  assert.equal(network.verifyFullMatch().isClean(), false, fixture.diagnostic);
});

test('JavaScript grammar retains zero-width missing nodes at source boundaries', () => {
  const source = 'if (true)';
  const network = LinkNetwork.parse(source, 'JavaScript');

  assert.equal(network.reconstructText(), source);
  assert.equal(network.verifyFullMatch().isClean(), false);
  assert.ok(network.links().some((link) => link.metadata().flags.isMissing));
});

test('ordinary parse dispatch returns grammar CSTs for the audited language inventory', () => {
  for (const fixture of corpus.defaultCstCases) {
    const network = LinkNetwork.parse(fixture.source, fixture.language);
    assert.equal(network.reconstructText(), fixture.source, fixture.language);
    for (const term of [fixture.root, fixture.requiredNode]) {
      assert.ok(
        network.links().some(
          (link) => link.metadata().linkType === LinkType.Syntax && link.metadata().term === term,
        ),
        `${fixture.language} ${term}`,
      );
    }
  }
});

test('every JavaScript grammar inventory alias selects a nontrivial lossless CST', () => {
  for (const fixture of grammarInventory.languages.filter(
    ({ javascript }) => javascript.status === 'grammar-cst',
  )) {
    for (const alias of fixture.aliases) {
      const network = LinkNetwork.parse(fixture.source, alias);
      assert.equal(network.reconstructText(), fixture.source, `${alias} reconstruction`);
      const syntax = network.links().filter(
        (link) => link.metadata().linkType === LinkType.Syntax,
      );
      assert.ok(syntax.length > 1, `${alias} must expose grammar nodes below its root`);
      assert.ok(
        syntax.some((link) => link.metadata().span !== undefined),
        `${alias} grammar nodes must retain spans`,
      );
    }
  }
});

test('every JavaScript grammar inventory frontend retains and diagnoses prohibited NUL input', async () => {
  for (const fixture of grammarInventory.languages) {
    const source = `${fixture.source}\0`;
    const network = LinkNetwork.parse(source, fixture.name);
    assert.equal(network.reconstructText(), source, `${fixture.name} malformed reconstruction`);
    await recordNegativeCstObservation(fixture.name, 'malformedInputRetained');
    await recordNegativeCstObservation(fixture.name, 'exactReconstruction');
    const verification = network.verifyFullMatch();
    assert.ok(verification.issues.length > 0, `${fixture.name} malformed diagnostic`);
    await recordNegativeCstObservation(fixture.name, 'diagnosticReported');
    assert.equal(verification.isClean(), false, `${fixture.name} malformed not clean`);
    await recordNegativeCstObservation(fixture.name, 'notReportedAsClean');
  }
});

test('JavaScript document and natural-language grammars expose their productions', () => {
  for (const [language, source, terms] of [
    ['LiNo', '1 1 1\n', ['lino_document', 'link']],
    ['txt', 'Plain text.\n', ['text_document', 'line']],
    ['PDF', '%PDF-1.7\n%%EOF\n', ['pdf_file', 'header', 'end_of_file']],
    ['DOCX', '<w:document><w:body/></w:document>\n', ['document', 'element']],
    ['English', 'Hawaii is a state.\n', ['natural_language_document', 'sentence', 'word']],
    ['Mandarin Chinese', '你好。\n', ['natural_language_document', 'sentence', 'word']],
  ]) {
    const network = LinkNetwork.parse(source, language);
    assert.equal(network.reconstructText(), source, language);
    assert.equal(network.verifyFullMatch().isClean(), true, language);
    for (const term of terms) {
      assert.ok(
        network.links().some(
          (link) => link.metadata().linkType === LinkType.Syntax && link.metadata().term === term,
        ),
        `${language} must expose ${term}`,
      );
    }
  }

  // `not a link` is a valid three-reference LiNo link; an unclosed paren is not.
  for (const [language, source] of [
    ['LiNo', '(unclosed\n'],
    ['PDF', 'not a PDF\n'],
  ]) {
    const network = LinkNetwork.parse(source, language);
    assert.equal(network.reconstructText(), source, `${language} recovery`);
    assert.equal(network.verifyFullMatch().isClean(), false, `${language} diagnostics`);
  }
});

test('capability reports match the shared versioned corpus', () => {
  assert.equal(LANGUAGE_REPRESENTATION_SCHEMA_VERSION, corpus.schemaVersion);
  assert.equal(fourLanguageSupport().length, 4);
  assert.equal(Object.isFrozen(fourLanguageSupport()), true);
  for (const fixture of corpus.languages) {
    const support = languageSupport(fixture.name);
    assert.equal(support.version, fixture.version);
    assert.equal(support.edition, fixture.edition);
    assert.deepEqual(support.extensions, fixture.extensions);
    assert.equal(support.bindingResolution, fixture.capabilities.bindingResolution);
    assert.equal(support.typeElaboration, fixture.capabilities.typeElaboration);
    assert.equal(support.dynamicExtensions, fixture.capabilities.dynamicExtensions);
    assert.equal(support.proofSyntax, fixture.capabilities.proofSyntax);
  }
});

test('project-aware analysis distinguishes missing context from recognized toolchain modules', () => {
  for (const fixture of corpus.semanticPrograms) {
    const withoutContext = analyzeProgram(fixture.source, fixture.language);
    assert.ok(
      withoutContext.diagnostics.some(({ kind }) => kind === 'missing-project-context'),
      `${fixture.language} missing project context`,
    );

    const withContext = analyzeProgram(fixture.source, fixture.language, fixture.project);
    assert.equal(
      withContext.diagnostics.some(({ kind }) => kind === 'missing-project-context'),
      false,
      `${fixture.language} supplied project context`,
    );
    assert.ok(
      withContext.modules.some(({ kind }) => kind === 'recognized-toolchain-module'),
      `${fixture.language} recognized toolchain module`,
    );
  }
});

test('an unrelated declared dependency does not resolve a project import', () => {
  for (const fixture of corpus.semanticPrograms) {
    const project = { ...fixture.project, dependencies: ['unrelated-package'] };
    const program = analyzeProgram(fixture.source, fixture.language, project);
    assert.ok(
      program.diagnostics.some(({ kind }) => kind === 'missing-project-context'),
      `${fixture.language} import remains unresolved`,
    );
  }
});

test('a matching but nonexistent dependency or file cannot resolve an import', () => {
  for (const fixture of corpus.phantomImportCases) {
    const project = {
      root: '/workspace', files: [fixture.file], dependencies: [fixture.dependency],
    };
    const program = analyzeProgram(fixture.source, fixture.language, project);
    assert.ok(program.modules.some(({ kind, name }) =>
      kind === 'module-import' && name === fixture.dependency), fixture.language);
    assert.ok(program.diagnostics.some(({ kind }) =>
      kind === 'missing-project-context'), fixture.language);
    assert.equal(program.modules.some(({ kind }) => kind === 'recognized-toolchain-module'), false);
  }
});

test('four-language semantic programs expose every required representation phase', () => {
  for (const fixture of corpus.semanticPrograms) {
    const program = analyzeProgram(fixture.source, fixture.language, fixture.project);
    assert.equal(program.emit(), fixture.source, `${fixture.language} source generation`);
    assert.equal(program.network.reconstructText(), fixture.source, `${fixture.language} network`);
    assert.equal(program.diagnostics.length, 0, `${fixture.language} diagnostics`);
    assert.ok(program.bindings.length > 0, `${fixture.language} bindings`);
    assert.ok(program.scopes.length > 0, `${fixture.language} scopes`);
    assert.ok(program.sourceMappings.length > 0, `${fixture.language} source mappings`);
    assert.ok(program.types.every(({ phase }) => phase !== 'resolved' && phase !== 'elaborated'),
      `${fixture.language} has only surface type facts`);

    const byKind = new Map(program.constructs.map((construct) => [construct.kind, construct]));
    for (const kind of fixture.represented) {
      assert.equal(byKind.get(kind)?.status, 'represented', `${fixture.language} ${kind}`);
      assert.ok(byKind.get(kind).evidence.length > 0, `${fixture.language} ${kind} evidence`);
    }
    for (const kind of fixture.notApplicable) {
      assert.equal(byKind.get(kind)?.status, 'not-applicable', `${fixture.language} ${kind}`);
      assert.ok(byKind.get(kind).rationale, `${fixture.language} ${kind} rationale`);
    }
    for (const kind of fixture.unavailable) {
      assert.equal(byKind.get(kind)?.status, 'unavailable', `${fixture.language} ${kind}`);
      assert.deepEqual(byKind.get(kind).evidence, [], `${fixture.language} ${kind} has no fabricated trace`);
    }
  }
});

test('binding-aware rename preserves shadowing, Unicode, templates, and comments', () => {
  for (const fixture of corpus.renameCases) {
    const program = analyzeProgram(fixture.source, fixture.language);
    const candidates = program.bindings.filter(({ name }) => name === fixture.binding);
    const binding = candidates[fixture.declarationOccurrence];
    assert.ok(binding, `${fixture.language} selected binding`);
    const renamed = program.renameBinding(binding.id, fixture.replacement);
    assert.equal(renamed.emit(), fixture.expected, `${fixture.language} binding rename`);
    assert.equal(renamed.network.verifyFullMatch().isClean(), true, `${fixture.language} reparse`);
    if (fixture.expectedObservation !== undefined) {
      for (const source of [fixture.source, renamed.emit()]) {
        const context = {};
        runInNewContext(source, context);
        assert.deepEqual(
          JSON.parse(JSON.stringify(context.result)),
          fixture.expectedObservation,
          `${fixture.language} binding rename preserves the independent observation`,
        );
      }
    }
    assert.throws(
      () => program.renameBinding(binding.id, fixture.capture),
      /capture|conflict/u,
      `${fixture.language} capture avoidance`,
    );
  }
});

test('binding rename rejects capture of an unresolved JavaScript reference', () => {
  const source = 'globalThis.y = 10; const x = 1; globalThis.result = x + y;';
  const program = analyzeProgram(source, 'JavaScript');
  const binding = program.bindings.find(({ name }) => name === 'x');
  assert.ok(binding);
  assert.ok(program.unresolvedReferences.some(({ name }) => name === 'y'));
  assert.throws(() => program.renameBinding(binding.id, 'y'), /capture/u);

  const siblingSource = 'function f() { const x = 1; return x; } function g() { return y; }';
  const siblingProgram = analyzeProgram(siblingSource, 'JavaScript');
  const siblingBinding = siblingProgram.bindings.find(({ name }) => name === 'x');
  assert.ok(siblingBinding);
  assert.equal(
    siblingProgram.renameBinding(siblingBinding.id, 'y').emit(),
    'function f() { const y = 1; return y; } function g() { return y; }',
  );
});

test('binding rename permits disjoint nested names and rejects actual nested capture', () => {
  for (const fixture of corpus.nestedRenameCases) {
    const program = analyzeProgram(fixture.source, 'JavaScript');
    const binding = program.bindings.find(({ name }) => name === fixture.binding);
    assert.ok(binding);
    const originalContext = {};
    runInNewContext(fixture.source, originalContext);
    assert.deepEqual(JSON.parse(JSON.stringify(originalContext.result)), fixture.expectedObservation);

    if (!fixture.allowed) {
      assert.throws(() => program.renameBinding(binding.id, fixture.replacement), /capture/u);
      continue;
    }
    const renamed = program.renameBinding(binding.id, fixture.replacement);
    assert.equal(renamed.emit(), fixture.expected);
    assert.equal(renamed.network.verifyFullMatch().isClean(), true);
    const renamedContext = {};
    runInNewContext(renamed.emit(), renamedContext);
    assert.deepEqual(JSON.parse(JSON.stringify(renamedContext.result)), fixture.expectedObservation);
  }
});

test('JavaScript var bindings use function scope and include references before declaration', () => {
  for (const fixture of corpus.varScopeCases) {
    const program = analyzeProgram(fixture.source, 'JavaScript');
    const binding = program.bindings
      .filter(({ name }) => name === fixture.binding)[fixture.declarationOccurrence];
    assert.ok(binding, `binding in ${fixture.source}`);
    assert.equal(binding.references.length, fixture.expectedReferences);
    const renamed = program.renameBinding(binding.id, fixture.replacement);
    assert.equal(renamed.emit(), fixture.expected);
    for (const source of [fixture.source, renamed.emit()]) {
      const context = {};
      runInNewContext(source, context);
      assert.deepEqual(JSON.parse(JSON.stringify(context.result)), fixture.expectedObservation);
    }
  }
  const lexical = analyzeProgram('{ let x = 1; } x;', 'JavaScript');
  assert.equal(lexical.bindings.find(({ name }) => name === 'x').references.length, 0);
  assert.ok(lexical.unresolvedReferences.some(({ name }) => name === 'x'));
});

test('structured construction, query, edits, cloning, movement, and emission reparse cleanly', () => {
  for (const fixture of corpus.transformationPrograms) {
    const program = constructProgram(fixture.source, fixture.language);
    assert.equal(program.emit(), fixture.source, `${fixture.language} construct and emit`);
    assert.ok(program.querySyntax('identifier').length >= 2, `${fixture.language} query`);
    const boundary = Buffer.byteLength(fixture.first);
    const first = { start: 0, end: boundary };
    const second = { start: boundary, end: Buffer.byteLength(fixture.source) };

    const replacement = fixture.first.replace(/first|FIRST/u, 'primary');
    assert.equal(
      program.replace(first, replacement).emit(),
      replacement + fixture.second,
      `${fixture.language} replace`,
    );
    assert.equal(
      program.insert(second.end, fixture.inserted).emit(),
      fixture.source + fixture.inserted,
      `${fixture.language} insert`,
    );
    assert.equal(program.delete(second).emit(), fixture.first, `${fixture.language} delete`);
    assert.equal(
      program.clone(first, second.end).emit(),
      fixture.source + fixture.first,
      `${fixture.language} clone`,
    );
    assert.equal(
      program.move(second, 0).emit(),
      fixture.second + fixture.first,
      `${fixture.language} move`,
    );
    assert.throws(() => program.replace({ start: 0, end: fixture.source.length + 1 }, ''), /range/u);
    assert.throws(() => program.move(first, 1), /inside/u);
  }
  assert.throws(() => constructProgram('const = ;\n', 'JavaScript'), /parse cleanly/u);
});

test('structured programs survive snapshots without retaining an original source buffer', () => {
  for (const fixture of corpus.transformationPrograms) {
    const project = {
      root: `/workspace/${fixture.language}`,
      files: ['main'],
      dependencies: ['standard-library'],
      extensions: ['fixture-extension'],
    };
    const constructed = constructProgramFromFragments(
      [fixture.first, fixture.second],
      fixture.language,
      project,
    );
    assert.equal(constructed.emit(), fixture.source, `${fixture.language} fragment construction`);

    const serialized = constructed.serializeSnapshot();
    const snapshot = JSON.parse(serialized);
    assert.equal(Object.hasOwn(snapshot, 'source'), false, `${fixture.language} source omitted`);
    assert.ok(snapshot.fragments.length > 0, `${fixture.language} retained fragments`);
    assert.deepEqual(snapshot.project, project, `${fixture.language} project context`);

    const restored = ProgramRepresentation.fromSnapshot(serialized);
    assert.equal(restored.emit(), fixture.source, `${fixture.language} snapshot emission`);
    assert.deepEqual(restored.project, project, `${fixture.language} restored project`);
    assert.equal(restored.network.verifyFullMatch().isClean(), true, `${fixture.language} reparse`);
    assert.ok(restored.querySyntax('identifier').length >= 2, `${fixture.language} restored CST`);

    snapshot.fragments[0].byteEnd += 1;
    assert.throws(
      () => ProgramRepresentation.fromSnapshot(snapshot),
      /snapshot/u,
      `${fixture.language} rejects corrupt fragment spans`,
    );
  }

  const unicode = constructProgramFromFragments(['const café = "', '☕";\n'], 'JavaScript');
  const restoredUnicode = ProgramRepresentation.fromSnapshot(unicode.serializeSnapshot());
  assert.equal(restoredUnicode.emit(), 'const café = "☕";\n');
});

test('all 12 translation hooks emit reversible target-native source envelopes', () => {
  const contracts = translationContracts();
  assert.equal(contracts.length, 12);
  assert.equal(new Set(contracts.map(({ source, target }) => `${source}->${target}`)).size, 12);
  for (const contract of contracts) {
    assert.equal(contract.support, TranslationSupport.PortableEncoding);
    assert.match(contract.encoding, /portable source envelope v1/u);
    assert.match(contract.obligation, /semantic translation is not implemented/u);

    const fixture = corpus.semanticPrograms.find(({ language }) => language === contract.source);
    const translated = translateProgram(fixture.source, contract.source, contract.target);
    assert.equal(translated.sourceLanguage, contract.source);
    assert.equal(translated.targetLanguage, contract.target);
    assert.notEqual(translated.code, fixture.source);
    assert.equal(
      LinkNetwork.parse(translated.code, contract.target).verifyFullMatch().isClean(),
      true,
      `${contract.source} -> ${contract.target} target CST`,
    );
    const decoded = decodeProgramTranslation(translated.code, contract.target);
    assert.equal(decoded.sourceLanguage, contract.source);
    assert.equal(decoded.source, fixture.source);
    assert.equal(analyzeProgram(decoded.source, decoded.sourceLanguage).emit(), fixture.source);
    assert.throws(
      () => decodeProgramTranslation(
        translated.code.replace('portable-source-envelope', 'portable-source-envelopf'),
        contract.target,
      ),
      /invalid portable envelope/u,
    );
  }
});

test('JavaScript parser registry mirrors Rust extension dispatch', () => {
  const registry = new ParserRegistry().with_parser('shout', (text, language, configuration) =>
    LinkNetwork.parseLosslessText(text.toUpperCase(), language, configuration));

  assert.equal(registry.isRegistered('SHOUT'), true);
  assert.equal(registry.is_registered('SHOUT'), true);
  assert.equal(registry.parser_for('shout'), registry.parserFor('shout'));
  assert.equal(registry.size(), 1);
  assert.equal(registry.len(), 1);
  assert.equal(registry.is_empty(), false);
  assert.equal(
    LinkNetwork.parse_with_registry(registry, 'hello', 'ShOuT').reconstructText(),
    'HELLO',
  );
});
