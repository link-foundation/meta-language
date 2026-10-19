// Instantiate the complete generated source; actual operands stay outside it.
import {readRuntimeProjectData} from '../project-data-loader.js';

let stringRelativeCharacterTemplatePolicy;

export function createStringRelativeCharacterTemplateExpression(span) {
  const policy=stringRelativeCharacterTemplatePolicy??=readRuntimeProjectData(new URL('./data/string-relative-character-template.lino',import.meta.url));
  if(policy?.schemaVersion!==1||policy.model!=='portable UTF16 relative character lambda 1'||policy.template?.k!=='lambda'){
    throw new TypeError('the generated UTF-16 relative character template is malformed');
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
