import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {readProjectData,serializeProjectData} from '../scripts/project-data.mjs';
import {readStructuredDataLino} from '../src/structured-data-links.js';
import {verifyGitArtifactCertificate} from '../scripts/git-object-certificate.mjs';
import {restoreNativeCompatibilitySourceHistory} from '../scripts/native-compatibility-source-history.mjs';
import {generateNativeCompatibilitySourceHistory} from '../../experiments/grammar-outcome-network/native-transport/inverse/generate-native-compatibility-source-history.mjs';
const root=new URL('../../',import.meta.url);
const pin=value=>({bytes:Buffer.byteLength(value),sha256:createHash('sha256').update(value).digest('hex')});

test('native compatibility repairs authenticate every current source and complete original module independently',()=>{
  const [history,fixtures]=generateNativeCompatibilitySourceHistory({check:true});
  assert.equal(history.sources.length,11);assert.equal(fixtures.sources.length,3);
  for(const row of [...history.sources,...fixtures.sources]){
    const current=readFileSync(new URL(row.file,root));assert.deepEqual(pin(current),row.current);
    assert.deepEqual(pin(row.sourceText),{bytes:row.bytes,sha256:row.sha256});
    verifyGitArtifactCertificate(row.certificate,Buffer.from(row.sourceText),{revision:row.commit,path:row.file});
    if(history.sources.includes(row)){
      assert.equal(restoreNativeCompatibilitySourceHistory(row.file,current.toString()),row.sourceText);
      assert.equal(restoreNativeCompatibilitySourceHistory(row.file,row.sourceText),row.sourceText);
      const changed=current.toString()+'\n# unknown current edit\n';
      assert.equal(restoreNativeCompatibilitySourceHistory(row.file,changed),changed);
    }
    assert.throws(()=>verifyGitArtifactCertificate(row.certificate,Buffer.from(row.sourceText+'\n'),{revision:row.commit,path:row.file}));
  }
  assert.equal(restoreNativeCompatibilitySourceHistory('unregistered/source.js','unregistered body'),'unregistered body');
});

test('native compatibility output updates preserve all original graph cases publication fields and template inputs',()=>{
  const history=readProjectData(new URL('parity/self-translation/native-compatibility-source-fixtures-history.lino',root));
  for(const row of history.sources){
    const original=readStructuredDataLino(row.sourceText),current=readProjectData(new URL(row.file,root));
    if(row.file.endsWith('concrete-program-native-consumer-template.lino')){
      assert.deepEqual(current.rustChanges.slice(0,-2),original.rustChanges);
      const additional=current.rustChanges.splice(-2);
      assert.equal(additional.length,2);
      for(const change of additional){assert.match(change.before,/format!\("\{:x\}"/u);assert.match(change.after,/\{byte:02x\}/u);assert.equal(change.occurrences,1);}
    }else{
      const key=row.file.endsWith('/manifest.lino')?'generatedConsumers':'outputs';
      assert.equal(current[key].length,original[key].length);
      for(const [index,output]of current[key].entries()){
        const path=output.path??output.file;assert.deepEqual(pin(readFileSync(new URL(path,root))),{bytes:output.bytes,sha256:output.sha256});
        assert.equal(path,original[key][index].path??original[key][index].file);
        // Only source digest fields can change; all inputs and expected results
        // remain independently preserved by the full serialized inverse below.
        output.bytes=original[key][index].bytes;output.sha256=original[key][index].sha256;
      }
    }
    assert.equal(serializeProjectData(current,row.file),row.sourceText,'every predecessor fixture byte is retained');
  }
});
