// Static declaration reachability for the actual source module's export set.
// Complete original dependencies remain captured; no body is executed or stubbed.
export function projectSourceLibraryEntryPoints(program,closure,path){
 if(program.main?.effects.length||program.externals?.length)throw new TypeError('source export projection requires a closed function library');
 const module=closure?.inputs.find(input=>input.path===path);if(!module||!Array.isArray(module.exports)||module.exports.length===0)throw new TypeError('source export projection needs its actual captured module');
 const roots=module.exports.map(name=>module.compiledDeclarations?.[module.declarations.indexOf(name)]??name),retained=new Set(),pending=[...roots];
 for(const [name,entry]of program.declarations)if(entry.k==='data'&&!entry.generated)pending.push(name);
 const seen=new WeakSet();let visits=0;
 function dependency(name){if(typeof name!=='string'||!program.declarations.has(name))throw new TypeError('an actual source dependency lost its checked declaration: '+name);pending.push(name);}
 function walk(value){
  if(!value||typeof value!=='object'||seen.has(value))return;seen.add(value);
  if(++visits>100000)throw new TypeError('source export reachability exceeds its reference budget');
  if(value.k==='call'||value.k==='functionRef')dependency(value.fn);
  if(value.k==='ctor')dependency(value.data);
  if(value.kind==='data')dependency(value.name);
  for(const child of Array.isArray(value)?value:Object.values(value))walk(child);
 }
 while(pending.length){const name=pending.pop();if(retained.has(name))continue;const entry=program.declarations.get(name);if(!entry)throw new TypeError('a source entry point has no actual declaration: '+name);retained.add(name);walk(entry);}
 function items(values){return values.flatMap(item=>item.k==='module'?[{...item,items:items(item.items)}]:program.declarations.get(item.fullName)===item?(retained.has(item.fullName)?[item]:[]):(()=>{throw new TypeError('source export projection needs actual checked declaration/module items');})());}
 return {program:{...program,declarations:new Map([...program.declarations].filter(([name])=>retained.has(name))),items:items(program.items)},
  projection:{model:'actual source module exports and transitive declarations 1',entryPoints:roots,retained:[...program.declarations.keys()].filter(name=>retained.has(name)),omitted:[...program.declarations.keys()].filter(name=>!retained.has(name))}};
}
