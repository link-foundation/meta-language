// Project two complete already certified sources; retain the driver predecessor.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync,writeFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {verifyGitArtifactCertificate} from './git-object-certificate.mjs';
import {readBootstrapStructuredDataLino} from '../../scripts/lib/structured-data-bootstrap.mjs';
import {captureGitArtifactCertificate} from './capture-git-artifact-certificate.mjs';
import {readOwnCodebaseHistoricalNamingPolicy,verifyOwnCodebaseHistoricalNamingSources} from './own-codebase-baseline-naming-history.mjs';
import {checkProjectData,writeProjectData} from './project-data.mjs';
const root=new URL('../../',import.meta.url);
const pin=bytes=>({bytes:Buffer.byteLength(bytes),sha256:createHash('sha256').update(bytes).digest('hex')});

/** Reproduce the exact compact offline history and the additive driver. */
export function generateOwnCodebaseHistoricalNamingSources({check=false}={}){
  const policy=readOwnCodebaseHistoricalNamingPolicy(),archiveBytes=readFileSync(new URL(policy.sourceArchive.path,root));
  assert.deepEqual(pin(archiveBytes),{bytes:policy.sourceArchive.bytes,sha256:policy.sourceArchive.sha256});
  let archive=readBootstrapStructuredDataLino(archiveBytes.toString('utf8'));
  assert.equal(archive.revision,policy.revision);assert.equal(archive.sources.length,policy.sourceArchive.sources);
  const views=policy.views.map(view=>{
    const selected=archive.sources.filter(row=>row.file===view.file);assert.equal(selected.length,1);const row=selected[0];
    assert.equal(row.commit,policy.revision);
    assert.deepEqual(pin(readFileSync(new URL(view.file,root))),view.current);
    return {...view,predecessor:{source:row.sourceText,bytes:row.bytes,sha256:row.sha256,certificate:structuredClone(row.certificate)}};
  });
  archive=null;global.gc?.();
  const original=captureGitArtifactCertificate(policy.revision,policy.driver.file);
  assert.deepEqual(pin(original.blob),policy.driver.original);
  const history={schemaVersion:policy.schemaVersion,model:policy.model,revision:policy.revision,sourceArchive:policy.sourceArchive,
    views,driver:{...policy.driver,source:original.blob.toString('utf8'),certificate:original.certificate}};
  verifyOwnCodebaseHistoricalNamingSources(history);
  let current=history.driver.source;
  for(const edit of history.driver.edits){assert.equal(current.split(edit.before).length,2);current=current.replace(edit.before,()=>edit.after);}
  const driver=new URL(policy.driver.file,root),actual=readFileSync(driver,'utf8');
  assert.ok(actual===history.driver.source||actual===current,'an unknown driver cannot be replaced or certified');
  const file=new URL('parity/naming/own-codebase-baseline-naming-history.lino',root);
  if(check){checkProjectData(file,history);assert.equal(actual,current);}
  else{writeProjectData(file,history);writeFileSync(driver,current);}
  return history;
}
/** Append only independently derived historical names; current extraction stays separate. */
export async function generateAdditionalOwnCodebaseHistoricalNamingSources({check=false}={}){
  const revision='6ffff3b2dc530a5f24c128dadf877f3fa106932b';
  const integration='ec9f4efb0e9ef936114e09823981352f6db13700';
  const integrationFiles=[
    'experiments/binary-metadata-buffer/native-observations.mjs',
    'experiments/grammar-outcome-network/native-transport/inverse/generate-publication-runner.mjs',
    'experiments/original-link-complete-table-projection/raw-consuming-emission.mjs',
    'experiments/original-link-complete-table-projection/raw-consuming-emission.test.mjs',
    'experiments/original-link-complete-table-projection/raw-emission-producer-applicability.mjs',
    'experiments/original-link-complete-table-projection/raw-native-observations.mjs',
    'js/scripts/current-ci-repair-source-history.mjs',
    'js/scripts/current-native-host-source-history.mjs',
    'js/scripts/generate-continuation-historical-inputs.mjs',
    'js/scripts/generate-current-ci-repair-source-history.mjs',
    'js/scripts/generate-modern-javascript-array-parity-fixtures.mjs',
    'js/scripts/generate-own-codebase-baseline-naming-history.mjs',
    'js/scripts/generate-own-codebase-test-registration-sources.mjs',
    'js/scripts/generate-raw-oracle-contract-partitions.mjs',
    'js/scripts/generate-tooling-frozen-source-history.mjs',
    'js/scripts/own-codebase-baseline-naming-history.mjs',
    'js/scripts/own-codebase-test-registration-history.mjs',
    'js/scripts/raw-consuming-snapshot-source-profile.mjs',
    'js/scripts/raw-oracle-contract-partition-history.mjs',
    'js/scripts/reference-identity-source-history.mjs',
    'js/scripts/required-field-native-host-succession.mjs',
    'js/scripts/rust-test-shards.mjs',
    'js/scripts/test-groups.mjs',
    'js/scripts/tooling-frozen-source-history.mjs',
    'js/src/translation/emit-javascript.js',
    'js/src/translation/emit-rust.js',
    'js/src/typed-program-links-validation.js',
    'js/tests/issue-195-evidence-node-dependency.test.js',
    'js/tests/issue-195-evidence-node-test-placement.test.js',
    'js/tests/issue-195-evidence-tooling-frozen-sources.test.js',
    'js/tests/issue-195-evidence-translation-partition-conservation.test.js',
    'js/tests/project-data-fixture-links-storage.test.js',
    'js/tests/self-translation.test.js',
    'js/tests/support/translation-workflow-history.js',
    'js/tests/test-groups.test.js',
    'js/tests/translation-arrays.test.js',
    'js/tests/translation-native-number-retention.test.js',
    'js/tests/translation-reference-identity.test.js',
    'js/tests/translation-rust-literal-matches.test.js',
    'js/tests/translation-typescript-workflow.test.js',
    'rust/src/self_translation/binding/owned_recursive_emission.rs',
    'rust/src/translation/check/records.rs',
    'rust/src/translation/emit_javascript/expressions.rs',
    'rust/src/translation/emit_lean/arrays.rs',
    'rust/src/translation/emit_rust/arrays.rs',
    'rust/src/translation/emit_rust/expressions.rs',
    'rust/src/translation/ir.rs',
    'rust/src/translation/ir/nodes.rs',
    'rust/src/translation/javascript/record_parameters.rs',
    'rust/src/translation/rust/expressions.rs',
    'rust/src/typed_program_links/checked.rs',
    'rust/tests/unit/self_translation_recursive_declaration_groups.rs',
    'rust/tests/unit/self_translation_scope.rs',
    'rust/tests/unit/translation_arrays.rs',
    'rust/tests/unit/translation_javascript_record_parameters.rs',
    'rust/tests/unit/translation_reference_identity.rs',
    'rust/tests/unit/translation_rust_literal_matches.rs',
    'rust/tests/unit/translation_surface_reference_analysis.rs',
  ];
  const file='parity/naming/array-identity-historical-inventories.lino';
  const stored=readOwnCodebaseHistoricalNamingPolicy().additionalSourceViews;
  let sources;
  if(check)sources=stored.sources;
  else{
    sources=stored.sources.map(row=>({...row,current:pin(readFileSync(new URL(row.file,root)))}));
    const namedPredecessors = [
      ...integrationFiles.map(path => [integration, path]),
      ['3866f25f3dde30644152829fb5d3bf8d680ba2fd', 'js/tests/issue-195-evidence-translation-partition-conservation.test.js'],
    ];
    for(const [revision,path] of namedPredecessors){
      if(sources.some(row=>row.file===path&&row.predecessor.revision===revision))continue;
      globalThis.gc?.();
      const {blob,certificate}=captureGitArtifactCertificate(revision,path);
      assert.deepEqual(verifyGitArtifactCertificate(certificate,blob,{revision,path}),blob);
      sources.push({file:path,predecessor:{revision,source:blob.toString('utf8'),...pin(blob),certificate},
        current:pin(readFileSync(new URL(path,root)))});
    }
  }
  // Finish Git object capture before retaining the complete naming inventory.
  // Its binding graph and Git's pack buffers need separate memory lifetimes.
  globalThis.gc?.();
  const {NAME_INVENTORIES}=await import('./issue-195-naming.mjs');
  const views=[];
  const projected=new Map();
  for(const source of sources){
    globalThis.gc?.();
    const current=readFileSync(new URL(source.file,root),'utf8');
    assert.deepEqual(pin(current),source.current,'normal freshness checks retain every complete current source pin');
    for(const entry of NAME_INVENTORIES.filter(entry=>!entry.words&&entry.files.includes(source.file))){
      const actual=new Set(entry.extract(current,source.file));
      const key=JSON.stringify([entry.inventory,source.file]);
      const retained=projected.get(key)??new Set();
      projected.set(key,retained);
      const names=[...new Set(entry.extract(source.predecessor.source,source.file))].filter(name=>!actual.has(name)&&!retained.has(name))
        .map(name=>({name,record:entry.recordOf?entry.recordOf(name):name}));
      for(const {name} of names)retained.add(name);
      if(names.length)views.push({inventory:entry.inventory,file:source.file,revision:source.predecessor.revision,names});
    }
  }
  const history={schemaVersion:1,model:'array identity historical naming inventories 1',revision,sources,views,
    boundary:'Each supplemental name is derived by its unchanged scoped inventory from a complete independently Git-certified original module. Every actual current source name is still extracted first. Historical definitions retain their original concept identities; a changed current body gains no equivalence proof from this source view. Whole current source hashes bind selection and unknown source bytes refuse. These source certificates do not establish runtime execution or requirement completion.'};
  if(check){assert.deepEqual(history,stored);checkProjectData(new URL(file,root),history);}
  else{writeProjectData(new URL(file,root),history);globalThis.gc?.();assert.deepEqual(readOwnCodebaseHistoricalNamingPolicy().additionalSourceViews,history);}
  return history;
}

if(process.argv[1]===fileURLToPath(import.meta.url)){
  const args=process.argv.slice(2);assert.ok(new Set(args).size===args.length&&args.every(arg=>['--check','--additional-inventories'].includes(arg)));
  if(args.includes('--additional-inventories')){
    const history=await generateAdditionalOwnCodebaseHistoricalNamingSources({check:args.includes('--check')});
    console.log(JSON.stringify({certifiedSources:history.sources.length,scopedViews:history.views.length,historicalNames:history.views.reduce((total,row)=>total+row.names.length,0),nativeExecution:false}));
  }else{
    generateOwnCodebaseHistoricalNamingSources({check:args.includes('--check')});
    console.log('Two complete historical source inventories and every original driver byte retained; current naming policy unchanged.');
  }
}
