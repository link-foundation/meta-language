// Reorder complete source preparation without changing any observation cell.
import assert from 'node:assert/strict';
import {readFileSync,writeFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {fileURLToPath} from 'node:url';
import {captureGitArtifactCertificate} from '../../js/scripts/capture-git-artifact-certificate.mjs';
import {checkProjectData,writeProjectData} from '../../js/scripts/project-data.mjs';
const root=new URL('../../',import.meta.url),revision='90636f9fb52b1b9095defbac041c8089608b8576';
const file='experiments/original-link-complete-table-projection/raw-native-observations.mjs';
const output=new URL('parity/self-translation/raw-successor-source-phases.lino',root);
const pin=source=>({bytes:Buffer.byteLength(source),sha256:createHash('sha256').update(source).digest('hex')});

/** Retain the full producer and derive all complete-source replay changes. */
export function generateOriginalLinkSuccessorSourcePhases({check=false,apply=false}={}) {
  assert.equal(check&&apply,false);
  const{blob,certificate}=captureGitArtifactCertificate(revision,file),original=blob.toString('utf8');assert.deepEqual(Buffer.from(original),blob);
  const actions=[];let source=original;
  const replace=(before,after)=>{assert.equal(source.split(before).length,2);source=source.replace(before,after);actions.push({before,after});};
  const anchor=original.split('\n').find(line=>line.includes("from './archive-transport.mjs'"))+'\n';
  replace(anchor,anchor+"import {createOriginalLinkSuccessorSourceArchives,readOriginalLinkSuccessorArchiveSource} from './successor-source-archives.mjs';\n");
  replace('const checkedPlans = new WeakSet();','const checkedPlans = new WeakSet();\nconst successorSourceArchives=createOriginalLinkSuccessorSourceArchives();\nconst readRawSourceTransports=()=>[readCanonicalArchiveTransportSource(),readOriginalLinkSuccessorArchiveSource()];');
  assert.equal(source.split('[readCanonicalArchiveTransportSource()]').length,5);
  for(let index=0;index<4;index++){
    const start=source.indexOf('[readCanonicalArchiveTransportSource()]'),before=source.slice(source.lastIndexOf('\n',start)+1,source.indexOf('\n',start));
    replace(before,before.replace('[readCanonicalArchiveTransportSource()]','readRawSourceTransports()'));
  }
  const start=source.indexOf('export async function planOriginalLinkRawNative('),end=source.indexOf('\nfunction originalLinkRawRecordDigest(',start);
  assert.ok(start>=0&&end>start);const previous=source.slice(start,end);
  let current=previous.replace('  return runOriginalLinkRawPlanningPhases({\n',
    "  const preparedSource=prepareOriginalLinkRawSource({readProjectData,parseJavaScript,checkProgram,emitRust,emitJavaScript,tokenize},emissionProfile);\n"
    +"  const successorSources=emissionProfile==='current'?await(async()=>{\n"
    +"    const generated=await generateOriginalLinkRawConsumingEmission(true);\n"
    +"    const borrowed=prepareOriginalLinkRawBorrowedEmission(generated.profile);\n"
    +"    return[{source:generated.rust.text,profile:generated.profile},{source:borrowed.source,profile:borrowed.profile}];\n"
    +"  })():[];\n  let sourceToken;\n  return runOriginalLinkRawPlanningPhases({\n");
  const prepare="    prepareSource: () => prepareOriginalLinkRawSource({readProjectData, parseJavaScript, checkProgram, emitRust, emitJavaScript, tokenize}, emissionProfile),";
  assert.equal(current.split(prepare).length,2);current=current.replace(prepare,'    prepareSource: () => preparedSource,');
  const replay='    replaySource: source => archiveCompleteRustSource(source, LinkNetwork, encodeLinkNetworkBinary, decodeLinkNetworkBinary),';
  assert.equal(current.split(replay).length,2);current=current.replace(replay,
    "    replaySource: source => {\n      const replay=source=>archiveCompleteRustSource(source,LinkNetwork,encodeLinkNetworkBinary,decodeLinkNetworkBinary);\n"
    +"      if(emissionProfile==='historical')return replay(source);\n"
    +"      sourceToken=successorSourceArchives.prepare({source,profile:preparedSource.emissionProfile},successorSources,replay);\n"
    +"      return successorSourceArchives.currentReceipt(sourceToken,source,preparedSource.emissionProfile);\n    },");
  const matrix='    buildMatrix: (prepared, receipt) => buildOriginalLinkRawPlan(prepared, receipt, modelSources, transportSources, {LinkNetwork, encodeLinkNetworkBinary, decodeLinkNetworkBinary, fixtures, corpus, transport, rules, oldDecoder, oldProjector, textProjection}),';
  assert.equal(current.split(matrix).length,2);current=current.replace(matrix,
    '    buildMatrix: (prepared, receipt) => {\n      const plan=buildOriginalLinkRawPlan(prepared,receipt,modelSources,transportSources,{LinkNetwork,encodeLinkNetworkBinary,decodeLinkNetworkBinary,fixtures,corpus,transport,rules,oldDecoder,oldProjector,textProjection});\n      if(sourceToken)successorSourceArchives.attach(plan,sourceToken);\n      return plan;\n    },');
  replace(previous,current);
  replace('  const originalReceipt=archiveCompleteRustSource(generated.rust.text,LinkNetwork,encodeLinkNetworkBinary,decodeLinkNetworkBinary);','  const originalReceipt=successorSourceArchives.read(currentPlan,generated.rust.text,generated.profile);');
  replace('  const originalReceipt=archiveCompleteRustSource(generated.source,LinkNetwork,encodeLinkNetworkBinary,decodeLinkNetworkBinary);','  const originalReceipt=successorSourceArchives.read(consumingPlan,generated.source,generated.profile);');
  const first='  checkedPlans.add(plan);return plan;\n}\n\n/** Reuse the complete checked consuming matrix';
  replace(first,first.replace('checkedPlans.add(plan);return plan;','checkedPlans.add(plan);successorSourceArchives.inherit(currentPlan,plan);return plan;'));
  const second='  checkedPlans.add(plan);return plan;\n}\n\nfunction parseObservations';
  replace(second,second.replace('checkedPlans.add(plan);return plan;','checkedPlans.add(plan);successorSourceArchives.inherit(consumingPlan,plan);return plan;'));
  let inverse=source;for(const action of actions.toReversed()){assert.equal(inverse.split(action.after).length,2);inverse=inverse.replace(action.after,action.before);}assert.equal(inverse,original);
  const actual=readFileSync(new URL(file,root),'utf8');assert.ok(actual===original||actual===source,'unknown producer edits refuse');
  const packet={schemaVersion:1,revision,sources:[{file,commit:revision,sourceText:original,...pin(blob),certificate,actions,proposed:{...pin(source),source}}],
    boundary:'Every full current, consuming and borrowed-consumer source still parses without recovery and replays both original archive formats at unchanged limits. All three finish before the complete unchanged observation matrix is retained. Only compact private replay receipts survive; all original fixture identities, input/result bits, carrier objects, process checks, source interfaces and observation counts remain mandatory. Complete workload resource fit and native execution require CI.'};
  if(check)checkProjectData(output,packet);else writeProjectData(output,packet);
  if(apply)writeFileSync(new URL(file,root),source);
  return packet;
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
  assert.ok(process.argv.slice(2).every(value=>['--check','--apply'].includes(value)));
  generateOriginalLinkSuccessorSourcePhases({check:process.argv.includes('--check'),apply:process.argv.includes('--apply')});
  console.log('Prepared complete successor source archives before retained observation-matrix allocation; full workload remains CI-only.');
}
