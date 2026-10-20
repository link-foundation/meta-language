// Exact original-source transport and primitive native observations, not a
// handwritten replacement for the borrowed visitor or numeric validation.
import assert from 'node:assert/strict';
import {readFileSync,writeFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {execFileSync} from 'node:child_process';
import {fileURLToPath} from 'node:url';
import {readRustSourceItems} from '../../../../js/scripts/generate-private-output-test-model.mjs';
import {readSourceTextRustBlock} from '../../../../js/scripts/source-text-byte-library-style.mjs';
import {readNativeSourceByteRange} from '../../../../js/scripts/native-source-byte-ranges.mjs';
import {parseCargoLock} from '../../../../js/scripts/dependency-inventory.mjs';
import {tokenize} from '../../../../js/src/translation/lexer.js';
import {DecoratorSet} from '../../../../js/src/decorators.js';
import {checkProjectData,writeProjectData} from '../../../../js/scripts/project-data.mjs';
import {buildOutcomePacket} from '../wire/generate.mjs';
import {outcomeWireEvents} from '../wire/events.mjs';
import {proposeParent} from '../bridge/generate.mjs';

const root=new URL('../../../../',import.meta.url);
const pin=source=>({bytes:Buffer.byteLength(source),sha256:createHash('sha256').update(source).digest('hex')});

function replaceObservationSourceTokens(source,replacements) {
  const {tokens}=tokenize(source,'Rust'),edits=[];
  for(let index=0;index<tokens.length;index++){
    const token=tokens[index];
    if(token.kind!=='identifier'||!Object.hasOwn(replacements,token.value))continue;
    const replacement=replacements[token.value];
    if(typeof replacement==='string')edits.push({start:token.start,end:token.end,from:token.value,to:replacement});
    else {
      assert.equal(tokens[index+1]?.value,'::');assert.ok(replacement.names.includes(tokens[index+2]?.value),'only an actual source-owned crate-root type is relocated');
      edits.push({start:token.start,end:tokens[index+1].end,from:source.slice(token.start,tokens[index+1].end),to:replacement.prefix});
    }
  }
  let current=source;
  for(const edit of [...edits].reverse())current=current.slice(0,edit.start)+edit.to+current.slice(edit.end);
  let previous=current;
  for(const edit of edits){assert.equal(previous.slice(edit.start,edit.start+edit.to.length),edit.to);previous=previous.slice(0,edit.start)+edit.from+previous.slice(edit.start+edit.to.length);}
  assert.equal(previous,source,'every original token and all other bytes survive inverse');
  return{source:current,edits};
}

function readOriginalNumberObservationSources() {
  const packet=buildOutcomePacket();
  const sinkFile='experiments/grammar-outcome-network/native-transport/bridge/sinks.rs';
  const sinkBytes=readFileSync(new URL('../bridge/sinks.rs',import.meta.url));
  const sinkSource=readNativeSourceByteRange(sinkBytes,0,sinkBytes.length),parsed=readRustSourceItems(sinkBytes);
  const pieces=parsed.items.filter(item=>['next_depth','CountArray','CountSink'].includes(item.name)
    ||item.kind==='impl_item'&&parsed.text(parsed.field(item.node,'type'))==='CountSink')
    .map(item=>({kind:item.kind,name:item.name,sourceText:readNativeSourceByteRange(sinkBytes,item.start,item.end),start:item.start,end:item.end}));
  assert.equal(pieces.length,5,'complete original depth helper, two storage declarations and both counter implementations');
  assert.equal(pieces.filter(row=>row.kind==='impl_item').length,2);
  const originalCounter=pieces.map(row=>row.sourceText).join('\n\n');
  const parentFile='rust/src/structured_data_links.rs',parentBytes=readFileSync(new URL(parentFile,root));
  const parent=readNativeSourceByteRange(parentBytes,0,parentBytes.length);
  const proposal=proposeParent(parent),parentItems=readRustSourceItems(parentBytes);
  const helpers=['fail','write_number'].map(name=>{
    const found=parentItems.items.filter(item=>item.kind==='function_item'&&item.name===name);assert.equal(found.length,1);
    return{name,sourceText:readNativeSourceByteRange(parentBytes,found[0].start,found[0].end),start:found[0].start,end:found[0].end};
  });
  const originalEntry=readSourceTextRustBlock(sinkSource,'fn','encode_grammar_parse_outcome_links');
  const preflightStart=originalEntry.indexOf('\n    if limits.node_limit'),preflightEnd=originalEntry.indexOf('\n    let mut sink = DirectSink::new(limits);');
  assert.ok(preflightStart>0&&preflightEnd>preflightStart);
  const preflight=originalEntry.slice(preflightStart,preflightEnd);
  assert.ok(preflight.includes('wire_parse_outcome(outcome, &mut count, 1)?;'));
  assert.ok(preflight.includes('count.occurrence(0, 1)?;'));
  return{packet,sink:{file:sinkFile,...pin(sinkSource),sourceText:sinkSource,pieces,originalCounter,originalEntry,preflight},
    parent:{file:parentFile,...pin(parent),sourceText:parent,helpers,exactNumberHelper:proposal.helper,numberArm:proposal.originalArm}};
}

const observationDeclaration=`#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NativeJsonNumberObservation {
    pub wire_occurrence: usize,
    pub signed: bool,
    pub unsigned: bool,
    pub floating: bool,
    pub signed_decimal: Option<String>,
    pub unsigned_decimal: Option<String>,
    pub floating_bits: Option<String>,
}`;
const observationMethod=`
    fn native_json_number(&mut self, value: &serde_json::Number, depth: usize) -> Result<(), String> {
        // These primitive facts belong to this original borrowed Number before
        // the retained projection validator can erase its representation.
        let signed = value.is_i64();
        let unsigned = value.is_u64();
        let floating = value.is_f64();
        let signed_decimal = value.as_i64().map(|integer| integer.to_string());
        let unsigned_decimal = value.as_u64().map(|integer| integer.to_string());
        let floating_bits = value.as_f64().map(|number| format!("{:016x}", number.to_bits()));
        self.json_number(value, depth)?;
        self.observations.try_reserve(1).map_err(|_| fail("unable to reserve bounded original number observations"))?;
        self.observations.push(NativeJsonNumberObservation {
            wire_occurrence: self.occurrences, signed, unsigned, floating,
            signed_decimal, unsigned_decimal, floating_bits,
        });
        Ok(())
    }
`;

export function buildOriginalNativeNumberObservations() {
  const original=readOriginalNumberObservationSources(),{packet,sink,parent}=original;
  const declaration=readSourceTextRustBlock(sink.originalCounter,'struct','CountSink');
  const storage=replaceObservationSourceTokens(declaration,{CountSink:'ObservationSink'});
  const observedStorage=storage.source.slice(0,-1)+'    observations: Vec<NativeJsonNumberObservation>,\n}';
  const inherent=sink.pieces.find(row=>row.kind==='impl_item'&&row.sourceText.startsWith('impl CountSink'));
  const implementation=sink.pieces.find(row=>row.kind==='impl_item'&&row.sourceText.startsWith('impl OutcomeWireSink'));
  assert.ok(inherent&&implementation);
  const renamedInherent=replaceObservationSourceTokens(inherent.sourceText,{CountSink:'ObservationSink'});
  const renamedImplementation=replaceObservationSourceTokens(implementation.sourceText,{CountSink:'ObservationSink'});
  const observedImplementation=renamedImplementation.source.slice(0,-1)+observationMethod+'}';
  let visitors=packet.source;
  assert.ok(!tokenize(visitors,'Rust').tokens.some(token=>token.value==='native_json_number'));
  const trait=readSourceTextRustBlock(visitors,'trait','OutcomeWireSink');
  const extendedTrait=trait.slice(0,-1)+`    fn native_json_number(&mut self, value: &serde_json::Number, depth: usize) -> Result<Self::Node, String> {
        self.json_number(value, depth)
    }
}`;
  visitors=visitors.replace(trait,extendedTrait);
  const jsonVisitor=readSourceTextRustBlock(visitors,'fn','wire_json');
  const arm='serde_json::Value::Number(value) => sink.json_number(value, depth),';
  assert.equal(jsonVisitor.split(arm).length,2);
  const observedVisitor=jsonVisitor.replace(arm,arm.replace('sink.json_number','sink.native_json_number'));
  visitors=visitors.replace(jsonVisitor,observedVisitor);
  assert.equal(visitors.replace(extendedTrait,trait).replace(observedVisitor,jsonVisitor),packet.source,'the complete original all-nine-type visitor survives exact inverse');
  const relocation=replaceObservationSourceTokens(visitors,{crate:{names:packet.inventory.declarations.map(row=>row.name),prefix:'meta_language::'}});
  const capture=`pub fn capture_original_json_number_observations(outcome: &ParseOutcome, limits: StructuredDataLinksLimits) -> Result<Vec<NativeJsonNumberObservation>, String> {
    // This new finite observation profile does not change any existing API.
    if limits.node_limit > 100_000 || limits.byte_limit > 16 * 1024 * 1024 {
        return Err(fail("original number observation profile exceeds its explicit resource bounds"));
    }
${sink.preflight}
    // The complete allocation-free original counting pass finishes first.
    let mut observed = ObservationSink { limits, occurrences: 0, bytes: 0, observations: Vec::new() };
    wire_parse_outcome(outcome, &mut observed, 1)?;
    observed.occurrence(0, 1)?;
    Ok(observed.observations)
}`;
  const counters=sink.originalCounter+'\n\n'+observationDeclaration+'\n\n'+observedStorage+'\n\n'
    +renamedInherent.source+'\n\n'+observedImplementation;
  const decorators=new DecoratorSet([
    {id:'original-number-observation-visitors',level:'emitter',order:0,when:[['format','Rust'],['component','visitors']],
      actions:[{op:'replace',field:'source',from:packet.source,to:relocation.source}]},
    {id:'original-number-observation-counters',level:'emitter',order:0,when:[['format','Rust'],['component','counters']],
      actions:[{op:'replace',field:'source',from:sink.originalCounter,to:counters}]},
  ]);
  assert.equal(decorators.decorate('emitter',{format:'Rust',component:'visitors',source:packet.source}).source,relocation.source);
  assert.equal(decorators.decorate('emitter',{format:'Rust',component:'counters',source:sink.originalCounter}).source,counters);
  const raw='// Generated from complete authenticated source declarations; do not edit.\n'
    +'use meta_language::{ParseOutcome, StructuredDataLinksLimits};\nuse serde_json::Number;\n'
    +parent.helpers.map(row=>row.sourceText).join('\n\n')+'\n'+parent.exactNumberHelper+'\n'
    +counters+'\n\n'+capture+'\n\n'
    +'include!("native_json_number_observation_visitors.rs");\n';
  const format=source=>execFileSync('rustfmt',['--edition','2024','--config','skip_children=true','--emit','stdout'],{input:source,encoding:'utf8',maxBuffer:2*1024*1024,timeout:15000});
  const outputs=[{file:'rust/tests/unit/native_json_number_observation_model.rs',source:format(raw)},
    {file:'rust/tests/unit/native_json_number_observation_visitors.rs',source:format(relocation.source)}];
  assert.ok(outputs.every(row=>row.source.split('\n').length<1000),'bounded complete source modules');
  return{outputs,decorators,receipt:{schemaVersion:1,
    boundary:'Original borrowed Number accessor facts and expanded wire occurrence positions only. Existing source numeric validation and counting remain exact. Native execution, selected compiler artifact provenance, correspondence to owned source node identities and complete DTO reconstruction remain separate requirements.',
    inputs:{sink,parent},sourceInventory:packet.inventory,wirePlan:packet.plan,
    author:{file:'experiments/grammar-outcome-network/native-transport/inverse/generate-original-number-observations.mjs',...pin(readFileSync(new URL(import.meta.url)))},
    byteRangeReader:{file:'js/scripts/native-source-byte-ranges.mjs',...pin(readFileSync(new URL('../../../../js/scripts/native-source-byte-ranges.mjs',import.meta.url)))},
    originalVisitors:packet.source,trait:{before:trait,after:extendedTrait},jsonVisitor:{before:jsonVisitor,after:observedVisitor},
    observationDeclaration,observationMethod,storage:{...storage,after:observedStorage},inherent:renamedInherent,implementation:{...renamedImplementation,after:observedImplementation},
    relocation,rawModel:raw,outputs:outputs.map(({file,source})=>({file,...pin(source)})),
    limits:{nodeLimit:100000,byteLimit:16*1024*1024,depthLimit:256},
    counterBoundary:'The complete original allocation-free preflight precedes observation construction. Primitive usize and i64 conversion calls retain json_number; only original serde_json::Value::Number uses native_json_number. Raw predicate and payload accessors execute before the unchanged projection validator.'}};
}

export function generateOriginalNativeNumberObservations({check=false}={}) {
  const lockSource=readFileSync(new URL('rust/Cargo.lock',root),'utf8'),packages=parseCargoLock(lockSource).filter(row=>row.name==='serde_json');
  assert.equal(packages.length,1);assert.equal(packages[0].version,'1.0.151','uncovered native number expectation dependency version');
  const result=buildOriginalNativeNumberObservations();
  const styleFile=new URL('../../../../parity/self-translation/original-native-number-observation-decorators.lino',import.meta.url);
  if(check)assert.equal(readFileSync(styleFile,'utf8'),result.decorators.toLino());else writeFileSync(styleFile,result.decorators.toLino());
  for(const row of result.outputs){const file=new URL(row.file,root);if(check)assert.equal(readFileSync(file,'utf8'),row.source);else writeFileSync(file,row.source);}
  const values=[0,-1,2**60,-0,1.5,Number.MIN_VALUE];
  const native={ok:true,tree:{$variant:'Token',kind:'word',trivia:false,start:0n,end:1n,lexical:values,text:{$variant:'Text',payload:'a'}},ambiguities:[]};
  let ordinal=0,lexicalDepth=null,awaitLexical=false;const occurrences=[];
  for(const event of outcomeWireEvents(result.receipt.wirePlan,native)){
    if(['endArray','endObject','endPair'].includes(event.kind)){
      if(event.kind==='endArray'&&event.depth===lexicalDepth)lexicalDepth=null;
      continue;
    }
    ordinal++;
    if(event.kind==='string'&&event.key&&event.value==='lexical')awaitLexical=true;
    else if(awaitLexical){assert.equal(event.kind,'array');lexicalDepth=event.depth;awaitLexical=false;}
    else if(event.kind==='number'&&lexicalDepth!==null)occurrences.push(ordinal);
  }
  assert.equal(occurrences.length,6);
  const observations=values.map((value,index)=>{
    const buffer=Buffer.alloc(8);buffer.writeDoubleBE(value);
    return{wireOccurrence:occurrences[index],signed:index<3,unsigned:index===0||index===2,floating:index>=3,
      signedDecimal:index<3?BigInt(value).toString():null,unsignedDecimal:index===0||index===2?BigInt(value).toString():null,floatingBits:buffer.toString('hex')};
  });
  const fixture={schemaVersion:1,numberVersions:{minimum:'1.0.151',maximumExclusive:'1.0.152',excludedFeature:'arbitrary_precision',lockSource:pin(lockSource)},
    observations,expandedOccurrences:ordinal,
    boundary:'Independent explicit native variant expectations for one constructed six-number lexical array. These are test inputs and expected observations, not actual native execution. Primitive usize positions are excluded; original native predicates and payloads require CI execution.'};
  const receiptFile=new URL('../../../../parity/self-translation/original-native-number-observations.lino',import.meta.url);
  const fixtureFile=new URL('../../../../parity/fixtures/original-native-number-observations.lino',import.meta.url);
  if(check){checkProjectData(receiptFile,result.receipt);checkProjectData(fixtureFile,fixture);}else{writeProjectData(receiptFile,result.receipt);writeProjectData(fixtureFile,fixture);}
  return{...result,fixture};
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
  assert.ok(process.argv.length===3&&['--write','--check'].includes(process.argv[2]));
  const result=generateOriginalNativeNumberObservations({check:process.argv[2]==='--check'});
  console.log('Generated the original source counter and complete visitor with '+result.fixture.observations.length+' explicit native Number controls; no native execution occurred.');
}
