import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { test } from 'node:test';

import {
  LANGUAGE_REPRESENTATION_SCHEMA_VERSION,
  LinkNetwork,
  LinkQuery,
  LinkType,
  ParseConfiguration,
  ParserRegistry,
  ReplacementRule,
  TranslationSupport,
  analyzeProgram,
  constructProgram,
  decodeProgramTranslation,
  fourLanguageSupport,
  languageSupport,
  translationContracts,
  translateProgram,
} from '../src/index.js';

const corpus = JSON.parse(
  readFileSync(new URL('../../parity/fixtures/four-language-conformance.json', import.meta.url)),
);
const grammarInventory = JSON.parse(
  readFileSync(new URL('../../parity/language-grammar-inventory.json', import.meta.url)),
);

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

test('every JavaScript grammar inventory frontend retains and diagnoses prohibited NUL input', () => {
  for (const fixture of grammarInventory.languages) {
    const source = `${fixture.source}\0`;
    const network = LinkNetwork.parse(source, fixture.name);
    assert.equal(network.reconstructText(), source, `${fixture.name} malformed reconstruction`);
    assert.equal(network.verifyFullMatch().isClean(), false, `${fixture.name} malformed diagnostic`);
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

  for (const [language, source] of [
    ['LiNo', 'not a link\n'],
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
    assert.equal(support.bindingResolution, 'unavailable');
    assert.equal(support.typeElaboration, 'unavailable');
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

    const byKind = new Map(program.constructs.map((construct) => [construct.kind, construct]));
    for (const kind of fixture.represented) {
      assert.equal(byKind.get(kind)?.status, 'represented', `${fixture.language} ${kind}`);
      assert.ok(byKind.get(kind).evidence.length > 0, `${fixture.language} ${kind} evidence`);
    }
    for (const kind of fixture.notApplicable) {
      assert.equal(byKind.get(kind)?.status, 'not-applicable', `${fixture.language} ${kind}`);
      assert.ok(byKind.get(kind).rationale, `${fixture.language} ${kind} rationale`);
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
    assert.throws(
      () => program.renameBinding(binding.id, fixture.capture),
      /capture|conflict/u,
      `${fixture.language} capture avoidance`,
    );
  }
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

test('all 12 translation hooks emit reversible target-native source envelopes', () => {
  const contracts = translationContracts();
  assert.equal(contracts.length, 12);
  assert.equal(new Set(contracts.map(({ source, target }) => `${source}->${target}`)).size, 12);
  for (const contract of contracts) {
    assert.equal(contract.support, TranslationSupport.PortableEncoding);
    assert.match(contract.encoding, /portable source envelope v1/u);
    assert.equal(contract.obligation, null);

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
