import assert from 'node:assert/strict';
import { test } from 'node:test';

import {
  GrammarBuilder,
  LinkCliSubstitution,
  LinkCliSubstitutionKind,
  LinkMetadata,
  LinkNetwork,
  LinkQuery,
  LinkType,
  ParseConfiguration,
  ProbabilisticTruthValue,
  Probability,
  ReplacementRule,
  SubstitutionRule,
  TruthValue,
  TranslationRule,
  TranslationRuleSet,
  TriviaAttachmentPolicy,
  emitJavascriptParser,
  emitPeggy,
} from '../src/index.js';

test('arbitrary file bytes round-trip without UTF-8 loss', () => {
  const source = Uint8Array.from([0x00, 0x25, 0x50, 0x44, 0x46, 0xff, 0x80, 0x0a]);

  const network = LinkNetwork.parseBytes(source, 'application/pdf');

  assert.deepEqual(network.reconstructBytes(), source);
  const tokens = network.links().filter((link) => link.metadata().linkType === LinkType.SourceToken);
  assert.equal(tokens.length, 1);
  assert.ok(tokens.every((link) => link.metadata().language === 'application/pdf'));
});

test('files round-trip across binary storage chunk boundaries', () => {
  const source = Uint8Array.from({ length: 5000 }, (_, index) => index % 251);

  const network = LinkNetwork.parseBytes(source, 'application/octet-stream');

  assert.deepEqual(network.reconstructBytes(), source);
  assert.equal(
    network.links().filter((link) => link.metadata().linkType === LinkType.SourceToken).length,
    2,
  );
});

test('empty files retain their format', () => {
  const network = LinkNetwork.parseBytes([], 'application/octet-stream');

  assert.deepEqual(network.reconstructBytes(), new Uint8Array());
  assert.ok(
    network
      .links()
      .some(
        (link) =>
          link.metadata().linkType === LinkType.Language &&
          link.metadata().term === 'application/octet-stream',
      ),
  );
});

test('lossless JavaScript parse reconstructs original text and indexes identifiers', () => {
  const network = LinkNetwork.parse(
    'const value = call(value);\n',
    'JavaScript',
    ParseConfiguration.default(),
  );

  assert.equal(network.reconstructText(), 'const value = call(value);\n');

  const identifiers = network.find(LinkQuery.fromSexpression('(identifier) @name'));
  assert.ok(identifiers.length >= 3);
});

test('lossless source tokens expose the Rust-compatible Token link type alias', () => {
  assert.equal(LinkType.Token, LinkType.SourceToken);

  const network = LinkNetwork.parse('x', 'JavaScript', ParseConfiguration.default());
  const tokenLinks = network.queryLinks(LinkQuery.byType(LinkType.Token));

  assert.equal(tokenLinks.length, 1);
  assert.equal(tokenLinks[0].metadata().linkType, LinkType.Token);
});

test('S-expression query transform replaces captured identifier source ranges', () => {
  const network = LinkNetwork.parse(
    'const oldName = call(oldName);\n',
    'JavaScript',
    ParseConfiguration.default(),
  );
  const query = LinkQuery.fromSexpression(`
    (identifier) @target
    (#eq? @target "oldName")
  `);

  const report = network.replace(
    network.find(query),
    ReplacementRule.capturedText('target', 'newName'),
  );

  assert.equal(report.isEmpty(), false);
  assert.equal(network.reconstructText(), 'const newName = call(newName);\n');
});

test('identifier transforms do not rewrite strings or comments', () => {
  const source = 'const x = 1; const s = "x"; // x\n';
  const network = LinkNetwork.parse(source, 'JavaScript', ParseConfiguration.default());
  const query = LinkQuery.fromSexpression(`
    (identifier) @target
    (#eq? @target "x")
  `);

  const matches = network.find(query);
  network.replace(matches, ReplacementRule.capturedText('target', 'y'));

  assert.equal(matches.length, 1);
  assert.equal(network.reconstructText(), 'const y = 1; const s = "x"; // x\n');
});

test('structural substitution updates relation references', () => {
  const network = new LinkNetwork();
  const one = network.insertPoint('1');
  const two = network.insertPoint('2');
  const relation = network.insertLink(
    [one, one],
    LinkMetadata.new().withLinkType(LinkType.Relation),
  );

  const report = network.applySubstitution(new SubstitutionRule([one, one], [one, two]));

  assert.deepEqual(report.updated().map((id) => id.asU64()), [relation.asU64()]);
  assert.deepEqual(
    network.link(relation).references().map((id) => id.asU64()),
    [one.asU64(), two.asU64()],
  );
});

test('link-cli substitution text covers create read update and delete', () => {
  const network = new LinkNetwork();

  const create = network.applyLinkCliSubstitutionText('() ((1 1))');
  assert.deepEqual(create.created().map((id) => id.asU64()), [1]);
  assert.deepEqual(network.link(1).references().map((id) => id.asU64()), [1, 1]);

  const read = network.applyLinkCliSubstitutionText('((1: 1 1)) ((1: 1 1))');
  assert.deepEqual(read.updated().map((id) => id.asU64()), [1]);
  assert.deepEqual(network.link(1).references().map((id) => id.asU64()), [1, 1]);

  const update = network.applyLinkCliSubstitutionText('((1: 1 1)) ((1: 1 2))');
  assert.deepEqual(update.updated().map((id) => id.asU64()), [1]);
  assert.deepEqual(network.link(1).references().map((id) => id.asU64()), [1, 2]);

  const deletion = network.applyLinkCliSubstitutionText('((1 2)) ()');
  assert.deepEqual(deletion.deleted().map((id) => id.asU64()), [1]);
  assert.equal(network.link(1), undefined);
});

test('link-cli substitution classifies command kinds', () => {
  assert.equal(
    LinkCliSubstitution.parse('() ((1 1))').kind(),
    LinkCliSubstitutionKind.Create,
  );
  assert.equal(
    LinkCliSubstitution.parse('((1: 1 1)) ((1: 1 1))').kind(),
    LinkCliSubstitutionKind.ReadIdentity,
  );
  assert.equal(
    LinkCliSubstitution.parse('((1: 1 1)) ((1: 1 2))').kind(),
    LinkCliSubstitutionKind.Update,
  );
  assert.equal(
    LinkCliSubstitution.parse('((1 1)) ()').kind(),
    LinkCliSubstitutionKind.Delete,
  );
});

test('LiNo serialization round-trips JavaScript network topology', () => {
  const network = LinkNetwork.parse('alpha', 'txt', ParseConfiguration.default());
  const lino = network.toLino();
  const restored = LinkNetwork.fromLino(lino);

  assert.equal(restored.toLino(), lino);
});

test('snapshots preserve immutable network versions', () => {
  const network = LinkNetwork.parse('alpha', 'txt', ParseConfiguration.default());
  const snapshot = network.snapshot(1, 'unit test');

  network.insertPoint('later');

  assert.equal(snapshot.version(), 1);
  assert.equal(snapshot.network().reconstructText(), 'alpha');
});

test('translation rule sets reconstruct through semantic query templates', () => {
  const network = new LinkNetwork();
  const concept = network.insertConceptExpression('greeting', 'English', 'hello');
  network.insertLink(
    [concept],
    LinkMetadata.new()
      .withLinkType(LinkType.Semantic)
      .withNamed(true)
      .withTerm('proposition:greeting'),
  );
  const rules = new TranslationRuleSet('greeting').withRule(
    new TranslationRule(
      'spanish greeting',
      LinkQuery.byType(LinkType.Semantic).withTerm('proposition:greeting'),
    ).withTemplate('Spanish', 'hola'),
  );

  assert.equal(
    network.reconstructTextAsWithRules('Spanish', ParseConfiguration.default(), rules),
    'hola',
  );
  assert.deepEqual(TranslationRuleSet.fromLino(rules.toLino()), rules);
  assert.deepEqual(TranslationRuleSet.fromJson(rules.toJson()), rules);

  const dynamicRules = new TranslationRuleSet('dynamic').withRule(
    new TranslationRule('dynamic link', LinkQuery.byType(LinkType.Dynamic))
      .withTemplate('English', 'dynamic'),
  );
  assert.match(dynamicRules.toLino(), /%22link_type%22%3A%22link%22/);
  assert.deepEqual(TranslationRuleSet.fromLino(dynamicRules.toLino()), dynamicRules);
});

test('translation rule sets load Rust canonical LiNo metadata', () => {
  const rustLino = [
    '(1: (meta: (t: semantic) (n: 1) (term: translation-rule-set) (def: capital-demo)))',
    '(2: 1 (meta: (t: semantic) (n: 1) (term: translation-rule) (def: capital%20sentence)))',
    '(3: 2 (meta: (t: semantic) (n: 1) (term: translation-rule-match) (def: %7B%22link_type%22%3A%22semantic%22%2C%22named%22%3Atrue%2C%22term%22%3A%22proposition%3Acapital%22%7D)))',
    '(4: 2 (meta: (t: semantic) (n: 1) (term: object) (def: 2) (lang: translation-rule-reference-capture)))',
    '(5: 2 (meta: (t: semantic) (n: 1) (term: subject) (def: 1) (lang: translation-rule-reference-capture)))',
    '(6: 2 (meta: (t: semantic) (n: 1) (term: %7Bobject%7D%20is%20the%20capital%20of%20%7Bsubject%7D.) (def: translation-rule-template) (lang: English)))',
    '(7: 2 (meta: (t: semantic) (n: 1) (term: %7Bobject%7D%20es%20la%20capital%20de%20%7Bsubject%7D.) (def: translation-rule-template) (lang: Spanish)))',
    '',
  ].join('\n');
  const expected = new TranslationRuleSet('capital-demo').withRule(
    new TranslationRule(
      'capital sentence',
      LinkQuery.byType(LinkType.Semantic)
        .withTerm('proposition:capital')
        .withNamed(true),
    )
      .withReferenceCapture('subject', 1)
      .withReferenceCapture('object', 2)
      .withTemplate('English', '{object} is the capital of {subject}.')
      .withTemplate('Spanish', '{object} es la capital de {subject}.'),
  );
  const restored = TranslationRuleSet.fromLino(rustLino);

  assert.deepEqual(restored, expected);
  assert.equal(expected.toLino(), rustLino);

  const network = new LinkNetwork();
  const proposition = network.insertConceptExpression('capital', 'English', 'capital');
  const france = network.insertConceptExpression('Q142', 'English', 'France');
  network.insertConceptExpression('Q142', 'Spanish', 'Francia');
  const paris = network.insertConceptExpression('Q90', 'English', 'Paris');
  network.insertConceptExpression('Q90', 'Spanish', 'Paris');
  network.insertLink(
    [proposition, france, paris],
    LinkMetadata.new()
      .withLinkType(LinkType.Semantic)
      .withNamed(true)
      .withTerm('proposition:capital'),
  );

  assert.equal(
    network.reconstructTextAsWithRules('Spanish', ParseConfiguration.default(), restored),
    'Paris es la capital de Francia.',
  );
});

test('verification reports parse recovery issues', () => {
  assert.equal(
    LinkNetwork.parse('alpha', 'txt', ParseConfiguration.default()).verifyFullMatch().isClean(),
    true,
  );
  assert.equal(
    LinkNetwork.parse(')', 'txt', ParseConfiguration.default()).verifyFullMatch().isClean(),
    false,
  );
});

test('CSV accepts RFC 4180 fields including single characters', () => {
  // Mirrors rust/tests/unit/grammar_parsing.rs.
  const source = 'name,value\na,1\n"q ""x""",,2.5\r\nb,true\n';
  const network = LinkNetwork.parse(source, 'CSV');
  assert.equal(network.reconstructText(), source);
  assert.equal(network.verifyFullMatch().isClean(), true);
  const fieldKinds = network.links()
    .filter((link) => link.metadata().linkType === LinkType.Syntax && link.metadata().term === 'field')
    .map((link) => network.link(link.references()[0]).metadata().term);
  assert.deepEqual(
    fieldKinds,
    ['text', 'text', 'text', 'number', 'text', 'text', 'float', 'text', 'boolean'],
  );
});

test('CSV rejects quotes outside RFC 4180 quoted fields', () => {
  for (const source of ['"a"b,1\n', 'a,"b\n', 'a,"x"y\n', 'a"b\n']) {
    const network = LinkNetwork.parse(source, 'CSV');
    assert.equal(network.reconstructText(), source);
    assert.equal(network.verifyFullMatch().isClean(), false, JSON.stringify(source));
  }
});

test('hidden grammar text is not labeled whitespace trivia', () => {
  // VB's `Module` and `End Module` keywords are hidden grammar rules, so
  // tree-sitter exposes no node for them; they must not become extras.
  const source = 'Module Program\nEnd Module\n';
  const network = LinkNetwork.parse(source, 'Visual Basic');
  assert.equal(network.reconstructText(), source);
  const tokens = network.links()
    .filter((link) => link.metadata().linkType === LinkType.SourceToken)
    .map((link) => [link.metadata().term, link.metadata().flags.isExtra]);
  for (const [text, extra] of tokens) {
    assert.equal(extra, /^\p{White_Space}+$/u.test(text), `${JSON.stringify(text)} extra flag`);
  }
  // The text is a token directly below its grammar node, not a synthetic
  // Syntax node, as in Rust.
  const owners = network.links()
    .filter((link) => link.metadata().linkType === LinkType.Syntax)
    .flatMap((link) => link.references()
      .map((reference) => network.link(reference).metadata())
      .filter(({ linkType, term }) => linkType === LinkType.SourceToken && term.includes('Module'))
      .map(({ term }) => [link.metadata().term, term]));
  assert.deepEqual(owners, [['module_block', 'Module'], ['module_block', 'End Module']]);
  assert.equal(
    network.links().some((link) => ['hidden_text', 'whitespace'].includes(link.metadata().term)),
    false,
  );
});

test('Markdown inline content is parsed with the inline grammar', () => {
  // Block continuations inside inline content stay under the deepest inline
  // node that spans them, here the emphasis split across two quote lines.
  const source = '# Title *x*\n\n> Quote with `code`\n> and [link](https://example.com) *em\n> ph* é.\n\n| a | b |\n|---|---|\n| *c* | d |\n';
  const network = LinkNetwork.parse(source, 'Markdown');
  const bytes = Buffer.from(source);
  const nodes = [];
  for (const parent of network.links()) {
    if (parent.metadata().linkType !== LinkType.Syntax) continue;
    for (const child of parent.references().map((reference) => network.link(reference))) {
      const metadata = child?.metadata();
      if (metadata?.linkType !== LinkType.Syntax || !metadata.named) continue;
      const { start, end } = metadata.span.byteRange;
      nodes.push(JSON.stringify([metadata.term, parent.metadata().term, bytes.subarray(start, end).toString()]));
    }
  }

  assert.equal(network.reconstructText(), source);
  assert.ok(network.verifyFullMatch().isClean());
  for (const expected of [
    ['emphasis', 'inline', '*x*'],
    ['code_span', 'inline', '`code`'],
    ['inline_link', 'inline', '[link](https://example.com)'],
    ['link_destination', 'inline_link', 'https://example.com'],
    ['block_continuation', 'inline', '> '],
    ['block_continuation', 'emphasis', '> '],
    ['emphasis', 'pipe_table_cell', '*c*'],
  ]) {
    assert.ok(nodes.includes(JSON.stringify(expected)), `${expected} in ${nodes}`);
  }
});

test('grammar builders emit Peggy grammar and JavaScript parser module text', () => {
  const grammar = new GrammarBuilder('Word')
    .terminal('letter', GrammarBuilder.charRange('a', 'z'))
    .nonterminal('Word', GrammarBuilder.repeat1(GrammarBuilder.ref('letter')))
    .build();

  const peggy = emitPeggy(grammar);
  const parserModule = emitJavascriptParser(grammar);

  assert.match(peggy, /Word/);
  assert.match(parserModule, /peggy\.generate/);
});

test('semantic truth values cover many-valued and paradox cases', () => {
  assert.equal(TruthValue.True.and(TruthValue.Unknown), TruthValue.Unknown);
  assert.equal(TruthValue.True.and(TruthValue.Both), TruthValue.Both);
  assert.equal(TruthValue.False.or(TruthValue.Unknown), TruthValue.Unknown);
  assert.equal(TruthValue.Both.negate(), TruthValue.Both);
});

test('probabilistic truth values cover relative-meta-logic probability cases', () => {
  const half = Probability.fromRatio(1, 2);
  const likely = Probability.fromBasisPoints(7_500);

  assert.ok(half);
  assert.ok(likely);
  assert.equal(Probability.from_ratio(1, 2).basis_points(), 5_000);

  const liar = new ProbabilisticTruthValue(half);
  const event = new ProbabilisticTruthValue(likely);
  const aliasedLiar = ProbabilisticTruthValue.from_ratio(1, 2);

  assert.equal(liar.trueProbability().basisPoints(), 5_000);
  assert.equal(liar.falseProbability().basisPoints(), 5_000);
  assert.ok(aliasedLiar.equals(liar));
  assert.ok(liar.negate().equals(liar));
  assert.equal(event.negate().trueProbability().basisPoints(), 2_500);
  assert.equal(liar.and(event).trueProbability().basisPoints(), 3_750);
  assert.equal(liar.or(event).trueProbability().basisPoints(), 8_750);
});

test('extra tokens get Trivia links owned by the Syntax link above them per policy', () => {
  const triviaOf = (source, language, policy) => {
    const network = LinkNetwork.parse(
      source,
      language,
      ParseConfiguration.default().withTriviaAttachmentPolicy(policy),
    );
    const describe = (id) => {
      const { linkType, term } = network.link(id).metadata();
      return `${linkType}:${term}`;
    };
    return network
      .links()
      .filter((link) => link.metadata().linkType === LinkType.Trivia)
      .map((link) => [link.metadata().term, ...link.references().map(describe)]);
  };
  const containment = ['containment trivia', 'Syntax:whitespace', 'SourceToken: '];
  const token = ['token trivia', 'SourceToken: '];
  assert.deepEqual(triviaOf('a b', 'txt', TriviaAttachmentPolicy.Combined), [containment, token]);
  assert.deepEqual(triviaOf('a b', 'txt', TriviaAttachmentPolicy.ContainmentLink), [containment]);
  assert.deepEqual(triviaOf('a b', 'txt', TriviaAttachmentPolicy.TokenLink), [token]);
  // A grammar extra such as a comment is owned by its own leaf Syntax link,
  // and whitespace between grammar nodes by the node enclosing it.
  assert.deepEqual(triviaOf('x; // note\n', 'JavaScript', TriviaAttachmentPolicy.ContainmentLink), [
    ['containment trivia', 'Syntax:comment', 'SourceToken:// note'],
    ['containment trivia', 'Syntax:program', 'SourceToken: '],
    ['containment trivia', 'Syntax:program', 'SourceToken:\n'],
  ]);
});
