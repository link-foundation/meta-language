// A source-only view for one historical relative-array descriptor reader.
// Current parsing, emission, source owners and receipts remain independent.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {readBootstrapStructuredDataLino} from '../../scripts/lib/structured-data-bootstrap.mjs';
import {verifyGitArtifactCertificate} from './git-object-certificate.mjs';
import {readOwnCodebaseRawDescriptorPolicy,readOwnCodebaseRawDescriptorPacket,
  deriveOwnCodebaseRawDescriptorSource} from './own-codebase-raw-source-descriptors.mjs';
import {readOwnCodebaseHistoricalNamingPolicy} from './own-codebase-baseline-naming-history.mjs';
const root=new URL('../../',import.meta.url);
export const RELATIVE_ARRAY_FRONTEND_PROFILE='relative array';
export const RELATIVE_ARRAY_FRONTEND_HOST={path:'js/scripts/generate-array-relative-element-history.mjs',revision:'7881e1e6f83818e0b25b77e1b3ab15c2d51ca9f3',
  bytes:8460,sha256:'588f97244142c9939845c079530469d198486771714fe41f20794032a7c4e4f1'};
export const RELATIVE_ARRAY_FRONTEND_HOST_CHANGES=[
  {before:"import {restoreContinuationHistoricalSource} from './continuation-source-history.mjs';\n",
    after:"import {restoreContinuationHistoricalSource} from './continuation-source-history.mjs';\nimport {restoreRelativeArrayFrontendDescriptorSource} from './relative-array-frontend-source-history.mjs';\n"},
  {before:"const historicalSource=file=>restoreContinuationHistoricalSource(readFileSync(new URL(file,root),'utf8'),file,'relative array');\n",
    after:"const historicalSource=file=>restoreRelativeArrayFrontendDescriptorSource(restoreContinuationHistoricalSource(readFileSync(new URL(file,root),'utf8'),file,'relative array'),file,'relative array');\n"},
];
// This one additional source successor preserves the newer array owner at
// runtime; only the historical descriptor reader requests its complete7881 view.
export const RELATIVE_ARRAY_NULLABLE_INFERENCE_SOURCE={path:'js/src/translation/javascript-infer.js',
  revision:'7881e1e6f83818e0b25b77e1b3ab15c2d51ca9f3',
  previous:{bytes:62588,sha256:'4a6f2db0a3c620d82cdea5048c013722070da9ebbe722d8c249b5197b63f1165'},
  current:{bytes:62902,sha256:'f87fa8dcb3362165869d8f4678fd014f13c057eee92243d1489297613c66972d'}};
export const RELATIVE_ARRAY_NULLABLE_INFERENCE_CHANGE={
  before:`        const object = this.expr(node.object, env);
        const result = { kind: 'array', element: this.fresh() };
        this.iterables.push({ object, result, where: node.span });`,
  after:`        const object = this.expr(node.object, env);
        // Keep the original array term and its optional-element owner. The
        // existing shared iterable plan already forwards known arrays unchanged.
        if (readArrayFromIterableType(this.resolve(object))?.form === 0) {
          this.collections.set(node, object);
          return object;
        }
        const result = { kind: 'array', element: this.fresh() };
        this.iterables.push({ object, result, where: node.span });`,
};

export function replayRelativeArrayNullableInference(source){
  const change=RELATIVE_ARRAY_NULLABLE_INFERENCE_CHANGE;
  assert.deepEqual(relativeArrayFrontendPin(source),RELATIVE_ARRAY_NULLABLE_INFERENCE_SOURCE.previous);
  assert.equal(source.split(change.before).length,2,'one exact nullable-array iterable owner interval');
  const start=source.indexOf(change.before),current=source.slice(0,start)+change.after+source.slice(start+change.before.length);
  const inverse=current.slice(0,start)+change.before+current.slice(start+change.after.length);
  assert.equal(inverse,source,'the complete certified predecessor is recovered without editing runtime source');
  assert.deepEqual(relativeArrayFrontendPin(current),RELATIVE_ARRAY_NULLABLE_INFERENCE_SOURCE.current);
  return{source:current,edits:[{start,removed:change.before,inserted:change.after}]};
}

const historyPath='parity/self-translation/relative-array-frontend-source-history.lino';
const historySha256='b54d02013f60b289d3b7197a87187497960efb951ceedc7dda117cca43467db3';
let cachedHistory,cachedCurrentSource;
export const relativeArrayFrontendPin=value=>({bytes:Buffer.byteLength(value),sha256:createHash('sha256').update(value).digest('hex')});
const exactKeys=(value,keys)=>{assert.ok(value&&typeof value==='object'&&!Array.isArray(value));assert.deepEqual(Object.keys(value).sort(),keys.slice().sort());};

export function replayRelativeArrayFrontendHost(source){
  let current=source;const edits=[];
  for(const change of RELATIVE_ARRAY_FRONTEND_HOST_CHANGES){
    assert.equal(current.split(change.before).length,2,'one exact existing relative-array source adapter interval');
    const start=current.indexOf(change.before);edits.push({start,removed:change.before,inserted:change.after});
    current=current.slice(0,start)+change.after+current.slice(start+change.before.length);
  }
  let inverse=current;
  for(const edit of edits.toReversed()){
    assert.equal(inverse.slice(edit.start,edit.start+edit.inserted.length),edit.inserted);
    inverse=inverse.slice(0,edit.start)+edit.removed+inverse.slice(edit.start+edit.inserted.length);
  }
  assert.equal(inverse,source,'every source input original assertion descriptor field and native receipt stays exact');
  return{source:current,edits};
}

/** Reuse the current source authority; do not invent a descriptor pin or
 * identify the distinct older and current source definitions as equivalent. */
export function readAuthenticatedCurrentRelativeArrayFrontend(){
  if(cachedCurrentSource===undefined){
    const policy=readOwnCodebaseRawDescriptorPolicy(),packet=readOwnCodebaseRawDescriptorPacket();
    assert.equal(packet.sourceOwner.file,policy.sourceFile);
    cachedCurrentSource=deriveOwnCodebaseRawDescriptorSource(policy.sourceOwner,packet.sourceOwner.sourceText);
    assert.deepEqual(relativeArrayFrontendPin(cachedCurrentSource),policy.sourceOwner.current);
  }
  return cachedCurrentSource;
}

/** The compact projection contains the complete old source and its original
 * Git membership. Normal verification never reloads the large naming archive. */
export function verifyRelativeArrayFrontendSourceHistory(history){
  exactKeys(history,['schemaVersion','model','profile','sourceArchive','historical','currentAuthority','nullableInference','host','boundary']);
  assert.equal(history.schemaVersion,2);assert.equal(history.model,'relative array frontend descriptor historical source 2');
  assert.equal(history.profile,RELATIVE_ARRAY_FRONTEND_PROFILE);
  const naming=readOwnCodebaseHistoricalNamingPolicy(),current=readOwnCodebaseRawDescriptorPolicy();
  assert.deepEqual(history.sourceArchive,{revision:naming.revision,...naming.sourceArchive});
  exactKeys(history.historical,['file','commit','bytes','sha256','sourceText','certificate']);
  const previous=current.artifactDescriptorPrevious;
  assert.deepEqual({file:history.historical.file,bytes:history.historical.bytes,sha256:history.historical.sha256},previous);
  assert.equal(history.historical.commit,naming.revision);
  assert.deepEqual(relativeArrayFrontendPin(history.historical.sourceText),{bytes:previous.bytes,sha256:previous.sha256});
  verifyGitArtifactCertificate(history.historical.certificate,Buffer.from(history.historical.sourceText),{revision:naming.revision,path:previous.file});
  assert.deepEqual(history.currentAuthority,{revision:current.revision,file:current.sourceFile,
    predecessor:current.sourceOwner.predecessor,current:current.sourceOwner.current});
  const inference=history.nullableInference,expectedInference=RELATIVE_ARRAY_NULLABLE_INFERENCE_SOURCE;
  exactKeys(inference,['path','revision','bytes','sha256','sourceText','certificate','current','edits']);
  assert.equal(inference.path,expectedInference.path);assert.equal(inference.revision,expectedInference.revision);
  assert.deepEqual({bytes:inference.bytes,sha256:inference.sha256},expectedInference.previous);
  assert.deepEqual(relativeArrayFrontendPin(inference.sourceText),expectedInference.previous);
  verifyGitArtifactCertificate(inference.certificate,Buffer.from(inference.sourceText),{revision:expectedInference.revision,path:expectedInference.path});
  const inferenceReplay=replayRelativeArrayNullableInference(inference.sourceText);
  assert.deepEqual(inference.current,expectedInference.current);assert.deepEqual(inference.edits,inferenceReplay.edits);
  exactKeys(history.host,['path','revision','bytes','sha256','sourceText','certificate','current','edits']);
  assert.deepEqual({path:history.host.path,revision:history.host.revision,bytes:history.host.bytes,sha256:history.host.sha256},RELATIVE_ARRAY_FRONTEND_HOST);
  assert.deepEqual(relativeArrayFrontendPin(history.host.sourceText),{bytes:RELATIVE_ARRAY_FRONTEND_HOST.bytes,sha256:RELATIVE_ARRAY_FRONTEND_HOST.sha256});
  verifyGitArtifactCertificate(history.host.certificate,Buffer.from(history.host.sourceText),{revision:RELATIVE_ARRAY_FRONTEND_HOST.revision,path:RELATIVE_ARRAY_FRONTEND_HOST.path});
  const replay=replayRelativeArrayFrontendHost(history.host.sourceText);
  assert.deepEqual(history.host.current,relativeArrayFrontendPin(replay.source));assert.deepEqual(history.host.edits,replay.edits);
  return{source:history.historical.sourceText,host:replay.source,inferenceSource:inference.sourceText,currentInference:inferenceReplay.source};
}

export function readRelativeArrayFrontendSourceHistory(){
  if(!cachedHistory){
    const bytes=readFileSync(new URL(historyPath,root));assert.equal(relativeArrayFrontendPin(bytes).sha256,historySha256);
    cachedHistory=readBootstrapStructuredDataLino(bytes.toString('utf8'));
    const verified=verifyRelativeArrayFrontendSourceHistory(cachedHistory);
    assert.equal(readFileSync(new URL(RELATIVE_ARRAY_FRONTEND_HOST.path,root),'utf8'),verified.host,'only the exact two scoped host edits are admitted');
    assert.equal(readFileSync(new URL(RELATIVE_ARRAY_NULLABLE_INFERENCE_SOURCE.path,root),'utf8'),verified.currentInference,'the actual nullable-array successor retains its separate current runtime source');
  }
  return structuredClone(cachedHistory);
}

/** This function is called only by the historical relative-array generator.
 * Other profiles/files are identities; unknown selected successors refuse. */
export function restoreRelativeArrayFrontendDescriptorSource(source,file,profile){
  assert.equal(typeof source,'string');assert.equal(typeof file,'string');assert.equal(typeof profile,'string');
  const policy=readOwnCodebaseRawDescriptorPolicy();
  if(profile!==RELATIVE_ARRAY_FRONTEND_PROFILE)return source;
  if(file===RELATIVE_ARRAY_NULLABLE_INFERENCE_SOURCE.path){
    const history=readRelativeArrayFrontendSourceHistory(),previous=history.nullableInference.sourceText;
    if(source===previous)return source;
    assert.deepEqual(relativeArrayFrontendPin(source),RELATIVE_ARRAY_NULLABLE_INFERENCE_SOURCE.current,'an unknown nullable-inference successor has no relative-array historical authority');
    assert.equal(source,replayRelativeArrayNullableInference(previous).source,'the complete current nullable-inference source has one certified finite inverse');
    return previous;
  }
  if(file!==policy.sourceFile)return source;
  const history=readRelativeArrayFrontendSourceHistory(),previous=history.historical.sourceText;
  if(source===previous)return source;
  assert.deepEqual(relativeArrayFrontendPin(source),policy.sourceOwner.current,'an unknown frontend successor has no relative-array historical authority');
  assert.equal(source,readAuthenticatedCurrentRelativeArrayFrontend(),'the actual complete current frontend is authenticated through its existing source receipt');
  return previous;
}
