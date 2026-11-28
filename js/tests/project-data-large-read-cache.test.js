import assert from 'node:assert/strict';
import {mkdtempSync,readFileSync,writeFileSync,statSync,utimesSync,rmSync} from 'node:fs';
import {tmpdir} from 'node:os';
import {join} from 'node:path';
import test from 'node:test';
import {readProjectData} from '../scripts/project-data.mjs';
import {readBootstrapStructuredDataLino} from '../../scripts/lib/structured-data-bootstrap.mjs';

test('repeated large Links reads retain exact independent values and fresh owners while checking all current bytes',()=>{
  const source=readFileSync(new URL('../../parity/naming/canonical-concepts.lino',import.meta.url),'utf8');
  const expected=readBootstrapStructuredDataLino(source);
  const directory=mkdtempSync(join(tmpdir(),'meta-language-large-data-control-'));
  const file=join(directory,'register.lino');
  try{
    writeFileSync(file,source);
    const first=readProjectData(file),second=readProjectData(file);
    assert.deepEqual(first,expected);assert.deepEqual(second,expected);
    assert.notEqual(first,second);assert.notEqual(first.concepts,second.concepts);
    first.concepts[0].definition='caller mutation';
    assert.deepEqual(readProjectData(file),expected,'no caller can change the private snapshot');
    assert.throws(()=>readProjectData(file,{limits:{nodeLimit:1}}),/resource limit/u,
      'custom limits still execute the original complete validation');
    const original=statSync(file);
    assert.ok(source.startsWith('(1:'));
    const corrupted=source.replace(/^\(1:/u,'(0:');assert.notEqual(corrupted,source);
    writeFileSync(file,corrupted);utimesSync(file,original.atime,original.mtime);
    assert.equal(statSync(file).size,original.size);
    assert.throws(()=>readProjectData(file),/identity|structured data/u,
      'equal file size and timestamps cannot authorize stale decoded data');
  }finally{rmSync(directory,{recursive:true,force:true});}
});
