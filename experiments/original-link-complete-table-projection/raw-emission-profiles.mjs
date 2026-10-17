// Separate current-emission proof. Historical producer/data/schema bytes stay exact.
// Importing this verifier performs no file reads, translation, CST or processes.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {structuredDataToLino} from '../../js/src/structured-data-links.js';

const sha256 = value => createHash('sha256').update(value).digest('hex');
const freeze = value => { if(value && typeof value==='object'){Object.values(value).forEach(freeze);Object.freeze(value);}return value; };
export const ORIGINAL_LINK_RAW_CURRENT_ARTIFACT = new URL('./data/raw-current-native-definitions.lino', import.meta.url);
export const ORIGINAL_LINK_RAW_HISTORICAL_ARTIFACT = new URL('./data/raw-native-definitions.lino', import.meta.url);
export const ORIGINAL_LINK_RAW_EMISSION_BASELINE = freeze({
  artifactSha256: 'd4b2a0c42bba5ae8f29137ead5bc264b633fcb6fad953b13d2adb5fed2d9d6cd',
  sourceBytes: 40766, sourceSha256: 'e4bafbec028815e973f2d3c0c683db6347d77d60b64548845f7669d01a0a35c5',
  historicalBytes: 351746, historicalSha256: '24db774177e047de0c0ea22adb2464e4e8481d03547f5a810b2d9fe317fb3046',
  currentBytes: 351680, currentSha256: '54388dca2da4b7f1a5bcbb7308d31d5f049997898b89a4bf940b2f7bb664aec5',
  definitionCount: 106, unchangedDefinitionCount: 103,
  introducingCommit: '7bb7fe07df8a9de54dc4f987bc270cc44f09113d',
  interpretation: 'deferred equality concrete facet capture 1',
  rootHeader: 'pub fn check_original_link_raw_shape(root_high: f64, root_low: f64, columns: Vec<Vec<f64>>) -> Vec<f64> {',
  concreteFacetPremise: 'const facetCode=fieldCodes[0]??-1;',
});
export const ORIGINAL_LINK_RAW_EMISSION_PRODUCERS = freeze([
  {file:'js/src/translation/javascript-surface-rules.js',bytes:8574,sha256:'794d918304dac11b384a3d29a1fd83235cacd51cd2cfe19ca630cfa2833ac40c'},
  {file:'js/src/translation/javascript-infer.js',bytes:62514,sha256:'55e1189678b62bdcb5542d4fcaa9db0aa33745ad7c13c3e8f7b0aa1ae2e66e62'},
]);
// These exact line pairs verify conservation only. Actual Rust is always made
// by the existing full parse/check/emit APIs, never by applying these edits.
export const ORIGINAL_LINK_RAW_EMISSION_CHANGES = freeze([
  {
    "owner": "checkOriginalLinkRawShape",
    "declarationIndex": 20,
    "from": "                                                                                                                                                                                                                                                                                        let ml_s26 = crate::ml_check_original_link_raw_shape_loop48(limits.clone(), parents.clone(), children.clone(), field_parents.clone(), field_children.clone(), field_labels.clone(), body, Some(facet_code), pending.clone(), seen.clone(), facet, count, steps);\n",
    "to": "                                                                                                                                                                                                                                                                                        let ml_s26 = crate::ml_check_original_link_raw_shape_loop48(limits.clone(), parents.clone(), children.clone(), field_parents.clone(), field_children.clone(), field_labels.clone(), body, facet_code, pending.clone(), seen.clone(), facet, count, steps);\n"
  },
  {
    "owner": "ml_checkOriginalLinkRawShape_loop54",
    "declarationIndex": 91,
    "from": "pub fn ml_check_original_link_raw_shape_loop54(mut field_parents: Vec<f64>, mut field_children: Vec<f64>, mut field_labels: Vec<f64>, mut body: f64, mut facet_code: Option<f64>, mut facet: f64, mut count: f64, mut node: f64, mut index: f64) -> crate::MlCheckOriginalLinkRawShapeLoop54Result {\n",
    "to": "pub fn ml_check_original_link_raw_shape_loop54(mut field_parents: Vec<f64>, mut field_children: Vec<f64>, mut field_labels: Vec<f64>, mut body: f64, mut facet_code: f64, mut facet: f64, mut count: f64, mut node: f64, mut index: f64) -> crate::MlCheckOriginalLinkRawShapeLoop54Result {\n"
  },
  {
    "owner": "ml_checkOriginalLinkRawShape_loop54",
    "declarationIndex": 91,
    "from": "            if (match (crate::ml_array::optional_at(&field_parents, crate::ml_array::number_index(index)), node) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false } && match (crate::ml_array::optional_at(&field_labels, crate::ml_array::number_index(index)), facet_code.clone()) { (Some(ml_left), Some(ml_right)) => ml_left == ml_right, (None, None) => true, _ => false }) {\n",
    "to": "            if (match (crate::ml_array::optional_at(&field_parents, crate::ml_array::number_index(index)), node) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false } && match (crate::ml_array::optional_at(&field_labels, crate::ml_array::number_index(index)), facet_code) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }) {\n"
  },
  {
    "owner": "ml_checkOriginalLinkRawShape_loop48",
    "declarationIndex": 92,
    "from": "pub fn ml_check_original_link_raw_shape_loop48(mut limits: Vec<f64>, mut parents: Vec<f64>, mut children: Vec<f64>, mut field_parents: Vec<f64>, mut field_children: Vec<f64>, mut field_labels: Vec<f64>, mut body: f64, mut facet_code: Option<f64>, mut pending: Vec<f64>, mut seen: Vec<f64>, mut facet: f64, mut count: f64, mut steps: f64) -> crate::MlCheckOriginalLinkRawShapeLoop48Result {\n",
    "to": "pub fn ml_check_original_link_raw_shape_loop48(mut limits: Vec<f64>, mut parents: Vec<f64>, mut children: Vec<f64>, mut field_parents: Vec<f64>, mut field_children: Vec<f64>, mut field_labels: Vec<f64>, mut body: f64, mut facet_code: f64, mut pending: Vec<f64>, mut seen: Vec<f64>, mut facet: f64, mut count: f64, mut steps: f64) -> crate::MlCheckOriginalLinkRawShapeLoop48Result {\n"
  },
  {
    "owner": "ml_checkOriginalLinkRawShape_loop48",
    "declarationIndex": 92,
    "from": "                                            let ml_s45 = crate::ml_check_original_link_raw_shape_loop54(field_parents.clone(), field_children.clone(), field_labels.clone(), body, facet_code.clone(), facet, count, node, index);\n",
    "to": "                                            let ml_s45 = crate::ml_check_original_link_raw_shape_loop54(field_parents.clone(), field_children.clone(), field_labels.clone(), body, facet_code, facet, count, node, index);\n"
  }
] );

export function readOriginalLinkRawEmissionProducerSources(read=readFileSync) {
  return Object.fromEntries(ORIGINAL_LINK_RAW_EMISSION_PRODUCERS.map(row=>[row.file,read(new URL('../../'+row.file,import.meta.url))]));
}
function producerDescriptors(producerSources) {
  assert.deepEqual(Object.keys(producerSources),ORIGINAL_LINK_RAW_EMISSION_PRODUCERS.map(row=>row.file),'exact inference producer inventory');
  return ORIGINAL_LINK_RAW_EMISSION_PRODUCERS.map(row=>{
    const bytes=producerSources[row.file];assert.ok(Buffer.isBuffer(bytes),'actual producer bytes are supplied');
    assert.equal(bytes.length,row.bytes,'inference producer byte count changed');
    assert.equal(sha256(bytes),row.sha256,'the selected inference producer changed');return {...row};
  });
}
function replaceExactlyOnce(source,from,to) {
  assert.equal(source.split(from).length,2,'each exact emitted line has one original owner');
  assert.equal(source.split(to).length,1,'a replacement line is not already present');
  return source.replace(from,()=>to);
}
function verifyHistorical(historicalArtifact,source) {
  const baseline=ORIGINAL_LINK_RAW_EMISSION_BASELINE;
  assert.equal(sha256(structuredDataToLino(historicalArtifact)),baseline.artifactSha256,'the whole immutable historical artifact changed');
  assert.equal(Buffer.byteLength(source),baseline.sourceBytes);assert.equal(sha256(source),baseline.sourceSha256,'the entire original raw source changed');
  assert.equal(source.split(baseline.concreteFacetPremise).length,2,'the actual source contains the unique concrete facet coalescing premise');
  assert.equal(historicalArtifact.version,1);assert.equal(historicalArtifact.model,'original selected Rust text graph shape component 1');
  assert.equal(historicalArtifact.sourceSha256,baseline.sourceSha256);
  assert.equal(historicalArtifact.declarationNames.length,baseline.definitionCount);
  assert.equal(historicalArtifact.declarations.length,baseline.definitionCount);
  assert.equal(Buffer.byteLength(historicalArtifact.completeRust),baseline.historicalBytes);
  assert.equal(sha256(historicalArtifact.completeRust),baseline.historicalSha256);
  assert.equal(historicalArtifact.completeRust.split(baseline.rootHeader).length,2,'the historical public root ABI stays exact');
}
function profileDescriptor(producerSources) {
  const baseline=ORIGINAL_LINK_RAW_EMISSION_BASELINE;
  return {version:1,name:'current',interpretation:baseline.interpretation,introducingCommit:baseline.introducingCommit,
    historicalArtifactSha256:baseline.artifactSha256,historicalSourceBytes:baseline.historicalBytes,historicalSourceSha256:baseline.historicalSha256,
    currentSourceBytes:baseline.currentBytes,currentSourceSha256:baseline.currentSha256,
    concreteFacetPremise:baseline.concreteFacetPremise,producerSources:producerDescriptors(producerSources),
    changedOwners:[...new Set(ORIGINAL_LINK_RAW_EMISSION_CHANGES.map(row=>row.owner))],changedLines:ORIGINAL_LINK_RAW_EMISSION_CHANGES.length,
    unchangedDefinitions:baseline.unchangedDefinitionCount,definitionCount:baseline.definitionCount,
    scope:'Complete automatic current emission with a finite historical inverse; no native execution or new graph capability evidence.'};
}
export function verifyOriginalLinkRawEmissionProfile({historicalArtifact,currentArtifact,source,producerSources}) {
  verifyHistorical(historicalArtifact,source);
  const baseline=ORIGINAL_LINK_RAW_EMISSION_BASELINE;
  assert.deepEqual(Object.keys(currentArtifact),[...Object.keys(historicalArtifact),'profile'],'only the explicit emission profile is added');
  assert.deepEqual(currentArtifact.profile,profileDescriptor(producerSources),'the current profile and causal source applicability are exact');
  for(const field of ['version','model','sourceSha256','declarationNames'])assert.deepEqual(currentArtifact[field],historicalArtifact[field],'original artifact metadata/order is conserved: '+field);
  assert.equal(currentArtifact.declarations.length,baseline.definitionCount);
  let unchanged=0;
  for(let index=0;index<historicalArtifact.declarations.length;index++){
    const changes=ORIGINAL_LINK_RAW_EMISSION_CHANGES.filter(row=>row.declarationIndex===index);
    if(!changes.length){assert.equal(currentArtifact.declarations[index],historicalArtifact.declarations[index],'an unaffected definition changed');unchanged++;continue;}
    let forward=historicalArtifact.declarations[index];
    for(const change of changes){assert.equal(historicalArtifact.declarationNames[index],change.owner,'the exact changed declaration owner is conserved');forward=replaceExactlyOnce(forward,change.from,change.to);}
    assert.equal(currentArtifact.declarations[index],forward,'only the declared emitted lines may differ');
    let inverse=currentArtifact.declarations[index];for(const change of [...changes].reverse())inverse=replaceExactlyOnce(inverse,change.to,change.from);
    assert.equal(inverse,historicalArtifact.declarations[index],'every changed definition has the exact complete historical inverse');
  }
  assert.equal(unchanged,baseline.unchangedDefinitionCount);
  let forward=historicalArtifact.completeRust;for(const change of ORIGINAL_LINK_RAW_EMISSION_CHANGES)forward=replaceExactlyOnce(forward,change.from,change.to);
  assert.equal(currentArtifact.completeRust,forward,'the entire actual source differs only by the five recorded lines');
  let inverse=currentArtifact.completeRust;for(const change of [...ORIGINAL_LINK_RAW_EMISSION_CHANGES].reverse())inverse=replaceExactlyOnce(inverse,change.to,change.from);
  assert.equal(inverse,historicalArtifact.completeRust,'the complete current source restores every historical byte');
  assert.equal(Buffer.byteLength(currentArtifact.completeRust),baseline.currentBytes);
  assert.equal(sha256(currentArtifact.completeRust),baseline.currentSha256,'the full current automatic source hash is exact');
  assert.equal(currentArtifact.completeRust.split(baseline.rootHeader).length,2,'the selected public root ABI stays exact');
  return freeze({profile:'current',interpretation:baseline.interpretation,introducingCommit:baseline.introducingCommit,
    historicalArtifactSha256:baseline.artifactSha256,historicalSourceSha256:baseline.historicalSha256,currentSourceSha256:baseline.currentSha256,
    changedOwners:currentArtifact.profile.changedOwners,changedLines:5,unchangedDefinitions:unchanged,definitionCount:baseline.definitionCount,
    producerSources:currentArtifact.profile.producerSources});
}
export function createOriginalLinkRawCurrentArtifact({historicalArtifact,source,emission,declarationNames,producerSources}) {
  verifyHistorical(historicalArtifact,source);
  const artifact={version:historicalArtifact.version,model:historicalArtifact.model,sourceSha256:sha256(source),
    declarationNames:[...declarationNames],declarations:[...emission.definitions],completeRust:emission.text,profile:profileDescriptor(producerSources)};
  verifyOriginalLinkRawEmissionProfile({historicalArtifact,currentArtifact:artifact,source,producerSources});return artifact;
}
export function readOriginalLinkRawEmissionProfile(profile='current',inputs) {
  assert.ok(profile==='historical'||profile==='current','unknown raw emission profile');
  const proof=verifyOriginalLinkRawEmissionProfile(inputs), artifact=profile==='current'?inputs.currentArtifact:inputs.historicalArtifact;
  const file=profile==='current'?'data/raw-current-native-definitions.lino':'data/raw-native-definitions.lino';
  return freeze({profile,source:artifact.completeRust,sourceBytes:Buffer.byteLength(artifact.completeRust),sourceSha256:sha256(artifact.completeRust),
    historicalSourceSha256:proof.historicalSourceSha256,currentSourceSha256:proof.currentSourceSha256,
    sourceArtifact:file,declarationNames:artifact.declarationNames,definitionCount:artifact.declarations.length,proof});
}
