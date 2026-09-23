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
  fourLanguageSupport,
  languageSupport,
  translationContracts,
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

test('all 12 translation hooks fail closed with a precise obligation', () => {
  const contracts = translationContracts();
  assert.equal(contracts.length, 12);
  assert.equal(new Set(contracts.map(({ source, target }) => `${source}->${target}`)).size, 12);
  for (const contract of contracts) {
    assert.equal(contract.support, TranslationSupport.UnsupportedObligation);
    assert.match(contract.obligation, new RegExp(`${contract.source}.*${contract.target}`));
    assert.match(contract.obligation, /must stop instead of relabelling source text/);
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
