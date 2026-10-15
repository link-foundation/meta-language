import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {fileURLToPath} from 'node:url';
import {writeProjectData,checkProjectData} from '../../js/scripts/project-data.mjs';
import {DecoratorSet} from '../../js/src/decorators.js';
import {rustTextCertificateAdapterPolicy,rustTextCertificateIdentityTransport} from './policy.mjs';
const hash=source=>createHash('sha256').update(source).digest('hex');
const importLine=path=>"import {checkOwnedRustTextCertificate,emitOwnedRustTextCertificate,decodeRustTextCertificateFacet} from '"+path+"';";
export function rustTextCertificateOverlayRecords(){
 const sources=new Map(rustTextCertificateAdapterPolicy.modules.map(module=>[module,readFileSync(new URL('../../js/src/'+module,import.meta.url),'utf8')]));
 const changes=new Map([...sources].map(([module,source])=>[module,[{from:source.split('\n')[0],to:importLine(module.startsWith('translation/')?'../rust-text-certificate-adapter.js':'./rust-text-certificate-adapter.js')+'\n'+source.split('\n')[0]}]]));
 const add=(module,from,to)=>changes.get(module).push({from,to});
 add('typed-program-links.js','  return { network, root };','  decodeRustTextCertificateFacet(network, root);\n  return { network, root };');
 add('typed-program-links.js','function decodeTypedProgramLinksContext(network, root, { nodeLimit = 100_000, depthLimit = 256 } = {}) {','function decodeTypedProgramLinksContext(network, root, { nodeLimit = 100_000, depthLimit = 256 } = {}) {\n  decodeRustTextCertificateFacet(network, root);');
 add('typed-program-links-validation.js','export function validateLinkedTypedProgram(program, references = new WeakMap(), { graphPatterns = false, patternScopes = new WeakMap() } = {}) {','export function validateLinkedTypedProgram(program, references = new WeakMap(), { graphPatterns = false, patternScopes = new WeakMap() } = {}) {\n  checkOwnedRustTextCertificate(program);');
 add('typed-program-checked-projection.js','export function recheckLinkedTypedProgram(program, {wholeProgram=false}={}) {','export function recheckLinkedTypedProgram(program, {wholeProgram=false}={}) {\n  if(checkOwnedRustTextCertificate(program))return program;');
 add('translation/emit-javascript.js','export function emitJavaScript(program, options = {}) {','export function emitJavaScript(program, options = {}) {\n  if(checkOwnedRustTextCertificate(program))return emitOwnedRustTextCertificate(program,"JavaScript",options);');
 add('translation/emit-rust.js','export function emitRust(program, options = {}) {','export function emitRust(program, options = {}) {\n  if(checkOwnedRustTextCertificate(program))return emitOwnedRustTextCertificate(program,"Rust",options);');
 return[...sources].map(([module,source])=>{
  let current=source;for(const{from,to}of changes.get(module)){if(current.split(from).length!==2)throw new Error('an exact overlay hook is missing or ambiguous: '+module);current=current.replace(from,to);}
  const decorators=new DecoratorSet([{id:'owned-text-'+module.replace(/[/.]/gu,'-'),level:'translation-rule',order:0,when:[['module',module]],actions:changes.get(module).map(({from,to})=>({op:'replace',field:'source',from,to}))}]).toLino();
  const actual=DecoratorSet.fromLino(decorators).decorate('translation-rule',{module,source}).source;if(actual!==current)throw new Error('the checked overlay differs from its exact recipe');
  return{module,sourceSha256:hash(source),overlaySha256:hash(actual),decorators,changes:changes.get(module).length};
 });
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
 const check=process.argv.includes('--check');for(const[path,value]of[['./data/policy.lino',rustTextCertificateAdapterPolicy],['./data/identity-transport.lino',rustTextCertificateIdentityTransport],['./data/overlays.lino',rustTextCertificateOverlayRecords()]]){const file=new URL(path,import.meta.url);if(check)checkProjectData(file,value);else writeProjectData(file,value);}
 console.log('Owned text adapter: exact public-module decorator receipts and original-LinkId transport recorded.');
}
