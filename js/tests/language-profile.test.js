import assert from 'node:assert/strict';
import { test } from 'node:test';

import {
  LinkMetadata,
  LinkNetwork,
  LinkQuery,
  LinkType,
  TranslationRule,
  TranslationRuleSet,
} from '../src/index.js';
import {
  LanguageProfile,
  LanguageProfileLinks,
  LanguageProfileViolation,
} from '../src/language-profile.js';

function countWithTerm(network, term) {
  return network
    .links()
    .filter((link) => link.metadata().linkType === LinkType.Semantic && link.metadata().term === term)
    .length;
}

test('language profiles are declared as queryable links', () => {
  const network = new LinkNetwork();
  const profile = LanguageProfile.new('Custom', 'custom')
    .withLinkType(LinkType.Semantic)
    .withConcept('proposition:custom')
    .withTranslationRule('custom render');

  const links = profile.declareIn(network);

  assert.ok(links instanceof LanguageProfileLinks);
  assert.ok(network.link(links.profile()) !== undefined);
  assert.equal(countWithTerm(network, 'language-profile'), 1);
  assert.equal(countWithTerm(network, 'language-profile:link-type'), 1);
  assert.equal(countWithTerm(network, 'language-profile:concept'), 1);
  assert.equal(countWithTerm(network, 'language-profile:translation-rule'), 1);
  // One profile root + three capabilities.
  assert.equal(links.capabilities().length, 3);
});

test('declareIn is idempotent and reuses existing profile/capability links', () => {
  const network = new LinkNetwork();
  const profile = LanguageProfile.new('Custom', 'custom')
    .withLinkType(LinkType.Semantic)
    .withConcept('proposition:custom');

  const first = profile.declareIn(network);
  const second = profile.declareIn(network);

  assert.ok(first.profile().equals(second.profile()));
  assert.equal(countWithTerm(network, 'language-profile'), 1);
  assert.equal(countWithTerm(network, 'language-profile:concept'), 1);
});

test('declared profile root records language and name as definition', () => {
  const network = new LinkNetwork();
  const links = LanguageProfile.javascript().declareIn(network);
  const root = network.link(links.profile());

  assert.equal(root.metadata().language, 'JavaScript');
  assert.equal(root.metadata().definition, 'JavaScript');
});

test('builtin resolves the javascript profile by name and alias', () => {
  const fromName = LanguageProfile.builtin('JavaScript');
  const fromAlias = LanguageProfile.builtin('js');

  assert.ok(fromName);
  assert.ok(fromAlias);
  assert.ok(fromName.supportsLinkType(LinkType.Semantic));
  assert.ok(fromName.supportsLinkType(LinkType.Syntax));
  assert.equal(LanguageProfile.builtin('rust'), undefined);
});

test('validateNetwork accepts supported link types and concepts', () => {
  const profile = LanguageProfile.javascript().withConcept('proposition:capital');
  const network = new LinkNetwork();
  network.insertLink(
    [],
    LinkMetadata.new().withLinkType(LinkType.Semantic).withTerm('proposition:capital'),
  );

  assert.doesNotThrow(() => profile.validateNetwork(network));
});

test('validateNetwork rejects an unsupported link type in id order', () => {
  // Profile supporting only Semantic; a Concept link is unsupported.
  const profile = LanguageProfile.new('OnlySemantic', 'custom').withLinkType(LinkType.Semantic);
  const network = new LinkNetwork();
  network.insertLink([], LinkMetadata.new().withLinkType(LinkType.Concept).withTerm('thing'));

  assert.throws(
    () => profile.validateNetwork(network),
    (error) => {
      assert.ok(error instanceof LanguageProfileViolation);
      assert.ok(error.feature().includes('concept'));
      assert.ok(error.message.includes('Unsupported feature:'));
      return true;
    },
  );
});

test('profiles can be computed from translation rule set domains', () => {
  const rules = new TranslationRuleSet('capital-demo').withRule(
    new TranslationRule(
      'capital sentence',
      LinkQuery.byType(LinkType.Semantic).withTerm('proposition:capital'),
    ).withTemplate('JavaScript', 'capital({subject}, {object})'),
  );

  const profile = LanguageProfile.fromRuleSet('JavaScript', 'JavaScript', rules);

  assert.ok(profile.supportsLinkType(LinkType.Semantic));
  assert.ok(profile.supportsConcept('proposition:capital'));
  assert.ok(profile.supportsTranslationRule('capital sentence'));

  const network = new LinkNetwork();
  const semantic = network.insertLink(
    [],
    LinkMetadata.new().withLinkType(LinkType.Semantic).withTerm('proposition:capital'),
  );

  assert.doesNotThrow(() => profile.validateNetwork(network));
  assert.ok(network.link(semantic).id().equals(semantic));

  network.insertLink(
    [],
    LinkMetadata.new().withLinkType(LinkType.Semantic).withTerm('proposition:population'),
  );

  assert.throws(
    () => profile.validateNetwork(network),
    (error) => {
      assert.ok(error instanceof LanguageProfileViolation);
      assert.ok(error.feature().includes('proposition:population'));
      return true;
    },
  );
});

test('accessors expose deterministic sorted order', () => {
  const profile = LanguageProfile.new('Sorted', 'custom')
    .withConcept('zebra')
    .withConcept('alpha')
    .withTranslationRule('rule-b')
    .withTranslationRule('rule-a')
    .withConceptFallback('heading', 'paragraph')
    .withConceptFallback('aside', 'note');

  assert.deepEqual(profile.concepts(), ['alpha', 'zebra']);
  assert.deepEqual(profile.translationRules(), ['rule-a', 'rule-b']);
  assert.deepEqual([...profile.fallbacks().keys()], ['aside', 'heading']);
  assert.equal(profile.conceptFallback('heading'), 'paragraph');
  assert.equal(profile.conceptFallback('missing'), undefined);
});

test('builders return immutable copies', () => {
  const base = LanguageProfile.new('Base', 'custom');
  const derived = base.withLinkType(LinkType.Semantic);

  assert.equal(base.supportsLinkType(LinkType.Semantic), false);
  assert.equal(derived.supportsLinkType(LinkType.Semantic), true);
});

test('snake_case aliases mirror camelCase methods', () => {
  const profile = LanguageProfile.new('Aliases', 'custom').with_link_type(LinkType.Semantic);
  assert.ok(profile.supports_link_type(LinkType.Semantic));
  assert.deepEqual(profile.link_types(), [LinkType.Semantic]);

  const ruleSet = new TranslationRuleSet('demo').withRule(
    new TranslationRule('only', LinkQuery.byType(LinkType.Semantic).withTerm('x')),
  );
  const derived = LanguageProfile.from_rule_set('X', 'X', ruleSet);
  assert.ok(derived.supports_translation_rule('only'));
});
