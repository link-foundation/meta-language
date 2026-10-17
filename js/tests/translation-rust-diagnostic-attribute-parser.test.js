// PRIVATE B66 production-test draft. Root applies the combined parser/facet
// packet before running it from js/tests; no native compiler is used here.
import assert from 'node:assert/strict';
import test from 'node:test';
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {runInNewContext} from 'node:vm';
import {parseRust,parseRustTokenStream} from '../src/translation/rust.js';
import {tokenize} from '../src/translation/lexer.js';
import {checkProgram} from '../src/translation/check.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
import {emitRust} from '../src/translation/emit-rust.js';
import {acceptJavaScriptTerminalTagGuard} from '../src/translation/javascript-surface-rules.js';
import {readProjectData} from '../scripts/project-data.mjs';
import {encodeTypedProgramLinks,decodeTypedProgramLinks} from '../src/typed-program-links.js';
import {recheckLinkedTypedProgram} from '../src/typed-program-checked-projection.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';

const models=['closed pure Rust const predicates 1','closed Boolean Rust const decision trees 1'];
const diagnosticModel='Rust const diagnostic attributes 1';
const attribute='#[expect(clippy::fn_params_excessive_bools, reason = "why")]';
const leaf='const fn keep(value:bool)->bool{value}';
const sha=source=>createHash('sha256').update(source).digest('hex');
const observe=(source,name,args)=>runInNewContext(source+'\n'+name+'(...'+JSON.stringify(args)+');',{}, {timeout:1000});
function archiveReplays(program){
  const packet=encodeTypedProgramLinks(program);
  return [LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))]
    .map(network=>recheckLinkedTypedProgram(decodeTypedProgramLinks(network,packet.root),{wholeProgram:true}));
}
function expectOriginalAttributeRefusal(source,model){
  assert.throws(()=>parseRust(source,{constFunctionModel:model}),error=>{
    assert.equal(error.kind,'unsupported');
    assert.equal(error.reason,'Rust const function attribute: only the actual zero-argument must_use function attribute is admitted');
    assert.equal(error.span,null);return true;
  });
}

// Authenticated, complete generated source; this is not a qualifier-stripped
// replacement oracle. The capture is also retained separately in private review.
const actualCapture={
  "path": "rust/src/translation/javascript/surface_rules.rs",
  "providerSha256": "d957b63185208b2f2ccc9b6641c2b69b7d6febe3c109abe577a7216b311c0ae7",
  "start": 9570,
  "end": 9971,
  "definitionSha256": "df595bebe7953c30b2cc5ddad6f1a176050f38cd9184fecb9829bb706f6a16dd",
  "definitionBytes": 401,
  "definition": "#[must_use]\n#[expect(\n    clippy::fn_params_excessive_bools,\n    reason = \"the checked source interface retains five independent ordered Boolean facts\"\n)]\npub const fn accept_java_script_terminal_tag_guard(\n    positive: bool,\n    alternate: bool,\n    terminal: bool,\n    same_binding: bool,\n    duplicate: bool,\n) -> bool {\n    (((positive && !alternate) && terminal) && same_binding) && !duplicate\n}"
};

test('complete generated diagnostic const function keeps every Boolean row and source-free attribute owner',()=>{
  const provider=readFileSync(new URL('../../'+actualCapture.path,import.meta.url),'utf8');
  assert.equal(sha(provider),actualCapture.providerSha256);
  assert.equal(provider.slice(actualCapture.start,actualCapture.end),actualCapture.definition);
  assert.equal(sha(actualCapture.definition),actualCapture.definitionSha256);
  assert.equal(Buffer.byteLength(actualCapture.definition),actualCapture.definitionBytes);
  const source=actualCapture.definition+'\nfn main(){}';
  for(const model of models){
    const program=checkProgram(parseRust(source,{constFunctionModel:model})),fn=program.items[0];
    const proof=structuredClone(fn.sourceConstFunction),facet=structuredClone(fn.sourceDiagnosticAttributes);
    assert.equal(facet.model,diagnosticModel);assert.equal(facet.language,'Rust');assert.equal(facet.edition,'2024');
    assert.deepEqual(facet.attachment,{sourceName:fn.name,modulePath:[],constKeywordSpan:proof.keywordSpan,functionKeywordSpan:proof.functionKeywordSpan});
    assert.equal(facet.raw,source.slice(facet.span.start,facet.span.end));
    assert.equal(facet.span.end,proof.keywordSpan.start);
    assert.deepEqual(Object.keys(proof),['model','language','sourceName','keyword','keywordSpan','functionKeywordSpan','headerSpan','bodySpan','interpretation']);
    const rust=emitRust(program).definitions[0];
    assert.ok(rust.startsWith(facet.raw),'the complete validated prefix keeps original order and spelling');
    assert.match(rust,/pub const fn accept_java_script_terminal_tag_guard\(/u);
    for(const replay of [program,...archiveReplays(program)]){
      assert.deepEqual(replay.items[0].sourceConstFunction,proof);
      assert.deepEqual(replay.items[0].sourceDiagnosticAttributes,facet);
      assert.equal(emitRust(replay).definitions[0],rust);
      const javascript=emitJavaScript(replay).text;
      for(let bits=0;bits<32;bits++){
        const [positive,alternate,terminal,sameBinding,duplicate]=Array.from({length:5},(_,at)=>Boolean(bits&(1<<at)));
        const expected=positive&&!alternate&&terminal&&sameBinding&&!duplicate;
        assert.equal(acceptJavaScriptTerminalTagGuard(positive,alternate,terminal,sameBinding,duplicate),expected);
        assert.equal(observe(javascript,fn.name,[positive,alternate,terminal,sameBinding,duplicate]),expected);
      }
    }
  }
});

test('actual ordered diagnostic prefixes retain UTF16 comments raw reasons visibility and separate following owners',()=>{
  const prefix='#[must_use]\n#[expect(clippy /* owner é */ :: fn_params_excessive_bools, reason=r###"😀 ] custom!(x)"###,)]\n'
    +'/* between 😀 */#[must_use]\n'+attribute+'\n/* before visibility */pub ';
  const source='// before 😀\n'+prefix+leaf+'\n#[must_use]const fn other(value:bool)->bool{!value}\nfn main(){}';
  const tokens=tokenize(source,'Rust').tokens;
  for(const model of models){
    const surface=parseRust(source,{constFunctionModel:model}),first=surface.items[0],second=surface.items[1];
    const facet=first.sourceDiagnosticAttributes,proof=first.sourceConstFunction;
    assert.equal(facet.span.start,source.indexOf('#'));assert.equal(facet.span.end,source.indexOf('const fn'));
    assert.equal(facet.raw,prefix);assert.ok(Buffer.byteLength(source.slice(0,facet.span.start))>facet.span.start);
    assert.deepEqual(facet.tokens,tokens.filter(token=>token.start>=facet.span.start&&token.end<=facet.span.end&&token.kind!=='eof')
      .map(token=>({kind:token.kind,value:token.value,raw:token.raw,span:{start:token.start,end:token.end}})));
    assert.deepEqual(facet.tokens.filter(token=>token.kind==='string').map(token=>token.value),['😀 ] custom!(x)','why']);
    assert.equal(source.slice(proof.keywordSpan.start,proof.keywordSpan.end),'const');
    assert.equal(source.slice(proof.functionKeywordSpan.start,proof.functionKeywordSpan.end),'fn');
    assert.equal(Object.hasOwn(second,'sourceDiagnosticAttributes'),false,'a pending prefix cannot drift to the following const function');
    const program=checkProgram(surface);
    for(const replay of [program,...archiveReplays(program)]){
      assert.deepEqual(replay.items[0].sourceDiagnosticAttributes,facet);
      assert.equal(Object.hasOwn(replay.items[1],'sourceDiagnosticAttributes'),false);
      const javascript=emitJavaScript(replay).text;
      for(const value of [false,true]){assert.equal(observe(javascript,'keep',[value]),value);assert.equal(observe(javascript,'other',[value]),!value);}
    }
  }
});

test('token-only const parsing retains old paths and refuses new diagnostic source without original text',()=>{
  const old='#[must_use]pub '+leaf+'fn main(){}',source=attribute+leaf+'fn main(){}';
  for(const model of models){
    const options={constFunctionModel:model};
    assert.deepEqual(parseRustTokenStream(tokenize(old,'Rust').tokens,options),parseRust(old,options));
    assert.equal(Object.hasOwn(parseRust(old,options).items[0],'sourceDiagnosticAttributes'),false);
    assert.throws(()=>parseRustTokenStream(tokenize(source,'Rust').tokens,options),/diagnostic.*source|original.*source|source.*diagnostic|prefix/iu);
    assert.deepEqual(parseRustTokenStream(tokenize(source,'Rust').tokens,options,source),parseRust(source,options));
    assert.throws(()=>parseRustTokenStream(tokenize(source,'Rust').tokens,options,' '.repeat(source.length)),/diagnostic|source|prefix|token/iu);
  }
});

test('diagnostic admission keeps all original const refusals and rejects unknown payloads and wrong attachments',()=>{
  const fixture=readProjectData(new URL('../../experiments/rust-const-function-fixture.lino',import.meta.url));
  assert.equal(fixture.invalid.length,27);
  for(const entry of fixture.invalid)assert.throws(()=>checkProgram(parseRust(entry.source,{constFunctionModel:fixture.model})),error=>{
    assert.equal(error.kind,entry.kind,entry.name);assert.equal(error.reason,entry.reason,entry.name);assert.deepEqual(error.span??null,entry.span,entry.name);return true;
  });
  const rejected=[attribute.replace('clippy::','other::'),attribute.replace('fn_params_excessive_bools','unknown'),
    attribute.replace('"why"','1'),attribute.replace('"why"','concat!("why")'),attribute.replace('"why"','"why", reason="again"'),
    '#[cfg(any())]','#[derive(Clone)]','#[inline]','#[no_mangle]','#[custom::expect("why")]'];
  for(const model of models){
    for(const prefix of rejected)expectOriginalAttributeRefusal(prefix+leaf+'fn main(){}',model);
    for(const item of ['mod nested{}','enum Other{One}','fn ordinary(value:bool)->bool{value}'])assert.throws(()=>parseRust(attribute+item+'fn main(){}',{constFunctionModel:model}),error=>{
      assert.equal(error.kind,'unsupported');assert.equal(error.reason,'Rust const function context: only actual const predicate declarations and the ordinary main are admitted');return true;
    });
    const mainSource=attribute+'\nfn main(){}';
    const functionKeyword=tokenize(mainSource,'Rust').tokens.find(token=>token.kind==='identifier'&&token.value==='fn');
    assert.ok(functionKeyword);
    assert.throws(()=>parseRust(mainSource,{constFunctionModel:model}),error=>{
      assert.equal(error.kind,'unsupported');assert.equal(error.reason,'Rust const diagnostic attribute attachment: the supported diagnostic prefix must belong to an actual const function');
      assert.deepEqual(error.span,{start:0,end:functionKeyword.start});return true;
    });
    assert.throws(()=>checkProgram(parseRust(attribute+'const fn bad(value:bool)->f64{1.0}fn main(){}',{constFunctionModel:model})),/const|Boolean|primitive/u);
    assert.throws(()=>parseRust('#![expect(clippy::fn_params_excessive_bools,reason="why")]'+leaf+'fn main(){}',{constFunctionModel:model}),error=>{
      assert.equal(error.reason,'Rust const function context: crate attributes require an independent compilation interpretation');assert.equal(error.span,null);return true;
    });
  }
});

test('diagnostic facets reject misplaced Surface owners before lowering and coherent source-free const boundary forgeries',()=>{
  const placementReason='Rust diagnostic source attributes: a source facet is not attached to an actual function item';
  const boundaryReason='Rust diagnostic source attributes: the retained prefix absorbs or changes the actual following const token';
  for(const model of models){
    const source=attribute+' pub    '+leaf+'fn main(){}';
    const surface=parseRust(source,{constFunctionModel:model});
    for(const position of ['main','body','parameter','non-function item']){
      const changed=structuredClone(surface),fn=changed.items[0],facet=fn.sourceDiagnosticAttributes;
      delete fn.sourceDiagnosticAttributes;
      if(position==='main')changed.main.sourceDiagnosticAttributes=facet;
      else if(position==='body')fn.body.sourceDiagnosticAttributes=facet;
      else if(position==='parameter')fn.params[0].sourceDiagnosticAttributes=facet;
      else{
        const datatype=parseRust('enum Other{One}fn main(){}').items[0];
        datatype.sourceDiagnosticAttributes=facet;changed.items.push(datatype);
      }
      assert.throws(()=>checkProgram(changed),error=>{
        assert.equal(error.kind,'type',position);assert.equal(error.reason,placementReason,position);return true;
      });
    }
    const missingProof=structuredClone(surface);delete missingProof.items[0].sourceConstFunction;
    assert.throws(()=>checkProgram(missingProof),error=>{
      assert.equal(error.kind,'type');assert.equal(error.reason,'Rust diagnostic source attributes: an actual const function proof is required');return true;
    });
    const checked=checkProgram(surface);
    for(const original of [checked,...archiveReplays(checked)])for(const raw of [attribute+'     pub',attribute+' pub // ']){
      const changed=structuredClone(original),facet=changed.items[0].sourceDiagnosticAttributes;
      assert.equal(raw.length,facet.raw.length,'only whitespace ownership changes; every original const/header/body span is retained');
      facet.raw=raw;
      facet.tokens=tokenize(raw,'Rust').tokens.filter(token=>token.kind!=='eof').map(token=>({
        kind:token.kind,value:token.value,raw:token.raw,span:{start:facet.span.start+token.start,end:facet.span.start+token.end},
      }));
      // Each forged prefix is internally coherent when lexed alone. It is
      // invalid only at the independently owned next actual const boundary.
      assert.equal(facet.tokens.at(-1).value,'pub');
      assert.equal(facet.span.end,changed.items[0].sourceConstFunction.keywordSpan.start);
      for(const action of [()=>emitRust(changed),()=>encodeTypedProgramLinks(changed),()=>recheckLinkedTypedProgram(changed,{wholeProgram:true})])assert.throws(action,error=>{
        assert.equal(error.kind,'type');assert.equal(error.reason,boundaryReason);return true;
      });
    }
  }
});
