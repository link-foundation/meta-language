// Mechanical instantiation of the complete shared source definition.
import {readRuntimeProjectData} from '../project-data-loader.js';

let stringCharacterTemplatePolicy;

export function createStringCharacterTemplateExpression(span) {
  const policy=stringCharacterTemplatePolicy??=readRuntimeProjectData(new URL('./data/string-code-unit-template.lino',import.meta.url)).character;
  if(policy?.schemaVersion!==1||policy.model!=='portable UTF16 character lambda 1'||policy.template?.k!=='lambda'){
    throw new TypeError('the generated UTF-16 character template is malformed');
  }
  const instantiate=value=>{
    if(Array.isArray(value))return value.map(instantiate);
    if(value&&typeof value==='object'){
      const result=Object.fromEntries(Object.entries(value).map(([key,child])=>[key,instantiate(child)]));
      if(result.k||Object.hasOwn(result,'span'))result.span=span;
      return result;
    }
    return value;
  };
  return instantiate(policy.template);
}
