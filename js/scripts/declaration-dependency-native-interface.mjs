// Derive transport from ACTUAL normal writer files and their owned headers,
// and libraryInterface returned by writeRecordRuleLibrary, never guessed loans.
import assert from 'node:assert/strict';
import {tokenize} from '../src/translation/lexer.js';
import {readRustLibraryItems} from './rust-library-packaging.mjs';

function readActualTextConstructor(files){
  const modules=files.flatMap(file=>readRustLibraryItems(file.source).items).filter(item=>item.kind==='mod'&&item.name==='ml_string');
  assert.equal(modules.length,1,'one actual generated Text module');
  const {tokens}=tokenize(modules[0].source,'Rust');
  const matching=new Map(),stack=[];
  for(let at=0;at<tokens.length;at++)if(tokens[at].kind==='punct'){
    if(['(','[','{'].includes(tokens[at].value))stack.push(at);
    else if([')',']','}'].includes(tokens[at].value)){const open=stack.pop();assert.equal(tokens[open]?.value,({')':'(',']':'[','}':'{'})[tokens[at].value]);matching.set(open,at);}
  }
  assert.equal(stack.length,0);const constructors=[];
  for(let at=0;at<tokens.length;at++)if(tokens[at].value==='impl'&&tokens[at+1]?.value==='Text'&&tokens[at+2]?.value==='{'){
    const close=matching.get(at+2);assert.ok(close>at);
    for(let next=at+3;next<close;next++){
      if(tokens[next].value==='pub'&&tokens[next+1]?.value==='fn'&&tokens[next+2]?.value==='from_text'){
        const opening=next+3,ending=matching.get(opening);assert.equal(tokens[opening].value,'(');assert.ok(ending>opening);
        const header=tokens.slice(next+1,ending+3).map(token=>token.value).join('');
        assert.match(header,/^fnfrom_text\([A-Za-z_][A-Za-z_0-9]*:&str\)->Self$/u,'the actual Text constructor has the exact borrowed-string-to-owned-Text header');
        assert.equal(tokens[ending+3]?.value,'{');constructors.push(header);
      }
      if(tokens[next].kind==='punct'&&tokens[next].value==='{')next=matching.get(next);
    }
  }
  assert.equal(constructors.length,1,'one public constructor actually owned by generated Text');
}

function readActualDecisionHeader(row,files){
  assert.match(row.target,/^[a-z_][a-z_0-9]*$/u);
  const file=files.find(file=>(file.path??'declaration_dependency_rules.rs')===row.path);assert.ok(file,'the inventoried signature has an actual generated file owner');
  const {tokens}=tokenize(file.source,'Rust');const headers=[];
  for(let at=0;at<tokens.length;at++)if(tokens[at].value==='fn'&&tokens[at+1]?.value===row.target){
    let end=at+2;while(end<tokens.length&&tokens[end].value!=='{')end+=1;
    assert.ok(end<tokens.length);headers.push(file.source.slice(tokens[at].start,tokens[end].start).trim());
  }
  assert.equal(headers.length,1);assert.equal(headers[0],row.signature,'the complete actual fn header matches its recorded ABI');
  const header=row.signature.replace(/\s+/gu,'');
  const match=/^fn([a-z_][a-z_0-9]*)\((.*)\)->(.+)$/u.exec(header);assert.ok(match);assert.equal(match[1],row.target);
  const parameters=match[2].split(',').filter(Boolean).map(parameter=>{const colon=parameter.indexOf(':');assert.ok(colon>0);return parameter.slice(colon+1);});
  return{parameters,result:match[3]};
}

/** Pure normal-author transport derivation. No source algorithm is replaced.
 * Caller writes/checks the returned source only after normal JS native generation. */
export function deriveDeclarationDependencyNativeInterface(files,libraryInterface){
  const decisions=['acceptDeclarationDependencyFacts','readDeclarationDependencyComponents','acceptRecursiveDeclarationGroup','acceptRecursiveDeclarationPublication','acceptDeclarationModuleScope','acceptDeclarationSignatureBinding'];
  for(const source of decisions)assert.equal(libraryInterface.filter(row=>row.source===source).length,1,'one actual shared decision owner: '+source);
  const scopeHeader=readActualDecisionHeader(libraryInterface.find(row=>row.source==='acceptDeclarationModuleScope'),files);
  if(scopeHeader.parameters[0]?.includes('ml_string::Text'))readActualTextConstructor(files);
  const specifications=[
    ['readDeclarationDependencyComponents','read_components',[['starts','numbers'],['counts','numbers'],['targets','numbers']],'Vec<f64>'],
    ['acceptRecursiveDeclarationGroup','accept_group',[['count','number'],['ordinary','boolean'],['unique','boolean']],'bool'],
    ['acceptRecursiveDeclarationPublication','accept_publication',[['members','number'],['declarations','number'],['signatures','number'],['flat','boolean'],['emitted','boolean'],['matching','boolean']],'bool'],
    ['acceptDeclarationModuleScope','accept_module_scope',[['forms','texts'],['complete_checked','boolean']],'bool'],
    ['acceptDeclarationSignatureBinding','accept_signature_binding',[['scope_known','boolean'],['owner_count','number']],'bool'],
  ];
  const definitions=[];
  for(const[source,name,parameters,result]of specifications){
    const row=libraryInterface.find(row=>row.source===source),actual=readActualDecisionHeader(row,files);
    assert.equal(actual.parameters.length,parameters.length);assert.equal(actual.result,result,'unknown result ABI refuses');
    const declarations=[],arguments_=[];let conversion='';
    parameters.forEach(([parameter,kind],index)=>{
      const type=actual.parameters[index];
      if(kind==='numbers'){
        declarations.push(parameter+': &[f64]');assert.ok(['&[f64]','Vec<f64>'].includes(type),'unknown actual Number array loan refuses');
        arguments_.push(parameter+(type==='Vec<f64>'?'.to_vec()':''));
      }else if(kind==='texts'){
        declarations.push(parameter+': &[&str]');
        assert.ok(['&[String]','Vec<String>','&[self::ml_string::Text]','Vec<self::ml_string::Text>'].includes(type),'unknown actual string array owner/loan refuses');
        conversion=type.includes('ml_string::Text')
          ?'    let forms = forms.iter().map(|value| rules::ml_string::Text::from_text(value)).collect::<Vec<_>>();\n'
          :'    let forms = forms.iter().map(|value| (*value).to_owned()).collect::<Vec<_>>();\n';
        arguments_.push(type.startsWith('&')?'&forms':'forms');
      }else{
        const primitive=kind==='number'?'f64':'bool';assert.equal(type,primitive,'unknown primitive ABI refuses');declarations.push(parameter+': '+primitive);arguments_.push(parameter);
      }
    });
    definitions.push('pub(super) fn '+name+'('+declarations.join(', ')+') -> '+result+' {\n'+conversion+'    rules::'+row.target+'('+arguments_.join(', ')+')\n}');
  }
  return'// Generated actual-ABI transport only; shared JS decisions own every dependency/admission result.\nuse crate::translation::declaration_dependency_rules as rules;\n\n'+definitions.join('\n\n')+'\n';
}
