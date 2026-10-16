// Expose the unchanged reviewed algorithm without editing a frozen producer.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';

const address=new URL('./product-rule-style-corrections.mjs',import.meta.url);
const source=readFileSync(address,'utf8');
assert.equal(createHash('sha256').update(source).digest('hex'),'9728ead881e57f16401321be9eb00e25416aab74cdb4d7164485cd0a3abbc216','the complete reviewed producer remains frozen');
const exported=source+'\nexport { simplifyBranches as simplifyRustBranches };\n';
const linked=exported.replace(/\bfrom (['"])(\.[^'"\n]+)\1/gu,(_,quote,path)=>'from '+quote+new URL(path,address).href+quote)
  .replaceAll('import.meta.url',JSON.stringify(address.href));
const reviewed=await import('data:text/javascript;base64,'+Buffer.from(linked).toString('base64'));

export const simplifyRustBranches=reviewed.simplifyRustBranches;
