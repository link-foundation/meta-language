// Source-only planner. No Cargo/oracle invocation and no writes.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {execFileSync} from 'node:child_process';
import {lstatSync,readFileSync,realpathSync} from 'node:fs';
import {resolve,relative,sep} from 'node:path';
import {fileURLToPath,pathToFileURL} from 'node:url';
import {overlayPolicy as policy} from './policy.mjs';
import {checkRunnerPublication} from './generate-publication.mjs';
import {buildIntegration} from '../bridge/generate.mjs';
import {verifyPublication} from '../wire/publication.mjs';
import {applyFrozenOutcomeProducerCorrection} from '../diagnostic-order/source-view.mjs';
import {diagnosticOrderDecorator} from '../diagnostic-order/decorator.mjs';

export const sha256=bytes=>createHash('sha256').update(bytes).digest('hex');
export const within=(root,file)=>{const path=relative(root,file);return path!==''&&!path.startsWith('..'+sep)&&path!=='..'&&!path.startsWith(sep);};
export function git(repository,args) {
  return execFileSync('git',['-C',repository,...args],{shell:false,encoding:'utf8',timeout:15000,maxBuffer:8*1024*1024});
}
export function bytesAt(directory,file,maximum=policy.maximumFileBytes) {
  const path=resolve(directory,file);assert.ok(within(directory,path),'owned input path');
  const stat=lstatSync(path);assert.ok(stat.isFile()&&!stat.isSymbolicLink()&&stat.size<=maximum,'bounded regular input: '+file);
  assert.ok(within(directory,realpathSync(path)),'input cannot escape its owner');
  return readFileSync(path);
}

/** Compile the named frozen Git snapshot, retaining separate current-input
 * pins so publication metadata cannot silently replace a reviewed source.
 */
export function frozenSourceBytes(repository,file,{readWorking=bytesAt,readBlob}={}) {
  assert.match(file.gitObject,/^[0-9a-f]{40}$/u,'exact frozen Git object');
  const current=readWorking(repository,file.path),workingSha256=sha256(current);
  if(file.workingSha256)assert.equal(workingSha256,file.workingSha256,'production input changed after planning: '+file.path);
  const object=bytes=>createHash('sha1').update('blob '+bytes.length+'\0').update(bytes).digest('hex');
  const bytes=object(current)===file.gitObject?current:(readBlob?readBlob(repository,file.gitObject):execFileSync('git',
    ['-C',repository,'cat-file','blob',file.gitObject],{shell:false,timeout:15000,maxBuffer:policy.maximumFileBytes}));
  assert.ok(bytes.length<=policy.maximumFileBytes,'bounded frozen source');
  assert.equal(object(bytes),file.gitObject,'frozen Git blob identity');
  if(file.sha256)assert.equal(sha256(bytes),file.sha256,'reviewed snapshot bytes');
  return{bytes,workingSha256,workingBytes:current.length};
}

/** Reproduce ONLY the already CST-qualified source move, from receipt spans.
 * No new Number semantics, native parser or hand-written declarations.
 */
export function verifyParentInverse(original,proposed,receipt) {
  assert.equal(sha256(original),receipt.parent.originalSha256);
  assert.equal(sha256(proposed),receipt.parent.proposedSha256);
  const {start,end,sha256:expected}=receipt.parent.sourceFunction;
  assert.ok(Number.isSafeInteger(start)&&Number.isSafeInteger(end)&&start>=0&&end>start&&end<=original.length);
  const sourceFunction=original.subarray(start,end).toString('utf8');assert.equal(sha256(sourceFunction),expected);
  const prefix='        Value::Number(number) => {\n',suffix='\n        }';
  const begin=sourceFunction.indexOf(prefix),finish=sourceFunction.indexOf('\n        Value::String(text) =>',begin);
  assert.ok(begin>=0&&finish>begin&&sourceFunction.indexOf(prefix,begin+1)===-1);
  const arm=sourceFunction.slice(begin,finish);assert.ok(arm.endsWith(suffix));
  assert.equal(sha256(arm),receipt.parent.originalNumberArmSha256);
  const lines=arm.slice(prefix.length,-suffix.length).split('\n');
  assert.ok(lines.every(line=>line===''||line.startsWith('            ')));
  const body=lines.map(line=>line.slice(12)).join('\n');
  const tail='StructuredDataValue::Number(value)';assert.ok(body.endsWith(tail));
  const helper='fn exact_json_number(number: &Number) -> Result<f64, String> {\n'
    +(body.slice(0,-tail.length)+'Ok(value)').split('\n').map(line=>'    '+line).join('\n')+'\n}\n\n';
  assert.equal(sha256(helper),receipt.parent.helperSha256);
  const module='mod grammar_outcome;\npub use grammar_outcome::encode_grammar_parse_outcome_links;\n\n';
  const call='        Value::Number(number) => StructuredDataValue::Number(exact_json_number(number)?),';
  let restored=proposed.toString('utf8');
  for(const part of [module,helper,call]) assert.equal(restored.split(part).length,2,'one exact inverse source operation');
  restored=restored.replace(module,'').replace(helper,'').replace(call,arm);
  assert.deepEqual(Buffer.from(restored),original,'whole original parent inverse');
}

/** Authenticate every selected tracked crate file against the frozen Git tree.
 * No ignored target/cache/workload files, symlinks, or hardlinks are copied.
 */
export function crateInventory(repository) {
  const format=git(repository,['rev-parse','--show-object-format']).trim();assert.equal(format,'sha1','explicit current Git object profile');
  const listing=git(repository,['ls-tree','-r','-z',policy.baseCommit,'--','rust',...policy.auxiliarySources]);
  const entries=listing.split('\0').filter(Boolean);assert.ok(entries.length>0&&entries.length<=policy.maximumFiles);
  let total=0;
  const files=entries.map(entry=>{
    const match=/^(100644|100755) blob ([0-9a-f]{40})\t([^\0]+)$/u.exec(entry);assert.ok(match,'ordinary tracked source blob required');
    const [,mode,oid,path]=match;
    const {bytes,workingSha256,workingBytes}=frozenSourceBytes(repository,{path,gitObject:oid});
    assert.ok(path.startsWith('rust/')||policy.auxiliarySources.includes(path),'exact build-helper source closure');
    const blob=createHash('sha1').update('blob '+bytes.length+'\0').update(bytes).digest('hex');assert.equal(blob,oid,'unchanged pinned crate: '+path);
    assert.ok(total<=policy.maximumCrateBytes-bytes.length,'bounded total crate copy');total+=bytes.length;
    return {path,mode,gitObject:oid,bytes:bytes.length,sha256:sha256(bytes),workingSha256,workingBytes};
  });
  assert.ok(files.some(file=>file.path==='rust/Cargo.toml')&&files.some(file=>file.path==='rust/Cargo.lock'));
  assert.ok(policy.auxiliarySources.every(path=>files.some(file=>file.path===path)),'every declared JS build-helper source remains tracked');
  assert.ok(!files.some(file=>file.path===policy.child||file.path==='rust/tests/'+policy.testName+'.rs'),'overlay additions must be absent in the base');
  return {files,bytes:total};
}

export async function createOverlayPlan({repository=fileURLToPath(new URL('../../../../',import.meta.url)),candidateDirectory,
  probeDirectory=fileURLToPath(new URL('../probe/',import.meta.url))}={}) {
  repository=realpathSync(repository);if(candidateDirectory)candidateDirectory=realpathSync(candidateDirectory);probeDirectory=realpathSync(probeDirectory);
  const runnerPublication=checkRunnerPublication();
  const publication=verifyPublication();
  for(const [group,pins]of [['candidate',policy.candidatePins],['probe',policy.probePins]])for(const [file,expected]of Object.entries(pins)) {
    const original=publication.originalSources.find(source=>source.group===group&&source.file===file);
    assert.ok(original,'archival reviewed input: '+group+'/'+file);
    assert.equal(original.sha256,expected,'retained original reviewed input: '+group+'/'+file);
  }
  for(const file of ['oracle.mjs','native-outcome-probe.rs']) {
    const published=publication.files.find(source=>source.path==='probe/'+file);
    assert.ok(published,'published probe source: '+file);
    const bytes=bytesAt(probeDirectory,file);
    assert.equal(bytes.length,published.bytes,'selected actual probe source bytes: '+file);
    assert.equal(sha256(bytes),published.sha256,'selected actual probe source hash: '+file);
  }
  assert.equal(git(repository,['rev-parse',policy.baseCommit+'^{commit}']).trim(),policy.baseCommit);
  // Future commits may publish experiment files. Core source/dependency drift
  // is authenticated by predecessor pins; the compiled full crate stays frozen.
  // Current source pins are retained separately and checked again after execution.
  // Dependency locks stay identical for the shared installed JS build dependencies.
  git(repository,['diff','--exit-code','--quiet',policy.baseCommit,'--','js/package.json','js/package-lock.json']);
  const executionCommit=git(repository,['rev-parse','HEAD']).trim();
  const {readProjectData}=await import(pathToFileURL(resolve(repository,'js/scripts/project-data.mjs')));
  const generated=buildIntegration(repository);
  const receipt=candidateDirectory?readProjectData(resolve(candidateDirectory,'integration-receipt.lino')):generated.receipt;
  assert.deepEqual(receipt,generated.receipt,'fresh qualified bridge receipt');
  assert.deepEqual(receipt.predecessor.pins,receipt.predecessor.sourceViews.historicalPins,'frozen predecessors remain separate');
  for(const input of receipt.predecessor.sourceViews.currentPins) assert.equal(sha256(bytesAt(repository,input.file)),input.sha256,'current authenticated source: '+input.file);
  const originalProducer=diagnosticOrderDecorator.inputs.find(input=>input.path==='rust/src/grammar/feature_runtime/grammar_selection.rs').source;
  const producerCorrection=applyFrozenOutcomeProducerCorrection(originalProducer,diagnosticOrderDecorator);
  const original=bytesAt(repository,policy.parent),parent=candidateDirectory?bytesAt(candidateDirectory,'structured_data_links.proposed.rs'):Buffer.from(generated.parent.source);
  const child=candidateDirectory?bytesAt(candidateDirectory,'grammar_outcome.rs'):Buffer.from(generated.child);
  assert.deepEqual(parent,Buffer.from(generated.parent.source),'fresh mechanically generated parent');
  assert.deepEqual(child,Buffer.from(generated.child),'fresh mechanically generated child');
  verifyParentInverse(original,parent,receipt);assert.equal(sha256(child),receipt.child.sha256);
  const manifest=bytesAt(repository,'rust/Cargo.toml').toString('utf8');
  assert.ok(!/^\s*autotests\s*=\s*false\s*$/mu.test(manifest),'actual probe auto-discovery required');
  return {repository,candidateDirectory,probeDirectory,executionCommit,inventory:crateInventory(repository),receipt,parent,child,
    probe:bytesAt(probeDirectory,'native-outcome-probe.rs'),policy,runnerPublication,publication,
    nativeOraclePolicy:runnerPublication.nativeOraclePolicy,producerCorrection};
}
