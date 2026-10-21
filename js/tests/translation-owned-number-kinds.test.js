import assert from 'node:assert/strict';
import test from 'node:test';
import {readFileSync} from 'node:fs';
import {runInNewContext} from 'node:vm';
import {generateOwnedOutcomeNumberKindRules} from '../../experiments/grammar-outcome-network/native-transport/inverse/generate-number-kinds.mjs';
import {readOwnedOutcomeNativeNumberKind} from '../../experiments/grammar-outcome-network/native-transport/inverse/number-kind-rules.mjs';
import {readOwnedOutcomeNativeNumberProfile} from '../../experiments/grammar-outcome-network/native-transport/inverse/number-profile.mjs';
import {readProjectData} from '../scripts/project-data.mjs';
import {selfTranslate} from '../src/self-translation.js';
import {createTypedProgramLinks,decodeTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';
import {recheckLinkedTypedProgram} from '../src/typed-program-checked-projection.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';

const sourceUrl=new URL('../../experiments/grammar-outcome-network/native-transport/inverse/number-kind-rules.mjs',import.meta.url);
const apply=(operation,row)=>operation(row.profileAvailable,row.version,row.arbitraryPrecision,row.signed,row.unsigned,row.floating);

test('all native number predicate combinations retain the independently bounded complete truth table',()=>{
  const {fixture,files}=generateOwnedOutcomeNumberKindRules({check:true});
  assert.equal(fixture.cases.length,160);assert.equal(files.length,2);
  assert.deepEqual(fixture.model.predicates,['is_i64','is_u64','is_f64']);
  for(const row of fixture.cases)assert.equal(apply(readOwnedOutcomeNativeNumberKind,row),row.expected,JSON.stringify(row));
  const canonical={profileAvailable:true,version:'1.0.151',arbitraryPrecision:false,signed:false,unsigned:false,floating:true};
  for(const key of ['profileAvailable','arbitraryPrecision','signed','unsigned','floating']){
    for(const value of [undefined,null,0,1,'', 'false',[],{}])assert.equal(apply(readOwnedOutcomeNativeNumberKind,{...canonical,[key]:value}),'');
  }
  for(const version of [undefined,null,1.0151,'v1.0.151','1.0.151\n','01.0.151','1.0.151.0'])assert.equal(apply(readOwnedOutcomeNativeNumberKind,{...canonical,version}),'');
  assert.equal(readOwnedOutcomeNativeNumberKind(true,'1.0.151',false,true,true,false),'unsigned integer','overlapping signed admission does not invent a second native representation');
});

test('complete number selector self translation and source free typed text and binary reload preserve every decision',()=>{
  const source=readFileSync(sourceUrl,'utf8'),translated=selfTranslate(source,'JavaScript','Rust');
  assert.equal(translated.items.filter(row=>row.status==='carried').length,0);
  assert.equal(translated.items.filter(row=>row.status==='translated'&&row.term!=='comment').length,1);
  assert.equal(selfTranslate(translated.code,'Rust','JavaScript').code,source);
  const packet=createTypedProgramLinks(source,'JavaScript'),{fixture}=generateOwnedOutcomeNumberKindRules({check:true});
  for(const network of [LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))]){
    recheckLinkedTypedProgram(decodeTypedProgramLinks(network,packet.root),{wholeProgram:true});
    const emitted=emitTypedProgramLinks(network,packet.root,'JavaScript').code;
    const operation=runInNewContext(emitted+';readOwnedOutcomeNativeNumberKind;');
    for(const row of fixture.cases)assert.equal(apply(operation,row),row.expected,JSON.stringify(row));
    assert.ok(emitTypedProgramLinks(network,packet.root,'Rust').code.includes('read_owned_outcome_native_number_kind'));
  }
});

test('native number profile contracts bind the selected linked feature unit and reject unsupported precision',()=>{
  const fixture=readProjectData(new URL('../../experiments/grammar-outcome-network/native-transport/runner/compiler-profile-fixture.lino',import.meta.url));
  const original=fixture.records['compile.stdout.log'].split(/\r?\n/u).filter(Boolean).map(line=>JSON.parse(line));
  const probe=original.find(row=>row.reason==='compiler-artifact'&&row.target?.name==='bulk95_outcome_probe');
  const dependencies=original.filter(row=>row.reason==='compiler-artifact'&&row.target?.name==='serde_json');
  assert.equal(dependencies.length,2);
  const inputs=index=>({cargoVersion:fixture.records['cargo-version.stdout.log'],rustcVersion:fixture.records['rustc-version.stdout.log'],
    stdout:original.map(row=>JSON.stringify(row)).join('\n')+'\n',
    stderr:'     Running `'+['/toolchain/bin/rustc','--crate-name','bulk95_outcome_probe','--test',probe.target.src_path,'--extern','serde_json='+dependencies[index].filenames.find(path=>path.endsWith('.rlib'))].map(word=>"'"+word.replaceAll("'","'\\''")+"'").join(' ')+'`\n',
    lockSource:fixture.lockSource,probeName:'bulk95_outcome_probe',manifestPath:probe.manifest_path,executable:probe.executable,usizeBits:64});
  for(let selected=0;selected<2;selected++){
    const input=inputs(selected),profile=readOwnedOutcomeNativeNumberProfile(input);
    assert.equal(profile.version,'1.0.151');assert.equal(profile.arbitraryPrecision,false);
    assert.deepEqual(profile.compilerProfile.dependency,dependencies[selected]);
    // An unrelated feature unit cannot exclude the valid selected artifact.
    const other=structuredClone(original);other.filter(row=>row.reason==='compiler-artifact'&&row.target?.name==='serde_json')[1-selected].features.push('arbitrary_precision');
    assert.equal(readOwnedOutcomeNativeNumberProfile({...input,stdout:other.map(row=>JSON.stringify(row)).join('\n')+'\n'}).arbitraryPrecision,false);
    const changed=structuredClone(original);changed.filter(row=>row.reason==='compiler-artifact'&&row.target?.name==='serde_json')[selected].features.push('arbitrary_precision');
    assert.throws(()=>readOwnedOutcomeNativeNumberProfile({...input,stdout:changed.map(row=>JSON.stringify(row)).join('\n')+'\n'}),/uncovered native JSON number representation/u);
  }
  assert.throws(()=>readOwnedOutcomeNativeNumberProfile({...inputs(0),stderr:''}),/compiler invocation/u);
  assert.throws(()=>readOwnedOutcomeNativeNumberProfile({...inputs(0),lockSource:fixture.lockSource.replace('version = "1.0.151"','version = "1.0.152"')}),/uncovered JSON dependency version/u);
});
