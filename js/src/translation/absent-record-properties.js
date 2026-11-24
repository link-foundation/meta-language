// Actual source receiver into the existing shared closed-record Get policy.
import {readJavaScriptRecordFormalDefaultFieldForm} from './javascript-record-parameter-rules.js';
import {UNIT,option} from './types.js';
import {acceptOptionalAbsence} from './structured-type-rules.js';
export function checkJavaScriptAbsentRecordProperty(checker,value,field,span,expected){
 if(checker.language!=='JavaScript'||checker.checkedProjection)return null;
 const form=readJavaScriptRecordFormalDefaultFieldForm(value.type.kind,value.type.open===true,value.type.fields?.some(item=>item.name===field)??false,field);
 if(form!=='absent')return null;
 const type=expected?.kind==='option'&&acceptOptionalAbsence(expected.absence,'undefined')?expected:option(UNIT,'undefined');
 return {k:'let',name:checker.freshName('ml_absent_property_receiver'),value,body:{k:'none',absence:'undefined',type},type};
}
