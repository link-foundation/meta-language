import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {execFileSync} from 'node:child_process';
import test from 'node:test';
import {readProjectData} from '../scripts/project-data.mjs';
import {verifyGitArtifactCertificate} from '../scripts/git-object-certificate.mjs';
import {generateBooleanParameterTuples} from '../scripts/generate-boolean-parameter-tuples.mjs';
import {readBooleanParameterTupleStyle,readBooleanParameterTupleCallStyle} from '../scripts/boolean-parameter-tuple-style.mjs';

const root=new URL('../../',import.meta.url);

test('both generated Boolean carriers retain the complete original decision body and ordered caller source',()=>{
  generateBooleanParameterTuples({check:true});
  const policy=readProjectData(new URL('parity/self-translation/boolean-parameter-tuples.lino',root));
  assert.deepEqual(policy.models.map(row=>row.name),['require_self_translation_layout_envelope','read_owned_outcome_native_number_kind']);
  for(const row of policy.models){
    assert.equal(row.groups.length,1);const group=row.groups[0];
    assert.equal(group.names.length,4);assert.equal(group.end-group.first,4);
    const originalBody=row.originalDefinition.slice(row.originalDefinition.indexOf('{')+1);
    const packedBody=row.packedDefinition.slice(row.packedDefinition.indexOf('{')+1);
    const binding='\n    let ('+group.names.join(', ')+') = '+group.carrier+';';
    assert.ok(packedBody.startsWith(binding));assert.equal(packedBody.slice(binding.length),originalBody,'every original decision byte remains, including all branch and return expressions');
    verifyGitArtifactCertificate(row.caller.certificate,Buffer.from(row.caller.sourceText),{revision:policy.revision,path:row.caller.file});
    let raw=row.caller.sourceText;
    for(const edit of row.caller.insertions)raw=raw.slice(0,edit.offset)+edit.text+raw.slice(edit.offset);
    const formatted=execFileSync('rustfmt',['--edition','2024','--config','skip_children=true','--emit','stdout'],{input:raw,encoding:'utf8',maxBuffer:1024*1024,timeout:15000});
    assert.equal(formatted,readFileSync(new URL(row.caller.file,root),'utf8'));
    for(const edit of [...row.caller.insertions].reverse()){
      assert.equal(raw.slice(edit.offset,edit.offset+edit.text.length),edit.text);raw=raw.slice(0,edit.offset)+raw.slice(edit.offset+edit.text.length);
    }
    assert.equal(raw,row.caller.sourceText,'all caller argument expressions and every other original byte survive inverse');
  }
});

test('tuple source styles preserve primitive binding scope and refuse ambiguous types or call owners',()=>{
  const original='pub fn choose(first: bool, second: bool, third: bool, fourth: bool) -> bool { let ml_boolean_parameters = true; first || second || third || fourth || ml_boolean_parameters }';
  const styled=readBooleanParameterTupleStyle(original,'choose');
  assert.equal(styled.groups[0].carrier,'ml_boolean_parameters_1');
  assert.equal(readBooleanParameterTupleStyle(styled.source,'choose').source,styled.source);
  assert.equal(styled.source.replace(styled.actions[0].to,styled.actions[0].from),original);
  assert.throws(()=>readBooleanParameterTupleStyle('type bool = u8; '+original,'choose'),/ambient type aliases/u);
  for(const source of ['call(true,false,true);','call(true,false,true,false);call(true,false,true,false);','other(true,false,true,false);']){
    assert.throws(()=>readBooleanParameterTupleCallStyle(source,'call',styled.groups));
  }
});
