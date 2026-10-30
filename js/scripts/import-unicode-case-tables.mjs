// Automatically convert official declarations; no casing algorithm is executed.
import {readFileSync,writeFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {join} from 'node:path';
import {writeProjectData} from './project-data.mjs';
const directory=process.argv[2];
if(!directory||process.argv[3]!=='--draft'||process.argv.length!==4)throw new TypeError('use <temporary Unicode directory> --draft');
const unicode='16.0.0',names=['UnicodeData.txt','SpecialCasing.txt','DerivedCoreProperties.txt','Unicode-License.txt'];
const sources=names.map(name=>{const bytes=readFileSync(join(directory,name));return{name,url:name==='Unicode-License.txt'?'https://www.unicode.org/license.txt':'https://www.unicode.org/Public/'+unicode+'/ucd/'+name,bytes:bytes.length,sha256:createHash('sha256').update(bytes).digest('hex'),text:bytes.toString('utf8')};});
const lower=new Map(),upper=new Map(),conditional=[];
for(const line of sources[0].text.split('\n')){
 if(!line)continue;const fields=line.split(';'),point=Number.parseInt(fields[0],16);
 if(fields[13])lower.set(point,[Number.parseInt(fields[13],16)]);
 if(fields[12])upper.set(point,[Number.parseInt(fields[12],16)]);
}
for(const line of sources[1].text.split('\n')){
 const body=line.split('#')[0].trim();if(!body)continue;
 const fields=body.split(';').map(value=>value.trim()),point=Number.parseInt(fields[0],16);
 const decode=value=>value.split(/\s+/u).filter(Boolean).map(unit=>Number.parseInt(unit,16));
 if(fields[4]){conditional.push({point,lower:decode(fields[1]),upper:decode(fields[3]),condition:fields[4]});continue;}
 lower.set(point,decode(fields[1]));upper.set(point,decode(fields[3]));
}
// Locale-insensitive conditional rules have exactly one declared context in
// this pinned profile. Unknown successors require an explicit source extension.
const contextual=conditional.filter(row=>!row.condition.split(/\s+/u).some(term=>/^[a-z]{2,3}$/u.test(term)));
if(contextual.length!==1||contextual[0].point!==931||contextual[0].condition!=='Final_Sigma'||contextual[0].lower.join(',')!=='962'||contextual[0].upper.join(',')!=='931')throw new TypeError('uncovered Unicode default case context declarations');
const compress=map=>{
 const single=[...map].filter(([point,values])=>values.length===1&&values[0]!==point).sort((a,b)=>a[0]-b[0]);
 const ranges=[];
 for(let index=0;index<single.length;){
  const [first,[mapped]]=single[index],delta=mapped-first;let last=first,stride=1,end=index+1;
  if(end<single.length){const gap=single[end][0]-first;if((gap===1||gap===2)&&single[end][1][0]-single[end][0]===delta){stride=gap;while(end<single.length&&single[end][0]-last===stride&&single[end][1][0]-single[end][0]===delta){last=single[end][0];end+=1;}}}
  ranges.push(first,last,stride,delta);index=end;
 }
 const indexes=[],values=[];
 for(const [point,mapping] of [...map].filter(([,mapping])=>mapping.length!==1).sort((a,b)=>a[0]-b[0])){indexes.push(point,values.length,mapping.length);values.push(...mapping);}
 return{ranges,indexes,values};
};
const properties={Cased:[],Case_Ignorable:[]};
for(const line of sources[2].text.split('\n')){
 const body=line.split('#')[0].trim();if(!body)continue;const [interval,property]=body.split(';').map(value=>value.trim());
 if(!Object.hasOwn(properties,property))continue;const ends=interval.split('..').map(value=>Number.parseInt(value,16));properties[property].push(ends[0],ends.at(-1));
}
const lowerTable=compress(lower),upperTable=compress(upper);
const columns={LowerRanges:lowerTable.ranges,LowerIndexes:lowerTable.indexes,LowerValues:lowerTable.values,UpperRanges:upperTable.ranges,UpperIndexes:upperTable.indexes,UpperValues:upperTable.values,CasedRanges:properties.Cased,IgnorableRanges:properties.Case_Ignorable};
writeProjectData(new URL('../src/data/unicode-case-tables.lino',import.meta.url),{schemaVersion:1,unicode,sources:sources.map(({text,...row})=>row),columns,conditional,generatedBy:'js/scripts/import-unicode-case-tables.mjs',boundary:'Automatically converted complete simple/full/default-context casing declarations. Locale-specific mappings are retained as declarations, not applied by default casing. No represented casing execution or observations.'});
writeFileSync(new URL('../../rust/src/data/unicode-case-tables.lino',import.meta.url),readFileSync(new URL('../src/data/unicode-case-tables.lino',import.meta.url)));
const declarations=Object.entries(columns).map(([name,values])=>'/** @returns {number[]} */\nexport function readUnicodeCase'+name+'(){return ['+values.join(',')+'];}');
writeFileSync(new URL('../src/translation/unicode-case-table-declarations.js',import.meta.url),'// Generated from official Unicode '+unicode+' declarations by import-unicode-case-tables.mjs.\n// Do not edit; license: ../data/LICENSE-UNICODE.txt; exact sources: ../data/unicode-case-tables.lino.\n'+declarations.join('\n')+'\n');
