import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import test from 'node:test';

import {
  GraphQlOperationType,
  GraphQlRootMapping,
  GraphQlSchemaRegistry,
  LinkType,
  QueryComparisonOperator,
  QueryOperation,
  QueryPlan,
  lowerGraphQl,
} from '../src/index.js';

const fixtureUrl = new URL('../../parity/fixtures/graphql-query-plans.json', import.meta.url);

async function registryAndCases() {
  const fixture = JSON.parse(await readFile(fixtureUrl, 'utf8'));
  return {
    registry: GraphQlSchemaRegistry.fromJson(fixture.registry),
    cases: fixture.cases,
  };
}

test('shared GraphQL fixtures lower to canonical query plans', async () => {
  const { registry, cases } = await registryAndCases();

  for (const fixture of cases) {
    const lowered = lowerGraphQl(fixture.source, registry);
    assert.deepEqual(lowered.plan().toCanonicalObject(), fixture.canonicalPlan, fixture.name);
    assert.notEqual(lowered.plan().sourceEvidence().length, 0);

    const root = lowered.network().link(lowered.rootLink());
    assert.equal(root.metadata().linkType, LinkType.Semantic);
    assert.ok(root.references().some((reference) => (
      lowered.network().link(reference)?.metadata().linkType === LinkType.Syntax
    )));
  }
});

test('public query plan API canonicalizes additional frontend objects', () => {
  const plan = new QueryPlan(QueryOperation.Select, 'user');
  plan.projection.push('user.id');
  plan.filter = {
    compare: {
      value: { z: 2, a: 1 },
      operator: QueryComparisonOperator.Equal,
      field: 'user.metadata',
    },
  };
  plan.limit = 10;

  assert.deepEqual(plan.toCanonicalObject().filter, {
    compare: {
      field: 'user.metadata',
      operator: 'eq',
      value: { a: 1, z: 2 },
    },
  });
  plan.limit = Number.MAX_SAFE_INTEGER + 1;
  assert.throws(() => plan.toCanonicalObject());
});

test('GraphQL lowering fails closed for unmapped or ambiguous input', async () => {
  const { registry } = await registryAndCases();

  for (const source of [
    'query { unknown { id } }',
    'query { users { unknown } }',
    'query { users(unmapped: 1) { id } }',
    'query { users { id } users { name } }',
    'query { users { id }',
    'query { users(first: 01) { id } }',
    'query { users(first: 9007199254740993) { id } }',
    'query { result: users { id } }',
    'query { users { identifier: id } }',
    'query { users { sum } }',
    'mutation { createUser(input: {name: {unmapped: "Ada"}}) { id } }',
  ]) {
    assert.throws(() => lowerGraphQl(source, registry), undefined, source);
  }

  const mapping = new GraphQlRootMapping(
    GraphQlOperationType.Query,
    'users',
    QueryOperation.Select,
    'user',
  );
  const duplicates = new GraphQlSchemaRegistry();
  duplicates.registerRoot(mapping);
  assert.throws(() => duplicates.registerRoot(mapping));
  assert.throws(() => new GraphQlSchemaRegistry().registerRoot(
    new GraphQlRootMapping(
      GraphQlOperationType.Query,
      'emptyField',
      QueryOperation.Select,
      'user',
    ).withField('id', ''),
  ));
  assert.throws(() => new GraphQlSchemaRegistry().registerRoot(
    new GraphQlRootMapping(
      GraphQlOperationType.Query,
      'ambiguousField',
      QueryOperation.Select,
      'user',
    ).withField('id', 'user.id').withField('ID', 'legacy.id'),
  ));
});

test('GraphQL source evidence uses UTF-8 byte offsets', async () => {
  const { registry } = await registryAndCases();
  const source = 'mutation { createUser(input: {name: "Zoë", status: ACTIVE}) { id } }';
  const evidence = lowerGraphQl(source, registry)
    .plan()
    .sourceEvidence()
    .find((entry) => entry.role() === 'projection:user.id');
  const projectionStart = source.lastIndexOf('id');

  assert.ok(evidence);
  assert.equal(
    evidence.span().byteRange.start,
    Buffer.byteLength(source.slice(0, projectionStart)),
  );
});
