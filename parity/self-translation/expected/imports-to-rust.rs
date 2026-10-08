// meta-language:self-translation:v1 source=JavaScript target=Rust sha256=ef6b343e2ae222c83242439b267cc4ff6c58fa310fec1e1df18b8d4bd875c10c bytes=463

// meta-language:prelude begin
#![allow(unused, unreachable_patterns, non_snake_case, non_camel_case_types, invalid_nan_comparisons)]
// meta-language:prelude end

// meta-language:translated JavaScript import_statement items=1 sha256=0fe453531ac7a10a9324e166ea21b065e1b99d02a58abe738bf2d7a0ed5566a8
// | // Relative imports of the items of other modules of the crate, beside the
// | // imports the portable core refuses.
// | import { double, GREETING as greeting } from './math.mjs';
use crate::math::{double, GREETING as greeting};
// meta-language:carried JavaScript import_statement (unsupported)
// | import { tree } from '../shapes/tree.mjs';
// meta-language:carried JavaScript import_statement (unsupported)
// | import fs from 'node:fs';
// meta-language:carried JavaScript import_statement (unsupported)
// | import { parse } from 'links-notation';
// meta-language:carried JavaScript import_statement (unsupported)
// | import * as geometry from './geometry.mjs';
// meta-language:carried JavaScript import_statement (unsupported)
// | import table from './table.mjs';

// meta-language:carried JavaScript export_statement (type)
// | /** @param {number} x @returns {number} */
// | export function quadruple(x) {
// |   return double(double(x));
// | }
