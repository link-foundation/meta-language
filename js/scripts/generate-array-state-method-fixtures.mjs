// Author declarations only; the represented operations are never invoked here.
import{writeProjectData}from'./project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft to author declarations');
const source=body=>'/** @returns {number} */\nexport function compute(){'+body+'}\n';
writeProjectData(new URL('../../parity/fixtures/array-state-methods.lino',import.meta.url),{
 schemaVersion:1,versions:{JavaScript:'Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',Rust:'>=1.90.0 <1.100.0; edition 2024'},
 cases:[
  {name:'splice returns a separate removed array and mutates aliases',source:source('const values=[1,2,3];const alias=values;const removed=values.splice(1,1,8,9);return alias.length*100+(removed[0]??0)*10+(alias[1]??0);')},
  {name:'zero argument splice leaves receiver intact',source:source('const values=[1,2,3];const removed=values.splice();return values.length*10+removed.length;')},
  {name:'one argument splice deletes to the end',source:source('const values=[1,2,3,4];const removed=values.splice(-2);return values.length*100+(removed[0]??0)*10+(removed[1]??0);')},
  {name:'explicit undefined delete count removes nothing',source:source('const values=[1,2,3];const removed=values.splice(1,undefined,7);return values.length*100+removed.length*10+(values[1]??0);')},
  {name:'splice truncates fractional boundaries',source:source('const values=[1,2,3,4];const removed=values.splice(1.9,1.9,7);return values.length*100+(removed[0]??0)*10+(values[1]??0);')},
  {name:'splice clamps infinities before native narrowing',source:source('const values=[1,2,3];const removed=values.splice(-Infinity,Infinity,7);return values.length*100+removed.length*10+(values[0]??0);')},
  {name:'splice treats NaN delete count as zero',source:source('const values=[1,2,3];const removed=values.splice(NaN,NaN,8);return values.length*100+removed.length*10+(values[0]??0);')},
  {name:'removed array keeps a fresh shared identity',source:source('const values=[1,2,3];const removed=values.splice(1);removed.push(7);return(removed===values?1000:0)+values.length*100+removed.length*10+(values[0]??0);')},
  {name:'removed elements retain their original record identities',source:source('const item={value:1};const values=[item];const removed=values.splice(0,1);item.value=9;return(removed[0]?.value??0)*10+values.length;')},
  {name:'splice observes argument mutations before its length read',source:source('const values=[1,2];const position=()=>{values.push(3);return 1;};const removed=values.splice(position(),1,9);return values.length*100+(removed[0]??0)*10+(values[1]??0);')},
  {name:'fill returns the original receiver',source:source('const values=[1,2,3];const alias=values;const result=values.fill(7,1);return(result===values?1000:0)+(alias[0]??0)*100+(alias[1]??0)*10+(alias[2]??0);')},
  {name:'fill explicit undefined end uses length',source:source('const values=[1,2,3];values.fill(8,1,undefined);return(values[0]??0)*100+(values[1]??0)*10+(values[2]??0);')},
  {name:'fill null end converts to zero',source:source('const values=[1,2,3];values.fill(8,0,null);return(values[0]??0)*100+(values[1]??0)*10+(values[2]??0);')},
  {name:'fill clamps negative indices',source:source('const values=[1,2,3,4];values.fill(8,-3,-1);return(values[0]??0)*1000+(values[1]??0)*100+(values[2]??0)*10+(values[3]??0);')},
  {name:'fill Boolean indices use primitive numeric conversion',source:source('const values=[1,2,3];values.fill(8,false,true);return(values[0]??0)*100+(values[1]??0)*10+(values[2]??0);')},
  {name:'fill reuses the same element reference',source:source('const first={value:1};const next={value:7};const values=[first,first];values.fill(next);next.value=9;return(values[0]?.value??0)*10+(values[1]?.value??0);')},
  {name:'copyWithin moves overlapping ranges backwards',source:source('const values=[1,2,3,4];const result=values.copyWithin(1,0,3);return(result===values?10000:0)+(values[0]??0)*1000+(values[1]??0)*100+(values[2]??0)*10+(values[3]??0);')},
  {name:'copyWithin moves overlapping ranges forwards',source:source('const values=[1,2,3,4];values.copyWithin(0,1);return(values[0]??0)*1000+(values[1]??0)*100+(values[2]??0)*10+(values[3]??0);')},
  {name:'copyWithin handles missing arguments',source:source('const values=[1,2,3];values.copyWithin();return(values[0]??0)*100+(values[1]??0)*10+(values[2]??0);')},
  {name:'copyWithin distinguishes NaN from undefined end',source:source('const values=[1,2,3];values.copyWithin(0,1,NaN);return(values[0]??0)*100+(values[1]??0)*10+(values[2]??0);')},
  {name:'copyWithin retains complete string elements',source:source('const values=["🙂","a","z"];values.copyWithin(1,0,2);return(values[1]??"").length*10+(values[2]??"").length;')},
  {name:'copyWithin retains exact BigInt elements',source:source('const values=[9007199254740993n,7n,8n];values.copyWithin(1,0,2);return(values[1]??0n)===9007199254740993n?1:0;')},
  {name:'unused copyWithin arguments still evaluate first',source:source('const values=[1,2,3];const extra=()=>{values.push(4);return "ignored";};values.copyWithin(1,0,2,extra());return values.length*1000+(values[1]??0)*100+(values[2]??0)*10+(values[3]??0);')},
  {name:'optional end retains runtime undefined and null distinctions',source:'/** @param {number|null|undefined} end @returns {number} */\nfunction fillWithSelectedEnd(end){const values=[1,2,3];values.fill(8,1,end);return(values[1]??0)*10+(values[2]??0);}\n'+source('return fillWithSelectedEnd(undefined)*100+fillWithSelectedEnd(null)*10+fillWithSelectedEnd(2);')},
 ],
 refusals:[
  {name:'BigInt indices keep their conversion error obligation',source:source('const values=[1,2,3];values.splice(1n,1);return values.length;'),reason:'index'},
  {name:'object index coercion keeps its complete effect obligation',source:source('const values=[1,2,3];const index={value:1};values.copyWithin(index,0);return values.length;'),reason:'index'},
 ],
});
