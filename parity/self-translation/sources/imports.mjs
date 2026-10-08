// Relative imports of the items of other modules of the crate, beside the
// imports the portable core refuses.
import { double, GREETING as greeting } from './math.mjs';
import { tree } from '../shapes/tree.mjs';
import fs from 'node:fs';
import { parse } from 'links-notation';
import * as geometry from './geometry.mjs';
import table from './table.mjs';

/** @param {number} x @returns {number} */
export function quadruple(x) {
  return double(double(x));
}
