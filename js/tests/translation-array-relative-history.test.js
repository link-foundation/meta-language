import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {readProjectData,serializeProjectData} from '../scripts/project-data.mjs';
import {verifyGitArtifactCertificate} from '../scripts/git-object-certificate.mjs';
import {generateArrayRelativeElementHistory} from '../scripts/generate-array-relative-element-history.mjs';
import {restoreArrayRelativeElementSource,restoreArrayRelativeElementProjectData} from '../scripts/array-relative-element-history.mjs';

const root=new URL('../../',import.meta.url);
const history=readProjectData(new URL('parity/self-translation/array-relative-element-history.lino',root));

test('relative array history retains every whole certified predecessor and recorded native definition',()=>{
  generateArrayRelativeElementHistory({check:true});
  assert.equal(history.sources.length,24);assert.equal(history.descriptorChanges.length,2);
  for(const row of history.sources){
    verifyGitArtifactCertificate(row.certificate,Buffer.from(row.sourceText),{revision:history.revision,path:row.file});
    const current=readFileSync(new URL(row.file,root),'utf8');
    assert.equal(restoreArrayRelativeElementSource(current,row.file),row.sourceText);
    assert.equal(restoreArrayRelativeElementSource(row.sourceText,row.file),row.sourceText);
    assert.throws(()=>restoreArrayRelativeElementSource(current+'\n# unreviewed\n',row.file),assert.AssertionError);
    if(row.role==='historical descriptor'){
      const value=readProjectData(new URL(row.file,root));
      assert.equal(serializeProjectData(restoreArrayRelativeElementProjectData(value,row.file),row.file),row.sourceText);
      const corrupt=structuredClone(value);corrupt.unreviewed=true;
      assert.throws(()=>restoreArrayRelativeElementProjectData(corrupt,row.file),assert.AssertionError);
    }
  }
  const file='experiments/original-link-complete-table-projection/data/raw-consuming-current-native-definitions.lino';
  const current=readProjectData(new URL(file,root)),previous=restoreArrayRelativeElementProjectData(current,file);
  for(const field of ['completeRust','owners','declarationNames','totals','assumptions','encodings','mappings','preludeHashes','replays','archive'])assert.deepEqual(current[field],previous[field],field);
});

test('the original array input is retained while the wrong unsupported expectation has explicit standard bounds',()=>{
  assert.equal(history.assertionCorrection.input,'/** @param {number[]} values @returns {number|undefined} */function last(values){return values.at(-1);}');
  assert.deepEqual(history.assertionCorrection.languageVersions,{minimum:'2022',maximumExclusive:'2027'});
  const row=history.sources.find(row=>row.file===history.assertionCorrection.file);
  assert.ok(row.sourceText.includes(history.assertionCorrection.input));
  assert.ok(row.sourceText.includes('assert.throws(()=>checkProgram(parseJavaScript(array)),/has no checked operation on array/u);'));
  const current=readFileSync(new URL(row.file,root),'utf8');
  assert.ok(current.includes(history.assertionCorrection.input));
  assert.ok(current.includes('assert.equal(arrayRead([1,2,3]),[1,2,3].at(-1));'));
  assert.ok(current.includes('assert.equal(arrayRead([]),[].at(-1));'));
});
