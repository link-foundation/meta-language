import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {readProjectData} from '../scripts/project-data.mjs';
import {createGrammarParser} from '../src/grammar-runtime.js';
import {parseGrammarLinks} from '../src/grammar-links.js';

const controls=readProjectData(new URL('../../parity/fixtures/keyword-parse-context-controls.lino',import.meta.url));
const pins={
  'js/src/grammar-runtime/executor.js':'3c65f3d601f61310a4a2c2230a71730aea1fc6086a1f863f24948a79bb30ed81',
  'js/src/grammar-runtime.js':'9157c49c2d917dff5d3e13dbbbf60940971a837f6edc588fc80bf2682362cef1',
};
assert.equal(controls.baseline.revision,'ae76b70abaa60d2ec051088da08dcd55fb3f73f4');
for(const row of controls.baseline.modules){
  assert.equal(createHash('sha256').update(row.source).digest('hex'),pins[row.path]);
  assert.equal(row.sha256,pins[row.path]);assert.equal(Buffer.byteLength(row.source),row.bytes);
}

function readChildWalkOperations(source){
  const chainStart=source.indexOf("const CHAIN = Symbol('children chain');");
  const chainEnd=source.indexOf('// A result whose MISSING leaf',chainStart);
  const walkStart=source.indexOf('function widthless('),walkEnd=source.indexOf('// Whether two subtrees hold the same tokens',walkStart);
  const firstStart=source.indexOf('function firstLeafStart('),firstEnd=source.indexOf('// The children of a joined result',firstStart);
  assert.ok(chainStart>=0&&chainEnd>chainStart&&walkStart>=0&&walkEnd>walkStart&&firstStart>=0&&firstEnd>firstStart);
  const scalar=['scannedToken','isTrivia','isSeparator'].map(name=>{
    const start=source.indexOf('function '+name+'('),end=source.indexOf('\n}',start)+2;
    assert.ok(start>=0&&end>start);return source.slice(start,end);
  }).join('\n');
  const rules=new URL('../src/translation/frontend-rules.js',import.meta.url);
  const moduleSource='import {acceptScannerPreemptionBoundary,readScannerContinuationAction} from '+JSON.stringify(rules.href)+';\n'
    +'const NO_CHILDREN=Object.freeze([]);\n'
    +scalar+'\n'+source.slice(chainStart,chainEnd)+'\n'+source.slice(walkStart,walkEnd)+'\n'+source.slice(firstStart,firstEnd)
    +'\nexport {CHAIN,partOf,childCount,shareChildren,flattenChain,leavesBackward,lastRepaired,startsWidthless,lexedPast,preempted,firstLeafStart};';
  return import('data:text/javascript;base64,'+Buffer.from(moduleSource).toString('base64'));
}

const original=await readChildWalkOperations(controls.baseline.modules[0].source);
const current=await readChildWalkOperations(readFileSync(new URL('../src/grammar-runtime/executor.js',import.meta.url),'utf8'));

function createChildWalkOwner(operations,children,form,chains=[],header={}){
  const values=children.map(child=>child.type==='node'
    ? createChildWalkOwner(operations,child.children,form,chains,child) : {...child});
  let part=[];
  if(form==='flat')part=values;
  else for(let index=0;index<values.length;index++){
    const right=[values[index]];
    part={left:part,right,length:index+1,flat:null};chains.push(part);
    if(form==='mixed'&&index%2===1)operations.flattenChain(part);
  }
  const source=Array.isArray(part)?{children:part}:{[operations.CHAIN]:part};
  const {children:unused,...fields}=header;
  return operations.shareChildren({type:'node',kind:'container',start:-12,end:40,cost:0,...fields},source);
}

test('lazy first and backward child walks preserve every frozen trivia and boundary observation',()=>{
  const token={type:'token',kind:'word',start:-4,end:-2,lexed:'source-word'};
  const boundary={type:'token',kind:'renamed-boundary',start:3,end:3,scanned:true};
  const separator={type:'token',kind:null,start:4,end:5,trivia:true};
  const namedTrivia={type:'node',kind:'comment',start:-8,end:-5,trivia:true,children:[token]};
  let deep={type:'node',kind:'empty',start:-0,end:0,children:[]};
  for(let index=0;index<32;index++)deep={type:'node',kind:'nested',start:-index,end:10,children:[namedTrivia,deep,boundary]};
  const cases=[[],[separator],[namedTrivia],[token],[boundary],[deep],
    [token,boundary,separator],[separator,namedTrivia,token,boundary],
    [{type:'node',kind:'empty',start:-7,end:9,children:[]},token],
    [{type:'missing',kind:'inserted',start:2,end:2},boundary],
    [{type:'embed',kind:'embedded',start:0,end:3},boundary],
    [{type:'node',kind:'group',start:-4,end:3,children:[token,boundary]},separator],
  ];
  for(const children of cases)for(const form of ['flat','joined','mixed']){
    const previous=createChildWalkOwner(original,children,'flat'),chains=[];
    const actual=createChildWalkOwner(current,children,form,chains);
    const expected={leaves:[...original.leavesBackward(previous.children)],last:original.lastRepaired(previous.children),
      widthless:original.startsWidthless(previous),start:original.firstLeafStart(previous)};
    const observed={leaves:[...current.leavesBackward(current.partOf(actual))],last:current.lastRepaired(current.partOf(actual)),
      widthless:current.startsWidthless(actual),start:current.firstLeafStart(actual)};
    if(form==='joined')assert.ok(chains.every(chain=>chain.flat===null),'edge queries never flatten a joined prefix');
    // Materialize semantic values only after checking lazy storage; the sole
    // internal CHAIN symbol and getter descriptors are representation details.
    assert.deepEqual(structuredClone(observed),structuredClone(expected));
    if(Object.is(expected.start,-0))assert.equal(Object.is(observed.start,-0),true);
  }
});

test('lazy backward walks preserve frozen scanner preemption across aliases states and repair costs',()=>{
  for(const aliases of [false,true])for(const shifted of [false,true])for(const sameState of [false,true])for(const cost of [0,1]){
    const run=operations=>{
      const first={type:'token',kind:'original',lexed:'same-source',start:0,end:1};
      const otherFirst={...first,kind:aliases?'alias':'different',lexed:aliases?'same-source':'other-source'};
      const boundary={type:'token',kind:'boundary',start:1,end:1,scanned:true};
      const next={type:'token',kind:'next',start:2,end:3};
      const left=createChildWalkOwner(operations,[first],'joined',[],{end:1,state:{key:'same'},cost});
      const other=createChildWalkOwner(operations,[otherFirst,...(shifted?[boundary]:[]),next],'joined',[],{end:3,state:{key:sameState?'same':'different'},cost:0});
      const right=createChildWalkOwner(operations,[boundary],'joined');
      const continued=[[left,[right]],[other,[]]],pruned=operations.preempted(continued);
      return {pruned:continued.map(([result])=>pruned?.has(result)??false),past:operations.lexedPast(other,first,cost)};
    };
    assert.deepEqual(run(current),run(original));
  }
});

test('materialized child arrays retain original parts while replacement children own their exact observations',()=>{
  const first={type:'token',kind:'first',start:0,end:1},second={type:'token',kind:'second',start:1,end:2};
  const left=[first],right=[second],chain={left,right,length:2,flat:null};
  const owner=current.shareChildren({kind:'owner'},{[current.CHAIN]:chain});
  assert.deepEqual(owner.children,[first,second]);assert.equal(owner.children,chain.flat);
  assert.equal(chain.left,left);assert.equal(chain.right,right);
  const replacement={type:'missing',kind:'replacement',start:-0,end:-0};
  const spread={...owner,children:[replacement]},copy=current.shareChildren({kind:'copy'},spread);
  assert.equal(current.partOf(spread),spread.children);assert.equal(current.childCount(spread),1);
  assert.equal(copy.children,spread.children);assert.equal(Object.hasOwn(copy,current.CHAIN),false);
  assert.deepEqual([...current.leavesBackward(current.partOf(spread))],[replacement]);
  const inherited=Object.create({children:[replacement]});inherited[current.CHAIN]=chain;
  assert.equal(current.partOf(inherited),inherited.children);assert.equal(current.childCount(inherited),1);
  let reads=0;
  const getter={[current.CHAIN]:chain,get children(){reads++;return [replacement];}};
  assert.deepEqual(current.partOf(getter),[replacement]);assert.equal(reads,1);
  assert.equal(current.childCount(getter),1);assert.equal(reads,2);
  assert.deepEqual(current.shareChildren({},getter).children,[replacement]);assert.equal(reads,3);
  const historical={left:null,right:null,length:1,flat:[replacement]};
  const oldOwner=current.shareChildren({type:'node',start:4},{[current.CHAIN]:historical});
  assert.equal(current.firstLeafStart(oldOwner),-0);
  assert.deepEqual([...current.leavesBackward(current.partOf(oldOwner))],[replacement]);
});

function childWalkBaselineModule(row,overrides=new Map()){
  const base=new URL('../../'+row.path,import.meta.url);
  const source=row.source.replace(/\bfrom (['"])(\.[^'"\n]+)\1/gu,(_,quote,path)=>
    'from '+quote+(overrides.get(new URL(path,base).href)??new URL(path,base).href)+quote)
    .replaceAll('import.meta.url',JSON.stringify(base.href));
  return 'data:text/javascript;base64,'+Buffer.from(source).toString('base64');
}

test('lazy child traversal retains complete frozen parser outcomes for scanner boundaries and repairs',async()=>{
  const executor=controls.baseline.modules[0],driver=controls.baseline.modules[1];
  const overrides=new Map([[new URL('../../'+executor.path,import.meta.url).href,childWalkBaselineModule(executor)]]);
  const baseline=await import(childWalkBaselineModule(driver,overrides));
  const grammar=parseGrammarLinks(`
    (grammar (start document) (matching longest))
    (extra (class plain (char %20) (char %0A)))
    (scanner layout (tokens boundary) (operations (if (not (valid boundary)) (then fail)) (if (next (literal %0A)) (then (emit boundary))) fail))
    (rule document normal (seq (choice unordered (ref single) (ref continued)) (optional (immediateToken (ref boundary)))))
    (rule single normal (literal a))
    (rule continued normal (seq (ref single) (literal b)))
  `);
  const actual=createGrammarParser(grammar),previous=baseline.createGrammarParser(grammar);
  for(const source of ['a','a\n','a b','a\nb','a c',''])for(const options of [{},{errorRecovery:true,recovery:'accept'}]){
    assert.deepEqual(actual.parseTree(source,options),previous.parseTree(source,options),JSON.stringify({source,options}));
  }
});
