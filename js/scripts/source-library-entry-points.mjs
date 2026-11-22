// Static declaration reachability for the actual source module's export set.
// Complete original dependencies remain captured; no body is executed or stubbed.
export function projectSourceLibraryEntryPoints(program,closure,path){
 if(program.main?.effects.length||program.externals?.length)throw new TypeError('source export projection requires a closed function library');
 const module=closure?.inputs.find(input=>input.path===path);if(!module||!Array.isArray(module.exports)||module.exports.length===0)throw new TypeError('source export projection needs its actual captured module');
 const roots=module.exports.map(name=>module.compiledExports?.find(row=>row.exported===name)?.compiled??module.compiledDeclarations?.[module.declarations.indexOf(name)]??name);
 const retained=new Set(),pending=[];
 function retain(name){
  if(typeof name!=='string'||!program.declarations.has(name))throw new TypeError('an actual source dependency lost its checked declaration: '+name);
  if(!retained.has(name)){retained.add(name);pending.push(name);}
 }
 for(const name of roots)retain(name);
 for(const [name,entry]of program.declarations)if(entry.k==='data'&&!entry.generated)retain(name);
 while(pending.length){
  const name=pending.pop(),seen=new WeakSet(),references=[program.declarations.get(name)];
  let visits=0;
  while(references.length){
   const value=references.pop();if(!value||typeof value!=='object'||seen.has(value))continue;seen.add(value);
   if(++visits>100000)throw new TypeError('source export reachability exceeds the reference budget for '+name);
   if(value.k==='call'||value.k==='functionRef')retain(value.fn);
   if(value.k==='ctor')retain(value.data);
   if(value.kind==='data')retain(value.name);
   if(value.kind==='record'&&typeof value.sourceClassOwner==='string'){
    const owner=program.sourceClassOwners?.owners.find(owner=>owner.constructor===value.sourceClassOwner);if(!owner)throw new TypeError('a source class entry point lost its actual constructor witness');retain(owner.constructor);for(const method of owner.methods)retain(method.helper);
   }
   for(const child of Array.isArray(value)?value:Object.values(value))references.push(child);
  }
 }
 function items(values){return values.flatMap(item=>item.k==='module'?[{...item,items:items(item.items)}]:program.declarations.get(item.fullName)===item?(retained.has(item.fullName)?[item]:[]):(()=>{throw new TypeError('source export projection needs actual checked declaration/module items');})());}
 const selected={...program,declarations:new Map([...program.declarations].filter(([name])=>retained.has(name))),items:items(program.items)};
 if(program.sourceClassOwners){const owners=program.sourceClassOwners.owners.filter(owner=>retained.has(owner.constructor));if(owners.length)selected.sourceClassOwners={...program.sourceClassOwners,owners};else delete selected.sourceClassOwners;}
 return {program:selected,
  projection:{model:'actual source module exports and transitive declarations 1',entryPoints:roots,retained:[...program.declarations.keys()].filter(name=>retained.has(name)),omitted:[...program.declarations.keys()].filter(name=>!retained.has(name))}};
}
