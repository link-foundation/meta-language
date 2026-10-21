import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {readProjectData} from './project-data.mjs';

export function readNodeDependencyGeneratorRegistrationHistory() {
  const file=new URL('../../parity/naming/node-dependency-generator-registration-source-history.lino',import.meta.url);
  assert.equal(createHash('sha256').update(readFileSync(file)).digest('hex'),'7cc0328f3dd07366df1eb1af079e427e7bd285fb89c765202550dd9d12ec06e8');
  const history=readProjectData(file);
  for(const source of history.sources){
    assert.equal(Buffer.byteLength(source.sourceText),source.bytes);
    assert.equal(createHash('sha256').update(source.sourceText).digest('hex'),source.sha256);
    assert.equal(source.commit,null);
  }
  return history;
}

export function restoreNodeDependencyGeneratorTestHistory(source) {
  source=restoreNodeDependencyInventoryTestSource(source);
  const history=readNodeDependencyGeneratorRegistrationHistory(),rule=history.testCorrection;
  const original=history.sources.find(row=>row.file===rule.file).sourceText;
  let expected=original;
  for(const edit of rule.edits){
    assert.equal(expected.split(edit.before).length,2);
    expected=expected.replace(edit.before,()=>edit.after);
  }
  expected+=rule.suffix;
  assert.equal(source,expected,'the complete current test retains every original input and assertion through only the recorded inventory correction');
  return original;
}

import {restoreNodeDependencyInventoryTestSource} from './dependency-inventory-append-history.mjs';
