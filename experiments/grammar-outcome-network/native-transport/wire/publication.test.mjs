import assert from 'node:assert/strict';
import {readFileSync,mkdtempSync,writeFileSync,rmSync,symlinkSync,existsSync} from 'node:fs';
import {tmpdir} from 'node:os';
import {join} from 'node:path';
import test from 'node:test';
import {fileURLToPath} from 'node:url';
import {readProjectData} from '../../../../js/scripts/project-data.mjs';
import {verifyPublication,replayPublicationEdits} from './publication.mjs';
import {explicitOutputDirectory} from './output.mjs';
import {repository} from './wire-policy.mjs';

test('published experiment retains authenticated original authors and explicit whole-source transformations',()=>{
  const result=verifyPublication({checkNativeCst:true});
  assert.equal(result.files.length,8);
  assert.equal(result.originalSources.length,9);
  const archive=readProjectData(fileURLToPath(new URL('./original-authoring.lino',import.meta.url)));
  const original=archive.records[0];
  assert.throws(()=>replayPublicationEdits({...original,original:original.original+'\n'}));
  assert.throws(()=>replayPublicationEdits({...original,edits:[{from:'unknown predecessor bytes',to:'',count:1}]}));
  const probe=readFileSync(new URL('../probe/native-outcome-probe.rs',import.meta.url),'utf8');
  const end=probe.indexOf('// Report only AFTER');
  assert.ok(probe.indexOf('std::panic::catch_unwind')<end);
  assert.ok(probe.indexOf('failures.is_empty()')<end);
  assert.ok(probe.indexOf('independent_boundaries();')<end);
});

test('generators refuse implicit output and production directories before writing',()=>{
  assert.throws(()=>explicitOutputDirectory([]));
  assert.throws(()=>explicitOutputDirectory(['--output-dir',repository]));
  assert.throws(()=>explicitOutputDirectory(['--output-dir',repository+'/rust/src']));
  const scratch=mkdtempSync(join(tmpdir(),'meta-language-native-output-control-'));
  try{
    const foreign=join(scratch,'foreign');
    writeFileSync(join(scratch,'unrelated.txt'),'preserve');
    assert.throws(()=>explicitOutputDirectory(['--output-dir',scratch]),/ownership marker/);
    assert.equal(readFileSync(join(scratch,'unrelated.txt'),'utf8'),'preserve');
    symlinkSync(repository+'/rust/src',foreign);
    assert.throws(()=>explicitOutputDirectory(['--output-dir',join(foreign,'native-outcome-refusal-control')]),/production sources/);
    assert.equal(existsSync(repository+'/rust/src/native-outcome-refusal-control'),false);
    const owned=join(scratch,'owned');
    assert.equal(explicitOutputDirectory(['--output-dir',owned]),explicitOutputDirectory(['--output-dir',owned]));
  }finally{rmSync(scratch,{recursive:true,force:true});}
});
