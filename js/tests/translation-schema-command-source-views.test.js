import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {spawnSync} from 'node:child_process';
import {fileURLToPath} from 'node:url';
import test from 'node:test';
import {SCHEMA_COMMAND_SOURCE_PROFILE,readSchemaCommandSourceProfile,renderSchemaCommandSourceView,deriveSchemaCommandSourceView} from '../scripts/schema-command-source-profile.mjs';
import {generateSchemaCommandSourceProfile} from '../scripts/generate-schema-command-source-profile.mjs';
import {readStructuredDataLino} from '../src/structured-data-links.js';
import {BULK_TRANSLATION_RULE_CHECKS,runBulkTranslationRuleChecks} from '../scripts/check-bulk-translation-rules.mjs';
import {satisfies} from '../scripts/semver-range.mjs';
import {generateCiRuntimeSourceProfiles} from '../../experiments/ci-runtime-source-profiles/generate.mjs';
const root=fileURLToPath(new URL('../../',import.meta.url));
const pin=bytes=>createHash('sha256').update(bytes).digest('hex');

test('schema source loaders admit the declared synchronous hook runtime ranges and expose uncovered versions',()=>{
  for(const[version,expected]of [['22.14.9',false],['22.15.0',true],['22.22.0',true],['23.0.0',false],['24.0.0',true],['24.18.0',true],['24.22.0',true],['25.0.0',false]]){
    assert.equal(satisfies(version,SCHEMA_COMMAND_SOURCE_PROFILE.nodeRange,'npm'),expected,version);
  }
  assert.equal(satisfies(process.versions.node,SCHEMA_COMMAND_SOURCE_PROFILE.nodeRange,'npm'),true,'the active runtime must be covered');
  assert.match(readFileSync(new URL('../scripts/schema-command-source-loader.mjs',import.meta.url),'utf8'),/uncovered schema command source-loader Node\.js version/u);
  generateCiRuntimeSourceProfiles({check:true});
});

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
  assert.equal(satisfies(process.versions.node,SCHEMA_COMMAND_SOURCE_PROFILE.nodeRange,'npm'),true,'uncovered schema refusal runtime '+process.versions.node);
  const schemaFile=new URL('../src/data/typed-program-links-schema.lino',import.meta.url);
  const sixthFile=new URL('../../experiments/original-link-complete-table-projection/data/raw-schema-sixth-extension-closure.lino',import.meta.url);
  const oracleFile=new URL('../../experiments/original-link-complete-table-projection/raw-native-observations.mjs',import.meta.url);
  const schemaBytes=readFileSync(schemaFile),sixthBytes=readFileSync(sixthFile),oracleBytes=readFileSync(oracleFile);
  const pins=[...oracleBytes.toString('utf8').matchAll(/^const schemaSixthExtensionReceiptSha256 = '([0-9a-f]{64})';$/gmu)];
  assert.equal(pins.length,1);assert.equal(pin(sixthBytes),pins[0][1],'the complete immutable sixth receipt retains its independent oracle pin');
  const sixth=readStructuredDataLino(sixthBytes.toString('utf8'));
  assert.equal(sixth.current.file,'js/src/data/typed-program-links-schema.lino');
  assert.ok(Number.isSafeInteger(sixth.current.bytes)&&sixth.current.bytes>0);
  assert.notEqual(sixth.current.bytes,schemaBytes.length,'this refusal profile requires an actual later schema with a different byte count');
  // The old153938 expectation belonged to the collection-size predecessor.
  // These commands first compare the immutable sixth145652 checkpoint with
  // the actual live successor (155087 at the observed899de CI revision).
  const expectedBoundary=sixth.current.bytes+' !== '+schemaBytes.length;
  for(const file of SCHEMA_COMMAND_SOURCE_PROFILE.files){
    const original=spawnSync(process.execPath,['--max-old-space-size=288',file,'--check'],{cwd:root,encoding:'utf8',timeout:15000,maxBuffer:262144});
    assert.equal(original.error,undefined);assert.equal(original.signal,null);assert.equal(original.status,1);
    assert.equal(original.stderr.split(/\r?\n/u).map(line=>line.trim()).filter(line=>line===expectedBoundary).length,1,'the exact authenticated-sixth versus actual-live numeric assertion is retained');
    assert.ok(original.stderr.includes('at verifyOriginalLinkRawSchemaSixthExtensionClosure '));
    assert.ok(original.stderr.includes('/'+file+':'),'the failure belongs to the same complete original command');
    const refused=spawnSync(process.execPath,['--import',fileURLToPath(new URL('../scripts/schema-command-source-loader.mjs',import.meta.url)),file,'--write'],
      {cwd:root,encoding:'utf8',timeout:10000,maxBuffer:262144});
    assert.equal(refused.error,undefined);assert.equal(refused.signal,null);assert.equal(refused.status,1);assert.match(refused.stderr,/check-only/u);
  }
  assert.equal(readFileSync(schemaFile).equals(schemaBytes),true);assert.equal(readFileSync(sixthFile).equals(sixthBytes),true);assert.equal(readFileSync(oracleFile).equals(oracleBytes),true);
});
