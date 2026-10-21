// Specialize only the declared receiver/result types; the checker owns semantics.
import {readRuntimeProjectData} from '../project-data-loader.js';

let arrayRelativeElementTemplatePolicy;

export function createArrayRelativeElementTemplateExpression(receiver,span) {
  const policy=arrayRelativeElementTemplatePolicy??=readRuntimeProjectData(new URL('./data/array-relative-element-template.lino',import.meta.url));
  if(receiver?.kind!=='array'||policy?.schemaVersion!==1||policy.model!=='portable dense array relative element lambda 1'||policy.template?.k!=='lambda'){
    throw new TypeError('the generated relative array element template is malformed');
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
  const template=instantiate(policy.template);
  template.params[0].type=structuredClone(receiver);
  template.ret.element=structuredClone(receiver.element);
  return template;
}
