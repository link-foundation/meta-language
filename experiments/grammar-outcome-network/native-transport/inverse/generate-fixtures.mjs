// Reuse the complete existing JavaScript oracle author; retain every input.
import assert from 'node:assert/strict';
import {mkdtempSync,rmSync} from 'node:fs';
import {tmpdir} from 'node:os';
import {join} from 'node:path';
import {fileURLToPath} from 'node:url';
import {spawnSync} from 'node:child_process';
import {readProjectData,writeProjectData,checkProjectData} from '../../../../js/scripts/project-data.mjs';
import {checkGrammarContextMetadataPredecessor} from '../../../grammar-context-source-metadata/generate.mjs';

export function generateOwnedOutcomeInverseFixtures({check=false}={}) {
  const directory=mkdtempSync(join(tmpdir(),'meta-language-inverse-fixtures-'));
  const output=join(directory,'projection-fixtures.lino');
  const repository=fileURLToPath(new URL('../../../../',import.meta.url));
  const result=spawnSync(process.execPath,[fileURLToPath(new URL('../probe/oracle.mjs',import.meta.url)),
    '--repository',repository,'--output',output],{encoding:'utf8',timeout:45000,maxBuffer:1024*1024});
  assert.equal(result.signal,null,'oracle generation signal; retained scratch: '+directory);
  assert.equal(result.status,0,result.stderr+'; retained scratch: '+directory);
  const fixture=readProjectData(output);
  assert.equal(fixture.cases.length,14,'every original parser and constructed boundary input remains');
  const target=new URL('./projection-fixtures.lino',import.meta.url);
  if(check){
    // Keep the original oracle fixture immutable. Only the authenticated
    // producer metadata has a historical view; every full case stays exact.
    const previous=readProjectData(target);
    const historical=checkGrammarContextMetadataPredecessor(
      'experiments/grammar-outcome-network/native-transport/inverse/projection-fixtures.lino',fixture,previous);
    checkProjectData(target,historical);
  }else writeProjectData(target,fixture);
  rmSync(directory,{recursive:true});
  return fixture;
}

if(process.argv[1]===fileURLToPath(import.meta.url)){
  assert.ok(process.argv.length===3&&['--write','--check'].includes(process.argv[2]));
  generateOwnedOutcomeInverseFixtures({check:process.argv[2]==='--check'});
  console.log('All fourteen unchanged JavaScript oracle projections match the generated Links fixture.');
}
