// Project two complete already certified sources; retain the driver predecessor.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync,writeFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {execFileSync,spawnSync} from 'node:child_process';
import {verifyGitArtifactCertificate} from './git-object-certificate.mjs';
import {readBootstrapStructuredDataLino} from '../../scripts/lib/structured-data-bootstrap.mjs';
import {captureGitArtifactCertificate} from './capture-git-artifact-certificate.mjs';
import {readOwnCodebaseHistoricalNamingPolicy,verifyOwnCodebaseHistoricalNamingSources} from './own-codebase-baseline-naming-history.mjs';
import {checkProjectData,writeProjectData,serializeProjectData} from './project-data.mjs';
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
export async function generateAdditionalOwnCodebaseHistoricalNamingSources({check=false,authorOnly=false}={}){
  const revision='6ffff3b2dc530a5f24c128dadf877f3fa106932b',baseline='8bc0ef9725388c7927acf1854870bf46f5e365be';
  const integration='8f2f6c483ec32a3ffc17afec694463965dc7551d';
  const file='parity/naming/array-identity-historical-inventories.lino';
  let sources,stored;
  if(check){stored=readOwnCodebaseHistoricalNamingPolicy().additionalSourceViews;sources=stored.sources;}
  else{
    const originalFiles=['js/src/translation/javascript-infer.js','js/tests/translation-frontend-rules.test.js',
      'rust/tests/unit/translation_surface_reference_analysis.rs','js/scripts/own-codebase-baseline-naming-history.mjs'];
    const changedAt=selected=>{
      const known=new Set(execFileSync('git',['ls-tree','-r','--name-only',selected],{cwd:root,encoding:'utf8',maxBuffer:1024*1024}).trim().split('\n'));
      return execFileSync('git',['diff',selected,'--name-only'],{cwd:root,encoding:'utf8'}).trim().split('\n')
        .filter(path=>known.has(path)&&/^(?:js\/(?:src|scripts|tests)|rust\/(?:src|tests\/unit)|experiments)\/.+\.(?:js|mjs|rs)$/u.test(path));
    };
    const selections=[...originalFiles.map(path=>({path,revision})),...changedAt(baseline).map(path=>({path,revision:baseline})),
      ...changedAt(integration).map(path=>({path,revision:integration}))];
    const scalarSources=new Map();
    sources=selections.map(({path,revision})=>{
      globalThis.gc?.();
      const {blob,certificate}=captureGitArtifactCertificate(revision,path);
      assert.deepEqual(verifyGitArtifactCertificate(certificate,blob,{revision,path}),blob);
      const sourcePin=pin(blob);
      if(!scalarSources.has(sourcePin.sha256))scalarSources.set(sourcePin.sha256,blob.toString('utf8'));
      // Only equal immutable scalar strings share storage. All ordered source
      // rows, certificates and containers keep their independent identities.
      return{file:path,predecessor:{revision,source:scalarSources.get(sourcePin.sha256),...sourcePin,certificate},
        current:pin(readFileSync(new URL(path,root)))};
    });
  }
  console.error('Captured '+sources.length+' complete naming source epochs.');
  // Finish Git object capture before retaining the complete naming inventory.
  // Its binding graph and Git's pack buffers need separate memory lifetimes.
  globalThis.gc?.();
  const {NAME_INVENTORIES}=await import('./issue-195-naming.mjs');
  console.error('Loaded complete naming inventories.');
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
  console.error('Derived '+views.length+' historical naming views.');
  if(check){assert.deepEqual(history,stored);checkProjectData(new URL(file,root),history);}
  else{writeProjectData(new URL(file,root),history);globalThis.gc?.();if(!authorOnly)assert.deepEqual(readOwnCodebaseHistoricalNamingPolicy().additionalSourceViews,history);}
  return history;
}

// The normal CLI keeps the full author/reader comparison while separating
// their large graphs into fresh serial processes. No assertion is optional in
// the normal command; the internal author emits only an invocation receipt.
if(process.argv[1]===fileURLToPath(import.meta.url)){
  const args=process.argv.slice(2),file=new URL('../../parity/naming/array-identity-historical-inventories.lino',import.meta.url);
  if(args.length===1&&args[0]==='--author-additional-inventories'){
    const history=await generateAdditionalOwnCodebaseHistoricalNamingSources({authorOnly:true});
    process.stdout.write(serializeProjectData({source:pin(readFileSync(file)),certifiedSources:history.sources.length,scopedViews:history.views.length,historicalNames:history.views.reduce((total,row)=>total+row.names.length,0)},'receipt.lino'));
  }else if(args.length===3&&args[0]==='--verify-additional-inventories'){
    assert.match(args[1],/^[a-f0-9]{64}$/u);const expected={bytes:Number(args[2]),sha256:args[1]};
    assert.ok(Number.isSafeInteger(expected.bytes)&&expected.bytes>0&&expected.bytes<=64*1024*1024);
    assert.deepEqual(pin(readFileSync(file)),expected,'the entire actual output equals the completed normal author receipt');
    const history=readOwnCodebaseHistoricalNamingPolicy().additionalSourceViews;
    globalThis.gc?.();assert.deepEqual(pin(serializeProjectData(history,'history.lino')),expected,'the normal fully authenticated reader reconstructs every generated byte');
    assert.deepEqual(pin(readFileSync(file)),expected,'output did not change during complete reader verification');
  }else{
    assert.ok(new Set(args).size===args.length&&args.every(arg=>['--check','--additional-inventories'].includes(arg)));
    if(args.includes('--additional-inventories')){
      if(args.includes('--check')){
        const history=await generateAdditionalOwnCodebaseHistoricalNamingSources({check:true});
        console.log(JSON.stringify({certifiedSources:history.sources.length,scopedViews:history.views.length,historicalNames:history.views.reduce((total,row)=>total+row.names.length,0),nativeExecution:false}));
      }else{
        const invoke=argumentsList=>{
          const child=spawnSync(process.execPath,[...process.execArgv,fileURLToPath(import.meta.url),...argumentsList],{stdio:['ignore','pipe','inherit'],maxBuffer:1024*1024,timeout:70000});
          if(child.error)throw child.error;assert.equal(child.signal,null);assert.equal(child.status,0);return child.stdout;
        };
        const receipt=readBootstrapStructuredDataLino(invoke(['--author-additional-inventories']).toString('utf8'));
        invoke(['--verify-additional-inventories',receipt.source.sha256,String(receipt.source.bytes)]);
        console.log(JSON.stringify({...receipt,nativeExecution:false}));
      }
    }else{
      generateOwnCodebaseHistoricalNamingSources({check:args.includes('--check')});
      console.log('Two complete historical source inventories and every original driver byte retained; current naming policy unchanged.');
    }
  }
}
