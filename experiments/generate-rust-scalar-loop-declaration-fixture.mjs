// Complete source and source-free graph observations for the versioned continuation model.
import assert from 'node:assert/strict';
import vm from 'node:vm';
import {readProjectData,writeProjectData,checkProjectData} from '../js/scripts/project-data.mjs';
import {parseRust} from '../js/src/translation/rust.js';
import {parseJavaScript} from '../js/src/translation/javascript.js';
import {checkProgram} from '../js/src/translation/check.js';
import {emitJavaScript} from '../js/src/translation/emit-javascript.js';
import {emitRust} from '../js/src/translation/emit-rust.js';
import {createTypedProgramLinks,decodeTypedProgramLinks,encodeTypedProgramLinks} from '../js/src/typed-program-links.js';
import {LinkNetwork} from '../js/src/network.js';
import {declarationScalarLoopModel as model,declarationScalarLoopCases as cases} from './rust-scalar-loop-declaration-cases.mjs';
const options={scalarLoopModel:model};
const scalar=value=>Object.is(value,-0)?'-0':String(value);
function observe(code,entry,name=entry.name){const context=vm.createContext({});vm.runInContext(code,context,{timeout:1000});for(const [args,expected] of entry.observations){context.scalarLoopArguments=args;assert.ok(Object.is(vm.runInContext(name+'(...scalarLoopArguments)',context,{timeout:1000}),expected),name+' independent observation');}}
const fixture={model,legacyModel:'closed primitive scalar loop 1',continuationPremise:'Source continue resolves the exact absolute root-owned declaration scope; parsed parameter and local names are retained, while existing emitters use hygienic output spellings',resourcePremise:'One closed scalar loop; existing iterative tail-call lowering; no termination, heap, effects, reference or universal resource claim',cases:cases.map(entry=>{
 if(entry.language==='JavaScript')observe(entry.source,entry);
 const surface=(entry.language==='Rust'?parseRust:parseJavaScript)(entry.source,options),program=checkProgram(surface);
 const javaScriptDefinitions=emitJavaScript(program).definitions.join('\n\n');observe(javaScriptDefinitions,entry);
 const reverseSurface=parseJavaScript(javaScriptDefinitions,options),reverse=checkProgram(reverseSurface),rustDefinitions=emitRust(program).definitions.join('\n\n'),reverseRustDefinitions=emitRust(reverse).definitions.join('\n\n');
 for(const definitions of [rustDefinitions,reverseRustDefinitions])observe(emitJavaScript(checkProgram(parseRust(definitions+'\nfn main(){}',options))).text,entry);
 return {language:entry.language,name:entry.name,source:entry.source,observations:entry.observations.map(([args,expected])=>({arguments:args.map(scalar),expected:scalar(expected)})),surface,javaScriptDefinitions,rustDefinitions,javaScriptReverseSurface:reverseSurface,reverseRustDefinitions};
})};
const previous=readProjectData(new URL('../parity/fixtures/rust-scalar-loops.lino',import.meta.url));
fixture.legacyCaptureRefusals=previous.bindingCaptureRefusals;
for(const entry of fixture.legacyCaptureRefusals)assert.throws(()=>createTypedProgramLinks(entry.source,entry.language,{[entry.language==='Rust'?'rustScalarLoopModel':'javaScriptScalarLoopModel']:fixture.legacyModel}),error=>error.reason===entry.reason&&error.span.start===entry.span.start&&error.span.end===entry.span.end);
const packet=createTypedProgramLinks(cases[0].source,'Rust',{rustScalarLoopModel:model}),program=decodeTypedProgramLinks(packet.network,packet.root),entry=program.declarations.get('run');
entry.modulePath=['nested'];entry.fullName='nested.run';entry.sourceLoop.continuation.path=['nested','run'];
function qualify(value){if(!value||typeof value!=='object')return;if(value.k==='call'&&value.fn==='run')value.fn='nested.run';for(const [key,child] of Object.entries(value))if(!['type','domain','span'].includes(key)){if(Array.isArray(child))child.forEach(qualify);else qualify(child);}}
qualify(entry.body);program.declarations.delete('run');program.declarations.set('nested.run',entry);program.items=[{k:'module',name:'nested',fullName:'nested',items:[entry],span:null}];
const linked=encodeTypedProgramLinks(program),reloaded=decodeTypedProgramLinks(LinkNetwork.fromLino(linked.network.toLino()),linked.root),javaScriptDefinitions=emitJavaScript(reloaded).definitions.join('\n\n');observe(javaScriptDefinitions,cases[0],'nested.run');
fixture.qualifiedLoopGraph={network:linked.network.toLino(),root:linked.root.toString(),sourceName:'run',fullName:'nested.run',javaScriptDefinitions,rustDefinitions:emitRust(reloaded).definitions.join('\n\n')};
(process.argv.includes('--check')?checkProjectData:writeProjectData)(new URL('../parity/fixtures/rust-scalar-loop-declarations.lino',import.meta.url),JSON.parse(JSON.stringify(fixture)));
