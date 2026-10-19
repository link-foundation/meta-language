import test from 'node:test';
import assert from 'node:assert/strict';
import {encodeStructuredDataLinks,decodeStructuredDataLinks,structuredDataToLino,readStructuredDataLino} from '../../../../js/src/structured-data-links.js';
import {captureOrdinaryInversePlan,authenticateOrdinaryInversePlan,readOrdinaryInversePlan,extendRecursiveInversePlan} from './plan.mjs';
import {reconstructOrdinaryOutcome,projectOrdinaryOutcome,preflightOrdinaryOutcome,OrdinaryInverseRefusal} from './owned.mjs';
import {outcomeWireEvents} from '../wire/events.mjs';
import {LinkNetwork} from '../../../../js/src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../../../../js/src/link-network-binary.js';
import {readProjectData} from '../../../../js/scripts/project-data.mjs';
import * as decisions from './rules.mjs';
import {generateOwnedOutcomeInverseRules} from './generate-rules.mjs';
import {generateOwnedOutcomeInverseFixtures} from './generate-fixtures.mjs';
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';

// Finish the fresh oracle process before retaining the complete source graph.
const oracleFixture=generateOwnedOutcomeInverseFixtures({check:true});
const authority=captureOrdinaryInversePlan(),plan=authority.plan(),settings={usizeBits:64};
const recursiveAuthority=extendRecursiveInversePlan(authority);
const rootReference=name=>plan.graph.roots.find(id=>plan.graph.nodes.find(node=>node.identity===id).name===name);
const token=(extra={})=>{
  const values={type:'token',kind:'identifier',start:0,end:1,text:'a',...extra};
  const order=['type','kind','field','trivia','start','end','attributes','lexical','text','hex'];
  return Object.fromEntries([...order.filter(key=>Object.hasOwn(values,key)),
    ...Object.keys(values).filter(key=>!order.includes(key))].map(key=>[key,values[key]]));
};
const outcome=tree=>({ok:true,tree,ambiguities:[],rejection:null});
const invert=(value,options={})=>{const encoded=encodeStructuredDataLinks(value);return reconstructOrdinaryOutcome(authority,encoded.network,encoded.root,{...settings,...options});};
const project=(model,options={})=>projectOrdinaryOutcome(authority,model,{...settings,...options});
const rejected=(work,code)=>assert.throws(work,error=>error instanceof OrdinaryInverseRefusal&&(!code||error.code===code));

test('source-derived inverse plan retains every declaration field variant and original owner pin',()=>{
  assert.equal(plan.graph.roots.length,9);assert.equal(plan.graph.sourceInventory.pins.length,13);
  assert.equal(plan.admission.ordinaryVariants.length,4);assert.equal(plan.admission.refusedVariants.length,1);
  assert.deepEqual(authenticateOrdinaryInversePlan(authority,plan),plan);
  assert.deepEqual(authenticateOrdinaryInversePlan(authority,readProjectData(new URL('./ordinary-inverse-plan.lino',import.meta.url))),plan);
  assert.deepEqual(authenticateOrdinaryInversePlan(recursiveAuthority,readProjectData(new URL('./recursive-inverse-plan.lino',import.meta.url))),recursiveAuthority.plan());
  for(const change of [value=>value.graph.roots.pop(),value=>value.bindings[0].fields.pop(),value=>value.admission.refusedVariants=[]]){
    const altered=authority.plan();change(altered);assert.throws(()=>authenticateOrdinaryInversePlan(authority,altered));
  }
  let invoked=false;
  const forged={get plan(){invoked=true;throw new Error('must not invoke');}};
  assert.throws(()=>readOrdinaryInversePlan(forged));assert.equal(invoked,false);
});

test('all four ordinary CST variants preserve complete ordered wire events and source field order',()=>{
  for(const tree of [token(),token({field:'',attributes:{},lexical:null,text:null,hex:'not physical hex'}),
    {type:'error',start:0,end:0,text:'',reason:''},{type:'missing',kind:'identifier',start:0,end:0,literal:true},
    {type:'node',kind:'root',trivia:true,start:0,end:1,children:[token(),{type:'missing',kind:'end',start:1,end:1}],attributes:{}}]){
    const input=outcome(tree),model=invert(input),actual=project(model);
    assert.equal(model.nodes[0].typeReference,plan.rootReference);
    assert.deepEqual(actual.events,[...outcomeWireEvents(plan.wirePlan,actual.native)]);
    assert.equal(actual.native.tree.$variant,tree.type[0].toUpperCase()+tree.type.slice(1));
    const source=plan.bindings.find(binding=>binding.declarationReference===plan.rootReference);
    assert.deepEqual(model.nodes[0].children.map(id=>model.nodes[id-1].typeReference),source.fields.map(field=>field.typeReference));
    assert.deepEqual(readStructuredDataLino(structuredDataToLino(model)),model);
    assert.deepEqual(project(readStructuredDataLino(structuredDataToLino(model))).events,actual.events);
    const encoded=encodeStructuredDataLinks(input),canonical=LinkNetwork.fromLino(encoded.network.toLino());
    const restored=decodeLinkNetworkBinary(encodeLinkNetworkBinary(canonical));
    assert.equal(restored.toLino(),canonical.toLino());
    assert.deepEqual(reconstructOrdinaryOutcome(authority,restored,encoded.root,settings),model);
    const ownedGraph=encodeStructuredDataLinks(model),ownedCanonical=LinkNetwork.fromLino(ownedGraph.network.toLino());
    const ownedRestored=decodeStructuredDataLinks(decodeLinkNetworkBinary(encodeLinkNetworkBinary(ownedCanonical)),ownedGraph.root);
    assert.deepEqual(project(ownedRestored).events,actual.events);
  }
});

test('None Some null flags diagnostics and actual static-borrow obligations remain distinguishable',()=>{
  const none=invert(outcome(token())),some=invert(outcome(token({lexical:null})));
  assert.notDeepEqual(none,some);
  assert.equal(Object.hasOwn(project(none).native.tree,'lexical'),false);
  assert.equal(project(some).native.tree.lexical,null);
  assert.equal(project(none).native.tree.trivia,false);
  const input={ok:false,tree:null,ambiguities:[],rejection:{reason:'syntax',offset:0,line:1,column:1,expected:[],limit:0,diagnostics:[],offsetSemantics:''},sourcePlanes:[]};
  const model=invert(input),native=project(model).native;
  assert.equal(native.rejection.reason,'syntax');assert.equal(native.rejection.limit,0n);
  assert.equal(model.borrowRequirements.length,1);
  const reference=model.nodes[model.borrowRequirements[0].node-1];assert.equal(reference.form,'reference');
  assert.equal(plan.graph.nodes.find(node=>node.identity===reference.typeReference).lifetime,'static');
  const altered=structuredClone(model);altered.borrowRequirements=[];rejected(()=>project(altered),'borrow-manifest');
  assert.throws(()=>invert({...input,sourcePlanes:undefined}));
  const {sourcePlanes,...withoutPlanes}=input;
  assert.notDeepEqual(model,invert(withoutPlanes));
});

test('ordered attribute maps preserve numeric-looking and prototype-like keys without sorting',()=>{
  const encoded=encodeStructuredDataLinks(outcome(token({attributes:{'1':1,'2':2,__proto__:null,z:'last'}})));
  const attributes=encoded.network.links().find(link=>link.metadata().term==='object value'&&link.references().length===3&&
    link.references().map(id=>encoded.network.link(encoded.network.link(id).references()[0]).metadata().definition).includes('z'));
  assert.ok(attributes);
  const byKey=new Map(attributes.references().map(id=>[encoded.network.link(encoded.network.link(id).references()[0]).metadata().definition,id]));
  attributes.setReferences(['2','1','z'].map(key=>byKey.get(key)));
  const model=reconstructOrdinaryOutcome(authority,encoded.network,encoded.root,settings),native=project(model).native;
  assert.deepEqual([...native.tree.attributes.keys()],['2','1','z']);
  assert.equal(native.tree.attributes.get('2').payload,2n);
  const prototypes=Object.create(null);Object.defineProperty(prototypes,'__proto__',{value:'data',enumerable:true});
  const prototypeModel=invert(outcome(token({attributes:prototypes})));
  assert.equal(project(prototypeModel).native.tree.attributes.get('__proto__').payload,'data');
});

test('exact large integers retain bits and selected pointer widths while invalid ranges refuse',()=>{
  const root=rootReference('OperationValue');
  for(const number of [0,1,-1,2**60,-(2**63)]){
    const model=invert(number,{rootReference:root}),actual=project(model,{rootReference:root});
    assert.equal(actual.native.payload,BigInt(number));
    const integer=model.nodes.find(row=>row.form==='i64');assert.equal(integer.data.decimal,BigInt(number).toString());
    const altered=structuredClone(model);altered.nodes[integer.id-1].data.bits='0000000000000000';
    if(number!==0)rejected(()=>project(altered,{rootReference:root}),'integer');
  }
  for(const number of [-0,0.5,2**63])rejected(()=>invert(number,{rootReference:root}));
  const wide=outcome(token({start:2**32,end:2**32}));
  assert.doesNotThrow(()=>invert(wide));rejected(()=>invert(wide,{usizeBits:32}),'integer-range');
  const narrow=invert(outcome(token({start:2**32-1,end:2**32-1})),{usizeBits:32});
  assert.equal(project(narrow,{usizeBits:32}).native.tree.start,4294967295n);
  rejected(()=>invert(wide,{usizeBits:16}),'target-profile');
});

test('embeddings opaque JSON unknown members and reordered fields refuse with source occurrence routes',()=>{
  const selection={mode:'any-grammar',mapping:'field-ranges',start:0,end:1,candidates:[],provenance:null,alternatives:[],sourceMapping:null,ambiguous:false};
  const embed=outcome({type:'embed',language:'Json',start:0,end:1,root:token(),sourceChildren:[],selection});
  assert.throws(()=>invert(embed),error=>error.code==='unsupported-embedding'&&error.typeReference.endsWith('#SyntaxTree')&&error.route.length>0);
  for(const value of [false,0,-0,'',[],{}]){
    assert.throws(()=>invert(outcome(token({lexical:value}))),error=>error.code==='unsupported-json'&&error.typeReference.includes('type:')&&error.route.length>0);
  }
  for(const tree of [token({future:true}),token({trivia:false}),token({text:null}),token({text:'a',hex:'00'})])rejected(()=>invert(outcome(tree)));
  const valid=outcome(token());rejected(()=>invert({rejection:null,...valid}),'record-members');
  rejected(()=>invert(outcome({type:'token',kind:'identifier',start:0,end:1,text:'a',attributes:{}})),'record-members');
  for(const numberFacts of [null,false,[{}],{ }])rejected(()=>invert(valid,{numberFacts}),'number-facts');
  assert.doesNotThrow(()=>invert(valid,{numberFacts:[]}));
  for(const codec of [null,false,0,'',[],{future:1}])rejected(()=>invert(valid,{codec}),'options');
});

test('full preflight budgets admit exact bounds and refuse one below without an owned result',()=>{
  const input=outcome({type:'node',kind:'root',start:0,end:1,children:[token()],attributes:{}}),encoded=encodeStructuredDataLinks(input);
  const cost=preflightOrdinaryOutcome(authority,encoded.network,encoded.root,settings);
  for(const [option,value]of [['nodeLimit',cost.occurrences],['edgeLimit',cost.edges],['depthLimit',cost.maximumDepth],['byteLimit',cost.scalarBytes],['metadataByteLimit',cost.metadataBytes]]){
    assert.ok(value>1);assert.doesNotThrow(()=>reconstructOrdinaryOutcome(authority,encoded.network,encoded.root,{...settings,[option]:value}));
    rejected(()=>reconstructOrdinaryOutcome(authority,encoded.network,encoded.root,{...settings,[option]:value-1}));
  }
  rejected(()=>reconstructOrdinaryOutcome(authority,encoded.network,encoded.root,{...settings,depthLimit:2}),'depth-budget');
  let tree=token();for(let depth=0;depth<12;depth++)tree={type:'node',kind:'nested',start:0,end:1,children:[tree]};
  assert.doesNotThrow(()=>invert(outcome(tree)));
});

test('owned model tampering and active inputs cannot hide extra source state or invoke callbacks',()=>{
  const model=invert(outcome(token()));
  for(const mutate of [
    value=>{value.extra=true;},value=>{value.nodes.push({...value.nodes.at(-1),id:value.nodes.length+1});},
    value=>{value.nodes[0].children.reverse();},value=>{value.nodes[0].children[0]=1;},
    value=>{value.nodes[0].typeReference=value.nodes[1].typeReference;},value=>{value.nodes[1].form='future';},
    value=>{value.nodes[0].data='payload';},value=>{value.nodes[0].extra='hidden';},value=>{value.planSha256='0'.repeat(64);},
    value=>{value.targetProfile.usizeBits=32;},value=>{value.borrowRequirements.push({node:1,typeReference:'foreign'});}
  ]){const altered=structuredClone(model);mutate(altered);rejected(()=>project(altered));}
  let calls=0;
  const active=structuredClone(model);Object.defineProperty(active,'root',{get(){calls++;return 1;}});
  rejected(()=>project(active),'active-input');assert.equal(calls,0);
  rejected(()=>project(new Proxy(model,{get(){calls++;throw new Error('trap');}})),'active-input');assert.equal(calls,0);
  const encoded=encodeStructuredDataLinks(outcome(token()));Object.defineProperty(encoded.network,'links',{get(){calls++;throw new Error('trap');}});
  rejected(()=>reconstructOrdinaryOutcome(authority,encoded.network,encoded.root,settings),'active-input');assert.equal(calls,0);
});

test('closed graph validation and owned container aliasing preserve all original codec refusals',()=>{
  const shared=token(),input=outcome({type:'node',kind:'root',start:0,end:1,children:[shared,shared]});
  rejected(()=>invert(input),'container-alias');
  assert.doesNotThrow(()=>invert(outcome({type:'node',kind:'root',start:0,end:1,children:[token(),token()]})));
  for(const mutate of [
    encoded=>encoded.network.insertLink([],encoded.network.link(encoded.root).metadata().clone()),
    encoded=>encoded.network.link(encoded.root).setReferences([encoded.root]),
    encoded=>encoded.network.link(encoded.root).setReferences([999999]),
    encoded=>{encoded.network.link(encoded.root).metadata().term='unknown schema root';},
    encoded=>{
      const object=encoded.network.link(encoded.network.link(encoded.root).references()[0]);
      const first=encoded.network.link(object.references()[0]),second=encoded.network.link(object.references()[1]);
      second.setReferences([first.references()[0],second.references()[1]]);
    }
  ]){const encoded=encodeStructuredDataLinks(outcome(token()));mutate(encoded);assert.throws(()=>reconstructOrdinaryOutcome(authority,encoded.network,encoded.root,settings));}
  const encoded=encodeStructuredDataLinks(outcome(token()));assert.deepEqual(decodeStructuredDataLinks(encoded.network,encoded.root),outcome(token()));
});

test('the recursive profile owns nested embeddings and every JSON projection kind without guessing native number tags',()=>{
  const lexical={nothing:null,flag:false,zero:0,negativeZero:-0,fraction:0.25,text:'π',empty:[],nested:[{value:'x'}]};
  let tree=token({lexical});
  for(let depth=0;depth<8;depth++)tree={type:'embed',language:depth%2?'Markdown':'JavaScript',start:0,end:1,root:tree,sourceChildren:[]};
  const invalid=encodeStructuredDataLinks(outcome({...tree,selection:null}));
  rejected(()=>reconstructOrdinaryOutcome(recursiveAuthority,invalid.network,invalid.root,settings),'record');
  const input={...outcome(tree),sourcePlanes:[lexical]},encoded=encodeStructuredDataLinks(input);
  const aliased=reconstructOrdinaryOutcome(recursiveAuthority,encoded.network,encoded.root,settings);
  const cost=preflightOrdinaryOutcome(recursiveAuthority,encoded.network,encoded.root,settings);
  for(const [option,value] of [['nodeLimit',cost.occurrences],['edgeLimit',cost.edges],['depthLimit',cost.maximumDepth],['byteLimit',cost.scalarBytes],['metadataByteLimit',cost.metadataBytes]]){
    assert.doesNotThrow(()=>reconstructOrdinaryOutcome(recursiveAuthority,encoded.network,encoded.root,{...settings,[option]:value}));
    rejected(()=>reconstructOrdinaryOutcome(recursiveAuthority,encoded.network,encoded.root,{...settings,[option]:value-1}));
  }
  const original=projectOrdinaryOutcome(recursiveAuthority,aliased,settings).native;
  let leaf=original.tree;while(leaf.$variant==='Embed')leaf=leaf.root;
  assert.equal(leaf.lexical,original.source_planes[0]);assert.ok(aliased.jsonAliases.length>0);
  const lost=structuredClone(aliased);lost.jsonAliases=[];
  rejected(()=>projectOrdinaryOutcome(recursiveAuthority,lost,settings),'json-alias');
  const inconsistent=structuredClone(aliased),nested=inconsistent.jsonAliases.find(row=>inconsistent.nodes[row.node-1].data.kind==='array');
  inconsistent.nodes[nested.node-1].data.aliasOf=null;
  inconsistent.jsonAliases=inconsistent.jsonAliases.filter(row=>row.node!==nested.node);
  rejected(()=>projectOrdinaryOutcome(recursiveAuthority,inconsistent,settings),'json-alias');
  assert.deepEqual(projectOrdinaryOutcome(recursiveAuthority,readStructuredDataLino(structuredDataToLino(aliased)),settings).events,
    projectOrdinaryOutcome(recursiveAuthority,aliased,settings).events);
  input.sourcePlanes=[structuredClone(lexical)];
  const packet=encodeStructuredDataLinks(input),model=reconstructOrdinaryOutcome(recursiveAuthority,packet.network,packet.root,settings);
  const result=projectOrdinaryOutcome(recursiveAuthority,model,settings);
  assert.equal(result.native.tree.$variant,'Embed');
  assert.equal(model.nodes.filter(row=>row.form==='enum_item'&&row.data.includes('#SyntaxTree@')).length,9);
  assert.equal(model.jsonRequirements.length,6);
  assert.equal(model.jsonAliases.length,0);
  let independent=result.native.tree;while(independent.$variant==='Embed')independent=independent.root;
  assert.notEqual(independent.lexical,result.native.source_planes[0]);
  assert.ok(model.jsonRequirements.every(row=>row.representation==='unresolved native number variant'));
  assert.ok(model.jsonRequirements.some(row=>row.bits==='8000000000000000'));
  assert.deepEqual(projectOrdinaryOutcome(recursiveAuthority,readStructuredDataLino(structuredDataToLino(model)),settings).events,result.events);
  const altered=structuredClone(model);altered.jsonRequirements=[];
  rejected(()=>projectOrdinaryOutcome(recursiveAuthority,altered,settings),'json-manifest');
  rejected(()=>project(model),'model');
});

test('recursive JSON descriptors refuse missing state duplicate keys nonfinite numbers and source type substitution',()=>{
  const encoded=encodeStructuredDataLinks(outcome(token({lexical:{value:[true,-0,'π',null]}})));
  const model=reconstructOrdinaryOutcome(recursiveAuthority,encoded.network,encoded.root,settings);
  const object=model.nodes.find(row=>row.form==='json'&&row.data.kind==='object');
  const number=model.nodes.find(row=>row.form==='json'&&row.data.kind==='number');
  const array=model.nodes.find(row=>row.form==='json'&&row.data.kind==='array');
  for(const mutate of [
    value=>{value.nodes[object.id-1].data.keys.push('value');},
    value=>{value.nodes[object.id-1].data.future=true;},
    value=>{value.nodes[number.id-1].data.bits='7ff0000000000000';},
    value=>{value.nodes[number.id-1].data.bits='7ff8000000000000';},
    value=>{value.nodes[array.id-1].data.kind='future';},
    value=>{value.nodes[array.id-1].data.aliasOf=object.id;},
    value=>{value.nodes[array.children[0]-1].typeReference=plan.rootReference;},
    value=>{value.jsonRequirements[0].representation='guessed unsigned integer';}
  ]){const altered=structuredClone(model);mutate(altered);rejected(()=>projectOrdinaryOutcome(recursiveAuthority,altered,settings));}
  assert.equal(recursiveAuthority.plan().admission.refusedVariants.length,0);
  assert.equal(authority.plan().admission.refusedVariants.length,1);
  assert.throws(()=>extendRecursiveInversePlan(recursiveAuthority));
});

test('the complete portable inverse decision module translates automatically and preserves the independent shared cases',()=>{
  const {fixture}=generateOwnedOutcomeInverseRules({check:true});
  for(const [width,expected] of fixture.widths)assert.equal(decisions.acceptOwnedOutcomeTargetWidth(width),expected);
  for(const [nullValue,arrayValue,valueType,expected] of fixture.kinds)assert.equal(decisions.readOwnedOutcomeJsonKind(nullValue,arrayValue,valueType),expected);
  for(const [kind,expected] of fixture.fields)assert.deepEqual(decisions.readOwnedOutcomeJsonFields(kind),expected);
  for(const [sourceKind,jsonKind,record,expected] of fixture.containers)assert.equal(decisions.acceptOwnedOutcomeContainer(sourceKind,jsonKind,record),expected);
});

test('every retained fourteen-case parser projection is either reconstructed completely or explicitly refused',()=>{
  const fixture=oracleFixture;
  assert.equal(fixture.cases.length,14);
  for(const input of fixture.inputs){
    const source=readFileSync(new URL('../../../../'+input.path,import.meta.url));
    assert.equal(source.length,input.bytes,input.path);
    assert.equal(createHash('sha256').update(source).digest('hex'),input.sha256,input.path);
  }
  const actual=[];
  for(const row of fixture.cases){
    const encoded=encodeStructuredDataLinks(row.expected);
    try{
      const model=reconstructOrdinaryOutcome(recursiveAuthority,encoded.network,encoded.root,settings);
      assert.deepEqual(projectOrdinaryOutcome(recursiveAuthority,model,settings).events,
        projectOrdinaryOutcome(recursiveAuthority,readStructuredDataLino(structuredDataToLino(model)),settings).events,row.id);
      actual.push([row.id,'reconstructed']);
    }catch(error){
      assert.ok(error instanceof OrdinaryInverseRefusal,row.id+': '+error.stack);
      actual.push([row.id,error.code]);
    }
  }
  assert.deepEqual(actual,[
    ['accepted-utf8','reconstructed'],['rejected-literal','reconstructed'],['recovered-refused','reconstructed'],
    ['recovered-accepted','reconstructed'],['attribute-order','reconstructed'],['any-alternatives','reconstructed'],
    ['ambiguity-refused','reconstructed'],['nested-selection','reconstructed'],['included-source-planes','reconstructed'],
    ['unknown-selection','reconstructed'],['invalid-utf8-byte','reconstructed'],['minimal','reconstructed'],
    ['all-native-variants','reconstructed'],['repeated-scalars','reconstructed']
  ]);
});
