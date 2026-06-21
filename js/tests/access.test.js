import assert from 'node:assert/strict';
import { test } from 'node:test';

import {
  AccessMode,
  EngineNetwork,
  ReadOnlyNetwork,
  ReadOnlyViolation,
  accessModeIsMutable,
  accessModeIsReadOnly,
  accessModeLabel,
  asReadOnly,
  freeze,
  parseEngine,
} from '../src/access.js';
import { LinkMetadata, LinkType, ParseConfiguration } from '../src/primitives.js';
import { LinkNetwork } from '../src/network.js';

test('parse configuration defaults to mutable access', () => {
  assert.equal(ParseConfiguration.default().accessMode, AccessMode.Mutable);
  assert.ok(accessModeIsMutable(AccessMode.Mutable));
  assert.ok(!accessModeIsReadOnly(AccessMode.Mutable));

  const readOnly = ParseConfiguration.default().withAccessMode(AccessMode.ReadOnly);
  assert.equal(readOnly.accessMode, AccessMode.ReadOnly);
  assert.ok(accessModeIsReadOnly(readOnly.accessMode));
  assert.equal(accessModeLabel(readOnly.accessMode), 'read-only');
});

test('frozen view supports non-mutating operations', () => {
  const network = LinkNetwork.parse('(a b)', 'plain-text', ParseConfiguration.default());
  const expectedText = network.reconstructText();
  const expectedLen = network.len();
  const view = freeze(network);

  assert.equal(view.reconstructText(), expectedText);
  assert.equal(view.len(), expectedLen);
  assert.equal(view.links().length, expectedLen);
  assert.ok(view.verifyFullMatch().isClean());
  assert.ok(view.findTerm('a') !== undefined);
});

test('frozen view rejects every mutation with ReadOnlyViolation', () => {
  const view = LinkNetwork.parse('alpha', 'plain-text', ParseConfiguration.default());
  const frozen = freeze(view);

  assert.throws(() => frozen.insertLink([], LinkMetadata.new()), ReadOnlyViolation);
  assert.throws(() => frozen.deleteLink(1), ReadOnlyViolation);
  assert.throws(() => frozen.setTerm(1, 'x'), ReadOnlyViolation);
  assert.throws(() => frozen.applySubstitution({}), ReadOnlyViolation);
  assert.throws(() => frozen.replace([], {}), ReadOnlyViolation);
});

test('ReadOnlyViolation carries a clear diagnostic', () => {
  const frozen = freeze(LinkNetwork.parse('alpha', 'plain-text', ParseConfiguration.default()));
  assert.throws(
    () => frozen.insertLink(),
    (error) => {
      assert.ok(error instanceof ReadOnlyViolation);
      assert.ok(error.message.includes('read-only'));
      assert.equal(error.name, 'ReadOnlyViolation');
      assert.ok(error.toString().includes('read-only'));
      return true;
    },
  );
});

test('frozen view can fork back to a mutable network', () => {
  const network = LinkNetwork.parse('alpha', 'plain-text', ParseConfiguration.default());
  const view = freeze(network);

  const editable = view.toMutable();
  const added = editable.insertLink([], LinkMetadata.new().withLinkType(LinkType.Concept));
  assert.ok(editable.link(added) !== undefined);
  // The original frozen view is unaffected by edits to the fork.
  assert.ok(view.link(added) === undefined);

  const recovered = view.intoMutable();
  assert.equal(recovered.reconstructText(), 'alpha');
});

test('asReadOnly clones, leaving the source network mutable', () => {
  const network = LinkNetwork.parse('alpha', 'plain-text', ParseConfiguration.default());
  const view = asReadOnly(network);

  assert.equal(view.reconstructText(), 'alpha');
  // Mutating the source does not affect the read-only clone.
  network.insertLink([], LinkMetadata.new().withLinkType(LinkType.Concept));
  assert.equal(view.len() + 1, network.len());
});

test('read-only views compare structurally', () => {
  const first = asReadOnly(LinkNetwork.parse('alpha', 'plain-text', ParseConfiguration.default()));
  const second = asReadOnly(LinkNetwork.parse('alpha', 'plain-text', ParseConfiguration.default()));
  const other = asReadOnly(LinkNetwork.parse('beta', 'plain-text', ParseConfiguration.default()));

  assert.ok(first.equals(second));
  assert.ok(!first.equals(other));
});

test('parseEngine returns read-only form under read-only mode', () => {
  const configuration = ParseConfiguration.default().withAccessMode(AccessMode.ReadOnly);
  const engine = parseEngine('alpha beta', 'plain-text', configuration);

  assert.ok(engine.isReadOnly());
  assert.ok(!engine.isMutable());
  assert.equal(engine.accessMode(), AccessMode.ReadOnly);
  assert.equal(engine.network().reconstructText(), 'alpha beta');

  assert.throws(
    () => engine.asMutable(),
    (error) => {
      assert.ok(error instanceof ReadOnlyViolation);
      assert.ok(error.message.includes('read-only'));
      return true;
    },
  );
});

test('parseEngine returns mutable form by default', () => {
  const engine = parseEngine('alpha', 'plain-text', ParseConfiguration.default());

  assert.ok(engine.isMutable());
  assert.equal(engine.accessMode(), AccessMode.Mutable);

  const editable = engine.asMutable();
  const added = editable.insertLink([], LinkMetadata.new().withLinkType(LinkType.Concept));
  assert.ok(engine.network().link(added) !== undefined);
});

test('EngineNetwork round-trips between modes', () => {
  const mutable = EngineNetwork.withAccessMode(
    LinkNetwork.parse('alpha', 'plain-text', ParseConfiguration.default()),
    AccessMode.Mutable,
  );
  const frozen = mutable.intoReadOnly();
  assert.ok(frozen instanceof ReadOnlyNetwork);
  assert.equal(frozen.reconstructText(), 'alpha');

  const editable = EngineNetwork.readOnly(frozen).intoMutable();
  assert.ok(editable instanceof LinkNetwork);
  assert.equal(editable.reconstructText(), 'alpha');
});

test('EngineNetwork.network exposes read operations in both modes', () => {
  const mutable = EngineNetwork.mutable(
    LinkNetwork.parse('gamma', 'plain-text', ParseConfiguration.default()),
  );
  const readOnly = EngineNetwork.readOnly(
    LinkNetwork.parse('gamma', 'plain-text', ParseConfiguration.default()),
  );

  assert.equal(mutable.network().reconstructText(), 'gamma');
  assert.equal(readOnly.network().reconstructText(), 'gamma');
});
