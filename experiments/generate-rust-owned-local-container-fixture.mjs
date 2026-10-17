import {fileURLToPath} from 'node:url';
import {checkProjectData,writeProjectData} from '../js/scripts/project-data.mjs';

const roles=[
 ['program','record'],['items','array'],['declarations','map'],['main','record'],['effects','array'],
 ['function','record'],['parameters','array'],['module path','array'],['parameter','record'],
 ['proof','record'],['bindings','array'],['reads','array'],['initializers','array'],['constructors','array'],
 ['binding','record'],['read','record'],['initializer','record','moves'],['constructor','record'],
 ['binding path','array'],['read path','array'],['initializer fields','array','moves'],['constructor path','array'],
 ['signature','record'],['signature parameters','array'],['lifetime bindings','array'],['signature position','record'],
 ['owner path','array'],['body','record'],['expression items','array'],
];

export function rustOwnedLocalContainerFixture(){
 const containerMutations=roles.flatMap(([owner,kind,selection])=>[
  ...['borrowEscape','opaqueBinding'].flatMap(field=>['enumerable data','hidden data','enumerable getter','hidden getter'].map(descriptor=>({owner,field,descriptor}))),
  {owner,field:'borrowEscape',descriptor:'symbol data'},
  ...(kind==='array'?[{owner,field:'borrowEscape',descriptor:'sparse position'}]:[]),
 ].map((mutation,index)=>({id:owner.replaceAll(' ','-')+'-'+index,...mutation,selections:selection?[selection]:['moves','copies']})));
 const fieldGetters=[['program','items'],['program','declarations'],['program','main'],['main','effects'],['function','params'],['function','sourceLocalState'],['proof','bindings'],['parameters','0']].map(([owner,field])=>({owner,field}));
 const recordMutations=[
  ['main span false','main','span',false],['main span true','main','span',true],['main span text','main','span','span'],
  ['main span array','main','span',[]],['main span empty','main','span',{}],
  ['main span missing end','main','span',{start:0}],['main span extra field','main','span',{start:0,end:1,opaqueBinding:false}],
  ['main span reversed','main','span',{start:2,end:1}],['main span fractional','main','span',{start:0.5,end:1}],
  ['main span negative','main','span',{start:-1,end:1}],['main span negative zero','main','span',{start:-0,end:1}],
  ['main span overflow','main','span',{start:0,end:9007199254740992}],
  ['body span false','body','span',false],['main Boolean null','main','sequentialAsync',null],
  ['main Boolean text','main','sequentialAsync','false'],['function Boolean null','function','generated',null],
  ['function recursive null','function','recursive',null],['function mutual null','function','mutual',null],
  ['program output flag null','program','outputThreaded',null],['program reference flag null','program','referenceSemantics',null],
  ['imports null','program','imports',null],['externals null','program','externals',null],
  ['unknown program field','program','opaqueBinding',false],['unknown main field','main','opaqueBinding',false],
  ['unknown function field','function','opaqueBinding',false],['unknown parameter field','parameter','borrowEscape',false],
  ['unknown expression field','body','borrowEscape',false],
 ].map(([id,owner,field,value])=>({id:id.replaceAll(' ','-'),owner,field,value}));
 const containerDecisions=[];
 for(const kind of['array','map','record','unknown'])for(const dense of[false,true])for(const unrepresented of[false,true])containerDecisions.push({kind,dense,unrepresented,expected:!unrepresented&&['array','map','record'].includes(kind)&&(kind!=='array'||dense)});
 const recordDecisions=[];
 for(const booleanRequired of[false,true])for(const booleanValue of[false,true])recordDecisions.push({booleanRequired,booleanValue,expected:!booleanRequired||booleanValue});
 const spanDecisions=[];
 for(const present of[false,true])for(const recordShape of[false,true])for(const offsetShape of[false,true])spanDecisions.push({present,recordShape,offsetShape,expected:!present||recordShape&&offsetShape});
 return{schemaVersion:1,scope:'Selected owned local text profiles refuse unrepresented host descriptors and malformed contextual flags or spans before reading values; prior source and runtime fixtures remain unchanged.',containerMutations,fieldGetters,recordMutations,containerDecisions,recordDecisions,spanDecisions};
}

export function readRustOwnedLocalContainerOwner(program,owner){
 const fn=program.declarations.values().next().value,proof=fn.sourceLocalState,signature=fn.sourceSignature;
 const expressionItems=()=>{const pending=[fn.body];while(pending.length){const node=pending.shift();if(node.k==='tuple')return node.items;for(const field of['value','body','left','right','cond','then','else','arg','object'])if(node[field])pending.push(node[field]);}throw Error('the source has no owned tuple');};
 const values={program,items:program.items,declarations:program.declarations,main:program.main,effects:program.main.effects,function:fn,parameters:fn.params,'module path':fn.modulePath,parameter:fn.params[0],proof,bindings:proof.bindings,reads:proof.reads,initializers:proof.initializers,constructors:proof.literalConstructors,binding:proof.bindings[0],read:proof.reads[0],initializer:proof.initializers[0],constructor:proof.literalConstructors[0],'binding path':proof.bindings[0].path,'read path':proof.reads[0].path,'initializer fields':proof.initializers[0]?.fields,'constructor path':proof.literalConstructors[0].path,signature,'signature parameters':signature.parameters,'lifetime bindings':signature.lifetimeBindings,'signature position':signature.parameters[0],'owner path':signature.parameters[0].owner.path,body:fn.body};
 return owner==='expression items'?expressionItems():values[owner];
}

export function applyRustOwnedLocalContainerMutation(program,mutation,getter=()=>false){
 const owner=readRustOwnedLocalContainerOwner(program,mutation.owner);
 if(!owner)throw Error('the requested actual container is absent: '+mutation.owner);
 if(mutation.descriptor==='sparse position'){owner.length++;return;}
 const field=mutation.descriptor==='symbol data'?Symbol(mutation.field):mutation.field;
 const descriptor=mutation.descriptor.includes('getter')?{get:getter}:{value:false};
 Object.defineProperty(owner,field,{...descriptor,enumerable:mutation.descriptor.startsWith('enumerable')||mutation.descriptor==='symbol data',configurable:true});
}

if(process.argv[1]===fileURLToPath(import.meta.url)){
 const file=new URL('./rust-owned-local-container-fixture.lino',import.meta.url),fixture=rustOwnedLocalContainerFixture();
 if(process.argv.includes('--check'))checkProjectData(file,fixture);else writeProjectData(file,fixture);
 console.log('Owned text containers: '+fixture.containerMutations.length+' descriptor mutations and '+fixture.recordMutations.length+' contextual record edits');
}
