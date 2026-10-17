import {readFileSync,writeFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {readProjectData} from '../js/scripts/project-data.mjs';
export function sourceTextByteRulesSource(){
 const {constants:c}=readProjectData(new URL('./source-text-byte-model.lino',import.meta.url));
 return `// Generated from the linked source-text byte model. Do not edit.
/** @param {number[]} units @param {number} index @returns {number} */
export function readSourceTextScalar(units,index){
 if(!Number.isInteger(index)||index<0||index>=units.length||units.length>${c.maximumUnits})return ${c.invalid};
 const first=units[index]??${c.invalid};
 if(!Number.isInteger(first)||first<0||first>${c.maximumUnit})return ${c.invalid};
 if(first>=${c.highStart}&&first<=${c.highEnd}){
  if(index+1>=units.length)return ${c.invalid};
  const second=units[index+1]??${c.invalid};
  if(!Number.isInteger(second)||second<${c.lowStart}||second>${c.lowEnd})return ${c.invalid};
  return ${c.supplementaryStart}+(first-${c.highStart})*${c.surrogateBase}+second-${c.lowStart};
 }
 if(first>=${c.lowStart}&&first<=${c.lowEnd})return ${c.invalid};
 return first;
}
/** @param {number} scalar @returns {number} */
export function readSourceTextScalarByteWidth(scalar){
 if(!Number.isInteger(scalar)||scalar<0||scalar>${c.maximumScalar}||scalar>=${c.highStart}&&scalar<=${c.lowEnd})return ${c.invalid};
 if(scalar<${c.oneByteLimit})return 1;
 if(scalar<${c.twoByteLimit})return 2;
 if(scalar<${c.threeByteLimit})return 3;
 return 4;
}
/** @param {number[]} units @returns {number} */
export function readSourceTextByteLength(units){
 if(units.length>${c.maximumUnits})return ${c.invalid};
 let index=0;let bytes=0;
 while(index<units.length){
  const scalar=readSourceTextScalar(units,index);
  const width=readSourceTextScalarByteWidth(scalar);
  if(width<0)return ${c.invalid};
  bytes=bytes+width;index=index+(scalar>=${c.supplementaryStart}?2:1);
 }
 return bytes;
}
/** @param {number[]} units @returns {boolean} */
export function acceptSourceTextScalarUnits(units){return readSourceTextByteLength(units)>=0;}
/** @param {number[]} units @param {number} offset @returns {number} */
export function readSourceTextCodeUnitOffset(units,offset){
 const length=readSourceTextByteLength(units);
 if(length<0||!Number.isInteger(offset)||offset<0||offset>length)return ${c.invalid};
 let index=0;let bytes=0;
 while(bytes<offset){
  const scalar=readSourceTextScalar(units,index);
  bytes=bytes+readSourceTextScalarByteWidth(scalar);
  index=index+(scalar>=${c.supplementaryStart}?2:1);
 }
 return bytes===offset?index:${c.invalid};
}
/** @param {number[]} units @param {number} start @param {number} end @returns {boolean} */
export function acceptSourceTextByteSlice(units,start,end){
 return start<=end&&readSourceTextCodeUnitOffset(units,start)>=0&&readSourceTextCodeUnitOffset(units,end)>=0;
}
/** @param {number[]} units @param {number} offset @returns {number} */
export function readSourceTextByte(units,offset){
 const length=readSourceTextByteLength(units);
 if(length<0||!Number.isInteger(offset)||offset<0||offset>=length)return ${c.invalid};
 let index=0;let bytes=0;
 while(index<units.length){
  const scalar=readSourceTextScalar(units,index);
  const width=readSourceTextScalarByteWidth(scalar);
  const position=offset-bytes;
  if(position<width){
   if(width===1)return scalar;
   if(position===0){
    if(width===2)return ${c.twoByteLead}+Math.floor(scalar/${c.byteBase});
    if(width===3)return ${c.threeByteLead}+Math.floor(scalar/${c.thirdDivisor});
    return ${c.fourByteLead}+Math.floor(scalar/${c.fourthDivisor});
   }
   if(position===width-1)return ${c.continuationBase}+scalar%${c.byteBase};
   if(width===4&&position===1)return ${c.continuationBase}+Math.floor(scalar/${c.thirdDivisor})%${c.byteBase};
   return ${c.continuationBase}+Math.floor(scalar/${c.byteBase})%${c.byteBase};
  }
  bytes=bytes+width;index=index+(scalar>=${c.supplementaryStart}?2:1);
 }
 return ${c.invalid};
}
`;
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
 const file=new URL('./source-text-byte-rules.mjs',import.meta.url),source=sourceTextByteRulesSource();
 if(process.argv.includes('--check')){if(readFileSync(file,'utf8')!==source)throw new Error('source-text byte rules are stale');}
 else writeFileSync(file,source);
 console.log('Source-text byte kernel: linked numeric rules generated');
}
