import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { test } from 'node:test';
import { readProjectData } from '../scripts/project-data.mjs';
import { selectVersionedAssertion } from '../scripts/assertion-version-ranges.mjs';
import { assertionVersionPolicy } from '../scripts/generate-assertion-version-policy.mjs';
import { selectUnicodeIdentifierAssertion } from '../scripts/unicode-identifier-assertions.mjs';

function boundedExpectation(id, minimum, maximum, expected, overrides = {}) {
  return { id, minimum, maximum, includeMinimum: true, includeMaximum: false,
    expected, evidence: 'independently specified boundary fixture', ...overrides };
}

function releaseAssertion(expectations, precision = 3) {
  return { id: 'dependency-observation', subject: 'example dependency',
    comparison: { kind: 'release', precision }, expectations };
}

test('version assertions preserve explicit inclusive and exclusive endpoints', () => {
  for (const includeMinimum of [false, true]) {
    for (const includeMaximum of [false, true]) {
      const policy = releaseAssertion([boundedExpectation('bounded', '1.2.3', '1.2.5', 'observed', { includeMinimum, includeMaximum })]);
      assert.equal(selectVersionedAssertion(policy, '1.2.4').expected, 'observed');
      for (const [version, admitted] of [['1.2.3', includeMinimum], ['1.2.5', includeMaximum]]) {
        if (admitted) assert.equal(selectVersionedAssertion(policy, version).id, 'bounded');
        else assert.throws(() => selectVersionedAssertion(policy, version), /uncovered active version/u);
      }
      for (const version of ['1.2.2', '1.2.6']) assert.throws(() => selectVersionedAssertion(policy, version), /uncovered active version/u);
    }
  }
  const exact = releaseAssertion([boundedExpectation('one release', '1.2.3', '1.2.3', 42, { includeMaximum: true })]);
  assert.equal(selectVersionedAssertion(exact, '1.2.3').expected, 42);
});

test('active dependency releases select adjacent expectations without priority or gaps', () => {
  const policy = releaseAssertion([
    boundedExpectation('old behavior', '1.0.0', '2.0.0', 'old'),
    boundedExpectation('new behavior', '2.0.0', '3.0.0', 'new'),
  ]);
  assert.equal(selectVersionedAssertion(policy, '1.99.99').expected, 'old');
  assert.equal(selectVersionedAssertion(policy, '2.0.0').expected, 'new');
  assert.equal(selectVersionedAssertion(policy, '2.99.99').expected, 'new');
  for (const version of ['0.99.99', '3.0.0']) {
    assert.throws(() => selectVersionedAssertion(policy, version), error => {
      assert.match(error.message, /dependency-observation \(example dependency /u);
      assert.equal(error.message.includes(version), true);
      assert.match(error.message, /old behavior \[1.0.0, 2.0.0\).*new behavior \[2.0.0, 3.0.0\)/u);
      return true;
    });
  }
  const gap = structuredClone(policy);
  gap.expectations[1].minimum = '2.1.0';
  assert.throws(() => selectVersionedAssertion(gap, '2.0.5'), /uncovered active version/u);
});

test('numeric coordinate comparison retains existing release ordering without decimal coercion', () => {
  for (const [precision, minimum, active, maximum] of [
    [1, '9', '10', '11'],
    [2, '17.2', '17.10', '17.11'],
    [3, '1.9.99', '1.10.0', '1.11.0'],
  ]) {
    const policy = releaseAssertion([boundedExpectation('coordinate ordering', minimum, maximum, active)], precision);
    assert.equal(selectVersionedAssertion(policy, active).expected, active);
  }
});

test('conflicting and duplicate overlapping ranges fail even when currently inactive', () => {
  for (const expected of ['old', 'conflicting']) {
    const policy = releaseAssertion([
      boundedExpectation('first', '1.0.0', '2.0.0', 'old'),
      boundedExpectation('second', '1.5.0', '3.0.0', expected),
    ]);
    for (const active of ['1.1.0', '1.7.0', '2.5.0', '5.0.0']) {
      assert.throws(() => selectVersionedAssertion(policy, active), /overlapping expectation ranges first .* and second/u);
    }
  }
  const sharedEndpoint = releaseAssertion([
    boundedExpectation('left', '1.0.0', '2.0.0', 'left', { includeMaximum: true }),
    boundedExpectation('right', '2.0.0', '3.0.0', 'right'),
  ]);
  assert.throws(() => selectVersionedAssertion(sharedEndpoint, '2.0.0'), /overlapping expectation ranges/u);
});

test('missing versions and malformed policies cannot become skipped or passing assertions', () => {
  const policy = releaseAssertion([boundedExpectation('valid', '1.0.0', '2.0.0', true)]);
  for (const active of [undefined, null, 1, '', 'v1.0.0', '1.0', '1.0.0-beta.1', '1.0.0+build', '01.0.0', '1.0.9007199254740992']) {
    assert.throws(() => selectVersionedAssertion(policy, active), /invalid 3-coordinate release/u);
  }
  const invalid = [
    { ...policy, comparison: { kind: 'semver' } },
    { ...policy, comparison: { kind: 'release', precision: 0 } },
    { ...policy, expectations: [] },
    { ...policy, expectations: [boundedExpectation('empty', '1.0.0', '1.0.0', true)] },
    { ...policy, expectations: [boundedExpectation('reversed', '2.0.0', '1.0.0', true)] },
    { ...policy, expectations: [boundedExpectation('missing maximum', '1.0.0', undefined, true)] },
    { ...policy, expectations: [boundedExpectation('missing expected', '1.0.0', '2.0.0', undefined)] },
    { ...policy, expectations: [boundedExpectation('missing evidence', '1.0.0', '2.0.0', true, { evidence: '' })] },
    { ...policy, expectations: [boundedExpectation('implicit endpoint', '1.0.0', '2.0.0', true, { includeMinimum: undefined })] },
    { ...policy, expectations: [policy.expectations[0], policy.expectations[0]] },
  ];
  for (const malformed of invalid) assert.throws(() => selectVersionedAssertion(malformed, '1.5.0'));
});

test('language editions use a declared order and the active Rust manifest edition', () => {
  const manifest = readFileSync(new URL('../../rust/Cargo.toml', import.meta.url), 'utf8');
  const edition = manifest.match(/^edition = "([^"]+)"$/mu)?.[1];
  assert.equal(typeof edition, 'string');
  const policy = {
    id: 'Rust edition assertion fixture', subject: 'Rust edition',
    comparison: { kind: 'ordered', versions: ['2015', '2018', '2021', '2024'] },
    expectations: [
      boundedExpectation('earlier editions', '2015', '2021', 'before 2021'),
      boundedExpectation('modern editions', '2021', '2024', '2021 or 2024', { includeMaximum: true }),
    ],
  };
  assert.equal(selectVersionedAssertion(policy, edition).expected, '2021 or 2024');
  assert.equal(selectVersionedAssertion(policy, '2018').expected, 'before 2021');
  for (const unknown of ['2022', '2027', '1.2024.0']) assert.throws(() => selectVersionedAssertion(policy, unknown), /unknown declared version/u);
  const duplicate = structuredClone(policy);
  duplicate.comparison.versions.push('2024');
  assert.throws(() => selectVersionedAssertion(duplicate, edition), /unique declared version order/u);
});

test('the actual pinned Unicode table selects its explicitly bounded unchanged expectation', () => {
  const stored = readProjectData(new URL('../../parity/assertion-version-ranges.lino', import.meta.url));
  assert.deepEqual(stored, assertionVersionPolicy(), 'the generated Links policy retains its authoring source');
  const properties = readProjectData(new URL('../src/data/unicode-identifier-properties.lino', import.meta.url));
  assert.equal(properties.unicode, '17.0');
  const selected = selectUnicodeIdentifierAssertion(properties.unicode);
  assert.equal(selected.expected, properties.unicode);
  assert.equal(selected.minimum, '17.0');
  assert.equal(selected.maximum, '17.1');
  assert.equal(selected.includeMinimum, true);
  assert.equal(selected.includeMaximum, false);
  for (const version of ['16.0', '17.1', '18.0']) assert.throws(() => selectUnicodeIdentifierAssertion(version), /uncovered active version/u);
  assert.throws(() => selectUnicodeIdentifierAssertion('17.0.0'), /invalid 2-coordinate release/u);
});

test('the active Node Unicode version either selects the real expectation or reports unsupported coverage', () => {
  const active = process.versions.unicode;
  assert.equal(typeof active, 'string');
  if (active === '17.0') {
    assert.equal(selectUnicodeIdentifierAssertion(active).expected, active);
  } else {
    assert.throws(() => selectUnicodeIdentifierAssertion(active), error => {
      assert.equal(error.message.includes(`Unicode ${active}`), true);
      assert.match(error.message, /uncovered active version/u);
      return true;
    });
  }
});
