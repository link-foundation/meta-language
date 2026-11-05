// Persistent class ownership is checked against actual constructor allocations,
// helper signatures and type edges. Generated identifier spelling is no proof.
import {readRequiredNumberFact as number,readRequiredTextFact as text} from './fact-column-rules.js';
import {readCollectionConstructionField as field,readCollectionConstructionElement as element} from './collection-construction-rules.js';
import {readJavaScriptClosedClassObservationOperationOperands} from './javascript-class-observation-rules.js';
import {readRecordFieldWriteName} from './record-write-rules.js';
import {containsJavaScriptClosedClassBrand} from './javascript-class-observation-rules.js';
/** @returns {string} */
export function readJavaScriptClassOwnerModel(){return 'closed JavaScript class data owners 1';}
/** @param {string} left @param {string} right @param {boolean} leftOpen @param {boolean} rightOpen @returns {string} */
export function readJavaScriptClassOwnerInference(left,right,leftOpen,rightOpen){if(left===right)return 'same';if(left!==''&&right===''&&rightOpen)return 'right';if(right!==''&&left===''&&leftOpen)return 'left';return 'error';}
/** @param {string[]} constructors @param {string[]} brands @param {string[]} fields @returns {string} */
export function readJavaScriptClassRecordOwner(constructors,brands,fields){let owner='';for(let at=0;at<brands.length;at+=1)if(fields.includes(text(brands,at))){if(owner!=='')return '#ambiguous';owner=text(constructors,at);}return owner;}
/** @param {string} representation @param {string} operation @returns {boolean} */
export function acceptJavaScriptClassOwnerObservation(representation,operation){return representation==='closure slots'&&operation==='identity';}
/** @param {string[]} forms @param {string[]} texts @param {number[]} starts @param {number[]} counts @param {string[]} labels @param {number[]} targets @param {number} node @param {string} name @returns {string} */
function word(forms,texts,starts,counts,labels,targets,node,name){const value=field(starts,counts,labels,targets,node,name);return value>=0&&text(forms,value)==='string'?text(texts,value):'';}
/** @param {number[]} starts @param {number[]} counts @param {string[]} labels @param {number[]} targets @param {number} node @param {string[]} names @returns {boolean} */
function exact(starts,counts,labels,targets,node,names){if(node<0||number(counts,node)!==names.length)return false;for(const name of names)if(field(starts,counts,labels,targets,node,name)<0)return false;return true;}
/** @param {string[]} forms @param {string[]} texts @param {number[]} starts @param {number[]} counts @param {string[]} labels @param {number[]} targets @param {number} node @returns {string[]} */
function strings(forms,texts,starts,counts,labels,targets,node){let result=[];if(node<0||text(forms,node)!=='array')return [''];for(let at=0;at<number(counts,node);at+=1){const value=element(starts,counts,targets,node,at);if(text(forms,value)!=='string'||text(texts,value)==='')return [''];result=result.concat([text(texts,value)]);}return result;}
/** @param {string[]} forms @param {string[]} texts @param {number[]} starts @param {number[]} counts @param {string[]} labels @param {number[]} targets @param {number} node @returns {string[]} */
function names(forms,texts,starts,counts,labels,targets,node){let result=[];const fields=field(starts,counts,labels,targets,node,'fields');if(fields<0)return [''];for(let at=0;at<number(counts,fields);at+=1)result=result.concat([word(forms,texts,starts,counts,labels,targets,element(starts,counts,targets,fields,at),'name')]);return result;}
/** @param {string[]} forms @param {string[]} texts @param {number[]} starts @param {number[]} counts @param {string[]} labels @param {number[]} targets @param {number} type @param {string} owner @param {number} depth @returns {boolean} */
function carries(forms,texts,starts,counts,labels,targets,type,owner,depth){if(type<0||depth>128)return false;if(word(forms,texts,starts,counts,labels,targets,type,'sourceClassOwner')===owner)return true;if(word(forms,texts,starts,counts,labels,targets,type,'kind')!=='tuple')return false;const elements=field(starts,counts,labels,targets,type,'elements');if(elements<0)return false;for(let at=0;at<number(counts,elements);at+=1)if(carries(forms,texts,starts,counts,labels,targets,element(starts,counts,targets,elements,at),owner,depth+1))return true;return false;}
/** @param {string[]} forms @param {string[]} texts @param {number[]} starts @param {number[]} counts @param {string[]} labels @param {number[]} targets @param {number} node @param {string} owner @param {number} depth @returns {boolean} */
function payload(forms,texts,starts,counts,labels,targets,node,owner,depth){if(node<0||depth>256)return false;const kind=word(forms,texts,starts,counts,labels,targets,node,'k');if(kind==='record')return word(forms,texts,starts,counts,labels,targets,field(starts,counts,labels,targets,node,'type'),'sourceClassOwner')===owner;if(kind==='let'||kind==='print'||kind==='cellWrite')return payload(forms,texts,starts,counts,labels,targets,field(starts,counts,labels,targets,node,'body'),owner,depth+1);if(kind==='if')return payload(forms,texts,starts,counts,labels,targets,field(starts,counts,labels,targets,node,'then'),owner,depth+1)&&payload(forms,texts,starts,counts,labels,targets,field(starts,counts,labels,targets,node,'else'),owner,depth+1);return false;}
/** Only actual return/value flow counts: conditions and discarded arguments
 * cannot certify the constructor's returned instance.
 * @param {string[]} forms @param {string[]} texts @param {number[]} starts @param {number[]} counts @param {string[]} labels @param {number[]} targets @param {number} node @param {string} owner @param {string[]} bindings @param {number[]} values @param {number} depth @returns {number} */
function fresh(forms,texts,starts,counts,labels,targets,node,owner,bindings,values,depth){
 if(node<0||depth>256)return -1;const type=field(starts,counts,labels,targets,node,'type');if(!carries(forms,texts,starts,counts,labels,targets,type,owner,0))return -2;
 const kind=word(forms,texts,starts,counts,labels,targets,node,'k');
 if(kind==='referenceAllocate'){const reference=field(starts,counts,labels,targets,type,'reference');return reference>=0&&text(forms,reference)==='boolean'&&text(texts,reference)==='true'&&payload(forms,texts,starts,counts,labels,targets,field(starts,counts,labels,targets,node,'value'),owner,0)?node:-1;}
 if(kind==='var'){const name=word(forms,texts,starts,counts,labels,targets,node,'name');for(let at=bindings.length-1;at>=0;at-=1)if(text(bindings,at)===name)return number(values,at);return -1;}
 if(kind==='let'){const value=field(starts,counts,labels,targets,node,'value'),origin=fresh(forms,texts,starts,counts,labels,targets,value,owner,bindings,values,depth+1);return fresh(forms,texts,starts,counts,labels,targets,field(starts,counts,labels,targets,node,'body'),owner,bindings.concat([word(forms,texts,starts,counts,labels,targets,node,'name')]),values.concat([origin]),depth+1);}
 if(kind==='print'||kind==='cellWrite')return fresh(forms,texts,starts,counts,labels,targets,field(starts,counts,labels,targets,node,'body'),owner,bindings,values,depth+1);
 if(kind==='if'){const left=fresh(forms,texts,starts,counts,labels,targets,field(starts,counts,labels,targets,node,'then'),owner,bindings,values,depth+1),right=fresh(forms,texts,starts,counts,labels,targets,field(starts,counts,labels,targets,node,'else'),owner,bindings,values,depth+1);return left>=0&&left===right?left:-1;}
 if(kind==='tuple'){let origin=-2;const items=field(starts,counts,labels,targets,node,'items');if(items<0)return -1;for(let at=0;at<number(counts,items);at+=1){const value=fresh(forms,texts,starts,counts,labels,targets,element(starts,counts,targets,items,at),owner,bindings,values,depth+1);if(value===-1||value>=0&&origin>=0&&value!==origin)return -1;if(value>=0)origin=value;}return origin;}
 if(kind==='tupleIndex')return fresh(forms,texts,starts,counts,labels,targets,field(starts,counts,labels,targets,node,'object'),owner,bindings,values,depth+1);
 return -1;
}
/** Validate raw witness shapes, every owner-bearing type, actual constructor
 * result flow and every observable operand. typeKeys are existing host type
 * identities for actual child types; this policy never constructs a type.
 * @param {string[]} forms @param {string[]} texts @param {number[]} starts @param {number[]} counts @param {string[]} labels @param {number[]} targets @param {string[]} typeKeys @param {number} root @returns {string} */
export function readJavaScriptClassOwnerError(forms,texts,starts,counts,labels,targets,typeKeys,root){
 if(starts.length>1048576||starts.length!==forms.length||starts.length!==texts.length||starts.length!==counts.length||starts.length!==typeKeys.length||labels.length!==targets.length)return 'class ownership exceeds its complete graph budget';
 const witness=field(starts,counts,labels,targets,root,'sourceClassOwners'),declarations=field(starts,counts,labels,targets,root,'declarations');
 let constructors=[],brands=[],representations=[],ownerNodes=[],constructorNodes=[];
 if(witness>=0){
  if(text(forms,witness)!=='object'||!exact(starts,counts,labels,targets,witness,['schemaVersion','language','model','owners'])||text(forms,field(starts,counts,labels,targets,witness,'schemaVersion'))!=='number'||text(texts,field(starts,counts,labels,targets,witness,'schemaVersion'))!=='1'||word(forms,texts,starts,counts,labels,targets,witness,'language')!=='JavaScript'||word(forms,texts,starts,counts,labels,targets,root,'sourceLanguage')!=='JavaScript'||word(forms,texts,starts,counts,labels,targets,witness,'model')!==readJavaScriptClassOwnerModel())return 'unrecognized owned class witness header';
  const owners=field(starts,counts,labels,targets,witness,'owners');if(owners<0||text(forms,owners)!=='array'||number(counts,owners)<1||number(counts,owners)>64||declarations<0)return 'complete actual class owners and declarations are required';
  for(let at=0;at<number(counts,owners);at+=1){
   const owner=element(starts,counts,targets,owners,at),constructor=word(forms,texts,starts,counts,labels,targets,owner,'constructor'),brand=word(forms,texts,starts,counts,labels,targets,owner,'brand'),representation=word(forms,texts,starts,counts,labels,targets,owner,'representation');
   if(!exact(starts,counts,labels,targets,owner,['constructor','brand','representation','fields','methods'])||constructor===''||brand===''||constructors.includes(constructor)||brands.includes(brand)||!['prototype helpers','closure slots'].includes(representation))return 'class owners need distinct actual constructor and brand identities';
   const fields=strings(forms,texts,starts,counts,labels,targets,field(starts,counts,labels,targets,owner,'fields'));if(fields.length>8192||fields.includes('')||new Set(fields).size!==fields.length||fields.includes(brand))return 'actual class data fields must be complete and distinct';
   const methods=field(starts,counts,labels,targets,owner,'methods');if(methods<0||text(forms,methods)!=='array'||number(counts,methods)>8192)return 'actual class methods must retain their declared helpers';let methodNames=[],helpers=[];
   for(let method=0;method<number(counts,methods);method+=1){const row=element(starts,counts,targets,methods,method),name=word(forms,texts,starts,counts,labels,targets,row,'name'),helper=word(forms,texts,starts,counts,labels,targets,row,'helper');if(!exact(starts,counts,labels,targets,row,['name','helper'])||name===''||helper===''||fields.includes(name)||methodNames.includes(name)||helpers.includes(helper))return 'actual class method owners are contradictory';methodNames=methodNames.concat([name]);helpers=helpers.concat([helper]);}
   let actual=-1;for(let declaration=0;declaration<number(counts,declarations);declaration+=1){const row=element(starts,counts,targets,declarations,declaration);if(word(forms,texts,starts,counts,labels,targets,row,'fullName')===constructor&&word(forms,texts,starts,counts,labels,targets,row,'k')==='fn')actual=row;}
   if(actual<0)return 'a class owner must reference its actual constructor declaration';
   const ret=field(starts,counts,labels,targets,actual,'ret'),body=field(starts,counts,labels,targets,actual,'body');
   const reference=field(starts,counts,labels,targets,ret,'reference');if(word(forms,texts,starts,counts,labels,targets,ret,'kind')==='record'&&(reference<0||text(forms,reference)!=='boolean'||text(texts,reference)!=='true'))return 'a class constructor result must retain its shared instance reference';
   if(!carries(forms,texts,starts,counts,labels,targets,ret,constructor,0)||fresh(forms,texts,starts,counts,labels,targets,body,constructor,[],[],0)<0)return 'an actual class constructor must return its own fresh reference allocation';
   for(const helper of helpers){let found=false;for(let declaration=0;declaration<number(counts,declarations);declaration+=1){const row=element(starts,counts,targets,declarations,declaration);if(word(forms,texts,starts,counts,labels,targets,row,'fullName')!==helper||word(forms,texts,starts,counts,labels,targets,row,'k')!=='fn')continue;const params=field(starts,counts,labels,targets,row,'params'),parameter=params<0?-1:element(starts,counts,targets,params,0),type=field(starts,counts,labels,targets,parameter,'type');if(word(forms,texts,starts,counts,labels,targets,type,'sourceClassOwner')===constructor)found=true;}if(!found)return 'a class method helper must retain its actual owned receiver signature';}
   constructors=constructors.concat([constructor]);brands=brands.concat([brand]);representations=representations.concat([representation]);ownerNodes=ownerNodes.concat([owner]);constructorNodes=constructorNodes.concat([actual]);
  }
 }
 for(let node=0;node<starts.length;node+=1)if(word(forms,texts,starts,counts,labels,targets,node,'kind')==='record'){
  const owner=word(forms,texts,starts,counts,labels,targets,node,'sourceClassOwner'),fields=names(forms,texts,starts,counts,labels,targets,node),position=constructors.indexOf(owner),present=brands.some(brand=>fields.includes(brand));
  if(field(starts,counts,labels,targets,node,'sourceClassOwner')>=0&&owner==='')return 'an actual class type owner must be a nonempty constructor reference';
  if(owner===''&&!present)continue;if(position<0)return 'an owned class type cannot drop its constructor witness';
  const definition=number(ownerNodes,position),expected=strings(forms,texts,starts,counts,labels,targets,field(starts,counts,labels,targets,definition,'fields')).concat([text(brands,position)]),methods=field(starts,counts,labels,targets,definition,'methods');let complete=expected;
  if(text(representations,position)==='closure slots')for(let at=0;at<number(counts,methods);at+=1)complete=complete.concat([word(forms,texts,starts,counts,labels,targets,element(starts,counts,targets,methods,at),'name')]);
  if(fields.length!==complete.length||complete.some(name=>!fields.includes(name)))return 'an owned class record contradicts its complete actual data fields';
  const brandedFields=field(starts,counts,labels,targets,node,'fields');for(let at=0;at<number(counts,brandedFields);at+=1){const row=element(starts,counts,targets,brandedFields,at);if(word(forms,texts,starts,counts,labels,targets,row,'name')===text(brands,position)&&word(forms,texts,starts,counts,labels,targets,field(starts,counts,labels,targets,row,'type'),'kind')!=='unit')return 'an actual class brand must retain its Unit field';}
  const reference=field(starts,counts,labels,targets,number(constructorNodes,position),'ret');
  if(word(forms,texts,starts,counts,labels,targets,reference,'kind')==='record'){
   const actualFields=field(starts,counts,labels,targets,node,'fields'),expectedFields=field(starts,counts,labels,targets,reference,'fields');
   for(let at=0;at<number(counts,actualFields);at+=1){const actual=element(starts,counts,targets,actualFields,at),name=word(forms,texts,starts,counts,labels,targets,actual,'name');for(let other=0;other<number(counts,expectedFields);other+=1){const expected=element(starts,counts,targets,expectedFields,other);if(word(forms,texts,starts,counts,labels,targets,expected,'name')===name&&text(typeKeys,field(starts,counts,labels,targets,actual,'type'))!==text(typeKeys,field(starts,counts,labels,targets,expected,'type')))return 'an owned class field changed its actual constructor type';}}
  }
 }
 if(constructors.length===0)return '';
 // Traverse expression containers of each real declaration, retaining its
 // lexical owner. A branded record payload cannot be forged in another item.
 let roots=[field(starts,counts,labels,targets,root,'main')],scopes=[''];
 for(let declaration=0;declarations>=0&&declaration<number(counts,declarations);declaration+=1){const row=element(starts,counts,targets,declarations,declaration);roots=roots.concat([row]);scopes=scopes.concat([word(forms,texts,starts,counts,labels,targets,row,'fullName')]);}
 for(let scope=0;scope<roots.length;scope+=1){const owner=text(scopes,scope);let pending=[number(roots,scope),0],seen=[];
  while(pending.length){const direct=number(pending,pending.length-1),node=number(pending,pending.length-2);pending=pending.slice(0,-2);if(node<0||seen.includes(node*2+direct))continue;if(seen.length>100000)return 'owned class body traversal exceeds its budget';seen=seen.concat([node*2+direct]);const kind=word(forms,texts,starts,counts,labels,targets,node,'k'),type=field(starts,counts,labels,targets,node,'type'),classOwner=word(forms,texts,starts,counts,labels,targets,type,'sourceClassOwner');
   if(kind==='record'&&classOwner!==''&&classOwner!==owner)return 'class data allocations belong only to their actual constructor';
   const object=field(starts,counts,labels,targets,node,'object'),objectType=field(starts,counts,labels,targets,object,'type'),instance=constructors.indexOf(word(forms,texts,starts,counts,labels,targets,objectType,'sourceClassOwner'));
   if(instance>=0){const definition=number(ownerNodes,instance),brand=text(brands,instance),name=kind==='recordField'?word(forms,texts,starts,counts,labels,targets,node,'field'):kind==='method'?readRecordFieldWriteName(word(forms,texts,starts,counts,labels,targets,node,'op')):'';
    if(name===brand)return 'a synthetic class brand is not an observable public data field';
    const methods=field(starts,counts,labels,targets,definition,'methods');for(let at=0;at<number(counts,methods);at+=1){const method=element(starts,counts,targets,methods,at);if(word(forms,texts,starts,counts,labels,targets,method,'name')!==name)continue;
     if(kind==='recordField'&&(text(representations,instance)!=='closure slots'||direct!==1))return 'class methods require receiver-retaining calls rather than detached slot observations';
     if(kind==='method'){
      const args=field(starts,counts,labels,targets,node,'args'),value=args<0?-1:element(starts,counts,targets,args,0),helper=word(forms,texts,starts,counts,labels,targets,method,'helper'),callArgs=field(starts,counts,labels,targets,value,'args'),receiver=callArgs<0?-1:element(starts,counts,targets,callArgs,0);
      if(owner!==text(constructors,instance)||text(representations,instance)!=='closure slots'||word(forms,texts,starts,counts,labels,targets,value,'k')!=='call'||word(forms,texts,starts,counts,labels,targets,value,'fn')!==helper||word(forms,texts,starts,counts,labels,targets,object,'k')!=='var'||word(forms,texts,starts,counts,labels,targets,receiver,'k')!=='var'||word(forms,texts,starts,counts,labels,targets,object,'name')!==word(forms,texts,starts,counts,labels,targets,receiver,'name'))return 'class method slots are installed only by their actual receiver factory inside the constructor';
     }
    }
   }
   for(let at=number(starts,node);at<number(starts,node)+number(counts,node);at+=1)if(!['type','domain','span','modulePath'].includes(text(labels,at)))pending=pending.concat([number(targets,at),kind==='apply'&&text(labels,at)==='fn'?1:0]);
  }
 }
 for(let node=0;node<starts.length;node+=1){const kind=word(forms,texts,starts,counts,labels,targets,node,'k'),proof=word(forms,texts,starts,counts,labels,targets,node,'p'),operation=word(forms,texts,starts,counts,labels,targets,node,'op'),reference=field(starts,counts,labels,targets,node,'reference'),identity=kind==='strictEqual'||kind==='binary'&&['eq','ne'].includes(operation)||['eq','ne'].includes(proof)&&reference>=0&&text(texts,reference)==='true';
  for(const key of readJavaScriptClosedClassObservationOperationOperands(kind,proof,operation).split(' '))if(key){const operand=field(starts,counts,labels,targets,node,key);if(operand<0)continue;let values=text(forms,operand)==='array'?[]:[operand];if(text(forms,operand)==='array')for(let at=0;at<number(counts,operand);at+=1)values=values.concat([element(starts,counts,targets,operand,at)]);
   for(const value of values){const type=field(starts,counts,labels,targets,value,'type');if(type<0)continue;
    for(let owner=0;owner<constructors.length;owner+=1)if(!acceptJavaScriptClassOwnerObservation(text(representations,owner),identity?'identity':'structural')&&containsJavaScriptClosedClassBrand(texts,starts,counts,labels,targets,type,declarations,[text(brands,owner)]))return 'this class representation does not admit the requested identity, debug or proof observation';
    let pending=[type],seen=[];while(pending.length){const actual=number(pending,pending.length-1);pending=pending.slice(0,-1);if(actual<0||seen.includes(actual))continue;if(seen.length>100000)return 'owned class observation traversal exceeds its budget';seen=seen.concat([actual]);const owner=word(forms,texts,starts,counts,labels,targets,actual,'sourceClassOwner'),position=constructors.indexOf(owner);if(position>=0&&!acceptJavaScriptClassOwnerObservation(text(representations,position),identity?'identity':'structural'))return 'this class representation does not admit the requested identity, debug or proof observation';
    for(let at=number(starts,actual);at<number(starts,actual)+number(counts,actual);at+=1)if(!['span','sourceClassOwner'].includes(text(labels,at)))pending=pending.concat([number(targets,at)]);
   }}
  }
 }return '';
}
