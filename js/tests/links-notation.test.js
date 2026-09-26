import assert from 'node:assert/strict';
import { test } from 'node:test';

import { LinkNetwork, LinkType } from '../src/index.js';

const key = (id) => id.asU64();

function namedRelation(network, term) {
  const id = network.findTerm(term);
  assert.ok(id, `named relation term ${term} exists`);
  assert.equal(network.link(id).metadata().linkType, LinkType.Relation);
  return id;
}

function assertRelationReferences(network, terms) {
  const references = terms.map((term) => {
    const id = network.findTerm(term);
    assert.ok(id, `term ${term} exists`);
    return key(id);
  });
  assert.ok(
    network.links().some((link) =>
      link.metadata().linkType === LinkType.Relation &&
      JSON.stringify(link.references().map(key)) === JSON.stringify(references)),
    `missing LiNo relation with references ${JSON.stringify(terms)}`,
  );
}

function parseLino(source) {
  const network = LinkNetwork.parse(source, 'LiNo');
  assert.equal(network.reconstructText(), source);
  assert.ok(network.verifyFullMatch().isClean());
  return network;
}

test('LiNo doublet, triplet and tuple sources emit Relation links', () => {
  const network = parseLino('(papa mama)\n(papa loves mama)\n');

  assertRelationReferences(network, ['papa', 'mama']);
  assertRelationReferences(network, ['papa', 'loves', 'mama']);
});

test('LiNo named links are reused by later references and self references', () => {
  const network = parseLino('(papa (lovesMama: loves mama))\n(son lovesMama)\n(obj_0: list obj_0)\n');

  assertRelationReferences(network, ['son', 'lovesMama']);
  assert.deepEqual(
    network.link(namedRelation(network, 'lovesMama')).references().map(key),
    [key(network.findTerm('loves')), key(network.findTerm('mama'))],
  );

  const obj = namedRelation(network, 'obj_0');
  assert.ok(
    network.link(obj).references().map(key).includes(key(obj)),
    'named LiNo relation should be able to reference itself',
  );
});

test('LiNo indented id syntax emits a named Relation link', () => {
  const network = parseLino('greeting:\n  hello\n');

  assertRelationReferences(network, ['hello']);
  assert.equal(network.link(namedRelation(network, 'greeting')).metadata().term, 'greeting');
});
