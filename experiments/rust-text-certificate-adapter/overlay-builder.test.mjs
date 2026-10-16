import assert from 'node:assert/strict';
import test from 'node:test';
import {relocateCheckedModuleImports} from './overlay.mjs';
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {readProjectData} from '../../js/scripts/project-data.mjs';
import {readReviewedOverlayProviderSources,reviewedOverlayProviderPolicy} from './reviewed-providers.mjs';
import {createCheckedModuleOverlay} from './overlay.mjs';

test('lexical module relocation preserves active imports and resource bases without changing inert expressions',()=>{
 const source='import value from "./provider.js";\nexport {value} from "./export.js";\nexport const data=new URL("./data.lino",import.meta.url);\nconst inert="import.meta.url";\nfrom\n"./inert.js";\nconst property=object.import.meta.url;\nconst lazy=import("./lazy.js");';
 const origin='file:///original/module.js',linked=relocateCheckedModuleImports(source,origin,url=>url==='file:///original/provider.js'?'file:///overlay/provider.js':url);
 assert.ok(linked.includes('from "file:///overlay/provider.js"'));assert.ok(linked.includes('new URL("./data.lino","file:///original/module.js")'));assert.ok(linked.includes('inert="import.meta.url"'));
 assert.ok(linked.includes('from "file:///original/export.js"'));assert.ok(linked.includes('from\n"./inert.js"'));assert.ok(linked.includes('object.import.meta.url'));assert.ok(linked.includes('import("file:///original/lazy.js")'));
});

test('reviewed provider revisions preserve historical receipts and refuse every unreviewed byte change',async()=>{
 const modules=reviewedOverlayProviderPolicy.revisions[0].providers.map(row=>row.module),sourceRoot=new URL('../../js/src/',import.meta.url);
 for(const historical of reviewedOverlayProviderPolicy.historicalOverlays){
  const file=new URL('../'+historical.experiment+'/data/overlays.lino',import.meta.url),before=readFileSync(file);
  assert.equal(createHash('sha256').update(before).digest('hex'),historical.sourceSha256);
  const selected=readReviewedOverlayProviderSources({experiment:historical.experiment,modules});
  assert.ok(reviewedOverlayProviderPolicy.revisions.includes(selected.revision));
  for(const changed of modules)assert.throws(()=>readReviewedOverlayProviderSources({experiment:historical.experiment,modules,
   readSource:module=>readFileSync(new URL(module,sourceRoot),'utf8')+(module===changed?'\n':'')}),/no unique reviewed revision/u,changed);
  if(selected.revision!==reviewedOverlayProviderPolicy.revisions[0])await assert.rejects(createCheckedModuleOverlay({records:readProjectData(file)}),/exact overlay provider changed/u);
  assert.deepEqual(readFileSync(file),before);
 }
});
