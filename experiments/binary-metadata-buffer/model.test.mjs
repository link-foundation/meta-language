import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {LinkNetwork} from '../../js/src/network.js';
import {LinkMetadata,LinkType,LinkFlags,SourceSpan,ByteRange,Point} from '../../js/src/primitives.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../../js/src/link-network-binary.js';
import {readProjectData} from '../../js/scripts/project-data.mjs';
import {transcodeLinkNetworkBinary as previous} from './prior-rules.mjs';
import {transcodeLinkNetworkBinary as candidate} from './candidate.mjs';

const fixture=readProjectData(new URL('../../parity/fixtures/link-network-binary.lino',import.meta.url));
const limit=1024*1024;
const compare=(input,binary,recordLimit=10000,byteLimit=limit)=>{
  const before=input.slice();
  const expected=previous(input,binary,recordLimit,byteLimit);
  assert.equal(candidate(input,binary,recordLimit,byteLimit),expected);
  assert.deepEqual(input,before,'neither portable framing operation mutates the input');
  return expected;
};
const bytes=hex=>Array.from({length:hex.length/2},(_,index)=>Number.parseInt(hex.slice(index*2,index*2+2),16));
const fields=encoded=>encoded.slice(0,-1).split(',').map(Number);
const record=[1,0,1,0,0,0,0,0,0,0,0,0];
const version1=[1,1,...record];
const version2=[2,2,1,...record,0,0];
const replace=(input,index,value)=>input.map((item,at)=>at===index?value:item);

test('the regression oracle is the exact previous portable rule, with its immutable revision and source digest',()=>{
  const source=readFileSync(new URL('./prior-rules.mjs',import.meta.url));
  assert.equal(createHash('sha256').update(source).digest('hex'),'4f0debafedcc6e1985ccad8ed0dd673261e511e705d4d7535db008d446cf11a4');
  assert.ok(source.includes('output = output.slice(0, lengthSlot) + encodedLength + output.slice(lengthSlot + 16);'));
});

test('the candidate generation receipt binds the explicit native model, source, builder, styles and every complete artifact',()=>{
  const root=new URL('../../',import.meta.url),digest=path=>createHash('sha256').update(readFileSync(new URL(path,root))).digest('hex');
  const gap=readProjectData(new URL('./generation-gap.lino',import.meta.url));
  const receipt=readProjectData(new URL('./native-generation.lino',import.meta.url));
  assert.equal(digest('experiments/binary-metadata-buffer/candidate.mjs'),gap.candidate.sha256);
  assert.equal(receipt.candidateSourceSha256,gap.candidate.sha256);
  assert.equal(receipt.baselineSourceSha256,gap.baseline.sha256);
  assert.equal(receipt.baselineStylesSha256,gap.nativeGeneration.originalStylesSha256);
  assert.equal(receipt.generatorSha256,digest('js/scripts/generate-link-network-binary-rules.mjs'));
  assert.equal(receipt.candidateStyleBuilderSha256,digest('js/scripts/binary-metadata-candidate-styles.mjs'));
  assert.equal(receipt.candidateStylesSha256,digest('experiments/binary-metadata-buffer/native-styles.lino'));
  assert.equal(receipt.generationPolicySha256,digest('experiments/binary-metadata-buffer/native-policy.lino'));
  assert.deepEqual(receipt.generationPolicy,readProjectData(new URL('./native-policy.lino',import.meta.url)));
  assert.deepEqual(receipt.generationPolicy.checkedProgram,{utf16Strings:true});
  assert.equal(receipt.generationPolicy.scope,'native generation input only');
  assert.equal(receipt.checkedDeclarations,53);
  assert.equal(receipt.historicalReplacementContexts,36);
  const names=receipt.declarations.map(row=>row.name);
  assert.equal(new Set(names).size,names.length);
  assert.ok(names.includes('transcode_link_network_binary'));
  assert.equal(receipt.nativeArtifacts.length,5);
  for(const artifact of receipt.nativeArtifacts)assert.equal(artifact.sha256,digest('experiments/binary-metadata-buffer/native/'+artifact.path));
});

test('multiple distinct metadata records preserve exact encoded-size thresholds and malformed-record precedence',()=>{
  const network=new LinkNetwork();
  network.insertLinkWithOptionalId(1,[2],LinkMetadata.new().withDefinition('first α😀').withLanguage('Rust'));
  network.insertLinkWithOptionalId(2,[1],LinkMetadata.new().withDefinition('second \0 Ω').withLanguage('JavaScript'));
  network.setTerm(1,'named first');network.setTerm(2,'different second');
  const current=[...encodeLinkNetworkBinary(network)];
  const flat=fields(previous(current,true,10000,limit));
  const firstOnly=new LinkNetwork();
  firstOnly.insertLinkWithOptionalId(1,[2],LinkMetadata.new().withDefinition('first α😀').withLanguage('Rust'));
  firstOnly.setTerm(1,'named first');
  const firstRecord=fields(previous([...encodeLinkNetworkBinary(firstOnly)],true,10000,limit)).slice(3,-2);
  assert.deepEqual(flat.slice(3,3+firstRecord.length),firstRecord);
  for(const version of[1,2]){
    // The frozen public v2 archive is projected through its actual reader;
    // v1 shares those same real, distinct record fields without v2 caches.
    const input=version===2?flat:[1,...flat.slice(2,-2)];
    const encoded=compare(input,false);
    assert.ok(!encoded.startsWith('-'));
    const width=encoded.length/2;
    assert.ok(input.length<width);
    assert.equal(compare(input,false,10000,width),encoded);
    assert.equal(compare(input,false,10000,width+1),encoded);
    assert.ok(compare(input,false,10000,width-1).startsWith('-2,'));
    const archive=bytes(encoded);
    assert.equal(compare(archive,true,10000,width),previous(archive,true,10000,limit));
    assert.equal(compare(archive,true,10000,width-1),'-2,0,');
    // Decode the second record cursor from the real flattened metadata; the
    // repeated identity is rejected before this record's final output limit.
    const second=(version===2?3:2)+firstRecord.length;
    assert.equal(input[second],2);
    const malformed=replace(input,second,1);
    assert.equal(compare(malformed,false,10000,width-1),previous(malformed,false,10000,limit));
    assert.ok(compare(malformed,false,10000,width-1).startsWith('-8,'));
  }
});

test('actual legacy and current graph archives retain every byte and decoded field across both portable directions',()=>{
  const archives=[fixture.legacyVersion1Bytes,fixture.version2Bytes];
  for(const count of[0,1,8,32]){
    const network=new LinkNetwork();
    for(let identity=1;identity<=count;identity++){
      const metadata=LinkMetadata.new().withLinkType(LinkType.Semantic).withNamed(identity%2===0)
        .withDefinition(identity%3===0?'':'record '+identity+'\0 α😀 '+'.'.repeat(identity))
        .withLanguage(identity%2===0?'Rust':'JavaScript')
        .withSpan(new SourceSpan(new ByteRange(identity,identity+1),new Point(identity,0),new Point(identity,1)))
        .withFlags(identity%2===0?LinkFlags.error().withMissing().withExtra():LinkFlags.clean());
      network.insertLinkWithOptionalId(identity,[identity,identity===count?1:identity+1],metadata);
      if(identity%3===0)network.setTerm(identity,'term '+identity+' Ω😀');
    }
    if(count){network.insertConceptExpression('shared meaning','Rust','selected syntax');network.deleteLink(network.insertLinkWithOptionalId(1000));}
    archives.push([...encodeLinkNetworkBinary(network)]);
  }
  for(const archive of archives){
    const decoded=compare(archive,true);
    assert.ok(!decoded.startsWith('-'));
    const flat=fields(decoded),encoded=compare(flat,false);
    assert.deepEqual(bytes(encoded),archive);
    assert.equal(compare(bytes(encoded),true),decoded);
    assert.equal(decodeLinkNetworkBinary(archive).toLino(),decodeLinkNetworkBinary(bytes(encoded)).toLino());
  }
});

test('all twenty-two refusal codes preserve exact offsets, precedence and resource-bound decisions',()=>{
  const encoded=bytes(previous(version1,false,10000,limit));
  const codes=new Set();
  const cases=[
    [version1,false,0,limit], [version1,false,1,15],
    [fixture.version2Bytes,true,10000,16],
    [replace(encoded,0,0),true], [replace(encoded,7,51),true],
    [replace(version2,1,0),false], [replace(version1,1,10001),false],
    [replace(version1,2,0),false], [replace(version1,2,Number.MAX_SAFE_INTEGER),false],
    [[1,2,...record,...record],false], [replace(version1,3,-1),false],
    [[1,1,1,1,0,...record.slice(2)],false],
    [replace(encoded,32,5),true], [replace(version1,4,256),false],
    [replace(version1,4,2),false], [replace(version1,10,2),false],
    [[...version1.slice(0,10),1,100,...version1.slice(11)],false],
    [[...version1.slice(0,10),1,1,256,...version1.slice(11)],false],
    [replace(version1,13,2),false], [[...version1.slice(0,13),1,-1,0,0,0,0,0],false],
    [replace(encoded,32,9),true], [replace(version2,15,-1),false],
    [[...version2.slice(0,15),1,0,2,0],false], [[...version1,0],false],
  ];
  for(const[input,binary,recordLimit=10000,byteLimit=limit]of cases){
    const result=compare(input,binary,recordLimit,byteLimit);
    assert.ok(result.startsWith('-'),JSON.stringify(input));
    codes.add(Number(result.split(',')[0]));
  }
  assert.deepEqual([...codes].sort((a,b)=>b-a),Array.from({length:22},(_,index)=>-index-1));
  for(const input of[version1,version2,fields(previous(fixture.version2Bytes,true,10000,limit))]){
    for(const recordLimit of[0,1,2,10000,NaN])for(const byteLimit of[0,15,16,input.length-1,input.length,2*input.length,limit])compare(input,false,recordLimit,byteLimit);
  }
  // The record is malformed before the output resource check can run.
  assert.equal(compare(replace(version1,4,2),false,10000,16),'-13,4,');
});

test('every archived truncation and bounded field or byte corruption matches the frozen rule',()=>{
  for(const archive of[fixture.legacyVersion1Bytes,fixture.version2Bytes]){
    const flat=fields(previous(archive,true,10000,limit));
    for(const[input,binary]of[[archive,true],[flat,false]]){
      for(let end=0;end<=input.length;end++)compare(input.slice(0,end),binary);
      for(let index=0;index<input.length;index++){
        for(const value of(binary?[0,255,256]:[-1,-0,NaN,Infinity,0.5,256,Number.MAX_SAFE_INTEGER+1]))compare(replace(input,index,value),binary);
      }
      compare([...input,0],binary);
    }
  }
});
