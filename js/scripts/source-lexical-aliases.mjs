// Share an already bound immutable value or a short Boolean/text literal.
// Property reads, calls, arithmetic and allocations remain actual expressions.
export function shareSourceLexicalAliases(program,entry,body,proofs){
 if(program.sourceLanguage!=='JavaScript')return body;
 let visits=0;const excluded=new Set(['type','domain','from','to','span','pattern']);
 function bound(pattern){return pattern?.k==='ctor'?(pattern.binds??[]).filter(name=>name!==null):['bind','natSucc'].includes(pattern?.k)?[pattern.name]:[];}
 function captureRisk(value,first,second){
  const pending=[value],seen=new WeakSet();
  while(pending.length){
   const node=pending.pop();if(!node||typeof node!=='object'||seen.has(node))continue;seen.add(node);
   if(++visits>1000000)throw new TypeError('lexical alias ownership exceeds its work budget');
   if(node.k==='lambda'||node.k==='let'&&[first,second].includes(node.name)||bound(node.pattern).some(name=>[first,second].includes(name)))return true;
   if(node.pattern&&!['ctor','wild','bind','natSucc','natZero','bool','lit'].includes(node.pattern.k))return true;
   for(const [key,child]of Object.entries(node))if(!excluded.has(key))pending.push(child);
  }
  return false;
 }
 function replace(node,name,target,depth=0,literal=null){
  if(!node||typeof node!=='object')return node;
  if(depth>256||++visits>1000000)throw new TypeError('lexical alias substitution exceeds its work budget');
  if(node.k==='var'&&node.name===name)return literal?{k:'lit',type:node.type,value:literal.value,...(node.span?{span:node.span}:{})}:{...node,name:target};
  return Array.isArray(node)?node.map(child=>replace(child,name,target,depth+1,literal)):Object.fromEntries(Object.entries(node).map(([key,child])=>[key,excluded.has(key)?child:replace(child,name,target,depth+1,literal)]));
 }
 function rewrite(node,scope,path,depth=0){
  if(!node||typeof node!=='object')return node;
  if(depth>256||++visits>1000000)throw new TypeError('lexical alias traversal exceeds its work budget');
  if(node.k==='lambda')return node;
  if(node.k==='let'){
   const value=rewrite(node.value,scope,[...path,'value'],depth+1),inner=new Set(scope);inner.add(node.name);
   const next=rewrite(node.body,inner,[...path,'body'],depth+1);
   if(value?.k==='lit'&&(value.type?.kind==='bool'&&typeof value.value==='boolean'||value.type?.kind==='string'&&typeof value.value==='string'&&value.value.length<=128)&&Object.keys(value).every(key=>['k','type','value','span'].includes(key))&&!captureRisk(next,node.name,node.name)){
    proofs.push({operation:'substitute checked primitive literal',owner:entry.fullName,path,binding:node.name,literal:value,preserveEffects:true});
    return replace(next,node.name,'',0,value);
   }
   if(value?.k==='var'&&scope.has(value.name)&&value.name!==node.name&&!captureRisk(next,node.name,value.name)){
    // Every occurrence was checked against this binder before rewriting. A
    // shadowed endpoint or deferred closure conservatively keeps the alias.
    const replaced=replace(next,node.name,value.name);
    proofs.push({operation:'substitute immutable lexical alias',owner:entry.fullName,path,binding:node.name,target:value.name,preserveValue:true,preserveEffects:true});
    return replaced;
   }
   return value===node.value&&next===node.body?node:{...node,value,body:next};
  }
  if(node.k==='match'){
   if(node.cases.some(arm=>!['ctor','wild','bind','natSucc','natZero','bool','lit'].includes(arm.pattern?.k)))return node;
   return {...node,scrutinee:rewrite(node.scrutinee,scope,[...path,'scrutinee'],depth+1),cases:node.cases.map((arm,index)=>{const inner=new Set(scope);for(const name of bound(arm.pattern))inner.add(name);return {...arm,body:rewrite(arm.body,inner,[...path,'cases',index,'body'],depth+1)};})};
  }
  return Array.isArray(node)?node.map((child,index)=>rewrite(child,scope,[...path,index],depth+1)):Object.fromEntries(Object.entries(node).map(([key,child])=>[key,excluded.has(key)?child:rewrite(child,scope,[...path,key],depth+1)]));
 }
 return rewrite(body,new Set(entry.params.map(parameter=>parameter.name)),['body']);
}
