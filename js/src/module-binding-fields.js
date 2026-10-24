// Actual CST nodes/field relations supply dense columns; shared rules own roles.
import { planJavaScriptModuleBindingFields, readSourceModuleBindingError } from './translation/module-binding-rules.js';
export function readModuleBindingFields(network,syntax,language) {
  if (language !== 'JavaScript') return [];
  const positions=new Map(syntax.map((node,index)=>[Number(node.linkId),index]));
  const fields=syntax.map(()=>[]);
  for(const field of network.fieldRelations()) {
    const parent=positions.get(Number(field.parent)), child=positions.get(Number(field.child));
    if(parent!==undefined && child!==undefined)fields[parent].push([field.label,child]);
  }
  const starts=[],counts=[],labels=[],targets=[];
  for(const row of fields){starts.push(labels.length);counts.push(row.length);for(const[label,child]of row){labels.push(label);targets.push(child);}}
  const plan = planJavaScriptModuleBindingFields(syntax.map(node=>node.term),syntax.map(node=>node.start),syntax.map(node=>node.end),starts,counts,labels,targets);
  if (plan[0] === -1) throw new TypeError(readSourceModuleBindingError(plan[1]));
  return plan;
}
