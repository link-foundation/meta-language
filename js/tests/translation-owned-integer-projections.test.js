import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {runInNewContext} from 'node:vm';
import test from 'node:test';
import {acceptOwnedOutcomeIntegerProjection} from '../../experiments/grammar-outcome-network/native-transport/inverse/integer-projection-rules.mjs';
import {generateOwnedOutcomeIntegerProjections,readOwnedOutcomeIntegerProjectionSource} from '../../experiments/grammar-outcome-network/native-transport/inverse/generate-integer-projections.mjs';
import {parseJavaScript} from '../src/translation/javascript.js';
import {checkProgram} from '../src/translation/check.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
import {selfTranslate} from '../src/self-translation.js';
import {createTypedProgramLinks,decodeTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';
import {recheckLinkedTypedProgram} from '../src/typed-program-checked-projection.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';

const apply=(operation,row)=>operation(row.text,row.signed,row.width,Buffer.from(row.projectedBits,'hex').readDoubleBE());

test('complete exact integer projection admission matches every independent BigInt boundary and original decimal',()=>{
  const {fixture,files}=generateOwnedOutcomeIntegerProjections({check:true});
  assert.equal(fixture.cases.length,3628);assert.equal(files.length,2);
  const code=emitJavaScript(checkProgram(parseJavaScript(fixture.closure.source))).text;
  const compiled=runInNewContext(code+';acceptOwnedOutcomeIntegerProjection;');
  for(const row of fixture.cases){
    assert.equal(apply(acceptOwnedOutcomeIntegerProjection,row),row.expected,JSON.stringify(row));
    assert.equal(apply(compiled,row),row.expected,JSON.stringify(row));
  }
  for(const [text,expected]of [['9007199254740992',true],['9007199254740993',false],
    ['1152921504606846976',true],['1152921504606846977',false],
    ['18446744073709549568',true],['18446744073709551615',false]]){
    assert.equal(acceptOwnedOutcomeIntegerProjection(text,false,64,Number(text)),expected,text);
  }
});

test('exact projection source closure and both typed archives retain every complete declaration and decision',()=>{
  const closure=readOwnedOutcomeIntegerProjectionSource(),root=new URL('../../',import.meta.url);
  assert.equal(closure.inputs.length,2);
  for(const row of closure.inputs)assert.equal(readFileSync(new URL(row.file,root),'utf8'),row.sourceText);
  assert.equal(closure.source,closure.inputs[0].sourceText+closure.assembly.separator
    +closure.inputs[1].sourceText.replace(closure.assembly.removedImport,''));
  const translated=selfTranslate(closure.source,'JavaScript','Rust');
  assert.equal(translated.items.filter(row=>row.status==='carried').length,0);
  assert.equal(selfTranslate(translated.code,'Rust','JavaScript').code,closure.source);
  const packet=createTypedProgramLinks(closure.source,'JavaScript'),{fixture}=generateOwnedOutcomeIntegerProjections({check:true});
  for(const network of [LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))]){
    recheckLinkedTypedProgram(decodeTypedProgramLinks(network,packet.root),{wholeProgram:true});
    const operation=runInNewContext(emitTypedProgramLinks(network,packet.root,'JavaScript').code+';acceptOwnedOutcomeIntegerProjection;');
    for(const row of fixture.cases)assert.equal(apply(operation,row),row.expected,JSON.stringify(row));
    assert.ok(emitTypedProgramLinks(network,packet.root,'Rust').code.includes('ml_accept_owned_outcome_integer_projection_loop3'));
  }
});

test('seeded original integers and exact wide powers retain precision while changed projections refuse',()=>{
  let state=0x6d657461n;
  for(let index=0;index<1024;index++){
    state=(state*6364136223846793005n+1442695040888963407n)&((1n<<64n)-1n);
    for(const integer of [state,state&((1n<<53n)-1n),(state&((1n<<53n)-1n))<<BigInt(index%12)]){
      for(const signed of [false,true]){
        const value=signed?-integer:integer,text=value.toString(),projected=Number(text);
        const within=signed?value>=-(1n<<63n):value<(1n<<64n);
        const expected=within&&BigInt(projected)===value;
        assert.equal(acceptOwnedOutcomeIntegerProjection(text,signed,64,projected),expected,text);
        if(expected){
          assert.equal(acceptOwnedOutcomeIntegerProjection(text,signed,64,projected===0?1:-projected),false,'a different projected value refuses');
        }
      }
    }
  }
});

test('integer projection admission refuses uncovered widths negative zero nonfinite and fractional projections',()=>{
  for(const width of [0,16,128,32.5,NaN,Infinity,-Infinity])
    assert.equal(acceptOwnedOutcomeIntegerProjection('0',false,width,0),false);
  for(const projected of [-0,NaN,Infinity,-Infinity,0.5,'0',null,undefined])
    assert.equal(acceptOwnedOutcomeIntegerProjection('0',false,64,projected),false);
  for(const text of ['-0','00','+0','0.0','0e0','0\n','0\u0000','𝟘','\ud800','9'.repeat(4096)])
    assert.equal(acceptOwnedOutcomeIntegerProjection(text,false,64,0),false);
});
