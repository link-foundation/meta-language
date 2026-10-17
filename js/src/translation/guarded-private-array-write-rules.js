// Complete conservative planner over actual source syntax facts.
// Every guard, allocation and incoming ownership edge stays inspectable;
// unsupported scope or escape facts leave the original write unchanged.
/** @param {string[]} columns @param {number[]} rows @param {number} size @param {number} node @param {string} label @returns {number} */
export function readGuardedWriteField(columns,rows,size,node,label){
 if(!Number.isSafeInteger(node)||node<0||node>=size)return -1;
 const start=rows[node]??-1,count=rows[size+node]??-1;
 for(let offset=0;offset<count;offset+=1){const edge=start+offset;if((columns[2*size+edge]??'')===label)return rows[2*size+edge]??-1;}
 return -1;
}
/** @param {string[]} columns @param {number} size @param {number} node @returns {string} */
export function readGuardedWriteText(columns,size,node){return node>=0&&node<size?(columns[size+node]??''):'';}
/** @param {string[]} columns @param {number[]} rows @param {number} size @param {number} node @param {string} label @returns {string} */
export function readGuardedWriteTextField(columns,rows,size,node,label){return readGuardedWriteText(columns,size,readGuardedWriteField(columns,rows,size,node,label));}
/** @param {string[]} columns @param {number[]} rows @param {number} size @param {number} node @returns {string} */
export function readGuardedWriteKind(columns,rows,size,node){return readGuardedWriteTextField(columns,rows,size,node,'k');}
/** @param {string[]} columns @param {number[]} rows @param {number} size @param {number} node @returns {number} */
export function readGuardedWriteCount(columns,rows,size,node){return node>=0&&node<size&&(columns[node]??'')==='array'?(rows[size+node]??-1):-1;}
/** @param {string[]} columns @param {number[]} rows @param {number} size @param {number} node @param {number} index @returns {number} */
export function readGuardedWriteElement(columns,rows,size,node,index){
 const count=readGuardedWriteCount(columns,rows,size,node);
 if(!Number.isSafeInteger(index)||index<0||index>=count)return -1;
 return rows[2*size+(rows[node]??-1)+index]??-1;
}
/** @param {string[]} columns @param {number[]} rows @param {number} size @param {number} node @returns {string} */
export function readGuardedWriteName(columns,rows,size,node){
 if(readGuardedWriteKind(columns,rows,size,node)!=='name')return '';
 const path=readGuardedWriteField(columns,rows,size,node,'path');
 return readGuardedWriteCount(columns,rows,size,path)===1?readGuardedWriteText(columns,size,readGuardedWriteElement(columns,rows,size,path,0)):'';
}
/** @param {string[]} columns @param {number[]} rows @param {number} size @param {number} root @returns {boolean} */
export function acceptGuardedWriteColumns(columns,rows,size,root){
 if(!Number.isSafeInteger(size)||size<=0||!Number.isSafeInteger(root)||root<0||root>=size||columns.length<2*size||rows.length!==columns.length)return false;
 for(let node=0;node<size;node+=1){
  const start=rows[node]??-1,count=rows[size+node]??-1;
  if(!Number.isSafeInteger(start)||start<0||!Number.isSafeInteger(count)||count<0||start+count>rows.length-2*size)return false;
 }
 for(let edge=2*size;edge<rows.length;edge+=1){const target=rows[edge]??-1;if(!Number.isSafeInteger(target)||target<0||target>=size)return false;}
 return (columns[root]??'')==='object';
}
/** @param {string[]} columns @param {number[]} rows @param {number} size @param {number} node @returns {boolean} */
export function acceptGuardedWriteLiteral(columns,rows,size,node){
 if(readGuardedWriteKind(columns,rows,size,node)!=='array')return false;
 const items=readGuardedWriteField(columns,rows,size,node,'items'),count=readGuardedWriteCount(columns,rows,size,items);
 if(count<=0)return false;
 for(let index=0;index<count;index+=1){
  const item=readGuardedWriteElement(columns,rows,size,items,index);
  if(readGuardedWriteTextField(columns,rows,size,item,'spread')==='true')return false;
  const kind=readGuardedWriteKind(columns,rows,size,readGuardedWriteField(columns,rows,size,item,'value'));
  if(kind!=='num'&&kind!=='bool'&&kind!=='str')return false;
 }
 return true;
}
/** @param {string[]} columns @param {number[]} rows @param {number} size @param {number} root @returns {boolean} */
export function acceptGuardedWriteScalar(columns,rows,size,root){
 const pending=[root];let cursor=0;
 while(cursor<pending.length){
  if(cursor>size)return false;
  const node=pending[cursor]??-1;cursor+=1;
  const kind=readGuardedWriteKind(columns,rows,size,node);
  if(kind==='name'){if(readGuardedWriteName(columns,rows,size,node)==='')return false;}
  else if(kind==='binary')pending.push(readGuardedWriteField(columns,rows,size,node,'left'),readGuardedWriteField(columns,rows,size,node,'right'));
  else if(kind==='unary')pending.push(readGuardedWriteField(columns,rows,size,node,'arg'));
  else if(kind==='field'){
   if(readGuardedWriteTextField(columns,rows,size,node,'field')!=='length'||readGuardedWriteName(columns,rows,size,readGuardedWriteField(columns,rows,size,node,'object'))==='')return false;
  }else if(kind==='index'){
   if(readGuardedWriteName(columns,rows,size,readGuardedWriteField(columns,rows,size,node,'object'))==='')return false;
   pending.push(readGuardedWriteField(columns,rows,size,node,'index'));
  }else if(kind==='math'){
   const args=readGuardedWriteField(columns,rows,size,node,'args'),count=readGuardedWriteCount(columns,rows,size,args);
   if(count<0)return false;
   for(let index=0;index<count;index+=1){const arg=readGuardedWriteElement(columns,rows,size,args,index);if(readGuardedWriteTextField(columns,rows,size,arg,'spread')==='true')return false;pending.push(readGuardedWriteField(columns,rows,size,arg,'value'));}
  }else if(kind!=='num'&&kind!=='bool'&&kind!=='str'&&kind!=='none'&&kind!=='unit')return false;
 }
 return true;
}
/** @param {string[]} columns @param {number[]} rows @param {number} size @param {number} root @param {string} owner @param {string} position @returns {boolean} */
export function acceptGuardedWriteBounds(columns,rows,size,root,owner,position){
 const pending=[root];let cursor=0,integer=false,lower=false,upper=false;
 while(cursor<pending.length){
  if(cursor>size)return false;
  const node=pending[cursor]??-1;cursor+=1;
  const kind=readGuardedWriteKind(columns,rows,size,node),op=readGuardedWriteTextField(columns,rows,size,node,'op');
  if(kind==='binary'&&op==='and')pending.push(readGuardedWriteField(columns,rows,size,node,'left'),readGuardedWriteField(columns,rows,size,node,'right'));
  else if(kind==='math'&&(op==='isInteger'||op==='isSafeInteger')){
   const args=readGuardedWriteField(columns,rows,size,node,'args');
   if(readGuardedWriteCount(columns,rows,size,args)===1){const arg=readGuardedWriteElement(columns,rows,size,args,0);if(readGuardedWriteTextField(columns,rows,size,arg,'spread')!=='true'&&readGuardedWriteName(columns,rows,size,readGuardedWriteField(columns,rows,size,arg,'value'))===position)integer=true;}
  }else if(kind==='binary'&&readGuardedWriteName(columns,rows,size,readGuardedWriteField(columns,rows,size,node,'left'))===position){
   const right=readGuardedWriteField(columns,rows,size,node,'right');
   if(op==='ge'&&readGuardedWriteKind(columns,rows,size,right)==='num'&&readGuardedWriteTextField(columns,rows,size,right,'value')==='0')lower=true;
   if(op==='lt'&&readGuardedWriteKind(columns,rows,size,right)==='field'&&readGuardedWriteTextField(columns,rows,size,right,'field')==='length'&&readGuardedWriteName(columns,rows,size,readGuardedWriteField(columns,rows,size,right,'object'))===owner)upper=true;
  }
 }
 return integer&&lower&&upper;
}
/** @param {string[]} columns @param {number[]} rows @param {number} size @param {number} statement @returns {number} */
export function readGuardedWriteSourceNode(columns,rows,size,statement){
 const kind=readGuardedWriteTextField(columns,rows,size,statement,'s');
 if(kind==='assignReference')return statement;
 if(kind!=='expr')return -1;
 const expression=readGuardedWriteField(columns,rows,size,statement,'expr');
 return readGuardedWriteKind(columns,rows,size,expression)==='referenceWrite'?expression:-1;
}
/** @param {string[]} columns @param {number[]} rows @param {number} size @param {number} statement @param {string} owner @returns {number} */
export function readGuardedWriteCandidate(columns,rows,size,statement,owner){
 if(readGuardedWriteTextField(columns,rows,size,statement,'s')!=='if')return -1;
 const alternative=readGuardedWriteField(columns,rows,size,statement,'else');
 if(alternative>=0&&(columns[alternative]??'')!=='null'&&(columns[alternative]??'')!=='undefined')return -1;
 const then=readGuardedWriteField(columns,rows,size,statement,'then');if(readGuardedWriteCount(columns,rows,size,then)!==1)return -1;
 const write=readGuardedWriteSourceNode(columns,rows,size,readGuardedWriteElement(columns,rows,size,then,0));
 if(write<0||readGuardedWriteTextField(columns,rows,size,write,'op')!=='=')return -1;
 if(readGuardedWriteName(columns,rows,size,readGuardedWriteField(columns,rows,size,write,'object'))!==owner)return -1;
 const position=readGuardedWriteName(columns,rows,size,readGuardedWriteField(columns,rows,size,write,'index'));
 const condition=readGuardedWriteField(columns,rows,size,statement,'cond');
 if(position===''||!acceptGuardedWriteScalar(columns,rows,size,condition)||!acceptGuardedWriteScalar(columns,rows,size,readGuardedWriteField(columns,rows,size,write,'value'))||!acceptGuardedWriteBounds(columns,rows,size,condition,owner,position))return -1;
 return write;
}
/** @param {string[]} columns @param {number[]} rows @param {number} size @param {string} owner @param {number} allocation @param {number[]} candidates @returns {boolean} */
export function acceptGuardedWriteUses(columns,rows,size,owner,allocation,candidates){
 for(let node=0;node<size;node+=1){
  const kind=readGuardedWriteKind(columns,rows,size,node),statement=readGuardedWriteTextField(columns,rows,size,node,'s');
  if(kind==='lambda'||statement==='block'||statement==='for'||statement==='while'||statement==='doWhile'||statement==='switch')return false;
  if((statement==='assign'||statement==='const'||statement==='let')&&node!==allocation&&readGuardedWriteTextField(columns,rows,size,node,'name')===owner)return false;
 }
 for(let node=0;node<size;node+=1){
  if(readGuardedWriteName(columns,rows,size,node)!==owner)continue;
  let uses=0;
  for(let parent=0;parent<size;parent+=1){
   const start=rows[parent]??-1,count=rows[size+parent]??-1;
   for(let offset=0;offset<count;offset+=1){
    const edge=start+offset;if((rows[2*size+edge]??-1)!==node)continue;
    uses+=1;
    const label=columns[2*size+edge]??'',kind=readGuardedWriteKind(columns,rows,size,parent),statement=readGuardedWriteTextField(columns,rows,size,parent,'s');
    const read=label==='object'&&(kind==='index'||kind==='field'&&readGuardedWriteTextField(columns,rows,size,parent,'field')==='length');
    const write=label==='object'&&(statement==='assignReference'||kind==='referenceWrite')&&candidates.includes(parent);
    const transfer=label==='expr'&&statement==='return';
    if(!read&&!write&&!transfer)return false;
   }
  }
  if(uses===0)return false;
 }
 return true;
}
/** @param {string[]} columns @param {number[]} rows @param {number} size @param {number} root @returns {number[]} */
export function planGuardedPrivateArrayWrites(columns,rows,size,root){
 const result=[];
 if(!acceptGuardedWriteColumns(columns,rows,size,root))return result;
 const statements=readGuardedWriteField(columns,rows,size,root,'statements'),count=readGuardedWriteCount(columns,rows,size,statements);
 if(count<0)return result;
 for(let index=0;index<count;index+=1){
  const allocation=readGuardedWriteElement(columns,rows,size,statements,index),statement=readGuardedWriteTextField(columns,rows,size,allocation,'s');
  if((statement!=='const'&&statement!=='let')||!acceptGuardedWriteLiteral(columns,rows,size,readGuardedWriteField(columns,rows,size,allocation,'value')))continue;
  const owner=readGuardedWriteTextField(columns,rows,size,allocation,'name');if(owner==='')continue;
  const candidates=[],owners=[];
  for(let later=index+1;later<count;later+=1){const candidate=readGuardedWriteElement(columns,rows,size,statements,later),write=readGuardedWriteCandidate(columns,rows,size,candidate,owner);if(write>=0){candidates.push(write);owners.push(candidate);}}
  if(candidates.length===0||!acceptGuardedWriteUses(columns,rows,size,owner,allocation,candidates))continue;
  for(let at=0;at<candidates.length;at+=1)result.push(owners[at]??-1,candidates[at]??-1,allocation);
 }
 return result;
}
