// Expectations and syntax declarations only; no represented code executes.
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const number={kind:'float'},order={kind:'array',element:number};
const names=['10','2','a','z'],values=[10,2,1,26];
const type={kind:'record',fields:names.map(name=>({name,type:number})),ordered:true};
const surface={language:'JavaScript',items:[{k:'fn',name:'make',params:[{name:'order',type:order}],ret:type,body:{k:'record',entries:names.map((name,index)=>({name,spread:false,value:{k:'num',value:String(values[index]),type:number}})),propertyOrder:{k:'name',path:['order']}}}],main:null};
writeProjectData(new URL('../../parity/fixtures/ordered-record-values.lino',import.meta.url),{
 schemaVersion:1,model:'dynamic own-field order carried by checked record values 1',surface,names,values,
 cases:[
  {name:'dynamic first insertion z then a',input:[3,2,0,1],keys:['2','10','z','a'],values:[2,10,26,1],error:null},
  {name:'same type has a different actual value order',input:[2,3,1,0],keys:['2','10','a','z'],values:[2,10,1,26],error:null},
  ...[{name:'duplicate field',input:[0,0,1,2]},{name:'missing field',input:[0,1,2]},{name:'negative field',input:[0,1,2,-1]},{name:'fractional field',input:[0,1,2,3.5]},{name:'out of range field',input:[0,1,2,4]}].map(row=>({...row,keys:null,values:null,error:{className:'TypeError',message:'invalid ordered record field sequence'}})),
 ],
 assertions:['dynamicValueOrderDistinctFromCanonicalType','numericKeysFirstInNumericOrder','ownedOrderParameterPreserved','wholeSemanticRecheckIdempotent','bothTextAndBinaryCodecs','fourTargetCompleteEmissionParity','actualSourceErrorForEveryInvalidOrder','ordinaryJavaScriptObjectKeysAndValuesControl','constructorErrorVisibleBeforeFormalEffectThreading'],
 boundary:'Unexecuted declarations. This is the internal ordered record value ABI, before automatic source enumeration/profile propagation. Existing complete translation requirements remain.',
});
