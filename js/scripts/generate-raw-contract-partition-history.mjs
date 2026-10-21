// Capture the exact source and workflow split after the observed cold timeout.
import assert from 'node:assert/strict';
import {fileURLToPath} from 'node:url';
import {captureGitArtifactCertificate} from './capture-git-artifact-certificate.mjs';
import {checkProjectData,writeProjectData} from './project-data.mjs';
import {parseWorkflow} from '../../scripts/check-cache-policy.mjs';

export function generateRawContractPartitionHistory({check=false}={}) {
  const revision='178aa1e7882183c536f1d10c6c40c54f2e06a924';
  const capture=path=>{const {certificate,blob}=captureGitArtifactCertificate(revision,path);return{certificate,source:blob.toString('utf8')};};
  const workflow=capture('.github/workflows/js.yml');
  const source=capture('experiments/original-link-complete-table-projection/raw-native-observations.test.mjs');
  const suites=[
    {group:'raw-emission-profiles',name:'Run complete raw emission profile contracts',file:'raw-emission-profiles.test.mjs'},
    {group:'raw-consuming-emission',name:'Run complete raw consuming emission contracts',file:'raw-consuming-emission.test.mjs'},
    {group:'raw-borrowed-emission',name:'Run complete raw borrowed emission contracts',file:'raw-borrowed-emission.test.mjs'},
  ];
  const job=parseWorkflow(workflow.source).find(row=>row.id==='package-checks');assert.ok(job);
  const step=job.steps.find(row=>row.name==='Run complete original link raw native oracle contracts');assert.ok(step);
  assert.ok(step.text.includes("ORIGINAL_LINK_RAW_NATIVE: '0'"));
  const matrix="          - group: source-model-raw\n            generated_slice: first\n";
  assert.equal(workflow.source.split(matrix).length,2);
  const additions=suites.map(row=>"          - group: "+row.group+"\n            generated_slice: first\n").join('');
  const isolated=step.text.replace("          ORIGINAL_LINK_RAW_NATIVE: '0'","          ORIGINAL_LINK_RAW_NATIVE: '0'\n          ORIGINAL_LINK_RAW_SEPARATE_SUITES: '1'");
  const separate=suites.map(row=>step.text.replace(step.name,row.name)
    .replace("matrix.group == 'source-model-raw'","matrix.group == '"+row.group+"'")
    .replace('raw-native-observations.test.mjs',row.file)).join('');
  const imports=suites.map(row=>"import './"+row.file+"';\n").join('');
  assert.equal(source.source.split(imports).length,2);
  const separateImports="const separateRawContractSuites=process.env.ORIGINAL_LINK_RAW_SEPARATE_SUITES;\n"
    +"if(separateRawContractSuites!==undefined){\n  assert.equal(separateRawContractSuites,'1');\n  assert.equal(process.env.CI,'true');\n  assert.equal(process.env.ORIGINAL_LINK_RAW_NATIVE,'0');\n}else{\n"
    +suites.map(row=>"  await import('./"+row.file+"');\n").join('')+"}\n";
  const policy={schemaVersion:1,revision,workflow,source,suites,
    changes:[{before:matrix,after:matrix+additions},{before:step.text,after:isolated+separate}],
    testChange:{before:imports,after:separateImports},
    boundary:'The measured job exceeded its unchanged ten-minute deadline while source planning and imported whole-emission tests allocated concurrently. Three complete imported suites run in distinct required JavaScript cells with the same commands, limits, flags and failure predicates. Only the explicit CI JavaScript-only selection omits duplicate imports; default and native invocation retain all imports. All assertions and source inputs remain exact. Full workload execution stays in CI.'};
  const file=new URL('../../parity/fixtures/raw-contract-partition-history.lino',import.meta.url);
  if(check)checkProjectData(file,policy);else writeProjectData(file,policy);
  return policy;
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
  assert.ok(process.argv.slice(2).every(value=>value==='--check'));
  generateRawContractPartitionHistory({check:process.argv.includes('--check')});
  console.log('Captured exact whole raw-contract source and three required isolated suite cells.');
}
