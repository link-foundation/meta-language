import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import test from 'node:test';

import {
  LinkNetwork,
  LinkType,
  QueryAuthorization,
  QueryPlan,
  QueryPlanRegistry,
  SourceEvidence,
  lowerSql,
} from '../src/index.js';

const fixtures = JSON.parse(
  await readFile(new URL('../../parity/query-plan-fixtures.json', import.meta.url), 'utf8'),
);

test('SQL CRUD and query concepts lower to shared canonical fixtures', () => {
  for (const fixture of fixtures.cases) {
    const plan = lowerSql(fixture.sql, fixture.profile);
    assert.deepEqual(plan.canonicalJson(), fixture.plan, fixture.name);
    assert.equal(plan.authorization(), QueryAuthorization.Required);
    assert.equal(plan.evidence().language, fixture.profile);
    assert.equal(plan.evidence().span.byteRange.end, Buffer.byteLength(fixture.sql));
  }
});

test('vendor profiles normalize their common subset to one operation', () => {
  const expected = lowerSql(fixtures.normalizationSql, 'sql-ansi').canonicalJson();
  for (const profile of fixtures.profiles) {
    const plan = lowerSql(fixtures.normalizationSql, profile);
    assert.deepEqual(plan.canonicalJson(), expected, profile);
    assert.equal(plan.evidence().language, profile);
  }
});

test('malformed and semantically invalid statements fail closed', () => {
  for (const fixture of fixtures.invalid) {
    assert.throws(() => lowerSql(fixture.sql, fixture.profile), undefined, fixture.sql);
  }
  assert.throws(() => lowerSql('SELECT id FROM users', 'sql-unknown'));
});

test('plans declare an engine-neutral semantic links representation', () => {
  const plan = lowerSql('SELECT COUNT(*) AS total FROM users', 'sql-ansi');
  const network = new LinkNetwork();
  const declared = plan.declareIn(network);
  const root = network.link(declared.root);

  assert.equal(root.metadata().linkType, LinkType.Semantic);
  assert.equal(root.metadata().term, 'query-plan');
  assert.ok(
    declared.links
      .map((id) => network.link(id))
      .some((link) => link.metadata().term === 'query-aggregate:count'),
  );
});

test('a second frontend can reuse the same canonical plan', () => {
  const fixture = fixtures.nonSqlFrontend;
  const sqlPlan = lowerSql(fixture.equivalentSql, 'sql-ansi');
  const registry = new QueryPlanRegistry();
  registry.register(fixture.language, {
    lower(_source, language) {
      return new QueryPlan(sqlPlan.operation(), SourceEvidence.synthetic(language));
    },
  });

  const linksPlan = registry.lower(fixture.source, fixture.language);
  assert.deepEqual(linksPlan.operation(), sqlPlan.operation());
  assert.equal(linksPlan.evidence().language, fixture.language);
});
