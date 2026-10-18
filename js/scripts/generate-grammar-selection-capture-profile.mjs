import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {fileURLToPath} from 'node:url';
import {DecoratorSet} from '../src/decorators.js';
import {checkProjectData,readProjectData,readProjectFileBytes,writeProjectData} from './project-data.mjs';
import {GRAMMAR_SELECTION_CAPTURE_PROFILE,GRAMMAR_SELECTION_HISTORICAL_FIXTURE,
  GRAMMAR_SELECTION_HISTORICAL_SHA256} from './grammar-selection-capture-profile.mjs';

const root=new URL('../../',import.meta.url);
const digest=bytes=>createHash('sha256').update(bytes).digest('hex');
export function generateGrammarSelectionCaptureProfile({check=false,seed}={}){
  const target=new URL(GRAMMAR_SELECTION_CAPTURE_PROFILE,root);
  const retained=seed??readProjectData(target);
  const fixtureBytes=readProjectFileBytes(new URL(GRAMMAR_SELECTION_HISTORICAL_FIXTURE,root));
  assert.equal(digest(fixtureBytes),GRAMMAR_SELECTION_HISTORICAL_SHA256);
  const fixture=readProjectData(new URL(GRAMMAR_SELECTION_HISTORICAL_FIXTURE,root));
  assert.equal(retained.historicalRevision,'49d790d56b402d5160bbf756238e28deccabc984');
  assert.equal(retained.sources.length,4);
  assert.equal(new Set(retained.sources.map(source=>source.path)).size,4);
  const forward=[],reverse=[];
  const sources=retained.sources.map((source,index)=>{
    const capture=fixture.captures.find(item=>item.path===source.path);assert.ok(capture);
    assert.equal(Buffer.byteLength(source.source),capture.bytes);
    assert.equal(digest(source.source),capture.sha256);
    assert.ok(source.edits.length>0);
    let current=source.source;
    for(const [before,after] of source.edits){
      assert.equal(typeof before,'string');assert.equal(typeof after,'string');
      assert.ok(before.length>0&&after.length>0&&before!==after);
      assert.equal(current.split(before).length,2,'each declared correction has one exact source occurrence');
      current=current.replace(before,after);
    }
    const bytes=readProjectFileBytes(new URL(source.path,root));
    assert.equal(current,new TextDecoder('utf-8',{fatal:true}).decode(bytes),'the exact correction accounts for every current source byte');
    forward.push({id:`mapping-capture-forward-${index}`,level:'emitter',when:[['capture',source.path]],
      actions:source.edits.map(([from,to])=>({op:'replace',field:'source',from,to}))});
    reverse.push({id:`mapping-capture-reverse-${index}`,level:'emitter',when:[['capture',source.path]],
      actions:[...source.edits].reverse().map(([to,from])=>({op:'replace',field:'source',from,to}))});
    return {path:source.path,source:source.source,edits:source.edits,current:{bytes:bytes.length,sha256:digest(bytes)}};
  });
  const profile={schemaVersion:1,historicalRevision:retained.historicalRevision,
    historicalFixture:{path:GRAMMAR_SELECTION_HISTORICAL_FIXTURE,bytes:fixtureBytes.length,sha256:GRAMMAR_SELECTION_HISTORICAL_SHA256},
    sources,forwardDecorators:new DecoratorSet(forward).toLino(),reverseDecorators:new DecoratorSet(reverse).toLino()};
  for(const source of sources){
    const original={capture:source.path,source:source.source};
    const current=DecoratorSet.fromLino(profile.forwardDecorators).decorate('emitter',original);
    assert.deepEqual(DecoratorSet.fromLino(profile.reverseDecorators).decorate('emitter',current),original);
  }
  if(check)checkProjectData(target,profile);else writeProjectData(target,profile);
  return profile;
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
  const args=process.argv.slice(2);assert.ok(args.every(arg=>arg==='--check'));
  generateGrammarSelectionCaptureProfile({check:args.includes('--check')});
}
