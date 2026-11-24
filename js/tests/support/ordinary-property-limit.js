import assert from 'node:assert/strict';
import {
  createJavaScriptOrdinarySetDescriptor,
  defineJavaScriptOrdinaryProperty,
  readJavaScriptOrdinaryHeapError,
} from '../../src/translation/javascript-ordinary-heap.js';

function slotKey(index) {
  return { $: 'text', units: Array.from('slot' + index, character => character.charCodeAt(0)) };
}

export function fillOrdinaryPropertyLimit(heap, identity) {
  const number = value => ({ $: 'number', value });
  const first = defineJavaScriptOrdinaryProperty(heap, identity, slotKey(0),
    createJavaScriptOrdinarySetDescriptor(number(0), true));
  assert.equal(first.success, true);
  const target = first.heap.objects.find(object => object.identity === identity);
  assert.equal(target.properties.length, 1);
  const [slot] = target.properties;

  // Validate the complete boundary through the public heap validator once.
  // Building it with sequential writes revalidates every growing prefix.
  const properties = Array.from({ length: 4096 }, (_, index) => ({
    ...slot,
    key: slotKey(index),
    descriptor: { ...slot.descriptor, value: number(index) },
  }));
  const boundary = {
    ...first.heap,
    objects: first.heap.objects.map(object => object === target ? { ...target, properties } : object),
  };
  assert.equal(readJavaScriptOrdinaryHeapError(boundary), '');
  return boundary;
}
