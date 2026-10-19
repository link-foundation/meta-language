// Generator input: original graph roles, never a second source-language checker.
import {readProjectData} from '../../js/scripts/project-data.mjs';
const certificate=readProjectData(new URL('../rust-text-certificate/data/policy.lino',import.meta.url));
export const originalLinkRolePolicy={
 version:1,model:'original link certificate role registration 1',
 sourceModel:certificate.sourceModel,certificateModel:certificate.model,
 translation:{referenceSemantics:false,premise:'All primitive numeric arrays are read-only values. No indexed writes, reference equality, alias mutation or escaping callbacks are observed.'},
 maximumMembership:100000,maximumOccurrences:4096,maximumEdges:1000000,
 fields:['sourceTextValueProjection','declarations','tokens','params','parameters','sourceSignature','result','body','main','operations','parts','kind'],
 stringSymbols:['source operation'],identityKinds:certificate.identityKinds,
 transportDomains:{root:{minimum:1,maximum:'membershipCount'},ordinals:['fieldParents','fieldChildren','referenceParents','referenceChildren'],labels:{vector:'fieldLabels',minimum:0,maximum:'fields.length'},strings:{vector:'stringValues',minimum:0,maximum:'stringSymbols.length'},precedence:['vector shape','root domain','membership bound','edge and code domains','role interpretation'],holes:'findIndex visits every numeric array position, including holes; missing values refuse'},
 instructions:[
  {name:'tokens',role:certificate.identityKinds.token,root:'proof',path:['tokens'],form:'references'},
  {name:'declarations',role:certificate.identityKinds.declaration,root:'body',path:['declarations'],form:'mapping values'},
  {name:'parameters',role:certificate.identityKinds.parameter,root:'declarations',path:['params'],form:'flat references'},
  {name:'signatures',role:certificate.identityKinds.signature,root:'declarations',path:['sourceSignature'],form:'fields'},
  {name:'positions',role:certificate.identityKinds.position,root:'signatures',path:['parameters','result'],form:'signature positions'},
  {name:'cores',role:certificate.identityKinds.core,root:'declarations',path:['body'],form:'fields'},
  {name:'operations',role:certificate.identityKinds.operation,root:'proof',path:['operations','parts'],form:'operation preorder'},
  {name:'mains',role:certificate.identityKinds.main,root:'body',path:['main'],form:'field'},
 ],
 output:{success:['1','registered count','occurrence count','registered original membership ordinals','kinds including leading zero','ordered occurrence handles'],refusals:{membership:[-1],shape:[-2],member:[-3,'occurrence'],role:[-4,'occurrence'],contradiction:[-5,'occurrence'],unregistered:[-6,'original membership ordinal'],root:[-10],transport:[-11],operations:[-12]}},
 interpretation:[
  'Original LinkIds and relation/label/child references stay in host transport; exact lookup uses the existing original-link-identity kernel. No u64 is converted to a Number.',
  'Before role interpretation, every dense root/edge handle is an integral Number from one through the actual membership count. Label and string codes are integral Numbers from zero through their shared dictionary bounds; zero retains unknown symbols.',
  'Only actual token, declaration, parameter, signature, position, core, operation and main roles become certificate identities. Membership of unrelated nodes creates no kind-table row.',
  'Repeated original identities share one compact handle. Equal-valued distinct original IDs remain distinct. Role 9 can be provisional but cannot remain unregistered; contradictory concrete roles refuse.',
  'The selected archive must first pass the unchanged existing raw text-certificate checker. This partial projection neither chooses source operations nor proves ownership from membership.',
  'The generated registry replaces the kinds table. Comparison remaps identity columns of the other sixteen existing tables through actual raw role correspondence, leaving their values with the existing projector.',
  'Primitive map/filter/slice/concat preserve immutable array values. Repeated scans and copied snapshots have bounded but potentially quadratic cost; no native execution or resource claim follows.',
  'The complete Rust output is automatically translated source data, not a native package adapter. Remaining table-value projection, direct native Program ownership and paired capture/replay are still gaps.',
 ],
};

export function originalLinkRoleRulesSource(){
 const p=originalLinkRolePolicy,f=name=>p.fields.indexOf(name)+1;
 const field=(owner,name)=>`readOriginalLinkRoleField(fieldParents,fieldLabels,fieldChildren,${owner},${f(name)})`;
 const refs=owner=>`readOriginalLinkRoleReferences(referenceParents,referenceChildren,${owner})`;
 const lines=p.instructions.map(row=>{
  const owner=row.root;
  if(row.form==='references')return ` const ${row.name}=${refs(field(owner,row.path[0]))};`;
  if(row.form==='mapping values')return ` const ${row.name}=${refs(field(owner,row.path[0]))}.map(pair=>${refs('pair')}[1]??-1);`;
  if(row.form==='flat references')return ` const ${row.name}=${owner}.flatMap(node=>${refs(field('node',row.path[0]))});`;
  if(row.form==='fields')return ` const ${row.name}=${owner}.map(node=>${field('node',row.path[0])});`;
  if(row.form==='signature positions')return ` const ${row.name}=${owner}.flatMap(node=>${refs(field('node','parameters'))}.concat([${field('node','result')}]));`;
  if(row.form==='field')return ` const ${row.name}=[${field(owner,row.path[0])}];`;
  if(row.form==='operation preorder')return ` let operations=[${field(owner,row.path[0])}];let pending=operations;let steps=0;
 while(pending.length>0){
  if(steps>=${p.maximumOccurrences})return [-12];steps=steps+1;
  const node=pending[0]??-1;
  const children=${refs(field('node',row.path[1]))}.filter(child=>readOriginalLinkRoleOperation(fieldParents,fieldLabels,fieldChildren,stringValues,child));
  pending=children.concat(pending.slice(1));
  if(steps>1)operations=operations.concat([node]);
 }`;
  throw new Error('unrepresented shared role instruction '+row.form);
 }).join('\n');
 const names=p.instructions.map(row=>row.name);
 return `// Generated from the linked original-ID role instructions. Do not edit.
/** @param {number[]} parents @param {number[]} labels @param {number[]} children @param {number} owner @param {number} label @returns {number} */
function readOriginalLinkRoleField(parents,labels,children,owner,label){let result=-1;for(let index=0;index<parents.length;index++){if((parents[index]??-1)===owner&&(labels[index]??-1)===label){if(result!==-1)return -1;result=children[index]??-1;}}return result;}
/** @param {number[]} parents @param {number[]} children @param {number} owner @returns {number[]} */
function readOriginalLinkRoleReferences(parents,children,owner){return children.map((child,index)=>(parents[index]??-1)===owner?child:0).filter(value=>value>0);}
/** @param {number[]} fieldParents @param {number[]} fieldLabels @param {number[]} fieldChildren @param {number[]} strings @param {number} node @returns {boolean} */
function readOriginalLinkRoleOperation(fieldParents,fieldLabels,fieldChildren,strings,node){const kind=${field('node','kind')};return (strings[kind-1]??-1)===1;}
/** @param {number[]} members @param {number} member @param {number} limit @returns {boolean} */
function firstOriginalLinkRoleOccurrence(members,member,limit){for(let index=0;index<limit;index++)if((members[index]??-1)===member)return false;return true;}
/** @param {number[]} members @param {number[]} roles @param {number} member @returns {number} */
function readOriginalLinkRoleKind(members,roles,member){let result=9;for(let index=0;index<members.length;index++)if((members[index]??-1)===member&&(roles[index]??9)!==9)result=roles[index]??9;return result;}
/** @param {number} membershipCount @param {number[]} members @param {number[]} roles @returns {number[]} */
export function registerOriginalLinkRoles(membershipCount,members,roles){
 if(!Number.isInteger(membershipCount)||membershipCount<0||membershipCount>${p.maximumMembership})return [-1];
 if(members.length!==roles.length||members.length>${p.maximumOccurrences})return [-2];
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
 if(fieldParents.length!==fieldLabels.length||fieldParents.length!==fieldChildren.length||referenceParents.length!==referenceChildren.length||fieldParents.length>${p.maximumEdges}||referenceParents.length>${p.maximumEdges}||stringValues.length!==membershipCount)return [-11];
 if(!Number.isInteger(root)||root<${p.transportDomains.root.minimum}||root>membershipCount)return [-10];
 if(membershipCount>${p.maximumMembership})return [-1];
 if(${p.transportDomains.ordinals.map(name=>`${name}.findIndex(value=>!Number.isInteger(value)||value<1||value>membershipCount)>=0`).concat([`${p.transportDomains.labels.vector}.findIndex(value=>!Number.isInteger(value)||value<${p.transportDomains.labels.minimum}||value>${p.fields.length})>=0`,`${p.transportDomains.strings.vector}.findIndex(value=>!Number.isInteger(value)||value<${p.transportDomains.strings.minimum}||value>${p.stringSymbols.length})>=0`]).join('||')})return [-11];
 const rootValues=${refs('root')};if(rootValues.length!==1)return [-10];
 const body=rootValues[0]??-1;const proof=${field('body','sourceTextValueProjection')};
${lines}
 const members=${names[0]}.concat(${names.slice(1).join(',')});
 const roles=${names[0]}.map(value=>${p.instructions[0].role}).concat(${p.instructions.slice(1).map(row=>`${row.name}.map(value=>${row.role})`).join(',')});
 return registerOriginalLinkRoles(membershipCount,members,roles);
}
`;
}
