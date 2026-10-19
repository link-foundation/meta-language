// An explicit source-compiler successor; both earlier artifacts remain immutable.
// Importing starts no compiler, grammar parse, fixture matrix or native process.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {readProjectData,writeProjectData,checkProjectData} from '../../js/scripts/project-data.mjs';
import {readStructuredDataLino,structuredDataToLino} from '../../js/src/structured-data-links.js';
import {tokenize} from '../../js/src/translation/lexer.js';
import {readRustLibraryItems} from '../../js/scripts/rust-library-packaging.mjs';
import {prepareCurrentOriginalLinkRawEmission} from './generate-raw-current-emission.mjs';
import {ORIGINAL_LINK_RAW_EMISSION_BASELINE,readOriginalLinkRawEmissionProfile} from './raw-emission-profiles.mjs';
import {ORIGINAL_LINK_RAW_COLLECTION_ARCHIVE_RECEIPT_REFRESH} from './raw-emission-producer-applicability.mjs';
import {restoreCompleteDeclarationReceipt} from '../../js/scripts/complete-declaration-receipts.mjs';

export const ORIGINAL_LINK_RAW_CONSUMING_HISTORICAL_ARTIFACT = new URL('./data/raw-consuming-native-definitions.lino',import.meta.url);
export const ORIGINAL_LINK_RAW_CONSUMING_ARTIFACT = new URL('./data/raw-consuming-current-native-definitions.lino',import.meta.url);
export const ORIGINAL_LINK_RAW_CONSUMING_SUCCESSOR = Object.freeze({
  version:1,
  original:Object.freeze({
    file:'experiments/original-link-complete-table-projection/data/raw-consuming-native-definitions.lino',
    revision:'410f51f086010fcfa895001f4f12b61700fbcfad',bytes:1801946,
    sha256:'3c1793f7ce672a846e2fc480f20596e985b9528644748a8078cd170ef5ac81c8',
  }),
  currentArtifact:'data/raw-consuming-current-native-definitions.lino',
  receiptRefresh:ORIGINAL_LINK_RAW_COLLECTION_ARCHIVE_RECEIPT_REFRESH,
  // No archive difference has been observed. Only reviewed exact before/after
  // textSha256 or binarySha256 rows may be added; every other field stays exact.
  archiveChanges:Object.freeze([]),
});
const root = new URL('../../',import.meta.url);
const sha256 = value => createHash('sha256').update(value).digest('hex');
const sourceArtifact = ORIGINAL_LINK_RAW_CONSUMING_SUCCESSOR.currentArtifact;
const maximumRecords = 1_000_000, maximumArchiveBytes = 64 * 1024 * 1024;
const decisionFiles = Object.freeze([
  'experiments/original-link-complete-table-projection/raw-consuming-emission.mjs',
  'js/src/translation/javascript.js', 'js/src/translation/javascript-infer.js',
  'js/src/translation/check.js', 'js/src/translation/emit-rust.js',
  'js/src/translation/emit-javascript.js',
  'js/src/translation/owned-array-moves.js', 'js/src/translation/owned-array-append-rules.js',
  'js/src/translation/fact-column-rules.js', 'js/src/typed-program-link-rules.js',
  'js/src/typed-program-links.js', 'js/src/typed-program-checked-projection.js',
]);
const decisionSources = read => decisionFiles.map(file => {
  const bytes = read(new URL(file,root));
  return {file,bytes:bytes.length,sha256:sha256(bytes)};
});
const words = tokens => tokens.filter(token => token.kind !== 'eof')
  .map(token => token.kind === 'number' ? token.raw : token.value);

/** Inspect actual emitted tokens; a spelling inside a String is never a site. */
export function readOriginalLinkRawConsumingOwnerSites(source) {
  const items = readRustLibraryItems(source).items;
  assert.equal(items.length,1,'one complete emitted owner is inspected');
  const owner = items[0], tokens = tokenize(owner.source,'Rust').tokens;
  const body = tokens.findIndex(token => token.kind === 'punct' && token.value === '{');
  assert.ok(body >= 0,'the complete emitted owner has its body');
  const clones = [], appends = [], consumingAppendBindings = [];
  for(let at=body+1;at<tokens.length;at++) {
    const token = tokens[at];
    if(token.kind === 'identifier' && token.value === 'clone' && tokens[at-1]?.value === '.'
      && tokens[at+1]?.value === '(' && tokens[at+2]?.value === ')') clones.push(token.start);
    if(token.kind === 'identifier' && token.value === 'append' && tokens[at-1]?.value === '::'
      && tokens[at-2]?.value === 'ml_array' && tokens[at+1]?.value === '(') appends.push(token.start);
    if(token.kind === 'identifier' && token.value === 'ml_append_left'
      && tokens[at-1]?.value === ',' && tokens[at+1]?.value === ')') consumingAppendBindings.push(token.start);
  }
  return {target:owner.name,kind:owner.kind,header:words(tokens.slice(0,body)),bytes:Buffer.byteLength(source),
    sha256:sha256(source),cloneSites:clones,appendSites:appends,consumingAppendBindingSites:consumingAppendBindings};
}

function consumingOwners(emitted,inventory) {
  assert.equal(emitted.definitions.length,inventory.length);
  return inventory.map((owner,index) => {
    const mappings = emitted.mappings.filter(mapping => mapping.source === owner.fullName
      && mapping.kind === (owner.kind === 'fn' ? 'function' : owner.kind));
    assert.equal(mappings.length,1,'one complete emitted source mapping: '+owner.fullName);
    const sites = readOriginalLinkRawConsumingOwnerSites(emitted.definitions[index]);
    assert.equal(sites.target,mappings[0].target,'the actual definition order follows the checked owner order');
    assert.equal(sites.kind,owner.kind === 'fn' ? 'fn' : 'enum');
    return sites;
  });
}
const totals = owners => ({cloneCalls:owners.reduce((sum,owner)=>sum+owner.cloneSites.length,0),
  appendCalls:owners.reduce((sum,owner)=>sum+owner.appendSites.length,0),
  consumingAppendBindings:owners.reduce((sum,owner)=>sum+owner.consumingAppendBindingSites.length,0)});

/** Restore only the recorded provenance changes and authenticate the entire original artifact. */
export function verifyOriginalLinkRawConsumingConservation(artifact,read=readFileSync) {
  assert.deepEqual(artifact.decisionSources,decisionSources(read),'actual final-use decision sources changed');
  artifact=restoreCompleteDeclarationReceipt(artifact,'raw consuming');
  const successor=ORIGINAL_LINK_RAW_CONSUMING_SUCCESSOR, originalBytes=read(ORIGINAL_LINK_RAW_CONSUMING_HISTORICAL_ARTIFACT);
  assert.equal(originalBytes.length,successor.original.bytes,'the original consuming artifact byte count is immutable');
  assert.equal(sha256(originalBytes),successor.original.sha256,'the complete original consuming artifact is immutable');
  const original=readStructuredDataLino(originalBytes.toString('utf8'));
  assert.deepEqual(artifact.predecessorConservation,successor,'the explicit consuming successor provenance is exact');
  assert.deepEqual(original.decisionSources.map(row=>row.file),decisionFiles,'the original decision-source inventory stays complete');
  assert.equal(Object.hasOwn(original,'predecessorConservation'),false);
  const inverse=structuredClone(artifact);
  delete inverse.predecessorConservation;
  inverse.decisionSources=structuredClone(original.decisionSources);
  const refresh=successor.receiptRefresh, applicability=inverse.predecessor.proof.producerApplicability;
  const originalApplicability=original.predecessor.proof.producerApplicability;
  assert.equal(Object.hasOwn(originalApplicability,'receiptRefresh'),false);
  assert.deepEqual(applicability.receiptRefresh,refresh,'the exact predecessor receipt refresh remains reachable');
  delete applicability.receiptRefresh;
  const positions=applicability.sources.flatMap((row,index)=>row.file===refresh.current.file?[index]:[]);
  const originalPositions=originalApplicability.sources.flatMap((row,index)=>row.file===refresh.previous.file?[index]:[]);
  assert.equal(positions.length,1,'only one exact predecessor artifact descriptor is replaced');
  assert.deepEqual(positions,originalPositions,'the predecessor artifact descriptor retains its original position');
  assert.deepEqual(applicability.sources[positions[0]],refresh.current);
  assert.deepEqual(originalApplicability.sources[positions[0]],refresh.previous);
  applicability.sources[positions[0]]=structuredClone(refresh.previous);
  const fields=new Set();
  for(const change of successor.archiveChanges) {
    assert.ok(['textSha256','binarySha256'].includes(change.field)&&!fields.has(change.field),
      'only distinct reviewed complete archive digest changes are admitted');
    fields.add(change.field);
    assert.match(change.before,/^[a-f0-9]{64}$/u);assert.match(change.after,/^[a-f0-9]{64}$/u);
    assert.notEqual(change.before,change.after);
    assert.equal(original.archive[change.field],change.before);
    assert.equal(inverse.archive[change.field],change.after);
    inverse.archive[change.field]=change.before;
  }
  assert.deepEqual(inverse.archive,original.archive,'unreviewed archive differences require exact recorded digest changes');
  const inverseBytes=Buffer.from(structuredDataToLino(inverse));
  assert.equal(inverseBytes.length,successor.original.bytes,'the complete inverse byte count remains exact');
  assert.equal(sha256(inverseBytes),successor.original.sha256,'only the recorded consuming provenance changes are permitted');
  assert.deepEqual(inverse,original,'every original consuming artifact field and owner remains exact');
  assert.deepEqual(inverseBytes,originalBytes,
    'the complete canonical original artifact bytes are restored, including every owner and archive field');
  return successor;
}

/** Revalidate stored bytes and actual decision sources around an already checked plan. */
export function readOriginalLinkRawConsumingProfile(currentProfile,read=readFileSync) {
  assert.equal(currentProfile.profile,'current','the consuming successor retains the actual current predecessor');
  const bytes = read(ORIGINAL_LINK_RAW_CONSUMING_ARTIFACT);
  const artifact = readStructuredDataLino(bytes.toString('utf8'));
  assert.equal(artifact.version,1);assert.equal(artifact.profile,'consuming');
  assert.deepEqual(artifact.options,{arrayConsumption:true});
  assert.deepEqual(artifact.predecessor,currentProfile);
  assert.deepEqual(artifact.decisionSources,decisionSources(read),'actual final-use decision sources changed');
  assert.equal(artifact.nativeCompilation,false);assert.equal(artifact.nativeExecution,false);
  assert.equal(artifact.definitionCount,106);assert.equal(artifact.owners.length,106);
  assert.deepEqual(artifact.declarationNames,currentProfile.declarationNames);
  assert.deepEqual(artifact.owners.map(owner=>owner.fullName),artifact.declarationNames);
  assert.deepEqual(artifact.source,{path:'experiments/original-link-complete-table-projection/raw-rules.mjs',
    bytes:ORIGINAL_LINK_RAW_EMISSION_BASELINE.sourceBytes,sha256:ORIGINAL_LINK_RAW_EMISSION_BASELINE.sourceSha256});
  assert.equal(Buffer.byteLength(artifact.completeRust),artifact.sourceBytes);
  assert.equal(sha256(artifact.completeRust),artifact.sourceSha256);
  assert.equal(artifact.completeRust.split(ORIGINAL_LINK_RAW_EMISSION_BASELINE.rootHeader).length,2,
    'the owned public root interface remains exact');
  const items=readRustLibraryItems(artifact.completeRust).items;
  for(const owner of artifact.owners) {
    const candidates=items.filter(item=>item.name===owner.consuming.target);
    assert.equal(candidates.length,1,'every stored owner has one actual complete definition');
    const actual=readOriginalLinkRawConsumingOwnerSites(candidates[0].source);
    assert.deepEqual(owner.copy.header,owner.consuming.header,'the stored successor keeps the original complete ABI');
    assert.deepEqual(actual.header,owner.consuming.header);
    for(const field of ['cloneSites','appendSites','consumingAppendBindingSites'])assert.equal(actual[field].length,owner.consuming[field].length,
      'stored token-site counts agree with the actual complete definition');
  }
  assert.deepEqual(artifact.totals,{copy:totals(artifact.owners.map(owner=>owner.copy)),
    consuming:totals(artifact.owners.map(owner=>owner.consuming))});
  assert.equal(artifact.replays.length,2);
  assert.deepEqual(artifact.replays.map(row=>row.format),['text','binary']);
  assert.equal(artifact.archive.binaryLimits.recordLimit,artifact.archive.recordCount);
  assert.ok(artifact.archive.recordCount>0&&artifact.archive.recordCount<=maximumRecords);
  assert.equal(artifact.archive.maximumRecords,maximumRecords);
  assert.equal(artifact.archive.binaryLimits.byteLimit,maximumArchiveBytes);
  assert.ok(artifact.archive.textBytes<=maximumArchiveBytes&&artifact.archive.binaryBytes<=maximumArchiveBytes);
  for(const replay of artifact.replays) {
    assert.equal(replay.ownerCount,106);assert.equal(replay.emittedSha256,artifact.sourceSha256);
    assert.equal(replay.emittedBytes,artifact.sourceBytes);
  }
  const predecessorConservation=verifyOriginalLinkRawConsumingConservation(artifact,read);
  return {profile:'consuming',sourceBytes:artifact.sourceBytes,sourceSha256:artifact.sourceSha256,
    historicalSourceSha256:currentProfile.historicalSourceSha256,currentSourceSha256:currentProfile.currentSourceSha256,
    sourceArtifact,declarationNames:artifact.declarationNames,definitionCount:106,
    proof:{artifactSha256:sha256(bytes),predecessor:currentProfile,predecessorConservation,
      decisionSources:artifact.decisionSources,options:artifact.options,archive:artifact.archive,replays:artifact.replays,
      tokenizedSites:artifact.totals,
      boundary:'Actual emitted token sites and existing checked final-use decision source pins; not an internal graph-fact inventory or native performance observation.'}};
}

/** Emit the same complete checked program, then independently reload both typed archives. */
export async function prepareOriginalLinkRawConsumingEmission(prepared,{check=true}={}) {
  const currentProfile = readOriginalLinkRawEmissionProfile('current',prepared);
  const {source:currentSource,...predecessor} = currentProfile;
  const {program,rust:copied} = prepared;
  assert.equal(copied.text,currentSource,'the complete original copy emission is still exact');
  assert.equal(program.declarations.size,106);assert.equal(program.items.length,106);
  const inventory = program.items.map((item,ordinal) => ({ordinal,kind:item.k,name:item.name,
    fullName:item.fullName,modulePath:item.modulePath,generated:item.generated===true}));
  assert.deepEqual(inventory.map(row=>row.fullName),predecessor.declarationNames);
  assert.equal(new Set(inventory.map(row=>row.fullName)).size,106);
  const {emitRust} = await import('../../js/src/translation/emit-rust.js');
  const {emitJavaScript} = await import('../../js/src/translation/emit-javascript.js');
  const emitted = emitRust(program,{arrayConsumption:true});
  const copyOwners = consumingOwners(copied,inventory), emittedOwners = consumingOwners(emitted,inventory);
  for(const field of ['mappings','assumptions','encodings','preludes']) assert.deepEqual(emitted[field],copied[field],
    'consumption retains the entire actual '+field);
  const owners = inventory.map((owner,index) => {
    assert.deepEqual(emittedOwners[index].header,copyOwners[index].header,'unchanged complete owner ABI: '+owner.fullName);
    return {...owner,copy:copyOwners[index],consuming:emittedOwners[index]};
  });
  const observedTotals = {copy:totals(copyOwners),consuming:totals(emittedOwners)};
  assert.ok(observedTotals.consuming.cloneCalls < observedTotals.copy.cloneCalls,
    'the selected option removes actual clone calls rather than only declaring a profile');
  assert.equal(observedTotals.consuming.appendCalls,observedTotals.copy.appendCalls);
  const {encodeTypedProgramLinks,decodeTypedProgramLinks} = await import('../../js/src/typed-program-links.js');
  const {recheckLinkedTypedProgram} = await import('../../js/src/typed-program-checked-projection.js');
  const {LinkNetwork} = await import('../../js/src/network.js');
  const {encodeLinkNetworkBinary,decodeLinkNetworkBinary} = await import('../../js/src/link-network-binary.js');
  const packet = encodeTypedProgramLinks(program), recordCount = packet.network.links().length;
  assert.ok(Number.isSafeInteger(recordCount)&&recordCount>0&&recordCount<=maximumRecords);
  const binaryLimits = {recordLimit:recordCount,byteLimit:maximumArchiveBytes};
  const text = packet.network.toLino();assert.ok(Buffer.byteLength(text)<=maximumArchiveBytes);
  const binary = encodeLinkNetworkBinary(packet.network,binaryLimits);assert.ok(binary.byteLength<=maximumArchiveBytes);
  const replays = [];
  for(const format of ['text','binary']) {
    const network = format==='text' ? LinkNetwork.fromLino(text) : decodeLinkNetworkBinary(binary,binaryLimits);
    assert.equal(network.links().length,recordCount,'every actual archive record survives '+format);
    const restored = recheckLinkedTypedProgram(decodeTypedProgramLinks(network,packet.root),{wholeProgram:true});
    assert.deepEqual(restored.items.map(row=>row.fullName),program.items.map(row=>row.fullName));
    assert.deepEqual([...restored.declarations.keys()],[...program.declarations.keys()]);
    const replay = emitRust(restored,{arrayConsumption:true});
    for(const field of ['text','definitions','preludes','mappings','assumptions','encodings']) assert.deepEqual(replay[field],emitted[field],
      'complete source-free '+field+' agrees after '+format);
    assert.deepEqual(consumingOwners(replay,inventory),emittedOwners);
    replays.push({format,ownerCount:restored.declarations.size,emittedBytes:Buffer.byteLength(replay.text),emittedSha256:sha256(replay.text)});
  }
  const artifact = {version:1,model:prepared.currentArtifact.model,profile:'consuming',options:{arrayConsumption:true},
    source:{path:'experiments/original-link-complete-table-projection/raw-rules.mjs',
      bytes:Buffer.byteLength(prepared.source),sha256:sha256(prepared.source)},
    predecessor,decisionSources:decisionSources(readFileSync),definitionCount:106,
    javaScriptSha256:sha256(emitJavaScript(program).text),
    declarationNames:[...predecessor.declarationNames],owners,totals:observedTotals,
    assumptions:emitted.assumptions,encodings:emitted.encodings,mappings:emitted.mappings,
    preludeHashes:emitted.preludes.map(source=>({bytes:Buffer.byteLength(source),sha256:sha256(source)})),
    archive:{root:packet.root.asU64().toString(),recordCount,maximumRecords,binaryLimits,
      textBytes:Buffer.byteLength(text),textSha256:sha256(text),binaryBytes:binary.byteLength,binarySha256:sha256(binary)},
    replays,sourceBytes:Buffer.byteLength(emitted.text),sourceSha256:sha256(emitted.text),completeRust:emitted.text,
    nativeCompilation:false,nativeExecution:false,
    boundary:'Whole checked source, all106owners and both complete source-free typed archives. Tokenized clone/append sites observe emitted syntax only; the existing emitter performs its actual checked final-use decisions. No internal proof-node inventory, native timing or completion claim.',
    predecessorConservation:structuredClone(ORIGINAL_LINK_RAW_CONSUMING_SUCCESSOR)};
  verifyOriginalLinkRawConsumingConservation(artifact);
  if(check)checkProjectData(ORIGINAL_LINK_RAW_CONSUMING_ARTIFACT,artifact);
  return {artifact,rust:emitted,profile:{profile:'consuming',sourceBytes:artifact.sourceBytes,sourceSha256:artifact.sourceSha256,
    historicalSourceSha256:predecessor.historicalSourceSha256,currentSourceSha256:predecessor.currentSourceSha256,
    sourceArtifact,declarationNames:artifact.declarationNames,definitionCount:106,
    proof:{artifactSha256:sha256(structuredDataToLino(artifact)),predecessor,
      predecessorConservation:artifact.predecessorConservation,decisionSources:artifact.decisionSources,
      options:artifact.options,archive:artifact.archive,replays:artifact.replays,tokenizedSites:artifact.totals,
      boundary:'Actual emitted token sites and existing checked final-use decision source pins; not an internal graph-fact inventory or native performance observation.'}}};
}

export async function generateOriginalLinkRawConsumingEmission(check=false) {
  const [{parseJavaScript},{checkProgram},{emitRust}] = await Promise.all([
    import('../../js/src/translation/javascript.js'),import('../../js/src/translation/check.js'),import('../../js/src/translation/emit-rust.js'),
  ]);
  const prepared = prepareCurrentOriginalLinkRawEmission({readProjectData,parseJavaScript,checkProgram,emitRust});
  const result = await prepareOriginalLinkRawConsumingEmission(prepared,{check});
  if(!check)writeProjectData(ORIGINAL_LINK_RAW_CONSUMING_ARTIFACT,result.artifact);
  assert.deepEqual(readOriginalLinkRawConsumingProfile(result.artifact.predecessor),result.profile);
  return result;
}

if(process.argv[1]===fileURLToPath(import.meta.url)) {
  const args=process.argv.slice(2);
  assert.ok(args.includes('--native')&&new Set(args).size===args.length&&args.every(arg=>['--native','--check'].includes(arg)),
    'explicit --native is required; only read-only --check is also supported');
  const {artifact}=await generateOriginalLinkRawConsumingEmission(args.includes('--check'));
  console.log('Consuming raw source emission: '+artifact.definitionCount+' whole owners; '+artifact.sourceBytes
    +' bytes; SHA256 '+artifact.sourceSha256+'; native execution not selected');
}
