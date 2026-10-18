// Apply only the already authenticated ring relocation hunk, never grammar edits.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync, writeFileSync} from 'node:fs';
import {resolve, join} from 'node:path';
import {fileURLToPath} from 'node:url';
import {gunzipSync} from 'node:zlib';
import {readProjectData} from './project-data.mjs';

const digest=value=>createHash('sha256').update(value).digest('hex');

export function readAgdaScannerPatchHunks(patch) {
  assert.ok(patch.startsWith('--- a/src/scanner.c\n+++ b/src/scanner.c\n'));
  const parts=patch.split(/^@@ .* @@.*\n/gmu).slice(1);
  assert.equal(parts.length,3,'the existing scanner patch has exactly its three original hunks');
  return parts.map(part=>{
    const lines=part.split('\n');if(lines.at(-1)==='')lines.pop();
    assert.ok(lines.every(line=>[' ','-','+'].includes(line[0])));
    return {before:lines.filter(line=>line[0]!=='+').map(line=>line.slice(1)).join('\n')+'\n',
      after:lines.filter(line=>line[0]!=='-').map(line=>line.slice(1)).join('\n')+'\n'};
  });
}

/** Authenticate both original and full successor before isolating one old hunk. */
export function prepareAgdaQueueSafeScanner(original,patch,manifest) {
  assert.equal(digest(original),manifest.originalScannerSha256,'the original scanner remains authenticated');
  assert.equal(digest(patch),manifest.files['layout-state.patch'],'the complete existing patch remains authenticated');
  const hunks=readAgdaScannerPatchHunks(patch);
  const replace=(source,hunk)=>{
    assert.equal(source.split(hunk.before).length,2,'an exact scanner hunk occurs once');
    return source.replace(hunk.before,()=>hunk.after);
  };
  const full=hunks.reduce(replace,original);
  assert.equal(digest(full),manifest.scannerSha256,'all existing scanner changes still reconstruct the original full successor');
  assert.match(hunks[0].before,/#define QUEUE_RESIZE\(queue, _cap\)/u);
  assert.match(hunks[0].after,/tmp\[index\] = \(queue\).data/u);
  const queue=replace(original,hunks[0]);
  assert.equal(queue.split(hunks[0].after).length,2);
  assert.equal(queue.replace(hunks[0].after,()=>hunks[0].before),original,'the single queue hunk restores every original source byte');
  for(const hunk of hunks.slice(1))assert.equal(queue.split(hunk.before).length,2,'serialization and deserialization retain their complete original bytes');
  return queue;
}

// This calls the actual scanner macros compiled in the same translation unit.
export const AGDA_QUEUE_GROWTH_CONTROL = '\nint meta_language_agda_queue_growth_control(void) {\n'
  +'    token_queue queue = token_queue_new();\n'
  +'    QUEUE_PUSH(queue, NEWLINE);\n'
  +'    QUEUE_POP(queue);\n'
  +'    QUEUE_PUSH(queue, DEDENT);\n'
  +'    QUEUE_PUSH(queue, NEWLINE);\n'
  +'    int valid = queue.tail - queue.head == 2 && QUEUE_FRONT(queue) == DEDENT;\n'
  +'    QUEUE_POP(queue);\n'
  +'    valid = valid && QUEUE_FRONT(queue) == NEWLINE;\n'
  +'    QUEUE_POP(queue);\n'
  +'    valid = valid && queue.tail == queue.head;\n'
  +'    free(queue.data);\n'
  +'    return valid;\n}\n';

if(process.argv[1]&&resolve(process.argv[1])===fileURLToPath(import.meta.url)) {
  const [vendor,output]=process.argv.slice(2);assert.ok(vendor&&output&&process.argv.length===4);
  const manifest=readProjectData(join(vendor,'source-manifest.lino'));
  const original=gunzipSync(readFileSync(join(vendor,'src/scanner.c.upstream.gz'))).toString('utf8');
  const patch=readFileSync(join(vendor,'layout-state.patch'),'utf8');
  writeFileSync(output,prepareAgdaQueueSafeScanner(original,patch,manifest)+AGDA_QUEUE_GROWTH_CONTROL);
}
