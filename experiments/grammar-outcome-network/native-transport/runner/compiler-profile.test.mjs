// Historical compiler bytes plus explicitly synthetic selector contracts.
// No compiler, native probe or requirement observation executes here.
import test from 'node:test';
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {readProjectData,serializeProjectData} from '../../../../js/scripts/project-data.mjs';
import {readRustSourceItems} from '../../../../js/scripts/generate-private-output-test-model.mjs';
import {deriveOutcomeTargetProbe,readCargoDiagnosticArguments,readOutcomeCompilerProfile} from './compiler-profile.mjs';

const file=new URL('./compiler-profile-fixture.lino',import.meta.url),fixture=readProjectData(file);
const rows=()=>fixture.records['compile.stdout.log'].split(/\r?\n/u).filter(Boolean).map(line=>JSON.parse(line));
const actualProbe=rows().find(row=>row.reason==='compiler-artifact'&&row.target?.name==='bulk95_outcome_probe');
const actualDependencies=rows().filter(row=>row.reason==='compiler-artifact'&&row.target?.name==='serde_json');
const quote=value=>"'"+value.replaceAll("'","'\\''")+"'";
const invocation=path=>'     Running `'+['/toolchain/bin/rustc','--crate-name','bulk95_outcome_probe','--test',actualProbe.target.src_path,'--extern','serde_json='+path].map(quote).join(' ')+'`\n';
const options=(index=1)=>({cargoVersion:fixture.records['cargo-version.stdout.log'],rustcVersion:fixture.records['rustc-version.stdout.log'],stdout:fixture.records['compile.stdout.log'],
  stderr:invocation(actualDependencies[index].filenames.find(path=>path.endsWith('.rlib'))),lockSource:fixture.lockSource,
  probeName:'bulk95_outcome_probe',manifestPath:actualProbe.manifest_path,executable:actualProbe.executable,usizeBits:64});
const output=records=>records.map(row=>JSON.stringify(row)).join('\n')+'\n';

test('complete actual historical compiler streams retain both distinct feature units and the failed-probe boundary',()=>{
  assert.equal(createHash('sha256').update(readFileSync(file)).digest('hex'),'bbe7e533b9fa356c2261d262deb900e01cdba5000a615e9488a11c428a8be514');
  const preceding=structuredClone(fixture);delete preceding.records['rustc-version.stdout.log'];
  assert.equal(createHash('sha256').update(serializeProjectData(preceding,file)).digest('hex'),'7354aaf4a6944b7b07cddc7af0796708079e7988976d05e9c05f963389a492f5','the complete original fixture survives the additive version stream');
  assert.equal(fixture.archive.artifactId,12147943919);assert.equal(fixture.archive.runId,39157701329);
  assert.equal(actualDependencies.length,2);
  assert.deepEqual(actualDependencies.map(row=>row.features),[
    ['default','indexmap','preserve_order','std'],['default','indexmap','preserve_order','std','unbounded_depth']
  ]);
  assert.match(fixture.scope,/original native probe failed/u);
  assert.throws(()=>readOutcomeCompilerProfile({...options(),stderr:fixture.records['compile.stderr.log']}),/compiler invocation/u);
});

test('synthetic invocation contracts select the exact linked artifact without combining dependency feature units',()=>{
  for(let index=0;index<2;index++){
    const input=options(index),profile=readOutcomeCompilerProfile(input);
    assert.deepEqual(profile.dependency,actualDependencies[index]);
    assert.deepEqual(profile.probe,actualProbe);assert.equal(profile.cargoVersion,'1.99.1');
    assert.equal(profile.rustcVersion,'1.99.1');assert.equal(profile.usizeBits,64);assert.equal(profile.dependency.features.includes('unbounded_depth'),index===1);
    assert.equal(profile.inputs.compilerStandardOutput.bytes,Buffer.byteLength(input.stdout));
    assert.deepEqual(profile.versionBounds,{cargo:'>=1.90.0 <1.100.0',rustc:'>=1.90.0 <1.100.0',serdeJson:'>=1.0.151 <1.0.152'});
  }
  const proxy=options();proxy.stderr=proxy.stderr.replace('/toolchain/bin/rustc','/home/runner/.cargo/bin/rustc');
  assert.equal(readOutcomeCompilerProfile(proxy).invocation.arguments[0],'/home/runner/.cargo/bin/rustc','the invoked compiler name is preserved for Rustup proxy dispatch');
});

test('active version contracts select explicit inclusive and exclusive bounds and reject uncovered profiles',()=>{
  for(const version of ['1.90.0','1.99.0','1.99.1','1.99.99'])assert.equal(readOutcomeCompilerProfile({...options(),cargoVersion:'cargo '+version+' (contract)\n'}).cargoVersion,version);
  for(const version of ['1.89.99','1.100.0','2.0.0','1.99.1-nightly','1.099.1'])assert.throws(()=>readOutcomeCompilerProfile({...options(),cargoVersion:'cargo '+version+' (contract)\n'}),/uncovered Cargo version/u);
  for(const version of ['1.90.0','1.99.1'])assert.equal(readOutcomeCompilerProfile({...options(),rustcVersion:'rustc '+version+' (contract)\n'}).rustcVersion,version);
  for(const version of ['1.89.99','1.100.0','1.99.1-nightly'])assert.throws(()=>readOutcomeCompilerProfile({...options(),rustcVersion:'rustc '+version+' (contract)\n'}),/uncovered Rust compiler version/u);
  for(const usizeBits of [16,128,0,null])assert.throws(()=>readOutcomeCompilerProfile({...options(),usizeBits}),/pointer width/u);
  assert.equal(readOutcomeCompilerProfile({...options(),usizeBits:32}).usizeBits,32);
  assert.throws(()=>readOutcomeCompilerProfile({...options(),lockSource:fixture.lockSource.replace('version = "1.0.151"','version = "1.0.152"')}),/uncovered JSON dependency version/u);
});

test('missing duplicated changed or unrelated artifacts and invocations cannot attest a selected dependency',()=>{
  const cases=[
    input=>{input.stderr='';},input=>{input.stderr+=input.stderr;},
    input=>{input.stderr=input.stderr.replace('serde_json=','another_json=');},
    input=>{input.stderr=input.stderr.replace('/toolchain/bin/rustc','/toolchain/bin/other');},
    input=>{input.stderr=input.stderr.replace("'--test'", "'--other'");},
    input=>{const values=rows();values.at(-1).success=false;input.stdout=output(values);},
    input=>{input.stdout=output(rows().filter(row=>row!==actualProbe&&row.target?.name!=='bulk95_outcome_probe'));},
    input=>{const values=rows();values.splice(-1,0,actualDependencies[1]);input.stdout=output(values);},
    input=>{const values=rows();values.find(row=>row.reason==='compiler-artifact'&&row.target?.name==='serde_json'&&row.features.includes('unbounded_depth')).features=['std'];input.stdout=output(values);},
    input=>{input.manifestPath+='.other';},input=>{input.executable+='.other';}
  ];
  for(const mutate of cases){const input=options();mutate(input);assert.throws(()=>readOutcomeCompilerProfile(input));}
});

test('diagnostic word decoding handles actual quoting without executing shell syntax',()=>{
  assert.deepEqual(readCargoDiagnosticArguments("rustc 'a b' \"c d\" e\\ f '' 'it'\\''s'"),['rustc','a b','c d','e f','','it\'s']);
  assert.deepEqual(readCargoDiagnosticArguments('rustc "a\\q" "a\\\"b"'),['rustc','a\\q','a"b']);
  for(const source of ['',"rustc 'unfinished",'rustc trailing\\','rustc x; echo y','rustc $(anything)','rustc "${value}"','rustc x\nother','rustc x\\\nother'])assert.throws(()=>readCargoDiagnosticArguments(source));
});

test('the native width observation has the complete actual source owner and exact original byte inverse',()=>{
  const original=readFileSync(new URL('../probe/native-outcome-probe.rs',import.meta.url));
  const derived=deriveOutcomeTargetProbe(original),insertion=derived.receipt.insertion;
  assert.equal(insertion.source,'"targetUsizeBits": ::std::primitive::usize::BITS, ');
  const bytes=Buffer.from(derived.source),restored=Buffer.concat([bytes.subarray(0,insertion.offset),bytes.subarray(insertion.offset+Buffer.byteLength(insertion.source))]);
  assert.deepEqual(restored,original);
  const declarations=source=>readRustSourceItems(source).items.map(({kind,name})=>({kind,name}));
  assert.deepEqual(declarations(bytes),declarations(original));
  const altered=deriveOutcomeTargetProbe(original);altered.receipt.original.sha256='0'.repeat(64);
  assert.notEqual(deriveOutcomeTargetProbe(original).receipt.original.sha256,altered.receipt.original.sha256);
  assert.throws(()=>deriveOutcomeTargetProbe(Buffer.concat([original,Buffer.from('\n')])));
});
