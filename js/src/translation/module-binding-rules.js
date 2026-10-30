// Actual CST fields distinguish module export names from lexical bindings.
import { readRequiredNumberFact, readRequiredTextFact } from './fact-column-rules.js';

/** @param {number[]} starts @param {number[]} counts @param {string[]} labels
 * @param {number[]} targets @param {number} owner @param {string} label @returns {number} */
function readModuleBindingField(starts,counts,labels,targets,owner,label) {
  let result = -1;
  for (let at=readRequiredNumberFact(starts,owner);at<readRequiredNumberFact(starts,owner)+readRequiredNumberFact(counts,owner);at+=1) {
    if (readRequiredTextFact(labels,at) === label) {
      if (result >= 0) return -2;
      result = readRequiredNumberFact(targets,at);
    }
  }
  return result;
}
/** Range triples [start,end,role]: 1 import shorthand,2 export shorthand,
 * 3 external name. Ordinary local aliases need no expansion.
 * @param {string[]} terms @param {number[]} nodeStarts @param {number[]} nodeEnds
 * @param {number[]} starts @param {number[]} counts @param {string[]} labels
 * @param {number[]} targets @returns {number[]} */
export function planJavaScriptModuleBindingFields(terms,nodeStarts,nodeEnds,starts,counts,labels,targets) {
  if (terms.length !== nodeStarts.length || terms.length !== nodeEnds.length || terms.length !== starts.length || terms.length !== counts.length || labels.length !== targets.length)
    return [-1,1,0];
  let plan = [];
  for (let node=0;node<terms.length;node+=1) {
    const term=readRequiredTextFact(terms,node);
    if (term === 'namespace_export') {
      plan=plan.concat([readRequiredNumberFact(nodeStarts,node),readRequiredNumberFact(nodeEnds,node),3]);
      continue;
    }
    if (term !== 'import_specifier' && term !== 'export_specifier') continue;
    const name=readModuleBindingField(starts,counts,labels,targets,node,'name');
    const alias=readModuleBindingField(starts,counts,labels,targets,node,'alias');
    for (let at=readRequiredNumberFact(starts,node);at<readRequiredNumberFact(starts,node)+readRequiredNumberFact(counts,node);at+=1)
      if (readRequiredTextFact(labels,at) !== 'name' && readRequiredTextFact(labels,at) !== 'alias')
        return [-1,4,0];
    if (name === -2 || alias === -2) return [-1,2,0];
    if (!Number.isSafeInteger(name) || name<0 || name>=terms.length || !Number.isSafeInteger(alias) || alias< -1 || alias>=terms.length)
      return [-1,3,0];
    let external=term === 'import_specifier' && alias>=0;
    if (term === 'export_specifier') {
      for (let owner=0;owner<terms.length;owner+=1) {
        if (readRequiredTextFact(terms,owner) === 'export_statement'
          && readRequiredNumberFact(nodeStarts,owner)<=readRequiredNumberFact(nodeStarts,node)
          && readRequiredNumberFact(nodeEnds,owner)>=readRequiredNumberFact(nodeEnds,node)) {
          const source=readModuleBindingField(starts,counts,labels,targets,owner,'source');
          if (source === -2) return [-1,2,0];
          if (source>=0) external=true;
        }
      }
    }
    plan=plan.concat([readRequiredNumberFact(nodeStarts,name),readRequiredNumberFact(nodeEnds,name),external?3:alias<0?(term === 'import_specifier'?1:2):0]);
    if (alias>=0 && term === 'export_specifier')
      plan=plan.concat([readRequiredNumberFact(nodeStarts,alias),readRequiredNumberFact(nodeEnds,alias),3]);
  }
  return plan;
}
/** @param {number[]} plan @param {number} start @param {number} end @returns {number} */
export function readSourceModuleBindingRangeRole(plan,start,end) {
  if (plan.length%3 !== 0) throw new Error('module binding ranges need complete triples');
  let selected=0, width=Infinity;
  for (let at=0;at<plan.length;at+=3) {
    const begin=readRequiredNumberFact(plan,at), finish=readRequiredNumberFact(plan,at+1), role=readRequiredNumberFact(plan,at+2);
    if (begin<=start && finish>=end) {
      if (role === 3) return 3;
      if (finish-begin<width) { selected=role;width=finish-begin; }
    }
  }
  return selected;
}
/** Return source fragments before/after the new lexical identifier. Hosts count
 * those actual fragments in their own source coordinate units.
 * @param {number} role @param {boolean} shorthand @param {string} original @returns {string[]} */
export function readSourceBindingRenameAffixes(role,shorthand,original) {
  if (role === 3) throw new Error('an external module name is not a local binding occurrence');
  if (role === 1) return [original+' as ',''];
  if (role === 2) return ['', ' as '+original];
  return shorthand?[original+': ','']:['',''];
}

/** @param {number} code @returns {string} */
export function readSourceModuleBindingError(code) {
  if (code === 1) return 'module bindings need complete actual CST field columns';
  if (code === 2) return 'a module binding field needs one actual owner';
  if (code === 3) return 'a module specifier needs its actual name and optional alias fields';
  if (code === 4) return 'unknown module specifier field requires an explicit source profile';
  if (code === 5) return 'an external module name is not a local binding occurrence';
  return 'unknown module binding source error';
}
