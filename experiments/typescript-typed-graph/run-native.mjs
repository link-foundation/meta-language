// Compile only in GitHub CI, after JavaScript gates. Native programs come
// entirely from the reloaded editable Links graph and the existing emitter.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {spawnSync,execFileSync} from 'node:child_process';
import {mkdirSync,mkdtempSync,rmSync,writeFileSync,readFileSync} from 'node:fs';
import {resolve,join} from 'node:path';
import {fileURLToPath} from 'node:url';
import {createTypeScriptSemanticGraph} from './projection.mjs';
import {LinkNetwork} from '../../js/src/network.js';
import {emitTypedProgramLinks,TYPED_PROGRAM_LINKS_SCHEMA} from '../../js/src/typed-program-links.js';
import {generateTypeScriptTypedGraphFixtures} from '../../js/scripts/generate-typescript-typed-graph-fixtures.mjs';
import {writeProjectData} from '../../js/scripts/project-data.mjs';

const root=fileURLToPath(new URL('../../',import.meta.url));
const pin=bytes=>({bytes:Buffer.byteLength(bytes),sha256:createHash('sha256').update(bytes).digest('hex')});

function processObservation(result) {
  return{exitCode:result.status,signal:result.signal,
    stdoutHex:(result.stdout??Buffer.alloc(0)).toString('hex'),
    stderrHex:(result.stderr??Buffer.alloc(0)).toString('hex'),
    error:result.error?{code:result.error.code??null,message:result.error.message}:null};
}

/** Keep every original result and run every case, retaining all diagnostics. */
export function runTypeScriptGraphNativeControls(resultsDirectory) {
  if(process.env.CI!=='true'||process.env.GITHUB_ACTIONS!=='true'){
    throw new Error('native TypeScript graph execution is GitHub CI-only');
  }
  const results=resolve(root,resultsDirectory);
  assert.equal(results,resolve(root,'issue-195-results'),'native controls use the project evidence directory');
  const artifacts=join(results,'artifacts','typescript-typed-graph-native');
  mkdirSync(artifacts,{recursive:true});mkdirSync(join(results,'work'),{recursive:true});
  const work=mkdtempSync(join(results,'work','typescript-typed-graph-'));
  const fixture=generateTypeScriptTypedGraphFixtures({check:true});
  const cases=[...fixture.cases.map(row=>({...row,edit:false})),{...fixture.edit,edit:true}];
  const version=spawnSync('rustc',['-vV'],{cwd:root,timeout:10_000,maxBuffer:1024*1024});
  const report={commit:execFileSync('git',['rev-parse','HEAD'],{cwd:root,encoding:'utf8'}).trim(),
    run:process.env.GITHUB_RUN_ID,attempt:process.env.GITHUB_RUN_ATTEMPT,
    nodeVersion:process.version,compiler:processObservation(version),
    fixture:pin(readFileSync(new URL('../../parity/fixtures/typescript-typed-semantic-graph.lino',import.meta.url))),
    observations:[],failures:[],
    boundary:'Actual standalone Rust executions of JavaScript-produced, source-free reloaded TypeScript semantic graphs. Complete original/lowered CSTs remain in each graph. This is not native TypeScript graph import, full TypeScript/compiler coverage or global meaning equivalence.'};
  const reportFile=join(artifacts,'observations.lino');
  try{
    if(version.status!==0)report.failures.push({operation:'compiler identity',observation:processObservation(version)});
    for(const [index,row]of cases.entries()){
      try{
      const name='case-'+index,packet=createTypeScriptSemanticGraph(row.source,{utf16Strings:true});
      // Emission receives the reloaded network and root alone; neither source
      // parser state nor an executable source envelope supplies its behavior.
      const graphText=packet.network.toLino(),network=LinkNetwork.fromLino(graphText);
      if(row.edit){
        const label=TYPED_PROGRAM_LINKS_SCHEMA.fields.op??'op';
        const operation=network.fieldRelations().find(field=>field.label===label
          &&network.link(field.child).metadata().definition===row.before);
        assert.ok(operation);const link=network.link(operation.child);
        link.setMetadata(link.metadata().withDefinition(row.after));
      }
      const actualGraph=network.toLino(),code=emitTypedProgramLinks(network,packet.root,'Rust').code;
      const graphFile=join(artifacts,name+'.lino'),sourceFile=join(artifacts,name+'.rs'),binary=join(work,name);
      writeFileSync(graphFile,actualGraph);writeFileSync(sourceFile,code);
      const compilation=spawnSync('rustc',['--edition=2024',sourceFile,'-o',binary],
        {cwd:root,timeout:60_000,maxBuffer:1024*1024});
      const execution=compilation.status===0?spawnSync(binary,[],{cwd:root,timeout:1000,maxBuffer:1024*1024}):null;
      const expected=Buffer.from(row.expectedLines.map(line=>line+'\n').join(''));
      const observation={case:index,variant:row.edit?'edited checked program':'original checked program',
        root:packet.root.asU64(),graph:{path:graphFile,...pin(actualGraph)},source:{path:sourceFile,...pin(code)},
        compilation:processObservation(compilation),execution:execution?processObservation(execution):null,
        expectedStdoutHex:expected.toString('hex')};
      report.observations.push(observation);
      if(compilation.status!==0||!execution||execution.status!==0||!execution.stdout.equals(expected)){
        report.failures.push({case:index,observation});
      }
      }catch(failure){
        report.failures.push({case:index,error:{name:failure.name,message:failure.message}});
      }finally{
        writeProjectData(reportFile,report);
      }
    }
  }finally{
    writeProjectData(reportFile,report);
    // Every synchronous child has stopped. Only this process's unique binary
    // scratch is removed; source graphs, code and actual observations survive.
    rmSync(work,{recursive:true});
  }
  assert.equal(report.failures.length,0,'native TypeScript graph controls failed; see '+reportFile);
  console.log('Executed '+report.observations.length+' reloaded TypeScript graph Rust programs with exact stdout checks.');
  return report;
}

if(process.argv[1]===fileURLToPath(import.meta.url)){
  assert.deepEqual(process.argv.slice(2),['--results-dir','issue-195-results']);
  runTypeScriptGraphNativeControls(process.argv[3]);
}
