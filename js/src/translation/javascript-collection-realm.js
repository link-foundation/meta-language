import {readJavaScriptArraySpeciesDirectoryError} from './javascript-array-species.js';
// Node authority supplies only actual standard identity/prototype witnesses.
// Every REPRESENTED collection allocation, mutation and copy uses the shared
// inspectable heap kernel; no host Map/Set stores represented collection data.
import {compileJavaScriptCollectionCallDirectory} from './javascript-collection-call-source.js';
const actualMap=Map,actualSet=Set,actualSymbol=Symbol;
const actualString=String,startsWith=Reflect.getOwnPropertyDescriptor(String.prototype,'startsWith')?.value;
const actualObject=Object,objectPrototype=Object.prototype;
const actualArray=Array,arrayPrototype=Array.prototype,isArray=Reflect.getOwnPropertyDescriptor(Array,'isArray')?.value;
const arraySpeciesGetter=Reflect.getOwnPropertyDescriptor(actualArray,actualSymbol.species)?.get;
const arrayCallbackOwners=['forEach','some','every','find','findIndex','findLast','findLastIndex','reduce','reduceRight'].map(name=>({name,value:Reflect.getOwnPropertyDescriptor(arrayPrototype,name)?.value}));
const arraySearchOwners=['includes','indexOf','lastIndexOf','at'].map(name=>({name,value:Reflect.getOwnPropertyDescriptor(arrayPrototype,name)?.value}));
const kinds=[{kind:'Map',constructor:actualMap,operations:['get','set','has','delete','clear']},{kind:'Set',constructor:actualSet,operations:['add','has','delete','clear']}];
const methodOwners=kinds.map(row=>({...row,prototype:row.constructor.prototype,methods:row.operations.map(name=>({name,value:Reflect.getOwnPropertyDescriptor(row.constructor.prototype,name)?.value})),iterationMethods:['keys','values','entries'].map(name=>({name,value:Reflect.getOwnPropertyDescriptor(row.constructor.prototype,name)?.value})),size:Reflect.getOwnPropertyDescriptor(row.constructor.prototype,'size')?.get,iterator:Reflect.getOwnPropertyDescriptor(row.constructor.prototype,actualSymbol.iterator)?.value}));

export function linkJavaScriptCollectionRealm(embedding,requirements,{realmIdentity}={}){
 if(globalThis.Map!==actualMap||globalThis.Set!==actualSet||globalThis.Symbol!==actualSymbol||typeof realmIdentity!=='bigint'||realmIdentity<=0n)throw new TypeError('standard collections require their actual protected Node realm and explicit identity');
 const requested=new Set(requirements.filter(row=>(row.kind==='construct'||row.kind==='global')&&['Map','Set'].includes(row.name)).map(row=>row.name)),constructors=[],references=[],iteratorOwners=[],iteratorMethods=[];
 for(const owner of methodOwners.filter(row=>requested.has(row.kind))){
  if(owner.constructor.prototype!==owner.prototype||typeof owner.size!=='function'||typeof owner.iterator!=='function'||Reflect.getOwnPropertyDescriptor(owner.prototype,'size')?.get!==owner.size||Reflect.getOwnPropertyDescriptor(owner.prototype,actualSymbol.iterator)?.value!==owner.iterator)throw new TypeError('collection intrinsics retain their actual standard prototype, size getter and iterator method');
  for(const method of owner.methods)if(typeof method.value!=='function'||Reflect.getOwnPropertyDescriptor(owner.prototype,method.name)?.value!==method.value)throw new TypeError('a collection operation cannot be certified by a method spelling or modified prototype');
  // This empty native probe establishes engine-owned iterator PROTOTYPE
  // identity only. It never carries represented values or implements copying.
  const probe=Reflect.apply(owner.iterator,Reflect.construct(owner.constructor,[]),[]),iteratorPrototype=Reflect.getPrototypeOf(probe),next=Reflect.getOwnPropertyDescriptor(iteratorPrototype,'next')?.value;
  if(typeof next!=='function')throw new TypeError('a collection copy needs its actual native iterator prototype and next owner');
  const sharedIteratorPrototype=Reflect.getPrototypeOf(iteratorPrototype),self=Reflect.getOwnPropertyDescriptor(sharedIteratorPrototype,actualSymbol.iterator)?.value;
  if(typeof self!=='function')throw new TypeError('standard collection iterators require their actual shared iterator identity method');
  const encodedIteratorPrototype=embedding.encode(iteratorPrototype),encodedArrayPrototype=embedding.encode(arrayPrototype),encodedObjectPrototype=embedding.encode(objectPrototype);
  for(const method of owner.iterationMethods){
   if(typeof method.value!=='function'||Reflect.getOwnPropertyDescriptor(owner.prototype,method.name)?.value!==method.value)throw new TypeError('a collection iterator requires its actual standard method owner');
   iteratorMethods.push({$:'collectionIteratorMethodOwner',identity:embedding.encode(method.value).identity,realm:realmIdentity,operation:owner.kind+'.'+method.name,iteratorPrototype:encodedIteratorPrototype,arrayPrototype:encodedArrayPrototype,objectPrototype:encodedObjectPrototype});
  }
  iteratorMethods.push({$:'collectionIteratorMethodOwner',identity:embedding.encode(next).identity,realm:realmIdentity,operation:owner.kind+'Iterator.next',iteratorPrototype:encodedIteratorPrototype,arrayPrototype:encodedArrayPrototype,objectPrototype:encodedObjectPrototype},{$:'collectionIteratorMethodOwner',identity:embedding.encode(self).identity,realm:realmIdentity,operation:'Iterator.self',iteratorPrototype:encodedIteratorPrototype,arrayPrototype:encodedArrayPrototype,objectPrototype:encodedObjectPrototype});
  iteratorOwners.push({prototype:iteratorPrototype,next,sharedIteratorPrototype,self});
  const callable=embedding.encode(owner.constructor),prototype=embedding.encode(owner.prototype),adder=embedding.encode(owner.methods.find(row=>row.name===(owner.kind==='Map'?'set':'add')).value);
  constructors.push({$:'collectionConstructorOwner',kind:owner.kind,identity:callable.identity,realm:realmIdentity,prototype,adder,iteratorKey:embedding.encode(actualSymbol.iterator),iteratorMethod:embedding.encode(owner.iterator),iteratorPrototype:embedding.encode(iteratorPrototype),nextMethod:embedding.encode(next),methods:[...owner.methods.map(row=>({$:'collectionMethodOwner',identity:embedding.encode(row.value).identity,operation:owner.kind+'.'+row.name})),{$:'collectionMethodOwner',identity:embedding.encode(owner.size).identity,operation:owner.kind+'.size'}]});
  for(const requirement of requirements.filter(row=>(row.kind==='construct'||row.kind==='global')&&row.name===owner.kind))references.push(embedding.capabilityReference(requirement.source,owner.constructor,{operation:'actual standard '+owner.kind+' constructor',owner:'actual Node collection intrinsics 1'}));
 }
 const arrayRequirements=requirements.filter(row=>row.kind==='global'&&row.name==='Array');
 if(globalThis.Object!==actualObject||actualObject.prototype!==objectPrototype)throw new TypeError('iterator results require their actual protected Object prototype');
 if(globalThis.Array!==actualArray||actualArray.prototype!==arrayPrototype||typeof isArray!=='function'||Reflect.getOwnPropertyDescriptor(actualArray,'isArray')?.value!==isArray)throw new TypeError('Array operations require their actual protected realm constructor, prototype and isArray owner');
 for(const requirement of arrayRequirements)references.push(embedding.capabilityReference(requirement.source,actualArray,{operation:'actual standard Array namespace',owner:'actual Node Array intrinsics 1'}));
 if(references.length!==requirements.length)throw new TypeError('every class source capability needs its actual supplied source or standard collection implementation');
 if(globalThis.String!==actualString||typeof startsWith!=='function'||Reflect.getOwnPropertyDescriptor(actualString.prototype,'startsWith')?.value!==startsWith)throw new TypeError('primitive methods require their actual protected standard function owner');
 for(const owner of [...arrayCallbackOwners,...arraySearchOwners])if(typeof owner.value!=='function'||Reflect.getOwnPropertyDescriptor(arrayPrototype,owner.name)?.value!==owner.value)throw new TypeError('Array callbacks require their actual protected standard method owners');
 if(typeof arraySpeciesGetter!=='function'||Reflect.getOwnPropertyDescriptor(actualArray,actualSymbol.species)?.get!==arraySpeciesGetter)throw new TypeError('Array species requires its actual protected standard getter');
 const directory={$:'collectionCallableDirectory',realms:[realmIdentity],exoticKey:embedding.encode(actualSymbol.toPrimitive),constructors,iteratorMethods,primitiveMethods:[{$:'primitiveMethodOwner',identity:embedding.encode(arraySpeciesGetter).identity,realm:realmIdentity,operation:'Array.species getter'},...[...arrayCallbackOwners,...arraySearchOwners].map(owner=>({$:'primitiveMethodOwner',identity:embedding.encode(owner.value).identity,realm:realmIdentity,operation:'Array.'+owner.name})),{$:'primitiveMethodOwner',identity:embedding.encode(startsWith).identity,realm:realmIdentity,operation:'String.startsWith'},...(arrayRequirements.length?[{$:'primitiveMethodOwner',identity:embedding.encode(isArray).identity,realm:realmIdentity,operation:'Array.isArray'}]:[])]},compiled=compileJavaScriptCollectionCallDirectory(directory);if(compiled.error)throw new TypeError(compiled.error);embedding.registerPortableProgram(compiled.program);
 const standardConstructorFacts=compiled.program.functions.map(fn=>({$:'arrayConstructorFact',identity:fn.identity,realm:fn.realm,constructable:constructors.some(owner=>owner.identity===fn.identity),intrinsicArray:false}));
 return {directory,program:compiled.program,references,speciesDirectory(){
  if(globalThis.Array!==actualArray||actualArray.prototype!==arrayPrototype||globalThis.Symbol!==actualSymbol||Reflect.getOwnPropertyDescriptor(actualArray,actualSymbol.species)?.get!==arraySpeciesGetter)throw new TypeError('species roots retain their actual protected builtin owners');
  const facts=new Map(),add=fact=>{const prior=facts.get(fact.identity);if(prior&&(prior.constructable!==fact.constructable||prior.realm!==fact.realm||prior.intrinsicArray!==fact.intrinsicArray))throw new TypeError('constructor ownership cannot have conflicting classifications');facts.set(fact.identity,fact);};
  const currentArray=embedding.encode(actualArray);add({$:'arrayConstructorFact',identity:currentArray.identity,realm:realmIdentity,constructable:true,intrinsicArray:true});
  for(const fact of standardConstructorFacts)add({...fact});
  for(const fact of embedding.sourceConstructorFacts())add(fact);
  const species={$:'arraySpeciesDirectory',realm:realmIdentity,arrayConstructor:currentArray,arrayPrototype:embedding.encode(arrayPrototype),speciesKey:embedding.encode(actualSymbol.species),constructors:[...facts.values()].toSorted((a,b)=>a.identity<b.identity?-1:a.identity>b.identity?1:0)},error=readJavaScriptArraySpeciesDirectoryError(embedding.heap(),species);if(error)throw new TypeError(error);return species;
 },verify(){
  if(globalThis.Map!==actualMap||globalThis.Set!==actualSet||globalThis.Symbol!==actualSymbol)throw new TypeError('the actual collection realm cannot change its registered standard owners');
  if(globalThis.String!==actualString||Reflect.getOwnPropertyDescriptor(actualString.prototype,'startsWith')?.value!==startsWith)throw new TypeError('the actual primitive method owner has changed');
  if(globalThis.Object!==actualObject||actualObject.prototype!==objectPrototype)throw new TypeError('the actual iterator result Object prototype has changed');
  if(globalThis.Array!==actualArray||actualArray.prototype!==arrayPrototype||Reflect.getOwnPropertyDescriptor(actualArray,'isArray')?.value!==isArray)throw new TypeError('the actual Array namespace or isArray owner has changed');
  if(Reflect.getOwnPropertyDescriptor(actualArray,actualSymbol.species)?.get!==arraySpeciesGetter)throw new TypeError('the actual Array species getter owner has changed');
  for(const owner of [...arrayCallbackOwners,...arraySearchOwners])if(Reflect.getOwnPropertyDescriptor(arrayPrototype,owner.name)?.value!==owner.value)throw new TypeError('an actual Array callback owner has changed');
  for(const owner of methodOwners.filter(row=>requested.has(row.kind))){if(owner.constructor.prototype!==owner.prototype||Reflect.getOwnPropertyDescriptor(owner.prototype,'size')?.get!==owner.size||Reflect.getOwnPropertyDescriptor(owner.prototype,actualSymbol.iterator)?.value!==owner.iterator)throw new TypeError('the actual collection prototype or iterator owner has changed');for(const method of [...owner.methods,...owner.iterationMethods])if(Reflect.getOwnPropertyDescriptor(owner.prototype,method.name)?.value!==method.value)throw new TypeError('the actual standard collection method owner has changed');}
  for(const owner of iteratorOwners)if(Reflect.getOwnPropertyDescriptor(owner.prototype,'next')?.value!==owner.next||Reflect.getPrototypeOf(owner.prototype)!==owner.sharedIteratorPrototype||Reflect.getOwnPropertyDescriptor(owner.sharedIteratorPrototype,actualSymbol.iterator)?.value!==owner.self)throw new TypeError('the actual standard iterator next owner has changed');
 }};
}
