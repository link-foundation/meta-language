import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {readAgdaScannerPatchHunks,prepareAgdaQueueSafeScanner,AGDA_QUEUE_GROWTH_CONTROL} from '../scripts/agda-queue-safe-oracle.mjs';

const patch=readFileSync(new URL('../../rust/vendor/tree-sitter-agda-layout/layout-state.patch',import.meta.url),'utf8');
const hunks=readAgdaScannerPatchHunks(patch);
const digest=value=>createHash('sha256').update(value).digest('hex');
// Project-authored assembly of the existing patch contexts, never an upstream
// grammar download, scanner execution or corpus experiment.
const original=hunks.map((hunk,index)=>'/* context '+index+' */\n'+hunk.before).join('\n');
const full=hunks.reduce((source,hunk)=>source.replace(hunk.before,()=>hunk.after),original);
const manifest={originalScannerSha256:digest(original),scannerSha256:digest(full),files:{'layout-state.patch':digest(patch)}};

test('Agda queue-only source derivation preserves the complete original serialization and full patched successor',()=>{
  const queue=prepareAgdaQueueSafeScanner(original,patch,manifest);
  assert.equal(queue,original.replace(hunks[0].before,()=>hunks[0].after));
  assert.equal(queue.replace(hunks[0].after,()=>hunks[0].before),original);
  for(const hunk of hunks.slice(1))assert.ok(queue.includes(hunk.before));
  assert.equal(hunks.slice(1).reduce((source,hunk)=>source.replace(hunk.before,()=>hunk.after),queue),full);
  assert.match(AGDA_QUEUE_GROWTH_CONTROL,/token_queue queue = token_queue_new\(\)/u);
  assert.match(AGDA_QUEUE_GROWTH_CONTROL,/QUEUE_POP\(queue\);\n    QUEUE_PUSH\(queue, DEDENT\);\n    QUEUE_PUSH\(queue, NEWLINE\);/u);
  assert.match(AGDA_QUEUE_GROWTH_CONTROL,/free\(queue.data\)/u);
});

test('Agda queue-only source derivation rejects changed missing duplicated or unrelated source edits',()=>{
  for(const source of [original+'/* unrelated edit */\n',original.replace(hunks[0].before,''),original+hunks[0].before])
    assert.throws(()=>prepareAgdaQueueSafeScanner(source,patch,manifest),/original scanner remains authenticated/u);
  assert.throws(()=>prepareAgdaQueueSafeScanner(original,patch+'\n',manifest),/complete existing patch remains authenticated/u);
  const duplicate=original+hunks[0].before;
  assert.throws(()=>prepareAgdaQueueSafeScanner(duplicate,patch,{...manifest,originalScannerSha256:digest(duplicate)}),/hunk occurs once/u);
  const missing=original.replace(hunks[0].before,'');
  assert.throws(()=>prepareAgdaQueueSafeScanner(missing,patch,{...manifest,originalScannerSha256:digest(missing)}),/hunk occurs once/u);
  const wrong={...manifest,scannerSha256:'0'.repeat(64)};
  assert.throws(()=>prepareAgdaQueueSafeScanner(original,patch,wrong),/full successor/u);
  assert.throws(()=>readAgdaScannerPatchHunks(patch.replace(/^@@ .* @@.*\n[\s\S]*?(?=^@@)/mu,'')),/three original hunks/u);
});
