import { readFileSync, writeFileSync } from 'node:fs';
import { translateProgram } from '../../js/src/index.js';
const [file, from, to, out] = process.argv.slice(2);
const t = translateProgram(readFileSync(file, 'utf8'), from, to);
writeFileSync(out, t.code);
console.log(t.contract.support, JSON.stringify(t.semantics.assumptions?.map(a=>a.id)));
