// Original public names are captured before private target relocation. This
// transports complete alias scopes; no source graph is rewritten or serialized.
import {unsupported} from './diagnostics.js';
import {readFormalDatatypeAliasBudgetError,readFormalDatatypeAliasTraversalError,renderFormalDatatypeTypeAlias,renderFormalDatatypeConstructorAlias,renderFormalDatatypeAliasModule} from './formal-datatype-alias-rules.js';
const key=path=>JSON.stringify(path);
const prefix=(parent,child)=>parent.length<=child.length&&parent.every((part,index)=>part===child[index]);
const fail=reason=>{throw unsupported('formal datatype public aliases',reason);};
export function captureFormalDatatypeAliases(aliases,state){let count=0;for(const alias of aliases){count+=1+alias.constructors.length;const error=readFormalDatatypeAliasBudgetError(count,0);if(error)fail(error);}return aliases.map(alias=>({...alias,publicName:state.localName(alias.sourceFullName),publicConstructors:alias.constructors.map(name=>state.ctorLocal(alias.sourceFullName,name))}));}
export class FormalDatatypeAliases{
 constructor(rows,target,state){this.rows=rows;this.count=rows.reduce((count,row)=>count+1+row.constructors.length,0);this.required=new Set(rows.map(row=>row.targetFullName));this.target=target;this.state=state;this.completed=new Set();this.emitted=new Set();this.opened=new Set();this.units=0;this.visits=0;const error=readFormalDatatypeAliasBudgetError(this.count,0);if(error)fail(error);}
 step(depth){const error=readFormalDatatypeAliasTraversalError(++this.visits,depth);if(error)fail(error);}
 complete(entries){for(const entry of entries)if(this.required.has(entry.fullName))this.completed.add(entry.fullName);}
 append(lines,value){if(!value)fail('the selected target needs its complete alias syntax');this.units+=value.length;const error=readFormalDatatypeAliasBudgetError(this.count,this.units);if(error)fail(error);lines.push(value);}
 open(path){if(!this.rows.length)return [];this.step(path.length);const scope=key(path);if(this.opened.has(scope)&&this.rows.some(row=>{this.step(path.length);return prefix(path,row.sourceModulePath);}))fail('an original alias module would have to reopen');this.opened.add(scope);const lines=[];
  for(const row of this.rows){this.step(path.length);if(key(row.sourceModulePath)===scope&&!this.emitted.has(row.sourceFullName)){
   if(!this.completed.has(row.targetFullName))fail('the original alias scope opened before its complete lifted datatype block');
   this.append(lines,renderFormalDatatypeTypeAlias(this.target,row.publicName,this.state.ref(row.targetFullName)));
   for(let index=0;index<row.constructors.length;index+=1)this.append(lines,renderFormalDatatypeConstructorAlias(this.target,row.publicName,row.publicConstructors[index],this.state.ctorRef(row.targetFullName,row.constructors[index])));
   this.emitted.add(row.sourceFullName);
  }}return lines;
 }
 close(path){if(!this.rows.length)return [];const lines=[];
  this.step(path.length);for(;;){const row=this.rows.find(row=>{this.step(path.length);return !this.emitted.has(row.sourceFullName)&&prefix(path,row.sourceModulePath);});if(!row)break;if(row.sourceModulePath.length===path.length)fail('the complete original alias scope was not emitted when opened');
   const child=[...path,row.sourceModulePath[path.length]],name=this.state.moduleName(child);if(!name)fail('the original alias module has no actual reserved target name');
   this.append(lines,renderFormalDatatypeAliasModule(this.target,name,true));lines.push(...this.open(child),...this.close(child));this.append(lines,renderFormalDatatypeAliasModule(this.target,name,false));
  }return lines;
 }
 finish(){if(this.emitted.size!==this.rows.length)fail('every relocated datatype and constructor needs its original public alias');}
}
