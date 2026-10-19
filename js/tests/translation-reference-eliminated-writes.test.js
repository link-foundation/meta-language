import assert from 'node:assert/strict';
import test from 'node:test';
import {parseJavaScript} from '../src/translation/javascript.js';
import {requiresReferenceSemantics} from '../src/translation/reference-analysis.js';
import {requireSurfaceReferenceToken,requireSurfaceReferenceTokenAfterElimination} from '../src/translation/surface-reference-analysis-rules.js';

test('proved indexed assignment syntax stops selecting a heap after the actual source operation is lowered',()=>{
 const source='/** @param {number} position @param {number} value @returns {number[]} */ function updated(position,value){const data=[2,4];if(Number.isInteger(position)&&position>=0&&position<data.length){data[position]=value;}return data;}';
 assert.equal(requiresReferenceSemantics(source),true);
 const start=source.indexOf('data[position]')+'data'.length,end=source.indexOf('value;',start);
 const removed=[{start,end}];
 assert.ok(start>0&&end>start);
 assert.equal(requiresReferenceSemantics(source,removed),false);
 assert.equal(parseJavaScript(source).referenceSemantics,undefined);
 for(const suffix of [
  'function other(data,position,value){data[position]=value;}',
  'function other(record,value){record.value=value;}',
  'function other(){let count=0;const change=()=>{count+=1;return count;};return change();}',
  'class Other {}',
 ])assert.equal(requiresReferenceSemantics(source+suffix,removed),true);
 const nested=source.replace('data[position]=value;','data[position]=other.push(value);');
 assert.equal(requiresReferenceSemantics(nested,[{start,end:nested.indexOf('other.push',start)}]),true);
});

test('the selected token decision retains complete old validation and every unproved reference trigger',()=>{
 const assignment=['punct','=','punct',']','position','identifier','value',';'];
 assert.equal(requireSurfaceReferenceToken(assignment,false,false),true);
 assert.equal(requireSurfaceReferenceTokenAfterElimination(assignment,false,false,false),true);
 assert.equal(requireSurfaceReferenceTokenAfterElimination(assignment,false,false,true),false);
 for(const window of [
  ['punct','+=','punct',']','position','identifier','value',';'],
  ['punct','=','identifier','field','.','identifier','value',';'],
  ['identifier','class','','','','identifier','Other','{'],
  ['punct','+=','identifier','count','{','number','1',';'],
 ])assert.equal(requireSurfaceReferenceTokenAfterElimination(window,true,false,true),requireSurfaceReferenceToken(window,true,false));
 for(const window of [[],assignment.slice(0,4),[...assignment.slice(0,7)]]){
  assert.throws(()=>requireSurfaceReferenceToken(window,false,false));
  assert.throws(()=>requireSurfaceReferenceTokenAfterElimination(window,false,false,true));
 }
});
