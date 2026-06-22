import assert from 'node:assert/strict';
import { test } from 'node:test';

import { LinkNetwork } from '../src/network.js';
import { LinkMetadata, LinkType, ParseConfiguration } from '../src/primitives.js';
import {
  LinkRule,
  LinkRuleRegistry,
  LinkRuleSnapshotCase,
  LinkRuleSnapshotExpectation,
  LinkRuleSnapshotSuite,
  TraversalStrategy,
} from '../src/query-algebra.js';

function syntax(network, references, kind) {
  return network.insertLink(
    references,
    LinkMetadata.new().withLinkType(LinkType.Syntax).withNamed(true).withTerm(kind),
  );
}

function ids(matches) {
  return matches.map((ruleMatch) => ruleMatch.linkId().asU64());
}

test('ast_grep_style rule algebra supports relations, booleans, and named refs', () => {
  const network = new LinkNetwork();
  const root = syntax(network, [], 'root');
  const block = syntax(network, [root], 'block');
  const firstIdentifier = syntax(network, [block], 'identifier');
  const number = syntax(network, [block], 'number');
  const secondIdentifier = syntax(network, [block], 'identifier');

  const registry = new LinkRuleRegistry();
  registry.insert('container', LinkRule.fromSexpression('(kind block)'));
  const rule = LinkRule.fromSexpression(`
    (all
      (meta target identifier)
      (inside (kind identifier) (ref container))
      (precedes (kind identifier) (kind number))
      (not (follows (kind identifier) (kind number))))
  `);

  const matches = rule.matches(network, registry);

  assert.equal(matches.length, 1);
  assert.equal(matches[0].linkId().asU64(), firstIdentifier.asU64());
  assert.equal(matches[0].captures().first('target')?.asU64(), firstIdentifier.asU64());
  assert.ok(
    matches.every((ruleMatch) => ruleMatch.linkId().asU64() !== secondIdentifier.asU64()),
  );
  assert.ok(matches.every((ruleMatch) => ruleMatch.linkId().asU64() !== number.asU64()));
});

test('semgrep/coccinelle ellipsis and typed metavariables match gaps', () => {
  const network = new LinkNetwork();
  const root = syntax(network, [], 'root');
  const call = syntax(network, [root], 'call_expression');
  const first = syntax(network, [call], 'identifier');
  const gap = syntax(network, [call], 'comment');
  const second = syntax(network, [call], 'identifier');

  const rule = LinkRule.fromSexpression(`
    (all
      (kind call_expression)
      (ellipsis (meta first identifier) (meta second identifier)))
  `);

  const matches = rule.matches(network, new LinkRuleRegistry());

  assert.equal(matches.length, 1);
  assert.equal(matches[0].linkId().asU64(), call.asU64());
  assert.equal(matches[0].captures().first('first')?.asU64(), first.asU64());
  assert.equal(matches[0].captures().first('second')?.asU64(), second.asU64());
  assert.notEqual(matches[0].captures().first('first')?.asU64(), gap.asU64());
});

test('comby_style text pattern matches plain text fallback tokens', () => {
  const network = LinkNetwork.parse(
    'alpha beta gamma',
    'UnwiredPlainText',
    ParseConfiguration.default(),
  );
  const rule = LinkRule.fromSexpression('(text "alpha {{gap}} gamma")');

  const matches = rule.matches(network, new LinkRuleRegistry());

  assert.equal(matches.length, 1);
  assert.equal(matches[0].captures().text('gap'), 'beta');
});

test('stratego/rascal traversal orders and fixpoint are available', () => {
  const network = new LinkNetwork();
  const root = syntax(network, [], 'root');
  const outer = syntax(network, [root], 'call_expression');
  const inner = syntax(network, [outer], 'call_expression');
  const rule = LinkRule.fromSexpression('(kind call_expression)');
  const registry = new LinkRuleRegistry();

  const topdown = TraversalStrategy.TopDown.matches(network, rule, registry);
  const bottomup = TraversalStrategy.BottomUp.matches(network, rule, registry);
  const innermost = TraversalStrategy.Innermost.matches(network, rule, registry);

  assert.deepEqual(ids(topdown), [outer.asU64(), inner.asU64()]);
  assert.deepEqual(ids(bottomup), [inner.asU64(), outer.asU64()]);
  assert.deepEqual(ids(innermost), [inner.asU64()]);

  const report = TraversalStrategy.Fixpoint({ maxIterations: 4 }).applyMut(
    network,
    rule,
    registry,
    (currentNetwork) => {
      if (currentNetwork.findTerm('fixpoint:done') === undefined) {
        currentNetwork.insertPoint('fixpoint:done');
        return true;
      }
      return false;
    },
  );

  assert.equal(report.iterations(), 2);
  assert.equal(report.changed(), 1);
});

test('ast_grep rule snapshot harness verifies valid and invalid cases', () => {
  const suite = LinkRuleSnapshotSuite.new(
    LinkRule.fromSexpression('(text "replace {{name}}")'),
  )
    .withCase(
      LinkRuleSnapshotCase.new(
        'valid replacement target',
        'replace me',
        'UnwiredPlainText',
        LinkRuleSnapshotExpectation.Valid,
      ),
    )
    .withCase(
      LinkRuleSnapshotCase.new(
        'invalid replacement target',
        'ignore me',
        'UnwiredPlainText',
        LinkRuleSnapshotExpectation.Invalid,
      ),
    );

  const report = suite.run(new LinkRuleRegistry(), ParseConfiguration.default());

  assert.ok(report.isSuccess(), JSON.stringify(report.cases()));
  assert.equal(report.cases().length, 2);
});
