// Read actual compiler records; declarations alone cannot select active features.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {posix} from 'node:path';
import {parseCargoLock} from '../../../../js/scripts/dependency-inventory.mjs';
import {readRustSourceItems} from '../../../../js/scripts/generate-private-output-test-model.mjs';
import {verifyPublication} from '../wire/publication.mjs';

const pin=value=>({bytes:Buffer.byteLength(value),sha256:createHash('sha256').update(value).digest('hex')});
const targetField='"targetUsizeBits": ::std::primitive::usize::BITS, ';
const probeName='borrowed_outcomes_match_fresh_javascript_values_and_independent_boundaries';
let retainedProbe;

/** One source-qualified additive host observation, after every original assertion. */
export function deriveOutcomeTargetProbe(original) {
  assert.ok(Buffer.isBuffer(original)&&original.length<=131072);
  const published=verifyPublication().files.filter(row=>row.path==='probe/native-outcome-probe.rs');
  assert.equal(published.length,1);assert.deepEqual(pin(original),{bytes:published[0].bytes,sha256:published[0].sha256},'the complete actual published probe remains the source authority');
  if(retainedProbe?.originalSha256===published[0].sha256)return structuredClone(retainedProbe);
  const parsed=readRustSourceItems(original),owners=parsed.items.filter(row=>row.name===probeName&&row.kind==='function_item');
  assert.equal(owners.length,1,'one complete original post-assertion probe owner');
  const owner=owners[0],marker=Buffer.from('let report = json!({"schemaVersion": 1, "profile": "private borrowed native outcome probe 1",');
  const start=original.indexOf(marker);
  assert.ok(start>=owner.node.start&&start+marker.length<=owner.node.end&&original.indexOf(marker,start+1)===-1);
  const macros=[],pending=[owner.node];
  while(pending.length){const node=pending.pop();if(node.term==='macro_invocation'&&node.start===start+Buffer.byteLength('let report = '))macros.push(node);for(const child of node.children)pending.push(child.node);}
  assert.equal(macros.length,1,'the observation belongs to the actual report macro');
  const insertion=start+Buffer.byteLength('let report = json!({');
  const source=Buffer.concat([original.subarray(0,insertion),Buffer.from(targetField),original.subarray(insertion)]);
  assert.deepEqual(Buffer.concat([source.subarray(0,insertion),source.subarray(insertion+Buffer.byteLength(targetField))]),original,'all original source bytes and assertions survive the exact inverse');
  const result={source:source.toString('utf8'),originalSha256:published[0].sha256,
    receipt:{schemaVersion:1,original:pin(original),derived:pin(source),owner:{name:probeName,start:owner.node.start,end:owner.node.end},
      insertion:{offset:insertion,source:targetField},boundary:'An actual primitive width observation after the unchanged native assertions; no DTO or number representation equality claim.'}};
  retainedProbe=structuredClone(result);return result;
}

/** Cargo's POSIX diagnostic words are data, never commands to execute. */
export function readCargoDiagnosticArguments(source) {
  assert.equal(typeof source,'string');assert.ok(Buffer.byteLength(source)<=65536);
  assert.ok(!/[\0\r\n]/u.test(source),'one diagnostic invocation line required');
  const words=[];let word='',active=false,quote=null;
  for(let index=0;index<source.length;index++){
    const character=source[index];
    assert.ok(character!=='\0'&&character!=='\n'&&character!=='\r','one diagnostic invocation line required');
    if(quote==="'"){if(character==="'")quote=null;else word+=character;continue;}
    if(character==='\\'&&quote!=="'"){
      assert.ok(index+1<source.length,'complete diagnostic escape required');
      const next=source[index+1];
      if(quote==='"'&&!['$','`','\\','"'].includes(next))word+='\\';
      else {word+=next;index++;}
      active=true;continue;
    }
    if(quote==='"'){if(character==='"')quote=null;else {assert.ok(!['$','`'].includes(character),'unescaped diagnostic expansion refuses');word+=character;}continue;}
    if(character==="'"||character==='"'){quote=character;active=true;continue;}
    if(/\s/u.test(character)){if(active){words.push(word);word='';active=false;}continue;}
    assert.ok(!['$','`',';','|','&','<','>','(',')'].includes(character),'unsupported diagnostic shell syntax');
    word+=character;active=true;
  }
  assert.equal(quote,null,'closed diagnostic quote required');if(active)words.push(word);
  assert.ok(words.length>0&&words.length<=4096);return words;
}

/** Bind a single absolute compiler command before querying its actual version. */
export function readOutcomeCompilerInvocation(stderr,selectedName,sourcePath) {
  assert.ok(typeof stderr==='string'&&stderr.isWellFormed()&&Buffer.byteLength(stderr)<=8*1024*1024);
  assert.match(selectedName,/^[a-z][a-z0-9_]{0,127}$/u);
  assert.ok(typeof sourcePath==='string'&&sourcePath.startsWith('/'));
  const clean=stderr.replace(/\x1b\[[0-?]*[ -/]*[@-~]/gu,'');
  const invocations=[];
  for(const line of clean.split(/\r?\n/u)){
    const match=/^\s*Running `(.+)`\s*$/u.exec(line);if(!match)continue;
    const arguments_=readCargoDiagnosticArguments(match[1]);
    const at=arguments_.indexOf('--crate-name');
    if(at<0||arguments_[at+1]!==selectedName)continue;
    assert.ok(arguments_[0].startsWith('/')&&posix.normalize(arguments_[0])===arguments_[0]&&/(?:^|\/)rustc$/u.test(arguments_[0]),'the selected diagnostic must invoke rustc directly');
    assert.ok(arguments_.includes('--test'),'selected test compiler invocation required');
    assert.ok(arguments_.includes(sourcePath)||arguments_.includes('tests/'+selectedName+'.rs'),'selected actual probe source argument required');
    const externals=[];
    for(let index=0;index<arguments_.length;index++)if(arguments_[index]==='--extern'&&arguments_[index+1]?.startsWith('serde_json='))externals.push(arguments_[index+1].slice('serde_json='.length));
    assert.equal(externals.length,1,'one actual linked JSON dependency argument');
    invocations.push({arguments:arguments_,linkedJsonArtifact:externals[0]});
  }
  assert.equal(invocations.length,1,'missing or ambiguous actual probe compiler invocation; artifact feature unions are insufficient');
  return invocations[0];
}

function readOutcomeToolVersion(text,tool) {
  assert.equal(typeof text,'string');assert.ok(Buffer.byteLength(text)<=16384);
  assert.ok(['cargo','rustc'].includes(tool));
  const match=new RegExp('^'+tool+' (1)\\.(0|[1-9][0-9]*)\\.(0|[1-9][0-9]*) \\([^\\r\\n]+\\)(?:\\r?\\n|$)','u').exec(text);
  assert.ok(match&&Number(match[2])>=90&&Number(match[2])<100,'uncovered '+(tool==='cargo'?'Cargo':'Rust compiler')+' version; supported stable range is >=1.90.0 <1.100.0');
  return match[1]+'.'+match[2]+'.'+match[3];
}

/** Select the linked dependency by its actual rustc --extern path, never a union. */
export function readOutcomeCompilerProfile({cargoVersion,rustcVersion,stdout,stderr,lockSource,probeName:selectedName,manifestPath,executable,usizeBits}) {
  const version=readOutcomeToolVersion(cargoVersion,'cargo'),compilerVersion=readOutcomeToolVersion(rustcVersion,'rustc');
  assert.match(selectedName,/^[a-z][a-z0-9_]{0,127}$/u);
  for(const path of [manifestPath,executable])assert.ok(typeof path==='string'&&path.startsWith('/')&&!path.includes('\0')&&posix.normalize(path)===path,'canonical absolute Linux artifact path required');
  for(const value of [stdout,stderr,lockSource])assert.ok(typeof value==='string'&&value.isWellFormed()&&Buffer.byteLength(value)<=8*1024*1024,'bounded complete compiler text required');
  assert.ok([32,64].includes(usizeBits),'uncovered observed native pointer width');
  const packages=parseCargoLock(lockSource).filter(row=>row.name==='serde_json');assert.equal(packages.length,1,'one exact locked JSON package');
  const locked=packages[0];assert.equal(locked.version,'1.0.151','uncovered JSON dependency version; supported range is >=1.0.151 <1.0.152');
  const records=[];
  for(let start=0;start<stdout.length;){
    let end=stdout.indexOf('\n',start);if(end<0)end=stdout.length;
    const line=stdout.slice(start,end).replace(/\r$/u,'');start=end+1;
    if(!line)continue;assert.ok(records.length<10000,'compiler record reservation exceeds the profile limit');
    records.push(JSON.parse(line));
  }
  assert.ok(records.length>0);assert.deepEqual(records.at(-1),{reason:'build-finished',success:true},'the exact compiler build finished successfully');
  const probes=records.filter(row=>row.reason==='compiler-artifact'&&row.target?.name===selectedName&&Array.isArray(row.target.kind)&&row.target.kind.length===1&&row.target.kind[0]==='test');
  assert.equal(probes.length,1,'one actual selected test artifact');const probe=probes[0];
  assert.equal(probe.manifest_path,manifestPath);assert.equal(probe.executable,executable);assert.ok(Array.isArray(probe.filenames)&&probe.filenames.includes(executable));
  assert.equal(probe.profile.test,true);
  assert.equal(probe.target.src_path,posix.join(posix.dirname(manifestPath),'tests',selectedName+'.rs'));
  const invocation=readOutcomeCompilerInvocation(stderr,selectedName,probe.target.src_path),identity=locked.source+'#serde_json@'+locked.version;
  const dependencies=records.filter(row=>row.reason==='compiler-artifact'&&row.package_id===identity&&row.target?.name==='serde_json'
    &&Array.isArray(row.target.kind)&&row.target.kind.length===1&&row.target.kind[0]==='lib'&&Array.isArray(row.filenames)&&row.filenames.includes(invocation.linkedJsonArtifact));
  assert.equal(dependencies.length,1,'one exact linked JSON compiler artifact');const dependency=dependencies[0];
  assert.equal(dependency.executable,null);assert.equal(dependency.profile.test,false);
  assert.equal(dependency.target.src_path,posix.join(posix.dirname(dependency.manifest_path),'src/lib.rs'));
  assert.ok(Array.isArray(dependency.features)&&dependency.features.every(value=>typeof value==='string')&&new Set(dependency.features).size===dependency.features.length);
  assert.ok(dependency.features.includes('preserve_order'),'the actual compiled mapping-order feature is required');
  return {schemaVersion:1,cargoVersion:version,rustcVersion:compilerVersion,usizeBits,versionBounds:{cargo:'>=1.90.0 <1.100.0',rustc:'>=1.90.0 <1.100.0',serdeJson:'>=1.0.151 <1.0.152'},
    inputs:{cargoVersion:pin(cargoVersion),rustcVersion:pin(rustcVersion),compilerStandardOutput:pin(stdout),compilerStandardError:pin(stderr),lockSource:pin(lockSource)},
    probe:structuredClone(probe),invocation,dependency:structuredClone(dependency),
    boundary:'Selected compiler artifact features and executed primitive width only. Native JSON number variants, DTO reconstruction, borrowed static resolution and physical CST ownership remain separate obligations.'};
}
