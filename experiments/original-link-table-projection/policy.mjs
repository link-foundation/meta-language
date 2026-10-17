// Generator input for whole fixed-symbol certificate tables, not source decisions.
import {readProjectData} from '../../js/scripts/project-data.mjs';
import {originalLinkRolePolicy,originalLinkRoleRulesSource} from '../original-link-role-projection/policy.mjs';
const certificate=readProjectData(new URL('../rust-text-certificate/data/policy.lino',import.meta.url));
const cell=(op,path=[],root)=>({op,path,...(root?{root}:{})});
export const originalLinkTablePolicy={
 version:1,model:'original link fixed-symbol certificate tables 1',
 sourceModel:certificate.sourceModel,certificateModel:certificate.model,
 maximumMembership:originalLinkRolePolicy.maximumMembership,maximumEdges:originalLinkRolePolicy.maximumEdges,maximumOccurrences:originalLinkRolePolicy.maximumOccurrences,
 translation:originalLinkRolePolicy.translation,
 fields:[...originalLinkRolePolicy.fields,'items','schemaVersion','sourceLanguage','effects','worker','model','language','observationModel','span','start','end','functions','declaration','signature','resultPosition','operationNode','coreValue','input','position','literal','inputAssumptions','valueAssumptions','id','meaning','type','owner','path','wrappers','lifetimes','arguments','op'],
 symbols:[...certificate.symbols,'source operation'],certificateSymbolCount:certificate.symbols.length,
 scalarKinds:{other:0,null:1,string:2,number:3,boolean:4},
 operationMarker:{symbol:'source operation',scalarKind:'string',root:'proof',path:['operations','kind']},
 operations:[...certificate.operations.map(row=>({symbol:certificate.symbols.indexOf(row.name)+1,code:row.code})),...Object.entries(certificate.operationCodes).map(([name,code])=>({symbol:certificate.symbols.indexOf({function:'text value function',body:'text value body',main:'independent empty main',root:'complete standard Rust text value module'}[name])+1,code}))],
 tables:[
  {name:'context',rows:'program',columns:[cell('number',['schemaVersion']),cell('symbol',['sourceLanguage']),cell('identity',['main']),cell('length',['effects'],'main'),cell('length',['items']),cell('length',['declarations']),cell('identity',['worker'],'proof'),cell('symbol',['model'],'proof'),cell('symbol',['language'],'proof'),cell('symbol',['observationModel'],'proof'),cell('identity',['operations'],'proof'),cell('number',['span','start'],'proof'),cell('number',['span','end'],'proof'),cell('number',['span','start'],'main'),cell('number',['span','end'],'main')]},
  {name:'positions',rows:'positions',columns:[cell('identity'),cell('signature'),cell('ordinal'),cell('symbol',['type','kind']),cell('symbol',['owner','kind']),cell('prefix paths'),cell('length',['owner','path']),cell('prefix layers'),cell('length',['wrappers']),cell('length',['lifetimes']),cell('length',['owner','arguments'])]},
  {name:'paths',rows:'positions',form:'symbols',path:['owner','path']},
  {name:'layers',rows:'positions',form:'wrapper lifetime pairs',path:['wrappers'],lifetimePath:['lifetimes']},
  {name:'functions',rows:'functions',columns:['declaration','signature','resultPosition','operationNode','coreValue','input','position','literal'].map(name=>cell('identity',[name]))},
  {name:'operations',rows:'operations',columns:[cell('identity'),cell('operation',['op']),cell('number',['span','start']),cell('number',['span','end']),cell('prefix parts'),cell('length',['parts'])]},
  {name:'parts',rows:'operations',form:'identities',path:['parts']},
  {name:'inputs',rows:'inputs',columns:[cell('symbol',['id']),cell('symbol',['meaning']),...['declaration','operationNode','coreValue','input','position'].map(name=>cell('identity',[name]))]},
  {name:'allocations',rows:'allocations',columns:[cell('symbol',['id']),cell('symbol',['meaning']),...['declaration','operationNode','coreValue','input','position'].map(name=>cell('identity',[name]))]},
  {name:'items',rows:'items',columns:[cell('identity')]},
 ],
 output:['context','kinds','positions','paths','layers','functions','operations','parts','inputs','allocations','items'],
 refusals:{primitiveTransport:[[-21]],root:[[-10]],rootOperationMarker:[[-22]],registry:'one row retaining the existing role-registry refusal vector'},
 remaining:['tokens','declarations','parameters','cores','names','units'],
 transport:{domain:'Dense actual primitive vectors; original IDs and ordered relation/label/child anchors remain separate. Every root/edge/registry handle is positive integral and bounded. Scalar tags and dictionary codes are bounded before interpretation.',numbers:'Primitive Number payloads retain signed zero, NaN and infinities. The old source checker and final certificate determine admission; projection never normalizes these payloads.',null:'Actual null scalar ->0; missing field or nonregistered object ->-1. Unknown fixed symbol ->-1, while the primitive dictionary transport uses unknown code0.',membership:'The existing exact-ID lookup and role registry remain mandatory. No equal-name or equal-Value identity substitution.',cost:'Repeated immutable scans/copies remain bounded but potentially quadratic. No native resource or performance claim.'},
 boundary:'Eleven complete table values including reused kinds; the remaining six arbitrary-String tables, native pre-Serde host integration, Program re-encoding and paired source capture/replay remain open. No production selector or source admission changes.',
};

export function originalLinkTableRulesSource(){
 const p=originalLinkTablePolicy,f=name=>{const code=p.fields.indexOf(name)+1;if(!code)throw new Error('unregistered table field '+name);return code;};
 const fld=(owner,path)=>path.reduce((node,name)=>`readOriginalLinkRoleField(fieldParents,fieldLabels,fieldChildren,${node},${f(name)})`,owner);
 const refs=owner=>`readOriginalLinkRoleReferences(referenceParents,referenceChildren,${owner})`;
 const identity=node=>`readOriginalLinkTableIdentity(anchors,scalarKinds,${node})`;
 const symbol=node=>`readOriginalLinkTableSymbol(scalarKinds,scalarSymbols,${node})`;
 const number=node=>`readOriginalLinkTableNumber(scalarKinds,scalarNumbers,${node})`;
 const expression=c=>{
  const node=fld(c.root??'node',c.path);
  if(c.op==='identity')return identity(node);if(c.op==='symbol')return symbol(node);if(c.op==='number')return number(node);if(c.op==='length')return refs(node)+'.length';
  if(c.op==='operation')return `readOriginalLinkTableOperationCode(${symbol(node)})`;
  if(c.op==='signature')return identity('(positionSignatures[index]??-1)');if(c.op==='ordinal')return '(positionOrdinals[index]??-1)';
  if(c.op==='prefix paths')return 'readOriginalLinkTablePrefix(fieldParents,fieldLabels,fieldChildren,referenceParents,referenceChildren,positions,index,1)';
  if(c.op==='prefix layers')return 'readOriginalLinkTablePrefix(fieldParents,fieldLabels,fieldChildren,referenceParents,referenceChildren,positions,index,2)';
  if(c.op==='prefix parts')return 'readOriginalLinkTablePrefix(fieldParents,fieldLabels,fieldChildren,referenceParents,referenceChildren,operations,index,3)';
  throw new Error('unrepresented table cell '+c.op);
 };
 const tableSource=p.tables.map(table=>{
  if(table.form==='symbols'||table.form==='identities')return ` const table_${table.name}=${table.rows}.flatMap(node=>${refs(fld('node',table.path))}.map(value=>${table.form==='symbols'?symbol('value'):identity('value')}));`;
  if(table.form==='wrapper lifetime pairs')return ` const table_${table.name}=${table.rows}.flatMap(node=>readOriginalLinkTableLayers(fieldParents,fieldLabels,fieldChildren,referenceParents,referenceChildren,scalarKinds,scalarSymbols,node));`;
  return ` const table_${table.name}=${table.rows}.flatMap((node,index)=>[${table.columns.map(expression).join(',')}]);`;
 }).join('\n');
 return originalLinkRoleRulesSource()+`
// Generated whole-table projection from the linked column instructions.
/** @param {number[]} anchors @param {number[]} scalarKinds @param {number} node @returns {number} */
function readOriginalLinkTableIdentity(anchors,scalarKinds,node){if(node<1)return -1;if((scalarKinds[node-1]??0)===1)return 0;const index=anchors.indexOf(node);return index<0?-1:index+1;}
/** @param {number[]} scalarKinds @param {number[]} symbols @param {number} node @returns {number} */
function readOriginalLinkTableSymbol(scalarKinds,symbols,node){if((scalarKinds[node-1]??0)!==2)return -1;const symbol=symbols[node-1]??0;return symbol<1||symbol>${p.certificateSymbolCount}?-1:symbol;}
/** @param {number[]} scalarKinds @param {number[]} numbers @param {number} node @returns {number} */
function readOriginalLinkTableNumber(scalarKinds,numbers,node){return (scalarKinds[node-1]??0)===3?(numbers[node-1]??-1):-1;}
/** @param {number} symbol @returns {number} */
function readOriginalLinkTableOperationCode(symbol){${p.operations.map(row=>`if(symbol===${row.symbol})return ${row.code};`).join('')}return -1;}
/** @param {number[]} anchors @param {number[]} kinds @param {number} role @returns {number[]} */
function readOriginalLinkTableRole(anchors,kinds,role){return anchors.map((member,index)=>(kinds[index+1]??-1)===role?member:0).filter(value=>value>0);}
/** @param {number[]} fieldParents @param {number[]} fieldLabels @param {number[]} fieldChildren @param {number[]} referenceParents @param {number[]} referenceChildren @param {number[]} nodes @param {number} limit @param {number} measure @returns {number} */
function readOriginalLinkTablePrefix(fieldParents,fieldLabels,fieldChildren,referenceParents,referenceChildren,nodes,limit,measure){let result=0;for(let index=0;index<limit;index++){const node=nodes[index]??-1;if(measure===1)result=result+${refs(fld('node',['owner','path']))}.length;else if(measure===2)result=result+2*Math.max(${refs(fld('node',['wrappers']))}.length,${refs(fld('node',['lifetimes']))}.length);else result=result+${refs(fld('node',['parts']))}.length;}return result;}
/** @param {number[]} fieldParents @param {number[]} fieldLabels @param {number[]} fieldChildren @param {number[]} referenceParents @param {number[]} referenceChildren @param {number[]} scalarKinds @param {number[]} scalarSymbols @param {number} node @returns {number[]} */
function readOriginalLinkTableLayers(fieldParents,fieldLabels,fieldChildren,referenceParents,referenceChildren,scalarKinds,scalarSymbols,node){const wrappers=${refs(fld('node',['wrappers']))};const lifetimes=${refs(fld('node',['lifetimes']))};const seed=wrappers.length>=lifetimes.length?wrappers:lifetimes;return seed.flatMap((value,index)=>[${symbol('(wrappers[index]??-1)')},(scalarKinds[(lifetimes[index]??-1)-1]??0)===1?0:${symbol('(lifetimes[index]??-1)')}]);}
/** @param {number} membershipCount @param {number} root @param {number[]} fieldParents @param {number[]} fieldLabels @param {number[]} fieldChildren @param {number[]} referenceParents @param {number[]} referenceChildren @param {number[]} scalarKinds @param {number[]} scalarNumbers @param {number[]} scalarSymbols @returns {number[][]} */
export function projectOriginalLinkCertificateTables(membershipCount,root,fieldParents,fieldLabels,fieldChildren,referenceParents,referenceChildren,scalarKinds,scalarNumbers,scalarSymbols){
 if(fieldParents.length!==fieldLabels.length||fieldParents.length!==fieldChildren.length||referenceParents.length!==referenceChildren.length||fieldParents.length>${p.maximumEdges}||referenceParents.length>${p.maximumEdges}||scalarKinds.length!==membershipCount||scalarNumbers.length!==membershipCount||scalarSymbols.length!==membershipCount)return [[-21]];
 if(!Number.isInteger(root)||root<1||root>membershipCount)return [[-10]];if(membershipCount>${p.maximumMembership})return [[-1]];
 if(${['fieldParents','fieldChildren','referenceParents','referenceChildren'].map(name=>`${name}.findIndex(value=>!Number.isInteger(value)||value<1||value>membershipCount)>=0`).join('||')}||fieldLabels.findIndex(value=>!Number.isInteger(value)||value<0||value>${p.fields.length})>=0||scalarKinds.findIndex(value=>!Number.isInteger(value)||value<0||value>4)>=0||scalarSymbols.findIndex(value=>!Number.isInteger(value)||value<0||value>${p.symbols.length})>=0||scalarNumbers.findIndex(value=>!Number.isFinite(value)&&!Number.isNaN(value)&&value!==Infinity&&value!==-Infinity)>=0)return [[-21]];
 const rootValues=${refs('root')};if(rootValues.length!==1)return [[-10]];const body=rootValues[0]??-1;const proof=${fld('body',['sourceTextValueProjection'])};const main=${fld('body',['main'])};
 const operationMarker=${fld(p.operationMarker.root,p.operationMarker.path)};if((scalarKinds[operationMarker-1]??0)!==${p.scalarKinds[p.operationMarker.scalarKind]}||(scalarSymbols[operationMarker-1]??0)!==${p.symbols.indexOf(p.operationMarker.symbol)+1})return [[-22]];
 const registry=projectOriginalLinkRoleRegistry(membershipCount,root,fieldParents,fieldLabels.map(value=>value>${originalLinkRolePolicy.fields.length}?0:value),fieldChildren,referenceParents,referenceChildren,scalarSymbols.map((value,index)=>(scalarKinds[index]??0)===${p.scalarKinds[p.operationMarker.scalarKind]}&&value===${p.symbols.indexOf(p.operationMarker.symbol)+1}?1:0));if((registry[0]??-1)!==1)return [registry];
 const count=registry[1]??0;const anchors=registry.slice(3,3+count);const kinds=registry.slice(3+count,4+2*count);
 const program=[body];
 const signatures=readOriginalLinkTableRole(anchors,kinds,4);const positions=readOriginalLinkTableRole(anchors,kinds,5);const operations=readOriginalLinkTableRole(anchors,kinds,7);
 const positionSignatures=signatures.flatMap(node=>${refs(fld('node',['parameters']))}.concat([${fld('node',['result'])}]).map(value=>node));
 const positionOrdinals=signatures.flatMap(node=>${refs(fld('node',['parameters']))}.map((value,index)=>index).concat([-1]));
 const functions=${refs(fld('proof',['functions']))};const inputs=${refs(fld('proof',['inputAssumptions']))};const allocations=${refs(fld('proof',['valueAssumptions']))};const items=${refs(fld('body',['items']))};
${tableSource}
 return [${p.output.map(name=>name==='kinds'?'kinds':'table_'+name).join(',')}];
}
`;
}
