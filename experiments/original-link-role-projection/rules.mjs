// Generated from the linked original-ID role instructions. Do not edit.
/** @param {number[]} parents @param {number[]} labels @param {number[]} children @param {number} owner @param {number} label @returns {number} */
function readOriginalLinkRoleField(parents,labels,children,owner,label){let result=-1;for(let index=0;index<parents.length;index++){if((parents[index]??-1)===owner&&(labels[index]??-1)===label){if(result!==-1)return -1;result=children[index]??-1;}}return result;}
/** @param {number[]} parents @param {number[]} children @param {number} owner @returns {number[]} */
function readOriginalLinkRoleReferences(parents,children,owner){return children.map((child,index)=>(parents[index]??-1)===owner?child:0).filter(value=>value>0);}
/** @param {number[]} fieldParents @param {number[]} fieldLabels @param {number[]} fieldChildren @param {number[]} strings @param {number} node @returns {boolean} */
function readOriginalLinkRoleOperation(fieldParents,fieldLabels,fieldChildren,strings,node){const kind=readOriginalLinkRoleField(fieldParents,fieldLabels,fieldChildren,node,12);return (strings[kind-1]??-1)===1;}
/** @param {number[]} members @param {number} member @param {number} limit @returns {boolean} */
function firstOriginalLinkRoleOccurrence(members,member,limit){for(let index=0;index<limit;index++)if((members[index]??-1)===member)return false;return true;}
/** @param {number[]} members @param {number[]} roles @param {number} member @returns {number} */
function readOriginalLinkRoleKind(members,roles,member){let result=9;for(let index=0;index<members.length;index++)if((members[index]??-1)===member&&(roles[index]??9)!==9)result=roles[index]??9;return result;}
/** @param {number} membershipCount @param {number[]} members @param {number[]} roles @returns {number[]} */
export function registerOriginalLinkRoles(membershipCount,members,roles){
 if(!Number.isInteger(membershipCount)||membershipCount<0||membershipCount>100000)return [-1];
 if(members.length!==roles.length||members.length>4096)return [-2];
 for(let index=0;index<members.length;index++){
  const member=members[index]??-1;const role=roles[index]??-1;
  if(!Number.isInteger(member)||member<1||member>membershipCount)return [-3,index];
  if(!Number.isInteger(role)||role<1||role>9)return [-4,index];
  for(let previous=0;previous<index;previous++)if((members[previous]??-1)===member&&role!==9&&(roles[previous]??9)!==9&&(roles[previous]??9)!==role)return [-5,index];
 }
 const anchors=members.map((member,index)=>firstOriginalLinkRoleOccurrence(members,member,index)?member:0).filter(value=>value>0);
 const kinds=anchors.map(member=>readOriginalLinkRoleKind(members,roles,member));
 for(let index=0;index<kinds.length;index++)if((kinds[index]??9)===9)return [-6,anchors[index]??-1];
 const handles=members.map(member=>anchors.indexOf(member)+1);
 return [1,anchors.length,members.length].concat(anchors,[0].concat(kinds),handles);
}
/** @param {number} membershipCount @param {number} root @param {number[]} fieldParents @param {number[]} fieldLabels @param {number[]} fieldChildren @param {number[]} referenceParents @param {number[]} referenceChildren @param {number[]} stringValues @returns {number[]} */
export function projectOriginalLinkRoleRegistry(membershipCount,root,fieldParents,fieldLabels,fieldChildren,referenceParents,referenceChildren,stringValues){
 if(fieldParents.length!==fieldLabels.length||fieldParents.length!==fieldChildren.length||referenceParents.length!==referenceChildren.length||fieldParents.length>1000000||referenceParents.length>1000000||stringValues.length!==membershipCount)return [-11];
 if(!Number.isInteger(root)||root<1||root>membershipCount)return [-10];
 if(membershipCount>100000)return [-1];
 if(fieldParents.findIndex(value=>!Number.isInteger(value)||value<1||value>membershipCount)>=0||fieldChildren.findIndex(value=>!Number.isInteger(value)||value<1||value>membershipCount)>=0||referenceParents.findIndex(value=>!Number.isInteger(value)||value<1||value>membershipCount)>=0||referenceChildren.findIndex(value=>!Number.isInteger(value)||value<1||value>membershipCount)>=0||fieldLabels.findIndex(value=>!Number.isInteger(value)||value<0||value>12)>=0||stringValues.findIndex(value=>!Number.isInteger(value)||value<0||value>1)>=0)return [-11];
 const rootValues=readOriginalLinkRoleReferences(referenceParents,referenceChildren,root);if(rootValues.length!==1)return [-10];
 const body=rootValues[0]??-1;const proof=readOriginalLinkRoleField(fieldParents,fieldLabels,fieldChildren,body,1);
 const tokens=readOriginalLinkRoleReferences(referenceParents,referenceChildren,readOriginalLinkRoleField(fieldParents,fieldLabels,fieldChildren,proof,3));
 const declarations=readOriginalLinkRoleReferences(referenceParents,referenceChildren,readOriginalLinkRoleField(fieldParents,fieldLabels,fieldChildren,body,2)).map(pair=>readOriginalLinkRoleReferences(referenceParents,referenceChildren,pair)[1]??-1);
 const parameters=declarations.flatMap(node=>readOriginalLinkRoleReferences(referenceParents,referenceChildren,readOriginalLinkRoleField(fieldParents,fieldLabels,fieldChildren,node,4)));
 const signatures=declarations.map(node=>readOriginalLinkRoleField(fieldParents,fieldLabels,fieldChildren,node,6));
 const positions=signatures.flatMap(node=>readOriginalLinkRoleReferences(referenceParents,referenceChildren,readOriginalLinkRoleField(fieldParents,fieldLabels,fieldChildren,node,5)).concat([readOriginalLinkRoleField(fieldParents,fieldLabels,fieldChildren,node,7)]));
 const cores=declarations.map(node=>readOriginalLinkRoleField(fieldParents,fieldLabels,fieldChildren,node,8));
 let operations=[readOriginalLinkRoleField(fieldParents,fieldLabels,fieldChildren,proof,10)];let pending=operations;let steps=0;
 while(pending.length>0){
  if(steps>=4096)return [-12];steps=steps+1;
  const node=pending[0]??-1;
  const children=readOriginalLinkRoleReferences(referenceParents,referenceChildren,readOriginalLinkRoleField(fieldParents,fieldLabels,fieldChildren,node,11)).filter(child=>readOriginalLinkRoleOperation(fieldParents,fieldLabels,fieldChildren,stringValues,child));
  pending=children.concat(pending.slice(1));
  if(steps>1)operations=operations.concat([node]);
 }
 const mains=[readOriginalLinkRoleField(fieldParents,fieldLabels,fieldChildren,body,9)];
 const members=tokens.concat(declarations,parameters,signatures,positions,cores,operations,mains);
 const roles=tokens.map(value=>1).concat(declarations.map(value=>2),parameters.map(value=>3),signatures.map(value=>4),positions.map(value=>5),cores.map(value=>6),operations.map(value=>7),mains.map(value=>8));
 return registerOriginalLinkRoles(membershipCount,members,roles);
}
