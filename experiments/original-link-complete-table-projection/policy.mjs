// Executable projection recipe input; runtime source is generated from this data.
import {readProjectData} from '../../js/scripts/project-data.mjs';
import {originalLinkTablePolicy,originalLinkTableRulesSource} from '../original-link-table-projection/policy.mjs';
import {originalLinkSymbolPolicy,originalLinkSymbolRulesSource} from '../original-link-symbol-projection/policy.mjs';
const certificate=readProjectData(new URL('../rust-text-certificate/data/policy.lino',import.meta.url));
const cell=(op,path=[])=>({op,path});
export const originalLinkCompleteTablePolicy={
 version:1,model:'complete original link closed Rust text certificate table projection 1',
 sourceModel:certificate.sourceModel,certificateModel:certificate.model,
 fields:[...new Set([...originalLinkTablePolicy.fields,'k','name','fullName','modulePath','ret','recursive','decreasing','lifetimeBindings','guard','default','value','raw','binding','literal','bindingReference'])],
 fixedSymbols:certificate.symbols,maximumMembership:originalLinkTablePolicy.maximumMembership,maximumEdges:originalLinkTablePolicy.maximumEdges,maximumStrings:originalLinkSymbolPolicy.maximumStrings,maximumUnits:originalLinkSymbolPolicy.maximumUnits,
 translation:originalLinkTablePolicy.translation,
 queries:{stages:['tokens','declarations','inputs','allocations','context'],tableColumnOperations:['symbol','token syntax','token value','mapping key','core value'],declarationStages:['parameters','positions','core','header'],position:[cell('references',['owner','path']),cell('lifetime layers'),cell('symbol',['type','kind']),cell('symbol',['owner','kind'])],condition:[cell('symbol',['id']),cell('symbol',['meaning'])],context:[{...cell('symbol',['sourceLanguage']),owner:'body'},{...cell('symbol',['model']),owner:'proof'},{...cell('symbol',['language']),owner:'proof'},{...cell('symbol',['observationModel']),owner:'proof'}],bindingValue:'An elided core Var name reads its actual binding-reference target.name; raw names remain primitive values. This never resolves owner identity by equal names.'},
 layouts:{token:certificate.layouts.token,declaration:certificate.layouts.declaration,parameter:certificate.layouts.parameter,core:certificate.layouts.core},
 output:['context','kinds','tokens','declarations','parameters','positions','paths','layers','cores','functions','operations','parts','inputs','allocations','names','units','items'],
 tables:[
  {name:'tokens',rows:'tokens',columns:[cell('identity'),cell('symbol',['kind']),cell('token syntax'),cell('token role'),cell('token target'),cell('token value'),cell('symbol',['raw']),cell('number',['span','start']),cell('number',['span','end'])]},
  {name:'declarations',rows:'declarations',columns:[cell('identity'),cell('symbol',['name']),cell('symbol',['fullName']),cell('length',['modulePath']),cell('identity',['sourceSignature']),cell('identity',['body']),cell('parameter prefix'),cell('length',['params']),cell('symbol',['ret','kind']),cell('boolean',['recursive']),cell('identity',['decreasing']),cell('number',['span','start']),cell('number',['span','end']),cell('mapping key'),cell('symbol',['sourceSignature','model']),cell('symbol',['sourceSignature','language']),cell('length',['sourceSignature','lifetimeBindings']),cell('symbol',['k']),cell('length',['sourceSignature','parameters']),cell('identity',['sourceSignature','result'])]},
  {name:'parameters',rows:'parameters',columns:[cell('identity'),cell('parameter owner'),cell('parameter ordinal'),cell('symbol',['name']),cell('symbol',['type','kind']),cell('identity',['guard']),cell('identity',['default']),cell('parameter position')]},
  {name:'cores',rows:'cores',columns:[cell('identity'),cell('core owner'),cell('symbol',['k']),cell('symbol',['type','kind']),cell('core value'),cell('number',['span','start']),cell('number',['span','end'])]},
 ],
 refusals:{transport:[[-30]],queries:[[-31]],scalarSymbols:[[-32]],symbols:'one row retaining the unchanged symbol-kernel failure',fixedTables:'the unchanged fixed-table failure'},
 boundary:'All17 table values are projected from original-ID primitive transport. Existing raw certificate checking remains mandatory. This does not add a production source selector, native pre-Serde adapter, native Program re-encoding or paired source capture/replay. Native execution is separate CI work.',
};

export function originalLinkCompleteTableRulesSource(){
 const p=originalLinkCompleteTablePolicy;const fixedStarts=[],fixedLengths=[],fixedUnits=[];for(const value of p.fixedSymbols){fixedStarts.push(fixedUnits.length);fixedLengths.push(value.length);for(let at=0;at<value.length;at++)fixedUnits.push(value.charCodeAt(at));}const markerUnits=Array.from('source operation',value=>value.charCodeAt(0));const f=name=>{const code=p.fields.indexOf(name)+1;if(!code)throw new Error('unregistered complete-table field '+name);return code;};
 const fld=(node,path)=>path.reduce((owner,name)=>`readOriginalLinkRoleField(fieldParents,fieldLabels,fieldChildren,${owner},${f(name)})`,node);
 const refs=node=>`readOriginalLinkRoleReferences(referenceParents,referenceChildren,${node})`;
 const identity=node=>`readOriginalLinkTableIdentity(anchors,scalarKinds,${node})`;
 const symbol=node=>`readOriginalLinkCompleteSymbol(queries,symbols,${node})`;
 const number=node=>`readOriginalLinkTableNumber(scalarKinds,scalarNumbers,${node})`;
 const rendered=node=>`readOriginalLinkCompleteValue(fieldParents,fieldLabels,fieldChildren,${node})`;
 const tokenRole=node=>`readOriginalLinkCompleteTokenRole(fieldParents,fieldLabels,fieldChildren,${node})`;
 const target=node=>`readOriginalLinkCompleteTokenTarget(fieldParents,fieldLabels,fieldChildren,${node})`;
 const tokenValue=node=>`readOriginalLinkCompleteTokenValue(fieldParents,fieldLabels,fieldChildren,${node})`;
 const queryColumns=name=>p.tables.find(table=>table.name===name).columns.filter(column=>p.queries.tableColumnOperations.includes(column.op));
 const queryCell=(column,node)=>{if(column.op==='symbol')return node==='node'&&column.path[0]==='sourceSignature'?fld('signature',column.path.slice(1)):fld(node,column.path);if(column.op==='token value')return tokenValue(node);if(column.op==='mapping key')return 'entries[0]??-1';if(column.op==='core value')return rendered(node);throw new Error('unrepresented scalar query '+column.op);};
 const queryArray=(columns,node)=>{const parts=[],values=[];const flush=()=>{if(values.length){parts.push('['+values.join(',')+']');values.length=0;}};for(const column of columns){if(column.op==='token syntax'){flush();parts.push(`${tokenRole(node)}===0?[${fld(node,['value'])}]:[]`);}else values.push(queryCell(column,node));}flush();return parts.length===1?parts[0]:parts[0]+'.concat('+parts.slice(1).join(',')+')';};
 const positionQueries=p.queries.position.map(column=>column.op==='references'?refs(fld('node',column.path)):column.op==='lifetime layers'?`seed.flatMap((value,index)=>[(wrappers[index]??-1)].concat((lifetimes[index]??-1)<1||(scalarKinds[(lifetimes[index]??-1)-1]??0)===1?[]:[lifetimes[index]??-1]))`:queryCell(column,'node'));
 const positionQuerySource=positionQueries[0]+'.concat('+positionQueries[1]+',['+positionQueries.slice(2).join(',')+'])';
 const declarationQuerySources={parameters:'parameters',positions:'positions',core:queryColumns('cores').map(column=>queryCell(column,'core')),header:queryColumns('declarations').map(column=>queryCell(column,'node'))};
 const declarationQueryOrder=p.queries.declarationStages.map(stage=>declarationQuerySources[stage]);const declarationQuerySource=declarationQueryOrder[0]+'.concat('+declarationQueryOrder[1]+',['+declarationQueryOrder.slice(2).flat().join(',')+'])';
 const stageQueries={tokens:'tokenQueries',declarations:'declarationQueries',inputs:'inputs',allocations:'allocations',context:'['+p.queries.context.map(column=>queryCell(column,column.owner)).join(',')+']'};const stageQueryOrder=p.queries.stages.map(stage=>stageQueries[stage]);const querySource=stageQueryOrder[0]+'.concat('+stageQueryOrder.slice(1).join(',')+')';
 const cells=c=>{
  const node=fld('node',c.path);
  if(c.op==='identity')return identity(node);if(c.op==='symbol')return symbol(node);if(c.op==='number')return number(node);if(c.op==='length')return refs(node)+'.length';
  if(c.op==='boolean')return `((scalarKinds[${node}-1]??0)===4?(scalarBooleans[${node}-1]??-1):-1)`;
  if(c.op==='token syntax')return `${tokenRole('node')}>0?0:${symbol(fld('node',['value']))}`;
  if(c.op==='token role')return tokenRole('node');if(c.op==='token target')return `${tokenRole('node')}>0?${identity(target('node'))}:0`;if(c.op==='token value')return symbol(tokenValue('node'));
  if(c.op==='mapping key')return symbol('(mappingKeys[index]??-1)');if(c.op==='core value')return symbol(rendered('node'));
  if(c.op==='core owner')return identity('(declarations[index]??-1)');
  if(c.op==='parameter prefix')return 'readOriginalLinkCompleteParameterPrefix(fieldParents,fieldLabels,fieldChildren,referenceParents,referenceChildren,declarations,index)*8';
  if(c.op==='parameter owner')return identity('(parameterOwners[index]??-1)');if(c.op==='parameter ordinal')return '(parameterOrdinals[index]??-1)';if(c.op==='parameter position')return identity('(parameterPositions[index]??-1)');
  throw new Error('unrepresented complete-table cell '+c.op);
 };
 return originalLinkTableRulesSource()+'\n'+originalLinkSymbolRulesSource()+`
// Generated String traversal and remaining complete table values.
/** @param {number[]} fieldParents @param {number[]} fieldLabels @param {number[]} fieldChildren @param {number} node @returns {number} */
function readOriginalLinkCompleteTokenRole(fieldParents,fieldLabels,fieldChildren,node){if(${fld('node',['declaration'])}>0)return 1;if(${fld('node',['binding'])}>0)return 2;if(${fld('node',['literal'])}>0)return 3;return 0;}
/** @param {number[]} fieldParents @param {number[]} fieldLabels @param {number[]} fieldChildren @param {number} node @returns {number} */
function readOriginalLinkCompleteTokenTarget(fieldParents,fieldLabels,fieldChildren,node){const role=${tokenRole('node')};return role===1?${fld('node',['declaration'])}:role===2?${fld('node',['binding'])}:role===3?${fld('node',['literal'])}:-1;}
/** @param {number[]} fieldParents @param {number[]} fieldLabels @param {number[]} fieldChildren @param {number} node @returns {number} */
function readOriginalLinkCompleteTokenValue(fieldParents,fieldLabels,fieldChildren,node){const role=${tokenRole('node')};const target=${target('node')};return role===1||role===2?${fld('target',['name'])}:role===3?${fld('target',['value'])}:${fld('node',['value'])};}
/** @param {number[]} fieldParents @param {number[]} fieldLabels @param {number[]} fieldChildren @param {number} node @returns {number} */
function readOriginalLinkCompleteValue(fieldParents,fieldLabels,fieldChildren,node){const name=${fld('node',['name'])};if(name>0)return name;const binding=${fld('node',['bindingReference'])};return binding>0?${fld('binding',['name'])}:${fld('node',['value'])};}
/** @param {number[]} queries @param {number[]} symbols @param {number} node @returns {number} */
function readOriginalLinkCompleteSymbol(queries,symbols,node){const index=queries.indexOf(node);return index<0?-1:(symbols[index]??-1);}
/** @param {number[]} rawUnits @param {number} start @param {number} length @param {number[]} fixedStarts @param {number[]} fixedLengths @param {number[]} fixedUnits @returns {number} */
function readOriginalLinkCompleteScalarSymbol(rawUnits,start,length,fixedStarts,fixedLengths,fixedUnits){for(let index=0;index<fixedLengths.length;index++)if(equalOriginalLinkSymbolUnits(rawUnits,start,length,fixedUnits,fixedStarts[index]??-1,fixedLengths[index]??-1))return index+1;return equalOriginalLinkSymbolUnits(rawUnits,start,length,${JSON.stringify(markerUnits)},0,${markerUnits.length})?${originalLinkTablePolicy.symbols.indexOf('source operation')+1}:0;}
/** @param {number[]} lengths @param {number} limit @returns {number} */
function readOriginalLinkCompleteStringPrefix(lengths,limit){let result=0;for(let index=0;index<limit;index++)if((lengths[index]??-1)>=0)result=result+(lengths[index]??0);return result;}
/** @param {number[]} fieldParents @param {number[]} fieldLabels @param {number[]} fieldChildren @param {number[]} referenceParents @param {number[]} referenceChildren @param {number[]} declarations @param {number} limit @returns {number} */
function readOriginalLinkCompleteParameterPrefix(fieldParents,fieldLabels,fieldChildren,referenceParents,referenceChildren,declarations,limit){let result=0;for(let index=0;index<limit;index++){const node=declarations[index]??-1;result=result+${refs(fld('node',['params']))}.length;}return result;}
/** @param {number[]} fieldParents @param {number[]} fieldLabels @param {number[]} fieldChildren @param {number[]} referenceParents @param {number[]} referenceChildren @param {number[]} scalarKinds @param {number} node @returns {number[]} */
function readOriginalLinkCompletePositionQueries(fieldParents,fieldLabels,fieldChildren,referenceParents,referenceChildren,scalarKinds,node){const wrappers=${refs(fld('node',['wrappers']))};const lifetimes=${refs(fld('node',['lifetimes']))};const seed=wrappers.length>=lifetimes.length?wrappers:lifetimes;return ${positionQuerySource};}
/** @param {number} root @param {number[]} fieldParents @param {number[]} fieldLabels @param {number[]} fieldChildren @param {number[]} referenceParents @param {number[]} referenceChildren @param {number[]} scalarKinds @returns {number[]} */
export function readOriginalLinkCompleteStringQueries(root,fieldParents,fieldLabels,fieldChildren,referenceParents,referenceChildren,scalarKinds){
 const body=${refs('root')}[0]??-1;const proof=${fld('body',['sourceTextValueProjection'])};const tokens=${refs(fld('proof',['tokens']))};const pairs=${refs(fld('body',['declarations']))};
 const tokenQueries=tokens.flatMap(node=>${queryArray(queryColumns('tokens'),'node')});
 const declarationQueries=pairs.flatMap(pair=>{
  const entries=${refs('pair')};const node=entries[1]??-1;const signature=${fld('node',['sourceSignature'])};const core=${fld('node',['body'])};
  const parameters=${refs(fld('node',['params']))}.flatMap(input=>${queryArray(queryColumns('parameters'),'input')});
  const positions=${refs(fld('signature',['parameters']))}.concat([${fld('signature',['result'])}]).flatMap(position=>readOriginalLinkCompletePositionQueries(fieldParents,fieldLabels,fieldChildren,referenceParents,referenceChildren,scalarKinds,position));
  return ${declarationQuerySource};
 });
 const inputs=${refs(fld('proof',['inputAssumptions']))}.flatMap(node=>${queryArray(p.queries.condition,'node')});const allocations=${refs(fld('proof',['valueAssumptions']))}.flatMap(node=>${queryArray(p.queries.condition,'node')});
 return ${querySource};
}
/** @param {number} membershipCount @param {number} root @param {number[]} fieldParents @param {number[]} fieldLabels @param {number[]} fieldChildren @param {number[]} referenceParents @param {number[]} referenceChildren @param {number[]} scalarKinds @param {number[]} scalarNumbers @param {number[]} scalarSymbols @param {number[]} scalarBooleans @param {number[]} rawStarts @param {number[]} rawLengths @param {number[]} rawUnits @param {number[]} fixedStarts @param {number[]} fixedLengths @param {number[]} fixedUnits @returns {number[][]} */
export function projectOriginalLinkCompleteCertificateTables(membershipCount,root,fieldParents,fieldLabels,fieldChildren,referenceParents,referenceChildren,scalarKinds,scalarNumbers,scalarSymbols,scalarBooleans,rawStarts,rawLengths,rawUnits,fixedStarts,fixedLengths,fixedUnits){
 if(fieldParents.length!==fieldLabels.length||fieldParents.length!==fieldChildren.length||referenceParents.length!==referenceChildren.length||fieldParents.length>${p.maximumEdges}||referenceParents.length>${p.maximumEdges}||scalarKinds.length!==membershipCount||scalarNumbers.length!==membershipCount||scalarSymbols.length!==membershipCount||scalarBooleans.length!==membershipCount||rawStarts.length!==membershipCount||rawLengths.length!==membershipCount)return [[-30]];
 if(!Number.isInteger(root)||root<1||root>membershipCount)return [[-10]];if(membershipCount>${p.maximumMembership})return [[-1]];
 if(fieldLabels.findIndex(value=>!Number.isInteger(value)||value<0||value>${p.fields.length})>=0||scalarBooleans.length!==membershipCount||rawStarts.length!==membershipCount||rawLengths.length!==membershipCount||scalarBooleans.findIndex(value=>!Number.isInteger(value)||value< -1||value>1)>=0||!checkOriginalLinkSymbolTransport(rawStarts,rawLengths,rawUnits,true))return [[-30]];
 const fixed=projectOriginalLinkCertificateTables(membershipCount,root,fieldParents,fieldLabels.map(value=>value>${originalLinkTablePolicy.fields.length}?0:value),fieldChildren,referenceParents,referenceChildren,scalarKinds,scalarNumbers,scalarSymbols);if(fixed.length!==${originalLinkTablePolicy.output.length})return fixed;
 if(rawLengths.findIndex((value,index)=>(scalarKinds[index]??0)===2?value<0:value!==-1)>=0)return [[-30]];
 if(!equalOriginalLinkSymbolUnits(fixedStarts,0,fixedStarts.length,${JSON.stringify(fixedStarts)},0,${fixedStarts.length})||!equalOriginalLinkSymbolUnits(fixedLengths,0,fixedLengths.length,${JSON.stringify(fixedLengths)},0,${fixedLengths.length})||!equalOriginalLinkSymbolUnits(fixedUnits,0,fixedUnits.length,${JSON.stringify(fixedUnits)},0,${fixedUnits.length}))return [[-32]];
 if(scalarSymbols.findIndex((value,index)=>value!==((scalarKinds[index]??0)===2?readOriginalLinkCompleteScalarSymbol(rawUnits,rawStarts[index]??-1,rawLengths[index]??-1,fixedStarts,fixedLengths,fixedUnits):0))>=0)return [[-32]];
 const queries=readOriginalLinkCompleteStringQueries(root,fieldParents,fieldLabels,fieldChildren,referenceParents,referenceChildren,scalarKinds);if(queries.length>${p.maximumStrings}||queries.findIndex(value=>!Number.isInteger(value)||value< -1||value===0||value>membershipCount)>=0)return [[-31]];
 const lengths=queries.map(node=>node>0&&(scalarKinds[node-1]??0)===2?(rawLengths[node-1]??-1):-1);if(readOriginalLinkCompleteStringPrefix(lengths,lengths.length)>${p.maximumUnits})return [[-31]];const starts=lengths.map((value,index)=>readOriginalLinkCompleteStringPrefix(lengths,index));const queryUnits=queries.flatMap(node=>node>0&&(scalarKinds[node-1]??0)===2?rawUnits.slice(rawStarts[node-1]??0,(rawStarts[node-1]??0)+(rawLengths[node-1]??0)):[]);
 const interned=projectOriginalLinkSymbolValues(fixedStarts,fixedLengths,fixedUnits,starts,lengths,queryUnits);if((interned[0]??-1)!==1)return [interned];
 const symbols=interned.slice(4,4+queries.length);const nameCount=interned[2]??0;const table_names=interned.slice(4+queries.length,4+queries.length+nameCount*2);const table_units=interned.slice(4+queries.length+nameCount*2);
 const registry=projectOriginalLinkRoleRegistry(membershipCount,root,fieldParents,fieldLabels.map(value=>value>12?0:value),fieldChildren,referenceParents,referenceChildren,scalarSymbols.map((value,index)=>(scalarKinds[index]??0)===2&&value===${originalLinkTablePolicy.symbols.indexOf('source operation')+1}?1:0));const anchors=registry.slice(3,3+(registry[1]??0));
 const body=${refs('root')}[0]??-1;const proof=${fld('body',['sourceTextValueProjection'])};const tokens=${refs(fld('proof',['tokens']))};const pairs=${refs(fld('body',['declarations']))};const mappingKeys=pairs.map(pair=>${refs('pair')}[0]??-1);const declarations=pairs.map(pair=>${refs('pair')}[1]??-1);const cores=declarations.map(node=>${fld('node',['body'])});
 const parameters=declarations.flatMap(node=>${refs(fld('node',['params']))});const parameterOwners=declarations.flatMap(node=>${refs(fld('node',['params']))}.map(value=>node));const parameterOrdinals=declarations.flatMap(node=>${refs(fld('node',['params']))}.map((value,index)=>index));const parameterPositions=declarations.flatMap(node=>${refs(fld('node',['sourceSignature','parameters']))});
${p.tables.map(table=>` const table_${table.name}=${table.rows}.flatMap((node,index)=>[${table.columns.map(cells).join(',')}]);`).join('\n')}
 return [${p.output.map(name=>originalLinkTablePolicy.output.includes(name)?`fixed[${originalLinkTablePolicy.output.indexOf(name)}]??[]`:'table_'+name).join(',')}];
}
`;
}
