// Replay the reviewed identity/field transport around the automatically
// translated Boolean declaration-value policy. No native semantics are written.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {existsSync,readFileSync,writeFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {readProjectData} from './project-data.mjs';
import {verifyGitArtifactCertificate} from './git-object-certificate.mjs';
const root=new URL('../../',import.meta.url);
const revision="d01acb2e642e65781369c2c0e2d6a198f7e051a3";
const hosts=[
  {
    "path": "rust/src/formal_concept_definition.rs",
    "predecessorBytes": 15637,
    "predecessorSha256": "b45d36d28147ca58579098d02588d2ce95f2c53cb98d715528348629a917fa01",
    "currentBytes": 18489,
    "currentSha256": "9850a48ca996e62e76e1f976894fbe0107a8badb7031c03b10511c44d9fed5e6"
  },
  {
    "path": "rust/src/formal_function_links.rs",
    "predecessorBytes": 14608,
    "predecessorSha256": "89d526ad76dcae2f78e073cbf9658ba5449a67cb107d4ad6aecf4ab383b8bd07",
    "currentBytes": 16507,
    "currentSha256": "9a7c908a4d81c5a5a8f40ab212ab559a1d76ed3a08f98b2d0bd65074d6edb6d1"
  },
  {
    "path": "rust/src/shared_numeric_function_model.rs",
    "predecessorBytes": 22981,
    "predecessorSha256": "3ca1c2c1c4f5b467ee55503b43995944ee93acf4427467ec39b96bb16c9a83d2",
    "currentBytes": 26599,
    "currentSha256": "4d7edc5f126877eaa272edabe6c2bd10dc38771bb4cb18c244434fdf98ee93ea"
  }
];
const decisionSource={
  "path": "js/src/boolean-declaration-value-rules.js",
  "sha256": "fd2a568833e7d0e607bd86400781f17b37b64963e7ec3fc63591b15d836938a9"
};
const registrations=[{path:'rust/src/translation/mod.rs',declaration:'pub(crate) mod boolean_declaration_value_rules;'}];
const digest=source=>createHash('sha256').update(source).digest('hex');
const hostOwnershipCorrections = new Map([
  ['rust/src/formal_concept_definition.rs', {
    sha256: 'da2042f377343f827df455003811385c212de591695926482ce53fbd1557e977',
    changes: [
      ['exclusions.insert((parent, relation.label.clone()));', 'exclusions.insert((parent, relation.label));'],
      [`// Private graph constructor for the additive declaration-value entry API.
// The public options type and original wrapper constructor remain unchanged.
pub(crate) fn encode_formal_function_declaration_context(`,
      `/// Build the declaration-value graph for the crate's entry API.
pub fn encode_formal_function_declaration_context(`],
    ],
  }],
  ['rust/src/shared_numeric_function_model.rs', {
    sha256: '10e503f648eca651a73929b1a8628d59fbbd25d3d9d13c3a0c7b47e13595d974',
    changes: [['.flat_map(|link| link.references())', '.flat_map(crate::link_network::Link::references)']],
  }],
]);
function sourcePin(source,bytes,sha256){
  assert.equal(typeof source,'string');assert.equal(Buffer.from(source).toString('utf8'),source);
  assert.equal(Buffer.byteLength(source),bytes);assert.equal(digest(source),sha256);
}

/** Verify complete certified predecessors, reviewed successors and exact inverses.
 * No repository state, subprocess, native parser or native compiler is read. */
export function verifyBooleanDeclarationValueHosts(archive){
  assert.equal(archive.schemaVersion,1);
  assert.equal(archive.model,'Boolean function declaration value host replay 1');
  assert.equal(archive.revision,revision);
  assert.equal(archive.coordinateUnit,'UTF-16 code units');
  assert.deepEqual(archive.hosts.map(row=>row.path),hosts.map(row=>row.path));
  assert.deepEqual(archive.decisionSource,decisionSource);
  assert.deepEqual(archive.registrations,registrations);
  const files=[];
  for(const [index,row]of archive.hosts.entries()){
    const expected=hosts[index];
    sourcePin(row.source,expected.predecessorBytes,expected.predecessorSha256);
    verifyGitArtifactCertificate(row.certificate,Buffer.from(row.source),{revision,path:row.path});
    assert.equal(row.currentBytes,expected.currentBytes);assert.equal(row.currentSha256,expected.currentSha256);
    assert.ok(Array.isArray(row.edits)&&row.edits.length>0);
    let current=row.source;
    for(const edit of row.edits){
      assert.ok(Number.isSafeInteger(edit.start)&&edit.start>=0&&edit.start<=current.length);
      assert.equal(typeof edit.removed,'string');assert.equal(typeof edit.inserted,'string');
      assert.notEqual(edit.removed,edit.inserted);
      assert.equal(current.slice(edit.start,edit.start+edit.removed.length),edit.removed);
      current=current.slice(0,edit.start)+edit.inserted+current.slice(edit.start+edit.removed.length);
    }
    sourcePin(current,expected.currentBytes,expected.currentSha256);
    let inverse=current;
    for(const edit of archive.hosts[index].edits.toReversed()){
      assert.equal(inverse.slice(edit.start,edit.start+edit.inserted.length),edit.inserted);
      inverse=inverse.slice(0,edit.start)+edit.removed+inverse.slice(edit.start+edit.inserted.length);
    }
    assert.equal(inverse,row.source,'every original source byte and old wrapper branch remains recoverable');
    files.push({path:row.path,source:current,predecessor:row.source});
  }
  return files;
}

/** Apply the exact native host ownership corrections from CI job 138183525811. */
function correctBooleanDeclarationValueHost({path,source}) {
  const correction = hostOwnershipCorrections.get(path);
  if (!correction) return source;
  for (const [previous, current] of correction.changes) {
    assert.equal(source.split(previous).length, 2);
    source = source.replace(previous, current);
  }
  assert.equal(digest(source), correction.sha256);
  return source;
}

/** Rebuild only three recorded host transports; actual native execution is CI-only. */
export function generateBooleanDeclarationValueHosts({check=false}={}){
  const archive=readProjectData(new URL('parity/self-translation/boolean-declaration-value-hosts.lino',root));
  const files=verifyBooleanDeclarationValueHosts(archive);
  assert.equal(digest(readFileSync(new URL(decisionSource.path,root))),decisionSource.sha256);
  for(const row of registrations)assert.equal(readFileSync(new URL(row.path,root),'utf8').split(row.declaration).length,2);
  // Validate every destination before the first possible source write.
  const currentFiles = files.map(row => ({...row, current: correctBooleanDeclarationValueHost(row)}));
  for(const row of currentFiles){
    const file=new URL(row.path,root);
    if(check)assert.equal(readFileSync(file,'utf8'),row.current,row.path);
    else if(existsSync(file)){
      const actual=readFileSync(file,'utf8');
      assert.ok(actual===row.current||actual===row.source||actual===row.predecessor,'unknown host source '+row.path);
    }
  }
  if(!check)for(const row of currentFiles)writeFileSync(new URL(row.path,root),row.current);
  return {archive,files};
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
  const args=process.argv.slice(2);assert.ok(args.length<=1&&args.every(arg=>arg==='--check'));
  const result=generateBooleanDeclarationValueHosts({check:args.includes('--check')});
  console.log(JSON.stringify({revision,hosts:result.files.map(row=>row.path),nativeCompilation:false,nativeExecution:false}));
}
