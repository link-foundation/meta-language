import test from 'node:test';
import assert from 'node:assert/strict';
import { readProjectData } from '../scripts/project-data.mjs';
import { LinkNetwork } from '../src/network.js';

const schema = readProjectData(new URL('../src/data/typed-program-links-schema.lino', import.meta.url));
const fieldLabel = (network, parent, label) => network.fieldRelations().find(relation => relation.parent.equals(parent) && relation.label === label)?.child;
const fieldKey = (network, parent, key) => fieldLabel(network, parent, schema.fields[key] ?? key.replace(/[A-Z]/gu, letter => ` ${letter.toLowerCase()}`));
const declaration = (network, body, name) => {
  const directory = fieldKey(network, body, 'declarations');
  return network.link(directory).references().map(id => network.link(id).references())
    .find(([key]) => network.link(key).metadata().definition === name)?.[1];
};

test('logical parameter keys and literal parameter labels retain distinct actual header containers', () => {
  const data = readProjectData(new URL('../../parity/fixtures/rust-source-header-emission.lino', import.meta.url));
  const entry = data.cases[1], network = LinkNetwork.fromLino(entry.links);
  const body = network.link(entry.root).references()[0];
  const owner = declaration(network, body, entry.functionName);
  assert.equal(schema.fields.params, 'parameters');
  assert.equal(schema.fields.parameters, 'source signature parameters');
  const parameters = fieldKey(network, owner, 'params');
  assert.ok(parameters.equals(fieldLabel(network, owner, 'parameters')));
  assert.equal(fieldKey(network, owner, 'parameters'), undefined, 'an already translated label cannot be translated a second time');
  const signature = fieldKey(network, owner, 'sourceSignature');
  const positions = fieldKey(network, signature, 'parameters');
  assert.ok(positions.equals(fieldLabel(network, signature, 'source signature parameters')));
  assert.equal(fieldLabel(network, signature, 'parameters'), undefined);
  assert.equal(parameters.equals(positions), false);
  assert.equal(network.link(parameters).references().length, 1);
  assert.equal(network.link(positions).references().length, 1);
  assert.equal(network.link(parameters).references()[0].equals(network.link(positions).references()[0]), false);
});

test('all preserved namespace container paths begin at the actual program body below the envelope', () => {
  const data = readProjectData(new URL('../../parity/fixtures/rust-type-namespace-containers.lino', import.meta.url));
  assert.equal(data.cases.length, 2);
  assert.equal(data.locations.length, 27);
  for (const entry of data.cases) {
    const network = LinkNetwork.fromLino(entry.archive), body = network.link(entry.root).references()[0];
    assert.equal(fieldKey(network, entry.root, 'declarations'), undefined);
    assert.ok(fieldKey(network, body, 'declarations'));
    for (const location of data.locations.filter(location => location.caseId === entry.id)) {
      let current = body;
      for (const segment of location.path) {
        const link = network.link(current);
        if (typeof segment === 'number') current = link.references()[segment];
        else if (link.metadata().term === schema.containers.map) {
          current = link.references().map(id => network.link(id).references())
            .find(([key]) => network.link(key).metadata().definition === segment)?.[1];
        } else current = fieldKey(network, current, segment);
        assert.ok(current, JSON.stringify(location));
      }
      assert.equal(network.link(current).metadata().term, schema.containers[location.kind], JSON.stringify(location));
    }
  }
});
