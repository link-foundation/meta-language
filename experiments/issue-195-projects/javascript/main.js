import assert from 'node:assert/strict';
import { double, sumTo, fetchValue } from './util.js';
import * as shapes from './shapes/index.js';
import config from './config.json' with { type: 'json' };
import { html } from '#template';

export function quadruple(n) {
  return double(double(n));
}

export function isSquare(value) {
  return value instanceof shapes.Square;
}

export async function load() {
  return (await fetchValue()) + sumTo(config.depth);
}

export const page = html`<p>${quadruple(2)}</p>`;

assert.equal(quadruple(3), double(6));
console.assert(sumTo(3) === 6);
