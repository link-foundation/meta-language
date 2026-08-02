import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import test from 'node:test';

import {
  GraphQlSchemaRegistry,
  LinkType,
  QueryAuthorization,
  SqlSchemaRegistry,
  lowerGraphQl,
  lowerSql,
} from '../src/index.js';

const sqlFixtures = JSON.parse(
  await readFile(
    new URL('../../parity/fixtures/sql-query-plans.json', import.meta.url),
    'utf8',
  ),
);
const graphqlFixtures = JSON.parse(
  await readFile(
    new URL('../../parity/fixtures/graphql-query-plans.json', import.meta.url),
    'utf8',
  ),
);

function registry() {
  return SqlSchemaRegistry.fromJson(sqlFixtures.registry);
}

test('SQL CRUD and query concepts lower to shared canonical fixtures', () => {
  for (const fixture of sqlFixtures.cases) {
    const lowered = lowerSql(fixture.sql, fixture.profile, registry());
    assert.deepEqual(lowered.plan().toCanonicalObject(), fixture.canonicalPlan, fixture.name);
    assert.equal(lowered.plan().authorization(), QueryAuthorization.Required);
    const [evidence] = lowered.plan().sourceEvidence();
    assert.equal(evidence.role(), `statement:${fixture.profile}`);
    assert.equal(evidence.span().byteRange.end, Buffer.byteLength(fixture.sql));
  }
});

test('vendor profiles normalize their common subset to one plan', () => {
  const expected = lowerSql(sqlFixtures.normalizationSql, 'sql-ansi', registry())
    .plan()
    .toCanonicalObject();
  for (const profile of sqlFixtures.profiles) {
    const plan = lowerSql(sqlFixtures.normalizationSql, profile, registry()).plan();
    assert.deepEqual(plan.toCanonicalObject(), expected, profile);
    assert.equal(plan.sourceEvidence()[0].role(), `statement:${profile}`);
  }
});

test('malformed, unmapped, and unrepresentable SQL fails closed', () => {
  for (const fixture of sqlFixtures.invalid) {
    assert.throws(
      () => lowerSql(fixture.sql, fixture.profile, registry()),
      undefined,
      fixture.sql,
    );
  }
  assert.throws(() => lowerSql('SELECT id FROM users', 'sql-unknown', registry()));
});

test('SQL lowering retains CST and semantic link evidence', () => {
  const lowered = lowerSql(
    'SELECT COUNT(*) AS total FROM users WHERE active = TRUE',
    'sql-ansi',
    registry(),
  );
  const root = lowered.network().link(lowered.rootLink());
  assert.equal(root.metadata().linkType, LinkType.Semantic);
  assert.equal(root.metadata().term, 'executable-query-plan');
  assert.ok(root.references().some((reference) => (
    lowered.network().link(reference)?.metadata().linkType === LinkType.Syntax
  )));
});

test('equivalent SQL and GraphQL fixture produce the same plan', () => {
  assert.equal(sqlFixtures.crossFrontend.fixture, 'graphql-query-plans.json');
  const fixture = graphqlFixtures.cases.find(
    ({ name }) => name === sqlFixtures.crossFrontend.case,
  );
  assert.ok(fixture, 'cross-frontend GraphQL case exists');

  const sqlPlan = lowerSql(fixture.equivalentSql, 'sql-ansi', registry()).plan();
  const graphqlRegistry = GraphQlSchemaRegistry.fromJson(graphqlFixtures.registry);
  const graphqlPlan = lowerGraphQl(fixture.source, graphqlRegistry).plan();
  assert.equal(sqlPlan.canonicalJson(), graphqlPlan.canonicalJson());
  assert.deepEqual(sqlPlan.toCanonicalObject(), fixture.canonicalPlan);
});
