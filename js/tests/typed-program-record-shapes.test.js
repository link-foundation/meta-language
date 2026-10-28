// Deferred complete layout, edit, ownership and formal model controls.
import test from 'node:test';
import assert from 'node:assert/strict';
import {assertVersions,recordShapeFixture,observeRecordShape,observeRecordShapePredecessors} from '../../experiments/javascript-properties/record-shape-controls.mjs';
import {parseJavaScript} from '../src/translation/javascript.js';
import {checkProgram} from '../src/translation/check.js';
import {encodeTypedProgramLinksWithRecordShapes,encodeTypedProgramLinksWithValueSharing} from '../src/index.js';
assertVersions();
for(const row of recordShapeFixture.programs)test(row.name,()=>observeRecordShape(row));
test('complete preceding source bytes remain available',()=>observeRecordShapePredecessors());
test('explicit sharing entry points refuse empty and unknown qualified models',()=>{
 const program=checkProgram(parseJavaScript(recordShapeFixture.programs[0].source));
 for(const profile of ['',undefined,'unregistered layout 7']){
  assert.throws(()=>encodeTypedProgramLinksWithRecordShapes(program,undefined,profile));
  assert.throws(()=>encodeTypedProgramLinksWithValueSharing(program,undefined,profile));
 }
});
