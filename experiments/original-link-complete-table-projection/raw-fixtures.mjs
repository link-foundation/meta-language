// Patch target: experiments/original-link-complete-table-projection/raw-fixtures.mjs.
// Expectations are contracts, never observations of implemented support. New
// source-only controls must be executed independently before claiming support.
import assert from 'node:assert/strict';
import {readProjectData} from '../../js/scripts/project-data.mjs';
import {TYPED_PROGRAM_LINKS_SCHEMA as schema} from '../../js/src/typed-program-links.js';
import {ByteRange,LinkFlags,LinkMetadata,LinkType,Point,SourceSpan} from '../../js/src/primitives.js';
import {createPortableRustTextCertificate,encodePortableRustTextCertificate} from '../rust-text-certificate/bridge.mjs';

const facet='sourceTextValueProjection';
const label=name=>schema.fields[name]??name.replace(/[A-Z]/gu,letter=>' '+letter.toLowerCase());
const key=id=>id.asU64().toString();

function field(packet,parent,name){
 const rows=packet.network.fieldRelations().filter(row=>row.parent.equals(parent)&&row.label===label(name));
 assert.equal(rows.length,1,'one original '+name+' relation exists');
 return rows[0];
}
function body(packet){return packet.network.link(packet.root).references()[0];}
function marker(packet){return field(packet,body(packet),facet);}
function proof(packet){return marker(packet).child;}
function replace(row,child){row.link.setReferences([row.parent,row.link.references()[1],child]);}
function scalar(packet,kind,definition){
 let metadata=LinkMetadata.new().withLinkType(LinkType.Semantic).withLanguage(schema.language).withNamed(true).withTerm(schema.scalars[kind]);
 if(definition!==undefined)metadata=metadata.withDefinition(definition);
 return packet.network.insertLink([],metadata);
}
function functionFacts(packet){
 return packet.network.link(field(packet,proof(packet),'functions').child).references();
}
function declaration(packet,name){
 const candidates=functionFacts(packet).map(id=>field(packet,id,'declaration').child);
 const found=candidates.find(id=>packet.network.link(field(packet,id,'name').child).metadata().definition===name);
 assert.ok(found,'the actual '+name+' declaration exists');
 return found;
}
function parameter(packet,owner,index=0){
 const parameters=packet.network.link(field(packet,owner,'params').child).references();
 assert.ok(index<parameters.length,'the actual parameter exists');
 return parameters[index];
}
function readEndOfFileRawString(packet){
 const tokens=packet.network.link(field(packet,proof(packet),'tokens').child).references();
 const eof=tokens.find(id=>packet.network.link(field(packet,id,'kind').child).metadata().definition==='eof');
 assert.ok(eof,'the complete actual source retains its EOF token');
 const raw=field(packet,eof,'raw').child;
 assert.equal(packet.network.link(raw).metadata().definition,'');
 return raw;
}
function replaceStringDefinition(packet,id,value){
 const node=packet.network.link(id);
 assert.equal(node.metadata().term,schema.scalars.string);
 node.setMetadata(node.metadata().withDefinition(value));
}

export function createRawTextCertificateFixture(id='owned-clone',{sameNameParameters=false}={}){
 const fixture=readProjectData(new URL('../rust-text-value-projection/data/fixtures.lino',import.meta.url));
 const entry=fixture.cases.find(row=>row.id===id);
 assert.ok(entry,'the existing closed Rust text source exists');
 let source=entry.source;
 if(sameNameParameters){
  assert.equal(id,'equal-valued-distinct-owners');
  const original='alternate(right:String)->String{right.clone()}';
  assert.equal(source.split(original).length,2,'the source edit has one exact declaration owner');
  source=source.replace(original,'alternate(left:String)->String{left.clone()}');
 }
 const value=createPortableRustTextCertificate(source);
 const packet=encodePortableRustTextCertificate(value.program,value.proof);
 const provenance=field(packet,packet.root,'provenance').child;
 const interpretation=field(packet,provenance,'sourceInterpretation').child;
 packet.network.insertFieldRelation(body(packet),label(facet),interpretation);
 return packet;
}

function example(id,expected,contract,create){return{id,expected,contract,create};}
const accepted={rawShape:'accepted',selectedGraph:'accepted'};
const refused={rawShape:'refused',selectedGraph:'refused'};

export function originalLinkRawAdversarialFixtures(){
 return[
  example('authentic-owned-clone',accepted,
   'An actual existing closed source certificate is admitted unchanged.',
   ()=>createRawTextCertificateFixture()),
  example('authentic-same-name-independent-parameters',accepted,
   'Equal parameter spelling in separate real declarations is valid before any retarget.',
   ()=>createRawTextCertificateFixture('equal-valued-distinct-owners',{sameNameParameters:true})),
  example('same-name-foreign-binding-retarget',refused,
   'Required final raw-role refusal: actual lexical ownership cannot be replaced by matching spelling. This is not a claim that the initial generic raw-shape draft already implements that pass.',()=>{
    const packet=createRawTextCertificateFixture('equal-valued-distinct-owners',{sameNameParameters:true});
    const choose=declaration(packet,'choose'),alternate=declaration(packet,'alternate');
    const original=parameter(packet,choose),foreign=parameter(packet,alternate);
    assert.ok(!original.equals(foreign),'independent owners retain distinct parameter LinkIds');
    const originalName=packet.network.link(field(packet,original,'name').child).metadata().definition;
    const foreignName=packet.network.link(field(packet,foreign,'name').child).metadata().definition;
    assert.equal(originalName,foreignName);
    const core=field(packet,choose,'body').child,reference=field(packet,core,'binding reference');
    assert.ok(reference.child.equals(original));
    replace(reference,foreign);
    return{...packet,witness:{original:key(original),foreign:key(foreign),name:originalName,core:key(core)}};
   }),
  example('duplicate-selected-body-facet',refused,
   'The body owns exactly one selected source facet, even when a duplicate points to the same actual proof.',()=>{
    const packet=createRawTextCertificateFixture();
    packet.network.insertFieldRelation(body(packet),label(facet),proof(packet));
    return packet;
   }),
  example('misplaced-sole-main-facet',refused,
   'A sole marker under the reachable main record selects refusal rather than an unselected default.',()=>{
    const packet=createRawTextCertificateFixture(),selected=marker(packet);
    packet.network.deleteLink(selected.link.id());
    packet.network.insertFieldRelation(field(packet,body(packet),'main').child,label(facet),selected.child);
    return packet;
   }),
  example('additional-main-facet',refused,
   'The genuine body marker does not authorize an additional reachable nested marker.',()=>{
    const packet=createRawTextCertificateFixture();
    packet.network.insertFieldRelation(field(packet,body(packet),'main').child,label(facet),proof(packet));
    return packet;
   }),
  example('additional-envelope-facet',refused,
   'A selected archive envelope permits only its existing provenance field.',()=>{
    const packet=createRawTextCertificateFixture();
    packet.network.insertFieldRelation(packet.root,label(facet),proof(packet));
    return packet;
   }),
  ...[2,4].map(arity=>example('body-claimed-field-arity-'+arity,
   {...refused,oldDecoder:'accepted'},
   'The new raw component must refuse a malformed Field relation claiming the actual body. The old decoder fieldRelations() ignores non-three-reference relations; its acceptance alone is insufficient.',()=>{
    const packet=createRawTextCertificateFixture(),selected=marker(packet);
    const labelId=selected.link.references()[1],references=[body(packet),labelId];
    if(arity===4)references.push(selected.child,selected.child);
    const malformed=packet.network.insertLink(references,LinkMetadata.new().withLinkType(LinkType.Field));
    const labelLink=packet.network.link(labelId),labelReferences=labelLink.references();
    assert.equal(labelReferences.length,1);
    assert.ok(labelReferences[0].equals(labelId),'the valid label self-link stays intact');
    return{...packet,witness:{malformed:key(malformed),label:key(labelId),arity}};
   })),
  example('ordinary-field-label-self-link',accepted,
   'The shared label point is a valid one-reference Field self-link, not a malformed relation.',()=>{
    const packet=createRawTextCertificateFixture(),labelId=marker(packet).link.references()[1];
    const labelLink=packet.network.link(labelId),references=labelLink.references();
    assert.equal(labelLink.metadata().linkType,LinkType.Field);
    assert.equal(references.length,1);
    assert.ok(references[0].equals(labelId));
    return{...packet,witness:{label:key(labelId)}};
   }),
  example('legacy-non-field-label-target',accepted,
   'An existing Field relation reads the target label term; the legacy decoder does not require the target itself to have Field metadata.',()=>{
    const packet=createRawTextCertificateFixture(),selected=marker(packet),references=selected.link.references();
    const replacement=packet.network.insertLink([],packet.network.link(references[1]).metadata().withLinkType(LinkType.Concept));
    selected.link.setReferences([selected.parent,replacement,selected.child]);
    return{...packet,witness:{label:key(replacement)}};
   }),
  example('proof-scalar-positional-child',refused,
   'A proof scalar cannot own positional structural children.',()=>{
    const packet=createRawTextCertificateFixture(),model=field(packet,proof(packet),'model').child;
    packet.network.link(model).setReferences([scalar(packet,'string','unexpected')]);
    return packet;
   }),
  example('proof-scalar-named-child',refused,
   'A proof scalar cannot own a named structural child.',()=>{
    const packet=createRawTextCertificateFixture(),model=field(packet,proof(packet),'model').child;
    packet.network.insertFieldRelation(model,'unexpected structural child',scalar(packet,'string','unexpected'));
    return packet;
   }),
  example('harmless-unrelated-root',accepted,
   'An unrelated root and its self-reference are outside the selected program body.',()=>{
    const packet=createRawTextCertificateFixture();
    const unrelated=packet.network.insertPoint('independent raw preflight root');
    return{...packet,witness:{unrelated:key(unrelated)}};
   }),
  example('harmless-second-archive-root',accepted,
   'Another envelope may reference the same actual body without adding a body-owned facet or changing aliases.',()=>{
    const packet=createRawTextCertificateFixture();
    const unrelated=packet.network.insertLink([body(packet)],packet.network.link(packet.root).metadata().clone());
    return{...packet,witness:{unrelated:key(unrelated)}};
   }),
  example('unselected-user-provenance-marker',{rawShape:'unselected',selectedGraph:'unselected',oldDecoder:'unselected'},
   'User provenance alone never activates a source interpretation.',()=>{
    const packet=createRawTextCertificateFixture(),selected=marker(packet);
    packet.network.deleteLink(selected.link.id());
    const provenance=field(packet,packet.root,'provenance').child;
    packet.network.insertFieldRelation(provenance,label(facet),scalar(packet,'boolean','false'));
    return packet;
   }),
  example('selected-user-provenance-marker',accepted,
   'Nonsemantic user provenance does not counterfeit or duplicate an independently selected body facet.',()=>{
    const packet=createRawTextCertificateFixture(),provenance=field(packet,packet.root,'provenance').child;
    packet.network.insertFieldRelation(provenance,label(facet),scalar(packet,'boolean','false'));
    return packet;
   }),
  ...['isError','hasError','isMissing','isExtra'].map(flag=>example('harmless-metadata-flag-'+flag,
   flag==='isError'||flag==='isMissing'?{...accepted,archives:{direct:'accepted',text:'accepted',binary:'serialization-refused'}}:accepted,
   'The existing typed decoder and certificate ignore '+flag+'. The original one-hot error/missing controls remain direct/text positives; binary serialization still requires hasError and must refuse those incoherent flag combinations.',()=>{
    const packet=createRawTextCertificateFixture(),link=packet.network.link(body(packet));
    link.setMetadata(link.metadata().withFlags(new LinkFlags({[flag]:true})));
    return packet;
   })),
  example('harmless-metadata-named-flag',accepted,
   'The existing typed record meaning does not depend on the named display flag.',()=>{
    const packet=createRawTextCertificateFixture(),link=packet.network.link(body(packet));
    link.setMetadata(link.metadata().withNamed(!link.metadata().named));
    return packet;
   }),
  example('harmless-metadata-span',accepted,
   'Link metadata source span is separate from the actual semantic span records checked by the certificate.',()=>{
    const packet=createRawTextCertificateFixture(),link=packet.network.link(body(packet));
    link.setMetadata(link.metadata().withSpan(new SourceSpan(new ByteRange(7,11),new Point(1,2),new Point(1,6))));
    return packet;
   }),
  ...['isError','isMissing'].map(flag=>example('harmless-metadata-flag-'+flag+'-with-hasError',accepted,
   'Adding the existing required hasError flag preserves the original semantic control and admits the coherent binary metadata profile.',()=>{
    const packet=createRawTextCertificateFixture(),link=packet.network.link(body(packet));
    link.setMetadata(link.metadata().withFlags(new LinkFlags({[flag]:true,hasError:true})));
    return packet;
   })),
  ...['bindingReference','declarationReference','datatypeReference','constructorReference'].map(name=>example('ordinary-camel-'+name+'-structural-cycle',refused,
   'An ordinary camel-case property is structural. It cannot acquire the special semantic-reference treatment of the registered raw label and hide its self-cycle.',()=>{
    const packet=createRawTextCertificateFixture(),owner=body(packet);
    const relation=packet.network.insertFieldRelation(owner,name,owner);
    assert.equal(packet.network.link(packet.network.link(relation).references()[1]).metadata().term,name);
    return{...packet,witness:{relation:key(relation),owner:key(owner),label:name}};
   })),
  ...[1,2,4].map(arity=>example('legacy-used-label-non-self-arity-'+arity,{...accepted,oldDecoder:'accepted'},
   'A used label contributes its actual term. Even a first reference to the selected body does not turn a non-three-reference used label into a malformed Field relation; remaining references are unrelated fillers.',()=>{
    const packet=createRawTextCertificateFixture(),labelId=marker(packet).link.references()[1];
    const unrelated=packet.network.insertPoint('independent non-self label target');
    const selectedParent=body(packet);
    packet.network.link(labelId).setReferences(Array.from({length:arity},(_,index)=>index===0?selectedParent:unrelated));
    return{...packet,witness:{label:key(labelId),unrelated:key(unrelated),selectedParent:key(selectedParent),arity}};
   })),
  example('used-label-three-reference-dual-role',refused,
   'A used label with three references and Field metadata still contributes its own actual Field relation; its label role cannot hide the duplicated selected body facet.',()=>{
    const packet=createRawTextCertificateFixture(),selected=marker(packet),labelId=selected.link.references()[1];
    packet.network.link(labelId).setReferences([body(packet),labelId,selected.child]);
    return{...packet,witness:{labelAndRelation:key(labelId)}};
   }),
  example('shared-module-path-dag',accepted,
   'Two declarations may share the same actual empty module-path array. A repeated completed structural node is a valid DAG alias rather than an active cycle.',()=>{
    const packet=createRawTextCertificateFixture('equal-valued-distinct-owners');
    const first=field(packet,declaration(packet,'choose'),'modulePath');
    const second=field(packet,declaration(packet,'alternate'),'modulePath');
    const fresh=packet.network.insertLink([],packet.network.link(second.child).metadata().clone());
    replace(second,fresh);
    const independent=field(packet,second.parent,'modulePath');
    assert.ok(independent.child.equals(fresh));
    assert.ok(!first.child.equals(independent.child));
    assert.equal(packet.network.link(first.child).references().length,0);
    assert.equal(packet.network.link(independent.child).references().length,0);
    const previous=independent.child;
    replace(independent,first.child);
    return{...packet,witness:{shared:key(first.child),previous:key(previous)}};
   }),
  example('harmless-unrelated-dangling-reference',{rawShape:'accepted',selectedGraph:'refused',oldDecoder:'accepted'},
   'The new raw component retains unrelated dangling raw facts without treating them as selected owners. The unchanged old66 transport globally requires registered references, so the composed wrapper still refuses after the old decoder independently accepts.',()=>{
    const packet=createRawTextCertificateFixture();
    const missing=packet.network.insertLink([],LinkMetadata.new().withLinkType(LinkType.Concept));
    const unrelated=packet.network.insertLink([missing],LinkMetadata.new().withLinkType(LinkType.Concept).withTerm('independent dangling raw root'));
    packet.network.deleteLink(missing);
    return{...packet,witness:{unrelated:key(unrelated),missing:key(missing)}};
   }),
  ...[['undefined',undefined],['empty',''],['null',null]].map(([name,value])=>example('empty-string-definition-'+name,
   value===null?{...accepted,selectedGraph:'refused',oldDecoder:'accepted',archives:{text:{rawShape:'accepted',selectedGraph:'refused',oldDecoder:'refused'},binary:'serialization-refused'}}:value===undefined?{...accepted,selectedGraph:'refused',oldDecoder:'accepted'}:{...accepted,oldDecoder:'accepted'},
   'The actual EOF raw String scalar decodes null/undefined/empty definitions as the same empty text in the new raw component and old decoder. The unchanged old66 String marker classifier refuses nullish definitions at[-32], so composed refusal remains explicit. Null also lies outside the binary string profile, while the LiNo writer stringifies it into nonempty text.',()=>{
    const packet=createRawTextCertificateFixture(),raw=readEndOfFileRawString(packet);
    replaceStringDefinition(packet,raw,value);
    return{...packet,witness:{emptyRaw:key(raw),definitionKind:name}};
   })),
  ...[
   ['null-undefined',null,undefined],
   ['null-empty',null,''],
   ['undefined-empty',undefined,''],
  ].map(([name,firstValue,secondValue])=>example('duplicate-empty-map-key-'+name,
   firstValue===null?{...refused,oldDecoder:'refused',archives:{text:{rawShape:'accepted',selectedGraph:'refused',oldDecoder:'refused'},binary:'serialization-refused'}}:{...refused,oldDecoder:'refused'},
   'Distinct original String key nodes with null/undefined/empty definitions all mean the empty key and must compare equal. Full-name scalars track their own map key to isolate duplicate-key admission before source semantics. Null-to-text conversion produces two different keys, while the complete source decoder still refuses the changed declaration names.',()=>{
    const packet=createRawTextCertificateFixture('equal-valued-distinct-owners');
    const mapping=field(packet,body(packet),'declarations').child;
    const pairs=packet.network.link(mapping).references();
    assert.equal(pairs.length,2);
    const keys=pairs.map(pair=>packet.network.link(pair).references()[0]);
    assert.ok(!keys[0].equals(keys[1]),'different original key identities remain observable');
    for(const[index,value]of[firstValue,secondValue].entries()){
     const pair=packet.network.link(pairs[index]).references();
     assert.equal(pair.length,2);
     replaceStringDefinition(packet,pair[0],value);
     replaceStringDefinition(packet,field(packet,pair[1],'fullName').child,value);
    }
    return{...packet,witness:{firstKey:key(keys[0]),secondKey:key(keys[1]),definitions:name}};
   })),
  example('compatible-empty-list-cross-role-alias',accepted,
   'An actual empty array may satisfy both wrappers and lifetimes list recipes. Each node/role obligation must be checked independently without rejecting compatible sharing.',()=>{
    const packet=createRawTextCertificateFixture(),owner=declaration(packet,'choose');
    const signature=field(packet,owner,'sourceSignature').child;
    const positions=packet.network.link(field(packet,signature,'parameters').child).references();
    assert.equal(positions.length,1);
    const wrappers=field(packet,positions[0],'wrappers').child,lifetimes=field(packet,positions[0],'lifetimes');
    assert.equal(packet.network.link(wrappers).references().length,0);
    assert.equal(packet.network.link(lifetimes.child).references().length,0);
    replace(lifetimes,wrappers);
    return{...packet,witness:{wrappersAndLifetimes:key(wrappers),position:key(positions[0])}};
   }),
  example('incompatible-nonempty-list-cross-role-alias',refused,
   'A nonempty source-owner path contains String scalars and cannot also serve as the actual Parameter-record list. Rechecking the second node/role obligation must reject the incompatible element roles.',()=>{
    const packet=createRawTextCertificateFixture(),owner=declaration(packet,'choose');
    const signature=field(packet,owner,'sourceSignature').child;
    const positions=packet.network.link(field(packet,signature,'parameters').child).references();
    assert.equal(positions.length,1);
    const sourceOwner=field(packet,positions[0],'owner').child,path=field(packet,sourceOwner,'path').child;
    assert.ok(packet.network.link(path).references().length>0);
    replace(field(packet,owner,'params'),path);
    return{...packet,witness:{pathAndParameters:key(path),declaration:key(owner)}};
   }),
 ];
}
