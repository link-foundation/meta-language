// Source-author successor only; no generated schema is edited or executed.
export function extendPropositionValueSchema(before){
 const after=structuredClone(before),install=(table,key,value)=>{
  if(!after[table]||typeof after[table]!=='object')throw new TypeError('proposition values need the complete schema table '+table);
  if(Object.hasOwn(after[table],key)&&JSON.stringify(after[table][key])!==JSON.stringify(value))throw new TypeError('preserve conflicting proposition value schema '+table+'.'+key);
  after[table][key]=value;
 };
 install('expressionFields','propositionValue',['k','op','left','right','domain','sameValue','type','span']);
 install('expressionOperands','propositionValue',['left','right']);
 install('expressionOperations','propositionValue',['eq','ne','lt','le','gt','ge']);
 install('expressionProjections','propositionValue',{construct:'propositionValue',fields:[['prop','proposition-comparison','left']]});
 return after;
}
