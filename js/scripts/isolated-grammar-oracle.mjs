// Development oracle worker. Production parsing never invokes this process.
import { readFileSync } from 'node:fs';
import { oracleSnapshot } from './native-grammar-rows.mjs';
if (process.env.META_LANGUAGE_ORACLE_WORKER !== '1') throw new Error('oracle workers require explicit isolation');
const { source, language } = JSON.parse(readFileSync(0, 'utf8'));
if (typeof source !== 'string' || typeof language !== 'string') throw new TypeError('oracle input requires source and language strings');
// One parse in the isolated process shares no scanner allocator history
// with another fixture or a production native parse.
process.stdout.write(`${JSON.stringify(oracleSnapshot(source, language))}\n`);
