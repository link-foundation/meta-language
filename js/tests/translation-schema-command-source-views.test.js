import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {spawnSync} from 'node:child_process';
import {fileURLToPath} from 'node:url';
import test from 'node:test';
import {SCHEMA_COMMAND_SOURCE_PROFILE,readSchemaCommandSourceProfile,renderSchemaCommandSourceView,deriveSchemaCommandSourceView} from '../scripts/schema-command-source-profile.mjs';
import {generateSchemaCommandSourceProfile} from '../scripts/generate-schema-command-source-profile.mjs';
import {BULK_TRANSLATION_RULE_CHECKS,runBulkTranslationRuleChecks} from '../scripts/check-bulk-translation-rules.mjs';
const root=fileURLToPath(new URL('../../',import.meta.url));
const pin=bytes=>createHash('sha256').update(bytes).digest('hex');

test('schema source adapters retain both complete original commands certificates and every assertion through exact Links recipes',()=>{
  const packet=generateSchemaCommandSourceProfile({check:true});assert.equal(packet.sources.length,2);assert.equal(packet.gateSources.length,1);
  assert.equal(packet.gateSources[0].file,'js/scripts/check-bulk-translation-rules.mjs');
  for(const row of packet.sources){
    assert.equal(readFileSync(new URL(row.file,new URL('../../',import.meta.url)),'utf8'),row.source);
    const generated=renderSchemaCommandSourceView(row.file,row.source);assert.deepEqual(generated.receipt,row.view);
    let inverse=generated.source;
    for(const action of row.view.actions.toReversed())inverse=inverse.replace(action.after,action.before);
    assert.equal(inverse,row.source);assert.equal(row.view.actions.length,2);
    assert.deepEqual(generated.source.match(/\bassert\.[A-Za-z]+\(/gu),row.source.match(/\bassert\.[A-Za-z]+\(/gu));
    assert.throws(()=>renderSchemaCommandSourceView(row.file,row.source+'\n'));
    const forged=structuredClone(row.certificate);forged.path='js/scripts/counterfeit.mjs';
    assert.throws(()=>deriveSchemaCommandSourceView(row.file,row.source,forged));
    const returned=readSchemaCommandSourceProfile(row.file);returned.view.actions[0].after+=' changed';
    assert.deepEqual(readSchemaCommandSourceProfile(row.file),row);
  }
});

test('the exact two original freshness commands pass with authenticated schema input views and preserve every source schema receipt and gate',async()=>{
  const checks=BULK_TRANSLATION_RULE_CHECKS.filter(row=>SCHEMA_COMMAND_SOURCE_PROFILE.files.includes(row.script));
  assert.equal(checks.length,2);assert.equal(BULK_TRANSLATION_RULE_CHECKS.length,35);
  const files=[...SCHEMA_COMMAND_SOURCE_PROFILE.files,'js/src/data/typed-program-links-schema.lino','rust/src/data/typed-program-links-schema.lino',
    'experiments/original-link-complete-table-projection/raw-native-observations.mjs',
    ...['source','extension','second-extension','third-extension','fourth-extension','fifth-extension','sixth-extension'].map(stage=>'experiments/original-link-complete-table-projection/data/raw-schema-'+(stage==='source'?'source-closure':stage==='extension'?'extension-closure':stage+'-closure')+'.lino')];
  const before=files.map(file=>pin(readFileSync(new URL(file,new URL('../../',import.meta.url))))),calls=[];
  const result=await runBulkTranslationRuleChecks({checks,root,report:()=>{},invoke:async(command,args,options)=>{
    calls.push({command,args,options});
    const child=spawnSync(command,['--max-old-space-size=288','--max-semi-space-size=4',...args],{...options,encoding:'utf8',timeout:45000,maxBuffer:262144});
    assert.equal(child.error,undefined);assert.equal(child.signal,null);assert.equal(child.status,0,child.stderr+'\n'+child.stdout);
    assert.match(child.stderr,/authenticated current string-order model with original command/u);
    return{exitCode:child.status,signal:child.signal};
  }});
  assert.deepEqual(result,{checks:2,failures:[]});assert.equal(calls.length,2);
  for(let index=0;index<calls.length;index++){
    assert.equal(calls[index].command,process.execPath);
    assert.equal(calls[index].args[0],fileURLToPath(new URL(checks[index].script,new URL('../../',import.meta.url))));
    assert.deepEqual(calls[index].args.slice(1),['--check']);assert.match(calls[index].options.env.NODE_OPTIONS,/schema-command-source-loader\.mjs/u);
  }
  assert.deepEqual(files.map(file=>pin(readFileSync(new URL(file,new URL('../../',import.meta.url))))),before);
});

test('standalone immutable commands retain their strict live-successor refusals and the source loader refuses update arguments',()=>{
  for(const file of SCHEMA_COMMAND_SOURCE_PROFILE.files){
    const original=spawnSync(process.execPath,['--max-old-space-size=288',file,'--check'],{cwd:root,encoding:'utf8',timeout:15000,maxBuffer:262144});
    assert.equal(original.error,undefined);assert.equal(original.signal,null);assert.equal(original.status,1);assert.match(original.stderr,/145652 !== 153938/u);
    const refused=spawnSync(process.execPath,['--import',fileURLToPath(new URL('../scripts/schema-command-source-loader.mjs',import.meta.url)),file,'--write'],
      {cwd:root,encoding:'utf8',timeout:10000,maxBuffer:262144});
    assert.equal(refused.error,undefined);assert.equal(refused.signal,null);assert.equal(refused.status,1);assert.match(refused.stderr,/check-only/u);
  }
});
