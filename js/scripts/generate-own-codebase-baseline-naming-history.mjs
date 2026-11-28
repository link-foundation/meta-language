// Project two complete already certified sources; retain the driver predecessor.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync,writeFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {execFileSync} from 'node:child_process';
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
  const revision='6ffff3b2dc530a5f24c128dadf877f3fa106932b',baseline='8bc0ef9725388c7927acf1854870bf46f5e365be';
  const file='parity/naming/array-identity-historical-inventories.lino';
  let sources,stored;
  if(check){stored=readOwnCodebaseHistoricalNamingPolicy().additionalSourceViews;sources=stored.sources;}
  else{
    const originalFiles=['js/src/translation/javascript-infer.js','js/tests/translation-frontend-rules.test.js',
      'rust/tests/unit/translation_surface_reference_analysis.rs','js/scripts/own-codebase-baseline-naming-history.mjs'];
    const changed=execFileSync('git',['diff',baseline,'--name-only'],{cwd:root,encoding:'utf8'}).trim().split('\n')
      .filter(path=>/^(?:js\/(?:src|scripts|tests)|rust\/(?:src|tests\/unit)|experiments)\/.+\.(?:js|mjs|rs)$/u.test(path));
    const selections=[...originalFiles.map(path=>({path,revision})),...changed.map(path=>({path,revision:baseline}))];
    sources=selections.map(({path,revision})=>{
      globalThis.gc?.();
      const {blob,certificate}=captureGitArtifactCertificate(revision,path);
      assert.deepEqual(verifyGitArtifactCertificate(certificate,blob,{revision,path}),blob);
      return{file:path,predecessor:{revision,source:blob.toString('utf8'),...pin(blob),certificate},
        current:pin(readFileSync(new URL(path,root)))};
    });
  }
  // Finish Git object capture before retaining the complete naming inventory.
  // Its binding graph and Git's pack buffers need separate memory lifetimes.
  globalThis.gc?.();
  const {NAME_INVENTORIES}=await import('./issue-195-naming.mjs');
  const views=[];
  for(const source of sources){
    globalThis.gc?.();
    const current=readFileSync(new URL(source.file,root),'utf8');
    assert.deepEqual(pin(current),source.current,'normal freshness checks retain every complete current source pin');
    for(const entry of NAME_INVENTORIES.filter(entry=>!entry.words&&entry.files.includes(source.file))){
      const actual=new Set(entry.extract(current,source.file));
      const names=[...new Set(entry.extract(source.predecessor.source,source.file))].filter(name=>!actual.has(name))
        .map(name=>({name,record:entry.recordOf?entry.recordOf(name):name}));
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
